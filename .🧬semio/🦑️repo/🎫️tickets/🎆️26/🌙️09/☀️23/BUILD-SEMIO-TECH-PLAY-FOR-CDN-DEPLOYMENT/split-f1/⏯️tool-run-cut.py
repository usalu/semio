#!/usr/bin/env python3
"""⏯️ Cuts the two consumer-free leaves out of `semio-framework-tool-run` so its library unit fits the byte ceiling.

- `semio-framework-tool-run-entities` (`👥️entities`): caller-owned entity edit workspace, version and provenance; below
  the core (it names only `semio-framework-value`).
- `semio-framework-tool-run-presentation` (`📊️progress/🪟️presentation`): the joint progress presentation, seed, update
  and slot; above the core (it names core progress/tick records and the entities crate).
The core drops both mounts and both re-exports; the five presentation laws move from the core unit tests to the
presentation crate's own unit tests.

usage: ⏯️tool-run-cut.py apply
"""
import os, re, sys

REPO = os.path.abspath(os.path.join(os.path.dirname(os.path.abspath(__file__)), *[".."] * 8))
TR = os.path.join(REPO, "🧰️framework/🔨️modules/⏯️tool-run")
ENT = os.path.join(TR, "👥️entities")
PRE = os.path.join(TR, "📊️progress/🪟️presentation")
ALLOC = "\n#[cfg(test)]\n#[global_allocator]\nstatic {name}:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;\n"


def edit(path, pairs):
    text = open(path, encoding="utf-8").read()
    for old, new in pairs:
        if old not in text:
            raise SystemExit(f"anchor missing in {os.path.relpath(path, REPO)}: {old[:80]}")
        text = text.replace(old, new, 1)
    open(path, "w", encoding="utf-8").write(text)


def manifest(pkg, package, description, deps, dev):
    os.makedirs(pkg, exist_ok=True)
    rows = "\n".join(f"{k} = {v}" for k, v in deps.items())
    dev_rows = "\n".join(f"{k} = {v}" for k, v in dev.items())
    open(os.path.join(pkg, "Cargo.toml"), "w", encoding="utf-8").write(
        f'[package]\nworkspace = "{os.path.relpath(os.path.join(REPO, "🧰️framework"), pkg)}"\nname = "{package}"\nversion = "0.1.0"\nedition = "2021"\nrust-version.workspace = true\n'
        f'description = "{description}"\n\n[package.metadata.semio]\nrole = "framework"\nid = "{package.removeprefix("semio-framework-")}"\n\n'
        f"[package.metadata.semio.rust-source-boundary]\nmaxSourceBytes = 262144\nmaxSourceFiles = 32\nmaxActiveLines = 4000\n\n[lints]\nworkspace = true\n\n"
        f'[lib]\nname = "{package.replace("-", "_")}"\npath = "../../🦀️.rs"\n\n[dependencies]\n{rows}\n\n[dev-dependencies]\n{dev_rows}\n')


def modules(pkg, rel):
    return os.path.relpath(os.path.join(REPO, "🧰️framework/🔨️modules", rel), pkg)


def main():
    if sys.argv[1:] != ["apply"]:
        raise SystemExit(__doc__)
    ent_pkg, pre_pkg = os.path.join(ENT, "📦️packages/🦀️rust"), os.path.join(PRE, "📦️packages/🦀️rust")
    manifest(ent_pkg, "semio-framework-tool-run-entities", "Caller-owned ToolRun entity edit workspace, entity versions and provenance with exact original retirement.",
             {"semio-framework-value": f'{{ path = "{modules(ent_pkg, "🌱️value/📦️packages/🦀️rust")}" }}'},
             {"semio-framework-trace": f'{{ path = "{modules(ent_pkg, "⏱️trace/📦️packages/🦀️rust")}" }}', "serde_json": "{ workspace = true }"})
    manifest(pre_pkg, "semio-framework-tool-run-presentation", "Joint ToolRun progress presentation: issuer-owned body, seed, atomic tick update and publishing slot.",
             {"semio-framework-value": f'{{ path = "{modules(pre_pkg, "🌱️value/📦️packages/🦀️rust")}" }}',
              "semio-framework-tool-run": f'{{ path = "{modules(pre_pkg, "⏯️tool-run/📦️packages/🦀️rust")}" }}',
              "semio-framework-tool-run-entities": f'{{ path = "{modules(pre_pkg, "⏯️tool-run/👥️entities/📦️packages/🦀️rust")}" }}'},
             {"semio-framework-trace": f'{{ path = "{modules(pre_pkg, "⏱️trace/📦️packages/🦀️rust")}" }}', "serde_json": "{ workspace = true }"})

    ent_root = os.path.join(ENT, "🦀️.rs")
    text = open(ent_root, encoding="utf-8").read()
    if "global_allocator" not in text:
        open(ent_root, "w", encoding="utf-8").write(text.rstrip("\n") + "\n" + ALLOC.format(name="TOOL_RUN_ENTITIES_HEAP"))

    edit(os.path.join(PRE, "🦀️.rs"), [
        ("use super::{ToolRunProgress,ToolRunIdentity,ToolRunState};", "use semio_framework_tool_run::{ToolRunProgress,ToolRunIdentity,ToolRunState};"),
        ("use super::entities::provenance::ToolRunEntityProvenance;", "use semio_framework_tool_run_entities::provenance::ToolRunEntityProvenance;"),
        ("&'a super::entities::EntityMap", "&'a semio_framework_tool_run_entities::EntityMap"),
    ])
    edit(os.path.join(PRE, "🌱️seed/🦀️.rs"), [("use super::super::{ToolRunIdentity,ToolRunState,ToolRunStepRing};", "use semio_framework_tool_run::{ToolRunIdentity,ToolRunState,ToolRunStepRing};")])
    edit(os.path.join(PRE, "🗃️slot/🦀️.rs"), [("use super::super::{ToolRunIdentity,ToolRunState,ToolRunTick,entities::{ToolRunEntityEditGrant,ToolRunEntityEditProgress}};",
                                             "use semio_framework_tool_run::{ToolRunIdentity,ToolRunState,ToolRunTick};\nuse semio_framework_tool_run_entities::{ToolRunEntityEditGrant,ToolRunEntityEditProgress};")])
    upd = os.path.join(PRE, "🧬️update/🦀️.rs")
    text = open(upd, encoding="utf-8").read()
    m = re.search(r"use super::super::\{(ToolRunProgress,.*?),entities::\{(.*)\}\};\n", text)
    if not m:
        raise SystemExit("update import anchor missing")
    text = text[:m.start()] + f"use semio_framework_tool_run::{{{m.group(1)}}};\nuse semio_framework_tool_run_entities::{{{m.group(2)}}};\n" + text[m.end():]
    open(upd, "w", encoding="utf-8").write(text)

    core_tests = os.path.join(TR, "🧪️tests/🔬️unit/🦀️.rs")
    lines = open(core_tests, encoding="utf-8").read().split("\n")
    start = next(i for i, line in enumerate(lines) if line.startswith("fn original_tool_run_progress_metadata_presentation_captures_keep_joint_original_body")) - 1
    end = next(i for i, line in enumerate(lines) if line.startswith("fn original_tool_run_progress_metadata_tick_source_keeps_actual_operation_and_trace_owners")) - 1
    moved = "\n".join(lines[start:end]).rstrip("\n")
    del lines[start:end]
    open(core_tests, "w", encoding="utf-8").write("\n".join(lines))
    moved = moved.replace("use super::{presentation::{ToolRunPresentation,ToolRunPresentationBody},entities::provenance::ToolRunEntityProvenance};",
                          "use super::{ToolRunPresentation,ToolRunPresentationBody};\n use semio_framework_tool_run_entities::provenance::ToolRunEntityProvenance;")
    moved = moved.replace("use super::{presentation::{ToolRunPresentation,ToolRunPresentationBody,update::ToolRunPresentationUpdate},entities::{ToolRunEntityEditGrant,provenance::ToolRunEntityProvenance}};",
                          "use super::{ToolRunPresentation,ToolRunPresentationBody,update::ToolRunPresentationUpdate};\n use semio_framework_tool_run_entities::{ToolRunEntityEditGrant,provenance::ToolRunEntityProvenance};")
    moved = moved.replace("use super::presentation::seed::ToolRunPresentationSeed;", "use super::seed::ToolRunPresentationSeed;")
    moved = moved.replace("use super::presentation::{ToolRunCapturedView,slot::ToolRunPresentationSlot};", "use super::{ToolRunCapturedView,slot::ToolRunPresentationSlot};")
    moved = moved.replace("use super::entities::ToolRunEntityEditGrant;", "use semio_framework_tool_run_entities::ToolRunEntityEditGrant;")
    moved = moved.replace("super::presentation::ToolRunPresentation", "super::ToolRunPresentation")
    moved = moved.replace('include_str!("../../📊️progress/🪟️presentation/🧪️tests/🧫️fixtures/🔣️.json")', 'include_str!("../🧫️fixtures/🔣️.json")')
    if "super::presentation" in moved or "super::entities" in moved:
        raise SystemExit("unrewritten presentation path in moved laws")
    os.makedirs(os.path.join(PRE, "🧪️tests/🔬️unit"), exist_ok=True)
    header = ("//! 🔬️ Joint presentation laws: capture, atomic update, foreign-original refusal, seed admission and slot publication.\n\n"
              "use super::*;\nuse semio_framework_tool_run::*;\nuse serde_json::Value;\n\n"
              'const TICKS: &str = include_str!("../../../../🧪️tests/🧫️fixtures/🎞️ticks.json");\n\n'
              "fn fixture(text: &str) -> Value {\n    serde_json::from_str(text).expect(\"fixture parses\")\n}\n\n")
    open(os.path.join(PRE, "🧪️tests/🔬️unit/🦀️.rs"), "w", encoding="utf-8").write(header + moved + "\n")
    pre_root = os.path.join(PRE, "🦀️.rs")
    text = open(pre_root, encoding="utf-8").read()
    if "global_allocator" not in text:
        open(pre_root, "w", encoding="utf-8").write(text.rstrip("\n") + "\n" + ALLOC.format(name="TOOL_RUN_PRESENTATION_HEAP") + '\n#[cfg(test)]\n#[path = "🧪️tests/🔬️unit/🦀️.rs"]\nmod tests;\n')

    edit(os.path.join(TR, "🦀️.rs"), [('#[path="👥️entities/🦀️.rs"]\npub mod entities;\n', ""), ('#[path="📊️progress/🪟️presentation/🦀️.rs"]\npub mod presentation;\n', "")])
    edit(os.path.join(TR, "📦️packages/🦀️rust/🦀️.rs"), [("pub use component::entities;\n", ""), ("pub use component::presentation;\n", "")])


main()
