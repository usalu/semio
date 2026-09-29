#!/usr/bin/env python3
"""🧰️ CD1: builds the specs of the three session-15 sets from the overlay + their bases and generates the set scripts
(`wp-cd1/t6-step-exchange/`, `wp-cd1/t6-construction/`, `wp-cd1/t6-planar-exact/`) with `cd1-make-set.py`. Land order: A → B → C
(B's two files shared with A are diffed from A's result `s15/s1/`)."""
import glob
import json
import os
import subprocess

HUB = "/Users/ueli/Documents/semio/.🧬semio/🌐hub"
O = f"{HUB}/s14-cd1-overlay"
W = f"{HUB}/s14-cd1-work/s15"
T = "/Users/ueli/Documents/semio/.tmp-ticket/wp-cd1"
BREP = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep"
CAD = "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any"
KERNEL = "✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine"
EDITOR = f"{CAD}/✏️editor/🦀️.rs"
SUITE = f"{KERNEL}/🧠️semio/🧪️tests/🧪️semio-tech-cad-js-spatial-kernel-semio/🟦️.ts"


def rel_files(base_dir):
    return sorted(os.path.relpath(path, base_dir) for path in glob.glob(f"{base_dir}/**/*", recursive=True) if os.path.isfile(path))


def spec(name, doc, base_dir, shared_target, creates, shared_base=None):
    edits = []
    for rel in sorted(set(rel_files(base_dir)) | set(shared_base or {})):
        base = shared_base.get(rel, f"{base_dir}/{rel}") if shared_base else f"{base_dir}/{rel}"
        target = shared_target.get(rel, f"{O}/{rel}")
        if open(base, "rb").read() != open(target, "rb").read():
            edits.append([rel, base, target])
    return {"name": name, "doc": doc, "backups": f"{HUB}/s14-cd1-t6/{name}", "edits": edits, "creates": [[rel, f"{O}/{rel}"] for rel in creates]}


S1 = {EDITOR: f"{W}/s1/{EDITOR}", SUITE: f"{W}/s1/{SUITE}"}
sets = [
    ("t6-step-exchange", "cd1-step-exchange", spec("cd1-step-exchange", "📤️ CD1 T6 set A — cad `brep:out` exports every pane shape through the semio brep's ONE AP214 writer: the kernel's `export_step` projects solids, shells, faces, wires and edges to a snapshot and writes it with `semio_brep_part21` (moved ungated into the kernel's `🟫️step`, stdio `SemioBrepToStep` wraps it; the kernel's own solid-only writer, its `({items},)` Part-21 error and MILLI.METRE units are gone): product structure, metre/radian context, one representation per class (ABSR solids incl. conformant BREP_WITH_VOIDS/ORIENTED_CLOSED_SHELL, MSSR open shells for free faces, EBWSR connected edge sets for free wires); `SemioBrepFromStep` reads BREP_WITH_VOIDS/ORIENTED_CLOSED_SHELL/ORIENTED_FACE; cad drops the repair shim and the STEP→semio→STEP double trip, export errors carry their reason (`cad.export.failed`, no silent spatial fallback); laws: step-exchange vectors (golden bytes + census), refusals, OpenCascade void reader, modelspace brep:out, cad export laws, OCCT exchange law (TS).", f"{W}/base", S1, [f"{BREP}/🧫️fixtures/📤️step-exchange/🔣️.json"])),
    ("t6-construction", "cd1-construction", spec("cd1-construction", "📐️ CD1 T6 set B (after A) — construction from two points and a height, schema-first: `spatial.typology` JSON Schema, `construction.from2PointsAndHeight` on the 20 two-point typologies (building wall 0.3 m / door, window 0.1 m centred segment prisms; rectangle, circle, surface and curve readings), building typologies bind solids (the forest's ground truth); one planner twinned TS/Rust over the vectors `🧬️typology/🧫️fixtures/📐️construction-from-2-points` (independent Python planner); both TS kernels realise plans (prism = box → yaw → translate, cylinder, quad face, line wire), the Rust commit persists the plan's frame and `typology_local_shape` builds faces/lines from flat extents; the name heuristics (`inferTypologyPrimitiveKinds`, `lower.contains(\"wall\")`) are gone; laws: schema census (AJV), planner vectors TS + Rust, OpenCascade realisation, diagonal wall in the kernel, interaction laws.", f"{W}/base2", {}, [f"{CAD}/✏️editor/⚙️engine/🧬️typology/🧬️schema/🔣️.json", f"{CAD}/✏️editor/⚙️engine/🧬️typology/🧫️fixtures/📐️construction-from-2-points/🔣️.json"], shared_base=S1)),
    ("t6-planar-exact", "cd1-planar-exact", spec("cd1-planar-exact", "📏️ CD1 T6 set C — exact planar-face mass properties: a planar loop's area and divergence-theorem moments come from Green's theorem over its exact edge curves (16-point Gauss–Legendre per ≤π/8 arc span) instead of a polygonised UV boundary, so a cylinder's or a cone's cap is πr² and a solid's volume/centre of mass no longer depend on where it sits (a translated cone lost 1.3e-4 of its volume); the dead polygon helpers are removed; law: placement exactness at 1e-9.", f"{W}/base3", {}, [])),
]
for folder, name, body in sets:
    os.makedirs(f"{T}/{folder}", exist_ok=True)
    spec_path = f"{W}/{name}.spec.json"
    json.dump(body, open(spec_path, "w", encoding="utf-8"), ensure_ascii=False)
    subprocess.run(["python3", f"{T}/cd1-make-set.py", spec_path, f"{T}/{folder}/{name}.py"], check=True)
