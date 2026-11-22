"""📝️ Writes the worker report T/📝️<worker>.md of an interrupted worker from the evaluated catalog assets, the measured proof lines (proof.json) and the finisher's notes (notes.json): python3 report.py HW-02 [HW-03 …]"""
import json
import sys

T = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE"
CAT = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans"
HERE = T + "/🧰️scripts/FIN-A"
NOTES = json.load(open(HERE + "/notes.json", encoding="utf-8"))
PROOF = json.load(open(HERE + "/proof.json", encoding="utf-8"))


def asset_section(slug, note):
    a = json.load(open(f"{CAT}/{slug}/🔣️.json", encoding="utf-8"))["asset"]
    row = json.load(open(f"{T}/📥️manifest-rows/{slug}.json", encoding="utf-8"))
    body, d, proof = a["thermal"]["body"], a["dimensions"], PROOF[slug]
    out = [f"## {slug}", "", f"**{a['title']['en']} / {a['title']['de']}.** {a['summary']['en']}", ""]
    out.append(f"- **Person**: body {body['massKg']} kg, DuBois skin area {body['skinAreaM2']} m², basal metabolism {body['basalMetabolismW']} W (`{body.get('reference', body.get('source'))}`); figure with everything it carries {d['massKg']} kg; drawn extent {d['heightM']} m high, {d['widthM']} m long, {d['depthM']} m wide; base activity {a['thermal'].get('baseMet')} met, environment {a['thermal']['environment']['airTempC']} °C air.")
    notes = a["provenance"].get("notes")
    notes = notes if isinstance(notes, list) else [notes]
    out.append(f"- **Derivation and sources**: {' '.join(n['en'] for n in notes if n)} References: {', '.join('`' + r['id'] + '`' for r in a['provenance'].get('references', []))}.")
    joints = {}
    for b in a["rig"]["bones"]:
        joints[b["joint"]["type"]] = joints.get(b["joint"]["type"], 0) + 1
    out.append(f"- **Rig**: {len(a['rig']['bones'])} bones ({', '.join(f'{n} {k}' for k, n in sorted(joints.items()))}), root `{a['rig'].get('root', a['rig']['bones'][0]['id'])}`, {sum(1 for b in a['rig']['bones'] if b.get('mass'))} segment masses, {len(a['anchors'])} anchors, {len(a['materials'])} materials, {len(a['thermal']['regions'])} thermal regions ({', '.join(r['id'] for r in a['thermal']['regions'])}).")
    out.append("- **Poses** (activity, breathing pattern): " + "; ".join(f"`{p['id']}` {p['thermal'].get('activityMet', '–')} met, {p['thermal'].get('breathing', '–')}" for p in a["poses"] if not p.get("key")) + (f"; {sum(1 for p in a['poses'] if p.get('key'))} key poses for the clips" if any(p.get("key") for p in a["poses"]) else "") + f". Default `{a['presentation']['defaultPose']}`.")
    out.append("- **Clips**: " + "; ".join(f"`{c['id']}` {c['durationS']} s, {'loop' if c.get('loop') else 'once'}, root motion {c.get('rootMotion', 'none')}" for c in a["clips"]) + f". Default `{a['presentation']['defaultClip']}`.")
    rated = [(r["id"], r["ratedPowerW"], r.get("ratedPowerBasis")) for r in a["thermal"]["regions"] if r.get("ratedPowerW") is not None]
    out.append(f"- **Power**: {row['powerW']} W in total" + (" (nameplates: " + ", ".join(f"`{i}` {w} W {b}" for i, w, b in rated) + "; the rest is the metabolic heat of the default pose)." if rated else " (metabolic heat of the default pose, no nameplate)."))
    out.append("- **Representations and volumes** (shapes side/front/top; solids body/thermal/structure):")
    for sc in ("50", "100", "200", "500"):
        rep, vol = a["representations"][sc], a["volumes"][sc]
        g = rep["geometry"]
        dec = rep["decisions"]
        out.append(f"  - 1:{sc}: {len(g['elevation'])}/{len(g['front'])}/{len(g['plan'])} shapes, {len(vol['body'])}/{len(vol['thermal'])}/{len(vol['structure'])} solids. {dec['silhouette']['en']} Merged: {dec['merged']['en']} Omitted: {dec['omitted']['en']} Thin elements: {dec['thinElements']['en']}")
    out.append(f"- **Proof measured by the finisher on {proof['at']}**: `validateAsset` {proof['validate']} issues; check CLI `{proof['check']}`; " + "; ".join(f"1:{s} {v}" for s, v in proof["scales"].items()) + f". Silhouette and volume gate (`HW-03/measure.ts`): {proof['gate']}.")
    out.append(f"- **Sheets opened** (default pose, third-angle side, front and top view at 1:50, 1:100, 1:200 and 1:500, light solid): {note['seen']}")
    out.append(f"- **State found and what the finisher did**: {note['did']}")
    out.append(f"- **Remaining doubts**: {note['doubts']}")
    out.append("")
    return out


for worker in sys.argv[1:]:
    w = NOTES[worker]
    lines = [f"# 📝️ {worker}: {w['theme']} ({', '.join(w['assets'])})", "",
             f"Worker {worker} of the Staffage Human Collection Wave was cut off at about 17:30 on 2026-11-22 before it wrote a report. This report was written by the finisher of the workers HW-01 … HW-09 from the evaluated catalog assets, the worker's generators in `🧰️scripts/{worker}/` and the finisher's own measurements (tools in `🧰️scripts/FIN-A/`). Manifest rows: `📥️manifest-rows/<id>.json`.", "",
             "## Method and generators", "", w["method"], ""]
    for slug in w["assets"]:
        lines += asset_section(slug, w["notes"][slug])
    if w.get("never"):
        lines += ["## Never started", "", w["never"], ""]
    lines += ["## Tests", "", w.get("tests", NOTES["tests"]), ""]
    open(f"{T}/📝️{worker}.md", "w", encoding="utf-8").write("\n".join(lines))
    print(worker, len(lines))
