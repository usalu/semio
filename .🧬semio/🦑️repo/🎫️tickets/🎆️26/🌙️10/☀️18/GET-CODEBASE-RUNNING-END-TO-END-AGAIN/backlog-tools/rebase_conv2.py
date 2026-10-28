"""🔁 [backlog] second conv-g rebase (live moved again: quiet soft-stop policy field, ambient-environment export, claims idle()): four conflict files resolved against a MIRROR state (default blc3): a one-line conv-g change is re-applied as a substring replacement onto the changed live line, docstring lines come from conv-g, code lines from live. Replaces staged/files/<rel> and staged/base/<rel> (originals in conv-g-tools/pre-backlog/ with the suffix .2). Idempotent. usage: rebase_conv2.py [mirror=blc3]"""
import os, shutil, subprocess, sys
C = os.path.dirname(os.path.dirname(os.path.abspath(__file__))); S = C + "/conv-g-tools/staged"
home = os.path.join(os.environ.get("TMPDIR", "/tmp"), "sm"); mir = sys.argv[1] if len(sys.argv) > 1 else "blc3"
LIB = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
def sub(base, mine):
    i = 0
    while i < min(len(base), len(mine)) and base[i] == mine[i]: i += 1
    j = 0
    while j < min(len(base), len(mine)) - i and base[-1 - j] == mine[-1 - j]: j += 1
    return i, base[i:len(base) - j], mine[i:len(mine) - j]
def one(m, b, l):
    i, old, new = sub(b[0], m[0]); before = b[0][max(0, i - 48):i]; after = b[0][i + len(old):i + len(old) + 48]
    assert l[0].count(before + old + after) == 1, ("anchor", before[-30:], old[:30], after[:30])
    return [l[0].replace(before + old + after, before + new + after)]
PICK = {
 "🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📏️decision/🟦️.ts": [one],
 "🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/🧫️fixtures/🔣️.json": [one],
 LIB + "🔭️witness/🟨️.d.mts": [lambda m, b, l: l + m],
 LIB + "🕸️graph/🕰️generations/🧩️claims/🟨️.d.mts": [lambda m, b, l: [m[0], l[0], m[2]]],
}
for rel, picks in PICK.items():
    live, mine, base = f"{home}/{mir}/{rel}", f"{S}/files/{rel}", f"{S}/base/{rel}"
    if open(live, "rb").read() == open(base, "rb").read(): print("skip", rel[-30:]); continue
    out = subprocess.run(["git", "merge-file", "-p", "--diff3", "-L", "mine", "-L", "base", "-L", "live", mine, base, live], capture_output=True, text=True).stdout
    lines = out.split("\n"); res = []; i = 0; h = 0
    while i < len(lines):
        if lines[i].startswith("<<<<<<< "):
            j = lines.index("||||||| base", i); k = lines.index("=======", j); e = next(x for x in range(k, len(lines)) if lines[x].startswith(">>>>>>> "))
            res += picks[h](lines[i + 1:j], lines[j + 1:k], lines[k + 1:e]); h += 1; i = e + 1
        else: res.append(lines[i]); i += 1
    assert h == len(picks), (rel, h)
    for f in (mine, base):
        keep = C + "/conv-g-tools/pre-backlog/" + f[len(S) + 1:].replace("/", "%") + ".2"
        if not os.path.exists(keep): shutil.copy(f, keep)
    open(mine, "w", encoding="utf8").write("\n".join(res)); shutil.copy(live, base); print("resolved", h, rel[-40:])
