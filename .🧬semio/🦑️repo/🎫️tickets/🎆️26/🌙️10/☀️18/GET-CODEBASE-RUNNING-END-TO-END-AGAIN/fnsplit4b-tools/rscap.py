"""🪤 Inline format captures of Rust format macros: `{name}`, `{name:?}` and `{:width$}` read the identifier without a token the lexer sees.
usage: rscap.py <file.rs>...   lists every function whose format strings capture an identifier that has no other token in that function (a captured outer local that was not passed)."""
import re, sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import rsfn

FMT = {"format", "write", "writeln", "print", "println", "eprint", "eprintln", "panic", "assert", "assert_eq", "assert_ne", "debug_assert", "debug_assert_eq", "unreachable", "todo", "unimplemented", "format_args", "info", "warn", "error", "debug", "trace", "expect"}
CAP = re.compile(r"(?<!\{)\{([A-Za-z_]\w*)(?::[^{}]*)?\}(?!\})")
DOL = re.compile(r"[:<>^+#0-9.]*?([A-Za-z_]\w*)\$")


def in_format_macro(toks, q):
    d = 0
    for i in range(q - 1, max(-1, q - 600), -1):
        t = toks[i][1]
        if toks[i][0] == "str": continue
        if t in ")]}": d += 1
        elif t in "([{":
            if d == 0:
                return t == "(" and i >= 2 and toks[i - 1][1] == "!" and toks[i - 2][1] in FMT
            d -= 1
    return False


def captures(toks, q):
    s = toks[q][1]
    if not s.startswith('"') and not s.startswith("r"): return set()
    if not in_format_macro(toks, q): return set()
    names = set(CAP.findall(s))
    for m in re.finditer(r"\{[^{}]*\}", s):
        inner = m.group(0)[1:-1]
        if ":" in inner:
            for mm in re.finditer(r"([A-Za-z_]\w*)\$", inner.split(":", 1)[1]): names.add(mm.group(1))
    return names


def audit(path):
    text = open(path, encoding="utf-8").read()
    toks = rsfn.lex(text)
    bad = []
    k = 0
    while k < len(toks) - 1:
        if toks[k][1] == "fn" and toks[k][0] == "id" and toks[k + 1][0] == "id":
            try:
                kk, op, cl = rsfn.find_fn(toks, toks[k + 1][1], sum(1 for x in range(k) if toks[x][1] == "fn" and toks[x + 1][1] == toks[k + 1][1]))
            except Exception:
                k += 1; continue
            ids = {toks[x][1] for x in range(kk, cl + 1) if toks[x][0] == "id"}
            for x in range(kk, cl + 1):
                if toks[x][0] == "str":
                    for n in captures(toks, x):
                        if n not in ids and not n[0].isupper(): bad.append((toks[kk + 1][1], n, toks[x][2]))
            k = op + 1
        else:
            k += 1
    return bad


if __name__ == "__main__":
    total = 0
    for p in sys.argv[1:]:
        for fn, n, line in audit(p):
            total += 1
            print("%s:%d fn %s captures `%s` with no other token" % (p[-70:], line, fn, n))
    print("capture problems:", total)
