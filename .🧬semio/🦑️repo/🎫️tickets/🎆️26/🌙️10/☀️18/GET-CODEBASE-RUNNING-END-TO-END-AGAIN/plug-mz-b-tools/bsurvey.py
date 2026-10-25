import re, sys, hashlib
sys.path.insert(0, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/plug-mz-b-tools")
import tjf
def enclosing_fn(s, idx):
    starts = [m.start() for m in re.finditer(r'\n[ \t]*(?:pub(?:\([a-z]+\))? )?fn \w+', s[:idx])]
    st = starts[-1] + 1
    # find body brace after return type
    sig_end = s.index('->', st)
    b = s.index('{', sig_end)
    e = tjf.match_brace(s, b)
    return st, b, e + 1
if __name__ == '__main__':
    out = {}
    for p in open(sys.argv[1]).read().split('\n'):
        if not p.strip(): continue
        s = open(p, encoding='utf-8').read()
        for m in re.finditer(r'ToolOperationSpec::new\(', s):
            try:
                st, b, e = enclosing_fn(s, m.start())
            except ValueError:
                print('NOFN', p[-90:], s.count('\n', 0, m.start()) + 1); continue
            text = s[st:e]
            sk = re.sub(r'"[^"]*"', '"S"', text)
            sk = re.sub(r'\s+', ' ', sk)
            sk = re.sub(r'\b[A-Z][A-Z0-9_]{3,}\b', 'K', sk)
            sk = re.sub(r'\b[A-Z][a-z]\w*(?:Work|Editor|Command|Snapshot|App)\w*', 'T', sk)
            h = hashlib.md5(sk.encode()).hexdigest()[:8]
            out.setdefault(h, []).append((p, len(text), sk))
    for h, v in sorted(out.items(), key=lambda kv: -len(kv[1])):
        print(h, len(v), v[0][1], v[0][0][-80:])
