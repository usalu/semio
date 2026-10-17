"""🔁 Rewrites a mutual `expect(A).toMatchObject(B);expect(B).toMatchObject(A);` pair to the original one-direction `expect<typeof B>(A).toEqual(B);` (A = actual, B = expected). usage: from tt6_pairs import pair; pair(path, actual, expected, typed=True)"""
def pair(path, a, b, typed=True, count=1):
    t = open(path, encoding="utf8", newline="").read()
    import re
    pat = re.compile(r"expect\(" + re.escape(a) + r"\)\.toMatchObject\(" + re.escape(b) + r"\);\s*expect\(" + re.escape(b) + r"\)\.toMatchObject\(" + re.escape(a) + r"\);")
    pat2 = re.compile(r"expect\(" + re.escape(b) + r"\)\.toMatchObject\(" + re.escape(a) + r"\);\s*expect\(" + re.escape(a) + r"\)\.toMatchObject\(" + re.escape(b) + r"\);")
    new = f"expect({a}).toEqual({b});" if typed == "plain" else f"expect<typeof {b}>({a}).toEqual({b});" if typed else f"expect({a}).toEqual<typeof {b}>({b});"
    t2, n1 = pat.subn(new, t)
    t2, n2 = pat2.subn(new, t2)
    assert n1 + n2 == count, f"{path[-60:]}: {a}/{b} matched {n1}+{n2}, wanted {count}"
    open(path, "w", encoding="utf8", newline="").write(t2)
