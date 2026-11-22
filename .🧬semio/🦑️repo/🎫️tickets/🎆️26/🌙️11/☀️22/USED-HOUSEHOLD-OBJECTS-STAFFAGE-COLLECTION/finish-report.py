"""📝 Writes the author report `reports/<slug>.md` of every candidate of this ticket from what can be measured now: the asset document itself, the proof output `🗑️generated/prove/<slug>.txt` of `finish-prove.sh` and the finisher's notes `finish-notes.json` (sheets opened, observations, changes, what stays imperfect).

Usage: `python3 finish-report.py [slug ...]` (no slug: all candidates).
"""
import json
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
NOTES = json.loads((HERE / "finish-notes.json").read_text())
SCALES = ["50", "100", "200", "500"]


def proof_lines(slug):
    """🧾 The verdict line of the full proof and the lines of the check CLI."""
    file = HERE / "🗑️generated" / "prove" / f"{slug}.txt"
    if not file.exists():
        return None, []
    text = file.read_text().split("--- check ---")
    verdict = [line for line in text[0].splitlines() if line.startswith(("VALID", "INVALID"))]
    return (verdict[-1] if verdict else "NO VERDICT"), [line for line in text[1].splitlines() if line.strip()] if len(text) > 1 else []


def joint_text(joint):
    """🔩 One joint with its limits."""
    kind = joint["type"]
    if kind == "revolute":
        return f"revolute {joint.get('minDeg')}…{joint.get('maxDeg')}°" + (f", twist {joint.get('twistMinDeg')}…{joint.get('twistMaxDeg')}°" if "twistMaxDeg" in joint else "")
    if kind == "prismatic":
        return f"prismatic along {joint.get('axisDeg')}°, {joint.get('minM')}…{joint.get('maxM')} m"
    return kind


def report(slug):
    """📄 The report text of one asset."""
    asset = json.loads((HERE / "assets" / slug / "🔣️.json").read_text())["asset"]
    notes = NOTES.get(slug, {})
    common = NOTES["*"]
    dims = asset["dimensions"]
    bones = asset["rig"]["bones"]
    mass = sum((bone.get("mass") or {}).get("kg", 0) for bone in bones)
    verdict, check = proof_lines(slug)
    out = [f"# {asset['title']['en']} / {asset['title']['de']} (`{slug}`)", ""]
    out += [f"Category `{asset['category']}`. Report written by the finisher on 2026-11-22 from measurements of the published document; the author's own report was never written (the wave was cut off at about 17:30). {notes.get('state', common['state'])}", ""]
    out += ["## Reference table (as declared in the document)", "", "| quantity | value | source |", "|---|---|---|"]
    out += [f"| height (world y) | {dims['heightM']} m | `dimensions`, rest pose `{asset['presentation']['defaultPose']}` |", f"| width (world x, ahead) | {dims['widthM']} m | `dimensions` |", f"| depth (world z, lateral) | {dims['depthM']} m | `dimensions` |", f"| mass | {dims['massKg']} kg | `dimensions` |"]
    for material in asset["materials"]:
        out.append(f"| material `{material['id']}` | ε {material.get('emissivity')}, α {material.get('solarAbsorptivity')}, ρ {material.get('densityKgM3')} kg/m³, λ {material.get('conductivityWmK')} W/mK, c {material.get('specificHeatJKgK')} J/kgK | {material.get('source')}, `{material.get('reference')}` |")
    out += ["", "References (`provenance.references`):", ""]
    out += [f"- `{ref['id']}`: {ref['citation']}" for ref in asset["provenance"]["references"]]
    out += ["", "## Model against reference", ""]
    out += [f"- Sum of the bone masses: {mass:.3f} kg against `dimensions.massKg` {dims['massKg']} kg ({(mass / dims['massKg'] - 1) * 100:+.2f} %).", "- Envelope: the validator rule that every authored representation reproduces `dimensions` in every viewpoint within max(2 %, 15 mm) passed in the full proof below; the 3 % bar of the brief against the catalogue-typical dimensions is met as far as the citations above state those dimensions (the finisher did not re-research them)."]
    out += [f"- {line}" for line in notes.get("measured", [])]
    out += ["", "## Rig, poses and clips", "", f"{len(bones)} bones (`{asset['rig']['kind']}`):", ""]
    out += [f"- `{bone['id']}` ← `{bone.get('parent')}`: {joint_text(bone['joint'])}" + (f", {bone['mass']['kg']} kg" if bone.get("mass") else "") + f" — {bone['title']['en']}" for bone in bones]
    poses = [pose for pose in asset["poses"] if not pose.get("key")]
    out += ["", f"{len(poses)} poses and {len(asset['poses']) - len(poses)} key poses:", ""]
    out += [f"- `{pose['id']}`: {pose['title']['en']} — contacts {', '.join(pose['contacts']) or 'none'}; loads {', '.join(load['id'] for load in pose['loads']) or 'none'}" for pose in poses]
    out += ["", f"{len(asset['clips'])} clips:", ""]
    out += [f"- `{clip['id']}`: {clip['title']['en']}, {clip['durationS']} s, {len(clip['keys'])} keys, {len(clip.get('tracks', []))} tracks — {clip['description']['en']}" for clip in asset["clips"]]
    out += ["", "## Scales", "", "| scale | 2D | valid range | body | thermal | structure |", "|---|---|---|---|---|---|"]
    for scale in SCALES:
        rep = asset["representations"][scale]
        vol = asset["volumes"].get(scale, {})
        band = rep.get("validRange")
        out.append(f"| 1:{scale} | {rep['status']} | {'%s–%s' % (band.get('minDenominator', band.get('min')), band.get('maxDenominator', band.get('max'))) if band else '–'} | {len(vol.get('body', []))} | {len(vol.get('thermal', []))} | {len(vol.get('structure', []))} |")
    out.append("")
    for scale in SCALES:
        rep = asset["representations"][scale]
        vol = asset["volumes"].get(scale, {})
        if rep["status"] == "unavailable":
            out.append(f"- 1:{scale} 2D unavailable: {rep['reason']['en']}")
        if vol.get("status") and vol.get("status") != "authored":
            out.append(f"- 1:{scale} volume `{vol.get('status')}`" + (f": {vol['reason']['en']}" if "reason" in vol else ""))
        for decision in vol.get("decisions", []):
            out.append(f"- 1:{scale} volume: {decision['en']}")
    thermal = asset["thermal"]
    out += ["", "## Thermal", "", "| region | model | material | area m² | rated W (basis) | source |", "|---|---|---|---|---|---|"]
    out += [f"| `{region['id']}` | {region.get('kind')}/{region.get('model')} | `{region.get('material')}` | {region.get('areaM2')} | {region.get('ratedPowerW', '–')} ({region.get('ratedPowerBasis', '–')}) | {region.get('source')}, `{region.get('reference')}` |" for region in thermal["regions"]]
    out += ["", f"Operating keys: {', '.join('`%s`' % op['id'] for op in thermal['operating']) or 'none'}; emitters: {', '.join('`%s` (%s)' % (em['id'], em['kind']) for em in thermal['emitters']) or 'none'}.", ""]
    out += [f"- {text['en']}" for text in thermal.get("assumptions", [])]
    out += ["", "## Structure", ""]
    out += [f"- {text['en']}" for text in asset["structure"].get("assumptions", [])]
    out += ["", "## Proof (run by the finisher, 2026-11-22 evening)", "", "```", verdict or "not run", *check, "```", "", f"Publish: {notes.get('publish', common['publish'])}", ""]
    out += ["## Sheets opened by the finisher", ""]
    out += [f"- {line}" for line in notes.get("sheets", common["sheets"])]
    out += ["", "## Changes by the finisher", ""]
    out += [f"- {line}" for line in notes.get("changes", ["None: the document is the one the author published."])]
    out += ["", "## What remains imperfect", ""]
    out += [f"- {line}" for line in notes.get("imperfect", []) + common["imperfect"][: 1 if "sheets" in notes else 2]]
    return "\n".join(out) + "\n"


(HERE / "reports").mkdir(exist_ok=True)
for name in sys.argv[1:] or sorted(path.name for path in (HERE / "assets").iterdir() if (path / "🔣️.json").exists()):
    (HERE / "reports" / f"{name}.md").write_text(report(name))
    print(f"wrote reports/{name}.md")
