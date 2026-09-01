"""🧹️ Drop imports that bring the SAME NAME in by two different paths.

Merging a leaf's three files can import one symbol twice — `crate::artifacts::las::LasDiff` from the
artifact root and `crate::artifacts::las::schema::diff::LasDiff` from where it is actually defined. Both
name the same type (the root re-exports it) but Rust rejects the duplicate binding. Grouping by module
path deduped identical lines; it could not see a collision ACROSS paths.
The longer path wins: it names where the item is defined rather than where it is re-exported.
"""
import io, re, glob

fixed = dropped = 0
for leaf in glob.glob("✏️s/🔌️plugins/**/🧬️mutations/*/🦀️.rs", recursive=True):
    text = io.open(leaf, encoding="utf-8").read()
    lines = text.split("\n")
    owner: dict[str, str] = {}
    for line in lines:
        m = re.match(r"^use ([A-Za-z0-9_:]+)::\{([^}]*)\};$", line.strip()) or re.match(r"^use ([A-Za-z0-9_:]+)::([A-Za-z0-9_]+);$", line.strip())
        if not m:
            continue
        path = m.group(1)
        syms = [s.strip() for s in m.group(2).split(",")] if "{" in line else [m.group(2)]
        for sym in syms:
            if not sym:
                continue
            if sym not in owner or len(path) > len(owner[sym]):
                owner[sym] = path
    out, changed = [], False
    for line in lines:
        m = re.match(r"^use ([A-Za-z0-9_:]+)::\{([^}]*)\};$", line.strip()) or re.match(r"^use ([A-Za-z0-9_:]+)::([A-Za-z0-9_]+);$", line.strip())
        if not m:
            out.append(line)
            continue
        path = m.group(1)
        syms = [s.strip() for s in m.group(2).split(",")] if "{" in line else [m.group(2)]
        keep = [s for s in syms if s and owner.get(s) == path]
        if len(keep) == len(syms):
            out.append(line)
            continue
        changed = True
        dropped += len(syms) - len(keep)
        if keep:
            indent = line[: len(line) - len(line.lstrip())]
            out.append(f"{indent}use {path}::{keep[0]};" if len(keep) == 1 else f"{indent}use {path}::{{{', '.join(keep)}}};")
    if changed:
        io.open(leaf, "w", encoding="utf-8").write("\n".join(out))
        fixed += 1
print(f"removed {dropped} cross-path duplicate import(s) in {fixed} leaf/leaves")
