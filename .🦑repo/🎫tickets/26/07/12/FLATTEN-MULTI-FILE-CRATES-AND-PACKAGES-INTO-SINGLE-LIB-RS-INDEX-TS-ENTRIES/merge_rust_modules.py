import re, sys, os

def indent(text, n=4):
    pad = ' ' * n
    lines = text.split('\n')
    return '\n'.join((pad + l if l.strip() else l) for l in lines)

def build_block(keyword, name, file_content):
    body = indent(file_content.rstrip('\n'))
    return f"{keyword} {name} {{\n{body}\n}}"

def merge(lib_path, mods, src_dir=None, use_path_attr=False):
    """mods: list of (keyword, name, file_path) in declaration order.
    Replaces the contiguous run of mod declarations (with optional #[path] attr lines)
    in lib_path with inline blocks, in order. Deletes the source files.
    """
    with open(lib_path, 'r') as f:
        text = f.read()
    lines = text.split('\n')

    # Build declaration line patterns to find & remove, and blocks to insert
    blocks = []
    first_idx = None
    last_idx = None
    remaining_lines = lines[:]

    # Find all matching declaration lines (and preceding #[path] attr if present)
    name_to_pattern = {}
    for keyword, name, _ in mods:
        name_to_pattern[name] = re.compile(r'^\s*(pub\s+)?mod\s+' + re.escape(name) + r'\s*;\s*$')

    indices_to_remove = set()
    name_order = []
    i = 0
    while i < len(lines):
        line = lines[i]
        matched_name = None
        for name, pat in name_to_pattern.items():
            if pat.match(line):
                matched_name = name
                break
        if matched_name:
            start = i
            # check preceding line for #[path = "..."] attribute
            if use_path_attr and i > 0 and re.match(r'^\s*#\[path\s*=\s*".*"\]\s*$', lines[i-1]):
                start = i - 1
            for j in range(start, i+1):
                indices_to_remove.add(j)
            name_order.append((matched_name, start, i))
            if first_idx is None or start < first_idx:
                first_idx = start
            if last_idx is None or i > last_idx:
                last_idx = i
        i += 1

    if first_idx is None:
        print(f"ERROR: no mod declarations found in {lib_path}", file=sys.stderr)
        sys.exit(1)

    mods_by_name = {name: (keyword, path) for keyword, name, path in mods}
    name_order.sort(key=lambda t: t[1])

    block_texts = []
    for name, _, _ in name_order:
        keyword, path = mods_by_name[name]
        with open(path, 'r') as f:
            content = f.read()
        block_texts.append(build_block(keyword, name, content))

    new_lines = lines[:first_idx] + ('\n\n'.join(block_texts)).split('\n') + lines[last_idx+1:]
    new_text = '\n'.join(new_lines)

    with open(lib_path, 'w') as f:
        f.write(new_text)

    for _, _, path in mods:
        os.remove(path)

    print(f"Merged {len(mods)} modules into {lib_path}")

if __name__ == '__main__':
    import json
    config_path = sys.argv[1]
    with open(config_path) as f:
        cfg = json.load(f)
    mods = [(m['keyword'], m['name'], m['path']) for m in cfg['mods']]
    merge(cfg['lib_path'], mods, use_path_attr=cfg.get('use_path_attr', False))
