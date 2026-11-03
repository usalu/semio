import os, re, sys, time
ROOT = "/home/deck/semio/✏️s/🔌️plugins/📕️norm"
MIN = int(sys.argv[1]); FILTERS = sys.argv[2:]
now = time.time()
out = []
for d, ds, fs in os.walk(ROOT):
    ds[:] = [x for x in ds if x not in ("target", "node_modules")]
    for f in fs:
        if not f.endswith(".rs"): continue
        p = os.path.join(d, f)
        if now - os.path.getmtime(p) > MIN*60: continue
        if FILTERS and not any(x in p for x in FILTERS): continue
        t = open(p, errors="ignore").read()
        t = re.sub(r'//[^\n]*', '', t)
        t = re.sub(r'"(?:\\.|[^"\\])*"', '""', t)
        t = re.sub(r"'(?:\\.|[^'\\])'", "''", t)
        stack = []  # (kind,name,depth)
        depth = 0; seen = {}
        for m in re.finditer(r'\{|\}|\b(mod|impl|trait|fn|struct|enum|const|static|type)\b\s*(?:<[^>{;]*>\s*)?([A-Za-z_][\w:<>, &\']*)?', t):
            s = m.group(0)
            if s == '{':
                depth += 1
            elif s == '}':
                depth -= 1
                while stack and stack[-1][2] >= depth + 1 and stack[-1][3] == 'blk' : break
                if stack and stack[-1][2] == depth + 1: stack.pop()
            else:
                kind, name = m.group(1), (m.group(2) or "").strip()
                # find what follows
                rest = t[m.end():m.end()+400]
                if kind in ("mod", "impl", "trait"):
                    br = re.search(r'[{;]', t[m.end():])
                    if br and br.group() == '{':
                        stack.append((kind, name if kind != "impl" else name.split(" for ")[-1][:40] + "|" + name[:60], depth + 1, 'blk'))
                    continue
                if kind in ("fn", "struct", "enum", "const", "static", "type"):
                    scope = tuple((k, n) for k, n, _, _ in stack)
                    if any(k in ("fn",) for k, _, _, _ in stack): continue
                    # skip fn nested in fn bodies: depth check
                    inner = [s_ for s_ in stack]
                    nm = name.split("<")[0].split(":")[0].strip().split(" ")[0]
                    if not nm or nm == "_": continue
                    cfg = t[max(0, m.start()-200):m.start()]
                    if '#[cfg' in cfg.split('\n')[-3:].__str__(): continue
                    # only items declared at the scope's own depth
                    if depth != (stack[-1][2] if stack else 0): continue
                    seen.setdefault((scope, kind if kind in ('fn','const','static') else 'ty', nm), []).append(m.start())
        for k, v in seen.items():
            if len(v) > 1 and k[1] in ('fn', 'ty', 'const', 'static') and not any(x[0] == 'impl' or x[0] == 'trait' for x in k[0]):
                out.append((p.replace(ROOT, ""), k[2], k[1], len(v)))
for o in sorted(out): print(f"{o[0]}\t{o[1]}\t{o[2]} x{o[3]}")
print(len(out), "duplicates")
