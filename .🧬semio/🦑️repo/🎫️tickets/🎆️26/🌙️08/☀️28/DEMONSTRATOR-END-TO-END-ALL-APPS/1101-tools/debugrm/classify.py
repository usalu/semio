import sys, re, os, time
ROOT = "/Users/ueli/Documents/semio"
files = [l.rstrip("\n") for l in open(sys.argv[1], encoding="utf-8") if l.strip()]
MACRO = re.compile(r'(eprintln!|println!|console\.log|console\.error|eprint!|print!)\s*\(')
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
for rel in files:
    p = os.path.join(ROOT, rel)
    try:
        lines = open(p, encoding="utf-8", errors="replace").read().split("\n")
    except Exception as e:
        print(f"{rel}\tERR\t{e}"); continue
    age = int((time.time() - os.stat(p).st_mtime) / 60)
    for n, line in enumerate(lines, 1):
        if "[DEBUG]" not in line: continue
        m = None
        for mm in MACRO.finditer(line):
            if "[DEBUG]" in line[mm.start():] and mm.start() <= line.find("[DEBUG]"):
                m = mm
        if not m:
            cls = "NOMACRO"
        else:
            op = m.end() - 1
            close = match_close(line, op)
            if close < 0:
                cls = "MULTI"
            else:
                rest = line[close+1:].strip()
                before = line[:m.start()].strip()
                full = (before == "" or before.endswith("{") or before.endswith("=>") or before.endswith(";") or before.endswith("}") or before.endswith("else"))
                tail_ok = rest in ("", ";") or rest.startswith(";") or rest.startswith("}")
                cls = ("FULL" if full and tail_ok else "PARTIAL") + ("" if before == "" else "+PREFIX")
        test = bool(re.search(r"(🧪️tests?|/tests?/|🧪️test/|\.test\.|🔬️unit|tests/)", rel))
        print(f"{rel}\t{n}\t{cls}\t{'TEST' if test else 'NONTEST'}\t{age}\t{line.strip()[:160]}")
