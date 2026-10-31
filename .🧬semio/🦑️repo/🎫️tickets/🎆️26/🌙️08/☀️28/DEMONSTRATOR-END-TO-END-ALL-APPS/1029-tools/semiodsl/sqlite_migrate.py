import re, sys, pathlib

SUBSETS = pathlib.Path(sys.argv[1])
NAMES = sys.argv[2:]
fn_re = re.compile(r'^(\s*)(?:#\[[^\]]*\]\s*)?(?:pub(?:\((?:crate|super)\))?\s*)?(?:const\s+)?fn\s+(\w+)')


def remove_fn(src, pred, keep=()):
    removed = []
    i = 0
    while i < len(src):
        m = fn_re.match(src[i])
        if m and pred(m.group(2)) and m.group(2) not in keep and len(m.group(1)) <= 0:
            depth = 0; started = False; j = i
            while True:
                depth += src[j].count('{') - src[j].count('}')
                if '{' in src[j]:
                    started = True
                if started and depth == 0:
                    break
                j += 1
            k = i
            while k > 0 and re.match(r'^\s*(///|//|#\[)', src[k - 1]):
                k -= 1
            removed.append(m.group(2))
            del src[k:j + 1]
            continue
        i += 1
    return removed


def binary_name(n):
    return 'binary' in n


for name in NAMES:
    d = SUBSETS / name / '🚪️io' / '🪶️sqlite' / '📸️snapshot'
    # decode
    p = d / '🛬️native' / '🦀️.rs'
    src = p.read_text(encoding='utf-8').split('\n')
    text = '\n'.join(src)
    m = re.search(r'pub fn binary_into\(([^)]*)\)->Result<\(\),ValueError>\s*\{', text)
    assert m, name
    args = m.group(1)
    start = m.start()
    depth = 0; j = m.end() - 1
    while True:
        c = text[j]
        if c == '{': depth += 1
        elif c == '}':
            depth -= 1
            if depth == 0: break
        j += 1
    out_ident = re.match(r'\s*(\w+)\s*:', args).group(1)
    new_args = re.sub(r'\blimits\b', '_limits', args)
    new = 'pub fn binary_into(%s)->Result<(),ValueError>{semio_s_artifact_stdio_semio_base::io::sqlite::snapshot::native_decoding::record_binary_into(%s,body,control)}' % (new_args, out_ident)
    text = text[:start] + new + text[j + 1:]
    src = text.split('\n')
    r = remove_fn(src, binary_name, keep=('binary_into',))
    p.write_text('\n'.join(src), encoding='utf-8')
    # semantic
    ps = d / '🧮️semantic' / '🦀️.rs'
    ss = ps.read_text(encoding='utf-8').split('\n')
    r2 = remove_fn(ss, binary_name)
    ps.write_text('\n'.join(ss), encoding='utf-8')
    # root: admit_binary
    pr = d / '🦀️.rs'
    rs = pr.read_text(encoding='utf-8').split('\n')
    r3 = remove_fn(rs, lambda n: n == 'admit_binary')
    pr.write_text('\n'.join(rs), encoding='utf-8')
    # encode
    pe = d / '🛫️native' / '🦀️.rs'
    et = pe.read_text(encoding='utf-8')
    m = re.search(r'pub fn fields\(([^)]*)\)->Result<\(\),ValueError>\s*\{', et)
    assert m, name
    snap = re.match(r'\s*(\w+)\s*:', m.group(1)).group(1)
    writer = re.search(r'(\w+)\s*:\s*&mut Writer', m.group(1)).group(1)
    enc = re.search(r'(\w+)\s*:\s*SnapshotEncoding', m.group(1)).group(1)
    ins = 'if %s==SnapshotEncoding::Binary{return semio_s_artifact_stdio_semio_base::io::sqlite::snapshot::native_encoding::record_binary(%s,%s)}' % (enc, snap, writer)
    et = et[:m.end()] + ins + et[m.end():]
    pe.write_text(et, encoding='utf-8')
    print(name, len(r), len(r2), len(r3))
