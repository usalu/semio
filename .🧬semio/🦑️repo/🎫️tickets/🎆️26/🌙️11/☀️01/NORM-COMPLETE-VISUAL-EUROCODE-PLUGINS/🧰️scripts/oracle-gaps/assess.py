"""🔎 Extracts the check-id expressions of a code's artifact: first argument of `CheckResult::assess(` plus (to a fixed point) the id argument of every function that forwards a parameter into it."""
import glob, os, re
ROOT = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts"
FN = re.compile(r"\bfn\s+(\w+)\s*(?:<[^()]*?>)?\s*\(")

def split_args(text, i):
    args, depth, in_s, j, start = [], 0, False, i, i
    while j < len(text):
        c = text[j]
        if in_s:
            if c == "\\": j += 1
            elif c == '"': in_s = False
        elif c == '"': in_s = True
        elif c == "'" and j + 2 < len(text) and text[j + 2] == "'": j += 2
        elif c in "([{": depth += 1
        elif c in ")]}":
            if depth == 0:
                args.append(text[start:j].strip()); return args
            depth -= 1
        elif c == "," and depth == 0:
            args.append(text[start:j].strip()); start = j + 1
        elif c == "<" and False: pass
        j += 1
    return args

def files(code):
    for dp, dn, fn in os.walk(glob.glob(f"{ROOT}/*{code}")[0]):
        if "🧪️tests" in dp or "/target" in dp: continue
        for f in fn:
            if f.endswith(".rs"): yield os.path.join(dp, f)

def params(text, m):
    ps = split_args(text, m.end())
    return [re.split(r"\s*:", p.replace("mut ", ""), maxsplit=1)[0].strip() for p in ps]

def ident_of(expr):
    m = re.fullmatch(r"(\w+)(?:\.(?:into|to_string|to_owned|clone)\(\))?", expr)
    return m.group(1) if m else None

def calls(code):
    texts = {p: open(p, encoding="utf8").read() for p in files(code)}
    sites, wrappers = [], {}
    work = [("CheckResult::assess", 0, None)]
    seen = set()
    while work:
        name, idx, _ = work.pop()
        if name in seen: continue
        seen.add(name)
        pat = re.compile(r"(?<![\w])" + re.escape(name) + r"\s*\(") if "::" in name else re.compile(r"(?<![\w])(?<!fn )" + re.escape(name) + r"\s*\(")
        for p, t in texts.items():
            for m in pat.finditer(t):
                if "::" not in name and re.search(r"fn\s+$", t[max(0, m.start() - 4):m.start()]): continue
                args = split_args(t, m.end())
                if len(args) <= idx: continue
                expr = re.sub(r"\s+", " ", args[idx])
                id_ = ident_of(expr)
                fns = [f for f in FN.finditer(t, 0, m.start())]
                f = fns[-1] if fns else None
                if id_ and f:
                    ps = params(t, f)
                    if id_ in ps and f.group(1) != name:
                        work.append((f.group(1), ps.index(id_), None)); continue
                if id_ and not expr.startswith(("format!", "&format!", '"')):
                    lets = list(re.finditer(r"let\s+(?:mut\s+)?" + re.escape(id_) + r"\s*(?::[^=;]+)?=\s*", t[: m.start()]))
                    if lets:
                        expr = re.sub(r"\s+", " ", split_args(t, lets[-1].end() - 1 + 1 - 1 + 0)[0] if False else t[lets[-1].end(): t.index(";", lets[-1].end())]).strip()
                cm = re.fullmatch(r'&?(\w+)\("([^"]+)"\)', expr)
                if cm:
                    cl = list(re.finditer(r"let\s+" + cm.group(1) + r"\s*=\s*\|\w+(?::\s*&str)?\|\s*format!\(\"([^\"]*)\"", t[: m.start()]))
                    if cl:
                        expr = 'format!("' + re.sub(r"\{\w+\}", cm.group(2), cl[-1].group(1), count=1) + '")'
                sites.append((p.split("✳️any/")[-1], t.count("\n", 0, m.start()) + 1, name.split("::")[-1], expr))
    return sites

if __name__ == "__main__":
    for c in ["en%d" % n for n in range(1990, 2000)]:
        cs = calls(c)
        lit = [x for x in cs if x[3].startswith(('format!("', '"'))]
        print(c, len(cs), "literal/format", len(lit))

STR = re.compile(r'"((?:en\d{4}(?:-\d)?)\.[^"]*)"')

FMT = re.compile(r'format!\("((?:en\d{4}(?:-\d)?)\.[^"]*)"')

def patterns(code):
    pats, unresolved = {}, []
    site_files = set()
    for f, line, via, expr in calls(code):
        site_files.add(f)
        m = STR.search(expr)
        if m and expr.lstrip("&").startswith(('format!', '"')):
            pats.setdefault(m.group(1), []).append(f"{f}:{line}")
        else:
            unresolved.append((f, line, via, expr[:90]))
    for p in files(code):
        if p.split("✳️any/")[-1] in site_files:
            t = open(p, encoding="utf8").read()
            for m in FMT.finditer(t):
                pats.setdefault(m.group(1), []).append(p.split("✳️any/")[-1] + ":" + str(t.count("\n", 0, m.start()) + 1) + " (literal)")
    return pats, unresolved
