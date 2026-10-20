#!/bin/zsh
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
$C/law-drift-tools/t.sh "$1" > /tmp/ld-fails.log 2>&1
python3 - <<'P'
import re
t=open('/tmp/ld-fails.log',encoding='utf-8').read()
parts=re.split(r'\n(?=\(fail\)|\(pass\))',t)
blocks=re.findall(r'((?:^error:[^\n]*\n(?:(?!\n\(fail\)|\n\(pass\))[^\n]*\n)*?)\(fail\)[^\n]*)',t,re.M)
for b in blocks:
    lines=[l for l in b.split('\n') if l.strip()]
    keep=[l[:260] for l in lines if re.match(r'(error:|Expected|Received|@@|[+-] |\(fail\)|\s+at <anonymous>)',l)]
    print("\n".join(keep[:14])); print('---')
m=re.findall(r'^ +\d+ (?:pass|fail)$',t,re.M); print(m)
P
