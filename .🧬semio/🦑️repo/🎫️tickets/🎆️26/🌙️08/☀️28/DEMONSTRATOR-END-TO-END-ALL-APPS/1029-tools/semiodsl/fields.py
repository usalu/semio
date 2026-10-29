import re, sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
import derive

SUBSETS = pathlib.Path(sys.argv[1])
FILES = [pathlib.Path(a) for a in sys.argv[2:]]


def mixed_enums(files):
    names = set()
    for f in files:
        lines = f.read_text(encoding='utf-8').split('\n')
        for i, l in enumerate(lines):
            m = derive.item_re.match(l)
            if m and m.group(1) == 'enum':
                end = derive.body_end(lines, i)
                if not derive.unit_only(lines[i:end + 1]):
                    names.add(m.group(2))
    return names


field_re = re.compile(r'^(\s+)(?:pub )?([a-z_0-9]+): (.+),\s*$')


def base_type(t):
    t = t.strip()
    while True:
        m = re.match(r'^(Option|Vec|Box)<(.+)>$', t)
        if not m:
            return t
        t = m.group(2).strip()


def apply(path, enums):
    lines = path.read_text(encoding='utf-8').split('\n')
    out = []
    i = 0
    count = 0
    while i < len(lines):
        l = lines[i]
        m = derive.item_re.match(l)
        if m:
            end = derive.body_end(lines, i)
            for k in range(i, end + 1):
                lk = lines[k]
                fm = field_re.match(lk)
                if fm and k > i:
                    ty = fm.group(3)
                    if base_type(ty) in enums:
                        j = len(out)
                        prev = []
                        q = len(out) - 1 if k == i else None
                        out.append(lk)
                        continue
                out.append(lk)
            i = end + 1
            continue
        out.append(l)
        i += 1
    return out, count


def apply_file(path, enums):
    lines = path.read_text(encoding='utf-8').split('\n')
    out = []
    i = 0
    inserted = 0
    while i < len(lines):
        m = derive.item_re.match(lines[i])
        if not m:
            out.append(lines[i])
            i += 1
            continue
        end = derive.body_end(lines, i)
        out.append(lines[i])
        for k in range(i + 1, end + 1):
            fm = field_re.match(lines[k])
            if fm and base_type(fm.group(3)) in enums:
                j = len(out) - 1
                has_dsl = False
                while j >= 0 and lines[k - (len(out) - j)].strip().startswith('#['):
                    if lines[k - (len(out) - j)].strip().startswith('#[dsl('):
                        has_dsl = True
                    j -= 1
                if not has_dsl:
                    out.append(fm.group(1) + '#[dsl(statements, block)]')
                    inserted += 1
            out.append(lines[k])
        i = end + 1
    path.write_text('\n'.join(out), encoding='utf-8')
    return inserted


if __name__ == '__main__':
    enums = mixed_enums([f for f in SUBSETS.rglob('📸️snapshot/🦀️.rs')])
    print(sorted(enums))
    for f in FILES:
        print(f, apply_file(f, enums))
