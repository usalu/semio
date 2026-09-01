import re, os, sys

ROOT = "/Users/ueli/Documents/semio"

def rel_glue_path(mutation_dir_rel, plugin):
    prefix = f"✏️s/🔌️plugins/{plugin}/"
    assert mutation_dir_rel.startswith(prefix), mutation_dir_rel
    tail = mutation_dir_rel[len(prefix):]
    return f"../../{tail}"

def patch_glue_for_plugin(plugin, mutation_dirs_rel, dry=False):
    glue_path = os.path.join(ROOT, f"✏️s/🔌️plugins/{plugin}/📦️packages/🦀️rust/📦️glue.rs")
    with open(glue_path, encoding='utf-8') as f:
        text = f.read()
    applied = []
    missing = []
    for d_rel in mutation_dirs_rel:
        relpath = rel_glue_path(d_rel, plugin)
        target = f'#[path = "{relpath}/🦀️.rs"]'
        # find this exact path mount followed by 'mod component;' on next line
        pat = re.compile(re.escape(target) + r'(\n(\s*)mod component;)')
        m = pat.search(text)
        if not m:
            missing.append(d_rel)
            continue
        indent = m.group(2)
        diff_mount = f'#[path = "{relpath}/🔺️diff/🦀️.rs"]\n{indent}pub mod diff;\n{indent}'
        inv_mount = f'#[path = "{relpath}/↩️inverse/🦀️.rs"]\n{indent}pub mod inverse;\n{indent}'
        replacement = diff_mount + inv_mount + target + m.group(1)
        text = text[:m.start()] + replacement + text[m.end():]
        applied.append(d_rel)
    if not dry:
        with open(glue_path, 'w', encoding='utf-8') as f:
            f.write(text)
    return applied, missing

if __name__ == "__main__":
    listfile = sys.argv[1]
    mode = sys.argv[2] if len(sys.argv) > 2 else "dry"
    with open(listfile, encoding='utf-8') as f:
        leaves = [l.strip() for l in f if l.strip()]
    by_plugin = {}
    for leaf in leaves:
        plugin = leaf.split('/')[2]
        d_rel = os.path.dirname(leaf)
        by_plugin.setdefault(plugin, []).append(d_rel)
    for plugin, dirs in by_plugin.items():
        applied, missing = patch_glue_for_plugin(plugin, dirs, dry=(mode == "dry"))
        print(f"{plugin}: applied={len(applied)} missing={len(missing)}")
        for m in missing:
            print("  MISSING:", m)
