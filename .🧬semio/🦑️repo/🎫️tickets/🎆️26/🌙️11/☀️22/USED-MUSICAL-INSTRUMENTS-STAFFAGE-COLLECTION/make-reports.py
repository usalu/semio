"""📝️ Writes the author report `reports/<slug>.md` of every started asset of this ticket from what is measured now.

Inputs: the candidate `assets/<slug>/🔣️.json`, the measurements of `measure.ts` (`🗑️generated/measure/<slug>.json`), the raw proof output of `prove.sh` (`🗑️generated/prove/<slug>.{proof,check}.txt`), the publish line kept in `🗑️generated/publish/<slug>.txt` when the asset was re-published, and the finisher's own notes `review.json` (sheets opened, findings, changes, what stays imperfect). Every number in a report is read from these files; nothing is taken from the ledger of the interrupted wave.

Usage: `python3 make-reports.py [slug …]` (all assets without arguments). Run it before `🗑️generated` is deleted.
"""
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROWS = {}
for line in open(os.path.join(HERE, "assign.md"), encoding="utf-8"):
    cells = [cell.strip() for cell in line.split("|")]
    if len(cells) > 5 and cells[1].isdigit():
        ROWS[cells[2]] = (cells[1], cells[5])


def text(value):
    """🔤️ English text of a bilingual value."""
    return value["en"] if isinstance(value, dict) else str(value)


def last(path, start):
    """🔚️ Lines of a raw tool output that start with one of the given marks."""
    if not os.path.exists(path):
        return ["(not run)"]
    return [line.rstrip() for line in open(path, encoding="utf-8") if line.lstrip().startswith(start)] or ["(no verdict line)"]


def joint(bone):
    """🔩️ One line for a moving joint with its limits."""
    j = bone["joint"]
    limits = ", ".join(f"{key} {value}" for key, value in j.items() if key != "type")
    return f"`{bone['id']}` {j['type']} ({limits})"


def report(slug, notes):
    """📄️ Markdown report of one asset."""
    asset = json.load(open(os.path.join(HERE, "assets", slug, "🔣️.json"), encoding="utf-8"))["asset"]
    measured = json.load(open(os.path.join(HERE, "🗑️generated", "measure", f"{slug}.json"), encoding="utf-8"))
    note = notes[slug]
    row, subject = ROWS[slug]
    dims = asset["dimensions"]
    out = [f"# {text(asset['title'])} — `{slug}`", ""]
    out += [f"Row {row} of `assign.md`, category `{asset['category']}`. Written by the finisher on 2026-11-22 from measurements of this session; the original author left no report (the wave was cut off at about 17:30).", ""]
    out += ["## State", "", f"- Before (20:23): {note['before']}", f"- After: {note['after']}", ""]
    out += ["## Subject and reference", "", f"Assignment: {subject}", "", f"Summary of the document: {text(asset['summary'])}", ""]
    if "notes" in asset["provenance"]:
        out += [f"Reference data the author modelled to (`provenance.notes`): {text(asset['provenance']['notes'])}", ""]
    out += ["References (`provenance.references`):", ""]
    out += [f"- `{ref['id']}`: {ref['citation']}" for ref in asset["provenance"]["references"]]
    out += ["", "Nothing is `measured` on a real instrument; the document tags its parameters `literature`, `authored-estimate` or `illustrative`.", ""]
    out += ["## Model measured against the document", "", f"`dimensions`: height {dims['heightM']} m, width {dims['widthM']} m, depth {dims['depthM']} m, mass {dims['massKg']} kg. Bone masses sum to {round(sum(bone.get('mass', {}).get('kg', 0) for bone in asset['rig']['bones']), 3)} kg.", ""]
    out += ["Bounding box of the 1:50 body solids per pose, as the sheet tool places them (x along the elevation, y up, z towards the viewer of the elevation):", "", "| pose | x m | y m | z m | lowest y | solids body / thermal / structure |", "|---|---|---|---|---|---|"]
    for pose, box in measured["poses"].items():
        out.append(f"| `{pose}` | {box['size'][0]} | {box['size'][1]} | {box['size'][2]} | {box['min'][1]} | {box['counts']['body']} / {box['counts']['thermal']} / {box['counts']['structure']} |")
    default = measured["poses"][asset["presentation"]["defaultPose"]]
    deviation = [abs(default["size"][0] - dims["widthM"]) / dims["widthM"], abs(default["size"][1] - dims["heightM"]) / dims["heightM"], abs(default["size"][2] - dims["depthM"]) / dims["depthM"]]
    out += ["", f"Default pose `{asset['presentation']['defaultPose']}` against `dimensions`: width {deviation[0] * 100:.1f} %, height {deviation[1] * 100:.1f} %, depth {deviation[2] * 100:.1f} % off (the brief allows 3 % at 1:50).", ""]
    bones = asset["rig"]["bones"]
    moving = [bone for bone in bones if bone["joint"]["type"] != "fixed"]
    out += ["## Rig, poses and clips", "", f"{len(bones)} bones, {len(moving)} of them moving:", ""]
    out += [f"- {joint(bone)}" for bone in moving]
    out += ["", "Poses:", ""]
    out += [f"- `{pose['id']}`{' (key pose of a clip)' if pose.get('key') else ''}: {text(pose['title'])}" for pose in asset["poses"]]
    out += ["", "Clips:", ""]
    out += [f"- `{clip['id']}` ({clip['durationS']} s, {len(clip['keys'])} keys: {' → '.join(key['pose'] for key in clip['keys'])}): {text(clip['title'])}" for clip in asset["clips"]]
    out += ["", "## Scales", "", "| scale | 2D representation | solids body / thermal / structure (expanded) | box x × y × z m |", "|---|---|---|---|"]
    for scale in ["50", "100", "200", "500"]:
        rep = asset["representations"][scale]
        if rep["status"] == "authored":
            box = measured["scales"][scale]
            out.append(f"| 1:{scale} | authored, valid 1:{rep['validRange']['minDenominator']}–1:{rep['validRange']['maxDenominator']} | {box['counts']['body']} / {box['counts']['thermal']} / {box['counts']['structure']} | {box['size'][0]} × {box['size'][1]} × {box['size'][2]} |")
        else:
            out.append(f"| 1:{scale} | unavailable: {text(rep['reason'])} | – | – |")
    out.append("")
    for scale in ["50", "100", "200", "500"]:
        volume = asset["volumes"][scale]
        if volume.get("status") == "authored":
            decisions = volume["decisions"]
            first = decisions[0] if isinstance(decisions, list) else next(iter(decisions.values()))
            out += [f"1:{scale} volume decision (first entry): {text(first)}", ""]
    thermal = asset["thermal"]
    out += ["## Thermal", "", "| region | material | area m² | model | rated W (basis) | source |", "|---|---|---|---|---|---|"]
    out += [f"| `{region['id']}` | `{region.get('material', '–')}` | {region.get('areaM2', '–')} | {region.get('model', '–')} | {region.get('ratedPowerW', '–')} ({region.get('ratedPowerBasis', '–')}) | {region.get('source', '–')} / {region.get('reference', '–')} |" for region in thermal["regions"]]
    out.append("")
    if thermal["operating"]:
        out += ["Operating keys: " + "; ".join(f"`{key['id']}` ({', '.join(text(level['title']) for level in key['levels'])})" for key in thermal["operating"]) + ".", ""]
    else:
        out += ["No operating key: the asset is acoustic and passive.", ""]
    if thermal["emitters"]:
        out += ["Emitters: " + "; ".join(f"`{emitter['id']}` ({emitter['kind']} at anchor `{emitter.get('anchor', '–')}`: {text(emitter['title'])})" for emitter in thermal["emitters"]) + ".", ""]
    out += [f"Manifest `powerW` (computed by the product's `totalPowerW`): {note['powerW']} W.", "", "Assumptions (`thermal.assumptions`):", ""]
    out += [f"- {text(item)}" for item in thermal["assumptions"]]
    out += ["", "## Structure", "", "Anchors: " + ", ".join(f"`{anchor['id']}` ({anchor['kind']}{' ' + anchor['support'] if anchor.get('support') else ''})" for anchor in asset["anchors"]) + ".", "", "Assumptions (`structure.assumptions`):", ""]
    out += [f"- {text(item)}" for item in asset["structure"]["assumptions"]]
    out += ["", "## Proof (run to the end in this session)", "", "```"]
    out += last(os.path.join(HERE, "🗑️generated", "prove", f"{slug}.proof.txt"), ("VALID", "INVALID"))
    out += last(os.path.join(HERE, "🗑️generated", "prove", f"{slug}.check.txt"), ("✓", "✗", "1:"))
    out += [note["row"]]
    publish = os.path.join(HERE, "🗑️generated", "publish", f"{slug}.txt")
    if os.path.exists(publish):
        out += [line.rstrip() for line in open(publish, encoding="utf-8") if line.strip()]
    out += ["```", "", "## Sheets opened by the finisher", ""]
    out += [f"- {item}" for item in note["sheets"]]
    out += ["", "## Review findings and changes", ""]
    out += [f"- {item}" for item in note["findings"]] or ["- None."]
    out += ["", "## What remains imperfect", ""]
    out += [f"- {item}" for item in note["imperfect"]]
    out.append("")
    os.makedirs(os.path.join(HERE, "reports"), exist_ok=True)
    with open(os.path.join(HERE, "reports", f"{slug}.md"), "w", encoding="utf-8") as handle:
        handle.write("\n".join(out))
    print(f"[DEBUG] wrote reports/{slug}.md")


NOTES = json.load(open(os.path.join(HERE, "review.json"), encoding="utf-8"))
for name in sys.argv[1:] or sorted(NOTES):
    report(name, NOTES)
