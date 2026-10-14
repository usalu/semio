#!/usr/bin/env python3
"""🔓️ Puts back the `pub(crate)` that `🔒️narrow.py --strip` removed, where no compiler is available to re-widen them.

The pre-strip visibility is read from one auto-commit (read-only `git show`): every line of the plugin crate that carried
`pub(crate)` there is indexed by its text with `pub(crate)` removed. A current line equal to such a key gets the indexed
line back, but only in a position where `pub(crate)` is legal and was the stripped meaning: an item head outside trait
definitions, trait impls and `macro_rules!` bodies, or a named field inside a struct body. Lines that changed since keep
whatever the re-widening loop gave them. Usage: restore-visibility.py <commit> [--dry]
"""
import importlib.util, os, re, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("narrow", os.path.join(HERE, "🔒️narrow.py"))
nw = importlib.util.module_from_spec(spec); spec.loader.exec_module(nw)
ri, PLUGIN, REPO = nw.ri, nw.PLUGIN, "/Users/ueli/Documents/semio"
ITEM = re.compile(r"^\s*(?:(?:const|async|unsafe|extern\s+\"[^\"]*\")\s+)*(?:fn|struct|enum|trait|type|const|static|mod|use|union)\b")
FIELD = re.compile(r"^\s*(?:r#)?\w+\s*:(?!:)")


def old_index(commit):
    rel = os.path.relpath(PLUGIN, REPO)
    names = subprocess.run(["git", "-c", "core.quotepath=off", "ls-tree", "-r", "--name-only", commit, "--", rel], cwd=REPO, capture_output=True, text=True).stdout.split("\n")
    index = {}
    for n in names:
        if not n.endswith(".rs"): continue
        text = subprocess.run(["git", "show", f"{commit}:{n}"], cwd=REPO, capture_output=True, text=True).stdout
        for line in text.split("\n"):
            if "pub(crate)" in line:
                index.setdefault(re.sub(r"\bpub\(crate\)\s+", "", line), line)
    return index


def spans(raw, cl):
    bad, structs = [], []
    for m in re.finditer(r"macro_rules!\s*\w+\s*\{", cl):
        bad.append((m.end() - 1, ri.skip_group(cl, m.end() - 1)))
    for m in re.finditer(r"(?m)^\s*(?:pub(?:\([^)]*\))?\s+)?(?:unsafe\s+)?(?:impl\b[^{;]*\bfor\b[^{;]*|trait\s+\w+[^{;]*)\{", cl):
        bad.append((m.end() - 1, ri.skip_group(cl, m.end() - 1)))
    for m in re.finditer(r"(?m)^\s*(?:pub(?:\([^)]*\))?\s+)?(?:struct|union)\s+\w+[^{;(]*\{", cl):
        structs.append((m.end() - 1, ri.skip_group(cl, m.end() - 1)))
    return bad, structs


def main():
    index = old_index(sys.argv[1])
    dry = "--dry" in sys.argv
    total = 0
    for f in nw.files():
        raw = open(f, encoding="utf-8").read()
        cl = ri.clean(raw)
        bad, structs = spans(raw, cl)
        lines, pos, out, n = raw.split("\n"), 0, [], 0
        for line in lines:
            start = pos; pos += len(line) + 1
            old = index.get(line)
            if old is None or "pub(crate)" in line or any(a < start < b for a, b in bad):
                out.append(line); continue
            if ITEM.match(line) or (FIELD.match(line) and any(a < start < b and cl.count("{", a + 1, start) == cl.count("}", a + 1, start) for a, b in structs)):
                out.append(old); n += 1
            else:
                out.append(line)
        if n and not dry: open(f, "w", encoding="utf-8").write("\n".join(out))
        total += n
    print(f"[restore] {total} lines regained pub(crate) from {sys.argv[1]}")


if __name__ == "__main__":
    main()
