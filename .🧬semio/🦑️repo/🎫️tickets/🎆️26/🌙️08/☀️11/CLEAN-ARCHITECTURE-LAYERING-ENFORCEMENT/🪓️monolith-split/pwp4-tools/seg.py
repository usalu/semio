# ✂️ Top-level segmenter: partitions a Rust file into chunks (leading attrs/docs/comments + one top-level statement), regions and filler.
import re
from common import *

ITEM = re.compile(r'^(?:pub(?:\([^)]*\))?\s+)?(?:(?:default\s+)?(?:async|const|unsafe)\s+)*(?:extern\s+"[^"]*"\s+)?(fn|struct|enum|union|trait|type|static|const|mod|impl|macro_rules!|use|extern\s+crate)\b\s*([A-Za-z_]\w*)?')
REGION = re.compile(r'^\s*//#(region|endregion)\b(.*)$')

class Chunk:
    __slots__ = ('s', 'e', 'lines', 'kind', 'name', 'head', 'target', 'trait', 'attrs', 'vis', 'region', 'hl')
    def __init__(self, s, e, lines, kind, name, head):
        self.s, self.e, self.lines, self.kind, self.name, self.head = s, e, lines, kind, name, head
        self.target = self.trait = None; self.attrs = ''; self.vis = ''; self.region = None; self.hl = s
    def text(self): return '\n'.join(self.lines)
    def __repr__(self): return f'<{self.kind} {self.name} {self.s}-{self.e}>'

def classify(code_lines, raw_lines):
    attrs = []; head = None; hi = None
    for i, l in enumerate(code_lines):
        t = l.strip()
        if not t: continue
        if t.startswith('#['): 
            # attribute possibly multi-line: collect raw text
            attrs.append(raw_lines[i].strip()); continue
        if t.startswith('#!['): attrs.append(raw_lines[i].strip()); continue
        if head is None:
            head = t; hi = i; break
    if head is None: return 'filler', '', '', ' '.join(attrs)
    m = ITEM.match(head)
    if m:
        kind = m.group(1).replace('extern crate', 'externcrate'); name = m.group(2) or ''
        if kind == 'macro_rules!': kind = 'macro_rules'
        return kind, name, head, ' '.join(attrs)
    mm = re.match(r'^([A-Za-z_][\w:]*)!\s*[\{\(\[]', head) or re.match(r'^([A-Za-z_][\w:]*)!', head)
    if mm: return 'macro', mm.group(1), head, ' '.join(attrs)
    return 'other', '', head, ' '.join(attrs)

def segment(path=None, raw=None):
    if raw is None: raw = open(path, encoding='utf-8').read()
    code = PT.strip(raw).split('\n'); R = raw.split('\n'); n = len(R)
    if raw.endswith('\n'):
        pass
    chunks = []
    i = 0
    # file-level inner docs/attrs stay together as 'moddoc'
    j = 0
    while j < n and (R[j].startswith('//!') or R[j].startswith('#![') or not R[j].strip()):
        j += 1
    # only treat as moddoc when at least one //! or #![ line is present
    if j > 0 and any(R[k].startswith(('//!', '#![')) for k in range(j)):
        while j > 0 and not R[j - 1].strip(): j -= 1
        chunks.append(Chunk(1, j, R[0:j], 'moddoc', '', '')); i = j
    cur_start = i
    depth = [0, 0, 0]
    def flush(s, e):
        seg_raw = R[s:e + 1]; seg_code = code[s:e + 1]
        # peel region markers and leading comment/blank lines into their own chunks only when the statement is a region marker
        kind, name, head, attrs = classify(seg_code, seg_raw)
        c = Chunk(s + 1, e + 1, seg_raw, kind, name, head); c.attrs = attrs
        for q, l in enumerate(seg_code):
            tt = l.strip()
            if tt and not tt.startswith('#['):
                c.hl = s + 1 + q; break
        mm = re.match(r'^(pub(?:\([^)]*\))?)\s', head) if head else None
        c.vis = mm.group(1) if mm else ''
        if kind == 'impl':
            try: c.target, c.trait = PT.impl_target(head)
            except Exception: pass
        chunks.append(c)
    k = i
    stmt_has_code = False
    while k < n:
        l = code[k]; t = l.strip()
        rm = REGION.match(R[k])
        if rm and not stmt_has_code and depth == [0, 0, 0]:
            # region markers are chunks of their own; pending leading lines before them stay with the previous filler
            if cur_start < k: flush(cur_start, k - 1) if any(code[x].strip() for x in range(cur_start, k)) else chunks.append(Chunk(cur_start + 1, k, R[cur_start:k], 'filler', '', ''))
            c = Chunk(k + 1, k + 1, [R[k]], 'region-' + rm.group(1), rm.group(2).strip(), R[k].strip()); chunks.append(c)
            cur_start = k + 1; k += 1; continue
        if t:
            stmt_has_code = True
            for ch in l:
                if ch == '{': depth[0] += 1
                elif ch == '}': depth[0] -= 1
                elif ch == '(': depth[1] += 1
                elif ch == ')': depth[1] -= 1
                elif ch == '[': depth[2] += 1
                elif ch == ']': depth[2] -= 1
            if depth == [0, 0, 0] and (t.endswith(';') or t.endswith('}')):
                flush(cur_start, k); cur_start = k + 1; stmt_has_code = False
        k += 1
    if cur_start < n:
        tail = R[cur_start:n]
        if any(x.strip() for x in tail) :
            if any(code[x].strip() for x in range(cur_start, n)):
                flush(cur_start, n - 1)
            else:
                chunks.append(Chunk(cur_start + 1, n, tail, 'filler', '', ''))
        else:
            chunks.append(Chunk(cur_start + 1, n, tail, 'filler', '', ''))
    return chunks, R

def check(path):
    chunks, R = segment(path)
    joined = []
    prev = 0
    for c in chunks:
        assert c.s == prev + 1, (path, c, prev)
        joined.extend(c.lines); prev = c.e
    assert prev == len(R), (path, prev, len(R))
    assert joined == R
    return chunks

if __name__ == '__main__':
    import sys
    for rel in sys.argv[1:]:
        chunks = check(PLUG + rel)
        print(rel, len(chunks), 'chunks')
        for c in chunks:
            print(f'  {c.s:5d}-{c.e:5d} {c.kind:12s} {c.name[:40]:40s} {c.head[:70]}')
