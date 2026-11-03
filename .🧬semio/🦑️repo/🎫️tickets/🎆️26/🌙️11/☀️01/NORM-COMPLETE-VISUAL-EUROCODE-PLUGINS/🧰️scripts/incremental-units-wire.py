#!/usr/bin/env python3
"""Wires one artifact to its incremental units: editor module + tests, family evaluate, inferences live source. Idempotent. Usage: incremental-units-wire.py <code> <units-path> <documents-expr> [<units-crate-use>]"""
import glob, sys, os
code, units_path, docs = sys.argv[1:4]
root = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts"
art = [d for d in glob.glob(f"{root}/*en{code}") if os.path.isdir(d)][0]
S = f"{art}/🏅️standards/🔖️1/🪆️subsets/✳️any"
C = f"En{code}"
mod = f"semio_s_artifact_norm_en{code}"
live_path = units_path
if "::" not in units_path:
    units_path = f"{mod}_inferences::{units_path}"
inf = f"{S}/🧬️schema/💡️inferences/🦀️.rs"
s = open(inf).read()
fold = f"semio_s_plugin_norm_compliance::live_fold_source!({C}Fold, {C}Snapshot, evaluate);\n"
if fold in s:
    s = s.replace(fold, "")
    s = s.replace(f"live_report!(pub mod live({C}Snapshot) = {C}Fold);", f"live_report!(pub mod live({C}Snapshot) = {live_path});")
    open(inf, "w").write(s)
ed = f"{S}/✏️editor/🦀️.rs"
s = open(ed).read()
old = f"{mod}_inferences::evaluate(document)"
if old in s:
    s = s.replace(old, f"<{C}Family as semio_s_plugin_norm_compliance::IncrementalNormFamily>::evaluate_incrementally(document)", 1)
if 'mod incremental;' not in s:
    mount = "#[path = \"🧮️incremental/🦀️.rs\"]\npub mod incremental;\n"
    anchor = "pub mod field_meta;\n" if "pub mod field_meta;\n" in s else "//#region 🪢️Mounts\n"
    s = s.replace(anchor, anchor + mount, 1)
    assert 'mod incremental;' in s
open(ed, "w").write(s)
d = f"{S}/✏️editor/🧮️incremental"
os.makedirs(f"{d}/🧪️tests/🔬️unit", exist_ok=True)
open(f"{d}/🦀️.rs", "w").write(f'''//! 🧮️ {C} editor evaluation: the family delegates its check units to the shared `{units_path.split("::")[-1]}`; the retained `SharedEvaluation` lives in `inferences::live`, read by editor and viewer alike, so a paint after an edit re-runs only the units whose read set changed.

use crate::{C}Family;
use {units_path.rsplit("::",1)[0]}::{units_path.split("::")[-1]};
use {mod}_model::{C}Snapshot;
use semio_s_plugin_norm_compliance::incremental_evaluation::{{CheckUnit, EntityEdge, UnitSource}};

impl UnitSource for {C}Family {{
    type Document = {C}Snapshot;

    fn units(document: &{C}Snapshot) -> Vec<CheckUnit<{C}Snapshot>> {{
        {units_path.split("::")[-1]}::units(document)
    }}

    fn references(document: &{C}Snapshot) -> Vec<EntityEdge> {{
        {units_path.split("::")[-1]}::references(document)
    }}

    fn summarise(document: &{C}Snapshot, report: &semio_s_plugin_norm_compliance::CheckReport) -> Vec<semio_s_plugin_norm_compliance::CheckResult> {{
        {units_path.split("::")[-1]}::summarise(document, report)
    }}
}}

impl semio_s_plugin_norm_compliance::IncrementalNormFamily for {C}Family {{}}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
''')
open(f"{d}/🧪️tests/🔬️unit/🦀️.rs", "w").write(f'''use crate::{C}Family;
use {mod}_inferences::evaluate;
use {mod}_model::{C}Snapshot;

fn documents() -> Vec<{C}Snapshot> {{
    {docs}
}}

semio_s_plugin_norm_compliance::incremental_laws!(source = {C}Family, full = evaluate, live = {mod}_inferences::live::report, documents = documents());
''')
print("wired", C)
