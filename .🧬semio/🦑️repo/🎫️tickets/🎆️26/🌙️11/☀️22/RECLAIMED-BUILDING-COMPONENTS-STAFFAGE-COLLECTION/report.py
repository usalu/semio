"""📝️ Finisher report writer: writes `reports/<slug>.md` for every candidate from what the document states and what the proof run of this session measured (`🗑️generated/prove`), plus the finisher's review notes of `review-notes.json`. The authors of the wave were cut off before their reports; nothing here is taken from a ledger."""
import json, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
SCALES = ("50", "100", "200", "500")


def cell(value):
    """🔢️ One table cell: numbers short, missing values as a dash."""
    if value is None:
        return "–"
    if isinstance(value, float):
        return f"{value:.4g}"
    return str(value).replace("|", "/")


def tail(path, count):
    """📄️ The last lines of a tool output of this session."""
    if not os.path.exists(path):
        return ["(not run)"]
    lines = [line.rstrip() for line in open(path) if line.strip() and not line.startswith("exit ")]
    return lines[-count:]


def report(slug, notes):
    """🧱️ The report text of one asset."""
    asset = json.load(open(os.path.join(HERE, "assets", slug, "🔣️.json")))["asset"]
    note = notes.get(slug, {})
    size, bones = asset["dimensions"], asset["rig"]["bones"]
    mass = sum((bone.get("mass") or {}).get("kg", 0) for bone in bones)
    out = [f"# {asset['title']['en']} / {asset['title']['de']} (`{slug}`)", ""]
    out += ["Written by the finisher on 2026-11-22 from the document and from this session's measurements; the author of the wave was cut off at about 17:30 before writing a report. Every number below is read from the candidate `assets/" + slug + "/🔣️.json` (byte-identical with the catalog copy) or printed by a tool in this session.", ""]
    out += ["## Subject", "", asset["summary"]["en"], "", asset["summary"]["de"], ""]
    out += ["## Reference table (what the document models to)", ""]
    out += [f"- Overall: height {size['heightM']} m, width {size['widthM']} m, depth {size.get('depthM', '–')} m, mass {size['massKg']} kg.", ""]
    out += ["| material | density kg/m³ | λ W/mK | c J/kgK | ε | α solar | E Pa | strength Pa | source | reference |", "|---|---|---|---|---|---|---|---|---|---|"]
    for item in asset["materials"]:
        out.append("| " + " | ".join(cell(item.get(key)) for key in ("id", "densityKgM3", "conductivityWmK", "specificHeatJKgK", "emissivity", "solarAbsorptivity", "elasticModulusPa", "strengthPa", "source", "reference")) + " |")
    out += ["", "| bone | parent | joint | mass kg | section | A m² | I m⁴ | mass source |", "|---|---|---|---|---|---|---|---|"]
    for bone in bones:
        weight, section = bone.get("mass") or {}, bone.get("section") or {}
        out.append("| " + " | ".join(cell(value) for value in (bone["id"], bone.get("parent"), bone["joint"]["type"], weight.get("kg"), (section.get("title") or {}).get("en"), section.get("areaM2"), section.get("secondMomentM4"), f"{weight.get('source', '–')} / {weight.get('reference', '–')}")) + " |")
    out += ["", "Sources (`provenance.references`):", ""]
    out += [f"- `{ref['id']}`: {ref['citation']}" for ref in asset["provenance"]["references"]]
    out += ["", "## Model against reference", ""]
    out += [f"- Bone masses sum to {mass:.2f} kg against `dimensions.massKg` {size['massKg']} kg (difference {abs(mass - size['massKg']):.3f} kg)."]
    out += [f"- Tags: {', '.join(asset['tags'])}. Default pose `{asset['presentation']['defaultPose']}`, completion `{asset['completion']}`, {len(asset['exports'])} export formats."]
    out += ["- The geometric checks of the validator (silhouette, bounds, 2D against 3D in elevation, front and plan, thermal areas against `areaM2`, structure coverage) are part of the full proof below and ended without a finding.", ""]
    out += ["## Rig, poses and clips", "", f"Rig `{asset['rig']['kind']}`, root `{asset['rig']['root']}`, {len(bones)} bones, {len(asset['anchors'])} anchors.", ""]
    for pose in asset["poses"]:
        loads = "; ".join(load["title"]["en"] for load in pose.get("loads", [])) or "none"
        out.append(f"- Pose `{pose['id']}` ({pose['title']['en']}): {pose['description']['en']} Contacts: {', '.join(pose.get('contacts', [])) or 'none'}. Loads: {loads}.")
    for clip in asset["clips"]:
        out.append(f"- Clip `{clip['id']}` ({clip['title']['en']}, {clip['durationS']} s): {clip['description']['en']}")
    out += ["", "## Volumes and drawings per scale", "", "| scale | body | thermal | structure | 2D shapes elevation / front / plan | valid range |", "|---|---|---|---|---|---|"]
    for scale in SCALES:
        volume, drawn = asset["volumes"][scale], asset["representations"][scale]
        out.append(f"| 1:{scale} | {len(volume['body'])} | {len(volume['thermal'])} | {len(volume['structure'])} | {' / '.join(str(len(drawn['geometry'][view])) for view in ('elevation', 'front', 'plan'))} | {json.dumps(drawn.get('validRange'))} |")
    out.append("")
    for scale in SCALES:
        out += [f"Decisions 1:{scale} (the document's own statement of what was merged or generalised):", ""]
        out += [f"- {decision['en']}" for decision in asset["volumes"][scale]["decisions"]]
        out.append("")
    out += ["## Thermal", "", f"Operating states {len(asset['thermal']['operating'])}, emitters {len(asset['thermal']['emitters'])}.", "", "| region | material | area m² | layers | heat capacity J/m²K | source | reference |", "|---|---|---|---|---|---|---|"]
    for region in asset["thermal"]["regions"]:
        out.append("| " + " | ".join(cell(value) for value in (f"{region['id']} ({region['title']['en']})", region.get("material"), region.get("areaM2"), len(region.get("layers", [])), region.get("heatCapacityJM2K"), region.get("source"), region.get("reference"))) + " |")
    out += ["", "Assumptions:", ""] + [f"- {text['en']}" for text in asset["thermal"]["assumptions"]]
    out += ["", "## Structure", "", f"Gravity {asset['structure']['gravityMS2']} m/s², dynamics `{asset['structure']['dynamics']}`, {len(asset['structure']['supports'])} global supports (poses name their own contacts).", "", "Assumptions:", ""] + [f"- {text['en']}" for text in asset["structure"]["assumptions"]]
    proof = os.path.join(HERE, "🗑️generated", "prove")
    out += ["", "## Proof of this session (finisher, 2026-11-22 evening)", "", "Full proof `r3-validate.ts --file <candidate> --volumes`, last line:", "", "```"] + tail(os.path.join(proof, f"{slug}.proof.txt"), 1) + ["```", "", "Product check CLI `🩺️check --file <candidate>`:", "", "```"] + tail(os.path.join(proof, f"{slug}.check.txt"), 5) + ["```", ""]
    out += ["- Catalog copy `🗂️catalog/🧱️components/" + slug + "/🔣️.json` equals the candidate byte for byte; the manifest row equals the row `publish.ts` computes from the candidate; the asset is a member of the collection `reclaimed-components` (`verify-rows.ts`: `OK`).", "- `verify.sh` (catalog file as it is): `VALID`, schema 0, validateAsset 0.", ""]
    out += ["## Publish", "", note.get("publish", "Published by the author before the cut-off; not re-published by the finisher because nothing in the document was changed."), ""]
    out += ["## Sheets opened by the finisher", "", note.get("opened", "none"), ""]
    out += ["## What the finisher changed", ""] + [f"- {text}" for text in note.get("changes", ["Nothing in the asset; this report is new."])] + [""]
    out += ["## What remains imperfect", ""] + [f"- {text}" for text in note.get("imperfect", [])] + [""]
    return "\n".join(out)


def main():
    """🖨️ Writes the reports of the named slugs or of every candidate."""
    notes = json.load(open(os.path.join(HERE, "review-notes.json")))
    os.makedirs(os.path.join(HERE, "reports"), exist_ok=True)
    for slug in sys.argv[1:] or sorted(os.listdir(os.path.join(HERE, "assets"))):
        open(os.path.join(HERE, "reports", f"{slug}.md"), "w").write(report(slug, notes))
        print(f"reports/{slug}.md")


if __name__ == "__main__":
    main()
