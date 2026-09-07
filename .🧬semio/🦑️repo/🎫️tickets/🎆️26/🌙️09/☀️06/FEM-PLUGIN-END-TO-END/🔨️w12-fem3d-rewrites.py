#!/usr/bin/env python3
"""🦀️ W12 — the one-shot rewriters that produced the hardened Rust and the repaired schemas.

Kept in the ticket rather than discarded: each region below is the exact program that wrote a
generation of files, in the order it ran. They are not idempotent in the sense of the authoring
script (`🔨️w12-author-fem3d-cases.py`) — each simply overwrites its targets with the text it
carries — so re-running one restores that generation verbatim.

Regions, in order:

1. `🔺️Diffs`        — the 22 rewritten per-kind `🔺️diff/🦀️.rs` builders
2. `🎯️OutcomeBranch` — the dead `"rejected"` assert branch, replaced in all 25 pre-existing cases
3. `🧬️Schemas`      — the four broken `🧬️.schema.json` and the snapshot `$defs`
4. `🧬️FacetSchema`   — `🌐️any/🧬️schema/🧬️mutations/🔣️.json` as the real tagged union
5. `🧪️CrateTests`    — the three in-crate tests the hardening made vacuous
6. `➕️AddLoadNoOp`   — `add-load` idempotence in the five Python references

Usage: uv run python 🔨️w12-fem3d-rewrites.py <1|2|3|4|5|6>
"""

import sys


# region 1
def region_1():
    import io, os, sys
    B = "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets"

    def write(rel, text):
        path = os.path.join(B, rel)
        with open(path, "w", encoding="utf-8") as h:
            h.write(text)
        print("wrote", path)

    FILES = {}

    # ── replace-material ────────────────────────────────────────────────────────────
    FILES["🧱️material/🧬️schema/🧬️mutations/🔁️replace-material/🔺️diff/🦀️.rs"] = '''//! 🔺️ Sparse diff builder for `ReplaceMaterial`.
    use super::ReplaceMaterial;
    use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dMaterialsDelta, Fem3dMaterialsPatchEntry};
    use crate::artifacts::fem3d::mutations::{id_mismatch, invariant, material_breach};
    use crate::artifacts::fem3d::Fem3dSnapshot;

    //#region 🔖️Diff
    pub fn diff(payload: &ReplaceMaterial, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
        let Some(existing) = base.materials.iter().find(|material| material.id == payload.id) else {
            return protocol::MutationOutcome::error("mutation.target-missing", format!("Material \\"{}\\" does not exist.", payload.id), [payload.id.clone()]);
        };
        if payload.new_material.id != payload.id {
            return id_mismatch("Material", &payload.id, &payload.new_material.id);
        }
        if existing == &payload.new_material {
            return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Material \\"{}\\" already has that value.", payload.id));
        }
        if let Some(breach) = material_breach(&payload.new_material) {
            return invariant(breach, vec![payload.id.clone()]);
        }
        protocol::MutationOutcome::new(Fem3dDiff { materials: Some(Fem3dMaterialsDelta { patched: vec![Fem3dMaterialsPatchEntry { id: payload.id.clone(), item: payload.new_material.clone() }], ..Default::default() }), ..Default::default() })
    }
    //#endregion 🔖️Diff
    '''

    # ── create-material ─────────────────────────────────────────────────────────────
    FILES["🧱️material/🧬️schema/🧬️mutations/🌱️create-material/🔺️diff/🦀️.rs"] = '''//! 🔺️ Sparse diff builder for `CreateMaterial`.
    use super::CreateMaterial;
    use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dMaterialsDelta};
    use crate::artifacts::fem3d::mutations::{invariant, material_breach};
    use crate::artifacts::fem3d::Fem3dSnapshot;

    //#region 🔖️Diff
    pub fn diff(payload: &CreateMaterial, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
        if base.materials.iter().any(|material| material.id == payload.material.id) {
            return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A material with id \\"{}\\" already exists.", payload.material.id), [payload.material.id.clone()]);
        }
        if let Some(breach) = material_breach(&payload.material) {
            return invariant(breach, vec![payload.material.id.clone()]);
        }
        protocol::MutationOutcome::new(Fem3dDiff { materials: Some(Fem3dMaterialsDelta { added: vec![payload.material.clone()], ..Default::default() }), ..Default::default() })
    }
    //#endregion 🔖️Diff
    '''

    # ── delete-material ─────────────────────────────────────────────────────────────
    FILES["🧱️material/🧬️schema/🧬️mutations/🗑️delete-material/🔺️diff/🦀️.rs"] = '''//! 🔺️ Sparse diff builder for `DeleteMaterial`.
    use super::DeleteMaterial;
    use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dMaterialsDelta};
    use crate::artifacts::fem3d::mutations::{material_referrers, target_referenced};
    use crate::artifacts::fem3d::Fem3dSnapshot;

    //#region 🔖️Diff
    pub fn diff(payload: &DeleteMaterial, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
        if !base.materials.iter().any(|material| material.id == payload.id) {
            return protocol::MutationOutcome::error("mutation.target-missing", format!("Material \\"{}\\" does not exist.", payload.id), [payload.id.clone()]);
        }
        let referrers = material_referrers(base, &payload.id);
        if !referrers.is_empty() {
            return target_referenced("Material", &payload.id, referrers);
        }
        protocol::MutationOutcome::new(Fem3dDiff { materials: Some(Fem3dMaterialsDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
    }
    //#endregion 🔖️Diff
    '''

    # ── create-section ──────────────────────────────────────────────────────────────
    FILES["🕸️mesh/🧬️schema/🧬️mutations/📐️create-section/🔺️diff/🦀️.rs"] = '''//! 🔺️ Sparse diff builder for `CreateSection`.
    use super::CreateSection;
    use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dSectionsDelta};
    use crate::artifacts::fem3d::mutations::{invariant, section_breach};
    use crate::artifacts::fem3d::Fem3dSnapshot;

    //#region 🔖️Diff
    pub fn diff(payload: &CreateSection, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
        if base.sections.iter().any(|section| section.id == payload.section.id) {
            return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A section with id \\"{}\\" already exists.", payload.section.id), [payload.section.id.clone()]);
        }
        if let Some(breach) = section_breach(&payload.section) {
            return invariant(breach, vec![payload.section.id.clone()]);
        }
        protocol::MutationOutcome::new(Fem3dDiff { sections: Some(Fem3dSectionsDelta { added: vec![payload.section.clone()], ..Default::default() }), ..Default::default() })
    }
    //#endregion 🔖️Diff
    '''

    # ── delete-section ──────────────────────────────────────────────────────────────
    FILES["🕸️mesh/🧬️schema/🧬️mutations/✂️delete-section/🔺️diff/🦀️.rs"] = '''//! 🔺️ Sparse diff builder for `DeleteSection`.
    use super::DeleteSection;
    use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dSectionsDelta};
    use crate::artifacts::fem3d::mutations::{section_referrers, target_referenced};
    use crate::artifacts::fem3d::Fem3dSnapshot;

    //#region 🔖️Diff
    pub fn diff(payload: &DeleteSection, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
        if !base.sections.iter().any(|section| section.id == payload.id) {
            return protocol::MutationOutcome::error("mutation.target-missing", format!("Section \\"{}\\" does not exist.", payload.id), [payload.id.clone()]);
        }
        let referrers = section_referrers(base, &payload.id);
        if !referrers.is_empty() {
            return target_referenced("Section", &payload.id, referrers);
        }
        protocol::MutationOutcome::new(Fem3dDiff { sections: Some(Fem3dSectionsDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
    }
    //#endregion 🔖️Diff
    '''

    # ── replace-section ─────────────────────────────────────────────────────────────
    FILES["🕸️mesh/🧬️schema/🧬️mutations/📏️replace-section/🔺️diff/🦀️.rs"] = '''//! 🔺️ Sparse diff builder for `ReplaceSection`.
    use super::ReplaceSection;
    use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dSectionsDelta, Fem3dSectionsPatchEntry};
    use crate::artifacts::fem3d::mutations::{id_mismatch, invariant, section_breach};
    use crate::artifacts::fem3d::Fem3dSnapshot;

    //#region 🔖️Diff
    pub fn diff(payload: &ReplaceSection, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
        let Some(existing) = base.sections.iter().find(|section| section.id == payload.id) else {
            return protocol::MutationOutcome::error("mutation.target-missing", format!("Section \\"{}\\" does not exist.", payload.id), [payload.id.clone()]);
        };
        if payload.new_section.id != payload.id {
            return id_mismatch("Section", &payload.id, &payload.new_section.id);
        }
        if existing == &payload.new_section {
            return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Section \\"{}\\" already has that value.", payload.id));
        }
        if let Some(breach) = section_breach(&payload.new_section) {
            return invariant(breach, vec![payload.id.clone()]);
        }
        protocol::MutationOutcome::new(Fem3dDiff { sections: Some(Fem3dSectionsDelta { patched: vec![Fem3dSectionsPatchEntry { id: payload.id.clone(), item: payload.new_section.clone() }], ..Default::default() }), ..Default::default() })
    }
    //#endregion 🔖️Diff
    '''

    # ── create-node ─────────────────────────────────────────────────────────────────
    FILES["🕸️mesh/🧬️schema/🧬️mutations/⚪️create-node/🔺️diff/🦀️.rs"] = '''//! 🔺️ Sparse diff builder for `CreateNode`.
    use super::CreateNode;
    use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dNodesDelta};
    use crate::artifacts::fem3d::mutations::{invariant, node_breach};
    use crate::artifacts::fem3d::Fem3dSnapshot;

    //#region 🔖️Diff
    pub fn diff(payload: &CreateNode, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
        if base.nodes.iter().any(|node| node.id == payload.node.id) {
            return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A node with id \\"{}\\" already exists.", payload.node.id), [payload.node.id.clone()]);
        }
        if let Some(breach) = node_breach(&payload.node) {
            return invariant(breach, vec![payload.node.id.clone()]);
        }
        protocol::MutationOutcome::new(Fem3dDiff { nodes: Some(Fem3dNodesDelta { added: vec![payload.node.clone()], ..Default::default() }), ..Default::default() })
    }
    //#endregion 🔖️Diff
    '''

    # ── delete-node (unchanged behaviour, documented exception) ─────────────────────
    FILES["🕸️mesh/🧬️schema/🧬️mutations/🕳️delete-node/🔺️diff/🦀️.rs"] = '''//! 🔺️ Sparse diff builder for `DeleteNode`.
    //!
    //! 🕳️ THE ONE VERB THAT DOES NOT REFUSE A REFERENCED TARGET. Every other `delete-` in this
    //! vocabulary raises `mutation.target-referenced` while a referrer is alive; `delete-node` keeps
    //! the permissive behaviour its own committed vector states in so many words —
    //! `🧪️tests/🚫️removes-the-column-head-056295` asserts that frame `f1` keeps naming `n3` after the
    //! node is gone ("delete-node is cascade-free"). Changing it would overturn a specified behaviour,
    //! so the asymmetry is recorded here rather than silently removed. `node_referrers` in
    //! `🌐️any/🧬️schema/🧬️mutations/🦀️.rs` is the scan a future cascade would use.
    use super::DeleteNode;
    use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dNodesDelta};
    use crate::artifacts::fem3d::Fem3dSnapshot;

    //#region 🔖️Diff
    pub fn diff(payload: &DeleteNode, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
        if !base.nodes.iter().any(|node| node.id == payload.id) {
            return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \\"{}\\" does not exist.", payload.id), [payload.id.clone()]);
        }
        protocol::MutationOutcome::new(Fem3dDiff { nodes: Some(Fem3dNodesDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
    }
    //#endregion 🔖️Diff
    '''

    # ── create-element ──────────────────────────────────────────────────────────────
    FILES["🕸️mesh/🧬️schema/🧬️mutations/🧩️create-element/🔺️diff/🦀️.rs"] = '''//! 🔺️ Sparse diff builder for `CreateElement`.
    use super::CreateElement;
    use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dElementsDelta};
    use crate::artifacts::fem3d::mutations::resolve_element;
    use crate::artifacts::fem3d::{element_id, Fem3dSnapshot};

    //#region 🔖️Diff
    pub fn diff(payload: &CreateElement, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
        let id = element_id(&payload.element);
        if base.elements.iter().any(|element| element_id(element) == id) {
            return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("An element with id \\"{id}\\" already exists."), [id.to_string()]);
        }
        if let Some(refusal) = resolve_element(base, &payload.element) {
            return refusal;
        }
        protocol::MutationOutcome::new(Fem3dDiff { elements: Some(Fem3dElementsDelta { added: vec![(*payload.element).clone()], ..Default::default() }), ..Default::default() })
    }
    //#endregion 🔖️Diff
    '''

    # ── delete-element ──────────────────────────────────────────────────────────────
    FILES["🕸️mesh/🧬️schema/🧬️mutations/🗑️delete-element/🔺️diff/🦀️.rs"] = '''//! 🔺️ Sparse diff builder for `DeleteElement`.
    use super::DeleteElement;
    use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dElementsDelta};
    use crate::artifacts::fem3d::mutations::{element_referrers, target_referenced};
    use crate::artifacts::fem3d::{element_id, Fem3dSnapshot};

    //#region 🔖️Diff
    pub fn diff(payload: &DeleteElement, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
        if !base.elements.iter().any(|element| element_id(element) == payload.id) {
            return protocol::MutationOutcome::error("mutation.target-missing", format!("Element \\"{}\\" does not exist.", payload.id), [payload.id.clone()]);
        }
        let referrers = element_referrers(base, &payload.id);
        if !referrers.is_empty() {
            return target_referenced("Element", &payload.id, referrers);
        }
        protocol::MutationOutcome::new(Fem3dDiff { elements: Some(Fem3dElementsDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
    }
    //#endregion 🔖️Diff
    '''

    # ── replace-element ─────────────────────────────────────────────────────────────
    FILES["🕸️mesh/🧬️schema/🧬️mutations/♻️replace-element/🔺️diff/🦀️.rs"] = '''//! 🔺️ Sparse diff builder for `ReplaceElement`.
    use super::ReplaceElement;
    use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dElementsDelta, Fem3dElementsPatchEntry};
    use crate::artifacts::fem3d::mutations::{id_mismatch, resolve_element};
    use crate::artifacts::fem3d::{element_id, Fem3dSnapshot};

    //#region 🔖️Diff
    pub fn diff(payload: &ReplaceElement, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
        let Some(existing) = base.elements.iter().find(|element| element_id(element) == payload.id) else {
            return protocol::MutationOutcome::error("mutation.target-missing", format!("Element \\"{}\\" does not exist.", payload.id), [payload.id.clone()]);
        };
        let replacement = element_id(&payload.new_element);
        if replacement != payload.id {
            return id_mismatch("Element", &payload.id, replacement);
        }
        if existing == payload.new_element.as_ref() {
            return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Element \\"{}\\" already has that value.", payload.id));
        }
        if let Some(refusal) = resolve_element(base, &payload.new_element) {
            return refusal;
        }
        protocol::MutationOutcome::new(Fem3dDiff { elements: Some(Fem3dElementsDelta { patched: vec![Fem3dElementsPatchEntry { id: payload.id.clone(), item: (*payload.new_element).clone() }], ..Default::default() }), ..Default::default() })
    }
    //#endregion 🔖️Diff
    '''

    # ── create-solid ────────────────────────────────────────────────────────────────
    FILES["🕸️mesh/🧬️schema/🧬️mutations/🧊️create-solid/🔺️diff/🦀️.rs"] = '''//! 🔺️ Sparse diff builder for `CreateSolid`.
    use super::CreateSolid;
    use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dSolidsDelta};
    use crate::artifacts::fem3d::mutations::{invariant, solid_breach};
    use crate::artifacts::fem3d::Fem3dSnapshot;

    //#region 🔖️Diff
    pub fn diff(payload: &CreateSolid, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
        if base.solids.iter().any(|solid| solid.id == payload.solid.id) {
            return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A solid with id \\"{}\\" already exists.", payload.solid.id), [payload.solid.id.clone()]);
        }
        if !base.materials.iter().any(|material| material.id == payload.solid.material_id) {
            return protocol::MutationOutcome::error("mutation.target-missing", format!("Material \\"{}\\" does not exist.", payload.solid.material_id), [payload.solid.material_id.clone()]);
        }
        if let Some(breach) = solid_breach(&payload.solid) {
            return invariant(breach, vec![payload.solid.id.clone()]);
        }
        protocol::MutationOutcome::new(Fem3dDiff { solids: Some(Fem3dSolidsDelta { added: vec![payload.solid.clone()], ..Default::default() }), ..Default::default() })
    }
    //#endregion 🔖️Diff
    '''

    # ── delete-solid ────────────────────────────────────────────────────────────────
    FILES["🕸️mesh/🧬️schema/🧬️mutations/🚫️delete-solid/🔺️diff/🦀️.rs"] = '''//! 🔺️ Sparse diff builder for `DeleteSolid`.
    use super::DeleteSolid;
    use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dSolidsDelta};
    use crate::artifacts::fem3d::mutations::{solid_referrers, target_referenced};
    use crate::artifacts::fem3d::Fem3dSnapshot;

    //#region 🔖️Diff
    pub fn diff(payload: &DeleteSolid, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
        if !base.solids.iter().any(|solid| solid.id == payload.id) {
            return protocol::MutationOutcome::error("mutation.target-missing", format!("Solid \\"{}\\" does not exist.", payload.id), [payload.id.clone()]);
        }
        let referrers = solid_referrers(base, &payload.id);
        if !referrers.is_empty() {
            return target_referenced("Solid", &payload.id, referrers);
        }
        protocol::MutationOutcome::new(Fem3dDiff { solids: Some(Fem3dSolidsDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
    }
    //#endregion 🔖️Diff
    '''

    # ── replace-solid ───────────────────────────────────────────────────────────────
    FILES["🕸️mesh/🧬️schema/🧬️mutations/🔄️replace-solid/🔺️diff/🦀️.rs"] = '''//! 🔺️ Sparse diff builder for `ReplaceSolid`.
    use super::ReplaceSolid;
    use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dSolidsDelta, Fem3dSolidsPatchEntry};
    use crate::artifacts::fem3d::mutations::{id_mismatch, invariant, solid_breach};
    use crate::artifacts::fem3d::Fem3dSnapshot;

    //#region 🔖️Diff
    pub fn diff(payload: &ReplaceSolid, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
        let Some(existing) = base.solids.iter().find(|solid| solid.id == payload.id) else {
            return protocol::MutationOutcome::error("mutation.target-missing", format!("Solid \\"{}\\" does not exist.", payload.id), [payload.id.clone()]);
        };
        if payload.new_solid.id != payload.id {
            return id_mismatch("Solid", &payload.id, &payload.new_solid.id);
        }
        if existing == &payload.new_solid {
            return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Solid \\"{}\\" already has that value.", payload.id));
        }
        if !base.materials.iter().any(|material| material.id == payload.new_solid.material_id) {
            return protocol::MutationOutcome::error("mutation.target-missing", format!("Material \\"{}\\" does not exist.", payload.new_solid.material_id), [payload.new_solid.material_id.clone()]);
        }
        if let Some(breach) = solid_breach(&payload.new_solid) {
            return invariant(breach, vec![payload.id.clone()]);
        }
        protocol::MutationOutcome::new(Fem3dDiff { solids: Some(Fem3dSolidsDelta { patched: vec![Fem3dSolidsPatchEntry { id: payload.id.clone(), item: payload.new_solid.clone() }], ..Default::default() }), ..Default::default() })
    }
    //#endregion 🔖️Diff
    '''

    # ── create-support ──────────────────────────────────────────────────────────────
    FILES["🛡️boundary/🧬️schema/🧬️mutations/🛡️create-support/🔺️diff/🦀️.rs"] = '''//! 🔺️ Sparse diff builder for `CreateSupport`.
    use super::CreateSupport;
    use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dSupportsDelta};
    use crate::artifacts::fem3d::Fem3dSnapshot;

    //#region 🔖️Diff
    pub fn diff(payload: &CreateSupport, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
        if base.supports.iter().any(|support| support.id == payload.support.id) {
            return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A support with id \\"{}\\" already exists.", payload.support.id), [payload.support.id.clone()]);
        }
        if !base.nodes.iter().any(|node| node.id == payload.support.node_id) {
            return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \\"{}\\" does not exist.", payload.support.node_id), [payload.support.node_id.clone()]);
        }
        protocol::MutationOutcome::new(Fem3dDiff { supports: Some(Fem3dSupportsDelta { added: vec![payload.support.clone()], ..Default::default() }), ..Default::default() })
    }
    //#endregion 🔖️Diff
    '''

    # ── delete-support (nothing in the schema points at a support) ──────────────────
    FILES["🛡️boundary/🧬️schema/🧬️mutations/🗑️delete-support/🔺️diff/🦀️.rs"] = '''//! 🔺️ Sparse diff builder for `DeleteSupport`.
    //!
    //! 🛡️ No `mutation.target-referenced` guard: a support is a LEAF of the reference graph — it points
    //! at a node and nothing in `Fem3dSnapshot` points back at it, so there is no referrer to protect.
    //! Same for `delete-combination`.
    use super::DeleteSupport;
    use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dSupportsDelta};
    use crate::artifacts::fem3d::Fem3dSnapshot;

    //#region 🔖️Diff
    pub fn diff(payload: &DeleteSupport, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
        if !base.supports.iter().any(|support| support.id == payload.id) {
            return protocol::MutationOutcome::error("mutation.target-missing", format!("Support \\"{}\\" does not exist.", payload.id), [payload.id.clone()]);
        }
        protocol::MutationOutcome::new(Fem3dDiff { supports: Some(Fem3dSupportsDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
    }
    //#endregion 🔖️Diff
    '''

    # ── replace-support ─────────────────────────────────────────────────────────────
    FILES["🛡️boundary/🧬️schema/🧬️mutations/🔁️replace-support/🔺️diff/🦀️.rs"] = '''//! 🔺️ Sparse diff builder for `ReplaceSupport`.
    use super::ReplaceSupport;
    use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dSupportsDelta, Fem3dSupportsPatchEntry};
    use crate::artifacts::fem3d::mutations::id_mismatch;
    use crate::artifacts::fem3d::Fem3dSnapshot;

    //#region 🔖️Diff
    pub fn diff(payload: &ReplaceSupport, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
        let Some(existing) = base.supports.iter().find(|support| support.id == payload.id) else {
            return protocol::MutationOutcome::error("mutation.target-missing", format!("Support \\"{}\\" does not exist.", payload.id), [payload.id.clone()]);
        };
        if payload.new_support.id != payload.id {
            return id_mismatch("Support", &payload.id, &payload.new_support.id);
        }
        if existing == &payload.new_support {
            return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Support \\"{}\\" already has that value.", payload.id));
        }
        if !base.nodes.iter().any(|node| node.id == payload.new_support.node_id) {
            return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \\"{}\\" does not exist.", payload.new_support.node_id), [payload.new_support.node_id.clone()]);
        }
        protocol::MutationOutcome::new(Fem3dDiff { supports: Some(Fem3dSupportsDelta { patched: vec![Fem3dSupportsPatchEntry { id: payload.id.clone(), item: payload.new_support.clone() }], ..Default::default() }), ..Default::default() })
    }
    //#endregion 🔖️Diff
    '''

    # ── add-load ────────────────────────────────────────────────────────────────────
    FILES["🏋️load/🧬️schema/🧬️mutations/➕️add-load/🔺️diff/🦀️.rs"] = '''//! 🔺️ Sparse diff builder for `AddLoad` — clones the target case, pushes the load, patches it.
    use super::AddLoad;
    use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dLoadCasesDelta, Fem3dLoadCasesPatchEntry};
    use crate::artifacts::fem3d::mutations::resolve_load;
    use crate::artifacts::fem3d::{load_id, Fem3dSnapshot};

    //#region 🔖️Diff
    pub fn diff(payload: &AddLoad, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
        let Some(existing) = base.load_cases.iter().find(|case| case.id == payload.case_id) else {
            return protocol::MutationOutcome::error("mutation.target-missing", format!("Load case \\"{}\\" does not exist.", payload.case_id), [payload.case_id.clone()]);
        };
        let new_load_id = load_id(&payload.load);
        if existing.loads.iter().any(|load| load_id(load) == new_load_id) {
            return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Load \\"{}\\" already exists in case \\"{}\\".", new_load_id, payload.case_id));
        }
        if let Some(refusal) = resolve_load(base, &payload.load) {
            return refusal;
        }
        let mut item = existing.clone();
        item.loads.push((*payload.load).clone());
        protocol::MutationOutcome::new(Fem3dDiff { load_cases: Some(Fem3dLoadCasesDelta { patched: vec![Fem3dLoadCasesPatchEntry { id: payload.case_id.clone(), item }], ..Default::default() }), ..Default::default() })
    }
    //#endregion 🔖️Diff
    '''

    # ── create-load-case ────────────────────────────────────────────────────────────
    FILES["🏋️load/🧬️schema/🧬️mutations/📋️create-load-case/🔺️diff/🦀️.rs"] = '''//! 🔺️ Sparse diff builder for `CreateLoadCase`.
    use super::CreateLoadCase;
    use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dLoadCasesDelta};
    use crate::artifacts::fem3d::mutations::resolve_load;
    use crate::artifacts::fem3d::Fem3dSnapshot;

    //#region 🔖️Diff
    pub fn diff(payload: &CreateLoadCase, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
        if base.load_cases.iter().any(|case| case.id == payload.load_case.id) {
            return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A load case with id \\"{}\\" already exists.", payload.load_case.id), [payload.load_case.id.clone()]);
        }
        for load in &payload.load_case.loads {
            if let Some(refusal) = resolve_load(base, load) {
                return refusal;
            }
        }
        protocol::MutationOutcome::new(Fem3dDiff { load_cases: Some(Fem3dLoadCasesDelta { added: vec![payload.load_case.clone()], ..Default::default() }), ..Default::default() })
    }
    //#endregion 🔖️Diff
    '''

    # ── delete-load-case ────────────────────────────────────────────────────────────
    FILES["🏋️load/🧬️schema/🧬️mutations/🗑️delete-load-case/🔺️diff/🦀️.rs"] = '''//! 🔺️ Sparse diff builder for `DeleteLoadCase`.
    use super::DeleteLoadCase;
    use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dLoadCasesDelta};
    use crate::artifacts::fem3d::mutations::{load_case_referrers, target_referenced};
    use crate::artifacts::fem3d::Fem3dSnapshot;

    //#region 🔖️Diff
    pub fn diff(payload: &DeleteLoadCase, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
        if !base.load_cases.iter().any(|case| case.id == payload.id) {
            return protocol::MutationOutcome::error("mutation.target-missing", format!("Load case \\"{}\\" does not exist.", payload.id), [payload.id.clone()]);
        }
        let referrers = load_case_referrers(base, &payload.id);
        if !referrers.is_empty() {
            return target_referenced("Load case", &payload.id, referrers);
        }
        protocol::MutationOutcome::new(Fem3dDiff { load_cases: Some(Fem3dLoadCasesDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
    }
    //#endregion 🔖️Diff
    '''

    # ── delete-combination (leaf, documented) ───────────────────────────────────────
    FILES["🏋️load/🧬️schema/🧬️mutations/✂️delete-combination/🔺️diff/🦀️.rs"] = '''//! 🔺️ Sparse diff builder for `DeleteCombination`.
    //!
    //! 🔗️ No `mutation.target-referenced` guard: a combination is a LEAF of the reference graph — it
    //! weights load cases and nothing in `Fem3dSnapshot` points back at it. Same for `delete-support`.
    use super::DeleteCombination;
    use crate::artifacts::fem3d::diff::{Fem3dCombinationsDelta, Fem3dDiff};
    use crate::artifacts::fem3d::Fem3dSnapshot;

    //#region 🔖️Diff
    pub fn diff(payload: &DeleteCombination, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
        if !base.combinations.iter().any(|combination| combination.id == payload.id) {
            return protocol::MutationOutcome::error("mutation.target-missing", format!("Combination \\"{}\\" does not exist.", payload.id), [payload.id.clone()]);
        }
        protocol::MutationOutcome::new(Fem3dDiff { combinations: Some(Fem3dCombinationsDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
    }
    //#endregion 🔖️Diff
    '''

    # ── update-analysis-settings ────────────────────────────────────────────────────
    FILES["📈️analysis/🧬️schema/🧬️mutations/🎛️update-analysis-settings/🔺️diff/🦀️.rs"] = '''//! 🔺️ Sparse diff builder for `UpdateAnalysisSettings`.
    use super::UpdateAnalysisSettings;
    use crate::artifacts::fem3d::diff::Fem3dDiff;
    use crate::artifacts::fem3d::mutations::{analysis_breach, invariant};
    use crate::artifacts::fem3d::Fem3dSnapshot;

    //#region 🔖️Diff
    pub fn diff(payload: &UpdateAnalysisSettings, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
        if payload.settings == base.analysis {
            return protocol::MutationOutcome::empty().warn("mutation.no-op", "Analysis settings already have that value.".to_string());
        }
        if let Some(breach) = analysis_breach(&payload.settings) {
            return invariant(breach, Vec::new());
        }
        protocol::MutationOutcome::new(Fem3dDiff { analysis: Some(payload.settings.clone()), ..Default::default() })
    }
    //#endregion 🔖️Diff
    '''

    for rel, text in FILES.items():
        write(rel, text)
    print(len(FILES), "diff builders rewritten")


# endregion 1

# region 2
def region_2():
    import os, re, sys

    B = "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets"
    OLD = re.compile(
        r'/// 🎯️ The declared outcome matches what the mutation actually produces\.\n'
        r'#\[test\]\n'
        r'fn declared_outcome_holds\(\) \{\n'
        r'    let outcome: dsl::DslValue = dsl::json::from_json_str\(OUTCOME\)\.expect\("outcome decodes"\);\n'
        r'    let status = outcome\.get\("status"\)\.and_then\(dsl::DslValue::as_str\)\.expect\("outcome carries a status"\);\n'
        r'    let mut snapshot = before\(\);\n'
        r'    let applied = apply_fem3d_mutation\(&mut snapshot, &mutation\(\)\)\.is_ok\(\);\n'
        r'    match status \{\n'
        r'        "applied" => assert!\(applied, "(?P<label>[^"]*): declared applied but the mutation was rejected"\),\n'
        r'        "rejected" => \{\n'
        r'            assert!\(!applied, "[^"]*: declared rejected but the mutation applied"\);\n'
        r'            assert_eq!\(snapshot, before\(\), "[^"]*: rejected mutation must leave the snapshot untouched"\);\n'
        r'        \}\n'
        r'        other => panic!\("[^"]*: unknown outcome status \{other:\?\}"\),\n'
        r'    \}\n'
        r'\}\n'
    )

    NEW = '''/// 🎯️ The declared outcome matches what the mutation actually produces.
    ///
    /// 🚦️ Refusal is read off the OUTCOME, never off the `Result`. `vcs::apply_mutation` is
    /// policy-agnostic: a refused mutation carries the empty diff, so it still applies cleanly and
    /// still returns `Ok` — asserting `is_err()` here would be a branch that can never fire.
    #[test]
    fn declared_outcome_holds() {{
        let outcome: dsl::DslValue = dsl::json::from_json_str(OUTCOME).expect("outcome decodes");
        let status = outcome.get("status").and_then(dsl::DslValue::as_str).expect("outcome carries a status");
        let produced = <Fem3dMutation as protocol::Mutation<Fem3dSnapshot>>::diff(&mutation(), &before());
        let refused = produced.messages().iter().any(|message| message.level >= protocol::Severity::Error);
        let mut snapshot = before();
        apply_fem3d_mutation(&mut snapshot, &mutation()).expect("the produced diff applies to its own before-snapshot");
        match status {{
            "applied" => assert!(!refused, "{label}: declared applied but the diff builder refused with {{:?}}", produced.messages()),
            "rejected" => {{
                assert!(refused, "{label}: declared rejected but the diff builder raised no Error or Fatal, only {{:?}}", produced.messages());
                assert_eq!(produced.diff(), &crate::artifacts::fem3d::diff::Fem3dDiff::default(), "{label}: a refused mutation must carry the empty diff");
                assert_eq!(snapshot, before(), "{label}: a refused mutation must leave the snapshot untouched");
            }}
            other => panic!("{label}: unknown outcome status {{other:?}}"),
        }}
    }}
    '''

    patched, skipped = 0, []
    for base, _, files in os.walk(B):
        if "🦀️.rs" not in files or "🧪️tests" not in base:
            continue
        path = os.path.join(base, "🦀️.rs")
        with open(path, encoding="utf-8") as handle:
            text = handle.read()
        if "assert!(!applied" not in text:
            continue
        match = OLD.search(text)
        if not match:
            skipped.append(path)
            continue
        text = text[: match.start()] + NEW.format(label=match.group("label")) + text[match.end() :]
        with open(path, "w", encoding="utf-8") as handle:
            handle.write(text)
        patched += 1
    print("patched", patched, "unmatched", len(skipped))
    for path in skipped:
        print("  UNMATCHED", path)


# endregion 2

# region 3
def region_3():
    import json, os, collections

    B = "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets"
    DOFS = ["Tx", "Ty", "Tz", "Rx", "Ry", "Rz"]


    def obj(title, props, extra=None):
        out = collections.OrderedDict()
        if title:
            out["title"] = title
        out["type"] = "object"
        out["additionalProperties"] = False
        fields = collections.OrderedDict(props)
        out["required"] = sorted(fields)
        out["properties"] = fields
        if extra:
            out.update(extra)
        return out


    S = lambda: {"type": "string"}
    N = lambda: {"type": "number"}


    def variant(tag, props):
        fields = collections.OrderedDict([("kind", {"const": tag})])
        fields.update(props)
        return obj(None, fields)


    BAR = variant("bar", [("id", S()), ("start", S()), ("end", S()), ("materialId", S()), ("sectionId", S())])
    FRAME = variant("frame", [("id", S()), ("start", S()), ("end", S()), ("materialId", S()), ("sectionId", S()), ("roll", N())])
    ELEMENT = {"title": "FemElement", "oneOf": [BAR, FRAME]}

    NODAL = variant("nodal", [("id", S()), ("nodeId", S()), ("dof", {"type": "string", "enum": DOFS}), ("value", N())])
    MEMBER_UDL = variant("memberUdl", [("id", S()), ("elementId", S()), ("wx", N()), ("wy", N()), ("wz", N())])
    AREA = variant("area", [("id", S()), ("solidId", S()), ("pressure", N())])
    LOAD = {"title": "FemLoad", "oneOf": [NODAL, MEMBER_UDL, AREA]}

    NODE = obj("FemNode", [("id", S()), ("x", N()), ("y", N()), ("z", N())])
    MATERIAL = obj("FemMaterial", [("id", S()), ("name", S()), ("e", N()), ("g", N()), ("nu", N()), ("rho", N())])
    SECTION = obj("FemSection", [("id", S()), ("name", S()), ("area", N()), ("iy", N()), ("iz", N()), ("j", N())])
    POINT = {"type": "array", "items": {"type": "number"}, "minItems": 2, "maxItems": 2}
    SOLID = obj(
        "FemSolid",
        [
            ("id", S()),
            ("name", S()),
            ("outline", {"type": "array", "items": POINT}),
            ("holes", {"type": "array", "items": {"type": "array", "items": POINT}}),
            ("baseZ", N()),
            ("height", N()),
            ("layers", {"type": "integer", "minimum": 0}),
            ("meshSize", N()),
            ("materialId", S()),
        ],
    )
    SUPPORT = obj("FemSupport", [("id", S()), ("nodeId", S()), ("fixed", {"type": "array", "items": {"type": "string", "enum": DOFS}})])
    LOAD_CASE = obj("FemLoadCase", [("id", S()), ("name", S()), ("loads", {"type": "array", "items": LOAD}), ("selfWeight", {"type": "boolean"})])
    COMBINATION = obj("FemCombination", [("id", S()), ("name", S()), ("terms", {"type": "object", "additionalProperties": {"type": "number"}})])
    ANALYSIS = obj("FemAnalysisSettings", [("modalCount", {"type": "integer", "minimum": 0}), ("bucklingCount", {"type": "integer", "minimum": 0}), ("deformationScale", {"type": "number"})])


    def payload(title, props):
        out = collections.OrderedDict([("$schema", "http://json-schema.org/draft-07/schema#")])
        out.update(obj(title, props))
        return out


    def dump(rel, value):
        path = os.path.join(B, rel)
        with open(path, "w", encoding="utf-8") as handle:
            json.dump(value, handle, ensure_ascii=False, indent=2)
            handle.write("\n")
        print("wrote", path)


    # ── the four broken per-kind payload schemas ────────────────────────────────────
    dump("🕸️mesh/🧬️schema/🧬️mutations/🧩️create-element/🧬️.schema.json", payload("CreateElement", [("element", ELEMENT)]))
    dump("🕸️mesh/🧬️schema/🧬️mutations/♻️replace-element/🧬️.schema.json", payload("ReplaceElement", [("id", S()), ("newElement", ELEMENT)]))
    dump("🏋️load/🧬️schema/🧬️mutations/➕️add-load/🧬️.schema.json", payload("AddLoad", [("caseId", S()), ("load", LOAD)]))
    dump("🏋️load/🧬️schema/🧬️mutations/📋️create-load-case/🧬️.schema.json", payload("CreateLoadCase", [("loadCase", LOAD_CASE)]))

    # ── the snapshot schema's $defs ─────────────────────────────────────────────────
    snapshot_path = os.path.join(B, "🌐️any/🧬️schema/📸️snapshot/🔣️.json")
    snapshot = json.load(open(snapshot_path, encoding="utf-8"), object_pairs_hook=collections.OrderedDict)
    defs = snapshot["$defs"]
    for name, value in (("FemNode", NODE), ("FemElement", ELEMENT), ("FemSolid", SOLID), ("FemMaterial", MATERIAL), ("FemSection", SECTION), ("FemSupport", SUPPORT), ("FemLoadCase", LOAD_CASE), ("FemCombination", COMBINATION)):
        defs[name] = value
    with open(snapshot_path, "w", encoding="utf-8") as handle:
        json.dump(snapshot, handle, ensure_ascii=False, indent=2)
        handle.write("\n")
    print("wrote", snapshot_path)


# endregion 3

# region 4
def region_4():
    import json, os, collections

    B = "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets"
    TAGS = {
        "create-node": "createNode", "delete-node": "deleteNode", "create-element": "createElement", "delete-element": "deleteElement",
        "replace-element": "replaceElement", "create-material": "createMaterial", "delete-material": "deleteMaterial",
        "replace-material": "replaceMaterial", "create-section": "createSection", "delete-section": "deleteSection",
        "replace-section": "replaceSection", "create-support": "createSupport", "delete-support": "deleteSupport",
        "replace-support": "replaceSupport", "create-solid": "createSolid", "delete-solid": "deleteSolid", "replace-solid": "replaceSolid",
        "create-load-case": "createLoadCase", "delete-load-case": "deleteLoadCase", "add-load": "addLoad", "remove-load": "removeLoad",
        "change-load-case-self-weight": "changeLoadCaseSelfWeight", "create-combination": "createCombination",
        "delete-combination": "deleteCombination", "update-analysis-settings": "updateAnalysisSettings",
    }
    ORDER = list(TAGS)

    directories = {}
    for sub in sorted(os.listdir(B)):
        root = os.path.join(B, sub, "🧬️schema", "🧬️mutations")
        if not os.path.isdir(root):
            continue
        for entry in sorted(os.listdir(root)):
            path = os.path.join(root, entry, "🧬️.schema.json")
            if os.path.exists(path):
                payload = json.load(open(path, encoding="utf-8"), object_pairs_hook=collections.OrderedDict)
                stem = entry.lstrip("".join(character for character in entry if not (character.isalnum() or character in "-_")))
                directories[stem] = payload

    missing = [kind for kind in ORDER if kind not in directories]
    assert not missing, missing

    variants = []
    for kind in ORDER:
        payload = directories[kind]
        fields = collections.OrderedDict([("mutation", {"const": TAGS[kind]})])
        fields.update(payload["properties"])
        variants.append(collections.OrderedDict([
            ("title", payload["title"]),
            ("type", "object"),
            ("additionalProperties", False),
            ("required", sorted(["mutation"] + payload["required"])),
            ("properties", fields),
        ]))

    schema = collections.OrderedDict([
        ("$schema", "https://json-schema.org/draft/2020-12/schema"),
        ("$id", "https://semio.tech/schema/s/fem/fem3d/mutation.json"),
        ("title", "Fem3dMutation"),
        ("description", "🧬️ The closed fem3d mutation vocabulary as it travels the wire: an internally tagged union discriminated by `mutation`, one variant per `🧬️mutations/<kind>/🧬️.schema.json` payload. Kept in the declaration order of the `Fem3dMutation` enum."),
        ("oneOf", variants),
    ])
    path = os.path.join(B, "🌐️any/🧬️schema/🧬️mutations/🔣️.json")
    with open(path, "w", encoding="utf-8") as handle:
        json.dump(schema, handle, ensure_ascii=False, indent=2)
        handle.write("\n")
    print("wrote", path, "with", len(variants), "variants")


# endregion 4

# region 5
def region_5():
    P = "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🧬️mutations/🦀️.rs"
    t = open(P, encoding="utf-8").read()

    old_material = '''        let after_replace = round_trip(&base, &Fem3dMutation::ReplaceMaterial(replace_material::mutation::ReplaceMaterial { id: "steel".into(), new_material: replaced }));
            round_trip(&after_replace, &Fem3dMutation::DeleteMaterial(delete_material::mutation::DeleteMaterial { id: "steel".into() }));'''
    new_material = '''        let after_replace = round_trip(&base, &Fem3dMutation::ReplaceMaterial(replace_material::mutation::ReplaceMaterial { id: "steel".into(), new_material: replaced }));
            let spare = FemMaterial { id: "alu".into(), name: "Aluminium EN AW-6082".into(), e: 70e9, g: 26e9, nu: 0.33, rho: 2700.0 };
            let after_create = round_trip(&after_replace, &Fem3dMutation::CreateMaterial(create_material::mutation::CreateMaterial { material: spare }));
            round_trip(&after_create, &Fem3dMutation::DeleteMaterial(delete_material::mutation::DeleteMaterial { id: "alu".into() }));'''
    assert old_material in t
    t = t.replace(old_material, new_material, 1)

    old_section = '''        let after_replace = round_trip(&base, &Fem3dMutation::ReplaceSection(replace_section::mutation::ReplaceSection { id: "hea200".into(), new_section: replaced }));
            round_trip(&after_replace, &Fem3dMutation::DeleteSection(delete_section::mutation::DeleteSection { id: "hea200".into() }));'''
    new_section = '''        let after_replace = round_trip(&base, &Fem3dMutation::ReplaceSection(replace_section::mutation::ReplaceSection { id: "hea200".into(), new_section: replaced }));
            let spare = FemSection { id: "shs120".into(), name: "SHS 120x120x6".into(), area: 0.00266, iy: 5.56e-6, iz: 5.56e-6, j: 8.9e-6 };
            let after_create = round_trip(&after_replace, &Fem3dMutation::CreateSection(create_section::mutation::CreateSection { section: spare }));
            round_trip(&after_create, &Fem3dMutation::DeleteSection(delete_section::mutation::DeleteSection { id: "shs120".into() }));'''
    assert old_section in t
    t = t.replace(old_section, new_section, 1)

    old_law = '''        let (base, ..) = cantilever_fixture();
            let mutation = Fem3dMutation::AddLoad(add_load::mutation::AddLoad { case_id: "point".into(), load: Box::new(FemLoad::Area { id: "l9".into(), solid_id: "sol1".into(), pressure: 400.0 }) });'''
    new_law = '''        let base = solid_slab_doc();
            let mutation = Fem3dMutation::AddLoad(add_load::mutation::AddLoad { case_id: "self".into(), load: Box::new(FemLoad::Area { id: "l9".into(), solid_id: "sol1".into(), pressure: 400.0 }) });'''
    assert old_law in t
    t = t.replace(old_law, new_law, 1)

    marker = '''    fn round_trip(snapshot: &Fem3dSnapshot, operation: &Fem3dMutation) -> Fem3dSnapshot {'''
    doc = '''    /// 🔁️ Forward, then every step of the mutation's own inverse, back to the pre-mutation document.
        ///
        /// 🔗️ Every `delete-` below strikes a record NOTHING points at — an unreferenced spare it created
        /// itself, or a leaf of the reference graph. Striking a live one is refused with
        /// `mutation.target-referenced` and its own committed vector pins that branch; a round-trip over
        /// a refusal would pass vacuously and prove nothing.
    '''
    assert marker in t
    t = t.replace(marker, doc + marker, 1)
    open(P, "w", encoding="utf-8").write(t)
    print("patched")


# endregion 5

# region 6
def region_6():
    import os
    B = "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧪️tests"
    OLD = '''        if find(case["loads"], load["id"]) is not None:
                raise AssertionError("%s: case %r already carries a load %r" % (kind, case["id"], load["id"]))
            resolve_load(result, load, kind)
            case["loads"].append(load)
    '''
    NEW = '''        if find(case["loads"], load["id"]) is None:
                resolve_load(result, load, kind)
                case["loads"].append(load)
    '''
    DOC_OLD = '''def apply_mutation(document, mutation):
        """🧬️ Applies one typed mutation, returning the resulting model."""
    '''
    DOC_NEW = '''def apply_mutation(document, mutation):
        """🧬️ Applies one typed mutation, returning the resulting model — or raising when the request is
        one the vocabulary refuses.

        Two verbs are IDEMPOTENT rather than refusing, and their committed vectors say so by declaring a
        no-op warning under an `applied` status: `add-load` with a load id the case already carries, and
        every `replace-`/`change-`/`update-` whose new value is the value already there. Both leave the
        model exactly as it was, which is what a caller asked for either way.
        """
    '''
    count = 0
    for case in sorted(os.listdir(B)):
        path = os.path.join(B, case, "🐍️.py")
        if not os.path.exists(path):
            continue
        text = open(path, encoding="utf-8").read()
        if OLD not in text:
            continue
        text = text.replace(OLD, NEW, 1).replace(DOC_OLD, DOC_NEW, 1)
        open(path, "w", encoding="utf-8").write(text)
        count += 1
    print("references patched:", count)


# endregion 6


if __name__ == "__main__":
    {"1": region_1, "2": region_2, "3": region_3, "4": region_4, "5": region_5, "6": region_6}[sys.argv[1]]()
