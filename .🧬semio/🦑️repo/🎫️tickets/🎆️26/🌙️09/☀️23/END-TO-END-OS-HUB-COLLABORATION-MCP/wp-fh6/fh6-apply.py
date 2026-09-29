"""✍️ FH6 class review: rewrites ONLY the `class` field of reviewed BD work-list entries (one entry per line, byte-stable
otherwise) and prints per-class change counts. Usage: python3 fh6-apply.py [--dry]"""
import json
import sys
from collections import Counter
from pathlib import Path

WORK = Path("/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s20-sets/class/family-BD.json")
I, P, C, D, N = "input-invalid", "precondition-failed", "conflict", "internal", "cancelled"
REVIEW = {
    "fem.editor.unhandled-action": I, "fem.window.stale": P, "fem2d-command-retained-route-rejected": D,
    "fem2d.analysis.field": I, "fem2d.camera.window-stale": P, "fem2d.focus-entity.unknown-entity": P,
    "fem2d.result-animation.field": I, "fem2d.results.field": I, "fem3d.analysis.field": I, "fem3d.camera.orbit-required": I,
    "fem3d.camera.window-stale": P, "fem3d.focus-entity.unknown-entity": P, "fem3d.focus-entity.window-stale": P,
    "fem3d.gumball-flag.window-stale": P, "fem3d.result-animation-tick.retained-route-required": D,
    "fem3d.result-animation-tick.window-transient-mismatch": D, "fem3d.result-animation.field": I, "fem3d.results.field": I,
    "drawing-canvas-window-stale": P, "drawing-viewer-camera-required": I, "drawing-viewer-retained-route-required": D,
    "drawing-window-work-terminal": D, "drawing.arrange.align-count": P, "drawing.arrange.bounds-missing": P,
    "drawing.arrange.capacity": P, "drawing.arrange.distribute-count": P, "drawing.child-projection": D,
    "drawing.editor.unhandled-action": I, "drawing.example.parse": D, "drawing.fill.edit-missing": I,
    "drawing.fill.stop-invalid": P, "drawing.fill.stop-limit": P, "drawing.gesture.command": D,
    "drawing.gesture.retained-route": D, "drawing.group.compositing": P, "drawing.mutation.rejected": D,
    "drawing.path.contour-invalid": P, "drawing.path.edit-missing": I, "drawing.path.endpoints-same": I,
    "drawing.path.geometry-invalid": P, "drawing.path.point-invalid": P, "drawing.path.points-changed": C,
    "drawing.selection.stack-invalid": P,
    "bounded-native-edit.command-mismatch": D, "snapshot-edit.ambiguous-object": I, "snapshot-edit.command-mismatch": D,
    "snapshot-edit.depth-exceeded": I, "snapshot-edit.descendant-move": I, "snapshot-edit.invalid-schema-contract": D,
    "snapshot-edit.inverse-invalid": D, "snapshot-edit.inverse-mismatch": D, "snapshot-edit.lossy-conversion": I,
    "snapshot-edit.path-invalid": P, "snapshot-edit.path-shape": I, "snapshot-edit.publication-codec": D,
    "snapshot-edit.publication-invalid": D, "snapshot-edit.publication-mismatch": D, "snapshot-edit.root-operation": I,
    "snapshot-edit.schema-identity": I, "snapshot-edit.schema-unregistered": D, "stdio-txt-schema-is-immutable": I,
    "stdio.bcf.column-stale": I, "stdio.bcf.row-stale": I, "stdio.csv.cell-stale": I, "stdio.csv.column-stale": I,
    "stdio.csv.header-stale": I, "stdio.csv.row-stale": I, "stdio.docx.set-page.address-duplicate": I,
    "stdio.docx.set-page.address-field": I, "stdio.docx.set-page.address-field-duplicate": I,
    "stdio.docx.set-page.address-fields": I, "stdio.docx.set-page.address-required": I,
    "stdio.docx.set-page.command-mismatch": D, "stdio.docx.set-page.copy-incomplete": D,
    "stdio.docx.set-page.copy-refused": I, "stdio.docx.set-page.node-path": I, "stdio.docx.set-page.node-path-index": I,
    "stdio.docx.set-page.paged-owner-required": P, "stdio.editor.native-edit-command-mismatch": D,
    "stdio.editor.retained-route-required": D, "stdio.editor.unhandled-action": I, "stdio.epw.command-mismatch": D,
    "stdio.epw.row-range": P, "stdio.gltf.inference.snapshot-decode": D, "stdio.gltf.inference.unknown-leaf": D,
    "stdio.mp4.index-out-of-range": P, "stdio.pdf.page.annotation-unaddressable": I,
    "stdio.pdf.page.catalog-key-required": I, "stdio.pdf.page.color-space-name-required": I,
    "stdio.pdf.page.destination-name-required": I, "stdio.pdf.page.embedded-file-id-required": I,
    "stdio.pdf.page.entry-key-required": I, "stdio.pdf.page.font-name-required": I,
    "stdio.pdf.page.form-field-name-required": I, "stdio.pdf.page.form-id-required": I,
    "stdio.pdf.page.form-setting-name-required": I, "stdio.pdf.page.glyph-name-required": I,
    "stdio.pdf.page.graphics-state-id-required": I, "stdio.pdf.page.image-samples-size": I,
    "stdio.pdf.page.layer-id-required": I, "stdio.pdf.page.mask-image-required": I, "stdio.pdf.page.open-uri-required": I,
    "stdio.pdf.page.outline-title-required": I, "stdio.pdf.page.output-intent-condition-required": I,
    "stdio.pdf.page.page-entry-key-required": I, "stdio.pdf.page.pattern-id-required": I,
    "stdio.pdf.page.property-list-required": I, "stdio.pdf.page.resource-owner-unknown": I,
    "stdio.pdf.page.separation-name-required": I, "stdio.pdf.page.trailer-key-required": I,
    "stdio.pdf.set-page.stale-target": P, "stdio.png.pixel-index-out-of-range": P,
    "stdio.png.pixel-region.bounds-overflow": I, "stdio.png.pixel-region.empty": I,
    "stdio.png.pixel-region.out-of-bounds": I, "stdio.png.pixel-region.patch-count-overflow": I,
    "stdio.png.pixel-region.raster-too-large": P, "stdio.pptx.set-page.stale-shape": P,
    "stdio.pptx.set-page.stale-slide": P, "stdio.pptx.set-page.unsupported-target": P,
    "stdio.semio.brep.set-vertex.command-mismatch": D, "stdio.semio.mesh.set-vertex.command-mismatch": D,
    "stdio.tiff.index-out-of-range": P, "stdio.tsv.cell-stale": I, "stdio.tsv.column-stale": I, "stdio.tsv.row-stale": I,
    "stdio.txt.replace-text-unrepresentable": I, "stdio.wav.audio-action": I, "stdio.wav.channel-stale": I,
    "stdio.wav.format-copy": P, "stdio.wav.format-data-mismatch": P, "stdio.wav.frame-stale": I,
    "stdio.wav.sample-index-out-of-range": P, "stdio.wav.sample-stale": I, "stdio.wav.zero-channels": P,
    "stdio.xlsx.cell-stale": P, "stdio.xlsx.sheet-stale": P, "stdio.zip.checkpoint-capacity": D,
    "stdio.zip.checkpoint-context": D, "stdio.zip.checkpoint-invalid": D, "stdio.zip.command-mismatch": D,
    "stdio.zip.name-required": I, "stdio.zip.work-complete": D,
}
lines = WORK.read_text().split("\n")
seen, moves = set(), Counter()
for index, line in enumerate(lines):
    if not line.startswith("{"):
        continue
    entry = json.loads(line.rstrip(","))
    target = REVIEW.get(entry["code"])
    if target is None:
        continue
    seen.add(entry["code"])
    if target == entry["class"]:
        print(f"unchanged {entry['code']}")
        continue
    needle = f'"class": "{entry["class"]}"'
    assert line.count(needle) == 1, entry["code"]
    lines[index] = line.replace(needle, f'"class": "{target}"')
    assert json.loads(lines[index].rstrip(","))["class"] == target
    moves[(entry["class"], target)] += 1
missing = set(REVIEW) - seen
assert not missing, missing
print(f"changed {sum(moves.values())}")
for (source, target), count in sorted(moves.items()):
    print(f"  {source} -> {target}: {count}")
if "--dry" not in sys.argv:
    WORK.write_text("\n".join(lines))
    after = json.loads(WORK.read_text())
    print("classes after:", dict(Counter(entry["class"] for entry in after)))
