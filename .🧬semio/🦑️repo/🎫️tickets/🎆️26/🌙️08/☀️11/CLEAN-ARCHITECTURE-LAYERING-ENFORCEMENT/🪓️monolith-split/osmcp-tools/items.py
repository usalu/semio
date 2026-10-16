import re, sys, collections
sys.path.insert(0, ".")
from prod import *

KW = r'(?:pub(?:\s*\([^)]*\))?\s+)?(?:default\s+)?(?:unsafe\s+)?(?:async\s+)?(?:const\s+)?(?:unsafe\s+)?(?:extern\s+"[^"]*"\s+)?'
HEAD_RE = re.compile(r'\s*(?:#\[[^\]]*\]\s*)*(?P<vis>pub(?:\s*\([^)]*\))?\s+)?(?:(?:default|unsafe|async|const|extern(?:\s+"[^"]*")?)\s+)*(?P<kind>fn|struct|enum|trait|type|const|static|impl|mod|use|macro_rules|union|extern|unsafe)\b\s*!?\s*(?P<name>[A-Za-z_]\w*)?')

class Item:
    __slots__ = ("file","start","end","kind","name","vis","text","attrs","cfg","impl_trait","impl_self","line0","line1","id","refs")
    def __repr__(self):
        return f"<{self.kind} {self.name} {self.line0}-{self.line1}>"

def split_items(f):
    """Split file code into top-level items; returns list of Item with absolute offsets in f.code/f.src."""
    code = f.code
    n = len(code)
    items = []
    i = 0
    while i < n:
        # skip whitespace
        while i < n and code[i].isspace(): i += 1
        if i >= n: break
        start = i
        # consume leading attributes
        j = i
        attrs_end = j
        while True:
            k = j
            while k < n and code[k].isspace(): k += 1
            if code.startswith('#[', k) or code.startswith('#![', k):
                d = 0
                while k < n:
                    if code[k] == '[': d += 1
                    elif code[k] == ']':
                        d -= 1
                        if d == 0: k += 1; break
                    k += 1
                j = k
            else:
                break
        # item body
        depth = 0
        k = j
        end = None
        while k < n:
            c = code[k]
            if c in '{([': depth += 1
            elif c in '})]':
                depth -= 1
                if depth == 0 and c == '}':
                    # look ahead for ';'
                    m = k + 1
                    while m < n and code[m] in ' \t': m += 1
                    if m < n and code[m] == ';':
                        end = m + 1
                    else:
                        end = k + 1
                    break
                if depth < 0:
                    end = k; break
            elif c == ';' and depth == 0:
                end = k + 1; break
            k += 1
        if end is None: end = n
        it = Item()
        it.file = f; it.start = start; it.end = end
        it.text = code[start:end]
        hm = HEAD_RE.match(code, start)
        it.kind = hm.group('kind') if hm else 'other'
        it.name = hm.group('name') if hm else None
        it.vis = (hm.group('vis') or '').strip() if hm else ''
        attrs = re.findall(r'#!?\[(?:[^\[\]]|\[[^\]]*\])*\]', f.src[start:j])
        it.attrs = attrs
        it.cfg = [a for a in attrs if a.startswith('#[cfg')]
        if it.kind == 'other' or it.name is None and it.kind not in ('use', 'impl'):
            body = code[j:end]
            mm = re.match(r'\s*dyn_enum_close\s*!\s*\{\s*(?:#\[[^\]]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?enum\s+([A-Za-z_]\w*)', body)
            if mm:
                it.kind = 'enum'; it.name = mm.group(1); it.vis = 'pub'
            else:
                mm = re.match(r'\s*thread_local\s*!\s*\{[^}]*?static\s+([A-Za-z_]\w*)', body)
                if mm: it.kind = 'static'; it.name = mm.group(1)
                else:
                    it.kind = 'other'
        it.impl_trait = it.impl_self = None
        if it.kind == 'impl':
            hdr = code[j:end]
            body_at = hdr.find('{')
            hdr = hdr[:body_at] if body_at >= 0 else hdr
            hdr = re.sub(r'^\s*(?:unsafe\s+)?impl\s*(<[^{]*?>\s+)?(?=[A-Za-z_&(\[!:])', 'impl ', hdr, count=1) if False else hdr
            m = re.match(r'\s*(?:pub\s+)?(?:unsafe\s+)?impl\b', hdr)
            rest = hdr[m.end():] if m else hdr
            rest = rest.strip()
            # strip generics
            if rest.startswith('<'):
                d = 0; q = 0
                for q, ch in enumerate(rest):
                    if ch == '<': d += 1
                    elif ch == '>':
                        d -= 1
                        if d == 0: break
                rest = rest[q+1:].strip()
            if re.search(r'\bfor\b', rest):
                tr, sf = re.split(r'\bfor\b', rest, maxsplit=1)
                it.impl_trait = tr.strip(); it.impl_self = re.split(r'\bwhere\b', sf)[0].strip()
            else:
                it.impl_self = re.split(r'\bwhere\b', rest)[0].strip()
        it.line0 = f.src.count('\n', 0, start) + 1
        it.line1 = f.src.count('\n', 0, end) + 1
        items.append(it)
        i = end
    return items

if __name__ == "__main__":
    files = walk(LIBROOT, ["crate"])
    tot = 0
    for p, f in files.items():
        its = split_items(f)
        tot += len(its)
    print("files", len(files), "items", tot)
    f = files[MCP + "/🧭️protocol/🦀️.rs"]
    for it in split_items(f)[:50]:
        print(it.line0, it.line1, it.kind, it.name, it.vis, it.impl_trait, it.impl_self, it.cfg)
