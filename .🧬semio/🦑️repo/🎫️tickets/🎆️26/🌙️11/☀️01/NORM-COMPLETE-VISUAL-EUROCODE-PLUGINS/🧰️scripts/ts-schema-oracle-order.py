import json,re,sys,glob
for S in [a+'/🏅️standards/🔖️1/🪆️subsets/✳️any' for a in glob.glob('/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/*') if a.endswith(('en1999','en1998'))]:
    t=open(S+'/🧬️schema/🧬️mutations/🦀️.rs').read()
    kinds=re.findall(r'"([^"]+)"',re.search(r'pub const KINDS:\s*&\[&str\]\s*=\s*&\[([\s\S]*?)\];',t).group(1))
    p=S+'/🔮️oracles/🔣️.json'; raw=open(p).read(); m=json.loads(raw)
    for c in m['mutationCatalogs']:
        by={v['mutationId']:v for v in c['vectors']}
        for k in kinds:
            if k in by: by[k]['scenarios']=[s for s in by[k]['scenarios'] if s['id'].startswith(k+'-') or s['id']=='apply']
        for k in kinds:
            if k not in by:
                d=[x for x in glob.glob(S+'/🧪️tests/🧫️fixtures/🧬️mutations/*'+k) if x.endswith(k)]
                assert len(d)==1,k
                dn=d[0].split('/')[-1]
                by[k]={'mutationId':k,'sourceMutationDirectoryName':dn,'mutationDirectoryName':dn,'scenarios':[{'id':'apply','directoryName':'✅apply'},{'id':k+'-dupe','directoryName':'⛔dupe'}]}
        c['vectors']=[by[k] for k in kinds]; c['kinds']=kinds
    new=json.dumps(m,indent=2,ensure_ascii=False)+'\n'
    if new!=raw:
        print(S.split('/')[-5], 'roundtrip-exact' if json.dumps(json.loads(raw),indent=2,ensure_ascii=False)+'\n'==raw else 'FORMAT-DIFF-ORIGINAL')
        if '--apply' in sys.argv: open(p,'w').write(new)
