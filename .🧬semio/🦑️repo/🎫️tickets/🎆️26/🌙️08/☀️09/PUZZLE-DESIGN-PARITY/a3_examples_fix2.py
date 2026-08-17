from __future__ import annotations
import re
from pathlib import Path

puzzle = Path(open("/tmp/puzzle_path.txt").read().strip())
examples = puzzle / "🗿️artifacts/🖐️5d/📚️examples"

GRIP_RE = re.compile(
    r'grip-kind=(?P<kind>"[^"]+"|\S+)\s+'
    r'grip-2d=angle=(?P<angle>\S+)\s+'
    r'grip-kind=(?P<kind2>"[^"]+"|\S+)\s+'
    r'radius=(?P<r2d>\S+)\s+'
    r'grip-3d=position=(?P<pos>@\S+)\s+'
    r'direction=(?P<dir>\^\S+)\s+'
    r'radius=(?P<r3d>\S+)'
)

def transform_grips(body: str) -> str:
    out = []
    for i, m in enumerate(GRIP_RE.finditer(body)):
        kind = m.group('kind')
        out.append(
            f'id=g{i} name={kind} label={kind} grip-kind={kind} point={m.group("pos")} direction={m.group("dir")} radius={m.group("r3d")}'
        )
    return ' '.join(out) if out else body.strip()

def split_row_fields(row: str):
    tokens = []
    i = 0
    n = len(row)
    while i < n:
        while i < n and row[i].isspace():
            i += 1
        if i >= n:
            break
        if row[i] == '"':
            j = i + 1
            while j < n and row[j] != '"':
                j += 1
            tokens.append(row[i:j+1])
            i = j + 1
        elif row[i] in '[{':
            open_c = row[i]
            close_c = ']' if open_c == '[' else '}'
            depth = 0
            j = i
            while j < n:
                if row[j] == open_c:
                    depth += 1
                elif row[j] == close_c:
                    depth -= 1
                    if depth == 0:
                        j += 1
                        break
                j += 1
            tokens.append(row[i:j])
            i = j
        else:
            j = i
            while j < n and not row[j].isspace():
                j += 1
            tokens.append(row[i:j])
            i = j
    return tokens

def transform_catalog_parts_section(catalogs: str) -> str:
    lines = catalogs.splitlines()
    out = []
    i = 0
    while i < len(lines):
        line = lines[i]
        if line.strip().startswith('parts [') and 'representations' in line:
            out.append(line)
            i += 1
            # consume until closing } at depth 0
            while i < len(lines):
                row = lines[i]
                if row.strip() == '}':
                    out.append(row)
                    i += 1
                    break
                if not row.strip() or row.strip() == '{':
                    out.append(row)
                    i += 1
                    continue
                if 'false []' in row and 'point=' in row:
                    out.append(row)
                    i += 1
                    continue
                toks = split_row_fields(row.strip())
                if len(toks) >= 5 and toks[4].startswith('['):
                    pid, name, label, mesh, grips = toks[:5]
                    new_grips = transform_grips(grips[1:-1])
                    if mesh.strip('_"') == '' or mesh == '_':
                        reps = '[]'
                    else:
                        reps = f'[ id=lod0 name=mesh url={mesh} mime=model/gltf-binary tags=[mesh] description="" ]'
                    indent = re.match(r'^(\s*)', row).group(1)
                    out.append(f'{indent}{pid} {name} {label} "" "" "" "" false [] {reps} [ {new_grips} ] [] []')
                else:
                    out.append(row)
                i += 1
            continue
        out.append(line)
        i += 1
    return '\n'.join(out)

def fix_instance_parts(text: str) -> str:
    lines = text.splitlines()
    out = []
    i = 0
    while i < len(lines):
        line = lines[i]
        if line.startswith('parts [id:TEXT part-kind:REF anchor:TEXT'):
            out.append(line)
            i += 1
            while i < len(lines):
                row = lines[i]
                if row.strip() == '}':
                    out.append(row)
                    i += 1
                    break
                if not row.strip() or row.strip() == '{':
                    out.append(row)
                    i += 1
                    continue
                toks = split_row_fields(row.strip())
                if len(toks) >= 3 and toks[1] != 'fixed' and (len(toks) < 3 or toks[2] != 'fixed'):
                    # id kind fixed? {2d}
                    if len(toks) >= 3 and toks[2].startswith('{'):
                        indent = re.match(r'^(\s*)', row).group(1)
                        out.append(f'{indent}{toks[0]} {toks[1]} fixed ' + ' '.join(toks[2:]))
                        i += 1
                        continue
                out.append(row)
                i += 1
            continue
        out.append(line)
        i += 1
    return '\n'.join(out)

for dsl in examples.rglob('🗣️*.dsl.semio'):
    text = dsl.read_text()
    m = re.search(r'(kind-catalogs=\n)(.*?)(\nkind-compatibility )', text, re.S)
    if not m:
        print('no catalogs', dsl)
        continue
    catalogs = transform_catalog_parts_section(m.group(2))
    text = text[:m.start(2)] + catalogs + text[m.end(2):]
    text = fix_instance_parts(text)
    dsl.write_text(text if text.endswith('\n') else text + '\n')
    t = dsl.read_text()
    cat = t[t.find('kind-catalogs='):t.find('\nkind-compatibility')]
    print(dsl.name, 'cat grip-2d', cat.count('grip-2d'), 'cat point', cat.count('point='), 'fixed', len(re.findall(r' fixed \{', t)))
    # show first catalog row snippet
    idx = cat.find('representations')
    print(' sample', cat[cat.find('{')+1:cat.find('{')+200].strip()[:180])
