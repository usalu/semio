#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Scratch-only scaffold generator for the en1995 mutations facet (ticket
26/08/12/SEMANTIC-MUTATIONS-OVERHAUL). Not committed as a permanent script; every field
name/type/verb mapping below was derived by hand from En1995Snapshot's real shape first — this
script only mechanizes the repetitive triad-file transcription, mirroring en1992/en1993's own
documented approach (see their wave2 reports)."""
import os

ROOT = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"

# (snake_field, rust_type, sample_a, sample_b_for_absorb_test)
FIELDS = [
    ("m_ed_knm", "f64", "999.0"),
    ("n_ed_kn", "f64", "111.0"),
    ("v_ed_kn", "f64", "77.0"),
    ("w_mm3", "f64", "2_000_000.0"),
    ("a_mm2", "f64", "30_000.0"),
    ("b_mm", "f64", "250.0"),
    ("h_mm", "f64", "400.0"),
    ("f_m_k", "f64", "28.0"),
    ("f_c_0_k", "f64", "24.0"),
    ("service_class", "String", "\"sc2\".to_string()"),
    ("load_duration", "String", "\"short\".to_string()"),
    ("m_crit_knm", "f64", "95.0"),
    ("f_ed_kn", "f64", "22.0"),
    ("a_ef_mm2", "f64", "14_000.0"),
    ("f_v_k", "f64", "4.5"),
    ("fire_duration_min", "f64", "60.0"),
    ("section_depth_mm", "f64", "350.0"),
    ("a_vert_m_s2", "f64", "0.5"),
    ("n_cycles_bridge", "f64", "750_000.0"),
]

def pascal(snake: str) -> str:
    return "".join((seg[0].upper() + seg[1:]) if seg else "" for seg in snake.split("_"))

def kebab(snake: str) -> str:
    return snake.replace("_", "-")

def words(snake: str) -> str:
    return snake.replace("_", " ")

def rust_default_repr(rust_type: str, sample) -> str:
    return sample

def write(path, content):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as f:
        f.write(content)

# region ChangeAnnex — repurpose 📄set-snapshot in place (glue.rs path-includes this exact dir)
set_snapshot_dir = f"{ROOT}/📄set-snapshot"

write(f"{set_snapshot_dir}/🦠️mutation/🦀️component.rs", '''//! 🔧 `change-annex` payload — changes the En1995 document's `annex` (national annex).
//! Repurposes the pre-migration `📄set-snapshot/` triad directory in place: `📦️glue.rs`
//! path-includes this exact directory outside this facet's writable boundary, so the directory
//! name stays `📄set-snapshot` while its content becomes `ChangeAnnex` — see this ticket's wave2
//! report `sharedFileRequests` for the rename once a later pass can touch `📦️glue.rs` (mirrors the
//! en1990/en1992 precedent).

use crate::artifacts::en1995::diff::En1995Diff;
use crate::artifacts::en1995::mutations::En1995Mutation;
use crate::artifacts::en1995::En1995Snapshot;
use crate::document::AnnexChoice;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeAnnex
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeAnnex {
    pub new_annex: AnnexChoice,
}

impl protocol::MutationKind<En1995Snapshot, En1995Mutation> for ChangeAnnex {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "annex", kind: "change-annex", record: "ChangedAnnex" };

    fn diff(&self, base: &En1995Snapshot) -> En1995Diff {
        crate::artifacts::en1995::mutations::set_snapshot::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1995Snapshot) -> Vec<En1995Mutation> {
        crate::artifacts::en1995::mutations::set_snapshot::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change national annex to {}", self.new_annex.label())
    }
}
//#endregion 🔖️ChangeAnnex
''')

write(f"{set_snapshot_dir}/🔺️diff/🦀️component.rs", '''//! 🔺️ `change-annex` sparse diff construction — writes only `En1995Diff.annex` from the payload.

use crate::artifacts::en1995::diff::En1995Diff;
use crate::artifacts::en1995::mutations::set_snapshot::mutation::ChangeAnnex;
use crate::artifacts::en1995::En1995Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeAnnex, _base: &En1995Snapshot) -> En1995Diff {
    En1995Diff { annex: Some(payload.new_annex.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
''')

write(f"{set_snapshot_dir}/↩️inverse/🦀️component.rs", '''//! ↩️ `change-annex` inverse — restores the pre-change `annex` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1995::mutations::set_snapshot::mutation::ChangeAnnex;
use crate::artifacts::en1995::mutations::En1995Mutation;
use crate::artifacts::en1995::En1995Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeAnnex, base: &En1995Snapshot) -> Vec<En1995Mutation> {
    vec![En1995Mutation::ChangeAnnex(ChangeAnnex { new_annex: base.annex.clone() })]
}
//#endregion 🔖️Inverse
''')
# endregion

# region 19 change-<field> triads
for snake, rust_type, sample in FIELDS:
    p = pascal(snake)
    kb = kebab(snake)
    dirname = f"🔧change-{kb}"
    mod = f"change_{snake}"
    variant = f"Change{p}"
    field = f"new_{snake}"
    d = f"{ROOT}/{dirname}"

    write(f"{d}/🦠️mutation/🦀️component.rs", f'''//! 🔧 `change-{kb}` payload — changes the En1995 document's `{snake}` (EN 1995 input).

use crate::artifacts::en1995::diff::En1995Diff;
use crate::artifacts::en1995::mutations::En1995Mutation;
use crate::artifacts::en1995::En1995Snapshot;
use serde::{{Deserialize, Serialize}};

//#region 🔖️{variant}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct {variant} {{
    pub {field}: {rust_type},
}}

impl protocol::MutationKind<En1995Snapshot, En1995Mutation> for {variant} {{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {{ verb: "change", entity: "{kb}", kind: "change-{kb}", record: "Changed{p}" }};

    fn diff(&self, base: &En1995Snapshot) -> En1995Diff {{
        crate::artifacts::en1995::mutations::{mod}::diff::diff(self, base)
    }}

    fn inverse(&self, base: &En1995Snapshot) -> Vec<En1995Mutation> {{
        crate::artifacts::en1995::mutations::{mod}::inverse::inverse(self, base)
    }}

    fn label(&self) -> String {{
        format!("Change {words(snake)} to {{:?}}", self.{field})
    }}
}}
//#endregion 🔖️{variant}
''')

    write(f"{d}/🔺️diff/🦀️component.rs", f'''//! 🔺️ `change-{kb}` sparse diff construction — writes only `En1995Diff.{snake}` from the payload.

use crate::artifacts::en1995::diff::En1995Diff;
use crate::artifacts::en1995::mutations::{mod}::mutation::{variant};
use crate::artifacts::en1995::En1995Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &{variant}, _base: &En1995Snapshot) -> En1995Diff {{
    En1995Diff {{ {snake}: Some(payload.{field}.clone()), ..Default::default() }}
}}
//#endregion 🔖️Diff
''')

    write(f"{d}/↩️inverse/🦀️component.rs", f'''//! ↩️ `change-{kb}` inverse — restores the pre-change `{snake}` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1995::mutations::{mod}::mutation::{variant};
use crate::artifacts::en1995::mutations::En1995Mutation;
use crate::artifacts::en1995::En1995Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &{variant}, base: &En1995Snapshot) -> Vec<En1995Mutation> {{
    vec![En1995Mutation::{variant}({variant} {{ {field}: base.{snake}.clone() }})]
}}
//#endregion 🔖️Inverse
''')
# endregion

print("done: wrote 20 triads (1 repurposed + 19 new)")
