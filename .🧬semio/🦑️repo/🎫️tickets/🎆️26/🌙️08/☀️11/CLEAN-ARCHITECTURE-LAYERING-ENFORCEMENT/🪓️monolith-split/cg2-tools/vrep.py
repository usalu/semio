import json,sys
d=json.load(open(sys.argv[1]))
rep=lambda s:s.replace('✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/','~/')[:240]
print('census',len(d['census']),'overHard',sum(1 for r in d['census'] if not r['withinHardBudget']),'overTarget',sum(1 for r in d['census'] if not r['withinTargetBudget']))
for k in ('moduleTreeProblems','visibility','orphan','glue'): print(k,len(d[k]))
print('unresolvedCrates nonempty',{k:v[:3] for k,v in d['unresolvedCrates'].items() if v})
print('inheritedUnresolved',d['inheritedUnresolvedCrates'])
for k,o in (d.get('oracle') or {}).items():
    print('ORACLE',k,o['original'],o['candidate'],'onlyOrig',o['onlyOriginal'],'onlyCand',o['onlyCandidate'])
    for s in o['sampleOnlyCandidate']: print('  C',rep(s))
for k,o in (d.get('layout') or {}).items():
    print('LAYOUT',k,o['union'] if 'union' in o else o.get('base'),o['combined'],'onlyCombined',o['onlyCombined'])
    for s in o['sampleOnlyCombined']: print('  L',rep(s))
