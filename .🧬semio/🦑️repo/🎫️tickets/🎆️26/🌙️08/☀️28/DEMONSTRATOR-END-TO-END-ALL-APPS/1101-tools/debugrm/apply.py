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
def idents(text):
    code = re.sub(r'"(\\.|[^"\\])*"|`[^`]*`|\'(\\.|[^\'\\])\'', lambda m: " ".join(re.findall(r'\{([A-Za-z_][A-Za-z0-9_]*)', m.group(0))), text)
    return set(re.findall(r'[A-Za-z_][A-Za-z0-9_]*', code)) - {"true","false","if","as","len","is_empty","unwrap","as_array","as_u64","as_str","name","usize","str","u64","i64","Some","None","String","x"}
def plan(rel, nums, apply):
    p = os.path.join(ROOT, rel)
    age = (time.time() - os.stat(p).st_mtime) / 60
    if age < 10:
        print(f"SKIP-RECENT {rel} age={age:.1f}m"); return
    raw = open(p, encoding="utf-8").read()
    lines = raw.split("\n")
    edits = []  # (index, start, end)
    for n in sorted(nums, reverse=True):
        L = lines[n-1]
        if "[DEBUG]" not in L: print(f"REFUSE {rel}:{n} no DEBUG"); continue
        ms = [m for m in MACRO.finditer(L) if m.start() <= L.find("[DEBUG]")]
        if not ms: print(f"REFUSE {rel}:{n} no macro"); continue
        m = ms[-1]; close = match_close(L, m.end()-1)
        if close < 0: print(f"REFUSE {rel}:{n} unclosed"); continue
        end = close + 1
        if L[end:].startswith(";"): end += 1
        before = L[:m.start()]; rest = L[end:]
        bs = before.strip()
        prev = next((lines[k].strip() for k in range(n-2, -1, -1) if lines[k].strip()), "")
        if bs and not bs.endswith(("{", ";", "}")): print(f"REFUSE {rel}:{n} before-ctx '{bs[-30:]}'"); continue
        if rest.strip() and not (rest.strip().startswith(("}", ";")) or rest.strip() == ""):
            print(f"REFUSE {rel}:{n} rest '{rest.strip()[:30]}'"); continue
        if not bs and prev.endswith((")", "else", "=>", ":", "=", "||")) and not prev.endswith(("*/",)):
            print(f"REFUSE {rel}:{n} prev-ctx '{prev[-40:]}'"); continue
        if bs == "" and rest.strip() == "" and not bs:
            new = None
        else:
            new = (before + rest)
        if new is not None and new.strip() == "":
            new = None
        edits.append((n, new, L, m.group(0)))
    if not edits: return
    # identifier check: identifiers in removed text must still be used elsewhere
    new_lines = list(lines)
    for n, new, L, _ in edits:
        new_lines[n-1] = None if new is None else new
    after = "\n".join(x for x in new_lines if x is not None)
    ok = []
    for n, new, L, mac in edits:
        removed = L if new is None else L.replace(new, "", 1)
        m = MACRO.search(L)
        ids = idents(L[m.start():]) if m else set()
        bad = [i for i in ids if i in set(re.findall(r'[A-Za-z_][A-Za-z0-9_]*', after)) and len(re.findall(r'\b%s\b' % re.escape(i), after)) == 1 and not i.isupper()]
        if bad:
            print(f"REFUSE {rel}:{n} would-unuse {bad}"); continue
        ok.append(n)
    if not apply: 
        print(f"PLAN {rel}: remove {ok}"); return
    # rebuild with only ok edits
    final = list(lines)
    for n, new, L, mac in edits:
        if n in ok: final[n-1] = None if new is None else new
    out = "\n".join(x for x in final if x is not None)
    open(p, "w", encoding="utf-8").write(out)
    print(f"APPLIED {rel}: removed {len(ok)} of {len(nums)}")
if __name__ == "__main__":
    apply = sys.argv[1] == "apply"
    spec = [l.rstrip("\n").split("\t") for l in open(sys.argv[2], encoding="utf-8") if l.strip()]
    by = {}
    for rel, n in spec: by.setdefault(rel, []).append(int(n))
    for rel, nums in by.items(): plan(rel, nums, apply)
