import json,sys
s=open(sys.argv[1],encoding='utf8').read()
i=s.find('{\n  "census"')
j=json.JSONDecoder().raw_decode(s[i:])[0]
for c in j['census']: print(c['name'],c['lib'],c['test'],c['withinHardBudget'])
for k in ['moduleTreeProblems','visibility','orphan','glue']: print(k,j[k][:5])
print('unresolved',{k:v for k,v in j['unresolvedCrates'].items() if v})
print('oracle',{k:(v['onlyCandidate'],v['onlyOriginal']) for k,v in (j['oracle'] or {}).items()})
print('layout',{k:v['onlyCombined'] for k,v in (j['layout'] or {}).items()})
for k,v in (j['oracle'] or {}).items(): print(k, v['sampleOnlyCandidate'][:6])
for k,v in (j['layout'] or {}).items(): print(k, v['sampleOnlyCombined'][:6])
