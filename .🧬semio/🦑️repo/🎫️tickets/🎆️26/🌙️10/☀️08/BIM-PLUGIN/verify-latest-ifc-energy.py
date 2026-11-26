"""🌡️ Build the current IFC testkit and run the physical thermal export checks against fresh fixtures."""
from bim_link import large_link
import json,os,pathlib,subprocess,sys,re
t=pathlib.Path(__file__).resolve().parent
g=t/'🗑️generated'
c=g/'compiler-retry'
out=g/'ifc-current-testkit';out.mkdir(exist_ok=True)
dependencies=g/'ifc-closed-shells-n'
def build(capture,name,source=None):
    r=json.loads((c/capture).read_text())
    args=r['arguments'][:];is_test='--test' in args
    args[args.index('--out-dir')+1]=str(out)
    if source:args[3]=str(source)
    for i,value in enumerate(args):
        if value.startswith('extra-filename='):args[i]='extra-filename=-current' if not is_test else 'extra-filename='
        if value.startswith('metadata='):args[i]='metadata=currentifctests'
        if value.startswith('semio_s_artifact_bim_model_') and '=' in value:
            dep,old=value.split('=',1);new=(out/('lib'+dep+'-current'+pathlib.Path(old).suffix)) if dep in ('semio_s_artifact_bim_model_ifc','semio_s_artifact_bim_model_ifc_2') else (dependencies/('lib'+dep+'-ifcterminals'+pathlib.Path(old).suffix))
            if new.is_file():args[i]=dep+'='+str(new)
    args+=['-L','dependency='+str(dependencies),'-L','dependency='+str(out),'-Zno-parallel-backend','-Zthreads=1','-Ccodegen-units=64']
    if name=='semio_s_artifact_bim_model_ifc_2':
        original=next(a.split('=',1)[1] for a in r['arguments'] if a.startswith(name+'=') and a.endswith('.rmeta'))
        ids=dict(re.findall(r'^\d+ (\w+)-(\w+) hash ',subprocess.check_output([r['compiler'],'-Zls=root',original],text=True),re.M))
        key='semio_s_artifact_bim_model_storey_levels'
        search=[pathlib.Path(a[11:]) for a in r['arguments'] if a.startswith('dependency=')]
        for ext in ['.rmeta','.rlib']:
            path=next(d/('lib'+key+'-'+ids[key]+ext) for d in search if (d/('lib'+key+'-'+ids[key]+ext)).is_file())
            args+=['--extern',key+'='+str(path)]
    if source:args[args.index('--crate-name')+1]=name
    if is_test:args+=['-Zno-link','-Copt-level=s']
    env=dict(os.environ,**r['cargo'])
    if source and not is_test:env.update(CARGO_MANIFEST_DIR=str(source.parent),CARGO_PKG_NAME=name.replace('_','-'))
    with (out/(name+'.build.log')).open('wb') as log:subprocess.run([r['compiler'],*args],cwd=r['cwd'],env=env,stdout=log,stderr=log,check=True)
    return r,env
if not (out/'libsemio_s_artifact_bim_model_ifc-current.rlib').is_file():build('n-ifc-89711.success.json','ifc')
name=sys.argv[1] if len(sys.argv)>1 else 'ifc_energy'
folder=next(pathlib.Path.cwd().glob('✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any'))
if name in ('ifc','ifc4') and not (out/'libsemio_s_artifact_bim_model_ifc_2-current.rlib').is_file():
    build('n-viewer-7715.success.json','semio_s_artifact_bim_model_ifc_2',folder/'🚪️io/📥️import/🏗️ifc/📦️packages/🦀️rust/🦀️.rs')
source=folder/'🚪️io/📤️export/🏗️ifc/🔥️energy/🧪️tests/🔬️unit/🦀️.rs' if name=='ifc_energy' else folder/('👁️viewer/📦️packages/🦀️rust/🧪️'+name+'.rs')
if (out/(name+'.rlink')).exists():
    r=json.loads((c/'n-viewer-8548.invocation.json').read_text());env=dict(os.environ,**r['cargo'])
else:r,env=build('n-viewer-8548.invocation.json',name,source)
rust=r['compiler']
root=pathlib.Path(subprocess.check_output([rust,'--print','sysroot'],text=True).strip())
link=[]
if sys.platform == 'darwin' and name=='ifc_energy':
    linker=next(root.glob('lib/rustlib/*/bin/gcc-ld/ld64.lld'))
    link=['-Clink-arg=-fuse-ld='+str(linker),'-Clink-arg=-Wl,--threads=1,--read-workers=0']
with large_link(), (out/(name+'.link.log')).open('wb') as log:subprocess.run([rust,'--crate-name',name,'--test','-Zlink-only',str(out/(name+'.rlink')),*link],env=env,cwd=r['cwd'],stdout=log,stderr=log,check=True)
with (out/(name+'.test.log')).open('wb') as log:subprocess.run([str(out/(name+('.exe' if os.name=='nt' else ''))),'--test-threads=2'],env=env,cwd=r['cargo']['CARGO_MANIFEST_DIR'],stdout=log,stderr=log,check=True)
