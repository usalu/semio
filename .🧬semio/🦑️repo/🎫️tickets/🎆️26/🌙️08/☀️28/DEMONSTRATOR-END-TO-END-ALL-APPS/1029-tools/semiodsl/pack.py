import re, sys, pathlib

impl_re = re.compile(r'^(\s*)impl pack::snapshot::ArtifactPack for (\w+) \{')


def replace(path):
    lines = path.read_text(encoding='utf-8').split('\n')
    out = []
    i = 0
    done = []
    while i < len(lines):
        m = impl_re.match(lines[i])
        if not m:
            out.append(lines[i])
            i += 1
            continue
        indent, name = m.groups()
        depth = 0
        j = i
        while True:
            depth += lines[j].count('{') - lines[j].count('}')
            if depth == 0 and j >= i:
                break
            j += 1
        out.append('%ssemio_s_artifact_stdio_semio_base::semio_snapshot_record_pack!(%s);' % (indent, name))
        done.append(name)
        i = j + 1
    path.write_text('\n'.join(out), encoding='utf-8')
    return done


if __name__ == '__main__':
    for arg in sys.argv[1:]:
        print(arg, replace(pathlib.Path(arg)))
