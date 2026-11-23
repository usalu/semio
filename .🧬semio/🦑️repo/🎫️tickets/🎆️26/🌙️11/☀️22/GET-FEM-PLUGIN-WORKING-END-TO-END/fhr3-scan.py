import os, re, sys
root = sys.argv[1]
pat = re.compile(r'impl\b[^{;]*\bErasedSnapshotRetirement\s+for\s+([^{]*?)(?:\s+where[^{]*)?\s*\{', re.S)
for d, _, fs in os.walk(root):
    for f in fs:
        if not f.endswith('.rs'):
            continue
        p = os.path.join(d, f)
        if 'tests' in p or 'testing' in p:
            continue
        try:
            s = open(p, encoding='utf-8').read()
        except Exception:
            continue
        for m in pat.finditer(s):
            i = m.end()
            depth = 1
            j = i
            while j < len(s) and depth:
                c = s[j]
                depth += (c == '{') - (c == '}')
                j += 1
            body = s[i:j]
            has = 'native_header_release' in body
            term = 'terminal_is_empty' in body
            line = s.count('\n', 0, m.start()) + 1
            if not has:
                print(f"{p}:{line} | {m.group(1).strip()} | term_in_impl={term}")
