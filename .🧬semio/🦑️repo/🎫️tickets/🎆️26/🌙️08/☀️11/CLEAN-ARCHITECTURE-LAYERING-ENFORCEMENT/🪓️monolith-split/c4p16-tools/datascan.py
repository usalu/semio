import re, sys, glob, os
for f in sorted(glob.glob('orig/*.rs.txt')):
    L = open(f, encoding='utf-8').read().split('\n')
    runs = []
    cur = None
    for i, l in enumerate(L):
        s = l.strip()
        lit = bool(re.match(r'^[\(\[\{]?\s*(b?"[^"]*"|\'.\'|-?0x[0-9A-Fa-f]+|-?[0-9][0-9_\.]*(f32|f64|u8|u16|u32|u64|i32|i64|usize)?)\s*[,\)\]\}]', s)) and len(s) < 160
        if lit:
            if cur is None: cur = [i, i]
            else: cur[1] = i
        else:
            if cur and cur[1] - cur[0] + 1 >= 12: runs.append(tuple(cur))
            cur = None
    long_ = [(i + 1, len(l)) for i, l in enumerate(L) if len(l) > 400]
    print(os.path.basename(f), 'literal runs >=12:', [(a + 1, b + 1) for a, b in runs], 'lines>400 chars:', long_[:8], 'bytes', sum(len(l.encode()) + 1 for l in L))
