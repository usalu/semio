//! 🎯 Dynamic GraphQL schema mirrored from `semio/graphql/target.schema.graphql` (parser + overlays).

use std::sync::Arc;

use async_graphql::dynamic::{
    Enum, EnumItem, Field, FieldFuture, FieldValue, InputObject, InputValue, Interface, InterfaceField, Object, Scalar,
    Schema, SchemaBuilder, SchemaError, Subscription, SubscriptionField, SubscriptionFieldFuture, Type, TypeRef, Union,
};
use async_graphql::parser::types::{
    BaseType, FieldDefinition, InputObjectType, InputValueDefinition, ObjectType, SchemaDefinition, ServiceDocument,
    Type as ParserType, TypeDefinition, TypeKind, TypeSystemDefinition, UnionType,
};
use async_graphql::parser::{parse_schema, Positioned};
use futures_util::stream;

use crate::event::EventBus;
use crate::id::Id;
use crate::worker::ParentRuntime;

/// 📜 Canonical target SDL (must match exported `semio/graphql/schema.graphql`).
pub const TARGET_GRAPHQL_SDL: &str = include_str!("../graphql/target.schema.graphql");

fn parser_type_to_tyref(ty: &ParserType) -> TypeRef {
    let mut t = match &ty.base {
        BaseType::Named(n) => TypeRef::Named(n.as_str().to_string().into()),
        BaseType::List(inner) => TypeRef::List(Box::new(parser_type_to_tyref(inner))),
    };
    if !ty.nullable {
        t = TypeRef::NonNull(Box::new(t));
    }
    t
}

fn stub_value_for_ty(ty: &TypeRef) -> FieldValue<'static> {
    match ty {
        TypeRef::NonNull(inner) => stub_value_for_ty(inner),
        TypeRef::List(_) => FieldValue::from(Vec::<FieldValue>::new()),
        TypeRef::Named(n) => match n.as_ref() {
            "String" => FieldValue::from(String::new()),
            "Int" => FieldValue::from(0i32),
            "Float" => FieldValue::from(0.0f64),
            "Boolean" => FieldValue::from(false),
            "ID" => FieldValue::from(String::new()),
            "Timestamp" => FieldValue::from(String::new()),
            _ => FieldValue::with_type(FieldValue::NULL, n.to_string()),
        },
    }
}

fn object_field_from_ast(field: &Positioned<FieldDefinition>) -> Field {
    let name = field.node.name.node.to_string();
    let ty = parser_type_to_tyref(&field.node.ty.node);
    let ty_clone = ty.clone();
    let mut f = Field::new(name.clone(), ty_clone.clone(), move |_ctx| {
        let t = ty_clone.clone();
        FieldFuture::new(async move { Ok(Some(stub_value_for_ty(&t))) })
    });
    for arg in &field.node.arguments {
        let mut iv = InputValue::new(arg.node.name.node.to_string(), parser_type_to_tyref(&arg.node.ty.node));
        if let Some(d) = &arg.node.default_value {
            iv = iv.default_value(d.node.clone().into_value());
        }
        f = f.argument(iv);
    }
    f
}

fn object_from_ast(name: &str, o: &ObjectType) -> Object {
    let mut obj = Object::new(name);
    for ifc in &o.implements {
        obj = obj.implement(ifc.node.as_str());
    }
    for field in &o.fields {
        obj = obj.field(object_field_from_ast(field));
    }
    obj
}

fn interface_from_ast(name: &str, i: &async_graphql::parser::types::InterfaceType) -> Interface {
    let mut iface = Interface::new(name);
    for ifc in &i.implements {
        iface = iface.implement(ifc.node.as_str());
    }
    for field in &i.fields {
        let fname = field.node.name.node.to_string();
        let ty = parser_type_to_tyref(&field.node.ty.node);
        let mut ifld = InterfaceField::new(fname, ty);
        for arg in &field.node.arguments {
            let mut iv = InputValue::new(arg.node.name.node.to_string(), parser_type_to_tyref(&arg.node.ty.node));
            if let Some(d) = &arg.node.default_value {
                iv = iv.default_value(d.node.clone().into_value());
            }
            ifld = ifld.argument(iv);
        }
        iface = iface.field(ifld);
    }
    iface
}

fn union_from_ast(name: &str, u: &UnionType) -> Union {
    let mut uni = Union::new(name);
    for m in &u.members {
        uni = uni.possible_type(m.node.as_str());
    }
    uni
}

fn enum_from_ast(name: &str, e: &async_graphql::parser::types::EnumType) -> Enum {
    let mut en = Enum::new(name);
    for v in &e.values {
        en = en.item(EnumItem::new(v.node.value.node.to_string()));
    }
    en
}

fn input_object_from_ast(name: &str, io: &InputObjectType) -> InputObject {
    let mut o = InputObject::new(name);
    for f in &io.fields {
        let mut iv = InputValue::new(f.node.name.node.to_string(), parser_type_to_tyref(&f.node.ty.node));
        if let Some(d) = &f.node.default_value {
            iv = iv.default_value(d.node.clone().into_value());
        }
        o = o.field(iv);
    }
    o
}

fn input_value_from_arg_def(arg: &Positioned<InputValueDefinition>) -> InputValue {
    let mut iv = InputValue::new(arg.node.name.node.to_string(), parser_type_to_tyref(&arg.node.ty.node));
    if let Some(d) = &arg.node.default_value {
        iv = iv.default_value(d.node.clone().into_value());
    }
    iv
}

fn subscription_from_object_ast(name: &str, o: &ObjectType) -> Subscription {
    let mut sub = Subscription::new(name);
    for field in &o.fields {
        let fname = field.node.name.node.to_string();
        let ty = parser_type_to_tyref(&field.node.ty.node);
        let mut args: Vec<InputValue> = Vec::new();
        for arg in &field.node.arguments {
            args.push(input_value_from_arg_def(arg));
        }
        let mut sf = SubscriptionField::new(fname.clone(), ty_c.clone(), move |_ctx| {
            SubscriptionFieldFuture::new(async move {
                Ok(stream::empty::<async_graphql::Result<FieldValue<'static>>>().boxed())
            })
        });
        for a in args {
            sf = sf.argument(a);
        }
        sub = sub.field(sf);
    }
    sub
}

fn extract_schema_roots(doc: &ServiceDocument) -> Result<(String, Option<String>, Option<String>), SchemaError> {
    let mut query = None;
    let mut mutation = None;
    let mut subscription = None;
    for def in &doc.definitions {
        if let TypeSystemDefinition::Schema(s) = def {
            let s: &SchemaDefinition = &s.node;
            query = s.query.as_ref().map(|p| p.node.to_string());
            mutation = s.mutation.as_ref().map(|p| p.node.to_string());
            subscription = s.subscription.as_ref().map(|p| p.node.to_string());
        }
    }
    let q = query.ok_or_else(|| SchemaError::from("schema missing query root"))?;
    Ok((q, mutation, subscription))
}

fn register_definitions(
    builder: SchemaBuilder,
    doc: &ServiceDocument,
    query: &str,
    mutation: &Option<String>,
    subscription: &Option<String>,
) -> Result<SchemaBuilder, SchemaError> {
    let mut b = builder;
    for def in &doc.definitions {
        match def {
            TypeSystemDefinition::Schema(_) => {}
            TypeSystemDefinition::Directive(_) => {}
            TypeSystemDefinition::Type(td) => {
                if td.node.extend {
                    continue;
                }
                let name = td.node.name.node.to_string();
                if Some(&name) == subscription.as_ref() {
                    if let TypeKind::Object(o) = &td.node.kind {
                        b = b.register(Type::Subscription(subscription_from_object_ast(&name, o)));
                    }
                    continue;
                }
                match &td.node.kind {
                    TypeKind::Scalar => {
                        b = b.register(Type::Scalar(Scalar::new(&name)));
                    }
                    TypeKind::Object(o) => {
                        b = b.register(Type::Object(object_from_ast(&name, o)));
                    }
                    TypeKind::Interface(i) => {
                        b = b.register(Type::Interface(interface_from_ast(&name, i)));
                    }
                    TypeKind::Union(u) => {
                        b = b.register(Type::Union(union_from_ast(&name, u)));
                    }
                    TypeKind::Enum(e) => {
                        b = b.register(Type::Enum(enum_from_ast(&name, e)));
                    }
                    TypeKind::InputObject(io) => {
                        b = b.register(Type::InputObject(input_object_from_ast(&name, io)));
                    }
                }
            }
        }
    }
    let _ = query;
    let _ = mutation;
    Ok(b)
}

//#region 🪢 relay helpers

/// Wraps `Vec<Arc<Design>>` for `DesignConnection` resolution.
pub struct DesignConn(pub Vec<Arc<crate::kit::design::Design>>);

/// Wraps `Vec<Arc<Piece>>` for `PieceConnection` resolution.
pub struct PieceConn(pub Vec<Arc<crate::kit::design::piece::Piece>>);

fn design_connection_object() -> Object {
    let page = empty_page_info_object();
    Object::new("DesignConnection")
        .implement("EntityConnectionInterface")
        .field(Field::new("edges", TypeRef::named_nn_list_nn("DesignEdge"), |ctx| {
            FieldFuture::new(async move {
                let parent = ctx.parent_value.downcast_ref::<DesignConn>().unwrap();
                let edges: Vec<FieldValue> = parent
                    .0
                    .iter()
                    .enumerate()
                    .map(|(i, d)| {
                        FieldValue::owned_any(DesignEdge {
                            cursor: edge_cursor(i),
                            design: d.clone(),
                        })
                    })
                    .collect();
                Ok(Some(FieldValue::from(edges)))
            })
        }))
        .field(
            Field::new("pageInfo", TypeRef::named_nn("PageInfo"), |ctx| {
                FieldFuture::new(async move {
                    let _ = ctx.parent_value.downcast_ref::<DesignConn>().unwrap();
                    Ok(Some(FieldValue::owned_any(())))
                })
            }),
        )
        .field(Field::new("hash", TypeRef::named_nn(TypeRef::STRING), |ctx| {
            FieldFuture::new(async move {
                let parent = ctx.parent_value.downcast_ref::<DesignConn>().unwrap();
                let mut h = blake3::Hasher::new();
                for d in &parent.0 {
                    h.update(d.id.as_str().as_bytes());
                    h.update(b"\x1f");
                }
                Ok(Some(FieldValue::from(h.finalize().to_hex().to_string())))
            })
        }))
}

pub struct DesignEdge {
    pub cursor: String,
    pub design: Arc<crate::kit::design::Design>,
}

fn design_edge_object() -> Object {
    Object::new("DesignEdge")
        .implement("EntityEdge")
        .field(Field::new("cursor", TypeRef::named_nn(TypeRef::STRING), |ctx| {
            FieldFuture::new(async move {
                let p = ctx.parent_value.downcast_ref::<DesignEdge>().unwrap();
                Ok(Some(FieldValue::from(p.cursor.clone())))
            })
        }))
        .field(Field::new("node", TypeRef::named_nn("Design"), |ctx| {
            FieldFuture::new(async move {
                let p = ctx.parent_value.downcast_ref::<DesignEdge>().unwrap();
                Ok(Some(FieldValue::owned_any(p.design.clone())))
            })
        }))
}

fn piece_connection_object() -> Object {
    Object::new("PieceConnection")
        .implement("EntityConnectionInterface")
        .field(Field::new("edges", TypeRef::named_nn_list_nn("PieceEdge"), |ctx| {
            FieldFuture::new(async move {
                let parent = ctx.parent_value.downcast_ref::<PieceConn>().unwrap();
                let edges: Vec<FieldValue> = parent
                    .0
                    .iter()
                    .enumerate()
                    .map(|(i, p)| {
                        FieldValue::owned_any(PieceEdge {
                            cursor: edge_cursor(i),
                            piece: p.clone(),
                        })
                    })
                    .collect();
                Ok(Some(FieldValue::from(edges)))
            })
        }))
        .field(
            Field::new("pageInfo", TypeRef::named_nn("PageInfo"), |ctx| {
                FieldFuture::new(async move {
                    let _ = ctx.parent_value.downcast_ref::<PieceConn>().unwrap();
                    Ok(Some(FieldValue::owned_any(())))
                })
            }),
        )
        .field(Field::new("hash", TypeRef::named_nn(TypeRef::STRING), |ctx| {
            FieldFuture::new(async move {
                let parent = ctx.parent_value.downcast_ref::<PieceConn>().unwrap();
                let mut h = blake3::Hasher::new();
                for p in &parent.0 {
                    h.update(p.id.as_str().as_bytes());
                    h.update(b"\x1f");
                }
                Ok(Some(FieldValue::from(h.finalize().to_hex().to_string())))
            })
        }))
}

pub struct PieceEdge {
    pub cursor: String,
    pub piece: Arc<crate::kit::design::piece::Piece>,
}

fn piece_edge_object() -> Object {
    Object::new("PieceEdge")
        .implement("EntityEdge")
        .field(Field::new("cursor", TypeRef::named_nn(TypeRef::STRING), |ctx| {
            FieldFuture::new(async move {
                let p = ctx.parent_value.downcast_ref::<PieceEdge>().unwrap();
                Ok(Some(FieldValue::from(p.cursor.clone())))
            })
        }))
        .field(Field::new("node", TypeRef::named_nn("Piece"), |ctx| {
            FieldFuture::new(async move {
                let p = ctx.parent_value.downcast_ref::<PieceEdge>().unwrap();
                Ok(Some(FieldValue::owned_any(p.piece.clone())))
            })
        }))
}

fn edge_cursor(i: usize) -> String {
    format!("e{}", i)
}

//#endregion

fn overlay_page_info_resolver() -> Object {
    Object::new("PageInfo")
        .field(Field::new("hasNextPage", TypeRef::named_nn(TypeRef::BOOLEAN), |_| {
            FieldFuture::new(async move { Ok(Some(FieldValue::from(false))) })
        }))
        .field(Field::new("hasPreviousPage", TypeRef::named_nn(TypeRef::BOOLEAN), |_| {
            FieldFuture::new(async move { Ok(Some(FieldValue::from(false))) })
        }))
        .field(Field::new("startCursor", TypeRef::named(TypeRef::STRING), |_| {
            FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
        }))
        .field(Field::new("endCursor", TypeRef::named(TypeRef::STRING), |_| {
            FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
        }))
}

/// Real `Query` object (target schema).
fn real_query_object() -> Object {
    Object::new("Query")
        .field(Field::new("session", TypeRef::named_nn("Session"), |ctx| {
            FieldFuture::new(async move {
                let rt = ctx.data::<Arc<ParentRuntime>>()?;
                let s = {
                    let sessions = rt.sessions.read().await;
                    sessions.first().cloned()
                };
                let s = match s {
                    Some(s) => s,
                    None => {
                        let s = crate::vcs::Session::new().await;
                        rt.sessions.write().await.push(s.clone());
                        s
                    }
                };
                Ok(Some(FieldValue::owned_any(s)))
            })
        }))
        .field(Field::new("wip", TypeRef::named_nn("Graph"), |ctx| {
            FieldFuture::new(async move {
                let rt = ctx.data::<Arc<ParentRuntime>>()?;
                Ok(Some(FieldValue::owned_any(rt.wip_graph.clone())))
            })
        }))
        .field(Field::new("authoritative", TypeRef::named("Graph"), |ctx| {
            FieldFuture::new(async move {
                let rt = ctx.data::<Arc<ParentRuntime>>()?;
                Ok(Some(FieldValue::owned_any(rt.auth_graph.clone())))
            })
        }))
        .field(Field::new("conflicts", TypeRef::named_nn("ConflictConnection"), |ctx| {
            FieldFuture::new(async move {
                let rt = ctx.data::<Arc<ParentRuntime>>()?;
                let rows = rt.conflicts.read().await.clone();
                Ok(Some(FieldValue::owned_any(ConflictConn(rows))))
            })
        }))
        .field(Field::new("node", TypeRef::named("Node"), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
        }))
        .field(Field::new("entity", TypeRef::named("Entity"), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
        }))
        .field(
            Field::new("pieceInDesign", TypeRef::named("Piece"), |_ctx| {
                FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
            })
            .argument(InputValue::new("designId", TypeRef::named_nn(TypeRef::ID)))
            .argument(InputValue::new("pieceId", TypeRef::named_nn(TypeRef::ID))),
        )
        .field(
            Field::new("alternativePieceKind", TypeRef::named("Blueprint"), |_ctx| {
                FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
            })
            .argument(InputValue::new("designId", TypeRef::named_nn(TypeRef::ID)))
            .argument(InputValue::new("pieceId", TypeRef::named_nn(TypeRef::ID))),
        )
}

pub struct ConflictConn(pub Vec<Arc<crate::vcs::Conflict>>);

fn conflict_connection_object() -> Object {
    Object::new("ConflictConnection")
        .implement("EntityConnectionInterface")
        .field(Field::new("edges", TypeRef::named_nn_list_nn("ConflictEdge"), |ctx| {
            FieldFuture::new(async move {
                let parent = ctx.parent_value.downcast_ref::<ConflictConn>().unwrap();
                let edges: Vec<FieldValue> = parent
                    .0
                    .iter()
                    .enumerate()
                    .map(|(i, c)| {
                        FieldValue::owned_any(ConflictEdge {
                            cursor: edge_cursor(i),
                            conflict: c.clone(),
                        })
                    })
                    .collect();
                Ok(Some(FieldValue::from(edges)))
            })
        }))
        .field(
            Field::new("pageInfo", TypeRef::named_nn("PageInfo"), |ctx| {
                FieldFuture::new(async move {
                    let _ = ctx.parent_value.downcast_ref::<ConflictConn>().unwrap();
                    Ok(Some(FieldValue::owned_any(())))
                })
            }),
        )
        .field(Field::new("hash", TypeRef::named_nn(TypeRef::STRING), |_| {
            FieldFuture::new(async move { Ok(Some(FieldValue::from(String::new()))) })
        }))
}

pub struct ConflictEdge {
    pub cursor: String,
    pub conflict: Arc<crate::vcs::Conflict>,
}

fn conflict_edge_object() -> Object {
    Object::new("ConflictEdge")
        .implement("EntityEdge")
        .field(Field::new("cursor", TypeRef::named_nn(TypeRef::STRING), |ctx| {
            FieldFuture::new(async move {
                let p = ctx.parent_value.downcast_ref::<ConflictEdge>().unwrap();
                Ok(Some(FieldValue::from(p.cursor.clone())))
            })
        }))
        .field(Field::new("node", TypeRef::named_nn("Conflict"), |ctx| {
            FieldFuture::new(async move {
                let p = ctx.parent_value.downcast_ref::<ConflictEdge>().unwrap();
                Ok(Some(FieldValue::owned_any(p.conflict.clone())))
            })
        }))
}

fn real_graph_object() -> Object {
    Object::new("Graph")
        .implement("Entity")
        .field(Field::new("id", TypeRef::named_nn(TypeRef::ID), |ctx| {
            FieldFuture::new(async move {
                let g = ctx.parent_value.downcast_ref::<Arc<crate::vcs::Graph>>().unwrap();
                Ok(Some(FieldValue::from(g.id.clone())))
            })
        }))
        .field(Field::new("hash", TypeRef::named_nn(TypeRef::STRING), |ctx| {
            FieldFuture::new(async move {
                let g = ctx.parent_value.downcast_ref::<Arc<crate::vcs::Graph>>().unwrap();
                Ok(Some(FieldValue::from(g.compute_hash().await)))
            })
        }))
        .field(Field::new("owner", TypeRef::named_nn("GraphOwner"), |ctx| {
            FieldFuture::new(async move {
                let g = ctx.parent_value.downcast_ref::<Arc<crate::vcs::Graph>>().unwrap();
                if let Some(s) = g.owner_session.upgrade() {
                    return Ok(Some(FieldValue::owned_any(s).with_type("Session")));
                }
                Ok(Some(FieldValue::with_type(FieldValue::NULL, "Session")))
            })
        }))
        .field(Field::new("sessionOwner", TypeRef::named("Session"), |ctx| {
            FieldFuture::new(async move {
                let g = ctx.parent_value.downcast_ref::<Arc<crate::vcs::Graph>>().unwrap();
                Ok(Some(
                    g.owner_session
                        .upgrade()
                        .map(|s| FieldValue::owned_any(s))
                        .unwrap_or(FieldValue::NULL),
                ))
            })
        }))
        .field(Field::new("entityOwner", TypeRef::named("OwnerEntity"), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
        }))
        .field(
            Field::new("ownedEntities", TypeRef::named("OwnedEntityConnection"), |_ctx| {
                FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
            }),
        )
        .field(Field::new("theKit", TypeRef::named("Kit"), |ctx| {
            FieldFuture::new(async move {
                let g = ctx.parent_value.downcast_ref::<Arc<crate::vcs::Graph>>().unwrap();
                Ok(Some(FieldValue::owned_any(g.the_kit.clone())))
            })
        }))
        .field(
            Field::new("alternatives", TypeRef::named_nn("AlternativeConnection"), |_ctx| {
                FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
            }),
        )
        .field(
            Field::new("checkpoints", TypeRef::named_nn("CheckpointConnection"), |_ctx| {
                FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
            }),
        )
        .field(
            Field::new("alternative", TypeRef::named("Alternative"), |_ctx| {
                FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
            })
            .argument(InputValue::new("id", TypeRef::named_nn(TypeRef::ID))),
        )
        .field(
            Field::new("checkpoint", TypeRef::named("Checkpoint"), |_ctx| {
                FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
            })
            .argument(InputValue::new("id", TypeRef::named_nn(TypeRef::ID))),
        )
        .field(
            Field::new("releases", TypeRef::named_nn("CheckpointConnection"), |_ctx| {
                FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
            }),
        )
        .field(
            Field::new("release", TypeRef::named("Checkpoint"), |_ctx| {
                FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
            })
            .argument(InputValue::new("id", TypeRef::named_nn(TypeRef::ID))),
        )
}

fn real_kit_object() -> Object {
    Object::new("Kit")
        .implement("Entity")
        .field(Field::new("id", TypeRef::named_nn(TypeRef::ID), |ctx| {
            FieldFuture::new(async move {
                let k = ctx.parent_value.downcast_ref::<Arc<crate::kit::Kit>>().unwrap();
                Ok(Some(FieldValue::from(k.id.clone())))
            })
        }))
        .field(Field::new("hash", TypeRef::named_nn(TypeRef::STRING), |ctx| {
            FieldFuture::new(async move {
                let k = ctx.parent_value.downcast_ref::<Arc<crate::kit::Kit>>().unwrap();
                Ok(Some(FieldValue::from(k.compute_hash().await)))
            })
        }))
        .field(Field::new("owner", TypeRef::named_nn("KitOwner"), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::with_type(FieldValue::NULL, "Graph"))) })
        }))
        .field(Field::new("graphOwner", TypeRef::named("Graph"), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
        }))
        .field(Field::new("checkpointOwner", TypeRef::named("Checkpoint"), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
        }))
        .field(Field::new("alternativeOwner", TypeRef::named("Alternative"), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
        }))
        .field(Field::new("entityOwner", TypeRef::named("OwnerEntity"), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
        }))
        .field(
            Field::new("ownedEntities", TypeRef::named("OwnedEntityConnection"), |_ctx| {
                FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
            }),
        )
        .field(Field::new("checkpoint", TypeRef::named("Checkpoint"), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
        }))
        .field(Field::new("draft", TypeRef::named("Draft"), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
        }))
        .field(Field::new("transaction", TypeRef::named("Transaction"), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
        }))
        .field(Field::new("name", TypeRef::named_nn(TypeRef::STRING), |ctx| {
            FieldFuture::new(async move {
                let k = ctx.parent_value.downcast_ref::<Arc<crate::kit::Kit>>().unwrap();
                Ok(Some(FieldValue::from(k.name.read().await.clone())))
            })
        }))
        .field(Field::new("description", TypeRef::named(TypeRef::STRING), |ctx| {
            FieldFuture::new(async move {
                let k = ctx.parent_value.downcast_ref::<Arc<crate::kit::Kit>>().unwrap();
                Ok(Some(
                    k.description
                        .read()
                        .await
                        .clone()
                        .map(FieldValue::from)
                        .unwrap_or(FieldValue::NULL),
                ))
            })
        }))
        .field(Field::new("icon", TypeRef::named(TypeRef::STRING), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
        }))
        .field(Field::new("image", TypeRef::named(TypeRef::STRING), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
        }))
        .field(Field::new("preview", TypeRef::named(TypeRef::STRING), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
        }))
        .field(Field::new("remote", TypeRef::named(TypeRef::STRING), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
        }))
        .field(Field::new("homepage", TypeRef::named(TypeRef::STRING), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
        }))
        .field(Field::new("license", TypeRef::named(TypeRef::STRING), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
        }))
        .field(Field::new("uri", TypeRef::named(TypeRef::STRING), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
        }))
        .field(Field::new("design", TypeRef::named("Design"), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
        }))
        .field(Field::new("designs", TypeRef::named_nn("DesignConnection"), |ctx| {
            FieldFuture::new(async move {
                let k = ctx.parent_value.downcast_ref::<Arc<crate::kit::Kit>>().unwrap();
                let v = k.designs.read().await.clone();
                Ok(Some(FieldValue::owned_any(DesignConn(v))))
            })
        }))
        .field(Field::new("type", TypeRef::named("Type"), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
        }))
        .field(
            Field::new("types", TypeRef::named_nn("TypeConnection"), |_ctx| {
                FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
            }),
        )
        .field(Field::new("files", TypeRef::named_nn("FileConnection"), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
        }))
        .field(Field::new("folders", TypeRef::named_nn("FolderConnection"), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
        }))
        .field(
            Field::new("families", TypeRef::named_nn("FamilyConnection"), |_ctx| {
                FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
            }),
        )
        .field(Field::new("authors", TypeRef::named_nn("AuthorConnection"), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
        }))
        .field(Field::new("concepts", TypeRef::named_nn("ConceptConnection"), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
        }))
        .field(Field::new("tags", TypeRef::named_nn("TagConnection"), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
        }))
        .field(Field::new("qualities", TypeRef::named_nn("QualityConnection"), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
        }))
        .field(Field::new("props", TypeRef::named_nn("PropConnection"), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
        }))
        .field(
            Field::new("attributes", TypeRef::named_nn("AttributeConnection"), |_ctx| {
                FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
            }),
        )
        .field(Field::new("stats", TypeRef::named_nn("StatConnection"), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
        }))
        .field(Field::new("createdAt", TypeRef::named("Timestamp"), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
        }))
        .field(Field::new("createdBy", TypeRef::named("Author"), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
        }))
        .field(
            Field::new("authoredBy", TypeRef::named("AuthorConnection"), |_ctx| {
                FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
            }),
        )
        .field(
            Field::new("changedIn", TypeRef::named("CheckpointConnection"), |_ctx| {
                FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
            }),
        )
        .field(Field::new("lastChangedAt", TypeRef::named("Timestamp"), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
        }))
        .field(Field::new("lastChangedBy", TypeRef::named("Author"), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
        }))
        .field(Field::new("lastChangedIn", TypeRef::named("Checkpoint"), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
        }))
}

fn real_design_object() -> Object {
    Object::new("Design")
        .implement("Entity")
        .field(Field::new("id", TypeRef::named_nn(TypeRef::ID), |ctx| {
            FieldFuture::new(async move {
                let d = ctx.parent_value.downcast_ref::<Arc<crate::kit::design::Design>>().unwrap();
                Ok(Some(FieldValue::from(d.id.clone())))
            })
        }))
        .field(Field::new("hash", TypeRef::named_nn(TypeRef::STRING), |ctx| {
            FieldFuture::new(async move {
                let d = ctx.parent_value.downcast_ref::<Arc<crate::kit::design::Design>>().unwrap();
                Ok(Some(FieldValue::from(d.compute_hash().await)))
            })
        }))
        .field(Field::new("owner", TypeRef::named_nn("DesignOwner"), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::with_type(FieldValue::NULL, "Kit"))) })
        }))
        .field(Field::new("kitOwner", TypeRef::named("Kit"), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
        }))
        .field(Field::new("entityOwner", TypeRef::named_nn("OwnerEntity"), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
        }))
        .field(
            Field::new("ownedEntities", TypeRef::named("OwnedEntityConnection"), |_ctx| {
                FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
            }),
        )
        .field(Field::new("name", TypeRef::named_nn(TypeRef::STRING), |ctx| {
            FieldFuture::new(async move {
                let d = ctx.parent_value.downcast_ref::<Arc<crate::kit::design::Design>>().unwrap();
                Ok(Some(FieldValue::from(d.name.read().await.clone())))
            })
        }))
        .field(Field::new("description", TypeRef::named(TypeRef::STRING), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
        }))
        .field(Field::new("icon", TypeRef::named(TypeRef::STRING), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
        }))
        .field(Field::new("image", TypeRef::named(TypeRef::STRING), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
        }))
        .field(Field::new("place", TypeRef::named("Place"), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
        }))
        .field(Field::new("unit", TypeRef::named(TypeRef::STRING), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
        }))
        .field(Field::new("createdAt", TypeRef::named("Timestamp"), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
        }))
        .field(Field::new("createdBy", TypeRef::named("Author"), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
        }))
        .field(
            Field::new("authoredBy", TypeRef::named("AuthorConnection"), |_ctx| {
                FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
            }),
        )
        .field(
            Field::new("changedIn", TypeRef::named("CheckpointConnection"), |_ctx| {
                FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
            }),
        )
        .field(Field::new("lastChangedAt", TypeRef::named("Timestamp"), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
        }))
        .field(Field::new("lastChangedBy", TypeRef::named("Author"), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
        }))
        .field(Field::new("lastChangedIn", TypeRef::named("Checkpoint"), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
        }))
        .field(Field::new("pieces", TypeRef::named_nn("PieceConnection"), |ctx| {
            FieldFuture::new(async move {
                let d = ctx.parent_value.downcast_ref::<Arc<crate::kit::design::Design>>().unwrap();
                let v = d.pieces.read().await.clone();
                Ok(Some(FieldValue::owned_any(PieceConn(v))))
            })
        }))
        .field(
            Field::new("piece", TypeRef::named("Piece"), |_ctx| {
                FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
            })
            .argument(InputValue::new("id", TypeRef::named_nn(TypeRef::ID))),
        )
        .field(
            Field::new("connections", TypeRef::named_nn("ConnectionConnection"), |_ctx| {
                FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
            }),
        )
        .field(
            Field::new("connection", TypeRef::named("Connection"), |_ctx| {
                FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
            })
            .argument(InputValue::new("id", TypeRef::named_nn(TypeRef::ID))),
        )
        .field(
            Field::new("layers", TypeRef::named_nn("LayerConnection"), |_ctx| {
                FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
            }),
        )
        .field(
            Field::new("groups", TypeRef::named_nn("GroupConnection"), |_ctx| {
                FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
            }),
        )
        .field(
            Field::new("authors", TypeRef::named_nn("AuthorConnection"), |_ctx| {
                FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
            }),
        )
        .field(
            Field::new("qualities", TypeRef::named_nn("QualityConnection"), |_ctx| {
                FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
            }),
        )
        .field(Field::new("props", TypeRef::named_nn("PropConnection"), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
        }))
        .field(
            Field::new("attributes", TypeRef::named_nn("AttributeConnection"), |_ctx| {
                FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
            }),
        )
        .field(Field::new("stats", TypeRef::named_nn("StatConnection"), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
        }))
        .field(Field::new("qualitySum", TypeRef::named_nn(TypeRef::FLOAT), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::from(0.0f64))) })
        }))
        .field(
            Field::new("references", TypeRef::named_nn("DesignConnection"), |_ctx| {
                FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
            }),
        )
        .field(
            Field::new("referencedBy", TypeRef::named_nn("PieceConnection"), |_ctx| {
                FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
            }),
        )
}

fn real_piece_object() -> Object {
    Object::new("Piece")
        .implement("Entity")
        .field(Field::new("id", TypeRef::named_nn(TypeRef::ID), |ctx| {
            FieldFuture::new(async move {
                let p = ctx
                    .parent_value
                    .downcast_ref::<Arc<crate::kit::design::piece::Piece>>()
                    .unwrap();
                Ok(Some(FieldValue::from(p.id.clone())))
            })
        }))
        .field(Field::new("hash", TypeRef::named_nn(TypeRef::STRING), |ctx| {
            FieldFuture::new(async move {
                let p = ctx
                    .parent_value
                    .downcast_ref::<Arc<crate::kit::design::piece::Piece>>()
                    .unwrap();
                Ok(Some(FieldValue::from(p.compute_hash().await)))
            })
        }))
        .field(Field::new("owner", TypeRef::named_nn("PieceOwner"), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::with_type(FieldValue::NULL, "Design"))) })
        }))
        .field(Field::new("typeOwner", TypeRef::named("Type"), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
        }))
        .field(Field::new("designOwner", TypeRef::named("Design"), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
        }))
        .field(
            Field::new("pieceModificationOwner", TypeRef::named("PieceModification"), |_ctx| {
                FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
            }),
        )
        .field(Field::new("pieceOwner", TypeRef::named("Piece"), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
        }))
        .field(Field::new("entityOwner", TypeRef::named_nn("OwnerEntity"), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
        }))
        .field(
            Field::new("ownedEntities", TypeRef::named("OwnedEntityConnection"), |_ctx| {
                FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
            }),
        )
        .field(Field::new("name", TypeRef::named(TypeRef::STRING), |ctx| {
            FieldFuture::new(async move {
                let p = ctx
                    .parent_value
                    .downcast_ref::<Arc<crate::kit::design::piece::Piece>>()
                    .unwrap();
                Ok(Some(
                    p.name
                        .read()
                        .await
                        .clone()
                        .map(FieldValue::from)
                        .unwrap_or(FieldValue::NULL),
                ))
            })
        }))
        .field(Field::new("description", TypeRef::named(TypeRef::STRING), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
        }))
        .field(Field::new("position", TypeRef::named("Position"), |ctx| {
            FieldFuture::new(async move {
                let p = ctx
                    .parent_value
                    .downcast_ref::<Arc<crate::kit::design::piece::Piece>>()
                    .unwrap();
                let pos = *p.position.read().await;
                Ok(Some(
                    pos.map(|x| FieldValue::owned_any(x))
                        .unwrap_or(FieldValue::NULL),
                ))
            })
        }))
        .field(Field::new("scale", TypeRef::named(TypeRef::FLOAT), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
        }))
        .field(Field::new("blueprint", TypeRef::named_nn("Blueprint"), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::with_type(FieldValue::NULL, "Type"))) })
        }))
        .field(Field::new("props", TypeRef::named_nn("PropConnection"), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
        }))
        .field(
            Field::new("attributes", TypeRef::named_nn("AttributeConnection"), |_ctx| {
                FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
            }),
        )
        .field(
            Field::new("connectionKind", TypeRef::named("PieceConnectionKind"), |_ctx| {
                FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
            }),
        )
        .field(Field::new("parentPiece", TypeRef::named("Piece"), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
        }))
        .field(Field::new("parentConnection", TypeRef::named("Connection"), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
        }))
        .field(
            Field::new("childConnections", TypeRef::named_nn("ConnectionConnection"), |_ctx| {
                FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
            }),
        )
        .field(
            Field::new("childPieces", TypeRef::named_nn("PieceConnection"), |_ctx| {
                FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
            }),
        )
        .field(
            Field::new("path", TypeRef::named_nn("PieceConnection"), |_ctx| {
                FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
            }),
        )
        .field(Field::new("depth", TypeRef::named_nn(TypeRef::INT), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::from(0i32))) })
        }))
        .field(Field::new("flatPosition", TypeRef::named_nn("Position"), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
        }))
        .field(
            Field::new("replaceableBlueprints", TypeRef::named_nn("BlueprintConnection"), |_ctx| {
                FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
            }),
        )
}

fn real_session_object() -> Object {
    Object::new("Session")
        .field(Field::new("id", TypeRef::named_nn(TypeRef::ID), |ctx| {
            FieldFuture::new(async move {
                let s = ctx.parent_value.downcast_ref::<Arc<crate::vcs::Session>>().unwrap();
                Ok(Some(FieldValue::from(s.id.clone())))
            })
        }))
        .field(Field::new("startedAt", TypeRef::named("Timestamp"), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
        }))
        .field(
            Field::new("drafts", TypeRef::named_nn_list_nn("Draft"), |_ctx| {
                FieldFuture::new(async move { Ok(Some(FieldValue::from(Vec::<FieldValue>::new()))) })
            }),
        )
}

fn real_conflict_object() -> Object {
    Object::new("Conflict")
        .field(Field::new("id", TypeRef::named_nn(TypeRef::ID), |ctx| {
            FieldFuture::new(async move {
                let c = ctx.parent_value.downcast_ref::<Arc<crate::vcs::Conflict>>().unwrap();
                Ok(Some(FieldValue::from(c.id.clone())))
            })
        }))
        .field(Field::new("backboneTip", TypeRef::named(TypeRef::STRING), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
        }))
        .field(Field::new("reason", TypeRef::named_nn(TypeRef::STRING), |ctx| {
            FieldFuture::new(async move {
                let c = ctx.parent_value.downcast_ref::<Arc<crate::vcs::Conflict>>().unwrap();
                Ok(Some(FieldValue::from(c.reason.read().await.clone())))
            })
        }))
        .field(Field::new("createdAt", TypeRef::named_nn("Timestamp"), |ctx| {
            FieldFuture::new(async move {
                let c = ctx.parent_value.downcast_ref::<Arc<crate::vcs::Conflict>>().unwrap();
                Ok(Some(FieldValue::from(c.created_at.read().await.clone())))
            })
        }))
}

fn position_value_object() -> Object {
    Object::new("Position")
        .implement("WeakEntity")
        .field(Field::new("id", TypeRef::named_nn(TypeRef::ID), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::from("position".to_string()))) })
        }))
        .field(Field::new("hash", TypeRef::named_nn(TypeRef::STRING), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::from(String::new()))) })
        }))
        .field(Field::new("owner", TypeRef::named_nn("PositionOwner"), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::with_type(FieldValue::NULL, "Piece"))) })
        }))
        .field(Field::new("entityOwner", TypeRef::named("OwnerEntity"), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
        }))
        .field(
            Field::new("ownedEntities", TypeRef::named("OwnedEntityConnection"), |_ctx| {
                FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
            }),
        )
        .field(Field::new("center", TypeRef::named_nn("Coordinate"), |ctx| {
            FieldFuture::new(async move {
                let p = ctx.parent_value.downcast_ref::<crate::geom::Position>().unwrap();
                Ok(Some(FieldValue::owned_any(p.center)))
            })
        }))
        .field(Field::new("plane", TypeRef::named_nn("Plane"), |ctx| {
            FieldFuture::new(async move {
                let p = ctx.parent_value.downcast_ref::<crate::geom::Position>().unwrap();
                Ok(Some(FieldValue::owned_any(p.plane)))
            })
        }))
}

fn coordinate_value_object() -> Object {
    Object::new("Coordinate")
        .implement("WeakEntity")
        .field(Field::new("id", TypeRef::named_nn(TypeRef::ID), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::from("coord".to_string()))) })
        }))
        .field(Field::new("hash", TypeRef::named_nn(TypeRef::STRING), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::from(String::new()))) })
        }))
        .field(Field::new("owner", TypeRef::named_nn("CoordinateOwner"), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::with_type(FieldValue::NULL, "Position"))) })
        }))
        .field(Field::new("entityOwner", TypeRef::named("OwnerEntity"), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
        }))
        .field(
            Field::new("ownedEntities", TypeRef::named("OwnedEntityConnection"), |_ctx| {
                FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
            }),
        )
        .field(Field::new("u", TypeRef::named_nn(TypeRef::FLOAT), |ctx| {
            FieldFuture::new(async move {
                let c = ctx.parent_value.downcast_ref::<crate::geom::Coordinate>().unwrap();
                Ok(Some(FieldValue::from(c.u)))
            })
        }))
        .field(Field::new("v", TypeRef::named_nn(TypeRef::FLOAT), |ctx| {
            FieldFuture::new(async move {
                let c = ctx.parent_value.downcast_ref::<crate::geom::Coordinate>().unwrap();
                Ok(Some(FieldValue::from(c.v)))
            })
        }))
}

fn plane_value_object() -> Object {
    Object::new("Plane")
        .implement("WeakEntity")
        .field(Field::new("id", TypeRef::named_nn(TypeRef::ID), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::from("plane".to_string()))) })
        }))
        .field(Field::new("hash", TypeRef::named_nn(TypeRef::STRING), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::from(String::new()))) })
        }))
        .field(Field::new("owner", TypeRef::named_nn("PlaneOwner"), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::with_type(FieldValue::NULL, "Position"))) })
        }))
        .field(Field::new("entityOwner", TypeRef::named("OwnerEntity"), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
        }))
        .field(
            Field::new("ownedEntities", TypeRef::named("OwnedEntityConnection"), |_ctx| {
                FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
            }),
        )
        .field(Field::new("origin", TypeRef::named_nn("Point"), |ctx| {
            FieldFuture::new(async move {
                let p = ctx.parent_value.downcast_ref::<crate::geom::Plane>().unwrap();
                Ok(Some(FieldValue::owned_any(p.origin)))
            })
        }))
        .field(Field::new("xAxis", TypeRef::named_nn("Vector"), |ctx| {
            FieldFuture::new(async move {
                let p = ctx.parent_value.downcast_ref::<crate::geom::Plane>().unwrap();
                Ok(Some(FieldValue::owned_any(p.x_axis)))
            })
        }))
        .field(Field::new("yAxis", TypeRef::named_nn("Vector"), |ctx| {
            FieldFuture::new(async move {
                let p = ctx.parent_value.downcast_ref::<crate::geom::Plane>().unwrap();
                Ok(Some(FieldValue::owned_any(p.y_axis)))
            })
        }))
}

fn point_value_object() -> Object {
    Object::new("Point")
        .implement("WeakEntity")
        .field(Field::new("id", TypeRef::named_nn(TypeRef::ID), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::from("pt".to_string()))) })
        }))
        .field(Field::new("hash", TypeRef::named_nn(TypeRef::STRING), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::from(String::new()))) })
        }))
        .field(Field::new("owner", TypeRef::named_nn("PointOwner"), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::with_type(FieldValue::NULL, "Plane"))) })
        }))
        .field(Field::new("entityOwner", TypeRef::named("OwnerEntity"), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
        }))
        .field(
            Field::new("ownedEntities", TypeRef::named("OwnedEntityConnection"), |_ctx| {
                FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
            }),
        )
        .field(Field::new("x", TypeRef::named_nn(TypeRef::FLOAT), |ctx| {
            FieldFuture::new(async move {
                let p = ctx.parent_value.downcast_ref::<crate::geom::Point>().unwrap();
                Ok(Some(FieldValue::from(p.x)))
            })
        }))
        .field(Field::new("y", TypeRef::named_nn(TypeRef::FLOAT), |ctx| {
            FieldFuture::new(async move {
                let p = ctx.parent_value.downcast_ref::<crate::geom::Point>().unwrap();
                Ok(Some(FieldValue::from(p.y)))
            })
        }))
        .field(Field::new("z", TypeRef::named_nn(TypeRef::FLOAT), |ctx| {
            FieldFuture::new(async move {
                let p = ctx.parent_value.downcast_ref::<crate::geom::Point>().unwrap();
                Ok(Some(FieldValue::from(p.z)))
            })
        }))
}

fn vector_value_object() -> Object {
    Object::new("Vector")
        .implement("WeakEntity")
        .field(Field::new("id", TypeRef::named_nn(TypeRef::ID), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::from("vec".to_string()))) })
        }))
        .field(Field::new("hash", TypeRef::named_nn(TypeRef::STRING), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::from(String::new()))) })
        }))
        .field(Field::new("owner", TypeRef::named_nn("VectorOwner"), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::with_type(FieldValue::NULL, "Plane"))) })
        }))
        .field(Field::new("entityOwner", TypeRef::named("OwnerEntity"), |_ctx| {
            FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
        }))
        .field(
            Field::new("ownedEntities", TypeRef::named("OwnedEntityConnection"), |_ctx| {
                FieldFuture::new(async move { Ok(Some(FieldValue::NULL)) })
            }),
        )
        .field(Field::new("x", TypeRef::named_nn(TypeRef::FLOAT), |ctx| {
            FieldFuture::new(async move {
                let v = ctx.parent_value.downcast_ref::<crate::geom::Vector>().unwrap();
                Ok(Some(FieldValue::from(v.x)))
            })
        }))
        .field(Field::new("y", TypeRef::named_nn(TypeRef::FLOAT), |ctx| {
            FieldFuture::new(async move {
                let v = ctx.parent_value.downcast_ref::<crate::geom::Vector>().unwrap();
                Ok(Some(FieldValue::from(v.y)))
            })
        }))
        .field(Field::new("z", TypeRef::named_nn(TypeRef::FLOAT), |ctx| {
            FieldFuture::new(async move {
                let v = ctx.parent_value.downcast_ref::<crate::geom::Vector>().unwrap();
                Ok(Some(FieldValue::from(v.z)))
            })
        }))
}

fn dispatch_mutation(ctx: &async_graphql::dynamic::ResolverContext<'_>, kind: &'static str) -> FieldFuture<'_> {
    let draft_id: Id = ctx.args.try_get("draftId").ok().and_then(|a| a.deserialize().ok()).unwrap_or_default();
    let transaction_id: Id = ctx
        .args
        .try_get("transactionId")
        .ok()
        .and_then(|a| a.deserialize().ok())
        .unwrap_or_default();
    let rt = match ctx.data::<Arc<ParentRuntime>>() {
        Ok(r) => r.clone(),
        Err(_) => {
            return FieldFuture::new(async move { Ok(Some(FieldValue::from(String::new()))) });
        }
    };
    let request_id = Id::new_sync();
    FieldFuture::new(async move {
        match kind {
            "renameKit" => {
                let name: String = ctx
                    .args
                    .try_get("name")
                    .ok()
                    .and_then(|a| a.deserialize().ok())
                    .unwrap_or_default();
                let cmd = crate::op::Command::RenameKit { request_id: request_id.clone(), draft_id, transaction_id, name };
                rt.dispatch_wip(cmd).await;
            }
            "changeDescription" => {
                let entity_id: Id = ctx
                    .args
                    .try_get("entityId")
                    .ok()
                    .and_then(|a| a.deserialize().ok())
                    .unwrap_or_default();
                let description: String = ctx
                    .args
                    .try_get("description")
                    .ok()
                    .and_then(|a| a.deserialize().ok())
                    .unwrap_or_default();
                let _ = entity_id;
                let cmd =
                    crate::op::Command::ChangeDescription { request_id: request_id.clone(), draft_id, transaction_id, description };
                rt.dispatch_wip(cmd).await;
            }
            "addFixedPieceToDesign" => {
                let design_id: Id = ctx
                    .args
                    .try_get("designId")
                    .ok()
                    .and_then(|a| a.deserialize().ok())
                    .unwrap_or_default();
                let blueprint_id: Id = ctx
                    .args
                    .try_get("blueprintId")
                    .ok()
                    .and_then(|a| a.deserialize().ok())
                    .unwrap_or_else(|| Id::new_sync());
                let position: crate::geom::Position = ctx
                    .args
                    .try_get("position")
                    .ok()
                    .and_then(|a| a.deserialize().ok())
                    .unwrap_or_default();
                let name: Option<String> = ctx.args.try_get("name").ok().and_then(|a| a.deserialize().ok());
                let description: Option<String> = ctx.args.try_get("description").ok().and_then(|a| a.deserialize().ok());
                let cmd = crate::op::Command::AddFixedPieceToDesign {
                    request_id: request_id.clone(),
                    draft_id,
                    transaction_id,
                    design_id,
                    blueprint_id,
                    position,
                    name,
                    description,
                };
                rt.dispatch_wip(cmd).await;
            }
            "fixPieceInDesign" => {
                let design_id: Id = ctx
                    .args
                    .try_get("designId")
                    .ok()
                    .and_then(|a| a.deserialize().ok())
                    .unwrap_or_default();
                let piece_id: Id = ctx
                    .args
                    .try_get("pieceId")
                    .ok()
                    .and_then(|a| a.deserialize().ok())
                    .unwrap_or_default();
                let cmd = crate::op::Command::FixPieceInDesign {
                    request_id: request_id.clone(),
                    draft_id,
                    transaction_id,
                    design_id,
                    piece_id,
                };
                rt.dispatch_wip(cmd).await;
            }
            _ => {}
        }
        Ok(Some(FieldValue::from(request_id.to_string())))
    })
}

fn real_mutation_object() -> Object {
    let mut m = Object::new("Mutation");
    // Kit
    m = m.field(Field::new("renameKit", TypeRef::named_nn(TypeRef::ID), |ctx| {
        dispatch_mutation(ctx, "renameKit")
    }).argument(InputValue::new("draftId", TypeRef::named_nn(TypeRef::ID)))
    .argument(InputValue::new("transactionId", TypeRef::named_nn(TypeRef::ID)))
    .argument(InputValue::new("name", TypeRef::named_nn(TypeRef::STRING))));
    m = m.field(
        Field::new("changeDescription", TypeRef::named_nn(TypeRef::ID), |ctx| {
            dispatch_mutation(ctx, "changeDescription")
        })
        .argument(InputValue::new("draftId", TypeRef::named_nn(TypeRef::ID)))
        .argument(InputValue::new("transactionId", TypeRef::named_nn(TypeRef::ID)))
        .argument(InputValue::new("entityId", TypeRef::named_nn(TypeRef::ID)))
        .argument(InputValue::new("description", TypeRef::named_nn(TypeRef::STRING))),
    );
    m = m.field(Field::new("addFixedPieceToDesign", TypeRef::named_nn(TypeRef::ID), |ctx| {
        dispatch_mutation(ctx, "addFixedPieceToDesign")
    })
    .argument(InputValue::new("draftId", TypeRef::named_nn(TypeRef::ID)))
    .argument(InputValue::new("transactionId", TypeRef::named_nn(TypeRef::ID)))
    .argument(InputValue::new("designId", TypeRef::named_nn(TypeRef::ID)))
    .argument(InputValue::new("blueprintId", TypeRef::named_nn(TypeRef::ID)))
    .argument(InputValue::new("position", TypeRef::named_nn("PositionInput")))
    .argument(InputValue::new("name", TypeRef::named(TypeRef::STRING)))
    .argument(InputValue::new("description", TypeRef::named(TypeRef::STRING))));
    m = m.field(Field::new("fixPieceInDesign", TypeRef::named_nn(TypeRef::ID), |ctx| {
        dispatch_mutation(ctx, "fixPieceInDesign")
    })
    .argument(InputValue::new("draftId", TypeRef::named_nn(TypeRef::ID)))
    .argument(InputValue::new("transactionId", TypeRef::named_nn(TypeRef::ID)))
    .argument(InputValue::new("designId", TypeRef::named_nn(TypeRef::ID)))
    .argument(InputValue::new("pieceId", TypeRef::named_nn(TypeRef::ID))));
    m
}

/// Build executable schema: full type graph from target SDL + runtime overlays.
pub fn build_executable_target_schema(rt: Arc<ParentRuntime>, bus: Arc<EventBus>) -> Result<Schema, SchemaError> {
    let doc = parse_schema(TARGET_GRAPHQL_SDL).map_err(|e| SchemaError(format!("parse target.schema.graphql: {e}")))?;
    let (query_name, mutation_name, subscription_name) = extract_schema_roots(&doc)?;
    let mut b = Schema::build(&query_name, mutation_name.as_deref(), subscription_name.as_deref());
    b = register_definitions(b, &doc, &query_name, &mutation_name, &subscription_name)?;
    // Overlays: PageInfo (pageInfo on connections), relay edges, core resolvers.
    b = b.register(Type::Object(overlay_page_info_resolver()));
    b = b.register(Type::Object(design_connection_object()));
    b = b.register(Type::Object(design_edge_object()));
    b = b.register(Type::Object(piece_connection_object()));
    b = b.register(Type::Object(piece_edge_object()));
    b = b.register(Type::Object(conflict_connection_object()));
    b = b.register(Type::Object(conflict_edge_object()));
    b = b.register(Type::Object(real_query_object()));
    b = b.register(Type::Object(real_graph_object()));
    b = b.register(Type::Object(real_kit_object()));
    b = b.register(Type::Object(real_design_object()));
    b = b.register(Type::Object(real_piece_object()));
    b = b.register(Type::Object(real_session_object()));
    b = b.register(Type::Object(real_conflict_object()));
    b = b.register(Type::Object(position_value_object()));
    b = b.register(Type::Object(coordinate_value_object()));
    b = b.register(Type::Object(plane_value_object()));
    b = b.register(Type::Object(point_value_object()));
    b = b.register(Type::Object(vector_value_object()));
    if let Some(mn) = &mutation_name {
        let mut mo = real_mutation_object();
        // Append stub resolvers for every other mutation field from AST so the object matches SDL.
        if let Some(TypeSystemDefinition::Type(td)) = doc.definitions.iter().find(|d| {
            matches!(d, TypeSystemDefinition::Type(t) if t.node.name.node.as_str() == mn.as_str())
        }) {
            if let TypeKind::Object(o) = &td.node.kind {
                let existing: std::collections::HashSet<String> =
                    ["renameKit", "changeDescription", "addFixedPieceToDesign", "fixPieceInDesign"]
                        .into_iter()
                        .map(String::from)
                        .collect();
                for field in &o.fields {
                    let fname = field.node.name.node.to_string();
                    if existing.contains(&fname) {
                        continue;
                    }
                    mo = mo.field(object_field_from_ast(field));
                }
            }
        }
        b = b.register(Type::Object(mo));
    }
    b.data(rt).data(bus).finish()
}
