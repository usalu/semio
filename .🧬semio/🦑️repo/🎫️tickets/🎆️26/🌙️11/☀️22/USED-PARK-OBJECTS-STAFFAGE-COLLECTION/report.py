"""📝 Writes `reports/<slug>.md` for started used-park-objects assets from what can be measured now: the candidate document, the last lines of the full proof and of the check CLI, the publish run and the finisher's review notes.

Usage: python3 report.py [<slug> ...]   (no slug: every candidate under assets/)
Inputs: assets/<slug>/🔣️.json, 🗑️generated/prove/<slug>.{proof,check}.txt, 🗑️generated/publish/<slug>.txt, review-notes.json, assign.md.
The authors of the interrupted wave never wrote their reports; these reports say so and contain no claim that was not measured or read in the finishing session.
"""
import json
import os
import sys

K = os.path.dirname(os.path.abspath(__file__))
NOTES = json.load(open(os.path.join(K, "review-notes.json")))
ROWS = {l.split("|")[2].strip(): (l.split("|")[1].strip(), l.split("|")[5].strip()) for l in open(os.path.join(K, "assign.md")) if l.startswith("| ") and l[2:4].isdigit()}


def last(path, n=1):
    """📎 The last lines of a tool output, or a plain statement that it is missing."""
    if not os.path.exists(path):
        return ["(not run)"]
    lines = [l.rstrip() for l in open(path) if l.strip()]
    return lines[-n:] if lines else ["(empty)"]


def joint(b):
    """🔩 A joint as one phrase."""
    j = b["joint"]
    lim = ""
    if "minDeg" in j:
        lim = f" {j['minDeg']}..{j['maxDeg']} deg"
    if "minM" in j:
        lim = f" {j['minM']}..{j['maxM']} m"
    return j["type"] + lim


def report(slug):
    """📄 One report as markdown text."""
    a = json.load(open(os.path.join(K, "assets", slug, "🔣️.json")))["asset"]
    n = NOTES[slug]
    row, subject = ROWS[slug]
    d = a["dimensions"]
    out = [f"# {a['title']['en']} / {a['title']['de']}", ""]
    out += [f"Row {row} of `assign.md`, category `{a['category']}`, asset id `{slug}`. Subject of the assignment: {subject}.", ""]
    out += ["This report was written by the finisher on 2026-11-22 from the candidate document `assets/" + slug + "/🔣️.json` and from tool runs of the finishing session. The author's session was cut off at about 17:30 before any report was written; nothing below is taken from an author's memory, and the model-against-reference check is limited to what the document itself states.", ""]
    out += ["## State", "", f"- Before (20:23): {n['before']}", f"- After: {n['after']}", ""]
    if n.get("changed"):
        out += ["### Changed in the finishing session", ""] + [f"- {c}" for c in n["changed"]] + [""]
    out += ["## Reference data the model states", "", a["provenance"].get("notes", a["summary"])["en"], "", "| reference | citation |", "|---|---|"]
    out += [f"| `{r['id']}` | {r['citation']} |" for r in a["provenance"]["references"]] + [""]
    bone_kg = sum(b.get("mass", {}).get("kg", 0) for b in a["rig"]["bones"])
    out += ["## Model measured against its own statements", "", "| quantity | value |", "|---|---|"]
    out += [f"| height / width / depth | {d['heightM']} / {d['widthM']} / {d.get('depthM', '–')} m |", f"| `dimensions.massKg` | {d['massKg']} kg |", f"| sum of the bone masses | {bone_kg:.3f} kg ({'equal' if abs(bone_kg - d['massKg']) < 0.06 else 'difference %.3f kg' % (bone_kg - d['massKg'])}) |"]
    out += [f"| rated power (manifest `powerW`) | {n['powerW']} W |", ""]
    out += ["An independent check of the stated dimensions against the cited standards and catalogue data (the 3 % rule of the brief) was not possible without the author's reference table; the numbers above are the document's own.", ""]
    out += ["## Rig", "", "| bone | parent | joint | mass kg |", "|---|---|---|---|"]
    out += [f"| `{b['id']}` | {b.get('parent') or '–'} | {joint(b)} | {b.get('mass', {}).get('kg', '–')} |" for b in a["rig"]["bones"]] + [""]
    out += ["## Poses", "", "| pose | key | moved bones | contacts | loads |", "|---|---|---|---|---|"]
    for p in a["poses"]:
        loads = ", ".join(f"`{l['id']}` {l.get('fyN', 0):g} N" for l in p["loads"]) or "–"
        out.append(f"| `{p['id']}` | {'yes' if p.get('key') else ''} | {', '.join(p['bones']) or '–'} | {len(p['contacts'])} | {loads} |")
    out += ["", "## Clips", "", "| clip | duration s | keys | loop |", "|---|---|---|---|"]
    out += [f"| `{c['id']}` | {c['durationS']} | {' → '.join(dict.fromkeys(k['pose'] for k in c['keys']))} | {'yes' if c.get('loop') else 'no'} |" for c in a["clips"]] + [""]
    out += ["## Scales", "", "| scale | status | body | thermal | structure | shapes elevation / front / plan |", "|---|---|---|---|---|---|"]
    for s in ("50", "100", "200", "500"):
        r = a["representations"][s]
        if r["status"] != "authored":
            out.append(f"| 1:{s} | {r['status']}: {r.get('reason', {}).get('en', '')} | – | – | – | – |")
            continue
        v = a["volumes"][s]
        g = r["geometry"]
        out.append(f"| 1:{s} | authored | {len(v['body'])} | {len(v['thermal'])} | {len(v['structure'])} | {len(g['elevation'])} / {len(g['front'])} / {len(g['plan'])} |")
    out.append("")
    for s in ("100", "200", "500"):
        r = a["representations"][s]
        if r["status"] == "authored":
            dec = r["decisions"]
            merged = dec.get("merged", {}).get("en") or dec.get("omitted", {}).get("en") if isinstance(dec, dict) else None
            if merged:
                out += [f"- 1:{s} merged or left out: {merged}"]
    out += ["", "## Thermal regions", "", "| region | material | area m² | model |", "|---|---|---|---|"]
    out += [f"| `{r['id']}` | {r.get('material', '–')} | {r.get('areaM2', '–')} | {r.get('model', '–')} |" for r in a["thermal"]["regions"]] + [""]
    out += ["## Structure assumptions", ""] + [f"- {x['en']}" for x in a["structure"]["assumptions"]] + [""]
    if n.get("reactions"):
        out += ["### Measured reactions (`reactions.ts`, the product's solver)", "", "```"] + n["reactions"] + ["```", ""]
    out += ["## Proof (this session)", "", "```"]
    out += last(os.path.join(K, "🗑️generated", "prove", f"{slug}.proof.txt"))
    out += last(os.path.join(K, "🗑️generated", "prove", f"{slug}.check.txt"), 5)
    published = os.path.join(K, "🗑️generated", "publish", f"{slug}.txt")
    out += last(published, 2) if os.path.exists(published) else ["(not re-published in the finishing session: the document is unchanged and the row of the wave stands)"]
    out += ["```", "", f"`verify.ts`: {n['verify']}", ""]
    out += ["## Sheets opened", "", n["sheets"], ""]
    out += ["## What remains imperfect", ""] + [f"- {x}" for x in n["imperfect"]] + [""]
    return "\n".join(out)


os.makedirs(os.path.join(K, "reports"), exist_ok=True)
for slug in sys.argv[1:] or sorted(os.listdir(os.path.join(K, "assets"))):
    with open(os.path.join(K, "reports", f"{slug}.md"), "w", encoding="utf8") as f:
        f.write(report(slug))
    print(f"[DEBUG] wrote reports/{slug}.md")
