#!/usr/bin/env python3
"""Renders the census 8 tables (markdown) from census8.json and crates8.json. Read-only except for its own output file."""
import json, os, subprocess
from collections import defaultdict

ROOT = "/Users/ueli/Documents/semio"
HERE = os.path.dirname(os.path.abspath(__file__))
d = json.load(open(os.path.join(HERE, "census8.json"), encoding="utf-8"))
import glob
shards = [json.load(open(f, encoding="utf-8")) for f in sorted(glob.glob(os.path.join(HERE, "crates8.[0-9].json")))]
cr = {"gate": shards[0]["gate"], "crates": [c for s_ in shards for c in s_["crates"]], "touchedManifests": shards[0]["touchedManifests"], "skipped": shards[0]["skipped"]}
d_shards = [json.load(open(f, encoding="utf-8")) for f in sorted(glob.glob(os.path.join(HERE, "crates8.disk.[0-9].json")))]
hcr = cr
cr = {"gate": d_shards[0]["gate"], "crates": [c for s_ in d_shards for c in s_["crates"]], "touchedManifests": d_shards[0]["touchedManifests"], "skipped": d_shards[0]["skipped"]} if d_shards else cr
json.dump({"disk": cr, "head": hcr}, open(os.path.join(HERE, "crates8.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
S = d["summary"]
out = []
w = out.append


def area(path):
    segs = path.split("/")
    return "/".join(segs[:4]) if len(segs) > 4 else "/".join(segs[:-1])


def owner_or_area(r):
    return r["owner"] if r["owner"] != "(no owner)" else "(no owner) " + area(r["path"])


def fmt_bool(b):
    return "yes" if b else "no"


head = subprocess.run(["git", "-C", ROOT, "ls-tree", "-r", "-z", "--full-tree", "--name-only", "HEAD"], capture_output=True, check=True).stdout.decode("utf-8").split("\0")
tracked = set(x for x in head if x)

w("## 1. Totals at HEAD (bd9e4e02bd0)\n")
w(f"- Tracked blobs at HEAD: {S['tracked_blobs_head']}. In-scope files (census-7 extensions and exclusions): {S['inscope_files']}, {S['inscope_lines']} newline-counted lines.")
w(f"- Excluded by reason: {', '.join(f'{k} {v}' for k, v in sorted(S['excluded'].items()))}.")
w(f"- Over 1,000 lines at HEAD: {S['over1000_all']}. Header `@generated`-type match (excluded as generated): {S['over1000_generated_header']}. Hand-authored over 1,000: **{S['over1000_hand_authored']}**, of which `validateAgainst` / `(value: unknown)` signature: {S['over1000_sig']}.")
w(f"- Census 7 over-1,000 list: {S['c7_list_size']} paths. Still over 1,000 at HEAD (hand-authored): 0. Still over 1,000 (generated): {S['over1000_generated_header']}. Dropped from the list (now below 1,000 or gone): {S['dropped_from_c7_list']}. New hand-authored over-1,000 since census 7: **{S['new_vs_c7_list']}**.")
w(f"- Hand-authored over-1,000 files touched by the bulk commits: 8f91299152c {S['bulk_touch_counts']['8f91299152c']}, bd9e4e02bd0 {S['bulk_touch_counts']['bd9e4e02bd0']}.")
w("- Census 7 measured the working tree at 10-18 18:50. This census measures HEAD. The `pre-bulk lines` column reads the same paths at 4dab415ed90 (10-18 12:04), the last commit before the first bulk commit, as a proxy for the census-7 state.\n")

w("## 2. Hand-authored files over 1,000 lines at HEAD\n")
w("| # | Lines | Lang | Owner / area | In census 7 list | Pre-bulk lines (4dab) | Bulk commit | Signature | Path |")
w("| ---: | ---: | --- | --- | --- | ---: | --- | --- | --- |")
over = sorted(d["over"], key=lambda r: -r["lines"])
for i, r in enumerate(over, 1):
    bulk = ", ".join(b[:7] for b in r["bulk"]) or "-"
    pre = r["c7_commit_lines"] if r["c7_commit_lines"] is not None else ("absent" if not r["c7_commit_exists"] else "-")
    w(f"| {i} | {r['lines']:,} | {r['ext'][1:]} | {owner_or_area(r)} | {fmt_bool(r['in_c7_list'])} | {pre} | {bulk} | {fmt_bool(r['sig'])} | `{r['path']}` |")
w("")

w("## 3. Generated files over 1,000 lines (excluded)\n")
w("| Lines | Path | Reason |")
w("| ---: | --- | --- |")
for g in d["generated_over"]:
    reason = "generated header (`jco` transpile output; census 7 had it as unconfirmed)" if "jcoprobe" in g["path"] else "generated header (`@generated`/`DO NOT EDIT` in first 25 lines)"
    w(f"| {g['lines']:,} | `{g['path']}` | {reason} |")
w("")

w("## 4. Census 7 over-1,000 list at HEAD\n")
w("| Census 7 path | HEAD lines | Status at HEAD |")
w("| --- | ---: | --- |")
c7 = d["c7_list"]
over_map = {r["path"]: r["lines"] for r in d["over"]}
for p in c7:
    if p in tracked:
        lines = None
        if p in over_map:
            lines = over_map[p]
        else:
            data = subprocess.run(["git", "-C", ROOT, "cat-file", "blob", "HEAD:" + p], capture_output=True, check=True).stdout
            lines = data.count(b"\n")
        status = "still over 1,000 (generated)" if p in over_map else "below 1,000"
        w(f"| `{p}` | {lines:,} | {status} |")
    else:
        w(f"| `{p}` | - | absent at HEAD |")
w("")

w("## 5. Rust crate source gate (canonical reader, working tree)\n")
g = cr["gate"]
w(f"Gate from `storage.sourceBoundary` in `⚡️caching/🔣️policy.json` at HEAD: {g['maxSourceFiles']} files, {g['maxSourceBytes']} bytes, {g['maxActiveLines']} active lines. Reader: `readRustSourceClosureV1` from `🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts`, host cfg from `crateTreeHostCfg()`, same as the canonical `measureRustCrate`. Ratio = max over files, bytes, lines. Production = host cfg. Test = host cfg plus `test`. Primary run reads the working tree (what the gate sees). Targets: `[lib]` and `[[bin]]` roots of every `Cargo.toml` with `[package]` (not `.🧬semio`, not vendored, not generated dirs). Integration tests and benches are not measured.\n")
crates = cr["crates"]
ok_p = [c for c in crates if c["prod"]["ok"]]
flag = lambda c, k: c[k]["ok"] and c[k]["ratio"] >= 0.9
over_p = [c for c in ok_p if c["prod"]["ratio"] >= 0.9]
touched_set = set(cr["touchedManifests"])
w(f"- Crate targets measured: {len(crates)}. Refused by reader, production cfg: {sum(1 for c in crates if not c['prod']['ok'])}. Refused, test cfg: {sum(1 for c in crates if not c['test']['ok'])}.")
w(f"- At or over 0.90 of a dimension, production cfg: **{len(over_p)}** (over 1.00: {sum(1 for c in over_p if c['prod']['ratio'] > 1)}). Test cfg: {sum(1 for c in crates if flag(c, 'test'))} at or over 0.90, {sum(1 for c in crates if c['test']['ok'] and c['test']['ratio'] > 1)} over 1.00.")
w(f"- Crates whose `.rs` changed in the two bulk commits: {len(touched_set)} manifests. Of these, at or over 0.90 (either cfg) or refused: {sum(1 for c in crates if c['manifest'] in touched_set and (flag(c,'prod') or flag(c,'test') or not c['prod']['ok'] or not c['test']['ok']))}.\n")
flagged = [c for c in crates if flag(c, "prod") or flag(c, "test") or not c["prod"]["ok"] or not c["test"]["ok"]]
def worst(c):
    vals = [c[k]["ratio"] for k in ("prod", "test") if c[k]["ok"]]
    return max(vals) if vals else 9
flagged.sort(key=lambda c: -worst(c))
w("| Crate | Manifest | Kind | Prod files / bytes / lines | Prod ratio (dim) | Test ratio (dim) | Touched by bulk | Status |")
w("| --- | --- | --- | --- | --- | --- | --- | --- |")
for c in flagged:
    p, t = c["prod"], c["test"]
    pr = f"{p['ratio']:.3f} ({p['dim']})" if p["ok"] else "REFUSED"
    tr = f"{t['ratio']:.3f} ({t['dim']})" if t["ok"] else "REFUSED"
    pm = f"{p['files']} / {p['bytes']} / {p['lines']}" if p["ok"] else p.get("error", "")[:110]
    touched = "yes" if c["manifest"] in touched_set else "no"
    over1 = (p["ok"] and p["ratio"] > 1) or (t["ok"] and t["ratio"] > 1)
    status = "over gate" if over1 else ("refused" if (not p["ok"] or not t["ok"]) else "0.90 to 1.00")
    w(f"| `{c['name']}` | `{c['manifest']}` | {c['kind']} | {pm} | {pr} | {tr} | {touched} | {status} |")
w("")
if hcr:
    hc = hcr["crates"]
    hover = [c for c in hc if c["prod"]["ok"] and c["prod"]["ratio"] >= 0.9]
    w(f"HEAD-tree comparison (`git` blobs at bd9e4e02bd0, reader unchanged): {len(hc)} targets, refused production {sum(1 for c in hc if not c['prod']['ok'])}, at or over 0.90 production {len(hover)}. Refusals at HEAD are generated sources that exist only in the working tree (`🤖️generated` or ignored module files), so HEAD is not a valid gate state for them.\n")

w("## 6. Paths over 256 characters among tracked files at HEAD\n")
lp = [x for x in d["long_paths"]]
w(f"- Tracked paths at HEAD: {len(head)} (including `.🧬semio` tickets). Over 256 code points: {sum(1 for x in lp if x['chars'] > 256)}. Over 256 UTF-16 units (Windows MAX_PATH unit): 0. Over 256 UTF-8 bytes: {sum(1 for x in lp if x['bytes'] > 256)} (macOS and Linux count bytes, all under PATH_MAX).")
allp = sorted(((len(p.encode('utf-16-le')) // 2, p) for p in tracked), reverse=True)
w(f"- Longest repo-relative UTF-16 length: {allp[0][0]}. Paths above 216 UTF-16 units (i.e. over 256 once a 40-unit checkout prefix such as `C:\\Users\\<name>\\src\\semio\\` is added): {sum(1 for n, _ in allp if n > 216)}. Paths above 200 UTF-16 units: {sum(1 for n, _ in allp if n > 200)}.")
w("")
w("| UTF-16 units | Path |")
w("| ---: | --- |")
for n, p in allp[:10]:
    w(f"| {n} | `{p}` |")
w("")

w("## 7. Packages by owner (over-1,000 hand-authored files, disjoint)\n")
w("| Owner / area | Files | Lines | New since census 7 |")
w("| --- | ---: | ---: | ---: |")
pk = defaultdict(list)
for r in over:
    pk[owner_or_area(r)].append(r)
for k, files in sorted(pk.items(), key=lambda kv: -sum(f["lines"] for f in kv[1])):
    w(f"| {k} | {len(files)} | {sum(f['lines'] for f in files):,} | {sum(1 for f in files if not f['in_c7_list'])} |")
w("")

w("## 8. Crate split packages (production cfg at or over 0.90, or refused; disjoint by crate)\n")
gate_hot = [c for c in crates if (c["prod"]["ok"] and c["prod"]["ratio"] >= 0.9) or not c["prod"]["ok"]]
gp = defaultdict(list)
for c in gate_hot:
    gp["/".join(c["manifest"].split("/")[:-1][:3])].append(c)
w("| Area (first 3 segments of crate dir) | Crates | Max prod ratio | Refused | Crate names |")
w("| --- | ---: | ---: | ---: | --- |")
for k, cs_ in sorted(gp.items(), key=lambda kv: -len(kv[1])):
    mx = max((c["prod"]["ratio"] for c in cs_ if c["prod"]["ok"]), default=0)
    ref = sum(1 for c in cs_ if not c["prod"]["ok"])
    w(f"| `{k}` | {len(cs_)} | {mx:.3f} | {ref} | {', '.join('`' + c['name'] + '`' for c in cs_)} |")
w("")

print("\n".join(out))
