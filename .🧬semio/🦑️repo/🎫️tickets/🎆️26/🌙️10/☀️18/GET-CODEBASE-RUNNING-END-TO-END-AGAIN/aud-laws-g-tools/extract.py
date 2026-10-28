import re, glob, os, sys
C = sys.argv[1]
files = sorted(glob.glob(os.path.join(C, 'result-*-g.md')) + glob.glob(os.path.join(C, 'result-*-f.md')))
pat = re.compile(r'[^\s`\'"()\[\]|,;]*🧪️tests[^\s`\'"()\[\]|,;:]*')
pat2 = re.compile(r'[^\s`\'"()\[\]|,;]*🧪️tests[^\s`\'"()\[\]|,;:]*')
out = {}
for f in files:
    with open(f, encoding='utf-8', errors='replace') as h:
        for ln, line in enumerate(h, 1):
            if 'law' not in line.lower() and 'test' not in line.lower():
                continue
            for m in pat.finditer(line):
                t = m.group(0).rstrip('.')
                out.setdefault(t, set()).add(os.path.basename(f))
for t in sorted(out):
    print(len(out[t]), t, ','.join(sorted(out[t]))[:160])
