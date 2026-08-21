import json, copy, os, re, subprocess
exec(open('gen.py').read())

def compact(v):
    return json.dumps(v, separators=(',',':'), ensure_ascii=False)

def sip(strings):
    p=subprocess.run(['./hashtool'], input='\n'.join(strings)+'\n', capture_output=True, text=True)
    return p.stdout.strip().split('\n')

OUT={}   # leafdir -> dict(case=..., files={rel:content}, applied=bool, meta=...)

def build_regular(d, p):
    etype=p['etype']; slug=p['entity']; title=title_of(slug)
    rid=f"{slug}-a"; rname=f"{title} A"
    coll=camel(p['coll']); dfield=camel(p['diff_field'])
    base_row=row(etype, rid, rname)
    b=base_snapshot(); a=base_snapshot()
    dj=empty_diff()
    if p['verb']=='create':
        b[coll]=[]; a[coll]=[copy.deepcopy(base_row)]
        dj[dfield]=delta(added=[copy.deepcopy(base_row)])
        mut=mutation_json(p['struct'], {camel(p['payload_field']): copy.deepcopy(base_row)})
        case_name=f"creates-{slug}-a"
    elif p['verb']=='delete':
        b[coll]=[copy.deepcopy(base_row)]; a[coll]=[]
        dj[dfield]=delta(removed=[rid])
        mut=mutation_json(p['struct'], {"id": rid})
        case_name=f"deletes-{slug}-a"
    elif p['verb']=='rename':
        new_name=f"Renamed {title} A"
        after_row=copy.deepcopy(base_row); after_row['name']=new_name
        b[coll]=[copy.deepcopy(base_row)]; a[coll]=[after_row]
        dj[dfield]=delta(patched=[{"id": rid, "patch": rename_patch(etype, new_name)}])
        mut=mutation_json(p['struct'], {"id": rid, "newName": new_name})
        case_name=f"renames-{slug}-a"
    elif p['verb']=='replace':
        fname, fty = pick_change(etype)
        newv=changed_value(fty, title)
        after_row=copy.deepcopy(base_row)
        set_path(after_row, etype, fname, newv)
        b[coll]=[copy.deepcopy(base_row)]; a[coll]=[copy.deepcopy(after_row)]
        dj[dfield]=delta(patched=[{"id": rid, "patch": full_patch(etype, after_row)}])
        mut=mutation_json(p['struct'], {camel(p['payload_field']): copy.deepcopy(after_row)})
        case_name=f"replaces-{slug}-a"
        p['changed_field']=fname; p['changed_json_key']=camel(fname.split('.')[-1]); p['changed_value']=newv
    else: raise SystemExit(d)
    return case_name, b, a, mut, dj, {'rid':rid,'rname':rname,'title':title,'coll':coll,'dfield':dfield}

def build_edge(d, p):
    dj=empty_diff()
    if d=='🔗🧲connect-adjacency':
        el=lambda i,n: row('ProgramElement', i, n)
        e1=el('element-a','Reception'); e2=el('element-b','Waiting')
        adj=row('Adjacency','adjacency-a','Reception To Waiting')
        adj['elementAId']='element-a'; adj['elementBId']='element-b'
        payload=copy.deepcopy(adj); payload['normalized']=False
        stored=copy.deepcopy(adj); stored['normalized']=True
        b=base_snapshot(); b['elements']=[copy.deepcopy(e1),copy.deepcopy(e2)]; b['adjacencies']=[]
        a=base_snapshot(); a['elements']=[copy.deepcopy(e1),copy.deepcopy(e2)]; a['adjacencies']=[copy.deepcopy(stored)]
        dj['adjacencies']=delta(added=[copy.deepcopy(stored)])
        mut=mutation_json(p['struct'], {"adjacency": copy.deepcopy(payload)})
        return "connects-reception-to-waiting", b, a, mut, dj, {'rid':'adjacency-a'}
    if d=='✂️🧲disconnect-adjacency':
        adj=row('Adjacency','adjacency-a','Reception To Waiting')
        adj['elementAId']='element-a'; adj['elementBId']='element-b'; adj['normalized']=True
        e1=row('ProgramElement','element-a','Reception'); e2=row('ProgramElement','element-b','Waiting')
        b=base_snapshot(); b['elements']=[copy.deepcopy(e1),copy.deepcopy(e2)]; b['adjacencies']=[copy.deepcopy(adj)]
        a=base_snapshot(); a['elements']=[copy.deepcopy(e1),copy.deepcopy(e2)]; a['adjacencies']=[]
        dj['adjacencies']=delta(removed=['adjacency-a'])
        mut=mutation_json(p['struct'], {"id":"adjacency-a"})
        return "disconnects-reception-from-waiting", b, a, mut, dj, {'rid':'adjacency-a'}
    if d=='🔗🧵connect-trace':
        tr=row('TraceLink','trace-a',None)
        tr['fromId']='requirement-a'; tr['toId']='decision-a'; tr['kind']='requirementToDecision'
        b=base_snapshot(); b['traces']=[]
        a=base_snapshot(); a['traces']=[copy.deepcopy(tr)]
        dj['traces']=delta(added=[copy.deepcopy(tr)])
        mut=mutation_json(p['struct'], {"trace": copy.deepcopy(tr)})
        return "connects-requirement-a-to-decision-a", b, a, mut, dj, {'rid':'trace-a'}
    if d=='✂️🧵disconnect-trace':
        tr=row('TraceLink','trace-a',None)
        tr['fromId']='requirement-a'; tr['toId']='decision-a'; tr['kind']='requirementToDecision'
        b=base_snapshot(); b['traces']=[copy.deepcopy(tr)]
        a=base_snapshot(); a['traces']=[]
        dj['traces']=delta(removed=['trace-a'])
        mut=mutation_json(p['struct'], {"id":"trace-a"})
        return "disconnects-requirement-a-from-decision-a", b, a, mut, dj, {'rid':'trace-a'}
    raise SystemExit(d)

SCALAR_SPEC={
 '✏️🏷️rename-meta':      ('meta','title','newTitle','Fixture Program','Clinic Program','renames-the-document-title','document-fixture'),
 '✏️📁rename-project':    ('project','code','newCode','FIX-000','CLN-001','renames-the-project-code','project-fixture'),
 '✏️🏛️rename-governance': ('governance','framework','newFramework','ISO 9001','ISO 41001','renames-the-governance-framework','governance-fixture'),
}
REPLACE_SPEC={
 '🔁🏷️replace-meta':      ('meta','ProgramMeta','newMeta','industrySector','healthcare','replaces-the-document-meta-block','document-fixture'),
 '🔁📁replace-project':    ('project','ProjectDefinition','newProject','clientName','Sample Health','replaces-the-project-definition','project-fixture'),
 '🔁🏛️replace-governance': ('governance','Governance','newGovernance','riskAppetite','Low','replaces-the-governance-block','governance-fixture'),
}

def build_scalar(d, p):
    dj=empty_diff()
    if d in SCALAR_SPEC:
        field, rustf, payload_key, old, new, case_name, target = SCALAR_SPEC[d]
        b=base_snapshot(); a=base_snapshot()
        assert b[field][camel(rustf)]==old, (d, b[field][camel(rustf)])
        a[field][camel(rustf)]=new
        dj[field]=copy.deepcopy(a[field])
        mut=mutation_json(p['struct'], {payload_key:new})
        return case_name, b, a, mut, dj, {'target':target,'field':field,'key':camel(rustf),'old':old,'new':new}
    field, sty, payload_key, jkey, new, case_name, target = REPLACE_SPEC[d]
    b=base_snapshot(); a=base_snapshot()
    a[field][jkey]=new
    dj[field]=copy.deepcopy(a[field])
    mut=mutation_json(p['struct'], {payload_key: copy.deepcopy(a[field])})
    return case_name, b, a, mut, dj, {'target':target,'field':field,'key':jkey,'new':new}

CHILD_SPEC={
 '🌱📚create-knowledge-record': ('knowledge','KnowledgeRecord','knowledgeRecord','knowledge-record-a','Knowledge Record A'),
 '🌱🏁create-benchmark-record': ('benchmarks','BenchmarkRecord','benchmarkRecord','benchmark-record-a','Benchmark Record A'),
}
def build_child(d, p):
    slug=p['entity']; title=title_of(slug); rid=f"{slug}-a"; rname=f"{title} A"
    dj=empty_diff()
    if d in CHILD_SPEC:
        slot, etype, pkey, rid, rname = CHILD_SPEC[d]
        r=row(etype, rid, rname)
        compact_records=compact([r])
        h=sip([compact_records])[0]
        handle=child(slot, h)
        b=base_snapshot(); a=base_snapshot(); a[slot]=copy.deepcopy(handle)
        dj[slot]=copy.deepcopy(handle)
        mut=mutation_json(p['struct'], {pkey: copy.deepcopy(r)})
        return f"creates-{rid}", b, a, mut, dj, {'rid':rid,'slot':slot,'hash':h,'records_json':compact_records}, True
    # rejected: cold working-scene cache => the row is not reachable
    slot='knowledge' if 'knowledge' in slug else 'benchmarks'
    b=base_snapshot(); a=base_snapshot()
    if p['verb']=='delete':
        mut=mutation_json(p['struct'], {"id": rid}); case_name=f"rejects-deleting-absent-{rid}"
    elif p['verb']=='rename':
        mut=mutation_json(p['struct'], {"id": rid, "newName": f"Renamed {title} A"}); case_name=f"rejects-renaming-absent-{rid}"
    else:
        etype='KnowledgeRecord' if slot=='knowledge' else 'BenchmarkRecord'
        pkey=camel([f[0] for f in p['fields'] if f[1]!='EntityId'][0])
        mut=mutation_json(p['struct'], {pkey: row(etype, rid, rname)}); case_name=f"rejects-replacing-absent-{rid}"
    return case_name, b, a, mut, None, {'rid':rid,'slot':slot}, False

RESULT={}
for d, p in plan.items():
    if p['cat']=='regular':
        cn,b,a,mut,dj,info=build_regular(d,p); applied=True
    elif p['cat']=='edge':
        cn,b,a,mut,dj,info=build_edge(d,p); applied=True
    elif p['cat']=='scalar':
        cn,b,a,mut,dj,info=build_scalar(d,p); applied=True
    else:
        cn,b,a,mut,dj,info,applied=build_child(d,p)
    RESULT[d]={'case':cn,'before':b,'after':a,'mutation':mut,'diff':dj,'info':info,'applied':applied,'plan':p,'doc':doc_of(d)}
json.dump(RESULT, open('fixtures.json','w'), ensure_ascii=False)
print('built', len(RESULT), 'applied', sum(1 for r in RESULT.values() if r['applied']))
import collections
print(collections.Counter(r['case'] for r in RESULT.values()).most_common(3))
