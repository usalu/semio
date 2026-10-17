//! 🔺️ DxfDiff — handcrafted sparse diff. Ticket
//! `26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION`: replaces the old
//! `DxfDiff{snapshot: Option<DxfSnapshot>}` full-replace template with real per-field patches:
//! a name-keyed triple for `header_vars`, one name-keyed triple per typed table kind
//! (`layers`/`styles`/`linetypes`), an index-keyed triple for `blocks` (each block's own nested
//! `entities` reuses the SAME entity-diff machinery as the top-level collection), and an
//! index-keyed triple for the top-level `entities` — each entity's own diff is `Replace{entity}`
//! when the entity KIND changes at that index, or a kind-specific field diff when it doesn't
//! (the plan's json/xml "Replace on kind change" rule, applied to an enum collection element).
//!
//! Two small intra-file generic cores do the position/name algebra ONCE (mirrors `stdio.obj`'s
//! `ObjIndexElem`/`generic_apply`/`generic_between`/`generic_absorb_pair` and its name-keyed
//! `HasFaces`-style sibling): `DxfIndexElem` for the two index-keyed collections
//! (`entities`/`blocks`, both reused verbatim for a block's own nested `entities`), and
//! `DxfNamedElem` for the four name-keyed collections (`header_vars`/`layers`/`styles`/
//! `linetypes`). Every PUBLIC diff type below stays a fully concrete, per-artifact named type —
//! this is pure code reuse WITHIN this one file, never exported.
//!
//! 🧪️ F6: `protocol::DiffBinary,DiffCodec,DiffText` for `DxfDiff` is **hand-rolled** (§`HandcraftedDiffCodec` below).
//! `#[derive(dsl::DslDiff)]` confirmed rejected by a real `cargo check`: `DxfEntityDiff` (this
//! file's own enum, `Replace{entity:DxfEntity}` plus one variant per typed entity kind) has no
//! `DslField` impl — `error[E0277]: the trait bound 'DxfEntityDiff: DslField' is not satisfied`
//! at `DxfEntityModified.diff: DxfEntityDiff` (recon report §3a — a data-carrying enum has no
//! `DslField` source, derivable or otherwise). Zero `Option<Option<_>>` tri-state anywhere in this
//! diff tree (3b does not apply) — this is the "enum-only" hand-roll case, same shape as
//! `stdio.json`/`stdio.svg`'s `SvgNodeDiff`. Grammar follows §5's established convention (hex for
//! strings/bytes, positional `[f1,f2,...]` tuples for structs, single-uppercase-letter tag prefix
//! for enums, `name{[removed];[modified];[added]}` for collection triples) — see
//! `f6-recon-report.md` in this ticket folder.

/// 🧩 Ordered removed keys, modified values, and inserted items.
pub(crate) type IndexedDiffParts<D, T> = (Vec<usize>, Vec<(usize, D)>, Vec<(usize, T)>);

/// 🧩 Ordered removed keys, modified values, and inserted items.
pub(crate) type NamedDiffParts<D, T> = (Vec<String>, Vec<(String, D)>, Vec<(usize, T)>);

use std::collections::{BTreeMap, BTreeSet, HashSet};

use crate::schema::snapshot::{DxfBlock, DxfEntity, DxfArc, DxfCircle, DxfInsert, DxfLine, DxfOther, DxfPolyline, DxfSolid, DxfText, DxfHeaderVar, DxfLayer, DxfLinetype, DxfOtherTable, DxfStyle, DxfTables, DxfValue, DxfVertex};
use crate::DxfSnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};

#[path = "🧮️collection-algebra-core/🦀️.rs"]
mod collection_algebra_core;
use collection_algebra_core::*;

#[path = "🏷️header-and-layer-diffs/🦀️.rs"]
mod header_and_layer_diffs;
pub use header_and_layer_diffs::*;

#[path = "🎨️style-linetype-and-tables-diffs/🦀️.rs"]
mod style_linetype_and_tables_diffs;
pub use style_linetype_and_tables_diffs::*;

#[path = "📐️entity-diff-records/🦀️.rs"]
mod entity_diff_records;
pub use entity_diff_records::*;

#[path = "🔧️entity-diff-apply/🦀️.rs"]
mod entity_diff_apply;
pub use entity_diff_apply::*;

#[path = "🧱️block-diffs/🦀️.rs"]
mod block_diffs;
pub use block_diffs::*;

#[path = "🎯️dxf-diff-root/🦀️.rs"]
mod dxf_diff_root;
pub use dxf_diff_root::*;

#[path = "🛠️diff-builders/🦀️.rs"]
mod diff_builders;
pub use diff_builders::*;

//#region 🔖️HandcraftedDiffCodec
/// 🧪️ F6: **hand-rolled** `protocol::DiffCodec` for `DxfDiff` — `#[derive(dsl::DslDiff)]`
/// rejected (see module doc for the real compiler citation). Same grammar family `GifDiff`'s and
/// `SvgDiff`'s hand-rolled codecs use (bracket-depth-aware split, hex for strings/bytes,
/// `[0]`/`[1,x]` for `Option<T>`, single-uppercase-letter tag prefix for data-carrying enums) —
/// this file re-derives its own copies of the small helper primitives since each hand-rolled
/// codec is self-contained (no shared "hand-roll helpers" module exists yet). Every helper touched
/// by `🧬️mutations`'s own hand-rolled `OpText`/`OpBinary` is `pub(crate)` for reuse (same pattern
/// svg's diff file uses for its mutations sibling).
//#region 🔖️Primitives

//#endregion 🔖️Primitives

//#region 🔖️GeometryCodecs

//#endregion 🔖️GeometryCodecs

//#region 🔖️ValueCodecs

//#endregion 🔖️ValueCodecs

//#region 🔖️EntityValueCodecs

//#endregion 🔖️EntityValueCodecs

//#region 🔖️ItemCodecs

//#endregion 🔖️ItemCodecs

//#region 🔖️DiffValueCodecs

//#endregion 🔖️DiffValueCodecs

//#region 🔖️CollectionTripleCodecs

//#endregion 🔖️CollectionTripleCodecs

//#region 🔖️BinaryPrimitives

//#endregion 🔖️BinaryPrimitives

//#region 🔖️ItemBinaryCodecs

//#endregion 🔖️ItemBinaryCodecs

//#region 🔖️DiffBinaryCodecs

//#endregion 🔖️DiffBinaryCodecs

//#region 🔖️CollectionTripleBinaryCodecs

//#endregion 🔖️CollectionTripleBinaryCodecs

//#region 🔖️TopLevel

//#endregion 🔖️TopLevel

//#region 🔖️DemoCases
/// 🧪️ P2-FG1: representative `DxfDiff` values — the empty/no-op diff, a single-collection sparse
/// diff (entities only, both a removal and an addition), and a rich multi-collection diff
/// exercising every collection kind simultaneously (name-keyed + index-keyed, `Replace`
/// (kind-change) AND non-`Replace` kind-specific patches, a nested block-level `entities`
/// sub-diff) — the single source of truth reused by `diff_codec_text_binary_roundtrip_law` below
/// AND by `⚙️engine/🦀️.rs`'s `diff_grammar_conformance_law`/`protocol_walk_law`
/// conformance tests.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_diff_cases() -> Vec<DxfDiff> {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn line_entity() -> DxfEntity {
        DxfEntity::Line(DxfLine { start: [0.0, 0.0, 0.0], end: [1.0, 2.0, 3.0], layer: "0".into(), unknown_group_codes: vec![(40, DxfValue::Double { value: 1.5 })] })
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn block_with_entity() -> DxfBlock {
        DxfBlock { name: "B1".into(), base_point: [1.0, 2.0, 3.0], entities: vec![line_entity()], unknown_group_codes: vec![(5, DxfValue::Str { value: "handle".into() })] }
    }

    let entities_only = DxfDiff {
        header_vars: None,
        tables: None,
        blocks: None,
        other_tables: None,
        entities: Some(DxfEntitiesDiff { removed: vec![2], modified: vec![], added: vec![DxfEntityAdded { index: 0, entity: DxfEntity::Circle(DxfCircle { center: [0.0, 0.0, 0.0], radius: 1.0, layer: "0".into(), unknown_group_codes: vec![] }) }] }),
    };

    let rich = DxfDiff {
        header_vars: Some(DxfHeaderVarsDiff {
            removed: vec!["$DROP".to_string()],
            modified: vec![DxfHeaderVarModified {
                name: "$MOD".to_string(),
                diff: DxfHeaderVarDiff { group_code: Some(40), value: Some(DxfValue::Double { value: 2.5 }), extra_group_codes: Some(vec![(999, DxfValue::Str { value: "note".into() })]) },
            }],
            added: vec![DxfHeaderVarAdded { index: 1, header_var: DxfHeaderVar { name: "$NEW".into(), group_code: 70, value: DxfValue::Int { value: 3 }, extra_group_codes: vec![] } }],
        }),
        tables: Some(DxfTablesDiff {
            layers: Some(DxfLayersDiff {
                removed: vec!["OLD".to_string()],
                modified: vec![DxfLayerModified { name: "0".to_string(), diff: DxfLayerDiff { color: Some(3), linetype: Some("DASHED".into()), flags: Some(1), unknown_group_codes: None } }],
                added: vec![DxfLayerAdded { index: 1, layer: DxfLayer { name: "NEW".into(), color: 5, linetype: "CONTINUOUS".into(), flags: 0, unknown_group_codes: vec![] } }],
            }),
            styles: Some(DxfStylesDiff::default()),
            linetypes: None,
        }),
        blocks: Some(DxfBlocksDiff {
            removed: vec![],
            modified: vec![DxfBlockModified {
                index: 0,
                diff: DxfBlockDiff { name: None, base_point: Some([9.0, 9.0, 9.0]), entities: Some(DxfEntitiesDiff { removed: vec![], modified: vec![], added: vec![DxfEntityAdded { index: 0, entity: line_entity() }] }), unknown_group_codes: None },
            }],
            added: vec![DxfBlockAdded { index: 1, block: block_with_entity() }],
        }),
        entities: Some(DxfEntitiesDiff {
            removed: vec![2],
            modified: vec![
                DxfEntityModified { index: 0, diff: DxfEntityDiff::Line(DxfLineDiff { start: Some([9.0, 9.0, 9.0]), end: None, layer: None, unknown_group_codes: None }) },
                DxfEntityModified { index: 1, diff: DxfEntityDiff::Replace { entity: DxfEntity::Text(DxfText { position: [0.0, 0.0, 0.0], height: 1.0, value: "swap".into(), layer: "0".into(), unknown_group_codes: vec![] }) } },
            ],
            added: vec![DxfEntityAdded { index: 3, entity: DxfEntity::Other(DxfOther { kind: "3DFACE".into(), group_codes: vec![(10, DxfValue::Double { value: 0.0 })] }) }],
        }),
        other_tables: Some(vec![DxfOtherTable { name: "VIEW".into(), tags: vec![] }]),
    };

    vec![DxfDiff::default(), entities_only, rich]
}
//#endregion 🔖️DemoCases
//#endregion 🔖️HandcraftedDiffCodec

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️handcrafted-diff-codec/🦀️.rs"]
mod handcrafted_diff_codec_tests;
//#endregion 🧪️Tests
