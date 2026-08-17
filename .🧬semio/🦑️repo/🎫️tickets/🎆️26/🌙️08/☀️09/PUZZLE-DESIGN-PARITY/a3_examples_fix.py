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
    """Split a DSL table row into tokens, respecting quotes and [...] / {...} groups."""
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
            open_c, close_c = row[i], (']' if row[i] == '[' else '}')
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

def fix_catalog_parts_rows(section: str) -> str:
    lines = section.splitlines()
    out = []
    in_parts = False
    depth = 0
    for line in lines:
        if line.strip().startswith('parts [') and 'representations' in line:
            in_parts = True
            out.append(line)
            continue
        if in_parts:
            depth += line.count('{') - line.count('}')
            if depth <= 0:
                in_parts = False
                out.append(line)
                continue
            if not line.strip() or line.strip() == '{':
                out.append(line)
                continue
            # already transformed?
            if 'representations' in line or 'false []' in line:
                out.append(line)
                continue
            toks = split_row_fields(line.strip())
            if len(toks) >= 5 and toks[4].startswith('['):
                pid, name, label, mesh, grips = toks[0], toks[1], toks[2], toks[3], toks[4]
                new_grips = transform_grips(grips[1:-1])
                if mesh in ('_', '""', "''") or mesh == '_':
                    reps = '[]'
                else:
                    reps = f'[ id=lod0 name=mesh url={mesh} mime=model/gltf-binary tags=[mesh] description="" ]'
                indent = re.match(r'^(\s*)', line).group(1)
                out.append(
                    f'{indent}{pid} {name} {label} "" "" "" "" false [] {reps} [ {new_grips} ] [] []'
                )
                continue
            out.append(line)
            continue
        out.append(line)
    return '\n'.join(out)

def fix_grips_catalog(section: str) -> str:
    lines = section.splitlines()
    out = []
    in_grips = False
    depth = 0
    for line in lines:
        if line.strip().startswith('grips [') and 'default-rope-kind' in line:
            in_grips = True
            # ensure new header
            out.append('grips [id:TEXT code:TEXT label:TEXT order:NUM compatible-with:LIST description:TEXT icon:TEXT color:TEXT default-rope-kind:REF] {')
            if '{' not in line:
                pass
            continue
        if in_grips:
            depth += line.count('{') - line.count('}')
            if line.strip() == '{':
                out.append(line)
                continue
            if depth <= 0:
                in_grips = False
                out.append(line)
                continue
            toks = split_row_fields(line.strip())
            # old: id name label color rope  OR already new with 9 fields
            if len(toks) == 5:
                indent = re.match(r'^(\s*)', line).group(1)
                gid, name, label, color, rope = toks
                out.append(f'{indent}{gid} {name} {label} 0 [] "" "" {color} {rope}')
                continue
            out.append(line)
            continue
        out.append(line)
    return '\n'.join(out)

def fix_instance_parts(text: str) -> str:
    lines = text.splitlines()
    out = []
    in_parts = False
    depth = 0
    for line in lines:
        if line.startswith('parts [id:TEXT part-kind:REF anchor:TEXT'):
            in_parts = True
            out.append(line)
            continue
        if in_parts:
            depth += line.count('{') - line.count('}')
            if depth <= 0:
                in_parts = False
                out.append(line)
                continue
            if 'fixed' in split_row_fields(line.strip())[:3]:
                out.append(line)
                continue
            toks = split_row_fields(line.strip())
            # id partkind {2d} {3d} [grips]
            if len(toks) >= 4 and toks[2].startswith('{'):
                indent = re.match(r'^(\s*)', line).group(1)
                out.append(f'{indent}{toks[0]} {toks[1]} fixed ' + ' '.join(toks[2:]))
                continue
            out.append(line)
            continue
        out.append(line)
    return '\n'.join(out)

def fix_fastener_rows(text: str) -> str:
    """Ensure fastener data rows have trailing x y when header requires them."""
    lines = text.splitlines()
    out = []
    in_f = False
    depth = 0
    for line in lines:
        if line.startswith('fasteners [id:TEXT source:TEXT target:TEXT'):
            in_f = True
            out.append(line)
            continue
        if in_f:
            depth += line.count('{') - line.count('}')
            if depth <= 0:
                in_f = False
                out.append(line)
                continue
            toks = split_row_fields(line.strip())
            # id source target kind gap shift rise rotation turn tilt [x y]
            if len(toks) == 10:
                indent = re.match(r'^(\s*)', line).group(1)
                out.append(f'{indent}{" ".join(toks)} 0 0')
                continue
            out.append(line)
            continue
        out.append(line)
    return '\n'.join(out)

for dsl in examples.rglob('🗣️*.dsl.semio'):
    text = dsl.read_text()
    # Fix catalogs block only (from kind-catalogs= through kind-compatibility)
    m = re.search(r'(kind-catalogs=\n)(.*?)(\nkind-compatibility )', text, re.S)
    if not m:
        print('no catalogs block', dsl)
        continue
    catalogs = m.group(2)
    catalogs = fix_catalog_parts_rows(catalogs)
    catalogs = fix_grips_catalog(catalogs)
    text = text[:m.start(2)] + catalogs + text[m.end(2):]
    text = fix_instance_parts(text)
    text = fix_fastener_rows(text)
    if not text.endswith('\n'):
        text += '\n'
    dsl.write_text(text)
    # stats
    t = dsl.read_text()
    print(dsl.name, 'grip-2d', t.count('grip-2d'), 'mesh-url hdr', t.count('mesh-url:TEXT'), 'fixed anchors', len(re.findall(r' fixed \{', t)))
