"""✂️ Asserted-anchor replacement helper. usage in python: from tt5_edit import edit; edit(path, [(old, new), ...]) — every old must occur exactly once."""
def edit(path, pairs):
    t = open(path, encoding="utf8", newline="").read()
    for old, new in pairs:
        n = t.count(old)
        assert n == 1, f"{path[-60:]}: anchor occurs {n}x: {old[:80]!r}"
        t = t.replace(old, new)
    open(path, "w", encoding="utf8", newline="").write(t)


def wrap_actuals(path, endswith, wrapper, only_lines=None):
    """Rewrites `expect(A)` to `expect(wrapper(A))` where A ends with `endswith`; wrapper contains `{}`. Balanced scan, assumes parens in strings are balanced."""
    t = open(path, encoding="utf8", newline="").read()
    out, i, count = [], 0, 0
    line_of = lambda pos: t.count("\n", 0, pos) + 1
    while True:
        j = t.find("expect(", i)
        if j < 0:
            out.append(t[i:]); break
        if j > 0 and (t[j - 1].isalnum() or t[j - 1] in "_$."):
            out.append(t[i:j + 7]); i = j + 7; continue
        k, depth = j + 7, 1
        while depth:
            c = t[k]
            depth += (c == "(") - (c == ")")
            k += 1
        actual = t[j + 7:k - 1]
        if actual.endswith(endswith) and (only_lines is None or line_of(j) in only_lines):
            out.append(t[i:j + 7] + wrapper.replace("{}", actual) + ")"); count += 1
        else:
            out.append(t[i:k])
        i = k
    open(path, "w", encoding="utf8", newline="").write("".join(out))
    return count
