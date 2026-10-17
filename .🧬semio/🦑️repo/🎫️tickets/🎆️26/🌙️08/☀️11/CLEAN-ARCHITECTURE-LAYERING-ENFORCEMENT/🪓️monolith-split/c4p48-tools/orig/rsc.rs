//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

//! 🏗️ The repo GraphQL schema: the type vocabulary, the repo schema builders and the SDL rendering. A leaf of
//! `semio-framework-repo-graphql`, which executes documents against it. The committed SDL
//! `🔗️graphql/🧪️tests/🧫️fixtures/📜️served-schema/🔗️.graphql` is the oracle shared with the Go twin in
//! `🔗️graphql/📦️packages/🐹️go/🐹️.go`.

//#endregion 🧲️Header

use serde_json::{json, Value as Json};
use std::collections::BTreeMap;

/// 🔌️ Explicit re-export: the argument defaults and the schema inventory speak `serde_json`, so a client of
/// this crate reaches that vocabulary through this crate and never declares the dependency itself.
pub use serde_json;

//#region 📜️Schema

/// 🗺️ A type reference in a field, argument or input-field position. Mirrors the reference
/// implementation's `NonNull`/`List` wrappers around a named type, except that the name is
/// resolved against [`Schema::types`] instead of a pointer, so the graph may be cyclic without a
/// thunk.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TypeRef {
    /// 🏷️ A named type, resolved against the schema's type table.
    Named(String),
    /// ❗️ A wrapper asserting the value is never null.
    NonNull(Box<TypeRef>),
    /// 📃️ A wrapper for a homogeneous sequence.
    List(Box<TypeRef>),
}

impl TypeRef {
    /// 🏷️ The innermost named type, ignoring every wrapper.
    pub fn type_name(&self) -> &str {
        match self {
            TypeRef::Named(name) => name,
            TypeRef::NonNull(inner) | TypeRef::List(inner) => inner.type_name(),
        }
    }

    /// 📜️ The SDL rendering of this reference.
    pub fn render(&self) -> String {
        match self {
            TypeRef::Named(name) => name.clone(),
            TypeRef::NonNull(inner) => format!("{}!", inner.render()),
            TypeRef::List(inner) => format!("[{}]", inner.render()),
        }
    }
}

/// 🏷️ A named type reference.
pub(crate) fn ty(name: &str) -> TypeRef {
    TypeRef::Named(name.to_string())
}

/// ❗️ Wraps a reference as non-null.
fn nn(inner: TypeRef) -> TypeRef {
    TypeRef::NonNull(Box::new(inner))
}

/// 📃️ Wraps a reference as a list.
fn lst(inner: TypeRef) -> TypeRef {
    TypeRef::List(Box::new(inner))
}

/// 📃️ The `[T!]!` shape every collection field of this schema uses.
fn nn_list(inner: TypeRef) -> TypeRef {
    nn(lst(nn(inner)))
}

/// 🎚️ One declared field argument and its default.
#[derive(Clone, Debug, PartialEq)]
pub struct Argument {
    /// 🏷️ The argument name.
    pub name: String,
    /// 🗺️ The declared argument type.
    pub ty: TypeRef,
    /// 🎁️ The declared default, `Json::Null` when there is none.
    pub default: Json,
}

/// 🧩️ One selectable field of an object or interface type.
#[derive(Clone, Debug, PartialEq)]
pub struct Field {
    /// 🏷️ The field name.
    pub name: String,
    /// 🗺️ The declared field type.
    pub ty: TypeRef,
    /// 🎚️ The declared arguments, in declaration order.
    pub args: Vec<Argument>,
    /// 🔑️ The resolver key, or `None` when the field is read off the source.
    pub resolver: Option<String>,
}

impl Field {
    /// 🧩️ A field resolved by reading the source object.
    pub(crate) fn new(name: &str, ty: TypeRef) -> Field {
        Field { name: name.to_string(), ty, args: Vec::new(), resolver: None }
    }

    /// 🔑️ A field resolved by the named resolver.
    fn resolved(name: &str, ty: TypeRef, resolver: &str) -> Field {
        Field { name: name.to_string(), ty, args: Vec::new(), resolver: Some(resolver.to_string()) }
    }

    /// 🎚️ Declares one argument without a default.
    fn arg(mut self, name: &str, ty: TypeRef) -> Field {
        self.args.push(Argument { name: name.to_string(), ty, default: Json::Null });
        self
    }
}

/// 🏛️ A composite type with a field table.
#[derive(Clone, Debug, PartialEq)]
pub struct ObjectType {
    /// 🏷️ The type name.
    pub name: String,
    /// 🎭️ The interfaces this object declares.
    pub interfaces: Vec<String>,
    /// 🧩️ The field table.
    pub fields: Vec<Field>,
}

impl ObjectType {
    /// 🧩️ Looks one field up by name.
    pub fn field(&self, name: &str) -> Option<&Field> {
        self.fields.iter().find(|field| field.name == name)
    }
}

/// 🎭️ An abstract type resolved to a concrete object at execution time.
#[derive(Clone, Debug, PartialEq)]
pub struct InterfaceType {
    /// 🏷️ The type name.
    pub name: String,
    /// 🧩️ The field table every implementor carries.
    pub fields: Vec<Field>,
}

/// 🎭️ A closed set of object types resolved at execution time.
#[derive(Clone, Debug, PartialEq)]
pub struct UnionType {
    /// 🏷️ The type name.
    pub name: String,
    /// 🏛️ The member object type names.
    pub types: Vec<String>,
}

/// 🔢️ A closed set of named constants and the wire value each stands for.
#[derive(Clone, Debug, PartialEq)]
pub struct EnumType {
    /// 🏷️ The type name.
    pub name: String,
    /// 🔢️ Member name to wire value, in declaration order.
    pub values: Vec<(String, String)>,
}

/// 📥️ A composite type accepted as an argument.
#[derive(Clone, Debug, PartialEq)]
pub struct InputObjectType {
    /// 🏷️ The type name.
    pub name: String,
    /// 📥️ The input field table, in declaration order.
    pub fields: Vec<(String, TypeRef)>,
}

/// 📜️ Anything nameable in the owned schema vocabulary.
#[derive(Clone, Debug, PartialEq)]
pub enum NamedType {
    /// 🔤️ A leaf type serialised straight into the result.
    Scalar(String),
    /// 🏛️ A composite type with a field table.
    Object(ObjectType),
    /// 🎭️ An abstract type with a field table.
    Interface(InterfaceType),
    /// 🎭️ A closed set of object types.
    Union(UnionType),
    /// 🔢️ A closed set of named constants.
    Enum(EnumType),
    /// 📥️ A composite argument type.
    InputObject(InputObjectType),
}

impl NamedType {
    /// 🏷️ The name this type is registered under.
    pub fn name(&self) -> &str {
        match self {
            NamedType::Scalar(name) => name,
            NamedType::Object(object) => &object.name,
            NamedType::Interface(interface) => &interface.name,
            NamedType::Union(union) => &union.name,
            NamedType::Enum(enumeration) => &enumeration.name,
            NamedType::InputObject(input) => &input.name,
        }
    }
}

/// 🗺️ The executable schema: a type table plus the operation roots.
#[derive(Clone, Debug, PartialEq)]
pub struct Schema {
    /// 📚️ Every declared type, by name.
    pub types: BTreeMap<String, NamedType>,
    /// 🔍️ The query root type name.
    pub query: String,
    /// ✍️ The mutation root type name, when one is configured.
    pub mutation: Option<String>,
}

impl Schema {
    /// 🏛️ Looks one object type up by name.
    pub fn object(&self, name: &str) -> Option<&ObjectType> {
        match self.types.get(name) {
            Some(NamedType::Object(object)) => Some(object),
            _ => None,
        }
    }
}

/// 🔤️ The five scalar names the repo schema is built from.
const SCALARS: &[&str] = &["String", "Int", "Boolean", "ID", "DateTime"];

//#endregion 📜️Schema

//#region 🏗️SchemaCommon

/// 🔢️ The enum types the repo schema declares.
fn schema_enums() -> Vec<NamedType> {
    let enumeration = |name: &str, values: &[(&str, &str)]| {
        NamedType::Enum(EnumType { name: name.to_string(), values: values.iter().map(|(member, wire)| ((*member).to_string(), (*wire).to_string())).collect() })
    };
    vec![
        enumeration("DefinitionKind", &[("IMPLEMENTATION", "implementation"), ("INTERFACE", "interface"), ("CONSTANT", "constant")]),
        enumeration("BundleKind", &[("LIBRARY", "library"), ("SCHEMA", "schema"), ("BINARY", "binary"), ("UI", "ui"), ("SITE", "site"), ("ASSETS", "assets"), ("REPO", "repo")]),
        enumeration("FolderKind", &[("ORGANIZATION", "organization"), ("REQUIRED", "required")]),
        enumeration("TicketStatus", &[("OPEN", "open"), ("CLOSED", "closed")]),
        enumeration(
            "TicketClient",
            &[
                ("COPILOT_CHAT", "copilot-chat"),
                ("WINDSURF", "windsurf"),
                ("WINDSURF_CHAT", "windsurf-chat"),
                ("ANTIGRAVITY", "antigravity"),
                ("ANTIGRAVITY_CHAT", "antigravity-chat"),
                ("CURSOR", "cursor"),
                ("CURSOR_CHAT", "cursor-chat"),
                ("VSCODE", "vscode"),
                ("CLAUDE_CODE", "claude-code"),
                ("CODEX", "codex"),
                ("DROID", "droid"),
                ("KIRO_CLI", "kiro-cli"),
            ],
        ),
        enumeration("BreachPriority", &[("HIGH", "high"), ("MEDIUM", "medium"), ("LOW", "low")]),
        enumeration(
            "FileKind",
            &[
                ("CODE", "code"),
                ("SCRIPT", "script"),
                ("CONFIG", "config"),
                ("LAB", "lab"),
                ("DOCS", "docs"),
                ("RESOURCE", "resource"),
                ("TEMPLATE", "template"),
                ("LICENSE", "license"),
            ],
        ),
    ]
}

/// 🔢️ The metric objects every aggregate shares.
fn schema_metrics() -> Vec<NamedType> {
    vec![
        NamedType::Object(ObjectType {
            name: "Range".to_string(),
            interfaces: Vec::new(),
            fields: vec![Field::new("start", nn(ty("Int"))), Field::new("end", nn(ty("Int")))],
        }),
        NamedType::Object(ObjectType {
            name: "CountMetrics".to_string(),
            interfaces: Vec::new(),
            fields: vec![Field::new("added", nn(ty("Int"))), Field::new("updated", nn(ty("Int"))), Field::new("removed", nn(ty("Int")))],
        }),
        NamedType::Object(ObjectType {
            name: "PriorityCount".to_string(),
            interfaces: Vec::new(),
            fields: vec![Field::new("high", nn(ty("Int"))), Field::new("medium", nn(ty("Int"))), Field::new("low", nn(ty("Int")))],
        }),
        NamedType::Object(ObjectType {
            name: "AnalyzeMetrics".to_string(),
            interfaces: Vec::new(),
            fields: vec![Field::new("total", nn(ty("Int"))), Field::new("byPriority", ty("PriorityCount")), Field::new("autofixable", nn(ty("Int")))],
        }),
        NamedType::Object(ObjectType {
            name: "AnalyzeResult".to_string(),
            interfaces: Vec::new(),
            fields: vec![Field::new("breachs", nn_list(ty("Breach"))), Field::new("metrics", nn(ty("AnalyzeMetrics")))],
        }),
    ]
}

//#endregion 🏗️SchemaCommon

//#region 🏗️SchemaRepo

/// 💠️ The repository root type: every aggregate reachable from one query.
fn schema_repo() -> Vec<NamedType> {
    vec![NamedType::Object(ObjectType {
        name: "Repo".to_string(),
        interfaces: Vec::new(),
        fields: vec![
            Field::new("id", nn(ty("ID"))),
            Field::new("name", nn(ty("String"))),
            Field::new("path", nn(ty("String"))),
            Field::resolved("technologies", nn_list(ty("Technology")), "technologies"),
            Field::resolved("checkpoints", nn_list(ty("Checkpoint")), "checkpoints").arg("limit", ty("Int")),
            Field::resolved("bundles", nn_list(ty("Bundle")), "bundles"),
            Field::resolved("folders", nn_list(ty("Folder")), "folders"),
            Field::resolved("files", nn_list(ty("File")), "files"),
            Field::resolved("sections", nn_list(ty("Section")), "sections"),
            Field::resolved("definitions", nn_list(ty("Definition")), "definitions"),
            Field::resolved("contributors", nn_list(ty("Contributor")), "contributors"),
            Field::resolved("goals", nn_list(ty("Goal")), "goals"),
            Field::resolved("tickets", nn_list(ty("Ticket")), "tickets")
                .arg("year", ty("Int"))
                .arg("month", ty("Int"))
                .arg("day", ty("Int"))
                .arg("status", ty("TicketStatus")),
            Field::resolved("policies", nn_list(ty("Policy")), "policies"),
            Field::resolved("statutes", nn_list(ty("Statute")), "statutes"),
            Field::resolved("breachs", nn_list(ty("Breach")), "breachs").arg("scope", ty("String")),
        ],
    })]
}

//#endregion 🏗️SchemaRepo

//#region 🏗️SchemaTechnology

/// 📜️ The technology type and the package it publishes.
fn schema_technology() -> Vec<NamedType> {
    vec![
        NamedType::Object(ObjectType {
            name: "Technology".to_string(),
            interfaces: Vec::new(),
            fields: vec![
                Field::new("id", nn(ty("ID"))),
                Field::new("name", nn(ty("String"))),
                Field::new("root", nn(ty("String"))),
                Field::new("kind", nn(ty("String"))),
                Field::new("bundles", nn_list(ty("Bundle"))),
                Field::new("uri", nn(ty("String"))),
            ],
        }),
        NamedType::Object(ObjectType {
            name: "Package".to_string(),
            interfaces: Vec::new(),
            fields: vec![
                Field::new("name", nn(ty("String"))),
                Field::new("version", nn(ty("String"))),
                Field::new("path", nn(ty("String"))),
                Field::new("kind", nn(ty("String"))),
            ],
        }),
    ]
}

//#endregion 🏗️SchemaTechnology

//#region 🏗️SchemaBundle

/// 🔵️ The bundle type.
fn schema_bundle() -> Vec<NamedType> {
    vec![NamedType::Object(ObjectType {
        name: "Bundle".to_string(),
        interfaces: Vec::new(),
        fields: vec![
            Field::new("id", nn(ty("ID"))),
            Field::new("name", nn(ty("String"))),
            Field::new("root", nn(ty("String"))),
            Field::new("sourceRoot", ty("String")),
            Field::new("projectType", ty("String")),
            Field::new("tags", nn_list(ty("String"))),
            Field::new("packages", nn_list(ty("Package"))),
            Field::new("kind", nn(ty("String"))),
            Field::new("uri", nn(ty("String"))),
            Field::resolved("folders", nn_list(ty("Folder")), "empty-list"),
            Field::resolved("files", nn_list(ty("File")), "empty-list"),
            Field::resolved("breachs", nn_list(ty("Breach")), "empty-list"),
        ],
    })]
}

//#endregion 🏗️SchemaBundle

//#region 🏗️SchemaFolder

/// 🩷️ The folder type.
fn schema_folder() -> Vec<NamedType> {
    vec![NamedType::Object(ObjectType {
        name: "Folder".to_string(),
        interfaces: Vec::new(),
        fields: vec![
            Field::new("id", nn(ty("ID"))),
            Field::new("path", nn(ty("String"))),
            Field::new("uri", nn(ty("String"))),
            Field::new("name", nn(ty("String"))),
            Field::new("kind", nn(ty("String"))),
            Field::new("parent", ty("Folder")),
            Field::resolved("children", nn_list(ty("Folder")), "folder.children"),
            Field::resolved("files", nn_list(ty("File")), "folder.files"),
            Field::new("ignored", nn(ty("Boolean"))),
            Field::new("generated", nn(ty("Boolean"))),
            Field::new("bundle", ty("Bundle")),
            Field::resolved("breachs", nn_list(ty("Breach")), "empty-list"),
        ],
    })]
}

//#endregion 🏗️SchemaFolder

//#region 🏗️SchemaFile

/// 📄️ The file type.
fn schema_file() -> Vec<NamedType> {
    vec![NamedType::Object(ObjectType {
        name: "File".to_string(),
        interfaces: Vec::new(),
        fields: vec![
            Field::new("id", nn(ty("ID"))),
            Field::new("path", nn(ty("String"))),
            Field::new("uri", nn(ty("String"))),
            Field::new("name", nn(ty("String"))),
            Field::new("extension", nn(ty("String"))),
            Field::new("folder", ty("Folder")),
            Field::new("kind", nn(ty("String"))),
            Field::new("ignored", nn(ty("Boolean"))),
            Field::new("generated", nn(ty("Boolean"))),
            Field::new("bundle", ty("Bundle")),
            Field::resolved("sections", lst(ty("Section")), "file.sections"),
            Field::resolved("definitions", lst(ty("Definition")), "file.definitions"),
            Field::resolved("breachs", lst(ty("Breach")), "empty-list"),
            Field::new("content", ty("String")),
            Field::resolved("contributors", nn_list(ty("Contributor")), "empty-list"),
        ],
    })]
}

//#endregion 🏗️SchemaFile

//#region 🏗️SchemaSection

/// 💗️ The section type and the abstract item a section contains.
fn schema_section() -> Vec<NamedType> {
    vec![
        NamedType::Interface(InterfaceType {
            name: "SectionItem".to_string(),
            fields: vec![Field::new("id", nn(ty("ID"))), Field::new("name", nn(ty("String"))), Field::new("range", ty("Range"))],
        }),
        NamedType::Object(ObjectType {
            name: "Section".to_string(),
            interfaces: vec!["SectionItem".to_string()],
            fields: vec![
                Field::new("id", nn(ty("ID"))),
                Field::new("name", nn(ty("String"))),
                Field::new("path", nn(ty("String"))),
                Field::resolved("file", ty("File"), "item.file"),
                Field::new("parent", ty("Section")),
                Field::resolved("children", lst(ty("SectionItem")), "section.children"),
                Field::new("definitions", lst(ty("Definition"))),
                Field::resolved("breachs", lst(ty("Breach")), "empty-list"),
                Field::new("range", nn(ty("Range"))),
            ],
        }),
    ]
}

//#endregion 🏗️SchemaSection

//#region 🏗️SchemaDefinition

/// 💕️ The definition type.
fn schema_definition() -> Vec<NamedType> {
    vec![NamedType::Object(ObjectType {
        name: "Definition".to_string(),
        interfaces: vec!["SectionItem".to_string()],
        fields: vec![
            Field::new("id", nn(ty("ID"))),
            Field::new("name", nn(ty("String"))),
            Field::new("kind", nn(ty("DefinitionKind"))),
            Field::resolved("file", nn(ty("File")), "item.file"),
            Field::resolved("section", ty("Section"), "definition.section"),
            Field::resolved("breachs", nn_list(ty("Breach")), "empty-list"),
            Field::new("range", nn(ty("Range"))),
        ],
    })]
}

//#endregion 🏗️SchemaDefinition

//#region 🏗️SchemaStatute

/// 📜️ The statute type.
fn schema_statute() -> Vec<NamedType> {
    vec![NamedType::Object(ObjectType {
        name: "Statute".to_string(),
        interfaces: Vec::new(),
        fields: vec![
            Field::new("id", nn(ty("ID"))),
            Field::resolved("policy", nn(ty("Policy")), "statute.policy"),
            Field::new("priority", nn(ty("BreachPriority"))),
            Field::new("autofixable", nn(ty("Boolean"))),
            Field::new("reason", nn(ty("String"))),
            Field::new("solution", nn(ty("String"))),
            Field::new("breachs", nn_list(ty("Breach"))).arg("scope", ty("String")),
        ],
    })]
}

//#endregion 🏗️SchemaStatute

//#region 🏗️SchemaBreach

/// 🔶️ The breach type.
fn schema_breach() -> Vec<NamedType> {
    vec![NamedType::Object(ObjectType {
        name: "Breach".to_string(),
        interfaces: Vec::new(),
        fields: vec![
            Field::new("id", nn(ty("ID"))),
            Field::new("kindId", nn(ty("ID"))),
            Field::new("kind", nn(ty("Statute"))),
            Field::new("scope", nn(ty("String"))),
            Field::new("file", ty("File")),
            Field::new("folder", ty("Folder")),
            Field::new("line", ty("Int")),
            Field::new("column", ty("Int")),
            Field::new("excerpt", ty("String")),
            Field::new("summary", nn(ty("String"))),
            Field::new("priority", nn(ty("BreachPriority"))),
            Field::new("autofixable", nn(ty("Boolean"))),
        ],
    })]
}

//#endregion 🏗️SchemaBreach

//#region 🏗️SchemaPolicy

/// 👮️ The policy and territory types.
fn schema_policy() -> Vec<NamedType> {
    vec![
        NamedType::Object(ObjectType {
            name: "Territory".to_string(),
            interfaces: Vec::new(),
            fields: vec![
                Field::new("name", nn(ty("String"))),
                Field::new("description", nn(ty("String"))),
                Field::new("scopes", nn_list(ty("String"))),
                Field::new("groups", nn_list(ty("Territory"))),
                Field::new("kinds", nn_list(ty("Statute"))),
            ],
        }),
        NamedType::Object(ObjectType {
            name: "Policy".to_string(),
            interfaces: Vec::new(),
            fields: vec![
                Field::new("id", nn(ty("ID"))),
                Field::new("name", nn(ty("String"))),
                Field::new("description", ty("String")),
                Field::new("scopes", nn_list(ty("String"))),
                Field::new("groups", nn_list(ty("Territory"))),
                Field::new("statutes", nn_list(ty("Statute"))),
            ],
        }),
    ]
}

//#endregion 🏗️SchemaPolicy

//#region 🏗️SchemaInteraction

/// 💬️ The interaction types.
fn schema_interaction() -> Vec<NamedType> {
    vec![
        NamedType::Object(ObjectType {
            name: "Interaction".to_string(),
            interfaces: Vec::new(),
            fields: vec![
                Field::new("kind", nn(ty("String"))),
                Field::new("prompt", nn(ty("String"))),
                Field::new("checkpoint", ty("String")),
                Field::new("llm", ty("String")),
                Field::new("effort", ty("String")),
                Field::new("date", nn(ty("String"))),
                Field::new("system", nn(ty("String"))),
                Field::new("client", nn(ty("String"))),
                Field::new("author", nn(ty("String"))),
            ],
        }),
        NamedType::Object(ObjectType {
            name: "InteractionResource".to_string(),
            interfaces: Vec::new(),
            fields: vec![
                Field::new("kind", nn(ty("String"))),
                Field::new("prompt", nn(ty("String"))),
                Field::new("checkpoint", ty("String")),
                Field::new("author", nn(ty("String"))),
                Field::new("sourceKind", nn(ty("String"))),
                Field::new("sourceId", nn(ty("String"))),
                Field::new("goalId", ty("String")),
                Field::new("ticketId", ty("String")),
                Field::new("llm", ty("String")),
                Field::new("effort", ty("String")),
                Field::new("date", nn(ty("String"))),
                Field::new("system", nn(ty("String"))),
                Field::new("client", nn(ty("String"))),
            ],
        }),
    ]
}

//#endregion 🏗️SchemaInteraction

//#region 🏗️SchemaGoal

/// 🎯️ The goal type.
fn schema_goal() -> Vec<NamedType> {
    vec![NamedType::Object(ObjectType {
        name: "Goal".to_string(),
        interfaces: Vec::new(),
        fields: vec![
            Field::new("id", nn(ty("ID"))),
            Field::new("title", nn(ty("String"))),
            Field::new("description", ty("String")),
            Field::new("prompt", ty("String")),
            Field::new("dueDate", ty("String")),
            Field::new("createdAt", ty("String")),
            Field::new("client", ty("String")),
            Field::new("llm", ty("String")),
            Field::new("effort", ty("String")),
            Field::new("status", nn(ty("String"))),
            Field::new("milestone", ty("Int")),
            Field::new("issue", ty("String")),
            Field::new("parent", ty("String")),
            Field::new("interactions", nn_list(ty("Interaction"))),
        ],
    })]
}

//#endregion 🏗️SchemaGoal

//#region 🏗️SchemaDraft

/// 📝️ The draft type.
fn schema_draft() -> Vec<NamedType> {
    vec![NamedType::Object(ObjectType { name: "Draft".to_string(), interfaces: Vec::new(), fields: vec![Field::new("id", nn(ty("ID")))] })]
}

//#endregion 🏗️SchemaDraft

//#region 🏗️SchemaTodo

/// ✅️ The todo type and the position it points at.
fn schema_todo() -> Vec<NamedType> {
    vec![
        NamedType::Object(ObjectType {
            name: "Location".to_string(),
            interfaces: Vec::new(),
            fields: vec![Field::new("filePath", ty("String")), Field::new("line", ty("Int")), Field::new("column", ty("Int"))],
        }),
        NamedType::Object(ObjectType {
            name: "Todo".to_string(),
            interfaces: Vec::new(),
            fields: vec![
                Field::new("id", nn(ty("ID"))),
                Field::new("name", nn(ty("String"))),
                Field::new("description", ty("String")),
                Field::new("parentId", nn(ty("ID"))),
                Field::new("location", ty("Location")),
            ],
        }),
    ]
}

//#endregion 🏗️SchemaTodo

//#region 🏗️SchemaTicket

/// 🎫️ The ticket type and the date pair it carries.
fn schema_ticket() -> Vec<NamedType> {
    vec![
        NamedType::Object(ObjectType {
            name: "TicketDate".to_string(),
            interfaces: Vec::new(),
            fields: vec![Field::new("started", nn(ty("DateTime"))), Field::new("finished", ty("DateTime"))],
        }),
        NamedType::Object(ObjectType {
            name: "Ticket".to_string(),
            interfaces: Vec::new(),
            fields: vec![
                Field::new("id", nn(ty("ID"))),
                Field::new("year", nn(ty("Int"))),
                Field::new("month", nn(ty("Int"))),
                Field::new("day", nn(ty("Int"))),
                Field::new("slug", nn(ty("String"))),
                Field::new("path", nn(ty("String"))),
                Field::new("llm", ty("String")),
                Field::new("effort", ty("String")),
                Field::new("client", ty("TicketClient")),
                Field::new("checkpoint", ty("String")),
                Field::new("uri", nn(ty("String"))),
                Field::new("title", nn(ty("String"))),
                Field::new("emoji", ty("String")),
                Field::new("prompt", nn(ty("String"))),
                Field::new("summary", ty("String")),
                Field::new("status", nn(ty("TicketStatus"))),
                Field::new("interactions", nn_list(ty("Interaction"))),
                Field::resolved("author", ty("Contributor"), "ticket.author"),
                Field::new("dates", nn(ty("TicketDate"))),
                Field::new("goal", ty("String")),
                Field::new("parent", ty("String")),
                Field::new("bundles", nn_list(ty("Bundle"))),
                Field::new("files", nn_list(ty("File"))),
            ],
        }),
        NamedType::Object(ObjectType {
            name: "TicketDay".to_string(),
            interfaces: Vec::new(),
            fields: vec![Field::new("day", nn(ty("Int"))), Field::new("tickets", nn_list(ty("Ticket")))],
        }),
        NamedType::Object(ObjectType {
            name: "TicketMonth".to_string(),
            interfaces: Vec::new(),
            fields: vec![Field::new("month", nn(ty("Int"))), Field::new("days", nn_list(ty("TicketDay")))],
        }),
        NamedType::Object(ObjectType {
            name: "TicketYear".to_string(),
            interfaces: Vec::new(),
            fields: vec![Field::new("year", nn(ty("Int"))), Field::new("months", nn_list(ty("TicketMonth")))],
        }),
    ]
}

//#endregion 🏗️SchemaTicket

//#region 🏗️SchemaCheckpoint

/// ✔️ The checkpoint type.
fn schema_checkpoint() -> Vec<NamedType> {
    vec![NamedType::Object(ObjectType {
        name: "Checkpoint".to_string(),
        interfaces: Vec::new(),
        fields: vec![
            Field::new("id", nn(ty("ID"))),
            Field::new("sha", nn(ty("String"))),
            Field::new("title", nn(ty("String"))),
            Field::new("date", nn(ty("DateTime"))),
        ],
    })]
}

//#endregion 🏗️SchemaCheckpoint

//#region 🏗️SchemaContributor

/// 🧑️‍💻️ The contributor type and its contribution tree.
fn schema_contributor() -> Vec<NamedType> {
    let named = |name: &str, child: &str, plural: &str| {
        NamedType::Object(ObjectType {
            name: name.to_string(),
            interfaces: Vec::new(),
            fields: vec![Field::new("name", nn(ty("String"))), Field::new(plural, nn_list(ty(child)))],
        })
    };
    vec![
        NamedType::Object(ObjectType {
            name: "ContributorIcons".to_string(),
            interfaces: Vec::new(),
            fields: vec![Field::new("avatar", ty("String")), Field::new("avatarRound", ty("String")), Field::new("github", ty("String"))],
        }),
        NamedType::Object(ObjectType {
            name: "ContributorLink".to_string(),
            interfaces: Vec::new(),
            fields: vec![Field::new("name", nn(ty("String"))), Field::new("url", nn(ty("String")))],
        }),
        NamedType::Object(ObjectType {
            name: "ContributorDefinition".to_string(),
            interfaces: Vec::new(),
            fields: vec![Field::new("name", nn(ty("String")))],
        }),
        named("ContributorSection", "ContributorDefinition", "definitions"),
        named("ContributorFile", "ContributorSection", "sections"),
        named("ContributorFolder", "ContributorFile", "files"),
        named("ContributorBundle", "ContributorFolder", "folders"),
        NamedType::Object(ObjectType {
            name: "ContributorContributions".to_string(),
            interfaces: Vec::new(),
            fields: vec![
                Field::new("checkpoints", nn_list(ty("Checkpoint"))),
                Field::new("tickets", nn_list(ty("TicketYear"))),
                Field::new("bundles", nn_list(ty("ContributorBundle"))),
            ],
        }),
        NamedType::Object(ObjectType {
            name: "Contributor".to_string(),
            interfaces: Vec::new(),
            fields: vec![
                Field::new("id", nn(ty("ID"))),
                Field::new("github", nn(ty("String"))),
                Field::new("emoji", ty("String")),
                Field::new("name", nn(ty("String"))),
                Field::new("names", nn_list(ty("String"))),
                Field::new("email", nn(ty("String"))),
                Field::new("emails", nn_list(ty("String"))),
                Field::new("fingerprint", ty("String")),
                Field::new("fingerprints", nn_list(ty("String"))),
                Field::new("links", nn_list(ty("ContributorLink"))),
                Field::resolved("contributions", ty("ContributorContributions"), "contributor.contributions"),
                Field::new("icons", ty("ContributorIcons")),
                Field::new("bundles", nn_list(ty("Bundle"))),
                Field::new("files", nn_list(ty("File"))),
                Field::new("tickets", nn_list(ty("Ticket"))),
            ],
        }),
    ]
}

//#endregion 🏗️SchemaContributor

//#region 🏗️SchemaInputs

/// 📥️ Every input object the query and mutation roots accept.
fn schema_inputs() -> Vec<NamedType> {
    let input = |name: &str, fields: Vec<(&str, TypeRef)>| {
        NamedType::InputObject(InputObjectType { name: name.to_string(), fields: fields.into_iter().map(|(field, kind)| (field.to_string(), kind)).collect() })
    };
    vec![
        input(
            "FilterInput",
            vec![
                ("filter", ty("String")),
                ("regex", ty("Boolean")),
                ("matchCase", ty("Boolean")),
                ("matchWholeWord", ty("Boolean")),
                ("showIgnored", ty("Boolean")),
                ("showGenerated", ty("Boolean")),
                ("excludeKinds", lst(ty("FileKind"))),
                ("includeKinds", lst(ty("FileKind"))),
            ],
        ),
        input("DraftCreateInput", vec![("title", nn(ty("String"))), ("files", lst(nn(ty("String"))))]),
        input(
            "TicketOpenInput",
            vec![
                ("emoji", nn(ty("String"))),
                ("title", nn(ty("String"))),
                ("prompt", nn(ty("String"))),
                ("llm", ty("String")),
                ("effort", ty("String")),
                ("client", nn(ty("TicketClient"))),
                ("noIssue", ty("Boolean")),
                ("draft", ty("String")),
                ("goal", nn(ty("String"))),
                ("parent", ty("String")),
                ("noManagement", ty("Boolean")),
                ("issue", ty("String")),
                ("planId", ty("String")),
                ("specId", ty("String")),
            ],
        ),
        input(
            "TicketCloseInput",
            vec![
                ("year", ty("Int")),
                ("month", ty("Int")),
                ("day", ty("Int")),
                ("slug", ty("String")),
                ("summary", ty("String")),
                ("files", lst(nn(ty("String")))),
                ("title", ty("String")),
                ("noManagement", ty("Boolean")),
                ("all", ty("Boolean")),
            ],
        ),
        input(
            "TicketReopenInput",
            vec![
                ("year", nn(ty("Int"))),
                ("month", nn(ty("Int"))),
                ("day", nn(ty("Int"))),
                ("slug", nn(ty("String"))),
                ("prompt", nn(ty("String"))),
                ("client", nn(ty("TicketClient"))),
                ("llm", ty("String")),
                ("effort", ty("String")),
                ("title", ty("String")),
                ("draft", ty("String")),
                ("goal", ty("String")),
                ("parent", ty("String")),
                ("noManagement", ty("Boolean")),
                ("planId", ty("String")),
                ("specId", ty("String")),
            ],
        ),
        input(
            "TicketChangeInput",
            vec![
                ("year", nn(ty("Int"))),
                ("month", nn(ty("Int"))),
                ("day", nn(ty("Int"))),
                ("slug", nn(ty("String"))),
                ("title", ty("String")),
                ("prompt", ty("String")),
                ("llm", ty("String")),
                ("effort", ty("String")),
                ("client", ty("TicketClient")),
                ("goal", ty("String")),
                ("parent", ty("String")),
                ("noManagement", ty("Boolean")),
            ],
        ),
        input("TodoCreateInput", vec![("name", nn(ty("String"))), ("description", nn(ty("String"))), ("parentId", nn(ty("String")))]),
        input("TodoChangeInput", vec![("id", nn(ty("String"))), ("name", ty("String")), ("description", ty("String"))]),
        input(
            "GoalCreateInput",
            vec![
                ("title", nn(ty("String"))),
                ("description", nn(ty("String"))),
                ("prompt", nn(ty("String"))),
                ("dueDate", nn(ty("String"))),
                ("llm", nn(ty("String"))),
                ("effort", ty("String")),
                ("client", nn(ty("String"))),
                ("parent", ty("String")),
                ("noManagement", ty("Boolean")),
                ("milestone", ty("String")),
            ],
        ),
        input(
            "GoalChangeInput",
            vec![
                ("id", nn(ty("String"))),
                ("title", ty("String")),
                ("description", ty("String")),
                ("dueDate", ty("String")),
                ("llm", ty("String")),
                ("effort", ty("String")),
                ("parent", ty("String")),
                ("noManagement", ty("Boolean")),
            ],
        ),
        input("GoalCloseInput", vec![("id", nn(ty("String"))), ("summary", nn(ty("String"))), ("noManagement", ty("Boolean"))]),
        input(
            "GoalReopenInput",
            vec![
                ("id", nn(ty("String"))),
                ("prompt", nn(ty("String"))),
                ("client", nn(ty("String"))),
                ("llm", nn(ty("String"))),
                ("effort", ty("String")),
                ("title", ty("String")),
                ("description", ty("String")),
                ("dueDate", ty("String")),
                ("parent", ty("String")),
                ("noManagement", ty("Boolean")),
            ],
        ),
        input(
            "ContributorAddInput",
            vec![
                ("github", nn(ty("String"))),
                ("name", ty("String")),
                ("names", lst(nn(ty("String")))),
                ("email", ty("String")),
                ("emails", lst(nn(ty("String")))),
                ("fingerprint", ty("String")),
                ("fingerprints", lst(nn(ty("String")))),
            ],
        ),
    ]
}

//#endregion 🏗️SchemaInputs

//#region 🏗️SchemaRoots

/// 🔍️ The query root and the node union it discriminates.
fn schema_query() -> Vec<NamedType> {
    vec![
        NamedType::Union(UnionType {
            name: "Node".to_string(),
            types: ["Repo", "Bundle", "Folder", "File", "Section", "Definition", "Contributor", "Ticket", "Policy", "Statute", "Breach", "Draft"].iter().map(|name| (*name).to_string()).collect(),
        }),
        NamedType::Object(ObjectType {
            name: "Query".to_string(),
            interfaces: Vec::new(),
            fields: vec![
                Field::resolved("node", nn(ty("Node")), "query.node").arg("id", nn(ty("ID"))),
                Field::resolved("repo", nn(ty("Repo")), "query.repo"),
                Field::resolved("technologies", nn_list(ty("Technology")), "technologies").arg("filter", ty("FilterInput")),
                Field::resolved("bundles", nn_list(ty("Bundle")), "bundles").arg("filter", ty("FilterInput")),
                Field::resolved("folders", nn_list(ty("Folder")), "folders"),
                Field::resolved("files", nn_list(ty("File")), "files"),
                Field::resolved("sections", nn_list(ty("Section")), "sections"),
                Field::resolved("definitions", nn_list(ty("Definition")), "definitions"),
                Field::resolved("contributors", nn_list(ty("Contributor")), "contributors").arg("filter", ty("FilterInput")),
                Field::resolved("todos", nn_list(ty("Todo")), "query.todos").arg("filter", ty("FilterInput")),
                Field::resolved("tickets", nn_list(ty("Ticket")), "tickets")
                    .arg("year", ty("Int"))
                    .arg("month", ty("Int"))
                    .arg("day", ty("Int"))
                    .arg("status", ty("TicketStatus"))
                    .arg("filter", ty("FilterInput")),
                Field::resolved("interactions", nn_list(ty("InteractionResource")), "query.interactions"),
                Field::resolved("drafts", nn_list(ty("Draft")), "query.drafts"),
                Field::resolved("policies", nn_list(ty("Policy")), "policies").arg("filter", ty("FilterInput")),
                Field::resolved("statutes", nn_list(ty("Statute")), "statutes"),
                Field::resolved("breachs", nn_list(ty("Breach")), "breachs").arg("scope", ty("String")),
                Field::resolved("bundle", ty("Bundle"), "query.bundle").arg("name", nn(ty("String"))),
                Field::resolved("folder", ty("Folder"), "query.folder").arg("path", nn(ty("String"))),
                Field::resolved("file", ty("File"), "query.file").arg("path", nn(ty("String"))),
                Field::resolved("section", ty("Section"), "query.section").arg("path", nn(ty("String"))).arg("sectionPath", nn_list(ty("String"))),
                Field::resolved("definition", ty("Definition"), "query.definition").arg("path", nn(ty("String"))).arg("name", nn(ty("String"))),
                Field::resolved("contributor", ty("Contributor"), "query.contributor").arg("id", nn(ty("String"))),
                Field::resolved("ticket", ty("Ticket"), "query.ticket")
                    .arg("year", nn(ty("Int")))
                    .arg("month", nn(ty("Int")))
                    .arg("day", nn(ty("Int")))
                    .arg("slug", nn(ty("String"))),
                Field::resolved("policy", ty("Policy"), "query.policy").arg("id", nn(ty("String"))),
                Field::resolved("statute", ty("Statute"), "query.statute").arg("id", nn(ty("String"))),
                Field::resolved("analyze", nn(ty("AnalyzeResult")), "query.analyze").arg("scope", ty("String")),
            ],
        }),
    ]
}

/// ✍️ The mutation root.
fn schema_mutation() -> Vec<NamedType> {
    vec![NamedType::Object(ObjectType {
        name: "Mutation".to_string(),
        interfaces: Vec::new(),
        fields: vec![
            Field::resolved("syncManagement", nn(ty("Boolean")), "mutation.syncManagement"),
            Field::resolved("goalCreate", ty("Goal"), "mutation.goalCreate").arg("input", nn(ty("GoalCreateInput"))),
            Field::resolved("goalChange", ty("Goal"), "mutation.goalChange").arg("id", nn(ty("ID"))).arg("input", nn(ty("GoalChangeInput"))),
            Field::resolved("goalClose", ty("Goal"), "mutation.goalClose").arg("input", nn(ty("GoalCloseInput"))),
            Field::resolved("goalReopen", ty("Goal"), "mutation.goalReopen").arg("input", nn(ty("GoalReopenInput"))),
            Field::resolved("draftCreate", ty("Draft"), "mutation.draftCreate").arg("input", nn(ty("DraftCreateInput"))),
            Field::resolved("draftDelete", nn(ty("Boolean")), "mutation.draftDelete").arg("id", nn(ty("String"))),
            Field::resolved("todoCreate", ty("Todo"), "mutation.todoCreate").arg("input", nn(ty("TodoCreateInput"))),
            Field::resolved("todoChange", ty("Todo"), "mutation.todoChange").arg("input", nn(ty("TodoChangeInput"))),
            Field::resolved("todoDelete", ty("Boolean"), "mutation.todoDelete").arg("id", nn(ty("ID"))),
            Field::resolved("ticketOpen", ty("Ticket"), "mutation.ticketOpen").arg("input", nn(ty("TicketOpenInput"))),
            Field::resolved("ticketClose", ty("Ticket"), "mutation.ticketClose").arg("input", nn(ty("TicketCloseInput"))),
            Field::resolved("ticketReopen", ty("Ticket"), "mutation.ticketReopen").arg("input", nn(ty("TicketReopenInput"))),
            Field::resolved("ticketChange", ty("Ticket"), "mutation.ticketChange").arg("input", nn(ty("TicketChangeInput"))),
            Field::resolved("contributorAdd", ty("Contributor"), "mutation.contributorAdd").arg("input", nn(ty("ContributorAddInput"))),
            Field::resolved("contributorRemove", nn(ty("Boolean")), "mutation.contributorRemove").arg("github", nn(ty("String"))),
            Field::resolved("folderCreate", ty("Folder"), "mutation.folderCreate").arg("path", nn(ty("String"))),
            Field::resolved("folderMove", ty("Folder"), "mutation.folderMove").arg("src", nn(ty("String"))).arg("dst", nn(ty("String"))),
            Field::resolved("folderDelete", nn(ty("Boolean")), "mutation.folderDelete").arg("path", nn(ty("String"))),
            Field::resolved("fileCreate", ty("File"), "mutation.fileCreate").arg("path", nn(ty("String"))),
            Field::resolved("fileMove", ty("File"), "mutation.fileMove").arg("src", nn(ty("String"))).arg("dst", nn(ty("String"))),
            Field::resolved("fileDelete", nn(ty("Boolean")), "mutation.fileDelete").arg("path", nn(ty("String"))),
            Field::resolved("sectionCreate", ty("Section"), "mutation.sectionCreate").arg("file", nn(ty("String"))).arg("name", nn(ty("String"))).arg("parent", ty("String")),
            Field::resolved("sectionMove", ty("Section"), "mutation.sectionMove").arg("file", nn(ty("String"))).arg("oldName", nn(ty("String"))).arg("newName", nn(ty("String"))),
            Field::resolved("sectionDelete", nn(ty("Boolean")), "mutation.sectionDelete").arg("file", nn(ty("String"))).arg("name", nn(ty("String"))),
            Field::resolved("integrate", ty("File"), "mutation.integrate")
                .arg("source", nn(ty("String")))
                .arg("targetSection", nn(ty("String")))
                .arg("targetFile", nn(ty("String")))
                .arg("targetParent", ty("String")),
            Field::resolved("extract", ty("File"), "mutation.extract").arg("sourceFile", nn(ty("String"))).arg("sourceSection", nn(ty("String"))).arg("targetFile", nn(ty("String"))),
        ],
    })]
}

/// 🗺️ Assembles the executable schema from every per-aggregate fragment.
pub fn build_schema() -> Schema {
    let mut types: BTreeMap<String, NamedType> = BTreeMap::new();
    for scalar in SCALARS {
        types.insert((*scalar).to_string(), NamedType::Scalar((*scalar).to_string()));
    }
    let fragments = [
        schema_enums(),
        schema_metrics(),
        schema_repo(),
        schema_technology(),
        schema_bundle(),
        schema_folder(),
        schema_file(),
        schema_section(),
        schema_definition(),
        schema_statute(),
        schema_breach(),
        schema_policy(),
        schema_interaction(),
        schema_goal(),
        schema_draft(),
        schema_todo(),
        schema_ticket(),
        schema_checkpoint(),
        schema_contributor(),
        schema_inputs(),
        schema_query(),
        schema_mutation(),
    ];
    for fragment in fragments {
        for declared in fragment {
            types.insert(declared.name().to_string(), declared);
        }
    }
    Schema { types, query: "Query".to_string(), mutation: Some("Mutation".to_string()) }
}

//#endregion 🏗️SchemaRoots

//#region 📜️Sdl

/// 📜️ Collects every type reachable from the operation roots.
fn reachable(schema: &Schema) -> Vec<String> {
    let mut pending: Vec<String> = std::iter::once(schema.query.clone()).chain(schema.mutation.clone()).collect();
    let mut seen: Vec<String> = Vec::new();
    while let Some(name) = pending.pop() {
        if seen.contains(&name) {
            continue;
        }
        seen.push(name.clone());
        let Some(declared) = schema.types.get(&name) else { continue };
        let push_field = |field: &Field, pending: &mut Vec<String>| {
            pending.push(field.ty.type_name().to_string());
            for argument in &field.args {
                pending.push(argument.ty.type_name().to_string());
            }
        };
        match declared {
            NamedType::Object(object) => {
                pending.extend(object.interfaces.clone());
                for field in &object.fields {
                    push_field(field, &mut pending);
                }
            }
            NamedType::Interface(interface) => {
                for field in &interface.fields {
                    push_field(field, &mut pending);
                }
            }
            NamedType::Union(union) => pending.extend(union.types.clone()),
            NamedType::InputObject(input) => pending.extend(input.fields.iter().map(|(_, kind)| kind.type_name().to_string())),
            NamedType::Scalar(_) | NamedType::Enum(_) => {}
        }
    }
    seen.sort();
    seen
}

/// 📜️ Renders one field in SDL.
fn render_field(field: &Field) -> String {
    let arguments = if field.args.is_empty() {
        String::new()
    } else {
        format!("({})", field.args.iter().map(|argument| format!("{}: {}", argument.name, argument.ty.render())).collect::<Vec<_>>().join(", "))
    };
    format!("  {}{}: {}", field.name, arguments, field.ty.render())
}

/// 📜️ Renders the schema as the SDL both implementations must serve.
///
/// Deterministic by construction: reachable types only, sorted by name, fields in declaration
/// order — so the dump is a comparable artifact rather than a map walk.
pub fn render_sdl(schema: &Schema) -> String {
    let mut out = String::new();
    out.push_str("schema {\n");
    out.push_str(&format!("  query: {}\n", schema.query));
    if let Some(mutation) = &schema.mutation {
        out.push_str(&format!("  mutation: {mutation}\n"));
    }
    out.push_str("}\n");
    for name in reachable(schema) {
        let Some(declared) = schema.types.get(&name) else { continue };
        out.push('\n');
        match declared {
            NamedType::Scalar(scalar) => {
                if !["String", "Int", "Boolean", "ID"].contains(&scalar.as_str()) {
                    out.push_str(&format!("scalar {scalar}\n"));
                }
            }
            NamedType::Object(object) => {
                let implements = if object.interfaces.is_empty() { String::new() } else { format!(" implements {}", object.interfaces.join(" & ")) };
                out.push_str(&format!("type {}{} {{\n", object.name, implements));
                for field in &object.fields {
                    out.push_str(&render_field(field));
                    out.push('\n');
                }
                out.push_str("}\n");
            }
            NamedType::Interface(interface) => {
                out.push_str(&format!("interface {} {{\n", interface.name));
                for field in &interface.fields {
                    out.push_str(&render_field(field));
                    out.push('\n');
                }
                out.push_str("}\n");
            }
            NamedType::Union(union) => out.push_str(&format!("union {} = {}\n", union.name, union.types.join(" | "))),
            NamedType::Enum(enumeration) => {
                out.push_str(&format!("enum {} {{\n", enumeration.name));
                for (member, _) in &enumeration.values {
                    out.push_str(&format!("  {member}\n"));
                }
                out.push_str("}\n");
            }
            NamedType::InputObject(input) => {
                out.push_str(&format!("input {} {{\n", input.name));
                for (field, kind) in &input.fields {
                    out.push_str(&format!("  {}: {}\n", field, kind.render()));
                }
                out.push_str("}\n");
            }
        }
    }
    let trimmed = out.trim_end();
    format!("{trimmed}\n")
}

/// 📜️ The language-neutral inventory of the served schema: what a conforming reader finds in the
/// SDL, in the one order both implementations can agree on.
pub fn schema_inventory(schema: &Schema) -> Json {
    let field_row = |field: &Field| {
        json!({
            "name": field.name,
            "type": field.ty.render(),
            "args": field.args.iter().map(|argument| json!({ "name": argument.name, "type": argument.ty.render() })).collect::<Vec<_>>(),
        })
    };
    let types = reachable(schema)
        .into_iter()
        .filter_map(|name| schema.types.get(&name))
        .filter(|declared| !matches!(declared, NamedType::Scalar(scalar) if ["String", "Int", "Boolean", "ID"].contains(&scalar.as_str())))
        .map(|declared| match declared {
            NamedType::Scalar(scalar) => json!({ "name": scalar, "kind": "scalar" }),
            NamedType::Object(object) => json!({ "name": object.name, "kind": "object", "interfaces": object.interfaces, "fields": object.fields.iter().map(&field_row).collect::<Vec<_>>() }),
            NamedType::Interface(interface) => json!({ "name": interface.name, "kind": "interface", "fields": interface.fields.iter().map(&field_row).collect::<Vec<_>>() }),
            NamedType::Union(union) => json!({ "name": union.name, "kind": "union", "possibleTypes": union.types }),
            NamedType::Enum(enumeration) => json!({ "name": enumeration.name, "kind": "enum", "values": enumeration.values.iter().map(|(member, _)| member.clone()).collect::<Vec<_>>() }),
            NamedType::InputObject(input) => json!({ "name": input.name, "kind": "input", "fields": input.fields.iter().map(|(field, kind)| json!({ "name": field, "type": kind.render() })).collect::<Vec<_>>() }),
        })
        .collect::<Vec<_>>();
    json!({ "query": schema.query, "mutation": schema.mutation, "types": types })
}

//#endregion 📜️Sdl

//#region 🧪️Tests

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

//#endregion 🧪️Tests
