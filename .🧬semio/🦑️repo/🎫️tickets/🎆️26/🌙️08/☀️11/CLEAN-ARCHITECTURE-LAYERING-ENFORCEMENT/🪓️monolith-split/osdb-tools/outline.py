"""🗺️ Top-level item outline of a rustfmt-formatted Rust file: kind, name, first line, last line, size.

Usage: outline.py <file> [minLines]
"""
import re
import sys

KINDS = re.compile(
    r'^(?P<attrs>(?:#\[[^\n]*\]\s*)*)'
    r'(?P<vis>pub(?:\([^)]*\))?\s+)?'
    r'(?P<unsafe>unsafe\s+)?(?P<async>async\s+)?(?P<const>const\s+)?'
    r'(?P<kind>fn|struct|enum|trait|impl|type|static|const|mod|macro_rules!|use|union|extern)\b\s*(?P<rest>.*)$'
)


def outline(path):
    lines = open(path, encoding='utf-8').read().split('\n')
    items = []
    i = 0
    n = len(lines)
    attr_start = None
    while i < n:
        line = lines[i]
        if line.startswith('#[') and not line.startswith('#[cfg_attr') or line.startswith('#!['):
            if attr_start is None:
                attr_start = i
            i += 1
            continue
        if line.startswith('///') or line.startswith('//!') or line.startswith('//'):
            if attr_start is None and line.startswith('///'):
                attr_start = i
            elif attr_start is None and not line.startswith('///'):
                pass
            i += 1
            continue
        m = KINDS.match(line)
        if m and line[:1] not in (' ', '\t', '}'):
            start = attr_start if attr_start is not None else i
            kind = m.group('kind')
            rest = m.group('rest')
            end = i
            if line.rstrip().endswith(';') or (line.rstrip().endswith('}') and '{' in line):
                end = i
            else:
                j = i
                while j < n:
                    l = lines[j]
                    if j > i and l.startswith('}'):
                        end = j
                        break
                    if l.rstrip().endswith(';') and j == i:
                        end = j
                        break
                    j += 1
                else:
                    end = n - 1
            name = rest.split('{')[0].split('(')[0].strip()[:90]
            items.append((kind, name, start + 1, end + 1, i + 1, m.group('vis') or ''))
            i = end + 1
            attr_start = None
            continue
        if line.strip() == '':
            attr_start = None if attr_start is not None and not lines[i - 1].startswith(('#[', '///')) else attr_start
        i += 1
    return items


if __name__ == '__main__':
    path = sys.argv[1]
    mn = int(sys.argv[2]) if len(sys.argv) > 2 else 0
    for kind, name, s, e, h, vis in outline(path):
        if e - s + 1 >= mn:
            print(f'{s:6d}-{e:6d} {e - s + 1:5d} {vis}{kind} {name}')
