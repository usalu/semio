"""✅ C4P5 static verification of the seven splits, against the staged tree or the live tree.
Checks per spec: lossless line multiset (original snapshot versus root plus children, `pub(super) ` prefix removed), #[test]/async_test fn-name multiset, region balance per file,
docstring emoji first and unique among siblings, path length (UTF-8 bytes, repo-relative), line counts, mount targets exist, brace depth zero at end of file, rustfmt parse.
usage: verify.py stage|live [n...]"""
import collections, glob, json, os, re, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/rf8-tools")
from rslex import lex

REPO = "/Users/ueli/Documents/semio"
mode = sys.argv[1]
only = [int(a) for a in sys.argv[2:]]
TESTRX = re.compile(r"#\[\s*(?:[\w:]*::)?(?:test|async_test|tokio::test)\b[^\]]*\]\s*(?:#\[[^\]]*\]\s*|//[^\n]*\n\s*)*(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?fn\s+(\w+)")
DOCRX = re.compile(r"^//[!/]? (\S)")


def strip_lines(text):
    out = []
    for l in text.split("\n"):
        t = l.strip().replace("pub(super) ", "")
        if t:
            out.append(t)
    return collections.Counter(out)


ok_all = True
rows = []
for path in sorted(glob.glob(HERE + "/specs/*.json")):
    n = int(os.path.basename(path).split(".")[0])
    if only and n not in only:
        continue
    spec = json.load(open(path, encoding="utf-8"))
    base = REPO if mode == "live" else spec["stage"]
    root_rel = spec["file"]
    folder_dir = os.path.dirname(root_rel)
    files = [root_rel] + [f"{folder_dir}/{c['folder']}/🦀️.rs" for c in spec["chunks"]]
    texts = {}
    problems = []
    for rel in files:
        p = os.path.join(base, rel)
        if not os.path.exists(p):
            problems.append(f"missing {rel}")
            continue
        texts[rel] = open(p, encoding="utf-8").read()
    if problems:
        print(f"[{n}] PROBLEMS {problems}")
        ok_all = False
        continue
    orig = open(spec["original"], encoding="utf-8").read()
    orig_imbalance = len(re.findall(r"^\s*//\s?#region ", orig, flags=re.M)) - len(re.findall(r"^\s*//\s?#endregion ", orig, flags=re.M))
    gone = strip_lines(orig) - sum((strip_lines(t) for t in texts.values()), collections.Counter())
    added = sum((strip_lines(t) for t in texts.values()), collections.Counter()) - strip_lines(orig)
    ot = sorted(m.group(1) for m in TESTRX.finditer(orig))
    nt = sorted(m.group(1) for m in TESTRX.finditer("\n".join(texts.values())))
    sibling_emoji = collections.Counter()
    for c in spec["chunks"]:
        first = texts[f"{folder_dir}/{c['folder']}/🦀️.rs"].split("\n")[0]
        m = DOCRX.match(first)
        if not m:
            problems.append(f"{c['folder']}: first line is no emoji doc: {first[:60]}")
        else:
            sibling_emoji[m.group(1)] += 1
        if c["folder"].split("️")[0] + "️" != c["folder"][: c["folder"].index("️") + 1]:
            problems.append(f"{c['folder']}: folder emoji not followed by U+FE0F")
    for e, k in sibling_emoji.items():
        if k > 1:
            problems.append(f"duplicate docstring emoji {e}")
    for rel, t in texts.items():
        if len(rel.encode("utf-8")) > 240:
            problems.append(f"path too long {len(rel.encode('utf-8'))}: {rel}")
        nopen = len(re.findall(r"^\s*//\s?#region ", t, flags=re.M))
        nclose = len(re.findall(r"^\s*//\s?#endregion ", t, flags=re.M))
        if nopen != nclose and not (rel == root_rel and nopen - nclose == orig_imbalance):
            problems.append(f"{rel.split('/')[-2]}: region markers {nopen}/{nclose} (original imbalance {orig_imbalance})")
        depths, inlit = lex(t)
        if depths[-1] != 0:
            problems.append(f"{rel.split('/')[-2]}: brace depth {depths[-1]} at end")
        r = subprocess.run(["rustfmt", "+nightly-2026-07-20", "--edition", "2021", "--emit", "stdout", "--config", "skip_children=true", os.path.join(base, rel)], capture_output=True, text=True)
        if r.returncode != 0 and "error" in r.stderr.lower():
            problems.append(f"rustfmt parse failed {rel.split('/')[-2]}: {r.stderr[:160]}")
    root_text = texts[root_rel]
    for c in spec["chunks"]:
        target = os.path.join(base, folder_dir, c["folder"], "🦀️.rs")
        mount = f'#[path = "{c["folder"]}/🦀️.rs"]' if "segments" in c else f'include!("{c["folder"]}/🦀️.rs");'
        if root_text.count(mount) != 1:
            problems.append(f"{c['folder']}: root carries the mount {root_text.count(mount)} times")
    sizes = {rel.split("/")[-2] if rel != root_rel else "ROOT": len(t.split("\n")) - 1 for rel, t in texts.items()}
    mx = max(v for k, v in sizes.items() if k != "ROOT")
    rows.append((n, sizes["ROOT"], mx, len(files) - 1, sum(gone.values()), sum(added.values()), ot == nt, len(ot)))
    print(f"[{n}] {root_rel.split('/')[-2]} root={sizes['ROOT']} max-child={mx} children={len(files)-1} lines-removed={sum(gone.values())} lines-added={sum(added.values())} test-fns {len(ot)}->{len(nt)} identical={ot == nt}")
    for l, k in sorted(gone.items()):
        print(f"    - removed {k}x {l[:150]}")
    shown = 0
    for l, k in sorted(added.items()):
        cls = "wiring" if (l.startswith("#[path") or l.startswith("mod ") or l.startswith("include!(") or re.match(r"^(pub(\([^)]*\))? )?use (self|super)::", l)) else "doc" if (l.startswith("//!") or l.startswith("///") or (l.startswith("// ") and not l.startswith("//#"))) else "region" if l.startswith("//#") else "other"
        if cls in ("other", "region"):
            print(f"    + added {k}x [{cls}] {l[:150]}")
            shown += 1
    if problems:
        ok_all = False
        for p in problems:
            print("    PROBLEM", p)
    if ot != nt:
        ok_all = False
print("VERIFY", "OK" if ok_all else "FAILED", mode)
sys.exit(0 if ok_all else 1)
