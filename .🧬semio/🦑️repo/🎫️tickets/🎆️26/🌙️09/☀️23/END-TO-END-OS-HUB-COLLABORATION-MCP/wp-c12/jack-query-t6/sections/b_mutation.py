# 🔎️ Section B of `c12-jack-query-document-patch.py`: the `set-query` document mutation leaf and its registration in every surface.
LEAF = MUT + "🔎️set-query/"
CASE = "🔎️replaces-the-query"
FIXTURE = ANY + "🧫️fixtures/🧬️mutations/🔎️set-query/" + CASE + "/"
NEW_QUERY = "MATCH (a:Piece) RETURN a.name"

create(LEAF + "🦀️.rs", '''//! 🔎️ TrinityGraph mutation — `SetQuery`: replaces the document's Jack query text.
use crate::standards::v1::subsets::any::schema::diff::JackDiff;
use crate::standards::v1::subsets::any::schema::mutations::TrinityGraphMutation;
use crate::JackSnapshot;

//#region 🔖️Mutation
/// 🔎️ `set-query` payload — the whole query text the document holds afterwards.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetQuery {
    pub value: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn set_query(value: String) -> TrinityGraphMutation {
    TrinityGraphMutation::SetQuery(SetQuery { value })
}

impl protocol::MutationKind<JackSnapshot, TrinityGraphMutation> for SetQuery {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "query", kind: "set-query", record: "SetQuery" };

    fn diff(&self, base: &JackSnapshot) -> protocol::MutationOutcome<JackDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &JackSnapshot) -> Vec<TrinityGraphMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("Edit Jack query", "Jack-Abfrage bearbeiten")
    }
    fn target(&self) -> Vec<String> {
        vec!["query".to_string()]
    }
}
//#endregion 🔖️Mutation
''', "leaf rust")
create(LEAF + "🔺️diff/🦀️.rs", '''//! 🔺️ Sparse diff builder for `SetQuery` — only the `query` slot moves.
use crate::standards::v1::subsets::any::schema::diff::JackDiff;
use crate::JackSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::SetQuery, base: &JackSnapshot) -> protocol::MutationOutcome<JackDiff> {
    if base.query == payload.value {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "The Jack query already reads this text.");
    }
    protocol::MutationOutcome::new(JackDiff { query: Some(payload.value.clone()), ..JackDiff::default() })
}
//#endregion 🔖️Diff
''', "leaf diff rust")
create(LEAF + "↩️inverse/🦀️.rs", '''//! ↩️ Inverse for `SetQuery` — a `set-query` back to the query BASE held.
use crate::standards::v1::subsets::any::schema::mutations::{set_query, TrinityGraphMutation};
use crate::JackSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::SetQuery, base: &JackSnapshot) -> Vec<TrinityGraphMutation> {
    vec![set_query(base.query.clone())]
}
//#endregion 🔖️Inverse
''', "leaf inverse rust")
create(LEAF + "💾️binary/🦀️.rs", '''//! 💾️ Direct binary-codec identity for `set-query` / `SetQuery`.

pub const BINARY_TAG: u8 = dsl::protocol_record::tag_u8(include_str!("../../💾️binary/📡️.protocol.semio"), "set-query");
''', "leaf binary rust")
create(LEAF + "📝️text/🦀️.rs", '''//! 📝️ Direct text-codec identity for `set-query` / `SetQuery`.

pub const TEXT_OPCODE: &str = "set-query";
''', "leaf text rust")
create(LEAF + "🟦️.ts", '''/** 🔎️ jack direct `set-query` payload mirror of `SetQuery`. */
export interface SetQuery {
  value: string;
}
''', "leaf ts")
create(LEAF + "🔺️diff/🟦️.ts", '''/** 🔺️ jack set-query/🔺️diff — mirror of the query-slot delta builder. */
import type { SetQuery } from "../🟦️.ts";

export function diff(payload: SetQuery, baseQuery: string): { query: string } | null {
  return baseQuery === payload.value ? null : { query: payload.value };
}
''', "leaf diff ts")
create(LEAF + "↩️inverse/🟦️.ts", '''/** ↩️ jack set-query/↩️inverse — mirror of the BASE-query inverse builder. */
import type { SetQuery } from "../🟦️.ts";

export function inverse(_payload: SetQuery, baseQuery: string): SetQuery[] {
  return [{ value: baseQuery }];
}
''', "leaf inverse ts")
create(LEAF + "🔗️.graphql", '''# 🔎️ Direct set-query / SetQuery payload.
input SetQueryInput { value: String! }
''', "leaf graphql")
create(LEAF + "🛰️.proto", '''syntax = "proto3";
package semio.s.trinity.jack.mutation.set_query;
// set-query / SetQuery
message SetQuery { string value = 1; }
''', "leaf proto")
create(LEAF + "🧬️schema/🔣️.json", json.dumps({
    "$schema": "http://json-schema.org/draft-07/schema#",
    "$id": "https://json.schemas.assets.semio-tech.com/s/trinity/jack/mutation/set-query/schema.json",
    "title": "SetQuery",
    "type": "object",
    "additionalProperties": False,
    "required": ["value"],
    "properties": {"value": {"type": "string"}},
}, indent=2, ensure_ascii=False) + "\n", "leaf payload schema")
create(LEAF + "🔣️.json", json.dumps({
    "schemaVersion": 1,
    "owner": MUT.rstrip("/") + "/🔎️set-query",
    "semanticKind": "set-query",
    "displayName": "Set Query",
    "emoji": "🔎️",
    "aggregateVariant": "SetQuery",
    "payloadSchema": "🧬️schema/🔣️.json",
    "textOpcode": "set-query",
    "binaryTag": 8,
    "invertibility": "explicit-mutation",
    "diffParticipation": "detect",
    "outcomeClasses": ["applied", "no-op"],
    "composition": "atomic",
    "requiredLanguageSurfaces": ["rust", "typescript", "graphql", "protobuf", "json-schema", "text", "binary"],
}, indent=2, ensure_ascii=False) + "\n", "leaf meta")

# ── aggregate registration ──
edit(MUT + "🦀️.rs", "pub use super::rename_node::{rename_node, RenameNode};\n", "pub use super::rename_node::{rename_node, RenameNode};\npub use super::set_query::{set_query, SetQuery};\n", "aggregate use")
edit(MUT + "🦀️.rs", "    RemoveDataProperty(RemoveDataProperty),\n}", "    RemoveDataProperty(RemoveDataProperty),\n    SetQuery(SetQuery),\n}", "aggregate variant")
edit(MUT + "🟦️.ts", 'import type { RenameNode } from "./✏️rename-node/🟦️.ts";\n', 'import type { RenameNode } from "./✏️rename-node/🟦️.ts";\nimport type { SetQuery } from "./🔎️set-query/🟦️.ts";\n', "aggregate ts import")
edit(MUT + "🟦️.ts", '  | ({ mutation: "renameNode" } & RenameNode);', '  | ({ mutation: "renameNode" } & RenameNode)\n  | ({ mutation: "setQuery" } & SetQuery);', "aggregate ts union")
edit(MUT + "🔣️.json", '''    {
      "$ref": "https://json.schemas.assets.semio-tech.com/s/trinity/jack/mutation/rename-node/schema.json"
    }
  ]''', '''    {
      "$ref": "https://json.schemas.assets.semio-tech.com/s/trinity/jack/mutation/rename-node/schema.json"
    },
    {
      "$ref": "https://json.schemas.assets.semio-tech.com/s/trinity/jack/mutation/set-query/schema.json"
    }
  ]''', "aggregate json")
edit(MUT + "🔗️.graphql", "input RenameNodeInput { id: String!, newName: String! }\n", "input RenameNodeInput { id: String!, newName: String! }\ninput SetQueryInput { value: String! }\n", "aggregate graphql input")
edit(MUT + "🔗️.graphql", "  renameNode: RenameNodeInput\n}", "  renameNode: RenameNodeInput\n  setQuery: SetQueryInput\n}", "aggregate graphql oneof")
edit(MUT + "🛰️.proto", "message RenameNode { string id = 1; string new_name = 2; }\n", "message RenameNode { string id = 1; string new_name = 2; }\nmessage SetQuery { string value = 1; }\n", "aggregate proto message")
edit(MUT + "🛰️.proto", "    RemoveDataProperty remove_data_property = 8;\n  }", "    RemoveDataProperty remove_data_property = 8;\n    SetQuery set_query = 9;\n  }", "aggregate proto oneof")
edit(MUT + "💾️binary/📡️.protocol.semio", "record remove-data-property tag=7\nfield payload bytes\n", "record remove-data-property tag=7\nfield payload bytes\nrecord set-query tag=8\nfield payload bytes\n", "binary protocol record")
edit(MUT + "💾️binary/🦀️.rs", '    ("RemoveDataProperty", super::remove_data_property::binary::BINARY_TAG),\n];', '    ("RemoveDataProperty", super::remove_data_property::binary::BINARY_TAG),\n    ("SetQuery", super::set_query::binary::BINARY_TAG),\n];', "binary registry")
edit(MUT + "📝️text/🦀️.rs", '    ("RemoveDataProperty", super::remove_data_property::text::TEXT_OPCODE),\n];', '    ("RemoveDataProperty", super::remove_data_property::text::TEXT_OPCODE),\n    ("SetQuery", super::set_query::text::TEXT_OPCODE),\n];', "text registry")
edit(MUT + "📝️text/📖️.grammar.semio", "/ change-data-property / remove-data-property\n", "/ change-data-property / remove-data-property / set-query\n", "grammar line")
edit(MUT + "📝️text/📖️.grammar.semio", 'remove-data-property = "remove-data-property" SP entity SP text\n', 'remove-data-property = "remove-data-property" SP entity SP text\nset-query = "set-query" SP text\n', "grammar rule")
edit(MUT + "📝️text/🅰️.g4", "| changeDataProperty | removeDataProperty ;", "| changeDataProperty | removeDataProperty | setQuery ;", "g4 line")
edit(MUT + "📝️text/🅰️.g4", "removeDataProperty: 'remove-data-property' SP entity SP text ;\n", "removeDataProperty: 'remove-data-property' SP entity SP text ;\nsetQuery: 'set-query' SP text ;\n", "g4 rule")
edit(MUT + "📝️text/🔤️.ebnf", "     | remove data property ;", "     | remove data property\n     | set query ;", "ebnf line")
edit(MUT + "📝️text/🔤️.ebnf", "remove data property = 'remove-data-property', space, entity, space, text ;\n", "remove data property = 'remove-data-property', space, entity, space, text ;\nset query = 'set-query', space, text ;\n", "ebnf rule")

# ── crate module tree ──
MODULE_ANCHOR = '''                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹️remove-data-property/📝️text/🦀️.rs"]
                            pub mod text;
                        }
'''
edit(ROOT, MODULE_ANCHOR, MODULE_ANCHOR + '''                        #[path = "."]
                        pub mod set_query {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔎️set-query/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔎️set-query/💾️binary/🦀️.rs"]
                            pub mod binary;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔎️set-query/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔎️set-query/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔎️set-query/🧪️tests/''' + CASE + '''/🦀️.rs"]
                            mod tests_replaces_the_query;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔎️set-query/📝️text/🦀️.rs"]
                            pub mod text;
                        }
''', "module tree set_query")

# ── the leaf's fixture quartet (the one jack case that CAN pin a state-changing branch: no content re-mint) ──
BEFORE_SNAPSHOT = {
    "schema": "trinity.graph",
    "name": "Capsule Stack",
    "manifest": {"nodeKinds": [], "edgeKinds": [], "portKinds": []},
    "camera": {"x": 0.0, "y": 0.0, "zoom": 1.0},
    "content": {"childId": "jack-content-set-query-fixture", "target": {"artifactId": "jack-content-set-query-fixture", "dialect": {"artifactKind": "s.stdio.semio", "standard": "v1", "subset": "graph"}}},
    "query": DEFAULT_QUERY,
}
AFTER_SNAPSHOT = dict(BEFORE_SNAPSHOT, query=NEW_QUERY)
DIFF_SLOTS = {key: None for key in ("schema", "name", "manifestId", "manifest", "camera", "content", "rootNodeId")}
for rel, value, label in (
    ("📸️snapshot/⬅️before/🔣️.json", BEFORE_SNAPSHOT, "before"),
    ("📸️snapshot/➡️after/🔣️.json", AFTER_SNAPSHOT, "after"),
    ("🦠️mutation/🔣️.json", {"mutation": "setQuery", "value": NEW_QUERY}, "mutation"),
    ("🔺️diff/🔣️.json", dict(DIFF_SLOTS, query=NEW_QUERY), "diff"),
    ("🎯️outcome/🔣️.json", {"status": "applied", "messages": []}, "outcome"),
):
    create(FIXTURE + rel, json.dumps(value, indent=2, ensure_ascii=False) + "\n", f"fixture set-query {label}")

create(LEAF + "🧪️tests/" + CASE + "/🦀️.rs", '''//! 🧪️ `set-query` fixture — `''' + CASE + '''`.
//!
//! Source of truth is the committed JSON quintet under `🧫️fixtures/🧬️mutations/🔎️set-query/''' + CASE + '''` (contract D1,
//! ticket `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Unlike the graph verbs, `set-query` never touches the composed content
//! child, so this case pins the state-CHANGING branch: the query moves, the content handle stays exactly where it was.

use crate::standards::v1::subsets::any::schema::diff::JackDiff;
use crate::standards::v1::subsets::any::schema::mutations::TrinityGraphMutation;
use crate::{apply_trinity_graph_mutation, inverse_trinity_graph_mutation, JackSnapshot};

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔎️set-query/''' + CASE + '''/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔎️set-query/''' + CASE + '''/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔎️set-query/''' + CASE + '''/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔎️set-query/''' + CASE + '''/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔎️set-query/''' + CASE + '''/🎯️outcome/🔣️.json");

fn before() -> JackSnapshot {
    pack::from_json_str(BEFORE).expect("before snapshot decodes")
}
fn after() -> JackSnapshot {
    pack::from_json_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> TrinityGraphMutation {
    pack::from_json_str(MUTATION).expect("mutation decodes")
}

/// ▶️ The committed `set-query` carries `⬅️before` to `➡️after`, and the content handle does not move.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_trinity_graph_mutation(&mut snapshot, &mutation()).expect("set-query applies");
    assert_eq!(snapshot, after(), "set-query/''' + CASE + ''': applied state differs from the committed after-snapshot");
    assert_eq!(snapshot.content.child_id, before().content.child_id, "set-query must never re-mint the composed content handle");
}

/// ↩️ The inverse is one `set-query` back to BASE's query, and it restores `⬅️before` exactly.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let inverse = inverse_trinity_graph_mutation(&base, &mutation());
    assert_eq!(inverse.len(), 1, "set-query emits exactly one undo step, got {inverse:?}");
    let TrinityGraphMutation::SetQuery(undo) = &inverse[0] else {
        panic!("set-query's inverse must itself be a set-query, got {:?}", inverse[0]);
    };
    assert_eq!(undo.value, base.query, "the inverse restores the query BASE held");
    let mut snapshot = base.clone();
    apply_trinity_graph_mutation(&mut snapshot, &mutation()).expect("forward applies");
    for step in &inverse {
        apply_trinity_graph_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "set-query/''' + CASE + ''': inverse did not restore the before-snapshot");
}

/// 🔺️ The produced diff is the committed one: every slot `None` except `query`.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <TrinityGraphMutation as protocol::Mutation<JackSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::from_str::<serde_json::Value>(&pack::to_json_string(outcome.diff())).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "set-query/''' + CASE + ''': produced diff differs from the committed 🔺️diff/🔣️.json");
    let typed: JackDiff = pack::from_json_str(DIFF).expect("committed diff decodes into JackDiff");
    assert_eq!(typed, JackDiff { query: Some(after().query), ..JackDiff::default() }, "set-query moves the query slot and nothing else");
    assert_eq!(serde_json::from_str::<serde_json::Value>(DIFF).expect("diff reparses").as_object().map(serde_json::Map::len), Some(8), "JackDiff emits all eight document slots");
}

/// 🩹 Applying the committed diff to `⬅️before` yields `➡️after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: JackDiff = pack::from_json_str(DIFF).expect("committed diff decodes");
    let produced = <JackDiff as protocol::MutationDiff<JackSnapshot>>::apply(&decoded, &before()).expect("committed diff applies");
    assert_eq!(produced, after(), "set-query/''' + CASE + ''': committed diff did not carry before to after");
}

/// 🎯️ The declared outcome — `applied`, without diagnostics — is what `set-query` emits; the same text again is a warned no-op.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"));
    let produced = <TrinityGraphMutation as protocol::Mutation<JackSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "an applied set-query carries no diagnostics, got {:?}", produced.messages());
    let repeated = <TrinityGraphMutation as protocol::Mutation<JackSnapshot>>::diff(&mutation(), &after());
    assert_eq!(repeated.diff(), &JackDiff::default(), "setting the query the document already holds changes nothing");
    assert_eq!(repeated.messages().first().map(|message| message.code.0.as_str()), Some("mutation.no-op"));
}

/// 🔣️ The committed snapshots, mutation and diff are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: JackSnapshot = pack::from_json_str(text).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&pack::to_json_string(&decoded)).expect("snapshot encodes");
        assert_eq!(reencoded, serde_json::from_str::<serde_json::Value>(text).expect("snapshot reparses"), "set-query: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::from_str::<serde_json::Value>(&pack::to_json_string(&mutation())).expect("mutation encodes");
    assert_eq!(reencoded, serde_json::from_str::<serde_json::Value>(MUTATION).expect("mutation reparses"), "set-query: committed mutation JSON is not canonical");
    let decoded: JackDiff = pack::from_json_str(DIFF).expect("diff decodes");
    let reencoded = serde_json::from_str::<serde_json::Value>(&pack::to_json_string(&decoded)).expect("diff encodes");
    assert_eq!(reencoded, serde_json::from_str::<serde_json::Value>(DIFF).expect("diff reparses"), "set-query: committed diff JSON is not canonical");
}
''', "leaf law")
