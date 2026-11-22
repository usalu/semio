"""📇 Prints the measured facts of shipped PW-10 … PW-17 plants as a markdown fragment for the worker reports: dimensions, rig, poses, clips, thermal regions and emitters, structure, shapes and solids per scale, references, and the proof lines found in `🗑️generated/FIN-B/<id>.cli.txt` and `<id>.probe.txt`.

Usage: `python3 facts.py <id> [<id> ...]`. See `../../📋️brief.md`.
"""
import json, pathlib, sys

TICKET = pathlib.Path(__file__).resolve().parents[2]
VEG = TICKET.parents[6] / "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation"
PROOF = TICKET / "🗑️generated/FIN-B"

for slug in sys.argv[1:]:
    a = json.loads((VEG / slug / "🔣️.json").read_text("utf8"))["asset"]
    d, bones = a["dimensions"], a["rig"]["bones"]
    joints = {}
    for b in bones: joints[b.get("joint", {}).get("type", "none")] = joints.get(b.get("joint", {}).get("type", "none"), 0) + 1
    print(f"### `{slug}` ({a['title']['en']} / {a['title']['de']})\n")
    print(f"- Summary: {a['summary']['en']}")
    print(f"- Dimensions: height {d['heightM']} m, width {d['widthM']} m, depth {d['depthM']} m, mass {d['massKg']} kg; power 0 W (passive).")
    print(f"- Rig: {len(bones)} bones ({', '.join(f'{n} {t}' for t, n in sorted(joints.items()))}), {len(a['structure']['supports'])} support(s) ({', '.join(s if isinstance(s, str) else s.get('id', '?') for s in a['structure']['supports'])}), dynamics `{a['structure']['dynamics']}`; {len(a['materials'])} materials.")
    print(f"- Poses: {', '.join('`' + p['id'] + '`' for p in a['poses'])}; default `{a['presentation']['defaultPose']}`, default scale 1:{a['presentation']['defaultScale']}.")
    print(f"- Clips: {', '.join('`%s` (%s s%s)' % (c['id'], c['durationS'], ', looping' if c.get('loop') or c.get('looping') else '') for c in a['clips'])}.")
    print(f"- Thermal: {', '.join('`%s` %s %s m²' % (r['id'], r.get('kind'), r.get('areaM2')) for r in a['thermal']['regions'])}; emitters {', '.join('`' + e['id'] + '`' for e in a['thermal']['emitters'])}.")
    for s in ["50", "100", "200", "500"]:
        rep, vol = a["representations"][s], a["volumes"].get(s, {})
        shapes = "/".join(str(len(rep["geometry"][v])) for v in ["elevation", "front", "plan"]) + " shapes (side/front/plan)" if rep["status"] == "authored" else f"{rep['status']}: {rep.get('reason', {}).get('en', '')[:260]}"
        solids = f"body {len(vol['body'])}, thermal {len(vol['thermal'])}, structure {len(vol['structure'])} solids" if vol.get("status") == "authored" else f"volume {vol.get('status')}"
        print(f"- 1:{s}: {shapes}; {solids}.")
    print(f"- Symbols: {', '.join('`' + s['id'] + '`' for s in a.get('symbols', []))}. References: {', '.join(r['id'] for r in a['provenance'].get('references', []))}.")
    for kind in ["cli", "probe"]:
        f = PROOF / f"{slug}.{kind}.txt"
        if f.exists():
            lines = [l[:330] for l in f.read_text("utf8").splitlines() if l.strip()]
            print(f"\nProof ({'check CLI `--file`, 9 samples per clip' if kind == 'cli' else '`FIN-B/probe.ts`'}):\n\n```\n" + "\n".join(lines) + "\n```")
    print()
