import re, sys
sys.path.insert(0, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/plug-mz-b-tools")
import tjf
for p in [l for l in open(sys.argv[1]).read().split('\n') if l.strip()]:
    s = open(p, encoding='utf-8').read()
    n = 0
    for m in reversed(list(re.finditer(r'\n((?:pub(?:\([a-z]+\))? )?fn [^\n]*->\s*Result<bool, semio_framework_plugin::__job::OriginalTurnFailure>\s*\{)\n(        let original = )', s))):
        ob = m.end(1) + 1 - 1
        ob = s.index('{', m.start(1) + len(m.group(1)) - 1)
        cb = tjf.match_brace(s, ob)
        seg = s[ob:cb+1]
        lines = seg.split('\n')
        out = []
        for i, l in enumerate(lines):
            if i == 0: out.append(l); continue
            if l.startswith('        ') and not l.startswith('         ') and (l.lstrip().startswith(('let original', 'semio_framework_plugin::retained_command::producer')) ):
                l = l[4:]
            out.append(l)
        seg2 = '\n'.join(out)
        seg2 = re.sub(r'\n    \}$', '\n}', seg2)
        if seg2 != seg:
            s = s[:ob] + seg2 + s[cb+1:]
            n += 1
    if n:
        open(p, 'w', encoding='utf-8').write(s); print('fmt', n, p[-60:])
