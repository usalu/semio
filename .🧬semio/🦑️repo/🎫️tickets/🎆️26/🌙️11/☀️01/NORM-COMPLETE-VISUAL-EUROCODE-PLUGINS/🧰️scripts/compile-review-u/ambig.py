import re,sys
M=r'(max|min|abs|sqrt|powi|powf|floor|ceil|round|sin|cos|tan|ln|exp|signum|clamp|to_degrees|to_radians|hypot|atan|asin|acos|recip|trunc|fract)'
decl=re.compile(r'^\s*let\s+(?:mut\s+)?([a-z_][a-z0-9_]*)\s*=\s*-?\d+\.\d+\s*;')
out=[]
for p in open(sys.argv[1],encoding='utf-8').read().split('\n'):
    p=p.strip()
    if not p: continue
    try: lines=open(p,encoding='utf-8').read().split('\n')
    except Exception: continue
    for i,l in enumerate(lines):
        m=decl.match(l)
        if not m: continue
        v=m.group(1)
        use=re.compile(r'(?<![\w.])'+re.escape(v)+r'\.'+M+r'\(')
        for j in range(i+1,min(i+60,len(lines))):
            if re.match(r'^\s*let\s+(?:mut\s+)?'+re.escape(v)+r'\b',lines[j]): break
            if use.search(lines[j]):
                out.append(f"{p.split('norm/')[-1].replace('🏅️standards/🔖️1/🪆️subsets/✳️any/','')}:{j+1}: {v} (declared line {i+1}) {lines[j].strip()[:100]}"); break
print("\n".join(out))
