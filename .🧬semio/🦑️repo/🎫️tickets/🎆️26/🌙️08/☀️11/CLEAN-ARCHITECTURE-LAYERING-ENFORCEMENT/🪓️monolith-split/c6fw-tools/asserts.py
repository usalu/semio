"""🔎️ C5G assertion probe: every `<var>.includes("lit")` / `expect(<var>).toContain("lit")` literal of the re-pointed readers, with the corpus it must hold in. usage: asserts.py"""
import re, sys, os
sys.path.insert(0, ".")
import modtree as M
R = M.REPO + "/"
H = R + "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/"
def T(d): return M.tree(H + d + "/🦀️.rs")
def F(p): return open(H + p, encoding="utf-8").read()
corp = {
  "hg": T("🎭️guest-runtime"), "hr": T("🧭️routers"), "he": T("⚡️effects"), "hs": T("🧵️shard"),
  "root": F("🦀️.rs"), "relay": F("🔀️post-turn-relay/🦀️.rs"), "engine": F("🔧️engine/🦀️.rs"),
}
corp["hostlist"] = corp["root"] + "\n" + corp["hg"] + "\n" + corp["relay"] + "\n" + corp["hr"]
corp["hostlist_e"] = corp["hostlist"] + "\n" + corp["engine"]
LIT = re.compile(r'(?:(\w+)\.includes\("((?:[^"\\]|\\.)*)"\)\)\.toBe\((true|false)\)|expect\((\w+)\)\.(not\.)?toContain\("((?:[^"\\]|\\.)*)"\))')
def un(s): return s.encode().decode("unicode_escape") if "\\" in s else s
def run(path, name_of_corpus):
    t = open(R + path, encoding="utf-8").read()
    bad = 0; n = 0
    for m in LIT.finditer(t):
        if m.group(1):
            var, lit, exp = m.group(1), un(m.group(2)), m.group(3) == "true"
        else:
            var, lit, exp = m.group(4), un(m.group(6)), m.group(5) is None
        c = name_of_corpus.get(var)
        if c is None: continue
        n += 1
        got = lit in corp[c]
        if got != exp:
            bad += 1; print("  FAIL", var, "->", c, "expect", exp, "got", got, repr(lit[:90]))
    print(path.split("/")[-3] + "/" + path.split("/")[-2], "checked", n, "bad", bad)
if __name__ == "__main__":
    pass
