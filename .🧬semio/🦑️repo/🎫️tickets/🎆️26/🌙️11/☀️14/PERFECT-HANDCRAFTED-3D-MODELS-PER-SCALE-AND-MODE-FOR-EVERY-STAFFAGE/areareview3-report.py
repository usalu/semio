"""Reviewer 3: writes the '## Reviewer 3' section of area-decisions.md (replaces only that section; text parts live in areareview3-text.md next to this script), and the findings in model-findings.md.
Usage: python3 areareview3-report.py"""
import glob, importlib.util, json, os, re, subprocess, tempfile, time
T = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(T, *[".."] * 7))
ROOT = os.path.join(REPO, "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog")
s = importlib.util.spec_from_file_location("dec3", os.path.join(T, "areareview3-decisions.py")); dec = importlib.util.module_from_spec(s); s.loader.exec_module(dec)
text = open(os.path.join(T, "areareview3-text.md"), encoding="utf8").read()
cache = {}
def head(slug):
    if slug in cache: return cache[slug]
    p = glob.glob(f"{ROOT}/*/{slug}/🔣️.json")[0]
    rel = os.path.relpath(p, REPO)
    try: com = {r["id"]: r for r in json.loads(subprocess.check_output(["git", "-C", REPO, "show", "HEAD:" + rel], stderr=subprocess.DEVNULL))["asset"]["thermal"]["regions"]}
    except Exception: com = {}
    now = {r["id"]: r for r in json.load(open(p))["asset"]["thermal"]["regions"]}
    cache[slug] = (os.path.basename(os.path.dirname(os.path.dirname(p))), com, now)
    return cache[slug]
rows, cnt = [], {}
NAME = {"A": "ACCEPT", "S": "SET", "X": "exchange only", "SX": "SET + exchange", "D": "DEFER"}
for (slug, region), (act, val, why) in sorted(dec.R.items()):
    cat, com, now = head(slug)
    n = now.get(region) or {}
    cnt[NAME[act]] = cnt.get(NAME[act], 0) + 1
    rows.append(f"| {cat}/{slug} | {region} | {n.get('kind', '')} | {n.get('model', '')} | {(com.get(region) or {}).get('areaM2')} | {n.get('areaM2')} | {NAME[act]} | {why} |")
body = []
for slug, region, new, old, *sf in dec.BODY:
    cat, com, now = head(slug)
    body.append(f"| {cat}/{slug} | {region} | {(com.get(region) or {}).get('areaM2')} | {old} | {new} | {sf[0] if sf else 'unchanged'} |")
out = text.replace("{TOTALS}", f"{len(dec.R)} decided rows: " + ", ".join(f"{k} {v}" for k, v in sorted(cnt.items())) + f"; C27 redistributions: {len(dec.BODY)} body-region values.")
out = out.replace("{DECISIONS}", "| asset | region | kind | model | committed m2 | value m2 | decision | reason (independent hand calculation of the real object) |\n|---|---|---|---|---|---|---|---|\n" + "\n".join(rows))
ps = os.path.join(T, "areareview3-passes.json")
passes = json.load(open(ps)) if os.path.exists(ps) else []
out = out.replace("{PASSES}", "| time | audit rows (all categories) | new in my categories | decided | note |\n|---|---|---|---|---|\n" + "\n".join(f"| {x['time']} | {x['rows']} | {x['new']} | {x['decided']} | {x['note']} |" for x in passes))
out = out.replace("{BODY}", "| asset | region | committed m2 | before m2 | after m2 | skinFraction |\n|---|---|---|---|---|---|\n" + "\n".join(body))
p = os.path.join(T, "area-decisions.md")
for _ in range(8):
    t = open(p, encoding="utf8").read()
    i = t.find("## Reviewer 3 (")
    j = t.find("\n## ", i + 5) if i >= 0 else -1
    new = (t[:i] + out + (t[j:] if j > 0 else "")) if i >= 0 else t.rstrip("\n") + "\n\n" + out
    if open(p, encoding="utf8").read() != t: time.sleep(0.2); continue
    fd, tmp = tempfile.mkstemp(dir=T); os.write(fd, new.encode("utf8")); os.close(fd); os.replace(tmp, p); break
mf = os.path.join(T, "model-findings.md")
f = open(mf, encoding="utf8").read()
f = re.sub(r"\n## Area review 3 \(reviewer 3\)\n.*?(?=\n## |\Z)", "", f, flags=re.S).rstrip("\n") + "\n"
f += "\n## Area review 3 (reviewer 3)\n" + "".join(f"- {a}: {b}\n" for a, b in dec.FIND)
fd, tmp = tempfile.mkstemp(dir=T); os.write(fd, f.encode("utf8")); os.close(fd); os.replace(tmp, mf)
print("rows", len(rows), "body", len(body), "findings", len(dec.FIND))
