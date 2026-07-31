import re

files = {
    'Go': ('compose/go/main.go', r'^// #region (.+)', r'^// #endregion (.+)'),
    'TS': ('compose/js/index.ts', r'^// #region (.+)', r'^// #endregion (.+)'),
    'Py': ('compose/py/main.py', r'^# #region (.+)', r'^# #endregion (.+)'),
    'CS': ('compose/net/Compose/Compose.cs', r'^\s{4}#region (.+)', r'^\s{4}#endregion (.+)'),
    'RS': ('compose/rs/lib.rs', r'^pub mod \w+ \{ // (.+)', r'^\} // (.+)'),
}

for lang, (path, start_re, end_re) in files.items():
    print(f'\n=== {lang} ===')
    with open(path, 'r', encoding='utf-8') as f:
        lines = f.readlines()
    depth = 0
    for i, line in enumerate(lines, 1):
        ls = line.rstrip()
        m = re.match(start_re, ls)
        if m:
            indent = '  ' * depth
            print(f'  {indent}L{i}: {m.group(1)}')
            depth += 1
        m2 = re.match(end_re, ls)
        if m2:
            depth = max(0, depth - 1)
