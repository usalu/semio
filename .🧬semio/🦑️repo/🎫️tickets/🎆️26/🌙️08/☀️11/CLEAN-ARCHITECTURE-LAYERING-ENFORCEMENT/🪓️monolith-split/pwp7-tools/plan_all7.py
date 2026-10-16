import sys; sys.path.insert(0,'.')
from asm7 import *
import hashlib
REHOMED=['render','dispatch','typed-ops','commands']
OLD_STAGING_DELETE=[ '🧵️instance/🧷️command-routes/📥️staging/🎟️lease/🦀️.rs','🧵️instance/🧷️command-routes/📥️staging/📨️dispatch/🦀️.rs','🧵️instance/🧷️command-routes/📥️staging/🚪️ingress/🦀️.rs','🧵️instance/🧷️command-routes/📥️staging/🤝️group/🦀️.rs','🧵️instance/🧾️typed-operations/📥️staging/🛠️tool-machine/🦀️.rs']
OLD_ROOTS=['🧵️instance/🧷️command-routes/🦀️.rs','🧵️instance/🧾️typed-operations/🦀️.rs','🧵️instance/🖼️render/🦀️.rs']
INPLACE={'ladders':['🪜️maintenance-ladder/🦀️.rs','🪜️maintenance-ladder/📥️staging/🏪️lifecycle-stores/🦀️.rs','🪜️maintenance-ladder/📥️staging/🔁️document-replacement/🦀️.rs','🪜️maintenance-ladder/📥️staging/🔄️full-operation/🦀️.rs','🪜️maintenance-ladder/📥️staging/🔒️close-ladder/🦀️.rs','🪜️maintenance-ladder/📥️staging/🗝️envelope-ladder/🦀️.rs','🪜️maintenance-ladder/📥️staging/🚪️ingress/🦀️.rs'],
 'tt-turns':['⏪️time-travel/🧵️instance/🦀️.rs'],'tr-turns':['⏯️tool-run/🧵️instance/🦀️.rs'],
 'app':['🧵️instance/🔌️plugin-app-impl/🦀️.rs','🧵️instance/📥️staging/📨️dispatch/🦀️.rs','🧵️instance/📥️staging/🔒️close-ladder/🦀️.rs','🧵️instance/📥️staging/🚪️ingress/🦀️.rs','🧵️instance/📥️staging/🪜️maintenance-ladder/🦀️.rs']}
LADDER_EXTRA={'child_root_retirement_step':'🪜️maintenance-ladder/📥️staging/🚪️ingress/🦀️.rs','retained_fields_terminal_is_empty':'🪜️maintenance-ladder/📥️staging/🔒️close-ladder/🦀️.rs'}
def plan(W):
    writes={}; deletes=[]; mounts=[]; reexports=collections.OrderedDict()
    emo=lambda: next_emoji()
    # domain files
    for crate in REHOMED:
        groups=collections.OrderedDict()
        for m in W.ms:
            if m['crate']!=crate: continue
            fine=DOMAIN_OF[m['name']]
            sdir=STAGE[crate][fine]
            groups.setdefault(sdir,[]).append(m)
        for sdir,members in groups.items():
            path=ROOT_OF[crate]+'/📥️staging/'+sdir+'/🦀️.rs'
            members.sort(key=lambda m:(W.ms.index(m)))
            writes[path]=domain_file(crate,sdir,members,next_emoji(path),W.T)
            slug=sdir[len(sdir.split('️')[0])+1:].replace('-','_')
            mounts.append((SNAKE[crate]+'_'+slug,path,None))
    # bundles
    for crate in ['render','dispatch','typed-ops','tt-turns','tr-turns','commands','ladders']:
        base=ROOT_OF[crate]+'/📥️staging/🧺️bundle/🦀️.rs' if crate not in ('tt-turns','tr-turns','ladders') else None
        if crate=='tt-turns': base='⏪️time-travel/🧵️instance/📥️staging/🧺️bundle/🦀️.rs'
        if crate=='tr-turns': base='⏯️tool-run/🧵️instance/📥️staging/🧺️bundle/🦀️.rs'
        if crate=='ladders': base='🪜️maintenance-ladder/📥️staging/🧺️bundle/🦀️.rs'
        writes[base]=bundle_file(W,crate,next_emoji(base))
        mounts.append((SNAKE[crate]+'_bundle',base,None))
        reexports.setdefault(SNAKE[crate]+'_bundle',[]).append(CTX[crate])
        if crate in VIEW: reexports[SNAKE[crate]+'_bundle'].append(VIEW[crate])
    # builders
    bpath='🧵️instance/📥️staging/🏗️ctx-builders/🦀️.rs'
    writes[bpath]=builders_text(W,next_emoji(bpath))
    mounts.append(('app_ctx_builders',bpath,None))
    # in-place
    extra=collections.defaultdict(list)
    for m in W.ms:
        if m['crate']=='ladders' and m['name'] in LADDER_EXTRA: extra[LADDER_EXTRA[m['name']]].append(m)
    for crate,files in INPLACE.items():
        for rel in files:
            writes[rel]=inplace(W,rel,crate,extra.get(rel,()))
    # old files
    for rel in OLD_STAGING_DELETE: deletes.append(rel)
    for rel in OLD_ROOTS:
        f=load_pre(rel)
        docs=[l for l in f['header'] if l.startswith('//!')]
        writes[rel]='\n'.join(docs)+'\n'
    return writes,deletes,mounts,reexports
if __name__=='__main__':
    W=build(); field_sets(W)
    writes,deletes,mounts,rex=plan(W)
    outd=GEN+'/pwp7-out'
    import shutil
    if os.path.exists(outd): shutil.rmtree(outd)
    for rel,txt in writes.items():
        p=outd+'/'+rel
        os.makedirs(os.path.dirname(p),exist_ok=True)
        open(p,'w',encoding='utf-8').write(txt)
    print(len(writes),'writes',len(deletes),'deletes')
    for rel,txt in sorted(writes.items()): print('%6d %s'%(txt.count('\n'),rel))
