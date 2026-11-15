import json,os,sys
S='/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R29/'
OUT='/Users/Shared/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/WORKERS-READY-WITH-LOW-RESOURCES/result-R29.md'
laws=json.load(open(S+'laws-summary.json'))
top=open(S+'top-table.txt',encoding='utf8').read()
body=json.load(open(S+'body-summary.json')) if os.path.exists(S+'body-summary.json') else None
ind=laws['before']['ind']
indrows='\n'.join(f'| `{k[4:]}` | {n} | {c} | 0 |' for k,(n,c) in sorted(ind.items(),key=lambda kv:-kv[1][1]))
b=laws['before'];a=laws['after']
bodytxt=body['text'] if body else 'BODY-PENDING'
txt=open(S+'report-template.txt',encoding='utf8').read()
for k,v in {'@@TOP@@':top,'@@INDROWS@@':indrows,'@@B_ALL30@@':str(b['all30'][0]),'@@A_ALL30@@':str(a['all30'][0]),'@@B_KEYS@@':str(b['all30'][1]),'@@A_KEYS@@':str(a['all30'][1]),'@@A_GROUPS@@':f"{a['all30'][2]} of {a['all30'][3]}",'@@B_GROUPS@@':f"{b['all30'][2]} of {b['all30'][3]}",'@@BODY@@':bodytxt}.items():
    txt=txt.replace(k,v)
open(OUT,'w',encoding='utf8').write(txt)
print('written',len(txt))
