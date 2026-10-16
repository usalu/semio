# 🌐️ External consumers of the WP5 use cases: `<shell>.<use_case>(…)` becomes `<Ctx>::of(&mut <shell>).<use_case>(…)` in tests and in other packages' files.
import os, re, sys, subprocess, collections, json
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import cfg, ctx, deps
from blank import blank

def method_table(root):
    ns = deps.load(root); tab = {}
    for n in ns:
        if n.kind == "method": tab[n.name] = n.crate
    return tab

def match_close(b, i):
    d = 0
    while i < len(b):
        if b[i] in "([{": d += 1
        elif b[i] in ")]}":
            d -= 1
            if d == 0: return i
        i += 1
    return -1

def shell_vars(raw, b):
    helpers = set(re.findall(r"fn\s+([A-Za-z_]\w*)\s*(?:<[^>]*>)?\s*\([^)]*\)\s*->\s*(?:[A-Za-z_]\w*::)*ShellState\b", b))
    own = {"shell"}; refs = set()
    hp = "|".join(sorted(helpers | {"ShellState::new"}, key=len, reverse=True))
    for m in re.finditer(r"let\s+(?:mut\s+)?([A-Za-z_]\w*)\s*(?::[^=;]*)?=\s*(?:" + hp + r")\s*\(", b): own.add(m.group(1))
    for m in re.finditer(r"let\s+\(([^)]*)\)\s*=\s*\(((?:[^;]|\n)*?)\)\s*;", b):
        lhs = [x.strip().replace("mut ", "") for x in m.group(1).split(",")]
        rhs = re.split(r",\s*(?=(?:" + hp + r")\s*\(|[A-Za-z_])", m.group(2))
        for i, x in enumerate(lhs):
            if i < len(rhs) and re.match(r"\s*(?:" + hp + r")\s*\(", rhs[i]): own.add(x)
    for m in re.finditer(r"(?<![\w])(?<![\w)\]]\.)(?:mut\s+)?([A-Za-z_]\w*)\s*:\s*&\s*(mut\s+)?(?:[A-Za-z_]\w*::)*ShellState\b", b):
        (refs if m.group(2) else refs).add(m.group(1))
    for m in re.finditer(r"(?<![\w])(?<![\w)\]]\.)(?:mut\s+)?([A-Za-z_]\w*)\s*:\s*(?:[A-Za-z_]\w*::)*ShellState\b", b): own.add(m.group(1))
    return own, refs

def rewrite(raw, rel, tab, report, qualified):
    b = blank(raw)
    own, refs = shell_vars(raw, b)
    names = own | refs
    alt = "|".join(sorted(map(re.escape, names), key=len, reverse=True))
    meths = "|".join(sorted(map(re.escape, tab), key=len, reverse=True))
    rx = re.compile(r"(?<![\w.])((?:[A-Za-z_]\w*(?:\(\))?\s*\.\s*)*(?:" + alt + r"))\s*\.\s*(" + meths + r")(\s*::\s*<[^>]*>)?(\s*)\(")
    edits = []
    for m in rx.finditer(b):
        recv, name = m.group(1), m.group(2)
        crate = tab[name]; x = cfg.CTX[crate]
        pre = "crate::shell::" if qualified else ""
        root_id = re.split(r"[.\s(]", recv.strip())[0]
        last = re.split(r"\s*\.\s*", recv.strip())[-1]
        if "." not in recv.strip() and root_id in refs and root_id not in own: arg = f"&mut *{recv.strip()}"
        else: arg = f"&mut {recv.strip()}"
        lp = m.end() - 1; rp = match_close(b, lp)
        args = b[lp + 1:rp]
        if re.search(r"(?<![\w.])" + re.escape(recv.strip()) + r"(?![\w])", args):
            report.append((rel, f"receiver {recv.strip()} appears in the arguments of {name}"))
        edits.append((m.start(1), m.end(1), f"{pre}{x}::of({arg})"))
    out = raw
    for s, e, r in sorted(edits, reverse=True): out = out[:s] + r + out[e:]
    return out, len(edits)

def candidates(tab):
    names = sorted(tab)
    rxs = "[.](" + "|".join(names) + ")[[:space:]]*(::<[^>]*>)?[(]"
    r = subprocess.run(["git", "-c", "core.quotepath=false", "-c", "core.fsmonitor=false", "grep", "-l", "-E", rxs, "--", cfg.ENG], cwd=cfg.ROOT, capture_output=True, text=True)
    return sorted(l for l in r.stdout.split("\n") if l.endswith(".rs"))
