import json,re,sys,copy,hashlib
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
N="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/🔣️.json"
d=json.load(open(T+"/nxpart-tools/backup/tests.fixture.json",encoding='utf8'))
PAT=re.compile(r'(\.cache/b)/g/m/nx/cache')
def pm(o):
    if isinstance(o,str): return PAT.sub(r'\1/n',o)
    if isinstance(o,list): return [pm(x) for x in o]
    if isinstance(o,dict): return {k:pm(v) for k,v in o.items()}
    return o
def rehost(obs):
    o=copy.deepcopy(obs)
    pool,ns=o['images'][0],o['images'][1]
    o['host']={k:pool[k] for k in('directory','root','backing','policy','journal','associations')}
    o['images']=[ns]
    return o
def observation_rows(rows,drop=(),overlay=False):
    out=[]
    base=None
    for r in rows:
        if r['name'] in drop: continue
        n=copy.deepcopy(r);o=pm(n['observation']);name=n['name']
        old_host_journal=copy.deepcopy(o['host']['journal'])
        if name=='missing chain image':
            o=rehost_from(base,o)
            o['images']=[]
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
def rehost_from(base,o):
    b=copy.deepcopy(base);b['operation']=o['operation'];return b
d['currentMaintenanceObservations']=observation_rows(d['currentMaintenanceObservations'])
d['currentMaintenancePresenceObservations']=observation_rows(d['currentMaintenancePresenceObservations'],drop=('pool-absent',),overlay=True)
json.dump(d,open(N,'w',encoding='utf8'),ensure_ascii=False,indent=2);open(N,'a').write("\n")
