import re, sys
sys.path.insert(0, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/plug-mz-b-tools")
import tjf
for p in [l for l in open(sys.argv[1]).read().split('\n') if l.strip()]:
    s = open(p, encoding='utf-8').read()
    n = 0
    for m in reversed(list(re.finditer(r'\|request\| -> (Box<dyn [^{]*?>) \{ ', s))):
        ob = s.index('{', m.end() - 2)
        cb = tjf.match_brace(s, ob)
        inner = s[ob+1:cb].strip()
        if re.match(r'(match\b|if\b|Box::new\()', inner): continue
        s = s[:m.start()] + '|request| ' + inner + s[cb+1:]
        n += 1
    if n:
        open(p, 'w', encoding='utf-8').write(s); print('untyped', n, p[-70:])
