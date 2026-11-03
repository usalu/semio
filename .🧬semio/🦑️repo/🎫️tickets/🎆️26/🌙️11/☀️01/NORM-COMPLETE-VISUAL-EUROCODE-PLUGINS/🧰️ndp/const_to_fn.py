import re, sys
def convert(path, const, fn, ident, count=0, use_files=(), crate="semio_s_plugin_norm_compliance"):
    s = open(path).read()
    pat = re.compile(r"(?P<pub>pub(?:\([a-z]+\))? )?const " + const + r": (?P<ty>f64|\[f64; \d+\]) = [^;]+;")
    m = pat.search(s)
    assert m, (path, const)
    pub = m.group("pub") or ""
    if m.group("ty") == "f64":
        body = f'{crate}::ndp_common("{ident}")'
        sig = "f64"
    else:
        n = int(re.search(r"\d+", m.group("ty")).group(0))
        body = "[" + ", ".join(f'{crate}::ndp_common("{ident}.{i + 1}")' for i in range(n)) + "]"
        sig = m.group("ty")
    indent = re.search(r"(?m)^([ \t]*)[^\n]*" + re.escape(m.group(0)[:20]), s)
    ind = s[s.rfind("\n", 0, m.start()) + 1:m.start()]
    ind = ind[: len(ind) - len(ind.lstrip())]
    s = s.replace(m.group(0), f"{pub}fn {fn}() -> {sig} {{\n{ind}    {body}\n{ind}}}", 1)
    open(path, "w").write(s)
    for p in {path, *use_files}:
        t = open(p).read()
        t = t.replace(f"[`{const}`]", f"[`{fn}`]")
        t = re.sub(r"(?<![A-Za-z0-9_])" + const + r"(?![A-Za-z0-9_])", fn + "()", t)
        t = t.replace(f"fn {fn}()() ->", f"fn {fn}() ->")
        open(p, "w").write(t)
if __name__ == "__main__":
    a = sys.argv
    convert(a[1], a[2], a[3], a[4], 0, a[5:])
