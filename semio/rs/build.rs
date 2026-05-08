//! @emoji 🏗️ Codegen: parse `target.schema.graphql` → `OUT_DIR/semio_schema_gen.rs`.
use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::PathBuf;

use graphql_parser::schema::Type as GqlType;
use graphql_parser::schema::{parse_schema, Definition, Field, InputValue, TypeDefinition};

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let schema_path = manifest_dir.join("../graphql/target.schema.graphql");
    println!("cargo:rerun-if-changed={}", schema_path.display());
    let lib_rs = manifest_dir.join("lib.rs");
    println!("cargo:rerun-if-changed={}", lib_rs.display());

    let sdl = fs::read_to_string(&schema_path).expect("read target.schema.graphql");
    let doc = parse_schema::<String>(&sdl).expect("parse schema").into_static();

    let object_impl_named = object_impl_graphql_names(&lib_rs);
    let mut manual = manual_graphql_types(&lib_rs);
    for t in simple_object_graphql_names() {
        manual.insert(t.to_string());
    }
    for t in gql_relay_hand_connection_type_names() {
        manual.insert(t.to_string());
    }
    for t in gql_relay_hand_edge_type_names() {
        manual.insert(t.to_string());
    }
    for t in hand_implemented_sdl_union_type_names() {
        manual.insert(t.to_string());
    }
    for t in hand_implemented_sdl_enum_type_names() {
        manual.insert(t.to_string());
    }
    for t in hand_implemented_sdl_input_type_names() {
        manual.insert(t.to_string());
    }

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR"));
    let out_path = out_dir.join("semio_schema_gen.rs");
    let mut w = CodeWriter::new();

    w.line("/// @emoji 🤖 Generated from `target.schema.graphql` — do not edit.");
    w.line("use std::sync::Arc;");
    w.line("use async_graphql::{InputObject, Interface, Object, Union};");
    w.line("use async_graphql::SchemaBuilder;");
    w.line("use crate::id::Id;");
    w.line("");

    let mut objects: Vec<(String, Vec<String>, Vec<Field<'static, String>>)> = Vec::new();
    let mut unions: Vec<(String, Vec<String>)> = Vec::new();
    let mut interfaces: Vec<(String, Vec<Field<'static, String>>)> = Vec::new();
    let mut enums: Vec<(String, Vec<String>)> = Vec::new();
    let mut inputs: Vec<(String, Vec<InputValue<'static, String>>)> = Vec::new();

    for def in &doc.definitions {
        if let Definition::TypeDefinition(td) = def {
            match td {
                TypeDefinition::Object(o) => {
                    let n = o.name.clone();
                    let impls: Vec<String> = o.implements_interfaces.iter().cloned().collect();
                    objects.push((n, impls, o.fields.clone()));
                }
                TypeDefinition::Union(u) => {
                    let n = u.name.clone();
                    let mem: Vec<String> = u.types.iter().cloned().collect();
                    unions.push((n, mem));
                }
                TypeDefinition::Interface(i) => {
                    let n = i.name.clone();
                    interfaces.push((n, i.fields.clone()));
                }
                TypeDefinition::Enum(e) => {
                    let n = e.name.clone();
                    let vals: Vec<String> = e.values.iter().map(|v| v.name.clone()).collect();
                    enums.push((n, vals));
                }
                TypeDefinition::InputObject(io) => {
                    let n = io.name.clone();
                    inputs.push((n, io.fields.clone()));
                }
                TypeDefinition::Scalar(_) => {}
            }
        }
    }

    let rust_ty = |gql: &str| -> String {
        match gql {
            "Type" => "SgType".to_string(),
            t => format!("Sg{}", t),
        }
    };

    let skip_objects: HashSet<String> = manual.iter().cloned().collect();

    for (name, vals) in &enums {
        if skip_objects.contains(name) {
            continue;
        }
        let rs = rust_ty(name);
        w.line("#[derive(Clone, Copy, Debug, Eq, PartialEq, Enum)]");
        w.fmt_line(format_args!("#[graphql(name = \"{}\")]", name));
        w.fmt_line(format_args!("pub enum {} {{", rs));
        for v in vals {
            w.fmt_line(format_args!("    #[graphql(name = \"{}\")]", v));
            w.fmt_line(format_args!("    {},", sanitize_variant(v)));
        }
        w.line("}");
        w.line("");
    }

    for (name, fields) in &inputs {
        if name == "Timestamp" {
            continue;
        }
        if skip_objects.contains(name) {
            continue;
        }
        let rs = rust_ty(name);
        w.line("#[derive(Clone, Debug, InputObject)]");
        w.fmt_line(format_args!("#[graphql(name = \"{}\")]", name));
        w.fmt_line(format_args!("pub struct {} {{", rs));
        for f in fields {
            let fname = to_snake(&f.name);
            let (ty, gql_name_attr) = input_rust_type(&f.value_type, &rust_ty);
            if let Some(gn) = gql_name_attr {
                w.fmt_line(format_args!("    #[graphql(name = \"{}\")]", gn));
            }
            w.fmt_line(format_args!("    pub {}: {},", fname, ty));
        }
        w.line("}");
        w.line("");
    }

    for (name, members) in &unions {
        if skip_objects.contains(name) {
            continue;
        }
        if members.is_empty() {
            continue;
        }
        let rs = rust_ty(name);
        w.line("#[derive(Clone, Union)]");
        w.fmt_line(format_args!("#[graphql(name = \"{}\")]", name));
        w.fmt_line(format_args!("pub enum {} {{", rs));
        for m in members {
            let var = sanitize_variant(m);
            if let Some((arc_ty, _)) = manual_union_inner(m) {
                w.fmt_line(format_args!("    {}({}),", var, arc_ty));
            } else {
                let inner = rust_ty(m);
                w.fmt_line(format_args!("    {}(Arc<{}>),", var, inner));
            }
        }
        w.line("}");
        w.line("");
    }

    for (name, members) in &unions {
        if skip_objects.contains(name) {
            continue;
        }
        if members.is_empty() {
            continue;
        }
        let rs = rust_ty(name);
        let arm = default_union_variant_arm(&members[0], &skip_objects, &rust_ty);
        w.fmt_line(format_args!("impl Default for {} {{", rs));
        w.fmt_line(format_args!("    fn default() -> Self {{ {} }}", arm));
        w.line("}");
        w.line("");
    }

    for (name, _impls, fields) in &objects {
        if skip_objects.contains(name) {
            continue;
        }
        let rs = rust_ty(name);
        w.line("#[derive(Clone, Default)]");
        w.fmt_line(format_args!("pub struct {} {{", rs));
        w.line("    pub id: Id,");
        w.line("}");
        w.line("");
        w.fmt_line(format_args!("#[Object(name = \"{}\")]", name));
        w.fmt_line(format_args!("impl {} {{", rs));
        w.line("    pub async fn id(&self) -> Id { self.id.clone() }");
        for f in fields {
            if f.name == "id" {
                continue;
            }
            let method = to_snake(&f.name);
            let gql_rename = if method != f.name { Some(f.name.as_str()) } else { None };
            let ret = output_rust_type(&f.field_type, &rust_ty, &skip_objects);
            if let Some(gn) = gql_rename {
                w.fmt_line(format_args!("    #[graphql(name = \"{}\")]", gn));
            }
            w.fmt_line(format_args!(
                "    pub async fn {}(&self) -> {} {{ {} }}",
                method, ret.ty, ret.body
            ));
        }
        w.line("}");
        w.line("");
    }

    for (iname, ifields) in &interfaces {
        if iname == "Operation" {
            continue;
        }
        let rs = rust_ty(iname);
        let entity_family = matches!(iname.as_str(), "Entity" | "WeakEntity" | "StrongEntity");
        let implementors: Vec<&str> = objects
            .iter()
            .filter(|(n, impls, _)| {
                if !impls.iter().any(|i| i == iname) {
                    return false;
                }
                if entity_family
                    && skip_objects.contains(n.as_str())
                    && simple_object_graphql_names().iter().copied().any(|s| s == n.as_str())
                    && !object_impl_named.contains(n)
                {
                    return false;
                }
                if iname == "EntityConnectionInterface"
                    && gql_relay_hand_connection_type_names()
                        .iter()
                        .copied()
                        .any(|cn| cn == n.as_str())
                {
                    return false;
                }
                if iname == "EntityEdge"
                    && gql_relay_hand_edge_type_names()
                        .iter()
                        .copied()
                        .any(|en| en == n.as_str())
                {
                    return false;
                }
                true
            })
            .map(|(n, _, _)| n.as_str())
            .collect();

        if implementors.is_empty() {
            continue;
        }

        w.line("#[derive(Clone, Interface)]");
        w.line("#[graphql(");
        w.fmt_line(format_args!("    name = \"{}\",", iname));
        for f in ifields {
            let ty = output_rust_type(&f.field_type, &rust_ty, &skip_objects).ty;
            let fname = &f.name;
            let rust_method = to_snake(fname);
            if f.arguments.is_empty() {
                if rust_method != fname.as_str() {
                    w.fmt_line(format_args!(
                        "    field(name = \"{}\", method = \"{}\", ty = \"{}\"),",
                        fname, rust_method, ty
                    ));
                } else {
                    w.fmt_line(format_args!("    field(name = \"{}\", ty = \"{}\"),", fname, ty));
                }
            } else {
                let args: String = f
                    .arguments
                    .iter()
                    .map(|a| {
                        let aty = output_rust_type(&a.value_type, &rust_ty, &skip_objects).ty;
                        let am = to_snake(&a.name);
                        if am != a.name {
                            format!("arg(name = \"{}\", method = \"{}\", ty = \"{}\")", a.name, am, aty)
                        } else {
                            format!("arg(name = \"{}\", ty = \"{}\")", a.name, aty)
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                if rust_method != fname.as_str() {
                    w.fmt_line(format_args!(
                        "    field(name = \"{}\", method = \"{}\", ty = \"{}\", {}),",
                        fname, rust_method, ty, args
                    ));
                } else {
                    w.fmt_line(format_args!(
                        "    field(name = \"{}\", ty = \"{}\", {}),",
                        fname, ty, args
                    ));
                }
            }
        }
        w.line(")]");
        w.fmt_line(format_args!("pub enum {} {{", rs));
        for imp in &implementors {
            let var = variant_name_for_graphql_type(imp);
            if skip_objects.contains(*imp) {
                if let Some((arc_ty, _)) = manual_union_inner(imp) {
                    w.fmt_line(format_args!("    {}({}),", var, arc_ty));
                } else {
                    panic!("skip {} has no manual_union_inner mapping", imp);
                }
            } else {
                let inner = rust_ty(imp);
                w.fmt_line(format_args!("    {}(Arc<{}>),", var, inner));
            }
        }
        w.line("}");
        w.line("");

        let first = *implementors.first().expect("non-empty implementors");
        let iface_rs = rust_ty(iname);
        let arm = default_union_variant_arm(first, &skip_objects, &rust_ty);
        w.fmt_line(format_args!("impl Default for {} {{", iface_rs));
        w.fmt_line(format_args!("    fn default() -> Self {{ {} }}", arm));
        w.line("}");
        w.line("");
    }

    w.line("/// @emoji 🔗 SDL `Operation` interface — hand-implemented as [`crate::op::OperationIface`].");
    w.line("pub type SgOperation = crate::op::OperationIface;");
    w.line("");
    w.line("/// @emoji 📌 Register generated inputs + outputs on the schema builder.");
    w.line("#[allow(clippy::too_many_lines)]");
    w.line("pub fn register_generated<B, M, S>(mut b: SchemaBuilder<B, M, S>) -> SchemaBuilder<B, M, S> {");
    for (name, _, _) in &objects {
        if skip_objects.contains(name) {
            continue;
        }
        w.fmt_line(format_args!(
            "    b = b.register_output_type::<{}>();",
            rust_ty(name)
        ));
    }
    for (name, _) in &unions {
        if skip_objects.contains(name) {
            continue;
        }
        w.fmt_line(format_args!(
            "    b = b.register_output_type::<{}>();",
            rust_ty(name)
        ));
    }
    for (name, _) in &interfaces {
        let has_impl = objects
            .iter()
            .any(|(_, impls, _)| impls.iter().any(|i| i == name));
        if !has_impl {
            continue;
        }
        w.fmt_line(format_args!(
            "    b = b.register_output_type::<{}>();",
            rust_ty(name)
        ));
    }
    for (name, _) in &enums {
        if skip_objects.contains(name) {
            continue;
        }
        w.fmt_line(format_args!(
            "    b = b.register_output_type::<{}>();",
            rust_ty(name)
        ));
    }
    for (name, _) in &inputs {
        if name == "Timestamp" {
            continue;
        }
        if skip_objects.contains(name) {
            continue;
        }
        w.fmt_line(format_args!(
            "    b = b.register_input_type::<{}>();",
            rust_ty(name)
        ));
    }
    w.line("    b");
    w.line("}");

    fs::write(&out_path, w.buf).expect("write semio_schema_gen.rs");
}

fn variant_name_for_graphql_type(gql: &str) -> String {
    sanitize_variant(gql)
}

fn default_union_variant_arm(
    member_gql: &str,
    skip: &HashSet<String>,
    rust_ty: &dyn Fn(&str) -> String,
) -> String {
    let var = sanitize_variant(member_gql);
    if let Some((_arc_ty, expr)) = manual_union_inner(member_gql) {
        format!("Self::{}({})", var, expr)
    } else if skip.contains(member_gql) {
        panic!(
            "default_union_variant_arm: `{}` is skipped but has no manual_union_inner",
            member_gql
        );
    } else {
        format!(
            "Self::{}(Arc::new({}::default()))",
            var,
            rust_ty(member_gql)
        )
    }
}

/// @emoji 🪢 Hand `gql_relay::*Connection` types (must not collide with generated `Sg*Connection` SDL stubs).
fn gql_relay_hand_connection_type_names() -> &'static [&'static str] {
    &[
        "FileConnection",
        "FolderConnection",
        "AuthorConnection",
        "ConceptConnection",
        "TagConnection",
        "QualityConnection",
        "BenchmarkConnection",
        "PropConnection",
        "AttributeConnection",
        "StatConnection",
        "LayerConnection",
        "GroupConnection",
        "PositionNodeConnection",
        "VectorNodeConnection",
        "CoordinateNodeConnection",
        "PointNodeConnection",
        "PlaneNodeConnection",
        "OffsetNodeConnection",
        "FamilyConnection",
        "DesignConnection",
        "PieceConnection",
        "TypeConnection",
        "ConflictConnection",
        "AlternativeConnection",
        "CheckpointConnection",
        "ConnectionConnection",
    ]
}

fn gql_relay_hand_edge_type_names() -> &'static [&'static str] {
    &[
        "FileEdge",
        "FolderEdge",
        "AuthorEdge",
        "ConceptEdge",
        "TagEdge",
        "QualityEdge",
        "BenchmarkEdge",
        "PropEdge",
        "AttributeEdge",
        "StatEdge",
        "LayerEdge",
        "GroupEdge",
        "PositionNodeEdge",
        "VectorNodeEdge",
        "CoordinateNodeEdge",
        "PointNodeEdge",
        "PlaneNodeEdge",
        "OffsetNodeEdge",
        "FamilyEdge",
        "DesignEdge",
        "PieceEdge",
        "TypeEdge",
        "ConflictEdge",
        "AlternativeEdge",
        "CheckpointEdge",
        "ConnectionEdge",
    ]
}

fn gql_relay_edge_inner(name: &str) -> Option<(&'static str, &'static str)> {
    match name {
        "FileEdge" => Some((
            "crate::gql_relay::FileEdge",
            "crate::gql_relay::FileEdge { cursor: String::new(), node: crate::meta::File::default() }",
        )),
        "FolderEdge" => Some((
            "crate::gql_relay::FolderEdge",
            "crate::gql_relay::FolderEdge { cursor: String::new(), node: crate::meta::Folder::default() }",
        )),
        "AuthorEdge" => Some((
            "crate::gql_relay::AuthorEdge",
            "crate::gql_relay::AuthorEdge { cursor: String::new(), node: crate::meta::Author::default() }",
        )),
        "ConceptEdge" => Some((
            "crate::gql_relay::ConceptEdge",
            "crate::gql_relay::ConceptEdge { cursor: String::new(), node: std::sync::Arc::new(crate::meta::Concept::default()) }",
        )),
        "TagEdge" => Some((
            "crate::gql_relay::TagEdge",
            "crate::gql_relay::TagEdge { cursor: String::new(), node: std::sync::Arc::new(crate::meta::Tag::default()) }",
        )),
        "QualityEdge" => Some((
            "crate::gql_relay::QualityEdge",
            "crate::gql_relay::QualityEdge { cursor: String::new(), node: std::sync::Arc::new(crate::meta::Quality::default()) }",
        )),
        "BenchmarkEdge" => Some((
            "crate::gql_relay::BenchmarkEdge",
            "crate::gql_relay::BenchmarkEdge { cursor: String::new(), node: crate::meta::Benchmark::default() }",
        )),
        "PropEdge" => Some((
            "crate::gql_relay::PropEdge",
            "crate::gql_relay::PropEdge { cursor: String::new(), node: crate::meta::Prop::default() }",
        )),
        "AttributeEdge" => Some((
            "crate::gql_relay::AttributeEdge",
            "crate::gql_relay::AttributeEdge { cursor: String::new(), node: crate::meta::Attribute::default() }",
        )),
        "StatEdge" => Some((
            "crate::gql_relay::StatEdge",
            "crate::gql_relay::StatEdge { cursor: String::new(), node: crate::meta::Stat::default() }",
        )),
        "LayerEdge" => Some((
            "crate::gql_relay::LayerEdge",
            "crate::gql_relay::LayerEdge { cursor: String::new(), node: crate::meta::Layer::default() }",
        )),
        "GroupEdge" => Some((
            "crate::gql_relay::GroupEdge",
            "crate::gql_relay::GroupEdge { cursor: String::new(), node: crate::meta::Group::default() }",
        )),
        "PositionNodeEdge" => Some((
            "crate::gql_relay::PositionNodeEdge",
            "crate::gql_relay::PositionNodeEdge { cursor: String::new(), node: std::sync::Arc::new(crate::geom::entity::PositionNode::default()) }",
        )),
        "VectorNodeEdge" => Some((
            "crate::gql_relay::VectorNodeEdge",
            "crate::gql_relay::VectorNodeEdge { cursor: String::new(), node: std::sync::Arc::new(crate::geom::entity::VectorNode::default()) }",
        )),
        "CoordinateNodeEdge" => Some((
            "crate::gql_relay::CoordinateNodeEdge",
            "crate::gql_relay::CoordinateNodeEdge { cursor: String::new(), node: std::sync::Arc::new(crate::geom::entity::CoordinateNode::default()) }",
        )),
        "PointNodeEdge" => Some((
            "crate::gql_relay::PointNodeEdge",
            "crate::gql_relay::PointNodeEdge { cursor: String::new(), node: std::sync::Arc::new(crate::geom::entity::PointNode::default()) }",
        )),
        "PlaneNodeEdge" => Some((
            "crate::gql_relay::PlaneNodeEdge",
            "crate::gql_relay::PlaneNodeEdge { cursor: String::new(), node: std::sync::Arc::new(crate::geom::entity::PlaneNode::default()) }",
        )),
        "OffsetNodeEdge" => Some((
            "crate::gql_relay::OffsetNodeEdge",
            "crate::gql_relay::OffsetNodeEdge { cursor: String::new(), node: std::sync::Arc::new(crate::geom::entity::OffsetNode::default()) }",
        )),
        "FamilyEdge" => Some((
            "crate::gql_relay::FamilyEdge",
            "crate::gql_relay::FamilyEdge { cursor: String::new(), node: crate::gql_relay::Family::default() }",
        )),
        "DesignEdge" => Some((
            "crate::gql_relay::DesignEdge",
            "crate::gql_relay::DesignEdge { cursor: String::new(), node: std::sync::Arc::new(crate::kit::design::Design::default()) }",
        )),
        "PieceEdge" => Some((
            "crate::gql_relay::PieceEdge",
            "crate::gql_relay::PieceEdge { cursor: String::new(), node: std::sync::Arc::new(crate::kit::design::piece::Piece::default()) }",
        )),
        "TypeEdge" => Some((
            "crate::gql_relay::TypeEdge",
            "crate::gql_relay::TypeEdge { cursor: String::new(), node: std::sync::Arc::new(crate::kit::r#type::Type::default()) }",
        )),
        "ConflictEdge" => Some((
            "crate::gql_relay::ConflictEdge",
            "crate::gql_relay::ConflictEdge { cursor: String::new(), node: std::sync::Arc::new(crate::vcs::Conflict::default()) }",
        )),
        "AlternativeEdge" => Some((
            "crate::gql_relay::AlternativeEdge",
            "crate::gql_relay::AlternativeEdge { cursor: String::new(), node: std::sync::Arc::new(crate::vcs::Alternative::default()) }",
        )),
        "CheckpointEdge" => Some((
            "crate::gql_relay::CheckpointEdge",
            "crate::gql_relay::CheckpointEdge { cursor: String::new(), node: std::sync::Arc::new(crate::vcs::Checkpoint::default()) }",
        )),
        "ConnectionEdge" => Some((
            "crate::gql_relay::ConnectionEdge",
            "crate::gql_relay::ConnectionEdge { cursor: String::new(), node: std::sync::Arc::new(crate::kit::design::connection::Connection::default()) }",
        )),
        _ => None,
    }
}

fn gql_relay_connection_inner(name: &str) -> Option<(&'static str, &'static str)> {
    match name {
        "FileConnection" => Some((
            "crate::gql_relay::FileConnection",
            "crate::gql_relay::FileConnection::from_rows(vec![])",
        )),
        "FolderConnection" => Some((
            "crate::gql_relay::FolderConnection",
            "crate::gql_relay::FolderConnection::from_rows(vec![])",
        )),
        "AuthorConnection" => Some((
            "crate::gql_relay::AuthorConnection",
            "crate::gql_relay::AuthorConnection::from_rows(vec![])",
        )),
        "ConceptConnection" => Some((
            "crate::gql_relay::ConceptConnection",
            "crate::gql_relay::ConceptConnection::from_rows(vec![])",
        )),
        "TagConnection" => Some((
            "crate::gql_relay::TagConnection",
            "crate::gql_relay::TagConnection::from_rows(vec![])",
        )),
        "QualityConnection" => Some((
            "crate::gql_relay::QualityConnection",
            "crate::gql_relay::QualityConnection::from_rows(vec![])",
        )),
        "BenchmarkConnection" => Some((
            "crate::gql_relay::BenchmarkConnection",
            "crate::gql_relay::BenchmarkConnection::from_rows(vec![])",
        )),
        "PropConnection" => Some((
            "crate::gql_relay::PropConnection",
            "crate::gql_relay::PropConnection::from_rows(vec![])",
        )),
        "AttributeConnection" => Some((
            "crate::gql_relay::AttributeConnection",
            "crate::gql_relay::AttributeConnection::from_rows(vec![])",
        )),
        "StatConnection" => Some((
            "crate::gql_relay::StatConnection",
            "crate::gql_relay::StatConnection::from_rows(vec![])",
        )),
        "LayerConnection" => Some((
            "crate::gql_relay::LayerConnection",
            "crate::gql_relay::LayerConnection::from_rows(vec![])",
        )),
        "GroupConnection" => Some((
            "crate::gql_relay::GroupConnection",
            "crate::gql_relay::GroupConnection::from_rows(vec![])",
        )),
        "PositionNodeConnection" => Some((
            "crate::gql_relay::PositionNodeConnection",
            "crate::gql_relay::PositionNodeConnection::from_rows(vec![])",
        )),
        "VectorNodeConnection" => Some((
            "crate::gql_relay::VectorNodeConnection",
            "crate::gql_relay::VectorNodeConnection::from_rows(vec![])",
        )),
        "CoordinateNodeConnection" => Some((
            "crate::gql_relay::CoordinateNodeConnection",
            "crate::gql_relay::CoordinateNodeConnection::from_rows(vec![])",
        )),
        "PointNodeConnection" => Some((
            "crate::gql_relay::PointNodeConnection",
            "crate::gql_relay::PointNodeConnection::from_rows(vec![])",
        )),
        "PlaneNodeConnection" => Some((
            "crate::gql_relay::PlaneNodeConnection",
            "crate::gql_relay::PlaneNodeConnection::from_rows(vec![])",
        )),
        "OffsetNodeConnection" => Some((
            "crate::gql_relay::OffsetNodeConnection",
            "crate::gql_relay::OffsetNodeConnection::from_rows(vec![])",
        )),
        "FamilyConnection" => Some((
            "crate::gql_relay::FamilyConnection",
            "crate::gql_relay::FamilyConnection::from_rows(vec![])",
        )),
        "DesignConnection" => Some((
            "crate::gql_relay::DesignConnection",
            "crate::gql_relay::DesignConnection::from_designs(vec![])",
        )),
        "PieceConnection" => Some((
            "crate::gql_relay::PieceConnection",
            "crate::gql_relay::PieceConnection::from_pieces(vec![])",
        )),
        "TypeConnection" => Some((
            "crate::gql_relay::TypeConnection",
            "crate::gql_relay::TypeConnection::from_types(vec![])",
        )),
        "ConflictConnection" => Some((
            "crate::gql_relay::ConflictConnection",
            "crate::gql_relay::ConflictConnection::from_conflicts(vec![])",
        )),
        "AlternativeConnection" => Some((
            "crate::gql_relay::AlternativeConnection",
            "crate::gql_relay::AlternativeConnection::from_alternatives(vec![])",
        )),
        "CheckpointConnection" => Some((
            "crate::gql_relay::CheckpointConnection",
            "crate::gql_relay::CheckpointConnection::from_checkpoints(vec![])",
        )),
        "ConnectionConnection" => Some((
            "crate::gql_relay::ConnectionConnection",
            "crate::gql_relay::ConnectionConnection::from_connections(vec![])",
        )),
        _ => None,
    }
}

/// @emoji 🗺️ `Arc<…>` type path for manual SDL types referenced from generated unions / interfaces.
fn manual_union_inner(name: &str) -> Option<(&'static str, &'static str)> {
    if let Some(v) = gql_relay_edge_inner(name) {
        return Some(v);
    }
    if let Some(v) = gql_relay_connection_inner(name) {
        return Some(v);
    }
    Some(match name {
        "Kit" => ("Arc<crate::kit::Kit>", "Arc::new(crate::kit::Kit::default())"),
        "Type" => ("Arc<crate::kit::r#type::Type>", "Arc::new(crate::kit::r#type::Type::default())"),
        "Design" => ("Arc<crate::kit::design::Design>", "Arc::new(crate::kit::design::Design::default())"),
        "Piece" => ("Arc<crate::kit::design::piece::Piece>", "Arc::new(crate::kit::design::piece::Piece::default())"),
        "Port" => ("Arc<crate::kit::r#type::Port>", "Arc::new(crate::kit::r#type::Port::default())"),
        "Connector" => ("Arc<crate::kit::r#type::Connector>", "Arc::new(crate::kit::r#type::Connector::default())"),
        "Representation" => ("Arc<crate::kit::r#type::Representation>", "Arc::new(crate::kit::r#type::Representation::default())"),
        "Tag" => ("Arc<crate::meta::Tag>", "Arc::new(crate::meta::Tag::default())"),
        "Concept" => ("Arc<crate::meta::Concept>", "Arc::new(crate::meta::Concept::default())"),
        "Quality" => ("Arc<crate::meta::Quality>", "Arc::new(crate::meta::Quality::default())"),
        "Graph" => ("Arc<crate::vcs::Graph>", "Arc::new(crate::vcs::Graph::default())"),
        "Session" => ("Arc<crate::vcs::Session>", "Arc::new(crate::vcs::Session::default())"),
        "Draft" => ("Arc<crate::vcs::Draft>", "Arc::new(crate::vcs::Draft::default())"),
        "Transaction" => ("Arc<crate::vcs::Transaction>", "Arc::new(crate::vcs::Transaction::default())"),
        "Change" => ("Arc<crate::vcs::Change>", "Arc::new(crate::vcs::Change::default())"),
        "Checkpoint" => ("Arc<crate::vcs::Checkpoint>", "Arc::new(crate::vcs::Checkpoint::default())"),
        "Alternative" => ("Arc<crate::vcs::Alternative>", "Arc::new(crate::vcs::Alternative::default())"),
        "Conflict" => ("Arc<crate::vcs::Conflict>", "Arc::new(crate::vcs::Conflict::default())"),
        "ReadVersion" => ("Arc<crate::vcs::ReadVersion>", "Arc::new(crate::vcs::ReadVersion::default())"),
        "WriteVersion" => ("Arc<crate::vcs::WriteVersion>", "Arc::new(crate::vcs::WriteVersion::default())"),
        "Connection" => ("Arc<crate::kit::design::connection::Connection>", "Arc::new(crate::kit::design::connection::Connection::default())"),
        "Side" => ("Arc<crate::kit::design::connection::Side>", "Arc::new(crate::kit::design::connection::Side::default())"),
        "Coordinate" => ("Arc<crate::geom::entity::CoordinateNode>", "Arc::new(crate::geom::entity::CoordinateNode::default())"),
        "Vector" => ("Arc<crate::geom::entity::VectorNode>", "Arc::new(crate::geom::entity::VectorNode::default())"),
        "Point" => ("Arc<crate::geom::entity::PointNode>", "Arc::new(crate::geom::entity::PointNode::default())"),
        "Plane" => ("Arc<crate::geom::entity::PlaneNode>", "Arc::new(crate::geom::entity::PlaneNode::default())"),
        "Position" => ("Arc<crate::geom::entity::PositionNode>", "Arc::new(crate::geom::entity::PositionNode::default())"),
        "Offset" => ("Arc<crate::geom::entity::OffsetNode>", "Arc::new(crate::geom::entity::OffsetNode::default())"),
        "Place" => ("Arc<crate::geom::entity::PlaceNode>", "Arc::new(crate::geom::entity::PlaceNode::default())"),
        "Author" => ("Arc<crate::meta::Author>", "Arc::new(crate::meta::Author::default())"),
        "File" => ("Arc<crate::meta::File>", "Arc::new(crate::meta::File::default())"),
        "Folder" => ("Arc<crate::meta::Folder>", "Arc::new(crate::meta::Folder::default())"),
        "Attribute" => ("Arc<crate::meta::Attribute>", "Arc::new(crate::meta::Attribute::default())"),
        "Benchmark" => ("Arc<crate::meta::Benchmark>", "Arc::new(crate::meta::Benchmark::default())"),
        "Prop" => ("Arc<crate::meta::Prop>", "Arc::new(crate::meta::Prop::default())"),
        "Stat" => ("Arc<crate::meta::Stat>", "Arc::new(crate::meta::Stat::default())"),
        "Group" => ("Arc<crate::meta::Group>", "Arc::new(crate::meta::Group::default())"),
        "Layer" => ("Arc<crate::meta::Layer>", "Arc::new(crate::meta::Layer::default())"),
        "Location" => ("Arc<crate::meta::Location>", "Arc::new(crate::meta::Location::default())"),
        "Family" => ("Arc<crate::gql_relay::Family>", "Arc::new(crate::gql_relay::Family::default())"),
        "PageInfo" => ("Arc<crate::gql_relay::PageInfo>", "Arc::new(crate::gql_relay::PageInfo::default())"),
        "Command" => ("Arc<crate::op::CommandReceipt>", "Arc::new(crate::op::CommandReceipt::default())"),
        "Error" => ("Arc<crate::error::SemioError>", "Arc::new(crate::error::SemioError::default())"),
        "Blueprint" => (
            "Arc<crate::kit::r#type::Blueprint>",
            "Arc::new(crate::kit::r#type::Blueprint::default())",
        ),
        "CreatedFixedPiece" => ("Arc<crate::op::CreatedFixedPiece>", "Arc::new(crate::op::CreatedFixedPiece::default())"),
        "FixedPiece" => ("Arc<crate::op::FixedPiece>", "Arc::new(crate::op::FixedPiece::default())"),
        "DraggedPiece" => ("Arc<crate::op::DraggedPiece>", "Arc::new(crate::op::DraggedPiece::default())"),
        "RenamedKit" => ("Arc<crate::op::RenamedKit>", "Arc::new(crate::op::RenamedKit::default())"),
        "ChangedDescription" => ("Arc<crate::op::ChangedDescription>", "Arc::new(crate::op::ChangedDescription::default())"),
        "RenamedKitInput" => ("Arc<crate::op::RenamedKitInput>", "Arc::new(crate::op::RenamedKitInput::default())"),
        "ChangedDescriptionInput" => ("Arc<crate::op::ChangedDescriptionInput>", "Arc::new(crate::op::ChangedDescriptionInput::default())"),
        "CreatedFixedPieceInput" => ("Arc<crate::op::CreatedFixedPieceInput>", "Arc::new(crate::op::CreatedFixedPieceInput::default())"),
        "FixedPieceInput" => ("Arc<crate::op::FixedPieceInput>", "Arc::new(crate::op::FixedPieceInput::default())"),
        "DraggedPieceInput" => ("Arc<crate::op::DraggedPieceInput>", "Arc::new(crate::op::DraggedPieceInput::default())"),
        "SemanticOpRecord" => (
            "Arc<crate::op::SemanticOpRecord>",
            "Arc::new(crate::op::SemanticOpRecord::default())",
        ),
        _ => return None,
    })
}

fn simple_object_graphql_names() -> &'static [&'static str] {
    &[
        "File",
        "Folder",
        "Author",
        "Benchmark",
        "Prop",
        "Stat",
        "Group",
        "Layer",
        "Family",
        "PageInfo",
        "Command",
        "Error",
        "SemanticOpRecord",
    ]
}

struct CodeWriter {
    buf: String,
}
impl CodeWriter {
    fn new() -> Self {
        Self { buf: String::new() }
    }
    fn line(&mut self, s: &str) {
        self.buf.push_str(s);
        self.buf.push('\n');
    }
    fn fmt_line(&mut self, args: std::fmt::Arguments<'_>) {
        use std::fmt::Write;
        let _ = write!(&mut self.buf, "{}\n", args);
    }
}

fn object_impl_graphql_names(lib_rs: &PathBuf) -> HashSet<String> {
    let s = fs::read_to_string(lib_rs).expect("read lib.rs");
    let mut out = HashSet::new();
    out.insert("Query".to_string());
    out.insert("Mutation".to_string());
    out.insert("Subscription".to_string());
    for pat in [r#"Object\(name = "([^"]+)""#, r#"async_graphql::Object\(name = "([^"]+)""#] {
        let re = regex::Regex::new(pat).unwrap();
        for cap in re.captures_iter(&s) {
            out.insert(cap[1].to_string());
        }
    }
    out
}

/// @emoji 🪢 SDL union names implemented by hand in `lib.rs` (must not emit/register a second `Sg…` union).
fn hand_implemented_sdl_union_type_names() -> &'static [&'static str] {
    &[
        "Blueprint",
        "ChangeOwner",
        "ConceptOwner",
        "DesignOwner",
        "GraphOwner",
        "KitOwner",
        "QualityOwner",
        "ReadVersionOwner",
        "SideOwner",
        "TagOwner",
        "WriteVersionOwner",
    ]
}

/// @emoji 🪢 SDL enum names implemented by hand in `lib.rs` (must not emit/register a second `Sg…` enum).
fn hand_implemented_sdl_enum_type_names() -> &'static [&'static str] {
    &["PieceConnectionKind"]
}

/// @emoji 🪢 SDL input object names implemented by hand in `lib.rs` (must not emit/register `Sg…Input` duplicates).
fn hand_implemented_sdl_input_type_names() -> &'static [&'static str] {
    &[
        "VectorInput",
        "PointInput",
        "CoordinateInput",
        "OffsetInput",
        "PlaneInput",
        "PositionInput",
        "AttributeInput",
        "TagInput",
        "ConceptInput",
        "QualityInput",
    ]
}

/// @emoji 🧾 Rust type path for hand `InputObject` types referenced from generated SDL inputs.
fn hand_implemented_sdl_input_rust_type(n: &str) -> Option<&'static str> {
    match n {
        "VectorInput" => Some("crate::geom::Vector"),
        "PointInput" => Some("crate::geom::Point"),
        "CoordinateInput" => Some("crate::geom::Coordinate"),
        "OffsetInput" => Some("crate::geom::Offset"),
        "PlaneInput" => Some("crate::geom::Plane"),
        "PositionInput" => Some("crate::geom::Position"),
        "AttributeInput" => Some("crate::meta::AttributeInput"),
        "TagInput" => Some("crate::meta::TagInput"),
        "ConceptInput" => Some("crate::meta::ConceptInput"),
        "QualityInput" => Some("crate::meta::QualityInput"),
        _ => None,
    }
}

fn manual_graphql_types(lib_rs: &PathBuf) -> HashSet<String> {
    let mut out = object_impl_graphql_names(lib_rs);
    for t in simple_object_graphql_names() {
        out.insert(t.to_string());
    }
    out
}

fn sanitize_variant(s: &str) -> String {
    let mut t = s.to_string();
    if t.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false) {
        t = format!("_{}", t);
    }
    if t == "type" {
        "r#type".to_string()
    } else {
        t
    }
}

fn to_snake(name: &str) -> String {
    let mut out = String::new();
    for (i, c) in name.chars().enumerate() {
        if c.is_uppercase() && i > 0 {
            out.push('_');
        }
        out.push(c.to_lowercase().next().unwrap());
    }
    if out == "type" {
        "kind".to_string()
    } else {
        out
    }
}

struct OutputRustType {
    ty: String,
    body: String,
}

fn output_rust_type(
    t: &GqlType<'_, String>,
    rust_ty: &dyn Fn(&str) -> String,
    skip: &HashSet<String>,
) -> OutputRustType {
    fn rec(
        t: &GqlType<'_, String>,
        rust_ty: &dyn Fn(&str) -> String,
        skip: &HashSet<String>,
        non_null: bool,
    ) -> OutputRustType {
        match t {
            GqlType::NonNullType(inner) => rec(inner, rust_ty, skip, true),
            GqlType::ListType(inner) => {
                let OutputRustType { ty, body } = rec(inner, rust_ty, skip, false);
                let ty = format!("Vec<{}>", ty);
                let body = format!("vec![{}]", body);
                if non_null {
                    OutputRustType { ty, body }
                } else {
                    OutputRustType {
                        ty: format!("Option<{}>", ty),
                        body: format!("Some({})", body),
                    }
                }
            }
            GqlType::NamedType(n) => {
                let base = match n.as_str() {
                    "ID" => (
                        "crate::id::Id".to_string(),
                        "crate::id::Id::default()".to_string(),
                    ),
                    "String" => ("String".to_string(), "String::new()".to_string()),
                    "Int" => ("i32".to_string(), "0".to_string()),
                    "Float" => ("f64".to_string(), "0.0".to_string()),
                    "Boolean" => ("bool".to_string(), "false".to_string()),
                    "Timestamp" => (
                        "Option<crate::timestamp::Timestamp>".to_string(),
                        "None".to_string(),
                    ),
                    x if skip.contains(x) => {
                        if let Some((arc_ty, expr)) = manual_union_inner(x) {
                            if arc_ty.starts_with("Arc<") {
                                (arc_ty.to_string(), expr.to_string())
                            } else {
                                (arc_ty.to_string(), expr.to_string())
                            }
                        } else {
                            panic!("output field references skipped type `{}` without manual_union_inner", x);
                        }
                    }
                    x => (
                        format!("Arc<{}>", rust_ty(x)),
                        format!("Arc::new({}::default())", rust_ty(x)),
                    ),
                };
                if non_null {
                    OutputRustType { ty: base.0, body: base.1 }
                } else {
                    OutputRustType {
                        ty: format!("Option<{}>", base.0),
                        body: format!("Some({})", base.1),
                    }
                }
            }
        }
    }
    rec(t, rust_ty, skip, false)
}

fn input_rust_type(t: &GqlType<'_, String>, rust_ty: &dyn Fn(&str) -> String) -> (String, Option<String>) {
    match t {
        GqlType::NonNullType(inner) => input_inner(inner, rust_ty),
        GqlType::ListType(inner) => {
            let (b, g) = input_inner(inner, rust_ty);
            (format!("Option<Vec<{}>>", b), g)
        }
        GqlType::NamedType(n) => {
            let (b, g) = input_named(n, rust_ty);
            (format!("Option<{}>", b), g)
        }
    }
}

fn input_inner(t: &GqlType<'_, String>, rust_ty: &dyn Fn(&str) -> String) -> (String, Option<String>) {
    match t {
        GqlType::NonNullType(inner) => input_inner(inner, rust_ty),
        GqlType::ListType(inner) => {
            let (b, g) = input_inner(inner, rust_ty);
            (format!("Vec<{}>", b), g)
        }
        GqlType::NamedType(n) => input_named(n, rust_ty),
    }
}

fn input_named(n: &str, rust_ty: &dyn Fn(&str) -> String) -> (String, Option<String>) {
    match n {
        "ID" => ("crate::id::Id".to_string(), None),
        "String" => ("String".to_string(), None),
        "Int" => ("i32".to_string(), None),
        "Float" => ("f64".to_string(), None),
        "Boolean" => ("bool".to_string(), None),
        "Timestamp" => ("crate::timestamp::Timestamp".to_string(), None),
        x => {
            if let Some(p) = hand_implemented_sdl_input_rust_type(x) {
                (p.to_string(), Some(x.to_string()))
            } else {
                (rust_ty(x), Some(x.to_string()))
            }
        }
    }
}
