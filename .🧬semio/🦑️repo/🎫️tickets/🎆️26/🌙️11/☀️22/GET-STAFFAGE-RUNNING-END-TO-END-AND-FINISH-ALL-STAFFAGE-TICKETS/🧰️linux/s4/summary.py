#!/usr/bin/env python3
"""🧾️ Prints the proof lines of prove.ts as a table and the rule findings below it. usage: summary.py <proof dir> [--rules]"""
import json, os, sys
folder = sys.argv[1]
rules = "--rules" in sys.argv
ids = open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "assets.txt")).read().split()
for asset in ids:
    path = os.path.join(folder, asset + ".txt")
    if not os.path.exists(path):
        print(f"| `{asset}` | no proof file |"); continue
    lines = open(path).read().splitlines()
    proof = next((json.loads(l[6:]) for l in lines if l.startswith("PROOF ")), None)
    if proof is None:
        err = open(os.path.join(folder, asset + ".err")).read().strip().splitlines()
        print(f"| `{asset}` | NO PROOF LINE: {' / '.join(err[-3:])[:300]} |"); continue
    b = "; ".join(f"1:{x['scale']} {'x'.join(map(str, x['sizeMm']))} vs {'x'.join(map(str, x['dimsMm']))} mm, miss {x['oracleMiss']}" for x in proof["bounds"])
    t = "; ".join(f"{mode} {'>'.join(map(str, row))}" for mode, row in proof["triangles"].items())
    s = "; ".join(f"1:{k} {'/'.join(map(str, v))}" for k, v in proof["solids"].items())
    r = proof["rules"] if isinstance(proof["rules"], str) else (", ".join(f"{k.split(':')[1].replace('volumes-', '')} {v}" for k, v in proof["rules"].items()) or "none")
    cf = proof.get("catalogFindings", [])
    print(f"| `{asset}` | {'ok' if proof['ok'] else 'FAIL'}{' (catalog: ' + str(len(cf)) + ')' if cf else ''} | {'/'.join(proof['apply'])}, untouched {str(proof['untouched']).lower()} | {len(proof['volumeIssues'])} | {len(proof['auditFindings'])} | {t} | {b} | {s} | {r}{' ERROR ' + proof['rulesError'] if proof.get('rulesError') else ''} |")
    if not proof["ok"]:
        for issue in proof["volumeIssues"][:6]: print(f"    issue {issue}")
        for finding in proof["auditFindings"][:6]: print(f"    audit {finding}")
    for finding in cf[:8]: print(f"    catalog {finding}")
    if rules:
        for l in lines:
            if l.startswith("RULE "): print("    " + l[5 + len(asset) + 1:][:420])
