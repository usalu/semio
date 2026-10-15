import os,subprocess,sys,re,shutil
from plan import *
from deps import analyse
D='/Users/ueli/Documents/semio-kinan-wip/target/debug/deps'
SP='/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/pets'
SERDE=D+'/libserde-2119fe5726a5eff0.rmeta'
JSON=os.environ.get('PETS_SERDE_JSON',D+'/libserde_json-7081bf7b8f264137.rmeta')
ORDER=['contract','kinematics','clearance','traversal','interaction','character','staging']
TC='+nightly-2026-07-07'

def lib_root(c):
    return src('clearance') if CRATES[c].get('root')=='module' else node_dir(c)+'/🦀️.rs'

def compile(c,out,mode,test,srcd,tstd,sj):
    deps=set(srcd[c])
    args=['rustc',TC,'--edition','2021','--crate-type','lib','--crate-name',ident(c),'--emit=metadata','-C','extra-filename=-pets','-C','metadata=pets-'+c+('-t' if test else ''),'--out-dir',out+('/t' if test else ''),'-L','dependency='+D,'-L','dependency='+out,'--extern','serde='+SERDE]
    if c=='contract' and mode=='test': args+=['--cfg','feature="testkit"','--extern','serde_json='+JSON]
    if test:
        deps|=tstd[c]
        if sj[c]: args+=['--extern','serde_json='+JSON]
        args.append('--test')
    for d in sorted(deps):
        args+=['--extern',f'{ident(d)}={out}/lib{ident(d)}-pets.rmeta']
    args.append(lib_root(c))
    os.makedirs(out+('/t' if test else ''),exist_ok=True)
    p=subprocess.run(args,capture_output=True,text=True,cwd=pkg_dir(c),env={**os.environ,'CARGO_MANIFEST_DIR':pkg_dir(c)})
    errs=[l for l in p.stderr.split('\n') if l.startswith('error')]
    warns=[l for l in p.stderr.split('\n') if l.startswith('warning')]
    open(f"{out}/{c}{'-test' if test else ''}.err",'w').write(p.stderr)
    print(mode,c,'test' if test else 'lib','errors',len(errs),'warnings',len(warns),flush=True)
    if errs: print('\n'.join(p.stderr.split('\n')[:int(os.environ.get('LINES','80'))]))
    return len(errs)

def run(mode):
    out=SP+'/out-'+mode
    shutil.rmtree(out,ignore_errors=True); os.makedirs(out)
    srcd,tstd,sj,kit=analyse()
    total=0
    for c in ORDER:
        total+=compile(c,out,mode,False,srcd,tstd,sj)
        if mode=='test': total+=compile(c,out,mode,True,srcd,tstd,sj)
    args=['rustc',TC,'--edition','2021','--crate-type','lib','--crate-name','pets','--emit=metadata','-C','extra-filename=-pets','-C','metadata=pets-facade','--out-dir',out+'/f','-L','dependency='+D,'-L','dependency='+out,'--extern','serde_json='+JSON,'--extern','serde='+SERDE]
    for c in ORDER: args+=['--extern',f'{ident(c)}={out}/lib{ident(c)}-pets.rmeta']
    args.append(PETS+'/🦀️.rs')
    os.makedirs(out+'/f',exist_ok=True)
    p=subprocess.run(args,capture_output=True,text=True,cwd=PETS+'/📦️packages/🦀️rust')
    errs=[l for l in p.stderr.split('\n') if l.startswith('error')]
    open(out+'/facade.err','w').write(p.stderr)
    print(mode,'facade lib errors',len(errs),'warnings',len([l for l in p.stderr.split('\n') if l.startswith('warning')]))
    if errs: print('\n'.join(p.stderr.split('\n')[:60]))
    total+=len(errs)
    print('TOTAL errors',total)

if __name__=='__main__':
    run(sys.argv[1] if len(sys.argv)>1 else 'native')
