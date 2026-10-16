"""📐 Item model of a Rust source file: top-level items with attached comments, attributes and region stacks.

Brace matching runs on comment/string-stripped text, so ranges are exact (unlike the line-heuristic `outline.py`).
"""
import re
from dataclasses import dataclass, field

from rs import strip

KIND_RE = re.compile(
    r'^(?P<vis>pub(?:\([^)]*\))?\s+)?(?:default\s+)?(?P<mods>(?:unsafe\s+|async\s+|const\s+|extern\s+"[^"]*"\s+|extern\s+)*)'
    r'(?P<kind>fn|struct|enum|trait|impl|type|static|const|mod|macro_rules!|use|union|extern\s+crate)\b\s*(?P<rest>.*)$',
    re.S)
NAME_RE = re.compile(r'\s*(?:mut\s+)?([A-Za-z_][A-Za-z0-9_]*)')


@dataclass
class Item:
    kind: str
    name: str
    vis: str
    start: int
    end: int
    header: int
    attrs: list
    self_type: str = ''
    trait: str = ''
    regions: tuple = ()
    cfg_test: bool = False
    gap_before: list = field(default_factory=list)

    @property
    def lines(self):
        return self.end - self.start + 1


def find_close(st, i, open_ch, close_ch):
    depth = 0
    n = len(st)
    while i < n:
        c = st[i]
        if c == open_ch:
            depth += 1
        elif c == close_ch:
            depth -= 1
            if depth == 0:
                return i
        i += 1
    raise ValueError('unbalanced')


def item_end_offset(st, off, kind):
    """Offset (inclusive) of the last char of the item whose header starts at `off` in stripped text."""
    i = off
    n = len(st)
    depth = {'(': 0, '[': 0, '{': 0}
    semi_kinds = kind in ('const', 'static', 'type', 'use', 'extern crate')
    while i < n:
        c = st[i]
        if c in '([{':
            if c == '{' and not semi_kinds and sum(depth.values()) == 0:
                j = find_close(st, i, '{', '}')
                if kind in ('macro_rules!',):
                    k = j + 1
                    return k - 1
                return j
            depth[c] += 1
        elif c in ')]}':
            depth[{')': '(', ']': '[', '}': '{'}[c]] -= 1
        elif c == ';' and sum(depth.values()) == 0:
            return i
        i += 1
    raise ValueError('item end not found')


def parse_items(text):
    """Top-level items. Gap lines between items (comments, markers, blanks) are attached to the next item as `gap_before`."""
    st = strip(text)
    lines = text.split('\n')
    slines = st.split('\n')
    offs = [0]
    for l in lines:
        offs.append(offs[-1] + len(l) + 1)
    items = []
    i = 0
    n = len(lines)
    region_stack = []
    pending_gap = []
    pending_attr_start = None
    pending_attrs = []
    while i < n:
        raw = lines[i]
        s = slines[i]
        if raw.strip() == '':
            if pending_attr_start is not None:
                pending_gap.extend(lines[pending_attr_start:i])
                pending_attr_start = None
                pending_attrs = []
            pending_gap.append(raw)
            i += 1
            continue
        if raw.startswith('//#region') or raw.startswith('//#endregion'):
            if pending_attr_start is not None:
                pending_gap.extend(lines[pending_attr_start:i])
                pending_attr_start = None
                pending_attrs = []
            if raw.startswith('//#region'):
                region_stack.append(raw)
            else:
                if region_stack:
                    region_stack.pop()
            pending_gap.append(raw)
            i += 1
            continue
        if pending_attr_start is not None and raw.startswith((' ', '\t')) and raw.lstrip().startswith('//'):
            i += 1
            continue
        if raw.startswith('//') and not raw.startswith('///') and not raw.startswith('//!'):
            if pending_attr_start is None:
                pending_attr_start = i
            i += 1
            continue
        if raw.startswith('//!'):
            if pending_attr_start is not None:
                pending_gap.extend(lines[pending_attr_start:i])
                pending_attr_start = None
                pending_attrs = []
            pending_gap.append(raw)
            i += 1
            continue
        if raw.startswith('///'):
            if pending_attr_start is None:
                pending_attr_start = i
            i += 1
            continue
        if raw.startswith('#!['):
            if pending_attr_start is not None:
                pending_gap.extend(lines[pending_attr_start:i])
                pending_attr_start = None
                pending_attrs = []
            pending_gap.append(raw)
            i += 1
            continue
        if raw.startswith('#['):
            if pending_attr_start is None:
                pending_attr_start = i
            j = offs[i] + raw.index('#[') + 1
            close = find_close(st, j, '[', ']')
            endline = st.count('\n', 0, close)
            pending_attrs.append('\n'.join(lines[i:endline + 1]))
            i = endline + 1
            continue
        m = KIND_RE.match(s)
        if m and not raw.startswith((' ', '\t', '}', ')', ']')):
            kind = re.sub(r'\s+', ' ', m.group('kind'))
            header_off = offs[i]
            try:
                end_off = item_end_offset(st, header_off, kind)
            except ValueError as e:
                raise ValueError(f'line {i + 1}: {e}')
            end_line = st.count('\n', 0, end_off)
            rest = m.group('rest')
            name = ''
            self_type = ''
            trait = ''
            if kind == 'impl':
                head = st[header_off:header_off + 400].split('{')[0]
                head = re.sub(r'^\s*(?:pub\s+)?(?:unsafe\s+)?impl\s*', '', head)
                if head.startswith('<'):
                    d = 0
                    for k, ch in enumerate(head):
                        if ch == '<':
                            d += 1
                        elif ch == '>':
                            d -= 1
                            if d == 0:
                                head = head[k + 1:]
                                break
                head = re.sub(r'\s+where\s.*$', '', head.strip(), flags=re.S)
                if ' for ' in head:
                    trait_s, ty = head.split(' for ', 1)
                    trait = re.sub(r'<.*$', '', trait_s.strip()).split('::')[-1].strip('! ')
                else:
                    ty = head
                tm = re.match(r'[&\s]*(?:mut\s+)?(?:dyn\s+)?([A-Za-z_][A-Za-z0-9_:]*)', ty.strip())
                self_type = tm.group(1).split('::')[-1] if tm else ''
                name = f'impl {trait + " for " if trait else ""}{self_type}'
            elif kind == 'use':
                name = rest.strip()[:60]
            else:
                nm = NAME_RE.match(rest)
                name = nm.group(1) if nm else ''
            start = pending_attr_start if pending_attr_start is not None else i
            attrs = pending_attrs
            cfg_test = any(re.search(r'cfg\(\s*test\s*\)', a) and 'not(' not in a for a in attrs)
            it = Item(kind=kind, name=name, vis=(m.group('vis') or '').strip(), start=start, end=end_line, header=i, attrs=list(attrs),
                      self_type=self_type, trait=trait, regions=tuple(region_stack), cfg_test=cfg_test, gap_before=pending_gap)
            items.append(it)
            pending_gap = []
            pending_attr_start = None
            pending_attrs = []
            i = end_line + 1
            # trailing same-line markers none
            continue
        # unknown top-level construct (macro invocation etc.)
        try:
            if re.match(r'[A-Za-z_][A-Za-z0-9_:]*!', s):
                bang = st.index('!', offs[i])
                brace = st[bang + 1:bang + 40].lstrip().startswith('{')
                end_off = item_end_offset(st, offs[i], 'macro_rules!' if brace else 'const')
                end_line = st.count('\n', 0, end_off)
                start = pending_attr_start if pending_attr_start is not None else i
                mname = s.split('!')[0]
                kind_m = 'macro_call'
                if mname.endswith('thread_local'):
                    sm = re.search(r'\bstatic\s+([A-Za-z_][A-Za-z0-9_]*)', '\n'.join(slines[i:end_line + 1]))
                    if sm:
                        mname, kind_m = sm.group(1), 'static'
                items.append(Item(kind=kind_m, name=mname, vis='', start=start, end=end_line, header=i, attrs=list(pending_attrs), regions=tuple(region_stack), gap_before=pending_gap, cfg_test=any(re.search(r'cfg\(\s*test\s*\)', a) and 'not(' not in a for a in pending_attrs)))
                pending_gap = []
                pending_attr_start = None
                pending_attrs = []
                i = end_line + 1
                continue
        except ValueError:
            pass
        raise ValueError(f'unparsed top-level line {i + 1}: {raw[:100]!r}')
    return items, pending_gap, lines


def region_name(marker_line):
    return re.sub(r'^//#(?:end)?region\s*', '', marker_line).strip()
