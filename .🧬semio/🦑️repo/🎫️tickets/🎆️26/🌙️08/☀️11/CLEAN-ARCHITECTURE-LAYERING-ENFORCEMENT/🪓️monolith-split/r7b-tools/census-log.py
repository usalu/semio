# 📊️ Prints the census rows and verdict keys of a `--verify` / `--check-applied` log: usage `python3 census-log.py <log>`.
import json,sys
t=open(sys.argv[1],encoding='utf8').read()
i=t.index('\n{\n')+1
j=json.JSONDecoder().raw_decode(t[i:])[0]
for r in j.get('census',[]): print(r['name'],'lib',r['lib'],'test',r['test'],'OK' if r['withinHardBudget'] else 'OVER')
for k,v in j.items():
    if k!='census': print(k,json.dumps(v)[:400])
print(t[t.rindex('}')+1:][:300])
