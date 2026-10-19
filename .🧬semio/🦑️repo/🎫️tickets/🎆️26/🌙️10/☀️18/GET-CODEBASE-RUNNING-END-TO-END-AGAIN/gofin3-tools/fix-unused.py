#!/usr/bin/env python3
import re, sys
T='/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/'
log=open(sys.argv[1],encoding='utf8').read()
edits={}
for m in re.finditer(r'\./(\S+🐹️\.go):(\d+):\d+: declared and not used: (\w+)',log):
    edits.setdefault(m.group(1),[]).append((int(m.group(2)),m.group(3)))
for f,es in edits.items():
    lines=open(T+f,encoding='utf8').read().split('\n')
    for ln,name in es:
        l=lines[ln-1]
        if l.strip()==name+' := withExhaustiveRepo(t)':
            lines[ln-1]=l.replace(name+' := ','')
            print('fixed',f,ln)
        else:
            print('MANUAL',f,ln,l)
    open(T+f,'w',encoding='utf8').write('\n'.join(lines))
