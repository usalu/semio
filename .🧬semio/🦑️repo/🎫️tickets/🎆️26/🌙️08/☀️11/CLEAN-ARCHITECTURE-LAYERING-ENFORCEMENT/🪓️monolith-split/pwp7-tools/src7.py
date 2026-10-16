import sys; sys.path.insert(0,'.')
from model7 import *
sys.path.insert(0,P6)
_c=os.getcwd(); os.chdir(P6)
from sig6 import parse_sig, match_close, match_angle, split_args, split_params
os.chdir(_c)
from plan7 import crate_of, ORDER, nodes, dom2
DOMAIN_OF={}   # name -> domain (fine) from dom2
for x in nodes: DOMAIN_OF[x['name']]=dom2(x)
# staging domain directories per crate and fine domain
STAGE={
 'render':{'gest':'🛠️tool-machine','settle':'🛠️tool-machine','hist':'📜️history-view','inter':'🧭️interaction','rjob':'🏷️reserved-jobs'},
 'dispatch':{'emit':'📤️emit','prod':'🎟️producers','host':'📣️host-events','tcmd':'⌨️typed-command','grphist':'🤝️group-history'},
 'typed-ops':{'tpub':'📬️publication','tret':'♻️retirement','grp':'🪆️child-group','tmhi':'⌨️typing'},
 'commands':{'resv':'🧊️reserved-routes','act':'🚦️actions','arch':'🗃️archive-merge'},
}
ROOT_OF={ # crate -> directory relative to PLUGP
 'render':'🧵️instance/🖼️render','dispatch':'🧵️instance/📨️dispatch','typed-ops':'🧵️instance/🧾️typed-operations','commands':'🧵️instance/🧷️command-routes',
 'tt-turns':'⏪️time-travel/🧵️instance','tr-turns':'⏯️tool-run/🧵️instance','ladders':'🪜️maintenance-ladder','app':'🧵️instance'}
CTX={'render':'RenderCtx','dispatch':'DispatchCtx','typed-ops':'TypedCtx','tt-turns':'TimeTravelCtx','tr-turns':'ToolRunCtx','commands':'CommandCtx','ladders':'LadderCtx'}
VIEW={'render':'RenderRef','tt-turns':'TimeTravelRef','tr-turns':'ToolRunRef','ladders':'LadderRef'}
SNAKE={'render':'render','dispatch':'dispatch','typed-ops':'typed','tt-turns':'time_travel','tr-turns':'tool_run','commands':'command','ladders':'ladder'}
SNAPDIR=GEN+'/pwp7-snap'
def load_pre(rel):
    """the pre-WP7 text of a family file: the snapshot when this slice touched it, else the live file"""
    if os.path.exists(SNAPDIR+'/'+rel): return parse_file(rel,plugp='',root=SNAPDIR+'/')
    return parse_file(rel)
def methods():
    """all members of impl VcsArtifactApp blocks of the family with crate, receiver kind, async flag"""
    out=[]
    for rel in TIER2_FILES:
        f=load_pre(rel)
        for pi,p in enumerate(f['pieces']):
            if p.kind!='vcsimpl': continue
            for mi,m in enumerate(p.members):
                txt='\n'.join(f['R'][m.a:m.e])
                sg=parse_sig(txt)
                composer_api = rel.endswith('composer-api/🦀️.rs')
                if composer_api: crate='composer-api'
                elif rel.endswith('plugin-app-impl/🦀️.rs') or rel.startswith('🧵️instance/📥️staging/') or rel=='🧵️instance/🦀️.rs': crate='app'
                else:
                    crate=crate_of_name(m.name, rel)
                out.append(dict(file=rel,piece=pi,idx=mi,name=m.name,trait=p.trait,vis=m.vis,a=m.a,e=m.e,text=txt,masked=mask(txt),sig=sg,
                    recv=(sg['recv']['kind'] if sg['recv'] else None),is_async=sg['is_async'],crate=crate,head=p.head_lines))
    return out
_NAMES={}
def crate_of_name(name, rel):
    if not _NAMES:
        for x in nodes: _NAMES.setdefault(x['name'],[]).append(x)
    xs=_NAMES.get(name)
    if not xs: raise KeyError(name)
    for x in xs:
        if x['file'].split('/')[-2] in rel or True: pass
    return crate_of(xs[0])
if __name__=='__main__':
    ms=methods()
    print(len(ms), collections.Counter(m['crate'] for m in ms))
    print(collections.Counter((m['crate'],m['recv']) for m in ms if m['crate'] not in('app','composer-api')))
