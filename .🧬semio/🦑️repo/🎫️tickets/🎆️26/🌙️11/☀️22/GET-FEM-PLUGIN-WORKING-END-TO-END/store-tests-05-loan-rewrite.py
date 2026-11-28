import sys, io
H0 = 'const IDENTITY_CEILING: usize'
for path in sys.argv[1:]:
    raw = io.open(path, encoding='utf-8', newline='').read()
    E = '\r\n' if '\r\n' in raw else '\n'
    L = raw.split(E)
    out = []
    i = 0
    n = 0
    while i < len(L):
        if L[i].lstrip().startswith(H0) and 'let mut identity_observer' in L[i + 1] and 'let mut identity' in L[i + 2]:
            j = i + 3
            while L[j] != '}':
                j += 1
            body = L[i + 3:j]
            while body and body[-1].strip() == '':
                body.pop()
            tail = 0
            while body and (body[-1].lstrip().startswith('identity.encode(') or body[-1].lstrip().startswith('drop(identity.pause()')):
                body.pop()
                tail += 1
            assert tail == 2, (path, i, tail)
            body = [b.replace('&mut identity', '&mut *identity') for b in body]
            out.append('    test_support::with_original_identity(|identity| test_support::block_on_original_turn(async move {')
            out += body
            out.append('    }));')
            i = j
            n += 1
            continue
        out.append(L[i])
        i += 1
    io.open(path, 'w', encoding='utf-8', newline='').write(E.join(out))
    print(n)
