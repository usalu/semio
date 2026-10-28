import os, re, sys, glob
C = sys.argv[1]
pat = re.compile(r'\U0001F5D1️?generated/[^\s`\'"()\[\]|,;{}<>*]+')
seen = {}
for f in sorted(glob.glob(os.path.join(C, 'result-*-g.md'))):
    with open(f, encoding='utf-8', errors='replace') as fh:
        for ln, line in enumerate(fh, 1):
            for m in pat.finditer(line):
                p = m.group(0).rstrip('.:/')
                seen.setdefault(p, []).append((os.path.basename(f), ln))
for p, refs in sorted(seen.items()):
    ok = os.path.exists(os.path.join(C, p))
    print(('OK  ' if ok else 'MISS'), p, refs[0][0] + ':' + str(refs[0][1]), len(refs))
