import re, sys, pathlib

RECORD = "semio_framework_dsl_record_derive::DslRecord"
SCALAR = "semio_framework_dsl_record_derive::DslScalar"
ENUM = "semio_framework_dsl_record_derive::DslEnum"
RETIRE = "semio_framework_value::RetireOwned"
RECEIVE = "semio_framework_value::NativeReceive"

item_re = re.compile(r'^pub (struct|enum) (\w+)')


def attr_block_start(lines, index):
    j = index
    while j > 0 and re.match(r'^\s*(#\[|///|//)', lines[j - 1]):
        j -= 1
    return j


def body_end(lines, index):
    if lines[index].rstrip().endswith(';'):
        return index
    j = index
    depth = 0
    started = False
    while j < len(lines):
        for ch in lines[j]:
            if ch == '{':
                depth += 1
                started = True
            elif ch == '}':
                depth -= 1
        if started and depth == 0:
            return j
        j += 1
    raise SystemExit('unterminated item at %d' % index)


def unit_only(body):
    inner = body[1:]
    for line in inner:
        s = line.strip()
        if not s or s.startswith('//') or s.startswith('#['):
            continue
        if s == '}':
            continue
        if '{' in s or '(' in s:
            return False
    return True


def retire_macro_names(path):
    parts = path.resolve().parts
    root = pathlib.Path(*parts[:next(i for i,x in enumerate(parts) if x.endswith('subsets')) + 2])
    names = set()
    for f in root.rglob('🦀️.rs'):
        for m in re.finditer(r'artifact_retire_(?:struct|leaf|enum)!\(\s*(?:[\w]+::)*(\w+)', f.read_text(encoding='utf-8')):
            names.add(m.group(1))
    return names


def transform(path, skip=()):
    macro_retired = retire_macro_names(path)
    lines = path.read_text(encoding='utf-8').split('\n')
    out = list(lines)
    inserts = []
    names = []
    for i, line in enumerate(lines):
        m = item_re.match(line)
        if not m:
            continue
        kind, name = m.groups()
        if name in skip:
            continue
        start = attr_block_start(lines, i)
        block = '\n'.join(lines[start:i])
        if 'NativeReceive' in block or ('ToValue' not in block and 'ArtifactSchema' not in block):
            continue
        end = body_end(lines, i)
        body = lines[i:end + 1]
        derive_idx = [k for k in range(start, i) if lines[k].lstrip().startswith('#[derive(')]
        if not derive_idx:
            continue
        extras = []
        if 'RetireOwned' not in block and name not in macro_retired:
            extras.append(RETIRE)
        extras.append(RECEIVE)
        if re.search(r'Dsl(Record|Scalar|Enum)', block):
            pass
        elif kind == 'struct':
            extras.append(RECORD)
        else:
            extras.append(SCALAR if unit_only(body) else ENUM)
        text = '\n'.join(body)
        recursive = re.search(r'\b%s\b' % name, '\n'.join(body[1:])) is not None
        add = []
        if recursive:
            add.append('#[value(receive_recursive)]')
        if 'ArtifactSchema' in block:
            add.append('#[dsl(layout = "lines")]')
        first = derive_idx[0]
        merged = out[first].rstrip()
        assert merged.endswith(')]'), merged
        out[first] = merged[:-2] + ', ' + ', '.join(extras) + ')]'
        inserts.append((first + 1, add))
        names.append((name, kind, recursive))
    for pos, add in sorted(inserts, reverse=True):
        out[pos:pos] = add
    path.write_text('\n'.join(out), encoding='utf-8')
    return names


if __name__ == '__main__':
    for arg in sys.argv[1:]:
        for name, kind, rec in transform(pathlib.Path(arg)):
            print(arg.split('/')[-4] if False else '', kind, name, 'RECURSIVE' if rec else '')
