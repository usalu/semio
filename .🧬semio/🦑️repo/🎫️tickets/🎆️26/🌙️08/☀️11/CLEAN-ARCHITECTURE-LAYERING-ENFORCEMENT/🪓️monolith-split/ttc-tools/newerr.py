import json,sys,collections,re
p=sys.argv[1]
r=json.load(open(p,encoding='utf8'))
for cfg,e in r['summary'].items():
    print('==',cfg,'cur',e['cur_total'],'base',e.get('base_total'),'new',e.get('new'),'gone',e.get('gone'))
    g=collections.Counter()
    for d in e['new_list']:
        f=re.sub(r'^.*?/__m_','__m_',d['file']).split('/')
        f=f[0][4:]+':'+'/'.join(x for x in f[1:] if x not in ('..',))[-60:]
        g[(d['code'],d['msg'][:110],d['label'][:50],f.split(':')[0])]+=1
    for k,n in sorted(g.items(),key=lambda x:-x[1])[:int(sys.argv[2]) if len(sys.argv)>2 else 25]:
        print('  ',n,k)
    gk=e.get('gone_keys',[])
    for k in gk[:10]: print('   GONE',k['n'],k['code'],k['msg'][:110])
