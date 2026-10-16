# ✂️ Span editing for Rust staging files: leading-comment-aware item and member spans, region stacks, balanced cut and paste.
import re, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import shellsrc as S
from blank import blank

REG_OPEN = re.compile(r"^\s*//#region\s*(.*)$")
REG_CLOSE = re.compile(r"^\s*//#endregion\s*(.*)$")

class Doc:
    def __init__(self, text):
        self.lines = text.split("\n")
        self.reload()

    def reload(self):
        self.blank = blank("\n".join(self.lines)).split("\n")
        st = []; self.stack = []
        for l in self.lines:
            self.stack.append(tuple(st))
            m = REG_OPEN.match(l)
            if m: st.append(m.group(1).strip())
            elif REG_CLOSE.match(l):
                if st: st.pop()
        self.stack.append(tuple(st))

    def text(self): return "\n".join(self.lines)

    def lead(self, s):
        while s > 0:
            p = self.lines[s - 1].strip()
            if p.startswith("//") and not REG_OPEN.match(self.lines[s - 1]) and not REG_CLOSE.match(self.lines[s - 1]): s -= 1
            elif p.startswith("#["): s -= 1
            else: break
        return s

    def items(self):
        out = []
        for it in S.outline(self.text()):
            s0 = it["start"] - 1; e0 = it["end"] - 1
            out.append(dict(it, s=self.lead(s0), e=e0))
        return out

    def members(self, impl):
        raw = "\n".join(self.lines[impl["s"]:impl["e"] + 1])
        base = impl["s"]
        out = []
        for m in S.methods(raw):
            s0 = base + m["start"] - 1
            s = self.lead_member(s0)
            out.append(dict(name=m["name"], s=s, e=base + m["end"] - 1, sig=m["sig"], body=m["body"], impl=impl))
        return out

    def lead_member(self, s):
        while s > 0:
            p = self.lines[s - 1].strip()
            if p.startswith("//") and not REG_OPEN.match(self.lines[s - 1]) and not REG_CLOSE.match(self.lines[s - 1]): s -= 1
            elif p.startswith("#["): s -= 1
            else: break
        return s

    def balanced(self, s, e):
        return self.stack[s] == self.stack[e + 1]

    def cut(self, s, e):
        txt = self.lines[s:e + 1]
        del self.lines[s:e + 1]
        if s < len(self.lines) and s > 0 and self.lines[s].strip() == "" and self.lines[s - 1].strip() == "": del self.lines[s]
        self.reload()
        return txt

def wrap_regions(stack, body_lines):
    out = []
    for r in stack: out.append("//#region " + r)
    out += body_lines
    for r in reversed(stack): out.append("//#endregion " + r)
    return out

def impl_attrs(doc, impl):
    s = impl["s"]
    out = []
    k = s
    while k < len(doc.lines) and not doc.lines[k].lstrip().startswith("impl"):
        if doc.lines[k].lstrip().startswith("#["): out.append(doc.lines[k].strip())
        k += 1
    return out
