import os,re,sys,shutil,glob
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
import widen
from engine import ident_set
T=os.path.dirname(os.path.abspath(__file__))
SC=os.environ.get('RHS_SCRATCH','/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad')
REPO='/Users/ueli/Documents/semio/'
RX=re.compile(r'^\s*//#(region|endregion)\s*(.*?)\s*$')

def top_regions(ls):
    out={};stack=[]
    for i,l in enumerate(ls):
        m=RX.match(l)
        if not m:continue
        if m.group(1)=='region':
            stack.append((m.group(2),i))
        else:
            name,a=stack.pop()
            if not stack:
                assert name not in out,('duplicate region',name)
                out[name]=(a,i)
    assert not stack
    return out

SPECS={}
def spec(name,**kw):
    SPECS[name]=kw

# ---------------------------------------------------------------- model
spec('model',
 orig='orig/model.rs.txt',
 real='🧰️framework/🛍️products/🦑️repo/🔨️modules/📐️model/📦️packages/🦀️rust',
 crate='semio_framework_repo_model',
 root_name='🦀️.rs',
 children=[
  dict(mod='allowed_values',folder='📋️allowed-values',emoji='📋️',desc='The allowed LLM, effort and client tables embedded from the schema, and the slug vocabularies derived from them.',
       regions=['📋️Allowed Values','🔤️Slugs'],sub=[('include_str!\\("\\.\\./\\.\\./🧬️schema/','include_str!("../../../🧬️schema/')]),
  dict(mod='kinds',folder='🏷️kinds',emoji='🏷️',desc='The kind and status enumerations, the kind derivations, their emojis and the section and definition ids.',
       regions=['🏷️Enumerations','🏷️File Kinds','🔣️Kind Emojis','🪪️Section Ids']),
  dict(mod='repo_aggregates',folder='🌐️repo-aggregates',emoji='🌐️',desc='The repo, contributor, checkpoint and interaction aggregates with their metrics.',
       regions=['🔢️Metrics','🌐️Repo Aggregate','🧑️Contributor Aggregate','🔀️Checkpoint Aggregate','💬️Interaction Aggregate']),
  dict(mod='work_aggregates',folder='🎫️work-aggregates',emoji='🎫️',desc='The ticket, goal, draft, todo and statute aggregates.',
       regions=['🎫️Ticket Aggregate','🎯️Goal Aggregate','📝️Draft Aggregate','✅️Todo Aggregate','👮️Statute Aggregate']),
  dict(mod='codebase_aggregate',folder='🖥️codebase-aggregate',emoji='🖥️',desc='The codebase aggregate: bundles, folders, files, sections, definitions, breaches and contributor rollups.',
       regions=['🖥️Codebase Aggregate']),
  dict(mod='tree_aggregate',folder='🌳️tree-aggregate',emoji='🌳️',desc='The tree aggregate and the input types of the ticket, goal, todo and contributor commands.',
       regions=['🌳️Tree Aggregate','📥️Input Types']),
  dict(mod='ansi',folder='🎨️ansi',emoji='🎨️',desc='The ANSI escapes the human renderer paints with.',
       regions=['🎨️Ansi'],unwrap='ansi',vis='pub'),
  dict(mod='entity',folder='🪪️entity',emoji='🪪️',desc='Artifact identities, property projection and the entity templates the renderers print.',
       regions=['🪪️Entity'],unwrap='entity',vis='pub'),
 ],
 external=[],
 mount_style='first',
 parent_sub=[('embedded below with\n//! [`include_str!`]','embedded by `📋️allowed-values` with\n//! [`include_str!`]')],
)

# ---------------------------------------------------------------- tickets
spec('tickets',
 orig='orig/tickets.rs.txt',
 real='🧰️framework/🛍️products/🦑️repo/🔨️modules/🎫️tickets/📦️packages/🦀️rust',
 crate='semio_framework_repo_tickets',
 root_name='🦀️.rs',
 children=[
  dict(mod='ids',folder='🪪️ids',emoji='🪪️',desc='The dated ticket id and path scheme and the path helpers.',regions=['🪪️Ids']),
  dict(mod='codec',folder='📄️codec',emoji='📄️',desc='The Go-compatible JSON writer and the ticket document codec.',regions=['🔤️GoJson','📄️Codec']),
  dict(mod='store',folder='🗄️store',emoji='🗄️',desc='The ticket store port with its in-memory and file implementations.',regions=['🗄️Store']),
  dict(mod='documents',folder='💾️documents',emoji='💾️',desc='The important-document transaction and the plan source resolution.',regions=['💾️Important','🗺️PlanSource']),
  dict(mod='file_scope',folder='📐️file-scope',emoji='📐️',desc='The version-control port, the diff parser and the ticket file scope.',regions=['📐️FileScope']),
  dict(mod='issue_sync',folder='🐙️issue-sync',emoji='🐙️',desc='Issue tracker synchronisation for ticket open and close.',regions=['🐙️IssueSync']),
  dict(mod='ports',folder='⏰️ports',emoji='⏰️',desc='The clock, event sink and null issue tracker ports with their system and recording implementations.',regions=['⏰️Ports']),
  dict(mod='lifecycle',folder='🔓️lifecycle',emoji='🔓️',desc='The open, close, reopen and change lifecycle, the ticket service and the real-tree purge.',regions=['🔓️Lifecycle','🧹️RealPurge']),
  dict(mod='search',folder='🔎️search',emoji='🔎️',desc='The ticket query.',regions=['🔎️Search']),
 ],
 external=sorted(glob.glob(REPO+'🧰️framework/🛍️products/🦑️repo/🔨️modules/🎫️tickets/🧪️tests/**/*.rs',recursive=True)),
 mount_style='first',
)

# ---------------------------------------------------------------- gateway
spec('gateway',
 orig='orig/gateway.rs.txt',
 real='🧰️framework/🛍️products/🖥️server/🔨️modules/📡️gateway',
 crate='semio_framework_server',
 root_name='🦀️.rs',
 children=[
  dict(mod='error',folder='❌️error',emoji='❌️',desc='The server error, its wire body and response mapping, and the wire JSON extractor.',regions=['🔖️Error']),
  dict(mod='instance',folder='🧩️instance',emoji='🧩️',desc='The server instance, module and store vocabulary a deployment implements.',regions=['🔖️Instance','🔖️Module','🔖️Store']),
  dict(mod='document_port',folder='📄️document-port',emoji='📄️',desc='The document authority and query handler ports with their empty implementations.',regions=['🔖️DocumentPort','🔖️Query'],macro_use=True),
  dict(mod='fanout',folder='📣️fanout',emoji='📣️',desc='The event fanout, lane names and the kick map.',regions=['🔖️Fanout','🔖️Kick']),
  dict(mod='presence',folder='👥️presence',emoji='👥️',desc='The presence roster and the presence rooms with their settings, readers and admission.',regions=['🔖️Presence','🔖️PresenceRoom']),
  dict(mod='edge',folder='🛂️edge',emoji='🛂️',desc='CORS, client addressing, throttling and the credential readers at the edge.',regions=['🔖️Cors','🔖️Edge','🔖️Credential']),
  dict(mod='state',folder='🧠️state',emoji='🧠️',desc='The shared server state with its command lane, and the blob and command routes over it.',regions=['🔖️State','🔖️Blob','🔖️Command']),
  dict(mod='static_apps',folder='🗂️static-apps',emoji='🗂️',desc='The static app host, the install registry and the app routes.',regions=['🔖️StaticApp','🔖️Apps','🔖️AppRoutes']),
  dict(mod='streams',folder='🔁️streams',emoji='🔁️',desc='The event stream and the document stream sockets.',regions=['🔖️EventStream','🔖️DocumentStream']),
  dict(mod='presence_stream',folder='🛰️presence-stream',emoji='🛰️',desc='The presence socket: admission, the room pump and the watcher.',regions=['🔖️PresenceStream']),
  dict(mod='server',folder='🖥️server',emoji='🖥️',desc='The route groups, the builder and the running server.',regions=['🔖️Server']),
 ],
 external=sorted(glob.glob(REPO+'🧰️framework/🛍️products/🖥️server/🔨️modules/📡️gateway/🧪️tests/**/*.rs',recursive=True)),
 mount_style='first',
)

def build(name):
    sp=SPECS[name]
    text=open(os.path.join(T,sp['orig']),encoding='utf-8').read()
    ls=text.split('\n')
    if ls and ls[-1]=='':ls=ls[:-1]
    regs=top_regions(ls)
    owner={}
    for c in sp['children']:
        for r in c['regions']:
            assert r in regs,('unknown region',r)
            assert r not in owner,('region assigned twice',r)
            owner[r]=c['mod']
    first_of={}
    for c in sp['children']:
        first_of[min(regs[r][0] for r in c['regions'])]=c
    cover=[None]*len(ls)
    for r,(a,b) in regs.items():
        if r in owner:
            for i in range(a,b+1):cover[i]=owner[r]
    # parent
    plines=[]
    i=0
    mounted=set()
    while i<len(ls):
        if cover[i] is None:
            plines.append(ls[i]);i+=1;continue
        if i in first_of:
            c=first_of[i]
            if c.get('macro_use'):plines.append('#[macro_use]')
            plines.append('#[path = "%s/🦀️.rs"]'%c['folder'])
            plines.append('%smod %s;'%(('pub ' if c.get('vis')=='pub' else ''),c['mod']))
            if not c.get('unwrap'):
                plines.append('pub use %s::*;'%c['mod'])
        while i<len(ls) and cover[i] is not None:i+=1
    ptxt='\n'.join(collapse(plines))+'\n'
    for a,b in sp.get('parent_sub',[]):
        assert ptxt.count(a)==1,('parent_sub anchor',a)
        ptxt=ptxt.replace(a,b)
    files={}
    for c in sp['children']:
        body=[];moddoc=[]
        for r in c['regions']:
            a,b=regs[r]
            seg=ls[a:b+1]
            if c.get('unwrap'):
                # find 'pub mod NAME {' and its closing line
                s=[k for k,l in enumerate(seg) if re.match(r'^\s*pub mod %s \{\s*$'%c['unwrap'],l)]
                assert len(s)==1,(r,s)
                s=s[0]
                e=max(k for k,l in enumerate(seg) if l=='}')
                pre=seg[:s];inner=seg[s+1:e];post=seg[e+1:]
                ded=[]
                for l in inner:
                    if l.strip()=='':ded.append('')
                    else:
                        assert l.startswith('    '),(r,l)
                        ded.append(l[4:])
                mdoc=[l for l in pre if l.lstrip().startswith('///')]
                pre=[l for l in pre if not l.lstrip().startswith('///')]
                moddoc=[re.sub(r'^(\s*)///',r'\1//!',l) for l in mdoc]
                seg=pre+ded+post
            if body:body.append('')
            body.extend(seg)
        for rx,rp in c.get('sub',[]):
            body=[re.sub(rx,rp,l) for l in body]
        uses=['use super::*;',''] if not c.get('unwrap') else []
        doc='//! %s %s'%(c['emoji'],c['desc'])
        files[c['folder']]='\n'.join([doc]+moddoc+['']+uses+collapse(body))+'\n'
    return text,ls,regs,ptxt,files

def collapse(lines):
    out=[];blank=0
    for l in lines:
        if l.strip()=='':
            blank+=1
            if blank>1:continue
            out.append('')
        else:
            blank=0;out.append(l)
    while out and out[-1]=='':out.pop()
    return out

def stage(name,do_widen=True):
    sp=SPECS[name]
    text,ls,regs,ptxt,files=build(name)
    if isinstance(ptxt,list):ptxt='\n'.join(ptxt)+'\n'
    st=os.path.join(SC,'rhs-stage',name)
    shutil.rmtree(st,ignore_errors=True)
    os.makedirs(st)
    open(os.path.join(st,sp['root_name']),'w',encoding='utf-8').write(ptxt if isinstance(ptxt,str) else '\n'.join(ptxt)+'\n')
    for f,t in files.items():
        os.makedirs(os.path.join(st,f))
        open(os.path.join(st,f,'🦀️.rs'),'w',encoding='utf-8').write(t if isinstance(t,str) else '\n'.join(t)+'\n')
    allf={}
    for p in glob.glob(st+'/**/🦀️.rs',recursive=True):allf[p]=open(p,encoding='utf-8').read()
    ext={}
    for p in sp['external']:
        if os.path.exists(p):ext[p]=ident_set(open(p,encoding='utf-8').read())
    idents={p:ident_set(t) for p,t in allf.items()}
    rep=[]
    if do_widen:
        for p in sorted(allf):
            if p==os.path.join(st,sp['root_name']):continue
            if any(c.get('unwrap') and os.path.join(st,c['folder'],'🦀️.rs')==p for c in sp['children']):continue
            others=set()
            for q,s in idents.items():
                if q!=p:others|=s
            for s in ext.values():others|=s
            ed=widen.plan(allf[p],others)
            open(p,'w',encoding='utf-8').write(widen.apply(allf[p],ed))
            for e in ed:rep.append((os.path.relpath(p,st),)+e[2])
    open(os.path.join(SC,'rhs-stage',name+'.widen.txt'),'w',encoding='utf-8').write('\n'.join('\t'.join(r) for r in rep))
    print(name,'children',len(files),'widened',len(rep),{f:t.count('\n') for f,t in files.items()},'parent',ptxt.count('\n'))

if __name__=='__main__':
    stage(sys.argv[1],len(sys.argv)<3 or sys.argv[2]!='nowiden')
    import fmt_widened
    print('formatted hunks',fmt_widened.run(os.path.join(SC,'rhs-stage',sys.argv[1])))
