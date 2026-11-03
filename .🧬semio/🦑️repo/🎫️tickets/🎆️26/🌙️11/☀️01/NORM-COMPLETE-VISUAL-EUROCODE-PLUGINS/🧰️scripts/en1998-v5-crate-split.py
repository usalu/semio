"""EN 1998 v5 crate budget: moves the structure views of the scene into `semio-s-artifact-norm-en1998-scene-structures`, the bridge, tank and tower checks of the inference into `semio-s-artifact-norm-en1998-structure-checks` and the edit rules of the mutations into `semio-s-artifact-norm-en1998-edit-rules`, and registers the three crates (workspace members and dependencies, Cargo.lock, package router, build owners). Idempotent: a crate whose directory exists is skipped.
Usage: /home/deck/semio/.venv/bin/python -I en1998-v5-crate-split.py [--dry]"""
import os as _o, sys as _s; _o.environ.get("TH_FORCE") or (_s.stderr.write("unsafe against the current tree (destructive or not idempotent), refusing to run (TH_FORCE=1 forces)\n"), _s.exit(2))  # unsafe-guard
import os, re, shutil, sys

DRY = "--dry" in sys.argv
SEMIO = "/home/deck/semio"
ART = SEMIO + "/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998"
ANY = ART + "/🏅️standards/🔖️1/🪆️subsets/✳️any"


def rd(p):
    return open(p, encoding="utf-8").read()


def wr(p, s):
    assert len(p[len(SEMIO) + 1:].encode()) < 256, "path of 256 bytes or more: " + p
    if DRY:
        return
    os.makedirs(os.path.dirname(p), exist_ok=True)
    open(p, "w", encoding="utf-8").write(s)


def manifest(name, description, depth, deps):
    ups = "/".join([".."] * depth)
    lines = "".join(f"{d} = {{ workspace = true }}\n" for d in sorted(deps))
    return f'[package]\nworkspace = "{ups}"\nname = "{name}"\nversion.workspace = true\nedition.workspace = true\nrust-version.workspace = true\ndescription = "{description}"\n\n[package.metadata.semio]\nrole = "library"\n\n[lints]\nworkspace = true\n\n[lib]\npath = "../../🦀️.rs"\n\n[dependencies]\n{lines}'


def register(name, directory, deps, dependents):
    rel = os.path.relpath(directory + "/📦️packages/🦀️rust", ART)
    cargo = rd(ART + "/Cargo.toml")
    if rel not in cargo:
        cargo = cargo.replace('    "📦️packages/🦀️rust",\n]', f'    "{rel}",\n    "📦️packages/🦀️rust",\n]', 1)
        cargo = cargo.replace("[workspace.lints.rust]", f'[workspace.dependencies.{name}]\npath = "{rel}"\n\n[workspace.lints.rust]', 1)
        wr(ART + "/Cargo.toml", cargo)
    lock = rd(ART + "/Cargo.lock")
    if f'name = "{name}"' not in lock:
        block = f'[[package]]\nname = "{name}"\nversion = "0.1.0"\ndependencies = [\n' + "".join(f' "{d}",\n' for d in sorted(deps)) + "]\n\n"
        position = next((m.start() for m in re.finditer(r'\[\[package\]\]\nname = "([^"]+)"', lock) if m.group(1) > name), len(lock))
        lock = lock[:position] + block + lock[position:]
    for dependent in dependents:
        m = re.search(rf'(\[\[package\]\]\nname = "{re.escape(dependent)}"\nversion = "[^"]+"\ndependencies = \[\n)(.*?)(\])', lock, re.S)
        lines = sorted(set(m.group(2).splitlines()) | {f' "{name}",'})
        lock = lock[:m.start(2)] + "\n".join(lines) + "\n" + lock[m.end(2):]
    wr(ART + "/Cargo.lock", lock)
    script = ART + "/📦️packages/🦀️rust/📜️script.ts"
    text = rd(script)
    if f'"{name}"' not in text:
        wr(script, text.replace('"semio-s-artifact-norm-en1998-mutations"', f'"{name}","semio-s-artifact-norm-en1998-mutations"', 1))
    owners = SEMIO + "/📋️build-owners.json"
    text = rd(owners)
    if f'"name": "{name}"' not in text:
        m = re.search(r'    \{\n      "name": "semio-s-artifact-norm-en1998-scene",\n.*?\n    \},\n', text, re.S)
        root = "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/" + rel
        block = f'    {{\n      "name": "{name}",\n      "root": "{root}",\n      "manifest": "{root}/Cargo.toml",\n      "packages": []\n    }},\n'
        wr(owners, text.replace(m.group(0), m.group(0) + block, 1))


def add_dependency(manifest_path, name):
    text = rd(manifest_path)
    if name not in text:
        wr(manifest_path, text.replace("\n[dev-dependencies]", f"{name} = {{ workspace = true }}\n\n[dev-dependencies]", 1).replace("]\n\n\n[dev", "]\n\n[dev"))


def scene_structures():
    directory = ANY + "/🎬️scene-structures"
    source = ANY + "/🎬️scene/🏗️structure-views/🦀️.rs"
    if os.path.exists(directory):
        return
    text = rd(source)
    header = '//! ' + text.split("\n", 1)[0][4:] + "\n\nextern crate semio_s_artifact_norm_en1998_model as model;\nextern crate semio_s_artifact_norm_en1998_schema as schema;\nextern crate semio_s_plugin_norm_compliance as norm;\nextern crate semio_s_plugin_norm_structural_scene as scene;\n\n"
    body = text.split("\n", 2)[2]
    body = body.replace("use super::model::", "use model::").replace("use super::response::Site;\n", "").replace("use super::scene::projection_kit::diagram_parts::", "use scene::projection_kit::diagram_parts::").replace("use super::scene::projection_kit::loc;", "use scene::projection_kit::loc;").replace("use super::scene::*;", "use scene::*;").replace("use super::norm::CheckReport;", "use norm::CheckReport;").replace("use super::schema::", "use schema::")
    site = '/// 🌍️ Site spectrum constants of one document: design ground acceleration inputs and corner periods.\n#[derive(Clone, Copy, Debug, PartialEq)]\npub struct Site {\n    pub a_gr: f64,\n    pub s: f64,\n    pub tb: f64,\n    pub tc: f64,\n    pub td: f64,\n    pub gamma_i: f64,\n}\n\n'
    body = body.replace("const TONES", site + "const TONES", 1)
    body = re.sub(r"pub\(crate\) ", "pub ", body)
    wr(directory + "/🦀️.rs", header + body)
    wr(directory + "/📦️packages/🦀️rust/Cargo.toml", manifest("semio-s-artifact-norm-en1998-scene-structures", "EN 1998 structure views: bridge, tank and tower response and diagrams of the scene", 7, ["semio-s-artifact-norm-en1998-model", "semio-s-artifact-norm-en1998-schema", "semio-s-plugin-norm-compliance", "semio-s-plugin-norm-structural-scene"]))
    if not DRY:
        os.remove(source)
        os.rmdir(os.path.dirname(source))
    register("semio-s-artifact-norm-en1998-scene-structures", directory, ["semio-s-artifact-norm-en1998-model", "semio-s-artifact-norm-en1998-schema", "semio-s-plugin-norm-compliance", "semio-s-plugin-norm-structural-scene"], ["semio-s-artifact-norm-en1998-scene"])
    add_dependency(ANY + "/🎬️scene/📦️packages/🦀️rust/Cargo.toml", "semio-s-artifact-norm-en1998-scene-structures")
    root = ANY + "/🎬️scene/🦀️.rs"
    r = rd(root)
    r = r.replace('#[path = "🏗️structure-views/🦀️.rs"]\npub(crate) mod structure_views;\n', "extern crate semio_s_artifact_norm_en1998_scene_structures as structure_views;\n")
    wr(root, r)
    response = ANY + "/🎬️scene/🧮️response/🦀️.rs"
    t = rd(response)
    a = t.index("/// 🌍️ Site spectrum constants")
    b = t.index("/// 🧭️ Seismic forces of one horizontal resisting system.")
    t = t[:a] + "pub use super::structure_views::Site;\n\n" + t[b:]
    wr(response, t)


def structure_checks():
    directory = ANY + "/🧬️schema/🌉️structure-checks"
    if os.path.exists(directory):
        return
    inf = ANY + "/🧬️schema/💡️inferences"
    for module in ("🌉️bridge-spectrum", "🛢️tank-pressure", "🗼️tower-modal"):
        text = rd(f"{inf}/{module}/🦀️.rs")
        text = text.replace("pub(super) ", "pub ")
        wr(f"{directory}/{module}/🦀️.rs", text)
        if not DRY:
            os.remove(f"{inf}/{module}/🦀️.rs")
            os.rmdir(f"{inf}/{module}")
    root = '''//! 🌉️ Structure checks of EN 1998 parts 2, 4 and 6: the spectrum analysis of the bridge rows, the hydrodynamic pressure and sloshing of the tanks, and the modal response and the foundation interaction of the towers.

extern crate semio_s_artifact_norm_en1998_model as model;
extern crate semio_s_artifact_norm_en1998_schema as schema;
extern crate semio_s_plugin_norm_compliance as compliance;

use compliance::{AnnexChoice, CheckReport, CheckResult, ClauseId, LocalizedCopy, Quantity, QuantityKind, Remedy, SubjectRef};
use model::En1998Snapshot;
use schema::{part_1, part_2};

fn q_force(n: f64) -> Quantity { Quantity::new(QuantityKind::Force, n) }

fn q_len(m: f64) -> Quantity { Quantity::length_m(m) }

fn q_dim(v: f64) -> Quantity { Quantity::new(QuantityKind::Dimensionless, v) }

fn q_press(pa: f64) -> Quantity { Quantity::new(QuantityKind::Pressure, pa) }

fn q_mom(nm: f64) -> Quantity { Quantity::new(QuantityKind::Moment, nm) }

/// 🛡️ A resistance of zero or less is floored to a tiny positive limit so a positive demand fails instead of passing at utilisation 0.
fn floor_limit(limit: Quantity) -> Quantity { Quantity::new(limit.kind, limit.value.max(1e-9)) }

fn lc(en: &str, de: &str) -> LocalizedCopy { LocalizedCopy::new(en, de) }

fn id_sel(collection: &str, id: &str) -> String {
    format!("{collection}[id={id}]")
}

#[path = "🌉️bridge-spectrum/🦀️.rs"]
mod bridge_spectrum;
pub use bridge_spectrum::{analyse as bridge_analysis, check_rows as check_bridge_rows, Analysis, Ground};

#[path = "🛢️tank-pressure/🦀️.rs"]
mod tank_pressure;
pub use tank_pressure::check_tank_hydrodynamics;

#[path = "🗼️tower-modal/🦀️.rs"]
mod tower_modal;
pub use tower_modal::check_tower_modal;
'''
    wr(directory + "/🦀️.rs", root)
    deps = ["semio-s-artifact-norm-en1998-model", "semio-s-artifact-norm-en1998-schema", "semio-s-plugin-norm-compliance"]
    wr(directory + "/📦️packages/🦀️rust/Cargo.toml", manifest("semio-s-artifact-norm-en1998-structure-checks", "EN 1998 structure checks: bridges, tanks and towers", 8, deps))
    register("semio-s-artifact-norm-en1998-structure-checks", directory, deps, ["semio-s-artifact-norm-en1998-inferences"])
    add_dependency(inf + "/📦️packages/🦀️rust/Cargo.toml", "semio-s-artifact-norm-en1998-structure-checks")
    root_path = inf + "/🦀️.rs"
    text = rd(root_path)
    a = text.index('#[path = "🌉️bridge-spectrum/🦀️.rs"]')
    b = text.index('use tower_modal::check_tower_modal;\n') + len('use tower_modal::check_tower_modal;\n')
    text = text[:a] + "use semio_s_artifact_norm_en1998_structure_checks::{bridge_analysis, check_bridge_rows, check_tank_hydrodynamics, check_tower_modal, Ground};\n" + text[b:]
    wr(root_path, text)


def edit_rules():
    directory = ANY + "/🧬️schema/🧬️mutations/🧭️edit-rules"
    if os.path.exists(directory + "/📦️packages"):
        return
    file = directory + "/🦀️.rs"
    text = rd(file)
    text = text.replace("use crate::En1998Mutation;", "use semio_s_artifact_norm_en1998_mutations::En1998Mutation;")
    wr(file, text)
    deps = ["semio-s-artifact-norm-en1998-model", "semio-s-artifact-norm-en1998-mutations", "semio-s-plugin-norm-app-surface"]
    wr(directory + "/📦️packages/🦀️rust/Cargo.toml", manifest("semio-s-artifact-norm-en1998-edit-rules", "EN 1998 editor edit rules: which value-tree path raises which mutation kind", 9, deps))
    register("semio-s-artifact-norm-en1998-edit-rules", directory, deps, ["semio-s-artifact-norm-en1998-editor", "semio-s-artifact-norm-en1998"])
    mutations = ANY + "/🧬️schema/🧬️mutations/🦀️.rs"
    t = rd(mutations)
    t = t.replace('#[path = "🧭️edit-rules/🦀️.rs"]\nmod edit_rules;\npub use crate::edit_rules::{resolve_edit, EDIT_RULES};\n', "")
    wr(mutations, t)
    add_dependency(ANY + "/✏️editor/📦️packages/🦀️rust/Cargo.toml", "semio-s-artifact-norm-en1998-edit-rules")
    for command in ("➖remove-item", "✏️set-field", "🩹apply-remedy", "➕insert-item"):
        path = f"{ANY}/✏️editor/🎮️commands/{command}/🦀️.rs"
        wr(path, rd(path).replace("semio_s_artifact_norm_en1998_mutations::resolve_edit", "semio_s_artifact_norm_en1998_edit_rules::resolve_edit"))
    root_manifest = ART + "/📦️packages/🦀️rust/Cargo.toml"
    print("root crate manifest:", root_manifest)


scene_structures()
structure_checks()
edit_rules()
