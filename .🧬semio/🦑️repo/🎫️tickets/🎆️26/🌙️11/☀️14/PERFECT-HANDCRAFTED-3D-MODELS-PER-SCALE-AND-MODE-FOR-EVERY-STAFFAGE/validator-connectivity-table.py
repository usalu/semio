"""🔗️ Turns the lines of `validator-connectivity-probe.ts` into markdown: `measure` prints the catalog table of the raw measurement (`probe.jsonl`), `rules` the per-asset findings of the real rules (`rules.jsonl`).

Usage: `python3 validator-connectivity-table.py measure|rules|detail` (stdout; `detail` lists the loose groups of the raw measurement per asset, the data behind the design decisions)."""
import collections
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "🗑️generated", "validator-connectivity")
SCALES = ["50", "100", "200", "500"]
FEATURE = {"50": 0.0075, "100": 0.015, "200": 0.03, "500": 0.075}


def lines(name):
    latest = {}
    path = os.path.join(OUT, name)
    if os.path.exists(path):
        for line in open(path, encoding="utf8"):
            if line.strip():
                record = json.loads(line)
                latest[record["asset"]] = record
    return [latest[key] for key in sorted(latest)]


def kind_of(group, scale, structure):
    if structure and group["kinds"] == ["mass"]:
        return "mass"
    resting = group["grounded"] or group["onPlane"]
    near = group["gapM"] <= FEATURE[scale] + 1e-12
    if structure:
        if "support" in group["kinds"] or resting:
            return "supported-near" if near else "supported"
        return "unsupported"
    if not resting:
        return "floating"
    return "resting-near" if near else "resting"


def measure():
    records = lines("probe.jsonl")
    ok = [r for r in records if "error" not in r]
    print(f"Assets measured: {len(ok)} of {len(records)} ({len(records) - len(ok)} do not evaluate at the moment, being rewritten by their authors: {', '.join(r['asset'] for r in records if 'error' in r) or 'none'}).\n")
    print("| scale | assets | body parts | assets with loose body groups | loose body groups | floating (touch neither ground nor contact plane) | resting within one feature of the rest | resting farther away | structure parts | assets with loose structure groups | loose mass markers | member groups that reach neither a support solid nor the ground or a contact plane | supported groups within one feature | supported groups farther away |")
    print("|---|---|---|---|---|---|---|---|---|---|---|---|---|---|")
    for scale in SCALES:
        c = collections.Counter()
        for r in ok:
            s = r["scales"].get(scale)
            if s is None:
                continue
            c["assets"] += 1
            c["bodyParts"] += s["body"]["parts"]
            c["structureParts"] += s["structure"]["parts"]
            loose = s["body"]["groups"][1:]
            c["bodyAssets"] += 1 if loose else 0
            c["bodyLoose"] += len(loose)
            for g in loose:
                c["body:" + kind_of(g, scale, False)] += 1
            loose = [g for g in s["structure"]["groups"] if g is not s["structure"]["groups"][0] or kind_of(g, scale, True) == "unsupported"]
            real = [g for g in loose if kind_of(g, scale, True) != "mass"]
            c["structureAssets"] += 1 if real else 0
            for g in loose:
                c["structure:" + kind_of(g, scale, True)] += 1
        print(f"| 1:{scale} | {c['assets']} | {c['bodyParts']} | {c['bodyAssets']} | {c['bodyLoose']} | {c['body:floating']} | {c['body:resting-near']} | {c['body:resting']} | {c['structureParts']} | {c['structureAssets']} | {c['structure:mass']} | {c['structure:unsupported']} | {c['structure:supported-near']} | {c['structure:supported']} |")
    print()
    print("| scale | list | own solids | under the feature, not slender, texture or a thin role | of these identical to a 1:50 solid (carry-over) | assets with such solids |")
    print("|---|---|---|---|---|---|")
    for scale in SCALES:
        for name in ["body", "thermal", "structure"]:
            c = collections.Counter()
            for r in ok:
                s = r["scales"].get(scale)
                if s is None:
                    continue
                f = s["feature"][name]
                c["solids"] += f["solids"]
                c["under"] += f["under"]
                c["carried"] += f["carried"]
                c["assets"] += 1 if f["under"] else 0
                c["findings"] += s["featureFindings"]
            print(f"| 1:{scale} | {name} | {c['solids']} | {c['under']} | {c['carried']} | {c['assets']} |")


def detail():
    for r in lines("probe.jsonl"):
        if "error" in r:
            continue
        rows = []
        for scale in SCALES:
            s = r["scales"].get(scale)
            if s is None:
                continue
            for name in ["body", "structure"]:
                for g in s[name]["groups"][0 if name == "structure" else 1:]:
                    k = kind_of(g, scale, name == "structure")
                    if g is s[name]["groups"][0] and k != "unsupported":
                        continue
                    if k == "mass":
                        continue
                    rows.append(f"  - 1:{scale} {name} {k}: {', '.join(g['ids'])}{' +' + str(g['more']) if g['more'] else ''} ({g['parts']} parts, {'/'.join(str(x) for x in g['kinds'])}{', slender' if g['slender'] else ''}{', texture' if g['texture'] else ''}), {g['gapM'] * 1000:.1f} mm from `{g['nearest']}`")
        if rows:
            print(f"- **{r['asset']}**")
            print("\n".join(rows))


def rules():
    records = lines("rules.jsonl")
    clean = [r["asset"] for r in records if "error" not in r and not r.get("findings")]
    broken = [r for r in records if "error" in r]
    failing = [r for r in records if r.get("findings")]
    per = collections.Counter()
    for r in failing:
        for rule in {f["rule"] for f in r["findings"]}:
            per[rule] += 1
    print(f"Assets checked: {len(records)}; without a finding of the two rules: {len(clean)}; with `volumes-connected` findings: {per['volumes-connected']}; with `volumes-feature` findings: {per['volumes-feature']}; not evaluable at the moment: {len(broken)} ({', '.join(r['asset'] for r in broken) or 'none'}).\n")
    print("| scale | assets with `volumes-connected` body findings | body findings | assets with structure findings | structure findings | assets with `volumes-feature` findings | `volumes-feature` findings |")
    print("|---|---|---|---|---|---|---|")
    for scale in SCALES:
        c = collections.Counter()
        for r in failing:
            seen = set()
            for f in r["findings"]:
                if str(f.get("scale")) != scale:
                    continue
                key = f["rule"] + (":" + f.get("list", "") if f["rule"] == "volumes-connected" else "")
                c[key] += 1
                seen.add(key)
            for key in seen:
                c["assets:" + key] += 1
        print(f"| 1:{scale} | {c['assets:volumes-connected:body']} | {c['volumes-connected:body']} | {c['assets:volumes-connected:structure']} | {c['volumes-connected:structure']} | {c['assets:volumes-feature']} | {c['volumes-feature']} |")
    print()
    for r in failing:
        print(f"### {r['asset']}\n")
        connected = [f for f in r["findings"] if f["rule"] == "volumes-connected"]
        for f in connected:
            print(f"- 1:{f['scale']} {f['list']}: {f['en']}")
        feature = [f for f in r["findings"] if f["rule"] == "volumes-feature"]
        by = collections.defaultdict(list)
        for f in feature:
            by[(f["scale"], f.get("list", ""))].append(f)
        for (scale, name), items in sorted(by.items()):
            ids = [str((f.get("data") or {}).get("solid", "?")) for f in items]
            print(f"- 1:{scale} {name}: `volumes-feature` {len(items)} solid(s): {', '.join(ids[:12])}{' …' if len(ids) > 12 else ''}")
        print()
    print("### Without a finding\n")
    print(", ".join(clean))


{"measure": measure, "detail": detail, "rules": rules}[sys.argv[1] if len(sys.argv) > 1 else "measure"]()
