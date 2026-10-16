"""🧹️ Applies only the rustfmt hunks that sit inside `use` blocks to the files this slice rewrote (rustfmt would sort the new `semio_framework_ui_*` imports into the block), leaving every pre-existing non-import style hunk alone. Proof: the multiset of whitespace-stripped `;`-statements of the file is unchanged. Usage: python3 rtd-fmt-imports.py [--apply] <changed-files.json>
@see ../RESULT-RENDERER-TEST-DEPS.md"""
import difflib, json, os, re, subprocess, sys, collections
ROOT = "/Users/ueli/Documents/semio"
APPLY = "--apply" in sys.argv
files = json.load(open([a for a in sys.argv[1:] if not a.startswith("--")][0], encoding="utf8"))
IMPORTISH = re.compile(r"^\s*(?:#\[[^\]]*\]\s*|(?:pub(?:\([^)]*\))?\s+)?use\s[^;]*;?\s*|[A-Za-z_][A-Za-z0-9_:]*(?:\s+as\s+\w+)?,?\s*|\};?\s*|\s*)$")
MINE = re.compile(r"semio_framework_ui_|ui_wgpu")


def fmt(path):
    p = subprocess.run(["rustfmt", "+nightly-2026-07-20", "--edition", "2021", "--unstable-features", "--skip-children", "--emit", "stdout", "--config-path", ROOT + "/rustfmt.toml", path], capture_output=True, text=True, cwd=ROOT)
    out = p.stdout
    m = re.match(r"^(?:.*\n)*?(?=)", out)
    if p.returncode != 0:
        return None
    lines = out.split("\n")
    if lines and lines[0].startswith("/") and lines[0].endswith(":"):
        lines = lines[2:] if lines[1] == "" else lines[1:]
    return "\n".join(lines)


def stmts(text):
    return collections.Counter(re.sub(r"\s+", "", s) for s in text.split(";") if s.strip())


total_applied = 0
for f in files:
    path = os.path.join(ROOT, f)
    live = open(path, encoding="utf8").read()
    formatted = fmt(path)
    if formatted is None:
        print("SKIP (rustfmt failed)", f[-60:])
        continue
    a, b = live.split("\n"), formatted.split("\n")
    sm = difflib.SequenceMatcher(None, a, b, autojunk=False)
    out, acc, rej = [], 0, 0
    for tag, i1, i2, j1, j2 in sm.get_opcodes():
        if tag == "equal":
            out += a[i1:i2]
            continue
        side = a[i1:i2] + b[j1:j2]
        if all(IMPORTISH.match(l) for l in side) and any(MINE.search(l) for l in side):
            out += b[j1:j2]
            acc += 1
        else:
            out += a[i1:i2]
            rej += 1
    new = "\n".join(out)
    if new == live:
        continue
    if stmts(new) != stmts(live):
        print("PROOF FAILED (statement multiset differs), not applied:", f[-70:])
        continue
    total_applied += acc
    print(f"{'APPLY' if APPLY else 'DRY'} {f[-70:]}: {acc} import hunk(s) applied, {rej} other hunk(s) left")
    if APPLY:
        now = open(path, encoding="utf8").read()
        assert now == live, "file changed under us"
        open(path, "w", encoding="utf8").write(new)
print("total import hunks", total_applied)
