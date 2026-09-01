import re, os

ROOT = "/Users/ueli/Documents/semio"
GLUE = os.path.join(ROOT, "✏️s/🔌️plugins/🏛️architect/📦️packages/🦀️rust/📦️glue.rs")
SCRATCH = "/private/tmp/claude-501/-Users-ueli-Documents-semio/b3777651-e26e-4d76-aa75-86723494357b/scratchpad"

with open(os.path.join(SCRATCH, "processed.txt"), encoding="utf-8") as f:
    processed = [l.strip() for l in f if l.strip()]
target_dirs = set(os.path.dirname(p) for p in processed)

with open(GLUE, encoding="utf-8") as f:
    glue = f.read()

glue_dir = os.path.dirname(GLUE)

pattern = re.compile(
    r'(?P<indent>[ \t]*)#\[path = "\."\]\n'
    r'(?P=indent)pub mod (?P<name>\w+) \{\n'
    r'(?P<indent2>[ \t]*)#\[path = "(?P<relpath>[^"]+/🦀️\.rs)"\]\n'
    r'(?P=indent2)mod component;\n'
    r'(?P=indent2)pub use component::\*;\n'
    r'(?P=indent)\}\n'
)

matched_count = 0
replaced_count = 0
unmatched_targets = set(target_dirs)

def resolve(relpath):
    full = os.path.normpath(os.path.join(glue_dir, relpath))
    rel_to_root = os.path.relpath(full, ROOT)
    return rel_to_root[:-len('/🦀️.rs')]

def repl(m):
    global matched_count, replaced_count
    relpath = m.group('relpath')
    d_abs = resolve(relpath)
    matched_count += 1
    if d_abs not in target_dirs:
        return m.group(0)
    unmatched_targets.discard(d_abs)
    replaced_count += 1
    indent2 = m.group('indent2')
    base_rel = relpath[:-len('🦀️.rs')]
    diff_path = base_rel + '🔺️diff/🦀️.rs'
    inv_path = base_rel + '↩️inverse/🦀️.rs'
    new_block = (
        f'{m.group("indent")}#[path = "."]\n'
        f'{m.group("indent")}pub mod {m.group("name")} {{\n'
        f'{indent2}#[path = "{diff_path}"]\n'
        f'{indent2}pub mod diff;\n'
        f'{indent2}#[path = "{inv_path}"]\n'
        f'{indent2}pub mod inverse;\n'
        f'{indent2}#[path = "{relpath}"]\n'
        f'{indent2}mod component;\n'
        f'{indent2}pub use component::*;\n'
        f'{m.group("indent")}}}\n'
    )
    return new_block

new_glue = pattern.sub(repl, glue)

print("total blocks matched by pattern:", matched_count)
print("blocks replaced (targets):", replaced_count)
print("target dirs not found/replaced:", len(unmatched_targets))
for u in list(unmatched_targets)[:20]:
    print(" MISSING:", u)

with open(GLUE, 'w', encoding='utf-8') as f:
    f.write(new_glue)
