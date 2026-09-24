"""🔓️ Leaf payload fields are the schema's public contract: inside every `dsl::MutationLeaf` payload struct under the given
roots, `pub(crate)` field visibility becomes `pub`, so case hosts (their own crates) can build and read payloads.
Positional args: repo-relative roots. `--dry` reports without writing."""
import os, re, sys
root = "/Users/ueli/Documents/semio/"
dry = "--dry" in sys.argv
roots = [a for a in sys.argv[1:] if not a.startswith("--")]
STRUCT = re.compile(r"(#\[derive\([^)]*dsl::MutationLeaf[^)]*\)\][^{;]*?pub struct \w+(?:<[^>{]*>)?\s*\{)(.*?)(\n\})", re.S)
files, fields = [], 0
for base in roots:
    for dp, ds, fs in os.walk(root + base):
        ds[:] = [d for d in ds if d not in ("target", "node_modules", "dist", "🗑️generated")]
        if "🦀️.rs" not in fs or "🔣️.json" not in fs or "🧬️mutations" not in dp: continue
        path = os.path.join(dp, "🦀️.rs")
        text = open(path, encoding="utf-8").read()
        count = 0
        def widen(m):
            global count
            body, n = re.subn(r"(?m)^(\s*(?:#\[[^\n]*\]\s*)*)pub\(crate\) ", r"\1pub ", m.group(2))
            count += n
            return m.group(1) + body + m.group(3)
        new = STRUCT.sub(widen, text)
        if new != text:
            files.append(path[len(root):]); fields += count
            if not dry: open(path, "w", encoding="utf-8").write(new)
print(f"{'would widen' if dry else 'widened'} {fields} field(s) in {len(files)} leaf file(s)")
for f in files: print("  ", f)
