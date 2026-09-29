#!/usr/bin/env python3
"""📥️ S19 set `sourcing-stock` (guest, T6 round 4+; coordinator decision 20:3x): an agent must be able to stock a blank hub
catalogue. The curation vocabulary had NO stock verb — stock reached the document only through `ArtifactStore::reset`
(`stockFromCatalogue`/`setActiveExample`/`setDocument` = whole-document replacements, which an agent transaction cannot
carry: `agent-lane-uncarried`), so on a hub-created (blank) curation an agent could create the document and nothing else.

Schema-first, two new direct mutation owners (every language surface + fixtures + both oracles):
- `create-stock-object` (`📥️`, binary tag 3): stocks one new object kind (`ObjectKindExtra` payload) — the stock gains the
  row and the composed kit catalogue child is re-pointed at the content-addressed handle of the new stock
  (`catalog_child_handle`, so the framework follows the derivable child). Fatal `mutation.duplicate-id` when stocked.
- `delete-stock-object` (`📤️`, tag 4): unstocks one object — `mutation.target-missing` when absent, Fatal
  `mutation.invariant` while it is curated. Inverse pair: create ↔ delete (delete's inverse re-stocks the removed row).
`stockFromCatalogue` now EMITS these mutations (one per installed-module kind not yet stocked, optional `moduleId` to stock
one module) on the Artifact lane instead of a document reset → undoable, carried by the agent lane; en + de description.
Laws: both leaves' fixture scenarios (Rust), the Python second implementation + feature rows (mutate / inverse /
spec-vector; the Python oracle computes the content-addressed catalogue id itself: `catalog-` + sha256(compact JSON of
the kit types)[..8] — verified against the committed demo id `catalog-32ab638c359f7f13`), the structural-correspondence
blocks, and the editor law `an_agent_stocks_and_curates_a_blank_catalogue` (blank example loaded → agent-lane
`stockFromCatalogue` → `curationAdd`, each prepared + committed as an agent transaction). Measured on the way: a hub-created
curation is the DEMO stock (`initial_snapshot` = `default_document`), so G12's 'no-change' came from invented ids, not a
blank catalogue — the verb is for blank or imported curations and for restocking after new module contributions.
usage: s19-sourcing-stock.py [--dry-run|--write|--revert]"""
import hashlib
import importlib.util
import json
import os
import sys

spec = importlib.util.spec_from_file_location("s19_setlib", os.path.join(os.path.dirname(os.path.abspath(__file__)), "s19_setlib.py"))
lib = importlib.util.module_from_spec(spec)
spec.loader.exec_module(lib)

CUR = "✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation"
ANY = f"{CUR}/🏅️standards/🔖️1/🪆️subsets/✳️any"
MUT = f"{ANY}/🧬️schema/🧬️mutations"
FIX = f"{ANY}/🧫️fixtures/🧬️mutations"
CREATE_DIR, DELETE_DIR = "📥️create-stock-object", "📤️delete-stock-object"
CREATE_CASE, DELETE_CASE = "🪵️stocks-a-glulam-beam-into-a-blank-catalogue", "🧹️unstocks-an-uncurated-steel-beam"
TAXONOMY = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"


def create_file(content):
    def transform(text):
        if text is not None and text == content:
            return text
        assert text is None, "file already exists with other content"
        return content
    return transform


# ── fixtures (JSON quintets; catalogue ids computed exactly as `catalog_child_handle`) ─────────────────────────────────
def handle(stock):
    types = [{"id": row["id"], "name": row["name"], "category": row["moduleId"]} for row in stock]
    child_id = "catalog-" + hashlib.sha256(json.dumps(types, separators=(",", ":"), ensure_ascii=False).encode()).hexdigest()[:16]
    return {"childId": child_id, "target": {"artifactId": child_id, "dialect": {"artifactKind": "s.stdio.semio", "standard": "v1", "subset": "kit"}}}


assert handle([{"id": i, "name": n, "moduleId": c} for i, n, c in [("beam-glulam-gl24h", "Glulam GL24h 200×400", "beams"), ("beam-kvh-c24", "KVH C24 100×200", "beams"), ("beam-steel-ipe200", "Steel IPE 200", "beams"), ("beam-steel-hea160", "Steel HEA 160", "beams"), ("window-casement-100x120", "Casement Window 100×120", "windows"), ("window-fixed-150x150", "Fixed Window 150×150", "windows"), ("window-tilt-turn-120x140", "Tilt & Turn Window 120×140", "windows"), ("slab-concrete-240", "Concrete Slab 240mm", "slabs"), ("slab-clt-160", "CLT Slab 160mm", "slabs"), ("slab-hollow-core-265", "Hollow Core Slab 265mm", "slabs"), ("hexagonal-cut-concrete-forest-left", "Hexagonal Cut Concrete Forest Left", "reuse"), ("hexagonal-cut-concrete-forest-right", "Hexagonal Cut Concrete Forest Right", "reuse")]])["childId"] == "catalog-32ab638c359f7f13", "catalogue id rule drifted from the committed demo"

GLULAM = {"id": "beam-glulam-gl24h", "name": "Glulam GL24h 200×400", "moduleId": "beams", "typologyPath": ["beams", "solid-timber", "glulam"], "availability": 24, "geometry": {"kind": "box", "width": 0.2, "height": 0.4, "depth": 6.0}}
IPE = {"id": "beam-steel-ipe200", "name": "Steel IPE 200", "moduleId": "beams", "typologyPath": ["beams", "steel", "ipe"], "availability": 12, "geometry": {"kind": "box", "width": 0.1, "height": 0.2, "depth": 5.0}}


def dump(value):
    return json.dumps(value, indent=2, ensure_ascii=False) + "\n"


def delta(added=(), removed=()):
    return {"added": list(added), "removed": list(removed), "patched": [], "reordered": None}


QUINTETS = {
    (CREATE_DIR, CREATE_CASE): {
        "📸️snapshot/⬅️before": {"catalog": handle([]), "stockExtra": [], "curated": []},
        "🦠️mutation": {"mutation": "createStockObject", "kind": GLULAM},
        "📸️snapshot/➡️after": {"catalog": handle([GLULAM]), "stockExtra": [GLULAM], "curated": []},
        "🔺️diff": {"artifact": None, "catalog": handle([GLULAM]), "stockExtra": delta(added=[GLULAM]), "curated": None},
        "🎯️outcome": {"status": "applied"},
    },
    (DELETE_DIR, DELETE_CASE): {
        "📸️snapshot/⬅️before": {"catalog": handle([GLULAM, IPE]), "stockExtra": [GLULAM, IPE], "curated": [{"objectId": "beam-glulam-gl24h", "count": 2}]},
        "🦠️mutation": {"mutation": "deleteStockObject", "objectId": "beam-steel-ipe200"},
        "📸️snapshot/➡️after": {"catalog": handle([GLULAM]), "stockExtra": [GLULAM], "curated": [{"objectId": "beam-glulam-gl24h", "count": 2}]},
        "🔺️diff": {"artifact": None, "catalog": handle([GLULAM]), "stockExtra": delta(removed=["beam-steel-ipe200"]), "curated": None},
        "🎯️outcome": {"status": "applied"},
    },
}

# ── leaf owners ────────────────────────────────────────────────────────────────────────────────────────────────────────
LEAVES = {
    CREATE_DIR: {
        "kind": "create-stock-object", "variant": "CreateStockObject", "module": "create_stock_object", "tag": 3, "emoji": "📥️", "tag_name": "createStockObject",
        "rs": '''//! 📥️ Direct `create-stock-object` mutation owner: brings one new object kind into the stock and re-points the composed
//! kit catalogue child at the content-addressed handle of the grown stock.
use crate::diff::CurationDiff;
use crate::mutations::SourcingMutation;
use crate::{CurationSnapshot, ObjectKindExtra};

//#region 🔖️Mutation
/// 📥️ `create-stock-object` payload — the full stock row (identity, typology, availability, geometry); a later change of
/// the row goes through `delete-stock-object` + `create-stock-object`.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-stock-object")]
pub struct CreateStockObject {
    #[dsl(block)]
    pub kind: ObjectKindExtra,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_stock_object(kind: ObjectKindExtra) -> SourcingMutation {
    SourcingMutation::CreateStockObject(CreateStockObject { kind })
}

impl protocol::MutationKind<CurationSnapshot, SourcingMutation> for CreateStockObject {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "stock-object", kind: "create-stock-object", record: "CreatedStockObject" };

    fn diff(&self, base: &CurationSnapshot) -> protocol::MutationOutcome<CurationDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &CurationSnapshot) -> Vec<SourcingMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Stock \\"{}\\"", self.kind.id), &format!("Bestand \\"{}\\"", self.kind.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.kind.id.clone()]
    }
}
//#endregion 🔖️Mutation
''',
        "diff_rs": '''//! 🔺 Sparse diff builder for `CreateStockObject` — one `added` stock row plus the content-addressed catalogue handle of
//! the grown stock (the composed kit child is a pure function of the stock). Fatal `duplicate-id` when already stocked,
//! `malformed-payload` for an empty id.
use crate::diff::{CurationDiff, CurationStockExtraDelta};
use crate::CurationSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::CreateStockObject, base: &CurationSnapshot) -> protocol::MutationOutcome<CurationDiff> {
    if payload.kind.id.is_empty() {
        return protocol::MutationOutcome::fatal("mutation.malformed-payload", "a stock object needs an id.".to_string(), [String::new()]);
    }
    if base.stock_extra.iter().any(|extra| extra.id == payload.kind.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("\\"{}\\" is already stocked.", payload.kind.id), [payload.kind.id.clone()]);
    }
    let mut stock = crate::stock_of(base);
    stock.push(crate::ObjectKind { id: payload.kind.id.clone(), name: payload.kind.name.clone(), module_id: payload.kind.module_id.clone(), typology_path: payload.kind.typology_path.clone(), availability: payload.kind.availability, geometry: payload.kind.geometry.clone() });
    protocol::MutationOutcome::new(CurationDiff { catalog: Some(crate::catalog_child_handle(&stock)), stock_extra: Some(CurationStockExtraDelta { added: vec![payload.kind.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
''',
        "diff_ts": '''/** 🔺️ sourcing curation create-stock-object/🔺️diff — mirror of the one-row stock insert (the catalogue handle is the
 * content-addressed id of the grown stock, computed by the owner that holds the stock). */
import type { CreateStockObject } from "../🟦️.ts";
import type { CurationStockExtraDelta } from "../../../🔺️diff/🟦️.ts";

export function diff(payload: CreateStockObject): { stockExtra: CurationStockExtraDelta } {
  return { stockExtra: { added: [payload.kind], removed: [], patched: [], reordered: null } };
}
''',
        "inverse_rs": '''//! ↩️ Inverse for `CreateStockObject` — always a `delete-stock-object` of the id it stocked (a new row is uncurated).
use crate::mutations::SourcingMutation;
use crate::CurationSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::CreateStockObject, _base: &CurationSnapshot) -> Vec<SourcingMutation> {
    vec![crate::mutations::delete_stock_object::delete_stock_object(payload.kind.id.clone())]
}
//#endregion 🔖️Inverse
''',
        "inverse_ts": '''/** ↩️ sourcing curation create-stock-object/↩️inverse — mirror of the id-only delete-stock-object inverse. */
import type { CreateStockObject } from "../🟦️.ts";
import type { DeleteStockObject } from "../../📤️delete-stock-object/🟦️.ts";

export function inverse(payload: CreateStockObject): [DeleteStockObject] {
  return [{ objectId: payload.kind.id }];
}
''',
        "ts": '''/** 📥️ Direct `create-stock-object` payload. */
import type { ObjectKindExtra } from "../../🟦️.ts";

export interface CreateStockObject {
  kind: ObjectKindExtra;
}
''',
        "graphql": '''"""📥️ Direct create-stock-object payload."""
# import GeometryRecipeInput from "../🔗️.graphql"
input CreateStockObjectInput {
  kind: ObjectKindExtraInput!
}

input ObjectKindExtraInput {
  id: String!
  name: String!
  moduleId: String!
  typologyPath: [String!]!
  availability: Int!
  geometry: GeometryRecipeInput!
}
''',
        "proto": '''syntax = "proto3";
import "✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🛰️.proto";
package semio.s.sourcing.curation.mutation;

message CreateStockObject {
  semio.s.sourcing.curation.artifact.ObjectKindExtra kind = 1;
}
''',
        "schema": {"$schema": "http://json-schema.org/draft-07/schema#", "$id": "https://json.schemas.assets.semio-tech.com/s/sourcing/curation/mutation/create-stock-object/schema.json", "title": "CreateStockObject", "type": "object", "additionalProperties": False, "required": ["kind"], "properties": {"kind": {"$ref": "https://json.schemas.assets.semio-tech.com/s/sourcing/curation/artifact.json#/$defs/ObjectKindExtra"}}},
        "display": "Create Stock Object",
        "case": CREATE_CASE,
        "case_rs": '''//! 🧪️ `create-stock-object` fixture — `🪵️stocks-a-glulam-beam-into-a-blank-catalogue`: a blank (hub-created) curation gains
//! one stock row, and its composed catalogue handle moves to the content-addressed id of the grown stock.

use crate::diff::CurationDiff;
use crate::mutations::SourcingMutation;
use crate::CurationSnapshot;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📥️create-stock-object/🪵️stocks-a-glulam-beam-into-a-blank-catalogue/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📥️create-stock-object/🪵️stocks-a-glulam-beam-into-a-blank-catalogue/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📥️create-stock-object/🪵️stocks-a-glulam-beam-into-a-blank-catalogue/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📥️create-stock-object/🪵️stocks-a-glulam-beam-into-a-blank-catalogue/🔺️diff/🔣️.json");

fn before() -> CurationSnapshot {
    dsl::json::from_json_str(BEFORE).expect("before curation document decodes")
}
fn mutation() -> SourcingMutation {
    dsl::json::from_json_str(MUTATION).expect("create-stock-object mutation decodes")
}
fn built_outcome() -> protocol::MutationOutcome<CurationDiff> {
    <SourcingMutation as protocol::Mutation<CurationSnapshot>>::diff(&mutation(), &before())
}

/// ▶️ Stocking the glulam beam lands on the committed after-document: one stock row and the grown stock's catalogue handle.
#[semio_framework_async_macros::async_test]
async fn stocks_the_beam_and_follows_the_catalogue_handle() {
    let outcome = built_outcome();
    assert!(outcome.messages().is_empty(), "create-stock-object raised {:?}", outcome.messages());
    assert_eq!(outcome.diff(), &dsl::json::from_json_str::<CurationDiff>(DIFF).expect("committed diff decodes"), "create-stock-object: the diff differs from the committed one");
    let applied = protocol::MutationDiff::apply(outcome.diff(), &before()).expect("create-stock-object applies to its committed before-document");
    assert_eq!(applied, dsl::json::from_json_str::<CurationSnapshot>(AFTER).expect("after curation document decodes"), "create-stock-object: the stocked curation differs from the committed after-snapshot");
}

/// ↩️ The inverse (`delete-stock-object`) restores the blank catalogue exactly, handle included.
#[semio_framework_async_macros::async_test]
async fn unstocking_the_new_beam_restores_before() {
    let base = before();
    let mut snapshot = protocol::MutationDiff::apply(built_outcome().diff(), &base).expect("forward create-stock-object applies");
    for step in <SourcingMutation as protocol::Mutation<CurationSnapshot>>::inverse(&mutation(), &base) {
        let undo = <SourcingMutation as protocol::Mutation<CurationSnapshot>>::diff(&step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the delete-stock-object inverse step applies");
    }
    assert_eq!(snapshot, base, "create-stock-object: unstocking the beam did not restore the blank catalogue");
}

/// 🚫️ Stocking an object twice is refused by name.
#[semio_framework_async_macros::async_test]
async fn stocking_twice_is_refused() {
    let stocked = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("forward create-stock-object applies");
    let again = <SourcingMutation as protocol::Mutation<CurationSnapshot>>::diff(&mutation(), &stocked);
    assert!(again.messages().iter().any(|message| message.code.0 == "mutation.duplicate-id"), "a second stock of the beam must be refused: {:?}", again.messages());
}
''',
    },
    DELETE_DIR: {
        "kind": "delete-stock-object", "variant": "DeleteStockObject", "module": "delete_stock_object", "tag": 4, "emoji": "📤️", "tag_name": "deleteStockObject",
        "rs": '''//! 📤️ Direct `delete-stock-object` mutation owner: takes one uncurated object kind out of the stock and re-points the
//! composed kit catalogue child at the content-addressed handle of the shrunk stock.
use crate::diff::CurationDiff;
use crate::mutations::SourcingMutation;
use crate::CurationSnapshot;

//#region 🔖️Mutation
/// 📤️ `delete-stock-object` payload — the stocked object's id.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "delete-stock-object")]
pub struct DeleteStockObject {
    pub object_id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn delete_stock_object(object_id: String) -> SourcingMutation {
    SourcingMutation::DeleteStockObject(DeleteStockObject { object_id })
}

impl protocol::MutationKind<CurationSnapshot, SourcingMutation> for DeleteStockObject {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "stock-object", kind: "delete-stock-object", record: "DeletedStockObject" };

    fn diff(&self, base: &CurationSnapshot) -> protocol::MutationOutcome<CurationDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &CurationSnapshot) -> Vec<SourcingMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Unstock \\"{}\\"", self.object_id), &format!("Aus dem Bestand \\"{}\\"", self.object_id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.object_id.clone()]
    }
}
//#endregion 🔖️Mutation
''',
        "diff_rs": '''//! 🔺 Sparse diff builder for `DeleteStockObject` — one `removed` stock row plus the content-addressed catalogue handle of
//! the shrunk stock. `target-missing` when the object is not stocked; Fatal `invariant` while it is still curated (the
//! curation would name an object the stock no longer holds).
use crate::diff::{CurationDiff, CurationStockExtraDelta};
use crate::CurationSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::DeleteStockObject, base: &CurationSnapshot) -> protocol::MutationOutcome<CurationDiff> {
    if !base.stock_extra.iter().any(|extra| extra.id == payload.object_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("\\"{}\\" is not stocked.", payload.object_id), [payload.object_id.clone()]);
    }
    if base.curated.iter().any(|item| item.object_id == payload.object_id) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("\\"{}\\" is curated; remove it from the curation before unstocking it.", payload.object_id), [payload.object_id.clone()]);
    }
    let stock: Vec<_> = crate::stock_of(base).into_iter().filter(|kind| kind.id != payload.object_id).collect();
    protocol::MutationOutcome::new(CurationDiff { catalog: Some(crate::catalog_child_handle(&stock)), stock_extra: Some(CurationStockExtraDelta { removed: vec![payload.object_id.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
''',
        "diff_ts": '''/** 🔺️ sourcing curation delete-stock-object/🔺️diff — mirror of the one-row stock removal (the catalogue handle is the
 * content-addressed id of the shrunk stock, computed by the owner that holds the stock). */
import type { DeleteStockObject } from "../🟦️.ts";
import type { CurationStockExtraDelta } from "../../../🔺️diff/🟦️.ts";

export function diff(payload: DeleteStockObject): { stockExtra: CurationStockExtraDelta } {
  return { stockExtra: { added: [], removed: [payload.objectId], patched: [], reordered: null } };
}
''',
        "inverse_rs": '''//! ↩️ Inverse for `DeleteStockObject` — a `create-stock-object` of the exact row the BASE held (a delete is only admitted for
//! an uncurated object, so re-stocking that row restores the document).
use crate::mutations::SourcingMutation;
use crate::CurationSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::DeleteStockObject, base: &CurationSnapshot) -> Vec<SourcingMutation> {
    base.stock_extra.iter().find(|extra| extra.id == payload.object_id).map(|extra| vec![crate::mutations::create_stock_object::create_stock_object(extra.clone())]).unwrap_or_default()
}
//#endregion 🔖️Inverse
''',
        "inverse_ts": '''/** ↩️ sourcing curation delete-stock-object/↩️inverse — mirror of the base-row create-stock-object inverse. */
import type { DeleteStockObject } from "../🟦️.ts";
import type { CreateStockObject } from "../../📥️create-stock-object/🟦️.ts";
import type { ObjectKindExtra } from "../../../🟦️.ts";

export function inverse(payload: DeleteStockObject, stockExtra: readonly ObjectKindExtra[]): CreateStockObject[] {
  const row = stockExtra.find((extra) => extra.id === payload.objectId);
  return row ? [{ kind: row }] : [];
}
''',
        "ts": '''/** 📤️ Direct `delete-stock-object` payload. */
export interface DeleteStockObject {
  objectId: string;
}
''',
        "graphql": '''"""📤️ Direct delete-stock-object payload."""
input DeleteStockObjectInput {
  objectId: String!
}
''',
        "proto": '''syntax = "proto3";
package semio.s.sourcing.curation.mutation;

message DeleteStockObject {
  string object_id = 1;
}
''',
        "schema": {"$schema": "http://json-schema.org/draft-07/schema#", "$id": "https://json.schemas.assets.semio-tech.com/s/sourcing/curation/mutation/delete-stock-object/schema.json", "title": "DeleteStockObject", "type": "object", "additionalProperties": False, "required": ["objectId"], "properties": {"objectId": {"type": "string"}}},
        "display": "Delete Stock Object",
        "case": DELETE_CASE,
        "case_rs": '''//! 🧪️ `delete-stock-object` fixture — `🧹️unstocks-an-uncurated-steel-beam`: an uncurated stock row leaves the stock, the
//! curation stays, and the composed catalogue handle moves to the content-addressed id of the shrunk stock.

use crate::diff::CurationDiff;
use crate::mutations::SourcingMutation;
use crate::CurationSnapshot;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📤️delete-stock-object/🧹️unstocks-an-uncurated-steel-beam/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📤️delete-stock-object/🧹️unstocks-an-uncurated-steel-beam/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📤️delete-stock-object/🧹️unstocks-an-uncurated-steel-beam/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📤️delete-stock-object/🧹️unstocks-an-uncurated-steel-beam/🔺️diff/🔣️.json");

fn before() -> CurationSnapshot {
    dsl::json::from_json_str(BEFORE).expect("before curation document decodes")
}
fn mutation() -> SourcingMutation {
    dsl::json::from_json_str(MUTATION).expect("delete-stock-object mutation decodes")
}
fn built_outcome() -> protocol::MutationOutcome<CurationDiff> {
    <SourcingMutation as protocol::Mutation<CurationSnapshot>>::diff(&mutation(), &before())
}

/// ▶️ Unstocking the steel beam lands on the committed after-document.
#[semio_framework_async_macros::async_test]
async fn unstocks_the_beam_and_follows_the_catalogue_handle() {
    let outcome = built_outcome();
    assert!(outcome.messages().is_empty(), "delete-stock-object raised {:?}", outcome.messages());
    assert_eq!(outcome.diff(), &dsl::json::from_json_str::<CurationDiff>(DIFF).expect("committed diff decodes"), "delete-stock-object: the diff differs from the committed one");
    let applied = protocol::MutationDiff::apply(outcome.diff(), &before()).expect("delete-stock-object applies to its committed before-document");
    assert_eq!(applied, dsl::json::from_json_str::<CurationSnapshot>(AFTER).expect("after curation document decodes"), "delete-stock-object: the shrunk stock differs from the committed after-snapshot");
}

/// ↩️ The inverse (`create-stock-object` of the base row) restores the before-document exactly.
#[semio_framework_async_macros::async_test]
async fn restocking_the_beam_restores_before() {
    let base = before();
    let mut snapshot = protocol::MutationDiff::apply(built_outcome().diff(), &base).expect("forward delete-stock-object applies");
    for step in <SourcingMutation as protocol::Mutation<CurationSnapshot>>::inverse(&mutation(), &base) {
        let undo = <SourcingMutation as protocol::Mutation<CurationSnapshot>>::diff(&step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the create-stock-object inverse step applies");
    }
    assert_eq!(snapshot, base, "delete-stock-object: re-stocking the beam did not restore the before-document");
}

/// 🚫️ A curated object cannot leave the stock.
#[semio_framework_async_macros::async_test]
async fn a_curated_object_stays_stocked() {
    let curated = SourcingMutation::DeleteStockObject(super::DeleteStockObject { object_id: "beam-glulam-gl24h".into() });
    let refused = <SourcingMutation as protocol::Mutation<CurationSnapshot>>::diff(&curated, &before());
    assert!(refused.messages().iter().any(|message| message.code.0 == "mutation.invariant"), "unstocking a curated object must be refused: {:?}", refused.messages());
}
''',
    },
}


def descriptor(directory, leaf):
    return dump({"schemaVersion": 1, "owner": f"{MUT}/{directory}", "semanticKind": leaf["kind"], "displayName": leaf["display"], "emoji": leaf["emoji"].rstrip("️"), "aggregateVariant": leaf["variant"], "payloadSchema": "🧬️schema/🔣️.json", "textOpcode": leaf["kind"], "binaryTag": leaf["tag"], "invertibility": "explicit-mutation", "diffParticipation": "detect", "outcomeClasses": ["applied", "rejected"], "composition": "atomic", "requiredLanguageSurfaces": ["rust", "typescript", "graphql", "protobuf", "json-schema", "text", "binary"]})


EDITS = []
for directory, leaf in LEAVES.items():
    base = f"{MUT}/{directory}"
    for rel, content in [
        ("🦀️.rs", leaf["rs"]),
        ("🔺️diff/🦀️.rs", leaf["diff_rs"]),
        ("🔺️diff/🟦️.ts", leaf["diff_ts"]),
        ("↩️inverse/🦀️.rs", leaf["inverse_rs"]),
        ("↩️inverse/🟦️.ts", leaf["inverse_ts"]),
        ("💾️binary/🦀️.rs", f"//! {leaf['emoji']} Binary identity owned by the direct `{leaf['kind']}` leaf.\n\npub const BINARY_TAG: u8 = {leaf['tag']};\n"),
        ("📝️text/🦀️.rs", f"//! {leaf['emoji']} Text identity owned by the direct `{leaf['kind']}` leaf.\n\npub const TEXT_OPCODE: &str = \"{leaf['kind']}\";\n"),
        ("🔣️.json", descriptor(directory, leaf)),
        ("🧬️schema/🔣️.json", dump(leaf["schema"])),
        ("🟦️.ts", leaf["ts"]),
        ("🔗️.graphql", leaf["graphql"]),
        ("🛰️.proto", leaf["proto"]),
        (f"🧪️tests/{leaf['case']}/🦀️.rs", leaf["case_rs"]),
    ]:
        EDITS.append((f"{base}/{rel}", create_file(content)))
for (directory, case), files in QUINTETS.items():
    for rel, value in files.items():
        EDITS.append((f"{FIX}/{directory}/{case}/{rel}/🔣️.json", create_file(dump(value))))


# ── aggregates ─────────────────────────────────────────────────────────────────────────────────────────────────────────
def aggregate_rs(text):
    text = lib.replace_once("pub use super::delete_curated_item::{delete_curated_item, DeleteCuratedItem};\n", "pub use super::delete_curated_item::{delete_curated_item, DeleteCuratedItem};\npub use super::create_stock_object::{create_stock_object, CreateStockObject};\npub use super::delete_stock_object::{delete_stock_object, DeleteStockObject};\n")(text)
    text = lib.replace_once("/// 🧮️ Closed curated-selection mutation vocabulary backed by direct semantic owners.", "/// 🧮️ Closed curation mutation vocabulary backed by direct semantic owners — the curated selection and the stock.")(text)
    return lib.replace_once("    ChangeCuratedItemCount(ChangeCuratedItemCount),\n}", "    ChangeCuratedItemCount(ChangeCuratedItemCount),\n    CreateStockObject(CreateStockObject),\n    DeleteStockObject(DeleteStockObject),\n}")(text)


def aggregate_json(text):
    value = json.loads(text)
    refs = [entry["$ref"] for entry in value["oneOf"]]
    for kind in ("create-stock-object", "delete-stock-object"):
        ref = f"https://json.schemas.assets.semio-tech.com/s/sourcing/curation/mutation/{kind}/schema.json"
        if ref not in refs:
            value["oneOf"].append({"$ref": ref})
    value["oneOf"].sort(key=lambda entry: entry["$ref"])
    return dump(value)


def binary_rs(text):
    return lib.replace_once('("ChangeCuratedItemCount", super::change_curated_item_count::binary::BINARY_TAG)]', '("ChangeCuratedItemCount", super::change_curated_item_count::binary::BINARY_TAG), ("CreateStockObject", super::create_stock_object::binary::BINARY_TAG), ("DeleteStockObject", super::delete_stock_object::binary::BINARY_TAG)]')(text)


def text_rs(text):
    return lib.replace_once('("ChangeCuratedItemCount", super::change_curated_item_count::text::TEXT_OPCODE)]', '("ChangeCuratedItemCount", super::change_curated_item_count::text::TEXT_OPCODE), ("CreateStockObject", super::create_stock_object::text::TEXT_OPCODE), ("DeleteStockObject", super::delete_stock_object::text::TEXT_OPCODE)]')(text)


def grammar(text):
    text = lib.replace_once("mutation-stmt = create-curated-item | delete-curated-item | change-curated-item-count\n", "mutation-stmt = create-curated-item | delete-curated-item | change-curated-item-count | create-stock-object | delete-stock-object\n")(text)
    if "create-stock-object =" in text:
        return text
    return text.rstrip("\n") + '\ncreate-stock-object = "create-stock-object" "id" "=" IDENT "name" "=" STRING "module-id" "=" IDENT "availability" "=" INT "typology-path" "=" "[" IDENT* "]" geometry-recipe\ngeometry-recipe = ("box" | "frame" | "slab" | "mesh" | "glb") (IDENT "=" VALUE)+\ndelete-stock-object = "delete-stock-object" "object-id" "=" IDENT\n'


def graphql(text):
    text = lib.replace_once("input ChangeCuratedItemCountInput { objectId: String!, newCount: Int! }\n", "input ChangeCuratedItemCountInput { objectId: String!, newCount: Int! }\ninput GeometryRecipeBoxInput { width: Float!, height: Float!, depth: Float! }\ninput GeometryRecipeFrameInput { width: Float!, height: Float!, depth: Float!, profile: Float! }\ninput GeometryRecipeSlabInput { width: Float!, depth: Float!, thickness: Float! }\ninput GeometryRecipeMeshInput { positions: [Float!]!, normals: [Float!]!, indices: [Int!]! }\ninput GeometryRecipeGlbInput { url: String!, extent: Float! }\ninput GeometryRecipeInput @oneOf { box: GeometryRecipeBoxInput, frame: GeometryRecipeFrameInput, slab: GeometryRecipeSlabInput, mesh: GeometryRecipeMeshInput, glb: GeometryRecipeGlbInput }\ninput ObjectKindExtraInput { id: String!, name: String!, moduleId: String!, typologyPath: [String!]!, availability: Int!, geometry: GeometryRecipeInput! }\ninput CreateStockObjectInput { kind: ObjectKindExtraInput! }\ninput DeleteStockObjectInput { objectId: String! }\n")(text)
    return lib.replace_once("  changeCuratedItemCount: ChangeCuratedItemCountInput\n}", "  changeCuratedItemCount: ChangeCuratedItemCountInput\n  createStockObject: CreateStockObjectInput\n  deleteStockObject: DeleteStockObjectInput\n}")(text)


def proto(text):
    text = lib.replace_once('import "🔢change-curated-item-count/🛰️.proto";\n', 'import "🔢change-curated-item-count/🛰️.proto";\nimport "📥️create-stock-object/🛰️.proto";\nimport "📤️delete-stock-object/🛰️.proto";\n')(text)
    return lib.replace_once("    ChangeCuratedItemCount change_curated_item_count = 3;\n", "    ChangeCuratedItemCount change_curated_item_count = 3;\n    CreateStockObject create_stock_object = 4;\n    DeleteStockObject delete_stock_object = 5;\n")(text)


def aggregate_ts(text):
    text = lib.replace_once('import type { ChangeCuratedItemCount } from "./🔢change-curated-item-count/🟦️.ts";\n', 'import type { ChangeCuratedItemCount } from "./🔢change-curated-item-count/🟦️.ts";\nimport type { CreateStockObject } from "./📥️create-stock-object/🟦️.ts";\nimport type { DeleteStockObject } from "./📤️delete-stock-object/🟦️.ts";\n')(text)
    return lib.replace_once('  | ({ mutation: "changeCuratedItemCount" } & ChangeCuratedItemCount);', '  | ({ mutation: "changeCuratedItemCount" } & ChangeCuratedItemCount)\n  | ({ mutation: "createStockObject" } & CreateStockObject)\n  | ({ mutation: "deleteStockObject" } & DeleteStockObject);')(text)


def operations(text):
    text = lib.replace_once('pub const KINDS: &[&str] = &["create-curated-item", "delete-curated-item", "change-curated-item-count"];', 'pub const KINDS: &[&str] = &["create-curated-item", "delete-curated-item", "change-curated-item-count", "create-stock-object", "delete-stock-object"];')(text)
    old = """/// `🗂️mutate-curation-1`'s exhaustive case measures itself against. Three kinds and no more: `stock` is
/// a bulk-populated reference catalogue that reaches the document through
/// `ArtifactStore::reset`, `CuratedItem` carries no name and no nested collection, and
/// whole-document replace was removed with no replacement — so `create`/`delete`/`change` over the
/// one id-keyed collection is the entire closed vocabulary this schema supports."""
    new = """/// `🗂️mutate-curation-1`'s exhaustive case measures itself against: `create`/`delete`/`change` over the id-keyed
/// curated selection and `create`/`delete` over the stock (an agent stocks a blank hub catalogue through them — stock
/// used to reach the document only through `ArtifactStore::reset`, which an agent transaction cannot carry)."""
    return lib.replace_once(old, new)(text)


IO_VARIANT_OLD = """    ChangeCuratedItemCount {
        object_id: String,
        new_count: u32,
    },
}"""
IO_VARIANT_NEW = """    ChangeCuratedItemCount {
        object_id: String,
        new_count: u32,
    },
    CreateStockObject {
        #[dsl(block)]
        kind: ObjectKindExtra,
    },
    DeleteStockObject {
        object_id: String,
    },
}"""


def io_text(text):
    text = lib.replace_once("use crate::schema::mutations::{change_curated_item_count, create_curated_item, delete_curated_item};\nuse crate::CuratedItem;\n", "use crate::schema::mutations::{change_curated_item_count, create_curated_item, create_stock_object, delete_curated_item, delete_stock_object};\nuse crate::{CuratedItem, ObjectKindExtra};\n")(text)
    text = lib.replace_once(IO_VARIANT_OLD, IO_VARIANT_NEW)(text)
    text = lib.replace_once("        SourcingMutation::ChangeCuratedItemCount(payload) => SourcingMutationDsl::ChangeCuratedItemCount { object_id: payload.object_id.clone(), new_count: payload.new_count },\n", "        SourcingMutation::ChangeCuratedItemCount(payload) => SourcingMutationDsl::ChangeCuratedItemCount { object_id: payload.object_id.clone(), new_count: payload.new_count },\n        SourcingMutation::CreateStockObject(payload) => SourcingMutationDsl::CreateStockObject { kind: payload.kind.clone() },\n        SourcingMutation::DeleteStockObject(payload) => SourcingMutationDsl::DeleteStockObject { object_id: payload.object_id.clone() },\n")(text)
    return lib.replace_once("        SourcingMutationDsl::ChangeCuratedItemCount { object_id, new_count } => SourcingMutation::ChangeCuratedItemCount(change_curated_item_count::ChangeCuratedItemCount { object_id, new_count }),\n", "        SourcingMutationDsl::ChangeCuratedItemCount { object_id, new_count } => SourcingMutation::ChangeCuratedItemCount(change_curated_item_count::ChangeCuratedItemCount { object_id, new_count }),\n        SourcingMutationDsl::CreateStockObject { kind } => SourcingMutation::CreateStockObject(create_stock_object::CreateStockObject { kind }),\n        SourcingMutationDsl::DeleteStockObject { object_id } => SourcingMutation::DeleteStockObject(delete_stock_object::DeleteStockObject { object_id }),\n")(text)


def mount(directory, leaf):
    rel = f"🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/{directory}"
    module_test = "tests_" + leaf["case"].split("️", 1)[-1].replace("-", "_")
    return f"""                                #[path = "."]
                                pub mod {leaf['module']} {{
                                    #[path = "{rel}/🦀️.rs"]
                                    mod component;
                                    #[path = "{rel}/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "{rel}/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[path = "{rel}/💾️binary/🦀️.rs"]
                                    pub mod binary;
                                    #[cfg(test)]
                                    #[path = "{rel}/🧪️tests/{leaf['case']}/🦀️.rs"]
                                    mod {module_test};
                                    #[path = "{rel}/📝️text/🦀️.rs"]
                                    pub mod text;
                                }}
"""


MOUNT_ANCHOR = """                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢change-curated-item-count/📝️text/🦀️.rs"]
                                    pub mod text;
                                }
"""


def lib_mounts(text):
    if "pub mod create_stock_object {" in text:
        return text
    assert text.count(MOUNT_ANCHOR) == 1, "change-curated-item-count mount block"
    return text.replace(MOUNT_ANCHOR, MOUNT_ANCHOR + "".join(mount(directory, leaf) for directory, leaf in LEAVES.items()))


def oracles(text):
    value = json.loads(text)
    catalog = value["mutationCatalogs"][0]
    for directory, leaf in LEAVES.items():
        if leaf["kind"] not in catalog["kinds"]:
            catalog["kinds"].append(leaf["kind"])
        if not any(vector["mutationId"] == leaf["kind"] for vector in catalog["vectors"]):
            catalog["vectors"].append({"mutationId": leaf["kind"], "sourceMutationDirectoryName": directory, "mutationDirectoryName": directory, "scenarios": [{"id": leaf["case"].split("️", 1)[-1], "directoryName": leaf["case"]}]})
        catalog["vectors"].sort(key=lambda vector: vector["mutationId"])
        manifest = value["mutationManifests"][0]["mutations"]
        if not any(entry["id"] == leaf["kind"] for entry in manifest):
            manifest.append({"id": leaf["kind"], "capability": "curation-1-mutate", "payloadSchema": "🧬️.schema.json", "outcomes": ["applied", "rejected"], "productionDispatch": {"operation": leaf["kind"], "bridgeVersion": 1, "variant": leaf["variant"]}, "oracleRequirements": [{"capability": "curation-1-mutate", "qualifyingKind": "verified-native-second-implementation"}]})
        manifest.sort(key=lambda entry: entry["id"])
    return dump(value)


def structural(text):
    if '"create-stock-object"' in text:
        return text
    blocks = ""
    for directory, leaf in LEAVES.items():
        blocks += f"""    {{
        let kind = "{leaf['kind']}";
        let variant = "{leaf['variant']}";
        let directory = "{directory}";
        let tag = {leaf['tag']};
        let outcomes = &["applied", "rejected"][..];
        let owner = mutation_root.join("{directory}");
        let source = std::fs::read_to_string(owner.join("🦀️.rs")).expect("direct Rust owner");
        let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(owner.join("🔣️.json")).expect("direct descriptor")).expect("valid descriptor");
        assert!(source.contains("MutationKind") && source.contains("SEMANTICS"));
        assert!(!source.contains(concat!("::", "mutation::")));
        assert_eq!(descriptor["semanticKind"], kind);
        assert_eq!(descriptor["aggregateVariant"], variant);
        assert_eq!(descriptor["payloadSchema"], "🧬️schema/🔣️.json");
        assert_eq!(descriptor["textOpcode"], kind);
        assert_eq!(descriptor["binaryTag"], tag);
        assert_eq!(descriptor["outcomeClasses"], serde_json::json!(outcomes));
        assert_eq!(descriptor["requiredLanguageSurfaces"], serde_json::json!(["rust", "typescript", "graphql", "protobuf", "json-schema", "text", "binary"]));
        for surface in ["🧬️schema/🔣️.json", "🟦️.ts", "🔗️.graphql", "🛰️.proto", "📝️text/🦀️.rs", "💾️binary/🦀️.rs"] {{
            assert!(owner.join(surface).is_file(), "{{kind}} owns no {{surface}}");
        }}
        assert!(vectors.iter().any(|vector| vector["mutationId"] == kind && vector["mutationDirectoryName"] == directory));
    }}
"""
    end = text.rstrip("\n")
    assert end.endswith("}\n}") or end.endswith("    }\n}"), "structural test tail"
    return end[: -len("}")] + blocks + "}\n"


# ── the second implementation (Python) + the feature ───────────────────────────────────────────────────────────────────
PY_KINDS_OLD = '''KINDS = ("create-curated-item", "delete-curated-item", "change-curated-item-count")
"""🏷️ Every kind the catalog declares."""

TAGS = {"create-curated-item": "createCuratedItem", "delete-curated-item": "deleteCuratedItem", "change-curated-item-count": "changeCuratedItemCount"}'''
PY_KINDS_NEW = '''KINDS = ("create-curated-item", "delete-curated-item", "change-curated-item-count", "create-stock-object", "delete-stock-object")
"""🏷️ Every kind the catalog declares."""

TAGS = {"create-curated-item": "createCuratedItem", "delete-curated-item": "deleteCuratedItem", "change-curated-item-count": "changeCuratedItemCount", "create-stock-object": "createStockObject", "delete-stock-object": "deleteStockObject"}

STOCK_KINDS = ("create-stock-object", "delete-stock-object")
"""📥️ The two kinds over the stock; the other three edit the curated selection only."""'''
PY_APPLY_OLD = '''    kind = kind_of(mutation)
    items = [dict(item) for item in document["curated"]]
    if kind == "create-curated-item":'''
PY_APPLY_NEW = '''    kind = kind_of(mutation)
    if kind in STOCK_KINDS:
        return apply_stock_mutation(document, kind, mutation)
    items = [dict(item) for item in document["curated"]]
    if kind == "create-curated-item":'''
PY_STOCK_HELPERS = '''

def catalogue_handle(stock):
    """🪪️ The content-addressed kit catalogue handle of a stock list — `catalog-` + the first 8 bytes of sha256 over the
    compact JSON of its kit types (`{id, name, category}` with the module as the category), computed here independently."""
    import hashlib

    types = [{"id": row["id"], "name": row["name"], "category": row["moduleId"]} for row in stock]
    child_id = "catalog-" + hashlib.sha256(json.dumps(types, separators=(",", ":"), ensure_ascii=False).encode()).hexdigest()[:16]
    return {"childId": child_id, "target": {"artifactId": child_id, "dialect": {"artifactKind": "s.stdio.semio", "standard": "v1", "subset": "kit"}}}


def apply_stock_mutation(document, kind, mutation):
    """📥️ Stocks or unstocks one object: the stock row lands at the tail or leaves, the catalogue handle follows the stock,
    and the curated selection never moves (a curated object cannot be unstocked)."""
    stock = [dict(row) for row in document["stockExtra"]]
    if kind == "create-stock-object":
        row = mutation["kind"]
        if not row["id"]:
            raise AssertionError("%s: a stock object needs an id" % kind)
        if next((at for at, existing in enumerate(stock) if existing["id"] == row["id"]), None) is not None:
            raise AssertionError("%s: %r is already stocked" % (kind, row["id"]))
        stock.append(dict(row))
    else:
        at = next((at for at, existing in enumerate(stock) if existing["id"] == mutation["objectId"]), None)
        if at is None:
            raise AssertionError("%s: %r is not stocked" % (kind, mutation["objectId"]))
        if index_of(document["curated"], mutation["objectId"]) is not None:
            raise AssertionError("%s: %r is curated and must stay stocked" % (kind, mutation["objectId"]))
        stock.pop(at)
    result = dict(document)
    result["stockExtra"] = stock
    result["catalog"] = catalogue_handle(stock)
    validate(result)
    return result
'''
PY_INVERSE_OLD = '''    kind = kind_of(mutation)
    items = document["curated"]
    if kind == "create-curated-item":
        return {"mutation": TAGS["delete-curated-item"], "objectId": mutation["item"]["objectId"]}'''
PY_INVERSE_NEW = '''    kind = kind_of(mutation)
    if kind == "create-stock-object":
        return {"mutation": TAGS["delete-stock-object"], "objectId": mutation["kind"]["id"]}
    if kind == "delete-stock-object":
        at = next((at for at, existing in enumerate(document["stockExtra"]) if existing["id"] == mutation["objectId"]), None)
        if at is None:
            raise AssertionError("inverse of %s: %r is not stocked" % (kind, mutation["objectId"]))
        return {"mutation": TAGS["create-stock-object"], "kind": dict(document["stockExtra"][at])}
    items = document["curated"]
    if kind == "create-curated-item":
        return {"mutation": TAGS["delete-curated-item"], "objectId": mutation["item"]["objectId"]}'''
PY_EFFECT_OLD = '''    was, now = entries(before), entries(after)
    if effect == "append":'''
PY_EFFECT_NEW = '''    if effect in ("stock", "unstock"):
        return stock_effect_holds(kind, effect, before, after)
    was, now = entries(before), entries(after)
    if effect == "append":'''
PY_STOCK_EFFECT = '''

def stock_effect_holds(kind, effect, before, after):
    """📥️ `stock` requires exactly one more stock row at the tail with every earlier row in place; `unstock` exactly one
    fewer with the survivors in order; both require the catalogue handle to be the content-addressed handle of the new
    stock and the curated selection to stay exactly as it was."""
    was, now = [row["id"] for row in before["stockExtra"]], [row["id"] for row in after["stockExtra"]]
    if effect == "stock" and (len(now) != len(was) + 1 or now[: len(was)] != was):
        raise AssertionError("%s: a stock must append exactly one row after the rows already present: %r -> %r" % (kind, was, now))
    if effect == "unstock" and (len(now) + 1 != len(was) or [row for row in was if row in now] != now):
        raise AssertionError("%s: an unstock must remove exactly one row and keep the survivors in order: %r -> %r" % (kind, was, now))
    if after["catalog"] != catalogue_handle(after["stockExtra"]):
        raise AssertionError("%s: the catalogue handle %r is not the content-addressed handle of the new stock" % (kind, after["catalog"]["childId"]))
    if before["curated"] != after["curated"]:
        raise AssertionError("%s: stocking must not move the curated selection" % kind)
'''
PY_RESTORES_OLD = '''    was, now = entries(original), entries(restored)
    if len(was) != len(now):'''
PY_RESTORES_NEW = '''    if kind in STOCK_KINDS:
        for name in MEMBERS:
            if restored[name] != original[name]:
                raise AssertionError("inverse-%s: %s did not come back" % (kind, name))
        return
    was, now = entries(original), entries(restored)
    if len(was) != len(now):'''


def python_oracle(text):
    text = lib.replace_once(PY_KINDS_OLD, PY_KINDS_NEW)(text)
    text = lib.replace_once(PY_APPLY_OLD, PY_APPLY_NEW)(text)
    text = lib.replace_once(PY_INVERSE_OLD, PY_INVERSE_NEW)(text)
    text = lib.replace_once(PY_EFFECT_OLD, PY_EFFECT_NEW)(text)
    text = lib.replace_once(PY_RESTORES_OLD, PY_RESTORES_NEW)(text)
    if "def apply_stock_mutation(" not in text:
        anchor = "\n\ndef inverse_mutation(document, mutation):"
        assert text.count(anchor) == 1, "inverse_mutation anchor"
        text = text.replace(anchor, PY_STOCK_HELPERS.rstrip("\n") + anchor)
    if "def stock_effect_holds(" not in text:
        anchor = "\n\ndef restores(kind, restored, original):"
        assert text.count(anchor) == 1, "restores anchor"
        text = text.replace(anchor, PY_STOCK_EFFECT.rstrip("\n") + anchor)
    return text


FEATURE_PROSE_OLD = """  is how SMALL it is and why. Three kinds over exactly one collection. `stock` is not in the
  vocabulary at all: it is a bulk-populated reference catalogue seeded from hot-installed
  `sourcing.module` contributions and replaced wholesale through `ArtifactStore::reset`, the same
  non-history path whole-document replace uses — so there is no `create-stock-item` and no
  `set-snapshot` here. Within `curated` the schema closes the vocabulary just as tightly: a"""
FEATURE_PROSE_NEW = """  is how SMALL it is and why. Three kinds over the curated selection and two over the stock. The stock
  used to reach the document only through `ArtifactStore::reset` (whole-document replacement), which an
  agent transaction cannot carry, so an agent on a blank hub catalogue could not stock it at all;
  `create-stock-object` and `delete-stock-object` now carry one stock row each, and the composed kit
  catalogue handle follows the stock by content address (`catalog-` + sha256 of the kit types' compact
  JSON, first 8 bytes), which the Python side computes itself; like `create-curated-item`, a re-stock lands at
  the tail, so the inverse table unstocks the TRAILING stock row. There is still no `set-snapshot` here.
  Within `curated` the schema closes the vocabulary just as tightly: a"""
PLATE = '{"mutation":"createStockObject","kind":{"id":"plate-steel-8","name":"Steel Plate 8mm","moduleId":"slabs","typologyPath":["slabs","steel"],"availability":40,"geometry":{"kind":"slab","width":2.0,"depth":1.0,"thickness":0.008}}}'
UNSTOCK = '{"mutation":"deleteStockObject","objectId":"beam-kvh-c24"}'
UNSTOCK_TRAILING = '{"mutation":"deleteStockObject","objectId":"slab-hollow-core-265"}'


def feature(text):
    text = lib.replace_once(FEATURE_PROSE_OLD, FEATURE_PROSE_NEW)(text)
    text = lib.replace_once(FEATURE_CARRIER_OLD, FEATURE_CARRIER_NEW)(text)
    mutate_row = "      | change-curated-item-count | retune | {\"mutation\":\"changeCuratedItemCount\",\"objectId\":\"window-tilt-turn-120x140\",\"newCount\":14}             |\n"
    assert text.count(mutate_row) == 2, f"mutate/inverse change rows x{text.count(mutate_row)}"
    if "| create-stock-object       | stock" not in text:
        first, second = text.split(mutate_row)[0], text.split(mutate_row)[1]
        text = first + mutate_row + f"      | create-stock-object       | stock   | {PLATE} |\n      | delete-stock-object       | unstock | {UNSTOCK} |\n" + second + mutate_row + f"      | create-stock-object       | stock   | {PLATE} |\n      | delete-stock-object       | unstock | {UNSTOCK_TRAILING} |\n" + text.split(mutate_row)[2]
    spec_row = "      | change-curated-item-count | retune | 🔢change-curated-item-count | 🔢️raises-the-glulam-beam-count-to-20         |\n"
    spec_rows = f"      | create-stock-object       | stock   | {CREATE_DIR} | {CREATE_CASE} |\n      | delete-stock-object       | unstock | {DELETE_DIR} | {DELETE_CASE} |\n"
    if f"| {CREATE_DIR} |" not in text:
        assert text.count(spec_row) == 1, "spec-vector change row"
        text = text.replace(spec_row, spec_row + spec_rows)
    return text


RS_KINDS_OLD = 'const KINDS: &[&str] = &["create-curated-item", "delete-curated-item", "change-curated-item-count"];'
RS_KINDS_NEW = 'const KINDS: &[&str] = &["create-curated-item", "delete-curated-item", "change-curated-item-count", "create-stock-object", "delete-stock-object"];'
RS_EFFECT_OLD = """        let entries = |snapshot: &CurationSnapshot| snapshot.curated.iter().map(|item| (item.object_id.clone(), item.count)).collect::<Vec<_>>();
        let (was, now) = (entries(before), entries(after));
        match effect {"""
RS_EFFECT_NEW = """        if matches!(effect, "stock" | "unstock") {
            let rows = |snapshot: &CurationSnapshot| snapshot.stock_extra.iter().map(|extra| extra.id.clone()).collect::<Vec<_>>();
            let (was, now) = (rows(before), rows(after));
            let moved = if effect == "stock" { now.len() == was.len() + 1 && now[..was.len()] == was[..] } else { now.len() + 1 == was.len() && was.iter().filter(|row| now.contains(row)).cloned().collect::<Vec<_>>() == now };
            if !moved {
                return Err(format!("{scenario}: the stock did not move the way {effect:?} claims: {was:?} -> {now:?}"));
            }
            if after.catalog != semio_s_artifact_sourcing_curation::catalog_child_handle(&semio_s_artifact_sourcing_curation::stock_of(after)) {
                return Err(format!("{scenario}: the catalogue handle {} is not the content-addressed handle of the new stock", after.catalog.child_id));
            }
            if before.curated != after.curated {
                return Err(format!("{scenario}: stocking must not move the curated selection"));
            }
            return Ok(());
        }
        let entries = |snapshot: &CurationSnapshot| snapshot.curated.iter().map(|item| (item.object_id.clone(), item.count)).collect::<Vec<_>>();
        let (was, now) = (entries(before), entries(after));
        match effect {"""


def rust_differential(text):
    text = lib.replace_once(RS_KINDS_OLD, RS_KINDS_NEW)(text)
    return lib.replace_once(RS_EFFECT_OLD, RS_EFFECT_NEW)(text)


# ── editor: `stockFromCatalogue` emits stock mutations; e2e agent law ─────────────────────────────────────────────────
HANDLER = f"{ANY}/✏️editor/🎮️commands/📇️stock-from-catalogue/🦀️.rs"
HANDLER_NEW = """//! 📄️ 📄️ Sourcing curation app commands command — `stock-from-catalogue`.

use crate::op::SourcingMutation;
use crate::schema::available_modules;
use crate::CurationSnapshot;
use crate::editor::sourcing::config::{SourcingCurationConfig, SourcingCurationConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, FaultCode, FaultOrigin};
use std::collections::HashSet;
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "stock-from-catalogue")]
pub struct StockFromCatalogue {
    pub module_id: Option<String>,
}

/// 🧺️ Stocks every object kind the installed sourcing modules offer (or the ones of `module_id`) that the stock does not
/// hold yet — one `create-stock-object` each, so the stocking is one undoable edit an agent transaction can carry (a blank
/// hub catalogue is stocked this way). Existing stock and curated counts stay; an unknown module is refused by name.
pub fn handle(payload: &StockFromCatalogue, doc: &ArtifactView<'_, CurationSnapshot>, cfg: &ConfigView<'_, SourcingCurationConfig>) -> Result<Emit<SourcingMutation, SourcingCurationConfigMutation>, Fault> {
    let modules = available_modules(&cfg.snapshot.contributions_json);
    if let Some(module_id) = payload.module_id.as_deref().filter(|module_id| !module_id.is_empty()) {
        if !modules.iter().any(|module| module.module_id == module_id) {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-args"), format!("no installed sourcing module '{module_id}'")));
        }
    }
    let mut stocked: HashSet<String> = doc.snapshot.stock_extra.iter().map(|extra| extra.id.clone()).collect();
    let mut mutations = Vec::new();
    for module in modules.into_iter().filter(|module| payload.module_id.as_deref().filter(|module_id| !module_id.is_empty()).is_none_or(|module_id| module.module_id == module_id)) {
        for kind in module.kinds {
            if stocked.insert(kind.id.clone()) {
                mutations.push(crate::mutations::create_stock_object::create_stock_object(crate::object_kind_extra_from_object_kind(&kind)));
            }
        }
    }
    Ok(Emit::mutations(mutations))
}
"""


def stock_handler(text):
    if "create_stock_object(" in text:
        return text
    assert "reset_document_effect(&document)" in text, "stock-from-catalogue reset body"
    return HANDLER_NEW


EDITOR = f"{ANY}/✏️editor/🦀️.rs"


def editor(text):
    text = lib.replace_once('"stockFromCatalogue" => SourcingCurationCommand::StockFromCatalogue(stock_from_catalogue::StockFromCatalogue {}),', '"stockFromCatalogue" => SourcingCurationCommand::StockFromCatalogue(stock_from_catalogue::StockFromCatalogue { module_id: args.and_then(|value| value.get("moduleId").or_else(|| value.get("module_id"))).and_then(dsl::DslValue::as_str).map(str::to_string) }),')(text)
    text = lib.replace_once('        ArtifactToolPublicationContract { tool_id: "stockFromCatalogue", lanes: &[ArtifactToolPublicationLane::HostOnly] },', '        ArtifactToolPublicationContract { tool_id: "stockFromCatalogue", lanes: &[ArtifactToolPublicationLane::Artifact] },')(text)
    text = lib.replace_once("        SourcingMutation::ChangeCuratedItemCount(payload) => payload.object_id.len(),\n    };", "        SourcingMutation::ChangeCuratedItemCount(payload) => payload.object_id.len(),\n        SourcingMutation::CreateStockObject(payload) => [payload.kind.id.len(), payload.kind.name.len(), payload.kind.module_id.len()].into_iter().chain(payload.kind.typology_path.iter().map(String::len)).sum(),\n        SourcingMutation::DeleteStockObject(payload) => payload.object_id.len(),\n    };")(text)
    return lib.replace_once("            // (ticket 26/09/18 S14). `stockFromCatalogue` remains HostOnly and cannot move `#s-checkin`.\n", "            // (ticket 26/09/18 S14). `stockFromCatalogue` journals one undoable stock edit on the Artifact lane.\n")(text)


def editor_args(text):
    old = '            .mutation("stockFromCatalogue", LocalizedLabel::native("Stock From Catalogue", "Bestand aus Katalog"))\n'
    new = old + '            .action_args("stockFromCatalogue", vec![ActionArgDef::text("moduleId", LocalizedLabel::native("Module", "Modul"))])\n'
    return lib.replace_once(old, new)(text)


E2E_LAW = '''
//#region 🤖️AgentStocksABlankCatalogue
/// 🤖️ LAW: an agent stocks and curates a blank catalogue end to end. Stock used to reach a document only through a
/// whole-document replacement an agent transaction cannot carry, so an agent facing a blank (or freshly imported) curation
/// could do nothing; now `stockFromCatalogue` publishes `create-stock-object` edits and `curationAdd` curates one of them —
/// each prepared on the agent lane and committed as an agent transaction.
#[semio_framework_async_macros::async_test]
async fn an_agent_stocks_and_curates_a_blank_catalogue() {
    use semio_framework_plugin::PluginApp;
    let mut app = new_app().await;
    app.dispatch_typed(SourcingCurationCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: EMPTY_EXAMPLE_ID.into() }), &semio_framework_plugin::artifact_app_laws::meta("local")).await.expect("blank example dispatch");
    let (pack, spr) = context::settle(&mut app).await.into_iter().find_map(|effect| match effect {
        semio_framework::kernel::Effect::LoadDocument { pack, spr } => Some((pack, spr)),
        _ => None,
    }).expect("the blank example publishes a document load");
    app.load_document_pack(&store::ArtifactPackFiles { pack, spr, ops: String::new() }).await.expect("load the blank catalogue");
    assert!(app.snapshot().expect("blank snapshot").stock_extra.is_empty(), "the catalogue starts blank");
    let definition = create_sourcing_curation_app();
    for (verb, args) in [("stockFromCatalogue", protocol::DslValue::Object(Vec::new())), ("curationAdd", protocol::DslValue::Object(vec![("objectId".into(), protocol::DslValue::String("beam-glulam-gl24h".into()))]))] {
        semio_framework_plugin::artifact_app_laws::agent_preview(&mut *app, &definition, verb, &args).await.unwrap_or_else(|fault| panic!("{verb} preview: {fault:?}"));
        let wire = app.take_last_emit_wire().await.unwrap_or_else(|| panic!("{verb} emits its wire"));
        let ops = protocol::os_spr::causal::decode_ops_vec(&wire.document).expect("document ops");
        assert!(!ops.is_empty(), "{verb} writes the document");
        let outcome = app.transaction_prepare(verb, "", &[], &ops, &wire.children, verb, None).await;
        assert!(outcome.rejection.is_none(), "{verb} prepare: {:?}", outcome.rejection);
        app.transaction_commit(verb, &semio_framework_plugin::artifact_app_laws::meta("agent")).await.unwrap_or_else(|fault| panic!("{verb} commit: {fault:?}"));
    }
    let snapshot = app.snapshot().expect("stocked snapshot");
    assert!(snapshot.stock_extra.iter().any(|extra| extra.id == "beam-glulam-gl24h"), "the agent stocked the installed modules");
    assert_eq!(snapshot.catalog, crate::catalog_child_handle(&crate::stock_of(&snapshot)), "the catalogue handle follows the stock");
    assert_eq!(snapshot.curated.iter().map(|item| (item.object_id.as_str(), item.count)).collect::<Vec<_>>(), vec![("beam-glulam-gl24h", 1)], "the agent curated one stocked object");
}
//#endregion 🤖️AgentStocksABlankCatalogue
'''
EDITOR_TESTS = f"{ANY}/✏️editor/🧪️tests/🔬️unit/🦀️.rs"


def editor_tests(text):
    if "fn an_agent_stocks_and_curates_a_blank_catalogue" in text:
        return text
    return text.rstrip("\n") + "\n" + E2E_LAW


def taxonomy(text):
    for anchor, names in (('        "🌱️create-curated-item",\n', [CREATE_DIR, DELETE_DIR]), ('        "🚫️removes-the-clt-panel-from-the-curation",\n', [CREATE_CASE, DELETE_CASE])):
        if all(f'"{name}"' in text for name in names):
            continue
        assert text.count(anchor) == 1, f"taxonomy anchor x{text.count(anchor)}: {anchor.strip()}"
        text = text.replace(anchor, anchor + "".join(f'        "{name}",\n' for name in names))
    return text


# ── existing laws that read the old reset path ────────────────────────────────────────────────────────────────────────
UNIT_TESTS = EDITOR_TESTS
APPLIED_ANCHOR = "    pub async fn dispatch(app: &mut SourcingApp, command: SourcingCurationCommand) -> InvocationResult {\n"
APPLIED = """    /// 🧮️ Folds a command's emitted document mutations over `document` through the production diff/apply — what a committed
    /// edit of them leaves (restocking publishes `create-stock-object` edits, never a document load).
    pub fn applied(document: &CurationSnapshot, mutations: &[SourcingMutation]) -> CurationSnapshot {
        mutations.iter().fold(document.clone(), |current, mutation| {
            let outcome = protocol::Mutation::diff(mutation, &current);
            assert!(outcome.messages().is_empty(), "{:?}", outcome.messages());
            protocol::MutationDiff::apply(outcome.diff(), &current).expect("emitted mutation applies")
        })
    }

"""


def unit_tests(text):
    text = editor_tests(text)
    text = lib.replace_once("        SourcingCurationCommand::StockFromCatalogue(stock_from_catalogue::StockFromCatalogue {}),\n", "        SourcingCurationCommand::StockFromCatalogue(stock_from_catalogue::StockFromCatalogue { module_id: None }),\n")(text)
    if "pub fn applied(document: &CurationSnapshot" not in text:
        assert text.count(APPLIED_ANCHOR) == 1, "context dispatch anchor"
        text = text.replace(APPLIED_ANCHOR, APPLIED + APPLIED_ANCHOR)
    return text


POOL_TESTS = f"{ANY}/✏️editor/🎭️modes/✏️edit/🪟️windows/🏊️pool/🧪️tests/🔬️unit/🦀️.rs"
POOL_HANDLE_OLD = """    let emit = stock_from_catalogue::handle(&stock_from_catalogue::StockFromCatalogue {}, &semio_framework_plugin::ArtifactView::new(&document, &history), &semio_framework_plugin::ConfigView { snapshot: &cfg, window: None }).expect("restock");
    let restocked = emit
        .effects
        .into_iter()
        .find_map(|effect| match effect {
            semio_framework::kernel::Effect::LoadDocument { pack, .. } => Some(<CurationSnapshot as store::ArtifactPack>::decode_pack(&pack).expect("restocked document")),
            _ => None,
        })
        .expect("restocking publishes a document load");
"""
POOL_HANDLE_NEW = """    let emit = stock_from_catalogue::handle(&stock_from_catalogue::StockFromCatalogue { module_id: None }, &semio_framework_plugin::ArtifactView::new(&document, &history), &semio_framework_plugin::ConfigView { snapshot: &cfg, window: None }).expect("restock");
    let restocked = crate::editor::sourcing::unit_tests::context::applied(&document, &emit.artifact_mutations);
"""
POOL_LIVE_OLD = """    app.dispatch_typed(SourcingCurationCommand::StockFromCatalogue(stock_from_catalogue::StockFromCatalogue {}), &semio_framework_plugin::artifact_app_laws::meta("local")).await.expect("restock dispatch");
    let restocked = settle(&mut app)
        .await
        .into_iter()
        .find_map(|effect| match effect {
            semio_framework::kernel::Effect::LoadDocument { pack, .. } => Some(<CurationSnapshot as store::ArtifactPack>::decode_pack(&pack).expect("restocked document")),
            _ => None,
        })
        .expect("restocking publishes a document load");
"""
POOL_LIVE_NEW = """    app.dispatch_typed(SourcingCurationCommand::StockFromCatalogue(stock_from_catalogue::StockFromCatalogue { module_id: None }), &semio_framework_plugin::artifact_app_laws::meta("local")).await.expect("restock dispatch");
    settle(&mut app).await;
    let restocked = app.snapshot().expect("restocked snapshot");
"""


def pool_tests(text):
    text = lib.replace_once(POOL_HANDLE_OLD, POOL_HANDLE_NEW)(text)
    if POOL_LIVE_NEW in text and POOL_LIVE_OLD not in text:
        pass
    else:
        assert text.count(POOL_LIVE_OLD) == 2, f"live restock blocks x{text.count(POOL_LIVE_OLD)}"
        text = text.replace(POOL_LIVE_OLD, POOL_LIVE_NEW)
    return lib.replace_once("/// ⚖️ LAW: dispatching the restock button's command through the live app publishes a document load\n/// carrying the merged catalogue — the built-in modules, since this app starts with an empty\n/// contributions lane.", "/// ⚖️ LAW: dispatching the restock button's command through the live app journals the merged catalogue as one undoable\n/// stock edit — the built-in modules, since this app starts with an empty contributions lane.")(text)


JSON_TESTS = f"{ANY}/✏️editor/🎮️commands/🗿️set-artifact-json/🧪️tests/🔬️unit/🦀️.rs"
JSON_OLD = """    let emit = stock_from_catalogue::handle(&stock_from_catalogue::StockFromCatalogue {}, &doc, &cfg).expect("handle");
    let loaded = load_document_pack(&emit);
"""
JSON_NEW = """    let emit = stock_from_catalogue::handle(&stock_from_catalogue::StockFromCatalogue { module_id: None }, &doc, &cfg).expect("handle");
    let loaded = crate::editor::sourcing::unit_tests::context::applied(&empty, &emit.artifact_mutations);
"""
JSON2_OLD = """    let emit2 = stock_from_catalogue::handle(&stock_from_catalogue::StockFromCatalogue {}, &doc2, &cfg).expect("handle");
    assert_eq!(load_document_pack(&emit2).stock_extra.len(), expected, "re-running against an already-full stock does not duplicate");
"""
JSON2_NEW = """    let emit2 = stock_from_catalogue::handle(&stock_from_catalogue::StockFromCatalogue { module_id: None }, &doc2, &cfg).expect("handle");
    assert!(emit2.artifact_mutations.is_empty(), "re-running against an already-full stock stocks nothing");
"""


def json_tests(text):
    return lib.chain(lib.replace_once(JSON_OLD, JSON_NEW), lib.replace_once(JSON2_OLD, JSON2_NEW))(text)


def editor_docs(text):
    return lib.replace_once("    /// `Effect::LoadDocument` rather than a store edit (`setDocument`/`stockFromCatalogue` share\n    /// `setActiveExample`'s `reset_document_effect` path); `Artifact` is the curated-selection\n    /// vocabulary, admitted by `SourcingCurationArtifactPreparationFactory`; `Config` is view state.", "    /// `Effect::LoadDocument` rather than a store edit (`setDocument` shares `setActiveExample`'s\n    /// `reset_document_effect` path); `Artifact` is the curation + stock vocabulary (`stockFromCatalogue` stocks\n    /// through `create-stock-object`), admitted by `SourcingCurationArtifactPreparationFactory`; `Config` is view state.")(text)


DERIVED = f"{ANY}/🧫️fixtures/🗂️mutate-curation-1/🔣️.snapshot.json"


def derived_snapshot(text):
    """🪪️ The derived curation's catalogue handle becomes the content-addressed handle of its own ten stock rows (it carried
    a stale id, `catalog-45b17760a5000259`; a stock edit re-derives the handle, so the stock-kind inverse law needs the
    committed document to be self-consistent). Only the id text changes — the file keeps its formatting."""
    value = json.loads(text)
    wanted = handle(value["stockExtra"])["childId"]
    current = value["catalog"]["childId"]
    if current == wanted:
        return text
    assert text.count(current) == 2, f"derived catalogue id x{text.count(current)}"
    return text.replace(current, wanted)


FEATURE_CARRIER_OLD = "  unchanged. The carrier does not hold a `name` or a `moduleId` for a stock entry at all, so both are\n"
FEATURE_CARRIER_NEW = "  unchanged, and the catalogue handle is the content-addressed handle of exactly those ten entries (the\n  id every stock edit re-derives). The carrier does not hold a `name` or a `moduleId` for a stock entry at all, so both are\n"


EDITS += [
    (f"{MUT}/🦀️.rs", aggregate_rs),
    (f"{MUT}/🔣️.json", aggregate_json),
    (f"{MUT}/💾️binary/🦀️.rs", binary_rs),
    (f"{MUT}/📝️text/🦀️.rs", text_rs),
    (f"{MUT}/📖️.grammar.semio", grammar),
    (f"{MUT}/🔗️.graphql", graphql),
    (f"{MUT}/🛰️.proto", proto),
    (f"{MUT}/🟦️.ts", aggregate_ts),
    (f"{ANY}/🧬️schema/⚙️operations/🦀️.rs", operations),
    (f"{ANY}/🚪️io/🧬️mutations/📝️text/🦀️.rs", io_text),
    (f"{CUR}/🦀️.rs", lib_mounts),
    (f"{ANY}/🔮️oracles/🔣️.json", oracles),
    (f"{MUT}/🧪️tests/🔬️structural-correspondence/🦀️.rs", structural),
    (f"{ANY}/🧪️tests/🗂️mutate-curation-1/🐍️.py", python_oracle),
    (f"{ANY}/🧪️tests/🗂️mutate-curation-1/🥒️.feature", feature),
    (f"{ANY}/🧪️tests/🗂️mutate-curation-1/🦀️.rs", rust_differential),
    (HANDLER, stock_handler),
    (EDITOR, lib.chain(editor, editor_args, editor_docs)),
    (UNIT_TESTS, unit_tests),
    (POOL_TESTS, pool_tests),
    (JSON_TESTS, json_tests),
    (DERIVED, derived_snapshot),
    (TAXONOMY, taxonomy),
]
lib.run("sourcing-stock", EDITS, sys.argv[1:])
