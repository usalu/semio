#!/usr/bin/env python3
import re, sys

FIELD_RE = re.compile(r'^\s*(pub(\(\w+\))?\s+)?\w+\s*:\s*(Option<)?(Vec<)?(BTreeMap<[^,]+,\s*)?(dsl::|crate::value::|protocol::value::|protocol::|store::)?DslValue')
STRUCT_RE = re.compile(r'^\s*(pub(\(\w+\))?\s+)?(struct|enum)\s+(\w+)')
DERIVE_RE = re.compile(r'#\[derive\(([^)]*)\)\]')

def process(path):
    with open(path, encoding='utf-8') as f:
        lines = f.readlines()
    n = len(lines)
    i = 0
    last_derive = None
    last_derive_line = None
    while i < n:
        line = lines[i]
        m = DERIVE_RE.search(line)
        if m:
            last_derive = m.group(1)
            last_derive_line = i+1
        m2 = STRUCT_RE.match(line)
        if m2:
            name = m2.group(4)
            struct_line = i+1
            derive = last_derive
            derive_line = last_derive_line
            # find body: from this line, count braces until matched (or ; for unit struct/tuple struct)
            depth = 0
            started = False
            body_lines = []
            j = i
            while j < n:
                for ch in lines[j]:
                    if ch == '{':
                        depth += 1
                        started = True
                    elif ch == '}':
                        depth -= 1
                if started:
                    body_lines.append((j+1, lines[j]))
                if started and depth == 0:
                    break
                if not started and ';' in lines[j]:
                    break
                j += 1
            has_dslvalue_field = False
            field_lines = []
            for ln, bl in body_lines:
                if FIELD_RE.match(bl):
                    has_dslvalue_field = True
                    field_lines.append((ln, bl.strip()))
            if has_dslvalue_field:
                print(f"{path}:{struct_line} type={name} derive_line={derive_line} derive={derive}")
                for ln, bl in field_lines:
                    print(f"    field@{ln}: {bl}")
        i += 1

for p in sys.argv[1:]:
    process(p)
