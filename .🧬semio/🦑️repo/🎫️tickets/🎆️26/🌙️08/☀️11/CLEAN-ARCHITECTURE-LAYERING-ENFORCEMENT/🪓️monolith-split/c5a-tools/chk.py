"""✅ C5A verification of include-fragment splits against the stage or the live tree.
usage: chk.py stage|live <tag>...   checks: lossless line multiset + spliced sequence equality (fragments spliced back at their include! lines, one `../` rebase reversed, balancing region markers and the fragment docstring removed),
include literal targets equal, #[test] fn-name multiset equal, region balance per file, docstring emoji first/unique among siblings/equal to the folder emoji/different from the root docstring emoji, path length (UTF-8 bytes), brace depth 0, rustfmt parse."""
import collections, json, os, re, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE + "/../rf8-tools")
sys.path.insert(0, HERE)
from rslex import lex
from specs5 import FRAG
REPO = "/Users/ueli/Documents/semio"
STAGE = "/Users/ueli/.cache/c5a/stage-frag"
mode = sys.argv[1]
base = REPO if mode == "live" else STAGE
files_tsv = dict(l.rstrip("\n").split("\t") for l in open(HERE + "/files.tsv", encoding="utf-8"))
TESTRX = re.compile(r"#\[\s*(?:[\w:]*::)?(?:test|async_test|tokio::test)\b[^\]]*\]\s*(?:#\[[^\]]*\]\s*|//[^\n]*\n\s*)*(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?fn\s+(\w+)")
RG = re.compile(r"^\s*//\s?#region ")
ER = re.compile(r"^\s*//\s?#endregion ")
INC = re.compile(r'include(?:_str|_bytes)?!\(\s*"([^"]+)"')
ok_all = True
for tag in sys.argv[2:]:
    st = json.load(open(f"{HERE}/staged-{tag}.json", encoding="utf-8"))
    rel = st["root"]
    bdir = os.path.dirname(rel)
    orig = open(f"{HERE}/orig/{tag}.rs", encoding="utf-8").read()
    for old, new in FRAG[tag].get("pre", []):
        orig = orig.replace(old, new)
    root = open(os.path.join(base, rel), encoding="utf-8").read()
    kids = {f: open(os.path.join(base, bdir, f, "🦀️.rs"), encoding="utf-8").read() for f in st["folders"]}
    problems = []
    spliced, new_all = [], []
    mount_rx = re.compile(r'^include!\("([^"/]+)/🦀️\.rs"\);$')
    mounted = collections.Counter()
    for line in root.split("\n"):
        m = mount_rx.match(line.strip())
        if m and m.group(1) in kids:
            mounted[m.group(1)] += 1
            body = kids[m.group(1)].split("\n")
            assert re.match(r"^\s*//[/]? \S", body[0]), (m.group(1), body[0][:60])
            body = body[1:]
            body = [re.sub(r'(include(?:_str|_bytes)?!\(\s*)"\.\./', r'\1"', l, count=1) for l in body]
            spliced.extend(body)
        else:
            spliced.append(line)
    if FRAG[tag].get("root_doc"):
        assert spliced[0] == FRAG[tag]["root_doc"] and spliced[1] == "", (tag, "root doc")
        spliced = spliced[2:]
    for f in kids:
        if mounted[f] != 1:
            problems.append(f"{f}: mounted {mounted[f]} times")
    strip = lambda ls: collections.Counter(l.strip() for l in ls if l.strip())
    marker = lambda l: bool(RG.match(l) or ER.match(l))
    o_lines = orig.split("\n")
    n_lines = [l for l in spliced if not mount_rx.match(l.strip())]
    gone = strip(o_lines) - strip(n_lines)
    extra = strip(n_lines) - strip(o_lines)
    extra_nonmarker = {l: k for l, k in extra.items() if not l.startswith("//#region") and not l.startswith("//#endregion")}
    seq_o = [l.strip() for l in o_lines if l.strip() and not marker(l)]
    seq_n = [l.strip() for l in n_lines if l.strip() and not marker(l)]
    if gone or extra_nonmarker:
        problems.append(f"multiset gone={sum(gone.values())} extra={sum(extra_nonmarker.values())}")
    if seq_o != seq_n:
        k = next((i for i, (a, b) in enumerate(zip(seq_o, seq_n)) if a != b), min(len(seq_o), len(seq_n)))
        problems.append(f"sequence differs at {k}: {seq_o[k:k+1]} vs {seq_n[k:k+1]}")
    obase = os.path.join(REPO, bdir)
    oinc = collections.Counter(os.path.normpath(os.path.join(obase, m)) for m in INC.findall(orig))
    ninc = collections.Counter()
    for d, t in [(os.path.join(base, bdir), root)] + [(os.path.join(base, bdir, f), t) for f, t in kids.items()]:
        for m in INC.findall(t):
            if d == os.path.join(base, bdir) and m.endswith("/🦀️.rs") and m.split("/")[0] in kids and "/" not in m[:-len("/🦀️.rs")]:
                continue
            ninc[os.path.normpath(os.path.join(d, m)).replace(base, REPO, 1) if base != REPO else os.path.normpath(os.path.join(d, m))] += 1
    if oinc != ninc:
        problems.append(f"include targets differ: orig-only {list((oinc-ninc))[:2]} new-only {list((ninc-oinc))[:2]}")
    for p in ninc:
        if not os.path.exists(p):
            problems.append(f"include target missing {p[-80:]}")
    ot = sorted(m.group(1) for m in TESTRX.finditer(orig))
    nt = sorted(m.group(1) for m in TESTRX.finditer("\n".join([root] + list(kids.values()))))
    if ot != nt:
        problems.append("test fn multiset differs")
    root_emoji = (re.match(r"^\s*//[!/]*\s*(\S+)", root) or re.match(r"(.)", "?")).group(1)
    seen = collections.Counter()
    for f, t in kids.items():
        first = t.split("\n")[0]
        m = re.match(r"^\s*//[/]? (\S+) ", first)
        emo = f[: f.index("️") + 1]
        if not m or m.group(1) != emo:
            problems.append(f"{f}: docstring emoji {m.group(1) if m else None} != folder emoji {emo}")
        seen[emo] += 1
        if m and m.group(1).replace("️", "") == root_emoji.replace("️", ""):
            problems.append(f"{f}: docstring emoji equals root emoji")
        for e in os.listdir(os.path.join(base, bdir)) if False else []:
            pass
    for e, k in seen.items():
        if k > 1:
            problems.append(f"duplicate sibling emoji {e}")
    live_dir = os.path.join(REPO, bdir)
    for e in (os.listdir(live_dir) if os.path.isdir(live_dir) else []):
        if e in kids:
            continue
        if os.path.isdir(os.path.join(live_dir, e)) and e[: e.index("️") + 1] in seen:
            problems.append(f"sibling emoji clash with existing folder {e}")
    for r, t in [(rel, root)] + [(f"{bdir}/{f}/🦀️.rs", t) for f, t in kids.items()]:
        if len(r.encode()) > 240:
            problems.append(f"path too long {len(r.encode())}")
        no, nc = len([1 for l in t.split("\n") if RG.match(l)]), len([1 for l in t.split("\n") if ER.match(l)])
        if no != nc:
            problems.append(f"{r.split('/')[-2]}: region markers {no}/{nc}")
        d, _ = lex(t)
        if d[-1] != 0:
            problems.append(f"{r.split('/')[-2]}: brace depth {d[-1]}")
        pr = subprocess.run(["rustfmt", "+nightly-2026-07-20", "--edition", "2021", "--emit", "stdout", "--config", "skip_children=true", os.path.join(base, r)], capture_output=True, text=True)
        if pr.returncode != 0 and re.search(r"^error(?!\[internal\])", pr.stderr, flags=re.M):
            problems.append(f"rustfmt parse failed {r.split('/')[-2]}: {pr.stderr[:120]}")
    sizes = {f: len(t.split("\n")) - 1 for f, t in kids.items()}
    print(f"[{tag}] {mode} root={len(root.split(chr(10))) - 1} children={len(kids)} max-child={max(sizes.values())} tests {len(ot)}->{len(nt)} gone={sum(gone.values())} extra(non-marker)={sum(extra_nonmarker.values())} region-markers-added={sum(extra.values()) - sum(extra_nonmarker.values())} {'OK' if not problems else 'PROBLEMS'}")
    for p in problems:
        print("    PROBLEM", p)
        ok_all = False
print("CHK", "OK" if ok_all else "FAILED", mode)
sys.exit(0 if ok_all else 1)
