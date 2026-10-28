"""🔁 [backlog] resolves the two conv-g MERGE-CONFLICT files against the live tree (witness2 changed the same lines conv-g re-documents): conv-g's docstring emojis/texts on top of the live code lines. Replaces staged/files/<rel> with the resolved text and staged/base/<rel> with the live text (originals kept in conv-g-tools/pre-backlog/, outside the staged tree because the tool treats every file under staged/files as a NEW file). Idempotent (a file whose staged base already equals live is skipped). usage: rebase_conv.py"""
import os, re, shutil, subprocess, sys, tempfile
C = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
R = os.path.abspath(os.path.join(C, *[".."] * 7)); S = C + "/conv-g-tools/staged"
LIB = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
PICK = {
 "🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📜️changes/🟦️.ts": [lambda m, l: [l[0], m[1]], lambda m, l: [l[0], m[1]]],
 LIB + "🔭️witness/🟨️.d.mts": [lambda m, l: [m[0], l[1]], lambda m, l: [l[0], l[1], m[2]], lambda m, l: [m[0], l[0], m[2]]],
}
for rel, picks in PICK.items():
    live, mine, base = f"{R}/{rel}", f"{S}/files/{rel}", f"{S}/base/{rel}"
    if open(live, "rb").read() == open(base, "rb").read(): print("skip (base == live)", rel[-30:]); continue
    out = subprocess.run(["git", "merge-file", "-p", "-L", "mine", "-L", "base", "-L", "live", mine, base, live], capture_output=True, text=True).stdout
    lines = out.split("\n"); res = []; hunk = 0; i = 0
    while i < len(lines):
        if lines[i].startswith("<<<<<<< "):
            j = lines.index("=======", i); k = next(x for x in range(j, len(lines)) if lines[x].startswith(">>>>>>> "))
            res += picks[hunk](lines[i + 1:j], lines[j + 1:k]); hunk += 1; i = k + 1
        else: res.append(lines[i]); i += 1
    assert hunk == len(picks), (rel, hunk)
    for f in (mine, base):
        keep = C + "/conv-g-tools/pre-backlog/" + f[len(C + "/conv-g-tools/staged/"):].replace("/", "%")
        os.makedirs(os.path.dirname(keep), exist_ok=True)
        if not os.path.exists(keep): shutil.copy(f, keep)
    open(mine, "w", encoding="utf8").write("\n".join(res)); shutil.copy(live, base); print("resolved", hunk, "hunks", rel[-40:])
