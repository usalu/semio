"""📐 E51 field widener: rewrites the visibility of named fields of one brace struct (located by `struct NAME` nearest to a hint line), through `ed.edit` so every change is journalled and replayable by the oracle's before-run.
Only the struct body is touched; each named field must be found exactly once at depth 1. `--hidden` prefixes `#[doc(hidden)]` when the field has no attribute yet (the repo's convention for cross-crate construction fields of framework structs).
usage: widen.py <rel-path> <StructName> <hint-line> <new-vis> <field,field,...> [--hidden]   (new-vis: pub | pub(crate) | pub(super) | pub(in path))"""
import os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from ed import edit, ROOT


def balanced(t, i):
    d = 0
    for j in range(i, len(t)):
        if t[j] == "{":
            d += 1
        elif t[j] == "}":
            d -= 1
            if d == 0:
                return j
    raise SystemExit("unbalanced")


def widen(rel, name, hint, vis, fields, hidden=False):
    tx = open(os.path.join(ROOT, rel), encoding="utf-8").read()
    starts = [m.start() for m in re.finditer(r"\bstruct\s+" + re.escape(name) + r"\b(?:<[^{;]*?>)?\s*(?:where[^{;]*)?\{", tx)]
    assert starts, f"no brace struct {name} in {rel[-60:]}"
    line_of = lambda p: tx.count("\n", 0, p) + 1
    s = min(starts, key=lambda p: abs(line_of(p) - hint))
    b = tx.index("{", s)
    e = balanced(tx, b)
    body = tx[b : e + 1]
    new = body
    for f in fields:
        pat = re.compile(r"(?P<pre>[{,\]]\s*|\n\s*)(?P<vis>pub(?:\([^)]*\))?\s+)?(?P<f>" + re.escape(f) + r")(?P<post>\s*:)(?!:)")
        ms = list(pat.finditer(new))
        assert len(ms) == 1, f"field {f} found {len(ms)}x in {name}"
        m = ms[0]
        pre = m.group("pre")
        prefix = new[: m.start()]
        attr = "" if (hidden is False or "doc(hidden)" in prefix[-80:].split("\n")[-1] or prefix.rstrip().endswith("#[doc(hidden)]")) else "#[doc(hidden)] "
        new = new[: m.start()] + pre + attr + vis + " " + m.group("f") + m.group("post") + new[m.end() :]
    if new == body:
        print("nothing to change", name)
        return
    old_seg = tx[s : e + 1]
    new_seg = tx[s:b] + new
    edit(rel, old_seg, new_seg)


if __name__ == "__main__":
    a = [x for x in sys.argv[1:] if x != "--hidden"]
    widen(a[0], a[1], int(a[2]), a[3], a[4].split(","), hidden="--hidden" in sys.argv)
