"""🧬 C4P28 expansion differential for the include-fragment splits (test target roots 3, 6, 7).
`rustc -Zunpretty=expanded` expands every `include!`, `include_str!` and `#[path]` mount, so the expanded item stream of the ORIGINAL root must equal the one of the split root:
the spliced fragments are the original text, a mis-based `include_str!` path would fail to read (or change the string literal), and a lost or reordered range would show as a diff hunk.
Both sides run in sandboxes under scratchpad/c4p28-work with `extern crate` lines removed and CARGO_MANIFEST_DIR fixed to the same constant; nothing is written to the repo.
usage: expand_diff.py stage|live <n>...   (3, 6, 7)"""
import json, os, re, shutil, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = "/Users/ueli/Documents/semio"
SCRATCH = "/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad/c4p28-work"
F = REPO + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
E = "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor"
mode = sys.argv[1]
ns = [int(a) for a in sys.argv[2:]]
NEED = {
    3: [f"{E}/⏳️precompute/🪣️fill/🧪️tests", "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json"],
    6: [f"{E}/🧪️tests/🧰️context", f"{E}/🧪️tests/🔬️budgets"],
    7: [f"{E}/🧪️tests/🧰️context", f"{E}/🧪️tests/🔬️panels"],
}


def under(path):
    rp = os.path.realpath(path)
    assert rp.startswith(os.path.realpath(SCRATCH) + os.sep), f"write outside sandbox: {rp}"
    return path


def copy(rel, dst_root, src_root=REPO):
    src = os.path.join(src_root, rel)
    dst = under(os.path.join(dst_root, rel))
    if os.path.isdir(src):
        shutil.copytree(src, dst, dirs_exist_ok=True)
    elif os.path.isfile(src):
        os.makedirs(os.path.dirname(dst), exist_ok=True)
        shutil.copyfile(src, dst)


def strip_extern(root):
    for dp, dn, fn in os.walk(root):
        for f in fn:
            if f.endswith(".rs"):
                p = under(os.path.join(dp, f))
                t = open(p, encoding="utf-8").read()
                t2 = re.sub(r"^\s*extern\s+crate\s+\w+(?:\s+as\s+\w+)?;[ \t]*\n", "", t, flags=re.M)
                if t2 != t:
                    open(p, "w", encoding="utf-8").write(t2)


def expand(root, rel):
    env = dict(os.environ, CARGO_MANIFEST_DIR="/CM")
    p = subprocess.run(["rustc", "+nightly-2026-07-20", "--edition", "2021", "--crate-type", "lib", "--crate-name", "x", "--test", "-Zunpretty=expanded", os.path.join(root, rel)], capture_output=True, text=True, env=env, cwd=root)
    return p.stdout, p.stderr.replace(root, "<S>")


bad = 0
for n in ns:
    spec = json.load(open(f"{HERE}/specs/{n}.json", encoding="utf-8"))
    base = f"{SCRATCH}/expand-{n}"
    under(base)
    if os.path.exists(base):
        shutil.rmtree(base)
    A, B = f"{base}/A", f"{base}/B"
    for root in (A, B):
        for rel in NEED[n]:
            copy(rel, root)
    shutil.copyfile(spec["original"], under(os.path.join(A, spec["file"])))
    if mode == "stage":
        staged_ctx = os.path.join(spec["stage"], E, "🧪️tests/🧰️context/🦀️.rs")
        if os.path.isfile(staged_ctx) and n in (6, 7):
            shutil.copyfile(staged_ctx, under(os.path.join(A, E, "🧪️tests/🧰️context/🦀️.rs")))
    if mode == "stage":
        stage = spec["stage"]
        for dp, dn, fn in os.walk(stage):
            for f in fn:
                s = os.path.join(dp, f)
                rel = os.path.relpath(s, stage)
                if any(rel.startswith(x) for x in NEED[n]):
                    dst = under(os.path.join(B, rel))
                    os.makedirs(os.path.dirname(dst), exist_ok=True)
                    shutil.copyfile(s, dst)
    for root in (A, B):
        strip_extern(root)
    outA, errA = expand(A, spec["file"])
    outB, errB = expand(B, spec["file"])
    open(f"{F}/🗑️generated/c4p28-expand-{n}-A.rs", "w", encoding="utf-8").write(outA)
    open(f"{F}/🗑️generated/c4p28-expand-{n}-B.rs", "w", encoding="utf-8").write(outB)
    la, lb = outA.split("\n"), outB.split("\n")
    import difflib
    keep = lambda l: l.strip() != "" and not re.match(r"^\s*(#\[doc = |//|source_file: |start_line: |start_col: |end_line: |end_col: )", l)
    docA = [l for l in la if keep(l)]
    docB = [l for l in lb if keep(l)]
    d = [x for x in difflib.unified_diff(docA, docB, "A", "B", lineterm="", n=0) if not x.startswith(("---", "+++", "@@"))]
    errs = lambda e: sorted(set(re.findall(r"^error(?:\[E\d+\])?: (.*)$", e, flags=re.M)))
    print(f"[{n}] expanded lines A={len(la)} B={len(lb)}; non-doc diff lines={len(d)}; error kinds A={len(errs(errA))} B={len(errs(errB))} same={errs(errA) == errs(errB)}; errors A={errA.count('error')} B={errB.count('error')}")
    for x in d[:40]:
        print("   ", x[:220])
    if d or errs(errA) != errs(errB):
        bad += 1
print("EXPAND", "IDENTICAL (modulo fragment docs)" if bad == 0 else f"{bad} DIFFERENT")
sys.exit(1 if bad else 0)
