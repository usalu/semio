"""🔌️ Wires `history_edit_acceptance_law!` into one plugin crate test context per row: one line after the `fn <manifest>() -> App`
item, inserted once (idempotent), re-reading each file right before writing (S2-AGNOSTIC, design §16.3)."""
import re, sys, subprocess
ROOT = "/Users/ueli/Documents/semio"
ROWS = [
    ("🌍️gis/🗿️artifacts/🏔️gisterrain", "gis3d_app_manifest_for_tests", "gis", "Gis3dPlayApp"),
    ("📏️layout/🗿️artifacts/📏️layout", "layout_app_manifest_for_tests", "layout", "LayoutPlayApp"),
    ("🗒️note/🗿️artifacts/🗒️note", "note_manifest_for_tests", "note", "NotePlayApp"),
    ("🎥️shooting/🗿️artifacts/🎥️shooting", "shooting_app_manifest_for_tests", "shooting", "ShootingPlayApp"),
    ("🖨️raster/🗿️artifacts/🖨️raster", "raster_app_manifest_for_tests", "raster", "RasterPlayApp"),
    ("💠️lowpoly/🗿️artifacts/💠️lowpoly", "lowpoly_manifest_for_tests", "lowpoly", "LowpolyPlayApp"),
    ("📕️norm/🗿️artifacts/⚖️en1990", "en1990_manifest_for_tests", "norm", "En1990PlayApp"),
    ("🧱️block/🗿️artifacts/◻️2d", "block2d_app_manifest_for_tests", "block", "Block2dPlayApp"),
    ("🌀️procedural/🗿️artifacts/🌀️generation2d", "generation2d_manifest_for_tests", "procedural", "Generation2dPlayApp"),
    ("🌿️vcs/🗿️artifacts/🌿️vcs", "vcs_app_manifest_for_tests", "vcs", "VcsPlayApp"),
    ("📸️remodel/🗿️artifacts/📸️remodeling", "remodeling_app_manifest_for_tests", "remodel", "RemodelingPlayApp"),
    ("✒️writer/🗿️artifacts/✒️writer", "writer_app_manifest_for_tests", "writer", "WriterPlayApp"),
    ("➗️mathematical/🗿️artifacts/➗️equation", "equation_app_manifest_for_tests", "mathematical", "EquationPlayApp"),
    ("🎞️animate/🗿️artifacts/🎬️presentation", "animate_presentation_app_manifest_for_tests", "animate", "AnimatePresentationPlayApp"),
    ("🏛️architect/🗿️artifacts/🏛️program", "architect_app_manifest_for_tests", "architect", "ArchitectPlayApp"),
    ("🏭️process/🗿️artifacts/🧊️process3d", "process3d_app_manifest_for_tests", "process", "Process3dPlayApp"),
    ("💡️reasoning/🗿️artifacts/🔌️wires", "wires_manifest_for_tests", "reasoning", "ReasoningWiresPlayApp"),
    ("📋️forms/🗿️artifacts/📋️forms", "forms_manifest_for_tests", "forms", "FormsPlayApp"),
    ("📐️cad/🗿️artifacts/📐️cad", "cad_app_manifest_for_tests", "cad", "CadPlayApp"),
    ("📖️playbook/🗿️artifacts/📖️playbook", "playbook_manifest_for_tests", "playbook", "PlaybookPlayApp"),
    ("📜️imperative/🗿️artifacts/📜️procedure", "imperative_app_manifest_for_tests", "imperative", "ImperativePlayApp"),
    ("🔋️energy/🗿️artifacts/🔋️model", "energy_model_manifest_for_tests", "energy", "EnergyModelEditor"),
    ("🔱️trinity/🗿️artifacts/♻️rewriting", "trinity_rewriting_manifest_for_tests", "trinity", "TrinityRewritingPlayApp"),
    ("🕸️dag/🗿️artifacts/🕸️dag", "dag_app_manifest_for_tests", "dag", "DagPlayApp"),
    ("🪐️space/🗿️artifacts/🪐️space", "space_index_manifest_for_tests", "space", "SpaceIndexEditor"),
    ("🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧪️tests/🔬️unit", "manifest", "fem", "Fem2dPlayApp"),
    ("🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧪️tests/🔬️unit", "manifest", "fem", "Fem3dPlayApp"),
    ("🎬️sequence/🗿️artifacts/🎬️sequence", "sequence_manifest_for_tests", "sequence", "SequencePlayApp"),
    ("🧩️puzzle/🗿️artifacts/🧊️3d", "puzzle3d_manifest_for_tests", "puzzle", "Puzzle3dPlayApp"),
    ("🧩️puzzle/🗿️artifacts/🖐️5d", "puzzle5d_app_manifest_for_tests", "puzzle", "Puzzle5dPlayApp"),
    ("🪵️sourcing/🗿️artifacts/🗂️curation", "sourcing_manifest_for_tests", "sourcing", "SourcingCurationApp"),
    ("🎪️demonstrator/🗿️artifacts/🎪️playground", "playground_editor_manifest_for_tests", "demonstrator", "PlaygroundEditor"),
]
only = set(sys.argv[1:])
for directory, manifest, plugin, editor in ROWS:
    if only and plugin not in only and editor not in only:
        continue
    paths = subprocess.run(["git", "grep", "-l", f"fn {manifest}()", "--", f"✏️s/🔌️plugins/{directory}/*.rs"], cwd=ROOT, capture_output=True, text=True).stdout.split()
    assert len(paths) == 1, (plugin, paths)
    path = f"{ROOT}/{paths[0]}"
    lines = open(path, encoding="utf-8").read().split("\n")
    if any("history_edit_acceptance_law!" in line for line in lines):
        print("already", plugin, paths[0]); continue
    start = next(i for i, line in enumerate(lines) if re.search(rf"\bfn {manifest}\(\)", line))
    indent = re.match(r"\s*", lines[start]).group(0)
    depth, end = 0, start
    for end in range(start, len(lines)):
        depth += lines[end].count("{") - lines[end].count("}")
        if depth == 0 and "{" in "".join(lines[start:end + 1]):
            break
    lines[end + 1:end + 1] = ["", f'{indent}semio_framework_plugin::history_edit_acceptance_law!("{plugin}", {editor}, {manifest}, "../..");']
    open(path, "w", encoding="utf-8").write("\n".join(lines))
    print("wired", plugin, paths[0], "after line", end + 1)
