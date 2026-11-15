import os,json,pickle,collections
src=open('analyze3.py').read().split("ownsrc={}")[0]
exec(src)
ns_tok,chk_dep,chk_own=pickle.load(open('keys2.pkl','rb'))
tok_owners_own=collections.defaultdict(set)
for o,(a,b,c) in chk_own.items():
    for t in a:tok_owners_own[t].add(o)
def projroot(r):
    x=r
    while True:
        if os.path.exists(R+x+'/📋️project.json'):return x
        up=os.path.dirname(x)
        if up==x or up=='':return ''
        x=up
rows=[]
for o,d in meta.items():
    r=d['root']
    if r not in byroot or r=='.':continue
    rows.append((len(dep_b[r]),o,r))
rows.sort(reverse=True)
out=[]
for n,o,r in rows[:10]:
    libs=[t['src'] for t in byroot[r]['targets'] if any(k in('lib','proc-macro','rlib','cdylib','dylib','staticlib') for k in t['kind'])]
    f=os.path.normpath(libs[0])[len(R):]
    m=stale(f)
    chk=owners_of(m,'check');bld=owners_of(m,'build');tst=owners_of(m,'test')
    own=set()
    for t in file2tok.get(f,()):own|=tok_owners_own.get(t,set())
    own&=chk
    pj_before={projroot(meta[x]['root']) for x in chk}
    pj_after={projroot(meta[x]['root']) for x in own}
    out.append({'crate':o,'oracle_dependents':n,'check_before':len(chk),'build_before':len(bld),'test_before':len(tst),'check_after_body_only':len(own),'check_after_interface_edit':len(chk),'projects_before':len(pj_before),'projects_after_body_only':len(pj_after)})
    print(json.dumps(out[-1],ensure_ascii=False))
json.dump(out,open('top10.json','w'),ensure_ascii=False)

o='semio-framework-value-source-turn'
f=os.path.normpath([t['src'] for t in byroot[meta[o]['root']]['targets'] if 'lib' in t['kind']][0])[len(R):]
own=set()
for t in file2tok.get(f,()):own|=tok_owners_own.get(t,set())
print('[DEBUG] own owners for',o,sorted(own&owners_of(stale(f),'check')))
