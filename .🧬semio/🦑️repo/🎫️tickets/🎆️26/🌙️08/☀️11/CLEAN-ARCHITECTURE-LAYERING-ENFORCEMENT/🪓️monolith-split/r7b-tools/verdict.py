# 🧾️ Prints oracle/layout rows with samples from a `--verify`/`--check-applied` log: usage `python3 verdict.py <log>`.
import json,sys
t=open(sys.argv[1],encoding='utf8').read()
i=t.index('\n{\n')+1
j=json.JSONDecoder().raw_decode(t[i:])[0]
for r in j.get('census',[]):
    print(r['name'].replace('semio-s-artifact-',''),'lib',r['lib'],'test',r['test'],'OK' if r['withinHardBudget'] else 'OVER')
for k in ('moduleTreeProblems','visibility','orphan','glue','inheritedUnresolvedCrates','admission','shared','sharedForeign'):
    if k in j and j[k]: print(k,json.dumps(j[k])[:600])
for k,v in j.get('unresolvedCrates',{}).items():
    if v: print('unresolved',k,v[:5])
for k in ('base','oracle','layout','native','test'):
    if k in j and isinstance(j[k],dict):
        for cfg,row in j[k].items():
            if isinstance(row,dict) and 'original' in row or isinstance(row,dict) and 'union' in row or isinstance(row,dict) and 'base' in row:
                print(k,cfg,{a:b for a,b in row.items() if not a.startswith('sample')})
                for s in row.get('sampleOnlyCandidate',row.get('sampleOnlyCombined',row.get('sampleExtra',[]))): print('   +',s.replace('✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/','').replace('🏅️standards/🔖️1/🪆️subsets/✳️any/','~/')[:330])
