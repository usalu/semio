# 🧩️ Section E of `c12-jack-query-document-patch.py`: every exhaustive match and bounded owner the new variant/field reaches
# (found by the overlay compile, `jack-overlay-proof-1.txt`).
WIRE_RT = SCHEMA + "🛜️wire-runtime/🦀️.rs"
EXEC = SCHEMA + "🧮️executor/🪜️execution/🦀️.rs"
EXEC_ROOT = SCHEMA + "🧮️executor/🦀️.rs"
OPS = SCHEMA + "⚙️operations/🦀️.rs"
JOB_LAWS = EDITOR + "🎮️commands/▶️run-query/🧵️job/🧪️tests/🔬️unit/🦀️.rs"

edit(WIRE_RT, "use crate::standards::v1::subsets::any::schema::mutations::{change_data_property, create_edge, create_node, delete_edge, delete_node, move_node, remove_data_property, rename_node};",
     "use crate::standards::v1::subsets::any::schema::mutations::{change_data_property, create_edge, create_node, delete_edge, delete_node, move_node, remove_data_property, rename_node, set_query};", "wire imports set_query")
edit(WIRE_RT, """    RemoveDataProperty {
        entity: EntityRefDsl,
        key: String,
    },
}""", """    RemoveDataProperty {
        entity: EntityRefDsl,
        key: String,
    },
    SetQuery {
        value: String,
    },
}""", "wire dsl mirror variant")
edit(WIRE_RT, """        TrinityGraphMutation::RemoveDataProperty(payload) => TrinityGraphOperationDsl::RemoveDataProperty { entity: (&payload.entity).into(), key: payload.key.clone() },
    }""", """        TrinityGraphMutation::RemoveDataProperty(payload) => TrinityGraphOperationDsl::RemoveDataProperty { entity: (&payload.entity).into(), key: payload.key.clone() },
        TrinityGraphMutation::SetQuery(payload) => TrinityGraphOperationDsl::SetQuery { value: payload.value.clone() },
    }""", "wire to dsl")
edit(WIRE_RT, """        TrinityGraphOperationDsl::RemoveDataProperty { entity, key } => remove_data_property(entity.into(), key),
    }""", """        TrinityGraphOperationDsl::RemoveDataProperty { entity, key } => remove_data_property(entity.into(), key),
        TrinityGraphOperationDsl::SetQuery { value } => set_query(value),
    }""", "wire from dsl")
edit(WIRE_RT, """    RemoveDataProperty { entity: Option<EntityRef>, key: String },
}""", """    RemoveDataProperty { entity: Option<EntityRef>, key: String },
    SetQuery(String),
}""", "wire mutation fields variant")
edit(WIRE_RT, """                    TrinityGraphMutation::RemoveDataProperty(value) => JackMutationFields::RemoveDataProperty { entity: Some(value.entity), key: value.key },
""", """                    TrinityGraphMutation::RemoveDataProperty(value) => JackMutationFields::RemoveDataProperty { entity: Some(value.entity), key: value.key },
                    TrinityGraphMutation::SetQuery(value) => JackMutationFields::SetQuery(value.value),
""", "wire mutation fields conversion")
edit(WIRE_RT, "                JackMutationFields::DeleteNode(value) | JackMutationFields::DeleteEdge(value) | JackMutationFields::MoveNode(value) => {",
     "                JackMutationFields::DeleteNode(value) | JackMutationFields::DeleteEdge(value) | JackMutationFields::MoveNode(value) | JackMutationFields::SetQuery(value) => {", "wire mutation fields retirement")

edit(OPS, """        TrinityGraphMutation::RemoveDataProperty(payload) => {
            validate_clear_data_property(snapshot, &payload.entity, &payload.key)?;
        }
    }
    Ok(())""", """        TrinityGraphMutation::RemoveDataProperty(payload) => {
            validate_clear_data_property(snapshot, &payload.entity, &payload.key)?;
        }
        TrinityGraphMutation::SetQuery(payload) => {
            if payload.value.len() > crate::JACK_QUERY_MAXIMUM_BYTES {
                return Err(TrinityRamError::QueryTooLarge { bytes: payload.value.len(), maximum: crate::JACK_QUERY_MAXIMUM_BYTES });
            }
        }
    }
    Ok(())""", "operation validation set-query bound")

edit(EXEC_ROOT, "use crate::standards::v1::subsets::any::schema::mutations::{change_data_property, create_edge, create_node, delete_node, move_node, rename_node, TrinityGraphMutation};",
     "use crate::standards::v1::subsets::any::schema::mutations::{change_data_property, create_edge, create_node, delete_node, move_node, rename_node, set_query, TrinityGraphMutation};", "executor imports set_query")
edit(EXEC, """                root_node_id: metadata.root_node_id.take(),
            };
            self.metadata_retirement""", """                root_node_id: metadata.root_node_id.take(),
                query: std::mem::take(&mut metadata.query),
            };
            self.metadata_retirement""", "executor graph query")
edit(EXEC, """                    None => Err(format!("edge {id} not found")),
                },
            },
        };
        if let Err(error) = applied {""", """                    None => Err(format!("edge {id} not found")),
                },
            },
            TrinityGraphMutation::SetQuery(value) => {
                let previous = std::mem::replace(&mut self.graph.query, value.value.clone());
                self.retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&JackMutationRetirementFactory, set_query(previous)));
                Ok(())
            }
        };
        if let Err(error) = applied {""", "executor applies set-query")

replace_every(JOB_LAWS, 'JackQueryWork::new("runQuery", "RETURN 1".into(), "editor".into(),', 'JackQueryWork::new("runQuery", "RETURN 1".into(), false, "editor".into(),', "job laws adopt flag")
edit(ROOT, """    UnknownPropertyInBag {
        path: String,
        key: String,
    },
}""", """    UnknownPropertyInBag {
        path: String,
        key: String,
    },
    /// 📏️ A `set-query` beyond [`JACK_QUERY_MAXIMUM_BYTES`].
    QueryTooLarge {
        bytes: usize,
        maximum: usize,
    },
}""", "root error query too large")
edit(ROOT, """            Self::UnknownPropertyInBag { path, key } => write!(formatter, "{path}/{key}: unknown property {key:?}"),
        }""", """            Self::UnknownPropertyInBag { path, key } => write!(formatter, "{path}/{key}: unknown property {key:?}"),
            Self::QueryTooLarge { bytes, maximum } => write!(formatter, "query: {bytes} bytes exceed the {maximum}-byte bound"),
        }""", "root error query too large display")
