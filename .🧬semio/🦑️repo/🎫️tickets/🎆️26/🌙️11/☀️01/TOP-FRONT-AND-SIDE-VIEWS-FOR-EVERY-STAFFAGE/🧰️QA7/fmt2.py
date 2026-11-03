import json, os, sys
sys.stdout.reconfigure(encoding='utf8')
HERE = os.path.dirname(os.path.abspath(__file__))
F = os.path.join(HERE, '..', '🧰️F', 'orig')
for a in sys.argv[1:]:
    raw = open(os.path.join(F, a + '.json'), encoding='utf8', newline='').read()
    out = json.dumps(json.loads(raw), ensure_ascii=False, indent=2) + '\n'
    x, y = out.splitlines(), raw.splitlines()
    n = 0
    print(a, len(x), len(y), 'crlf' if '\r\n' in raw else 'lf', repr(raw[-3:]))
    for i, (p, q) in enumerate(zip(x, y)):
        if p != q:
            print(i, repr(p[:120]), '|||', repr(q[:120]))
            n += 1
            if n > 4:
                break
