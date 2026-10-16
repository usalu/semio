import os,re,sys,shutil,json
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from carve import *

T=os.path.dirname(os.path.abspath(__file__))
SC=os.environ.get('RHS_SCRATCH','/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad')

CHILDREN=[
 ('schema','🗃️table-schema','🗃️','The table DDL and constraint text the backend applies when it opens a fresh store.'),
 ('connection','🔌️connection','🔌️','The backend handle, its constructors and seeding, and the test-only fault hooks.'),
 ('rows','🧱️rows','🧱️','Row decoders, select lists and the small value helpers every section shares.'),
 ('projections','📽️projections','📽️','Event persistence and the projection of directory events into the read tables.'),
 ('artifact_creation','🏭️artifact-creation','🏭️','Artifact creation claims, facts, recovery and document genesis.'),
 ('capabilities','🎟️capabilities','🎟️','Share tokens and invites: the capability issuers and their atomic redemption.'),
 ('accounts','👤️accounts','👤️','Users, auth sessions, credentials and agent delegations.'),
 ('spaces','🏘️spaces','🏘️','Spaces, memberships, document descriptors and artifact checkpoint reads.'),
 ('administration','🛡️administration','🛡️','Admin operation audit, directory command receipts and checkpoint publication claims.'),
 ('sync_sessions','🔄️sync-sessions','🔄️','Document sync session bookkeeping.'),
 ('artifact_cas','🧺️artifact-cas','🧺️','Artifact CAS reservations, sweep candidates and delete fences.'),
 ('event_log','📜️event-log','📜️','The append-only directory event log and projection rebuild.'),
]
CH={c[0]:c for c in CHILDREN}

REGION_OWNER={'ShareTokens':'capabilities','Invites':'capabilities','Users':'accounts','AuthSessions':'accounts','Spaces':'spaces','Documents':'spaces','AdminOperations':'administration','CommandReceipts':'administration','SyncSessions':'sync_sessions','EventLog':'event_log'}

CREATION_TRAIT={'claim_artifact_creation','read_artifact_creation','append_artifact_creation_fact','artifact_creation_terminate_uncommitted','artifact_creation_recovery_candidates','append_document_genesis'}

def trait_owner(name,region):
    if name in CREATION_TRAIT:return 'artifact_creation'
    if region in REGION_OWNER:return REGION_OWNER[region]
    if 'artifact_cas' in name or name=='append_reserved_artifact_checkpoint':return 'artifact_cas'
    raise Exception('unowned trait member '+name+' region '+str(region))

CONN_METHODS=re.compile(r'^(connect|lock|seed|fail_next_.*|install_.*|clear_.*|pause_genesis_before_authority_for_test)$')

def inherent_owner(name,backend):
    if CONN_METHODS.match(name):return 'connection'
    if name in('creation_facts','creation_authority','insert_creation_fact'):return 'artifact_creation'
    if name.endswith('_attempt'):
        if 'artifact_creation' in name or 'document_genesis' in name:return 'artifact_creation'
        if 'invite' in name:return 'capabilities'
        if 'artifact_cas' in name or 'reserved_artifact_checkpoint' in name:return 'artifact_cas'
        if 'decided_events' in name:return 'event_log'
    if name.startswith('revoke_auth_sessions'):return 'accounts'
    if name.startswith('cas_') or name=='append_cas_release':return 'artifact_cas'
    if name in('persist_event','persist_event_with_identity','project_verified_checkpoint','projection_rejection','project'):return 'projections'
    raise Exception('unowned inherent member '+name)

def top_owner(p,backend):
    k,n=p.kind,p.name
    if k=='use':return 'parent'
    if k=='mod':return 'parent'
    if k=='macro':return 'parent'
    if k=='const' and n=='SCHEMA':return 'schema'
    if k=='struct' and n in('PostgresDirectory','SqliteDirectory','Neo4jDirectory','ArtifactGenesisTestControlV1'):return 'connection'
    if k=='impl' and 'ArtifactGenesisTestControlV1' in n:return 'connection'
    if n in('insert_auth_audit',):return 'accounts'
    if n in('insert_admin_operation_effect_receipt','admin_effect_rollback'):return 'administration'
    if n in('artifact_creation_phase','lock_artifact_creation_request','artifact_creation_facts','validate_artifact_creation_authority','insert_artifact_creation_fact'):return 'artifact_creation'
    if n.startswith('cas_'):return 'artifact_cas'
    if k=='impl' and 'HubDirectoryfor' in n:return 'TRAITIMPL'
    if k=='impl' and n.startswith('Neo4jDirectory') or k=='impl' and n.startswith('SqliteDirectory') or k=='impl' and n.startswith('PostgresDirectory'):return 'INHERENT'
    return 'rows'

def region_events(ls,lo,hi):
    ev=[]
    for i in range(lo,hi+1):
        m=re.match(r'^\s*//#(region|endregion)\s*(\S*)',ls[i])
        if m:ev.append((i,m.group(1),m.group(2)))
    return ev

def region_at(events,line):
    cur=None
    for i,kind,name in events:
        if i>line:break
        cur=name if kind=='region' else None
    return cur

def run(backend,stage,only_report=False):
    src_path=os.path.join(T,'orig/%s.rs.txt'%backend)
    text,ls,rs=parse_file(src_path)
    pcs=build_pieces(ls,rs)
    typename={'sqlite':'SqliteDirectory','postgres':'PostgresDirectory','neo4j':'Neo4jDirectory'}[backend]
    child_pieces={c[0]:[] for c in CHILDREN}
    parent_pieces=[]
    trait_piece=None
    plan=[]
    for p in pcs:
        if p.kind in('endmark','loose'):
            p.owner=None
            continue
        o=top_owner(p,backend)
        p.owner=o
    # resolve markers at top level before expanding impls
    top=[p for p in pcs]
    resolve_markers(top)
    flat=[]
    for p in top:
        if p.owner=='INHERENT':
            header,mem,bl,el=impl_piece_members(ls,p)
            ev=region_events(ls,bl,el)
            for m in mem:
                if m.kind in('endmark','loose'):
                    m.owner=None;m.impl=('inherent',header);flat.append(m);continue
                m.owner=inherent_owner(m.name,backend)
                m.impl=('inherent',header)
                flat.append(m)
            p.members=mem
        elif p.owner=='TRAITIMPL':
            header,mem,bl,el=impl_piece_members(ls,p)
            ev=region_events(ls,bl,el)
            for m in mem:
                if m.kind in('endmark','loose'):
                    m.owner=None;m.impl=('trait',header);flat.append(m);continue
                assert m.kind=='fn',(m.kind,m.name)
                rg=region_at(ev,m.start+len([1 for l in m.lines if False]))
                # region marker inside piece lines? use the line of the fn header
                hdr=m.start
                for off,l in enumerate(m.lines):
                    if FNLINE.match(l):hdr=m.start+off;break
                rg=region_at(ev,hdr)
                m.owner=trait_owner(m.name,rg)
                m.impl=('trait',header)
                flat.append(m)
            p.members=mem
            trait_piece=p
        else:
            flat.append(p)
    # resolve markers inside impls: endmark -> previous owner within same impl, loose -> next
    seq=[]
    cur=None
    for x in flat:seq.append(x)
    # group by impl identity for marker resolution
    def resolve_group(group):
        resolve_markers(group)
    cur_group=[];cur_key=None
    for x in seq:
        key=x.impl[0]+str(x.impl[1][0]) if x.impl else None
        if key!=cur_key and cur_group:
            resolve_group(cur_group);cur_group=[]
        cur_key=key
        cur_group.append(x)
    if cur_group:resolve_group(cur_group)
    return text,ls,pcs,flat,trait_piece,typename

if __name__=='__main__':
    b=sys.argv[1]
    text,ls,pcs,flat,tp,tn=run(b,None)
    import collections
    cnt=collections.Counter();lines=collections.Counter()
    for x in flat:
        cnt[x.owner]+=1;lines[x.owner]+=len(x.lines)
    for k in sorted(cnt):print(k,cnt[k],lines[k])


def assemble(backend):
    text,ls,pcs,flat,tp,tn=run(backend,None)
    out={}
    toplevel_by_child={c[0]:False for c in CHILDREN}
    for x in flat:
        if x.owner in toplevel_by_child and not x.impl:toplevel_by_child[x.owner]=True
    used=[c for c in CHILDREN if any(x.owner==c[0] for x in flat)]
    def emit(owner):
        lines=[];cur=None;first=True
        for x in flat:
            if x.owner!=owner:continue
            if x.impl:
                key=x.impl[0]+str(x.impl[1][0])
                if cur!=key:
                    if cur is not None:lines.append('}')
                    if not first:lines.append('')
                    if x.impl[0]=='trait':lines.append('impl %s {'%tn)
                    else:lines.extend(x.impl[1])
                    cur=key;first=False
                    firstm=True
                else:firstm=False
                body=x.lines
                if x.kind=='fn' and x.impl[0]=='trait':body=insert_vis(body)
                if x.kind=='endmark':lines.extend(body)
                else:
                    if not firstm:lines.append('')
                    lines.extend(body)
            else:
                if cur is not None:lines.append('}');cur=None
                if x.kind=='endmark':lines.extend(x.lines)
                else:
                    if not first:lines.append('')
                    lines.extend(x.lines)
                first=False
        if cur is not None:lines.append('}')
        return lines
    for c in used:
        name,folder,emoji,desc=c
        doc='//! %s %s'%(emoji,desc)
        body=emit(name)
        head=[doc,'']+([] if name=='schema' else ['use super::*;',''])
        out[folder]='\n'.join(head+body)+'\n'
    # delegates
    dl=['/// 🚪️ The port adapter: every method forwards to the section module that owns it, an inherent `pub(super)` method of the same name, so the one `HubDirectory` impl stays a single block while its bodies live in cohesive children. See [`HubDirectory`].','impl HubDirectory for %s {'%tn]
    names=[]
    for m in tp.members:
        if m.kind!='fn':continue
        d,n=delegate_lines(m,ls)
        dl.extend(d);names.append(n)
    dl.append('}')
    # mounts
    mounts=[]
    for c in used:
        name,folder,emoji,desc=c
        mounts.append('#[path = "%s/🦀️.rs"]'%folder)
        mounts.append('mod %s;'%name)
        if toplevel_by_child[name]:
            vis='pub' if name in('connection','rows') else 'pub(crate)'
            mounts.append('%s use %s::*;'%(vis,name))
        mounts.append('')
    plines=[];inserted=False
    ppieces=[x for x in flat if x.owner=='parent' or (x.impl is None and x.owner=='TRAITIMPL')]
    for x in flat:
        if x.owner=='TRAITIMPL' and x.impl is None:continue
    for x in flat:
        if x.impl:continue
        if x.owner=='parent':
            if x.kind=='mod' and not inserted:
                plines.append('');plines.extend(mounts);plines.extend(dl);plines.append('');inserted=True
            if x.kind=='endmark':plines.extend(x.lines)
            else:
                if plines and x.kind not in('use',):plines.append('')
                plines.extend(x.lines)
    if not inserted:
        plines.append('');plines.extend(mounts);plines.extend(dl)
    out['🦀️.rs']='\n'.join(collapse(plines))+'\n'
    return out,names
