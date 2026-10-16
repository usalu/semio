import sys, os, re, json
sys.path.insert(0,'.')
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__))+'/../pwp0-tools')
import gen_registry as G
import crates as C
from model7 import PLUG, ROOT, PLUGP
INST=PLUG+'🧵️instance/'
NEW='instance-dispatch'
def read(p): return open(p,encoding='utf-8').read()
def pkgdir(rel): return PLUG+rel+'/📦️packages/🦀️rust'
FAMILY_DEPS={ # crate key -> first-party family crates (tier 1 + extension + lower use-case crates)
 'instance-render':['instance-stores','instance-history','instance-children','instance-jobs','instance-docs','instance-extension'],
 'instance-dispatch':['instance-stores','instance-history','instance-children','instance-jobs','instance-docs','instance-extension','instance-render'],
 'instance-typed-ops':['instance-stores','instance-history','instance-children','instance-jobs','instance-docs','instance-extension','instance-render','instance-dispatch'],
 'time-travel-turns':['instance-stores','instance-history','instance-children','instance-jobs','instance-docs','instance-extension','instance-render','instance-dispatch'],
 'tool-run-turns':['instance-stores','instance-history','instance-children','instance-jobs','instance-docs','instance-extension','instance-render'],
 'instance-commands':['instance-stores','instance-history','instance-children','instance-jobs','instance-docs','instance-extension','instance-render','instance-dispatch','instance-typed-ops','time-travel-turns'],
 'instance-ladders':['instance-stores','instance-history','instance-children','instance-jobs','instance-docs','instance-extension','instance-render','instance-typed-ops','time-travel-turns','tool-run-turns'],
 'instance-app':['instance-stores','instance-history','instance-children','instance-jobs','instance-docs','instance-extension','instance-render','instance-dispatch','instance-typed-ops','time-travel-turns','tool-run-turns','instance-commands','instance-ladders'],
 'instance-extension':['instance-stores','instance-history','instance-children','instance-jobs','instance-docs'],
}
ROOTS={'instance-render':'🧵️instance/🖼️render','instance-dispatch':'🧵️instance/📨️dispatch','instance-typed-ops':'🧵️instance/🧾️typed-operations','time-travel-turns':'⏪️time-travel/🧵️instance','tool-run-turns':'⏯️tool-run/🧵️instance','instance-commands':'🧵️instance/🧷️command-routes','instance-ladders':'🪜️maintenance-ladder','instance-app':'🧵️instance','instance-extension':'🧵️instance/🧩️extensions','instance-stores':'🧵️instance/🏪️stores','instance-history':'🧵️instance/📜️history','instance-children':'🧵️instance/🪆️children','instance-jobs':'🧵️instance/👷️jobs','instance-docs':'🧵️instance/📄️docs'}
LOWER_EXTRA={'instance-render':['tool-run','time-travel','decl-app','decl-views'],'instance-dispatch':['tool-run','time-travel','decl-app','decl-views','child','document-media','publication-mounted','plugin-program'],'instance-typed-ops':['time-travel','decl-app','decl-views'],'time-travel-turns':['tool-run','time-travel','decl-app'],'tool-run-turns':['tool-run','decl-app'],'instance-commands':['tool-run','time-travel','decl-app'],'instance-ladders':['tool-run','time-travel','decl-app'],'instance-app':['tool-run','time-travel','decl-app']}
DESCS={'instance-render':'Instance use-cases of the plugin SDK: tool overlays, history view, interaction state and reserved job inputs over the render bundle',
 'instance-dispatch':'Instance use-cases of the plugin SDK: emit dispatch, reserved emit producers, host events and typed command admission over the dispatch bundle',
 'instance-typed-ops':'Instance use-cases of the plugin SDK: typed operation publication, retirement, private child group publication and tool dispatch over the typed bundle',
 'instance-commands':'Instance use-cases of the plugin SDK: framework-reserved routes, action dispatch and archive merge over the command bundle'}
def rel_dep(from_root, key, lower_path_map):
    pass
def dep_line(key, from_root):
    c=C.build()[0].get(key)
    if key in ROOTS and key not in ('time-travel-turns','tool-run-turns') or key in ('time-travel-turns','tool-run-turns'):
        tgt=PLUG+ROOTS[key]+'/📦️packages/🦀️rust'
    else:
        tgt=C.R+'/'+c['pkg']
    src=PLUG+from_root+'/📦️packages/🦀️rust'
    name='semio-framework-plugin-'+key
    return '%s = { path = "%s" }'%(name,os.path.relpath(tgt,src))
def lower_line(key, from_root):
    c=C.build()[0][key]
    src=PLUG+from_root+'/📦️packages/🦀️rust'
    return '%s = { path = "%s" }'%(c['name'],os.path.relpath(C.R+'/'+c['pkg'],src))
def update_manifest(key):
    root=ROOTS[key]
    p=pkgdir(root)+'/Cargo.toml'
    t=read(p)
    deps_start=t.index('[dependencies]\n')+len('[dependencies]\n')
    nxt=t.index('\n[',deps_start) if '\n[' in t[deps_start:] else len(t)
    block=t[deps_start:nxt]
    lines=[l for l in block.split('\n') if l.strip()]
    names={l.split(' = ')[0]:l for l in lines}
    for fk in FAMILY_DEPS.get(key,[]):
        nm='semio-framework-plugin-'+fk
        names[nm]=dep_line(fk,root)
    for lk in LOWER_EXTRA.get(key,[]):
        nm='semio-framework-plugin-'+lk
        if nm not in names: names[nm]=lower_line(lk,root)
    # remove family deps that are not in the new set (upward or sibling edges of the old file-aligned partition)
    fam_all={'semio-framework-plugin-'+k for k in ROOTS}
    keep=set('semio-framework-plugin-'+k for k in FAMILY_DEPS.get(key,[]))
    for nm in list(names):
        if nm in fam_all and nm not in keep: del names[nm]
    order=sorted(names)  # first-party plugin crates first, then others, each byte-sorted would reshuffle; keep the original relative order, new lines appended after the last plugin crate line
    out=[]
    seen=set()
    for l in lines:
        nm=l.split(' = ')[0]
        if nm in names: out.append(names[nm]); seen.add(nm)
    extra=[names[nm] for nm in names if nm not in seen]
    # insert extras after the last 'semio-framework-plugin-' line
    idx=max([i for i,l in enumerate(out) if l.startswith('semio-framework-plugin-')]+[-1])+1
    out[idx:idx]=extra
    t2=t[:deps_start]+'\n'.join(out)+'\n'+t[nxt:]
    if key in DESCS: t2=re.sub(r'(?m)^description = ".*"$','description = "%s"'%DESCS[key],t2,count=1)
    if key=='instance-render' and '[features]' not in t2:
        t2=t2.replace('[lib]\npath = "🦀️.rs"\n','[lib]\npath = "🦀️.rs"\n\n[features]\nartifact-app-testing = []\n')
    return p,t,t2

def create_dispatch(write):
    root=ROOTS[NEW]
    base=pkgdir(root)
    src=read(pkgdir(ROOTS['instance-typed-ops'])+'/Cargo.toml')
    t=src.replace('instance-typed-ops','instance-dispatch').replace('plugin-instance-typed-ops','plugin-instance-dispatch')
    t=re.sub(r'(?m)^description = ".*"$','description = "%s"'%DESCS[NEW],t,count=1)
    t=t.replace('[lib]\npath = "🦀️.rs"','[lib]\npath = "../../🦀️.rs"')
    # relative paths: the typed-operations and dispatch directories are siblings, so every relative dependency path is unchanged
    proj=json.dumps({"name":"@semio-tech/plugin-instance-dispatch-rs","tags":["scope:framework","language:rust"]},indent=2,ensure_ascii=False)+'\n'
    rootrs='//! %s️ Instance use-cases of the plugin SDK: emit dispatch, reserved emit producers, host events and typed command admission over the dispatch bundle.\n//!\n//! 🧭️ WP7 root of `semio-framework-plugin-instance-dispatch`: the sources are staged under `📥️staging` and mounted by the instance parent until the crate is flipped.\n\nextern crate semio_framework_os_kernel as protocol;\nextern crate semio_framework_os_kernel as store;\n'
    return base,t,proj,rootrs
if __name__=='__main__':
    mode=sys.argv[1]; write=(mode=='write')
    base,t,proj,rootrs=create_dispatch(write)
    print('dispatch manifest', len(t), 'project', len(proj))
    for k in ['instance-render','instance-typed-ops','instance-commands','time-travel-turns','tool-run-turns','instance-ladders','instance-app','instance-extension']:
        p,a,b=update_manifest(k)
        print(k,'changed' if a!=b else 'same')

def write_all():
    from asm7 import EMO
    base,t,proj,rootrs=create_dispatch(True)
    os.makedirs(base,exist_ok=True)
    cp=base+'/Cargo.toml'
    assert not os.path.exists(cp)
    open(cp,'w',encoding='utf-8').write(t)
    open(base+'/📋️project.json','w',encoding='utf-8').write(proj)
    rootp=PLUG+ROOTS[NEW]+'/🦀️.rs'
    assert not os.path.exists(rootp)
    open(rootp,'w',encoding='utf-8').write(rootrs%EMO[30])
    # manifests
    changed=[]
    for k in ['instance-dispatch','instance-render','instance-typed-ops','instance-commands','time-travel-turns','tool-run-turns','instance-ladders','instance-app','instance-extension']:
        p,a,b=update_manifest(k)
        if a!=b: open(p,'w',encoding='utf-8').write(b); changed.append(k)
    # registries
    c=dict(key=NEW,name='semio-framework-plugin-'+NEW,project='@semio-tech/plugin-%s-rs'%NEW,pkg=os.path.relpath(base,C.R))
    crates={NEW:c}
    out=[G.reg_members(crates,True),G.reg_owners(crates,True),G.reg_launch(crates,True,[NEW])]
    return changed,out
if __name__=='__main__' and sys.argv[1]=='apply':
    print(write_all())
