import os, re, sys, hashlib
ROOT = "/Users/ueli/Documents/semio"
def read(p):
    return open(os.path.join(ROOT, p), encoding="utf-8").read()
def write(p, text):
    full = os.path.join(ROOT, p)
    assert os.path.realpath(full).startswith(ROOT + "/")
    tmp = full + ".cpf.tmp"
    open(tmp, "w", encoding="utf-8").write(text)
    os.replace(tmp, full)
def cut(text, anchor, repl="", count=1):
    assert text.count(anchor) == count, (anchor[:80], text.count(anchor))
    return text.replace(anchor, repl)

def match_paren(t, i):
    """index of the closer matching the opener at t[i]; string/template/regex-literal aware enough for single-line minified TS."""
    op = t[i]; cl = {"(": ")", "{": "}", "[": "]"}[op]
    depth = 0; j = i; n = len(t)
    while j < n:
        c = t[j]
        if c in "\"'`":
            q = c; j += 1
            while j < n and t[j] != q:
                if t[j] == "\\": j += 1
                elif q == "`" and t[j] == "$" and j + 1 < n and t[j+1] == "{":
                    j = match_paren(t, j + 1)
                j += 1
        elif c == "/" and j + 1 < n and t[j+1] == "/" :
            j = t.index("\n", j)
        elif c in "([{":
            depth += 1
        elif c in ")]}":
            depth -= 1
            if depth == 0:
                assert c == cl, (c, cl, t[i:i+80]); return j
        j += 1
    raise AssertionError("unbalanced")
