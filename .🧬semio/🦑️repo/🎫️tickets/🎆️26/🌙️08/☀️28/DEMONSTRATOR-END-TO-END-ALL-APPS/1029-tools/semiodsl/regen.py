import re, sys, pathlib, os

SUBSETS = pathlib.Path(sys.argv[2]).resolve()
MARK = '[DEBUG] REGEN'
FILES = [SUBSETS / d / '🚪️io' / '🧪️tests' / '🔬️derived-composition-unit' / '🦀️.rs' for d in sys.argv[3:]]


def patch(path):
    s = path.read_text(encoding='utf-8')
    if MARK in s:
        return 'already'
    m = re.search(r'const FIXTURE_PACK: &\[u8\] = include_bytes!\("([^"]+)"\);', s)
    target = os.path.normpath(str(path.parent / m.group(1)))
    d = re.search(r'^(\s*)let demo = (snapshot::\w+\(\));$', s[m.end():], re.M)
    insert_at = m.end() + d.end()
    line = '\n%sprintln!("%s {}|{}", %s, pack::snapshot::ArtifactPack::encode_pack(&demo).iter().map(|b| format!("{b:02x}")).collect::<String>());' % (d.group(1), MARK, '"' + target + '"')
    s = s[:insert_at] + line + s[insert_at:]
    path.write_text(s, encoding='utf-8')
    return target


def unpatch(path):
    lines = path.read_text(encoding='utf-8').split('\n')
    path.write_text('\n'.join(l for l in lines if MARK not in l), encoding='utf-8')


def apply_output(log):
    count = 0
    for line in pathlib.Path(log).read_text(encoding='utf-8', errors='replace').split('\n'):
        if MARK in line:
            payload = line.split(MARK + ' ', 1)[1]
            target, hexed = payload.rsplit('|', 1)
            pathlib.Path(target).write_bytes(bytes.fromhex(hexed.strip()))
            print('wrote', target.split('subsets/')[-1])
            count += 1
    return count


if __name__ == '__main__':
    mode = sys.argv[1]
    if mode == 'patch':
        for f in FILES:
            print(f.parts[-5], patch(f))
    elif mode == 'unpatch':
        for f in FILES:
            unpatch(f)
    elif mode == 'apply':
        print(apply_output(sys.argv[2]))
