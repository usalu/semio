"""✂️ Asserted-anchor replacement helper. usage: from tt6_edit import edit; edit(path, [(old, new), ...]) — every old must occur exactly once."""
def edit(path, pairs):
    t = open(path, encoding="utf8", newline="").read()
    for old, new in pairs:
        n = t.count(old)
        assert n == 1, f"{path[-60:]}: anchor occurs {n}x: {old[:80]!r}"
        t = t.replace(old, new)
    open(path, "w", encoding="utf8", newline="").write(t)
