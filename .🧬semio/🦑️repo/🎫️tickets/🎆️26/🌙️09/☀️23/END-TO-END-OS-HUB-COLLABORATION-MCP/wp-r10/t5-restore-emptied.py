#!/usr/bin/env python3
"""🩹️ R10 T5 follow-up of `debug-trace.ts --scope all --apply`: a deleted test print that was the ONLY statement of its block
(`if let Err(error) = … {`, `for … {`, `} else {`, a match arm) leaves an empty block — dead code whose binding is now unused —
and a deleted Rust print that was the last reader of a nearby binding (`Err(error) => { eprintln!(…{error}); None }`) orphans it.
Those prints are restored as `test-note`s (the line comes back from the backup with the `[DEBUG]` tag dropped), the codemod's
own rule for a print whose deletion orphans what it reads. Idempotent: a block that is no longer empty is left alone.
usage: python3 t5-restore-emptied.py <backup dir> <file list> [--apply]"""
import difflib
import re
import sys

RUST_WORDS = {"self", "true", "false", "None", "Some", "Ok", "Err", "format", "len", "unwrap", "as_str", "to_string", "iter", "label", "is_empty"}

def print_reads(line):
    """🔎️ The bindings a Rust print line reads: inline `{name}` / `{name:?}` captures of its format literal and the leading
    identifier of every argument after it (`facet.label` → facet, `&rows` → rows, `x.len()` → x)."""
    literal = re.search(r'"((?:[^"\\]|\\.)*)"', line)
    if not literal:
        return set()
    inline = set(re.findall(r"(?<!\{)\{([A-Za-z_][A-Za-z0-9_]*)(?::[^}]*)?\}", literal.group(1)))
    tail = line[literal.end():].rsplit(")", 1)[0]
    trailing = {match.group(1) for match in re.finditer(r",\s*&?\s*([A-Za-z_][A-Za-z0-9_]*)", tail)}
    return inline | trailing


backup, listing = sys.argv[1], sys.argv[2]
apply = "--apply" in sys.argv
restored = 0
for path in [line for line in open(listing, encoding="utf-8").read().split("\n") if line]:
    old = open(f"{backup}/{path}", encoding="utf-8").read().split("\n")
    new = open(path, encoding="utf-8").read().split("\n")
    inserts = []
    for tag, i1, i2, j1, _ in difflib.SequenceMatcher(a=old, b=new, autojunk=False).get_opcodes():
        if tag != "delete":
            continue
        prev = next((new[k] for k in range(j1 - 1, -1, -1) if new[k].strip()), "")
        following = new[j1] if j1 < len(new) else ""
        emptied = prev.rstrip().endswith(("{", "(")) and following.strip().startswith(("}", ")"))
        window = "\n".join(new[max(0, j1 - 30):j1 + 30])
        names = {name for line in old[i1:i2] for name in print_reads(line)}
        orphaned = path.endswith(".rs") and any(len(re.findall(rf"\b{re.escape(name)}\b", window)) <= 1 for name in names if name not in RUST_WORDS)
        if emptied or orphaned:
            inserts.append((j1, [line.replace("[DEBUG] ", "").replace("[DEBUG]", "") for line in old[i1:i2]]))
    for at, lines in sorted(inserts, reverse=True):
        new[at:at] = lines
        restored += len(lines)
        print(f"restore {path}:{at + 1} {lines[0].strip()[:100]}")
    if inserts and apply:
        open(path, "w", encoding="utf-8").write("\n".join(new))
print(f"restored {restored} lines{' (applied)' if apply else ' (dry run)'}")
