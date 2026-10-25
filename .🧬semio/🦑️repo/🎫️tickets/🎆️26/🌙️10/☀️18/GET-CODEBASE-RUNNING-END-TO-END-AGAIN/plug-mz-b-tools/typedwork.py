import re, subprocess, sys
sys.path.insert(0, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/plug-mz-b-tools")
import tjf
files = [l for l in open(sys.argv[1]).read().split('\n') if l.strip()]
for p in files:
    s = open(p, encoding='utf-8').read()
    if 'admit_retained_tool_producer' not in s: continue
    head = subprocess.run(['git', 'show', 'HEAD:' + p], capture_output=True, text=True).stdout
    types = re.findall(r'let\s+work\s*:\s*(Box<dyn\s+[^=;]+?>)\s*=', head)
    if not types: continue
    ty = types[0]
    changed = 0
    for m in reversed(list(re.finditer(r'admit_retained_tool_producer\(', s))):
        o = m.end() - 1
        c = tjf.match_brace(s.replace('(', '{').replace(')', '}'), o)
        call = s[o+1:c]
        cm = re.search(r'\|request\| (?!-> )', call)
        if not cm: continue
        expr = call[cm.end():]
        if expr.lstrip().startswith('{') and False: continue
        newcall = call[:cm.start()] + f'|request| -> {ty} {{ {expr} }}'
        s = s[:o+1] + newcall + s[c:]
        changed += 1
    if changed:
        open(p, 'w', encoding='utf-8').write(s)
        print('typed', changed, p[-80:], ty)
