from pathlib import Path
import re
import json

path = Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs")
ticket = Path(__file__).resolve().parent

lines = path.read_text().splitlines(keepends=True)

wp_start = wp_end = None
for i, line in enumerate(lines):
    if "//#region" in line and "WorldProjection" in line:
        wp_start = i
    if wp_start is not None and "//#endregion" in line and "WorldProjection" in line:
        wp_end = i
        break

CAMERA_RES = [
    re.compile(r"WorldProjection"),
    re.compile(r"world3d_projection"),
    re.compile(r"world3d_camera_projection"),
    re.compile(r"setProjection"),
    re.compile(r"Oblique_projection"),
    re.compile(r"Axonometric_projection"),
    re.compile(r"computeWorldProjection"),
    re.compile(r"projection_measures_tree_matches"),
    re.compile(r"projection_spec_json_projects_only_active_kind"),
    re.compile(r"id_prefix\}-projection"),
    re.compile(r"-projection-orthographic"),
    re.compile(r"-projection-axonometric"),
    re.compile(r"-projection-oblique"),
    re.compile(r'measure_group_with_open\(format!\("\{id_prefix\}-projection"'),
    re.compile(r'"Projection"'),
    re.compile(r"Plugin-owned projection state for a `world-3d`"),
    re.compile(r"Canonical camera pose .* projection config"),
    re.compile(r"MediaType` projection"),
]

REPLACEMENTS = [
    (r"DummyProjection", "DummySnapshot"),
    (r"TestProjection", "TestSnapshot"),
    (r"revert_to_command_restores_the_projection_and_appends_one_entry", "revert_to_command_restores_the_snapshot_and_appends_one_entry"),
    (r"document_projection_schema", "document_snapshot_schema"),
    (r"projection_override_json", "snapshot_override_json"),
    (r"override_projection", "override_snapshot"),
    (r"draft_projection", "draft_snapshot"),
    (r"projection_with_conflicts", "snapshot_with_conflicts"),
    (r"initial_projection", "initial_snapshot"),
    (r"test_projection", "test_snapshot"),
    (r"projection_a", "snapshot_a"),
    (r"projection_b", "snapshot_b"),
    (r"type Projection", "type Snapshot"),
    (r"\$Projection", r"$Snapshot"),
    (r"A::Projection", "A::Snapshot"),
    (r"Self::Projection", "Self::Snapshot"),
    (r"<A as DocumentApp>::Projection", "<A as DocumentApp>::Snapshot"),
    (r"DocumentApp::Projection", "DocumentApp::Snapshot"),
    (r"\.projection_with_conflicts\b", ".snapshot_with_conflicts"),
    (r"\.projection\b", ".snapshot"),
    (r"\bfn projection\b", "fn snapshot"),
    (r"\bfn projection_with_conflicts\b", "fn snapshot_with_conflicts"),
    (r"pub fn projection\b", "pub fn snapshot"),
    (r"\bpub projection:", "pub snapshot:"),
    (r"\bprojection:", "snapshot:"),
    (r"\blet mut projection\b", "let mut snapshot"),
    (r"\blet projection\b", "let snapshot"),
    (r"\bprojection\)", "snapshot)"),
    (r"\(projection,", "(snapshot,"),
    (r", projection,", ", snapshot,"),
    (r", projection\)", ", snapshot)"),
    (r"\{ projection,", "{ snapshot,"),
    (r"\{ projection \}", "{ snapshot }"),
    (r"&projection\b", "&snapshot"),
    (r"\bprojection\.", "snapshot."),
    (r"\bprojection,", "snapshot,"),
    (r"\bprojection;", "snapshot;"),
    (r"\bprojection =", "snapshot ="),
    (r"\(_projection:", "(_snapshot:"),
    (r"\b_projection\b", "_snapshot"),
    (r"\bprojection\b", "snapshot"),
    (r"\bProjection\b", "Snapshot"),
]

def is_camera_line(i, line):
    if wp_start is not None and wp_start <= i <= wp_end:
        return True
    return any(r.search(line) for r in CAMERA_RES)

kept = []
renamed = []
new_lines = []

for i, line in enumerate(lines):
    if not re.search(r"projection|Projection", line):
        new_lines.append(line)
        continue
    if is_camera_line(i, line):
        kept.append({"line": i + 1, "text": line.rstrip("\n"), "reason": "camera/world3d or protected media phrase"})
        new_lines.append(line)
        continue

    original = line
    new = line
    for pat, repl in REPLACEMENTS:
        new = re.sub(pat, repl, new)

    ctx = "".join(lines[max(0, i - 20) : i + 1])
    if "WindowKindDefinition" in ctx and re.search(r"document_snapshot_schema:\s*window\.document_snapshot_schema", new):
        new = new.replace(
            "document_snapshot_schema: window.document_snapshot_schema",
            "document_projection_schema: window.document_snapshot_schema",
            1,
        )
    if "def.document_snapshot_schema" in new:
        new = new.replace("def.document_snapshot_schema", "def.document_projection_schema")

    if new != original:
        renamed.append({"line": i + 1, "before": original.rstrip("\n"), "after": new.rstrip("\n")})
    else:
        kept.append({"line": i + 1, "text": original.rstrip("\n"), "reason": "no-op after replacements — review"})
    new_lines.append(new)

joined = "".join(new_lines)

for sym in [
    "WorldProjectionConfig",
    "world3d_projection_spec_json",
    "setProjection",
    "apply_world3d_projection_action",
    "Axonometric_projection",
    "Oblique_projection",
    "projection_measures_tree_matches_the_requested_taxonomy",
    "projection_spec_json_projects_only_active_kind_fields",
]:
    if sym not in joined:
        print("ERROR missing camera symbol:", sym)

for sym in [
    "type Projection",
    "DummyProjection",
    "TestProjection",
    "initial_projection",
    "A::Projection",
    "Self::Projection",
    "$Projection",
    "fn projection(",
    "test_projection",
    "draft_projection",
    "projection_override_json",
    "override_projection",
    "projection_with_conflicts",
]:
    if sym in joined:
        for j, l in enumerate(joined.splitlines()):
            if sym in l and not any(r.search(l) for r in CAMERA_RES) and not (wp_start <= j <= wp_end):
                print("REMAINING DOC SYM", sym, "at", j + 1, ":", l.strip()[:100])

path.write_text(joined)
print("Wrote", path)
print("renamed_lines", len(renamed))
print("kept_lines", len(kept))
print("total_projection_lines_before", len(renamed) + len(kept))

out = {
    "renamed_line_count": len(renamed),
    "kept_line_count": len(kept),
    "world_projection_region": [wp_start + 1, wp_end + 1],
    "kept": kept,
    "renamed": renamed,
}
(ticket / "🧪fixup-classification.json").write_text(json.dumps(out, indent=2))

rem = [(i + 1, l) for i, l in enumerate(joined.splitlines()) if re.search(r"projection|Projection", l)]
print("remaining projection lines:", len(rem))
for i, l in rem:
    print(f"{i}: {l.strip()[:160]}")
