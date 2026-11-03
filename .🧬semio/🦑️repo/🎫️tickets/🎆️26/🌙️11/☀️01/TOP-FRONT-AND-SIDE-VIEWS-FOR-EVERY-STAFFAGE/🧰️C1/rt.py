import json, sys, io, os
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf8')
base = r'C:\git\semio\🎓️teaching\🏛️architecture\🧍️staffage\🗂️catalog\🧱️components'
for i in sys.argv[1:]:
    raw = open(os.path.join(base, i, '🔣️.json'), encoding='utf8', newline='').read()
    rt = json.dumps(json.loads(raw), ensure_ascii=False, indent=2)
    print(i, 'crlf', '\r\n' in raw, 'endsnl', raw.endswith('\n'), 'len', len(raw), len(rt))
    n = min(len(raw), len(rt))
    for k in range(n):
        if raw[k] != rt[k]:
            print(' first diff at', k, repr(raw[max(0, k - 60):k + 60]), '|||', repr(rt[max(0, k - 60):k + 60]))
            break
