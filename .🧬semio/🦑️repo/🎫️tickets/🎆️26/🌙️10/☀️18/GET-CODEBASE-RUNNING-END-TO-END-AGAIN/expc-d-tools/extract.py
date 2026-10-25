import json,sys,collections
src,out=sys.argv[1],sys.argv[2]
d=json.load(open(src))
v=d['summary']['violations']
edges=[dict(rule=x['rule']['name'],type=x['type'],**{'from':x['from'],'to':x.get('to','')}) for x in v]
json.dump({'ok':True,'edges':edges,'violations':v},open(out,'w'),ensure_ascii=False)
def area(f):
    return 'process' if f.startswith('🧰️framework/🔨️modules/🏃️process') else 'print' if f.startswith(('🧰️framework/🛍️products/📓️print','♻️mit-bestand/🎤️','🧰️framework/🛍️products/🎤️presentation')) else 'course' if f.startswith(('🎓️teaching','🧰️framework/🛍️products/📚️course')) else 'repo' if f.startswith('🧰️framework/🛍️products/🦑️repo') else 'other'
c=collections.Counter(area(e['from']) for e in edges)
print(len(edges),dict(c))
cc=collections.Counter((area(e['from']),e['rule'].replace('no-cross-package-relative-','ncpr-')) for e in edges if area(e['from']) in ('process','print','course'))
for k,n in sorted(cc.items()): print(n,k)
