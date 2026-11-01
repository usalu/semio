import sys, re, os, time
ROOT = "/Users/ueli/Documents/semio"
MACRO = re.compile(r'(eprintln!|println!|console\.log|console\.error)\s*\(')
def match_close(s, i):
    depth = 0; j = i; q = None
    while j < len(s):
        c = s[j]
        if q:
            if c == '\\' and q != '`': j += 2; continue
            if c == q: q = None
        elif c in '"\'`': q = c
        elif c == '(': depth += 1
        elif c == ')':
            depth -= 1
            if depth == 0: return j
        j += 1
    return -1
SUS = re.compile(r'(next\(|take\(|pop\(|drain|advance|poll|tick|step\(|borrow_mut|\.send|\bset\(|insert|remove|push|await|\+\+|\+=|=[^=>]|resolve|close|admit|\.run|read\(|write|spawn|unwrap|\.get\()')
for line in open(sys.argv[1], encoding="utf-8"):
    rel, n, cls, test, age, text = line.rstrip("\n").split("\t", 5)
    if cls != "FULL" and cls != "FULL+PREFIX": continue
    p = os.path.join(ROOT, rel)
    src = open(p, encoding="utf-8", errors="replace").read().split("\n")
    L = src[int(n)-1]
    m = None
    for mm in MACRO.finditer(L):
        if mm.start() <= L.find("[DEBUG]"): m = mm
    close = match_close(L, m.end()-1)
    args = L[m.end():close]
    flag = "SUS" if SUS.search(args) else "ok"
    print(f"{flag}\t{rel}\t{n}\t{test}\t{args[:200]}")
