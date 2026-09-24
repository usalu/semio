import json,sys
S="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/"
d=json.load(open('.tmp-ticket/wp-t4/gltf/survey.json'))
def load(p): return json.load(open(S+p.replace('../../',''),encoding='utf8'))
def summ(doc):
    n=[{k:v for k,v in x.items() if k in('name','children','mesh','camera','skin','weights','translation','extras','extensions')} for x in doc.get('nodes',[])]
    sc=[{k:v for k,v in x.items() if k in('name','nodes','extras','extensions')} for x in doc.get('scenes',[])]
    me=[{'name':m.get('name'),'weights':m.get('weights'),'extras':m.get('extras'),'extensions':m.get('extensions'),'prims':[{k:(v if k!='targets' else [list(t.items()) for t in v]) for k,v in p.items()} for p in m['primitives']]} for m in doc.get('meshes',[])]
    acc=[(a.get('bufferView'),a.get('componentType'),a['type'],a['count']) for a in doc.get('accessors',[])]
    bv=[(b['buffer'],b.get('byteOffset',0),b['byteLength']) for b in doc.get('bufferViews',[])]
    bu=[(b['byteLength'],(b.get('uri') or '')[:40]) for b in doc.get('buffers',[])]
    return dict(scene=doc.get('scene'),scenes=sc,nodes=n,meshes=me,accessors=acc,bufferViews=bv,buffers=bu,skins=len(doc.get('skins',[])),cameras=len(doc.get('cameras',[])),materials=len(doc.get('materials',[])),anims=[[(c['target'].get('node')) for c in a['channels']] for a in doc.get('animations',[])],skinj=[(s.get('skeleton'),s.get('joints'),s.get('inverseBindMatrices')) for s in doc.get('skins',[])],animsamp=[[(s['input'],s['output']) for s in a['samplers']] for a in doc.get('animations',[])])
ks=sys.argv[1:] or list(d)
for k in ks:
    v=d[k]; b=load(v['fixture']['expected-before-gltf']); a=load(v['fixture']['expected-after-gltf'])
    sb,sa=summ(b),summ(a)
    print("=====",k, "|", v['notes'])
    for key in sb:
        if sb[key]!=sa[key]: print("  ",key,"BEFORE",json.dumps(sb[key])[:900]); print("  ",key,"AFTER ",json.dumps(sa[key])[:900])
