import json,re,sys,copy,hashlib
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
N="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/🔣️.json"
d=json.load(open(T+"/nxpart-tools/backup/tests.fixture.json",encoding='utf8'))
FIX="/workspace/.cache/b/p/m/gf-Fk58e1"
OLD_OWNER="repository:nx-graph-pool";NEW_OWNER="repository:nx-store"
PAT=re.compile(r'(\.cache/b)/g/m/nx/cache')
NS=re.compile(r'(\.cache/b/n/m)/nx/graphs/([a-f0-9]{20})/([a-f0-9]{20})/native')
def pm(o):
    if isinstance(o,str): return NS.sub(r'\1/\2\3/native',PAT.sub(r'\1/n',o))
    if isinstance(o,list): return [pm(x) for x in o]
    if isinstance(o,dict): return {k:pm(v) for k,v in o.items()}
    return o
def pmfix(o):
    if isinstance(o,str): return NS.sub(r'\1/\2\3/native',o.replace('/workspace/.cache/b/g/m/nx/cache',FIX+'/.cache/b/n'))
    if isinstance(o,list): return [pmfix(x) for x in o]
    if isinstance(o,dict): return {k:pmfix(v) for k,v in o.items()}
    return o
def rehost(obs):
    o=copy.deepcopy(obs)
    pool,ns=o['images'][0],o['images'][1]
    o['host']={k:pool[k] for k in('directory','root','backing','policy','journal','associations')}
    o['images']=[ns]
    return o
def rehost_from(base,o):
    b=copy.deepcopy(base);b['operation']=o['operation'];return b
def observation_rows(rows,drop=(),overlay=False):
    out=[];base=None
    for r in rows:
        if r['name'] in drop: continue
        n=copy.deepcopy(r);o=pm(n['observation']);name=n['name']
        old_host_journal=copy.deepcopy(o['host']['journal'])
        if name=='missing chain image':
            o=rehost_from(base,o);o['images']=[]
        elif name=='wrong chain order':
            ob=rehost_from(base,o);ob['images']=[copy.deepcopy(base['images'][0]),copy.deepcopy(base['images'][0])];o=ob
        else:
            o=rehost(o)
        if name=='wrong host directory':o['host']['directory']='/elsewhere'
        if name=='wrong host policy':o['host']['policy']['capacityBytes']=1
        if name=='wrong host backing':o['host']['journal']['backing']['identity']='other'
        if name=='missing selected association':o['host']['associations']=[]
        if name=='duplicate association':o['host']['associations']=[copy.deepcopy(o['images'][0]['association']) for _ in range(2)]
        if overlay:
            for k in('presence','state','journalSha256','journalBytes'):
                if k in old_host_journal:o['host']['journal'][k]=old_host_journal[k]
                else:o['host']['journal'].pop(k,None)
        n['observation']=o
        if base is None:base=o
        out.append(n)
    return out
d['currentMaintenanceObservations']=observation_rows(d['currentMaintenanceObservations'])
d['currentMaintenancePresenceObservations']=observation_rows(d['currentMaintenancePresenceObservations'],drop=('pool-absent',),overlay=True)

def roster_key(a): return (len(a['root']),a['root'])
owner_t=copy.deepcopy([r for r in d['currentMaintenanceOwnerObservations'] if r['name']=='current Products namespace publication'][0])
artifact_t=copy.deepcopy([r for r in d['currentArtifactObservations'] if r['name']=='Products Graph complete current receipt payload'][0])
def tmpl_assoc(t,suffix):
    return [a for a in t['observation']['host']['associations'] if a['root'].endswith(suffix)][0]
def to_fixture_pool_row(old,tmpl):
    n=copy.deepcopy(old);o=pmfix(n['observation']);t=copy.deepcopy(tmpl)
    n['intent']=copy.deepcopy(t['intent']);ns=old['intent']['selection']['namespaces'];n['intent']['selection']['namespaces']=copy.deepcopy(ns)
    pool=o['images'][0]
    pool['policy']['leaseDirectory']=FIX+'/.cache/b/l/m/data'
    host=pm(copy.deepcopy(t['observation']['host']))
    graph_a=tmpl_assoc(t,'/.cache/b/g/m');lease_a=tmpl_assoc(t,'/.cache/b/l/m')
    roster=[copy.deepcopy(graph_a),copy.deepcopy(lease_a),copy.deepcopy(pool['association'])]+copy.deepcopy(pool['associations'])
    host['associations']=sorted(roster,key=roster_key)
    o['host']=host
    o['fixture']=pm(copy.deepcopy(t['observation']['fixture']));o['producer']=pm(copy.deepcopy(t['observation']['producer']))
    n['observation']=o
    return n
def pool_receipt(row):
    o=row['observation'];allocs=o['images'][0]['journal'].get('state',{}).get('allocations',[])
    if not allocs or o.get('payload') is None: return
    a=allocs[0];text=json.dumps({"version":1,"allocation":a['id'],"owner":a['owner'].replace(OLD_OWNER,NEW_OWNER),"bytes":a['bytes']},separators=(',',':'))+"\n"
    facts={"dev":"20","ino":"410","mode":33188,"nlink":1,"size":len(text.encode()),"mtimeNs":"3","ctimeNs":"4"}
    stamp=":".join(str(facts[k]) for k in("dev","ino","mode","nlink","size","mtimeNs","ctimeNs"))
    p=o['payload'];root=p['root']
    p['entries'].append({"path":"nx.json","kind":"file","stamp":stamp,"sha256":hashlib.sha256(text.encode()).hexdigest()})
    p['entries'].append({"path":"tmp","kind":"directory","stamp":"20:420:16877:2:512:3:4","sha256":None})
    p['entries'].sort(key=lambda e:e['path'].encode())
    o['payloadFiles'].append({"path":root+"/nx.json","text":text,"before":facts,"after":dict(facts)})
    kn=row.get('known')
    if kn is not None:
        kn['files'].append({"path":"nx.json","text":text});kn['directories'].append("tmp")
def fix_pool(o):
    if isinstance(o,dict):
        if str(o.get('directory','')).endswith('/.cache/b/n') and ('policy' in o or 'journal' in o):
            def sw(m,k,old_value,new_value):
                if m.get(k)==old_value:m[k]=new_value
            if 'policy' in o:
                sw(o['policy'],'capacityBytes',939524096,1610612736);sw(o['policy'],'maximumWorkspaceBytes',938475520,1609564160);sw(o['policy'],'maximumArtifactBytes',938475520,536870912)
            if 'backing' in o:sw(o['backing'],'maximumPhysicalBytes',939524096,1610612736)
            if 'journal' in o:
                sw(o['journal']['backing'],'maximumPhysicalBytes',939524096,1610612736)
                for a in o['journal'].get('state',{}).get('allocations',[]):
                    if a.get('owner')==OLD_OWNER and a.get('bytes')==938475520:a['bytes']=536870912
        for v in o.values():fix_pool(v)
    elif isinstance(o,list):
        for v in o:fix_pool(v)
owner_rows=d['currentMaintenanceOwnerObservations']
pool_names={'current pool row publication','pool live row retirement','pool foreign owner','pool repriced row','pool retiring row','known zero pool retirement'}
new_owner=[]
for r in owner_rows:
    if r['name'] in pool_names:
        new_owner.append(to_fixture_pool_row(r,owner_t))
    elif r['observation']['images'][0]['directory'].endswith('/.cache/b/g'):
        n=copy.deepcopy(r);n['observation']=pm(n['observation']);n['observation']['images']=n['observation']['images'][1:]
        pool=n['observation']['images'][0];pool['policy']['leaseDirectory']='/workspace/.cache/b/l/m/data' if n['name']=='Products global versus local graph lease' else FIX+'/.cache/b/l/m/data'
        if n['name']=='Products global versus local graph lease':n['name']='Products global versus local pool lease'
        new_owner.append(n)
    else:
        new_owner.append(r)
d['currentMaintenanceOwnerObservations']=new_owner
art=[]
for r in d['currentArtifactObservations']:
    if r['observation']['images'][0]['directory'].endswith('/.cache/b/g/m/nx/cache'):
        n=to_fixture_pool_row(r,artifact_t);fix_pool(n);pool_receipt(n);art.append(n)
    else:
        n=copy.deepcopy(r);n['observation']=pm(n['observation']);o=n['observation']
        p=o.get('payload')
        if p:
            p['entries']=[e for e in p['entries'] if e['path']!='nx']
        kn=n.get('known')
        if kn and 'nx' in kn.get('directories',[]) : kn['directories']=[x for x in kn['directories'] if x!='nx']
        art.append(n)
d['currentArtifactObservations']=art
fix_pool(d['currentMaintenanceObservations']);fix_pool(d['currentMaintenancePresenceObservations']);fix_pool(d['currentMaintenanceOwnerObservations']);fix_pool(d['currentArtifactObservations'])
# every other list: global path map
for k in list(d):
    if k in('currentMaintenanceObservations','currentMaintenancePresenceObservations','currentMaintenanceOwnerObservations','currentArtifactObservations'):continue
    d[k]=pm(d[k])
# owner rename everywhere
s=json.dumps(d,ensure_ascii=False).replace(OLD_OWNER,NEW_OWNER)
d=json.loads(s)
def fix_producer_sizes(o):
    if isinstance(o,dict):
        if 'text' in o and 'path' in o and str(o['path']).endswith('producer.log') and 'before' in o:
            size=len(o['text'].encode())
            o['before']['size']=size;o['after']['size']=size
        for v in o.values():fix_producer_sizes(v)
    elif isinstance(o,list):
        for v in o:fix_producer_sizes(v)
fix_producer_sizes(d)
# fixture entries: pool directory present in every fixture
def add_entries(o):
    if isinstance(o,dict):
        if 'entries' in o and 'directory' in o and isinstance(o['entries'],list) and o['entries'] and all(isinstance(e,dict) and 'kind' in e and 'path' in e for e in o['entries']) and o['directory'].endswith('gf-Fk58e1'):
            paths=[e['path'] for e in o['entries']]
            n=o['directory']+'/.cache/b/n'
            if n not in paths and o['directory']+'/.cache/b/l' in paths:
                o['entries'].append({"path":n,"kind":"directory"})
        for v in o.values():add_entries(v)
    elif isinstance(o,list):
        for v in o:add_entries(v)
add_entries(d)
# producer sha
def producers(o,acc):
    if isinstance(o,dict):
        if 'producer' in o and isinstance(o['producer'],dict) and 'text' in o['producer'] and o['producer']['path'].endswith('producer.log'): acc.append(o['producer']['text'])
        for v in o.values():producers(v,acc)
    elif isinstance(o,list):
        for v in o:producers(v,acc)
acc=[];producers(d,acc)
old_sha="ef015073a6a66b034a9d1cab63b6e81398bf9685ecae2c308b4ea7247b598f5c"
base=[r for r in d['currentMaintenanceOwnerObservations'] if r['name']=='current Products namespace publication'][0]['observation']['producer']['text']
new_sha=hashlib.sha256(base.encode()).hexdigest()
d=json.loads(json.dumps(d,ensure_ascii=False).replace(old_sha,new_sha))
print("producer texts",len(acc),len(set(acc)),"sha",new_sha[:8])
json.dump(d,open(N,'w',encoding='utf8'),ensure_ascii=False,indent=2);open(N,'a').write("\n")
