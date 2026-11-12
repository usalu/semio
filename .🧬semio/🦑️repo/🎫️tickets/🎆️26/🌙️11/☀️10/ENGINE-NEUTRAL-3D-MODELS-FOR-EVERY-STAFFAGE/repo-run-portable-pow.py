"""Rewrites `a ** b` (b not 2, a not 10 or 2) to `portable.pow(a, b)` in production sources and adds the import; prints each rewritten line."""
import os, re, sys
R = "/Users/ueli/Documents/semio"
PORTABLE = f"{R}/🧰️framework/🔨️modules/🧊️3d/🗿️model/🧮️math/🎯️portable/🟦️.ts"
roots = [f"{R}/🎓️teaching/🏛️architecture/🧍️staffage/{d}" for d in ("🔨️modules", "🎯️targets", "🌐️site", "🔮️oracles")] + [f"{R}/🧰️framework/🔨️modules/🧊️3d/🗿️model"]
skip = {"🧪️tests", "🧫️fixtures", "node_modules", "🎯️portable", "📤️dist"}
def right(s, i):
    while s[i] == " ": i += 1
    if s[i] == "(":
        d = 0; j = i
        while True:
            d += (s[j] == "(") - (s[j] == ")"); j += 1
            if d == 0: return s[i:j], j
    m = re.match(r"-?[\w.]+", s[i:]); return m.group(0), i + m.end()
def left(s, i):
    j = i
    while s[j - 1] == " ": j -= 1
    end = j
    while True:
        if s[j - 1] in ")]":
            close = s[j - 1]; opn = "(" if close == ")" else "["; d = 0
            while True:
                j -= 1; d += (s[j] == close) - (s[j] == opn)
                if d == 0: break
            continue
        k = j
        while k > 0 and (s[k - 1].isalnum() or s[k - 1] in "_.!$"): k -= 1
        if k == j: break
        j = k
        if j > 0 and s[j - 1] in ")]": continue
        break
    return s[j:end], j
def rewrite_line(line):
    changed = False
    while True:
        pos = [m.start() for m in re.finditer(r" \*\* ", line)]
        done = False
        for p in reversed(pos):
            r, rj = right(line, p + 1 + 2)
            l, lj = left(line, p)
            if r == "2" or l in ("10", "2"): continue
            line = line[:lj] + f"portable.pow({l}, {r})" + line[rj:]
            changed = done = True
            break
        if not done: return line, changed
for root in roots:
    for d, ds, fs in os.walk(root):
        ds[:] = [x for x in ds if x not in skip]
        for f in fs:
            if not f.endswith((".ts", ".tsx")): continue
            path = os.path.join(d, f); s = open(path, encoding="utf8").read()
            if " ** " not in s: continue
            lines = s.split("\n"); any_change = False
            for n, line in enumerate(lines):
                st = line.lstrip()
                if st.startswith(("*", "//", "/*")) or " ** " not in line: continue
                new, ch = rewrite_line(line)
                if ch: lines[n] = new; any_change = True; print(path.replace(R + "/", "") + ":" + str(n + 1) + "\n  - " + line.strip()[:200] + "\n  + " + new.strip()[:200])
            if not any_change: continue
            s2 = "\n".join(lines)
            if "import * as portable from" not in s2:
                last = max([i for i, l in enumerate(lines) if re.match(r'^(?:import\b.*|\}.*)\bfrom\s+"[^"]+";?\s*$', l)] or [-1])
                rel = os.path.relpath(PORTABLE, os.path.dirname(path))
                if not rel.startswith("."): rel = "./" + rel
                lines.insert(last + 1, f'import * as portable from "{rel}";'); s2 = "\n".join(lines)
            open(path, "w", encoding="utf8").write(s2)
