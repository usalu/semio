"""👁️ Replay the independent viewer Cargo test targets using the linked Cargo graph."""
from pathlib import Path
from bim_link import large_link
import json, os, subprocess, sys

ticket=Path(__file__).resolve().parent
generated=ticket/'🗑️generated'
capture=generated/'compiler-retry'
test=json.loads((capture/'n-viewer-8548.invocation.json').read_text())
library=json.loads((capture/'n-viewer-7715.success.json').read_text())
compiler=test['compiler']
pkg=Path(test['cargo']['CARGO_MANIFEST_DIR'])
base=test['arguments']
env=dict(os.environ,**test['cargo'])
env.setdefault('RUST_MIN_STACK','67108864')
if sys.platform=='darwin':
    env['SDKROOT']=subprocess.check_output(['xcrun','--sdk','macosx26.5','--show-sdk-path'],text=True).strip()
out=generated/'viewer-sized'
out.mkdir(exist_ok=True)
sources={'ifc_energy':pkg/'../../../🚪️io/📤️export/🏗️ifc/🔥️energy/🧪️tests/🔬️unit/🦀️.rs','viewer':pkg/'../../🧪️tests/🔬️unit/🦀️.rs','viewer_lib':pkg/'🦀️.rs'}
link=[]
if sys.platform=='darwin':
    root=Path(subprocess.check_output([compiler,'--print','sysroot'],text=True).strip())
    linker=next(root.glob('lib/rustlib/*/bin/gcc-ld/ld64.lld'))
    link=['-Clink-arg=-Wl,-no_deduplicate,-x,-S']
for name in sys.argv[1:] or sources:
    diagnose=name=='viewer_restore'
    source_name='viewer' if diagnose else name
    payload=None
    target=out/name
    target.mkdir(exist_ok=True)
    args=list(base); args[args.index('--crate-name')+1]=name;args[3]=str(sources[source_name].resolve());args[args.index('--out-dir')+1]=str(target)
    args=[a.replace('extra-filename=-27a926896e5660d1','extra-filename=') for a in args]
    if source_name=='viewer':
        v=library['arguments'];directory=Path(v[v.index('--out-dir')+1]);archive=next(directory.glob('libsemio_s_artifact_bim_model_viewer-*.rlib'))
        args+=['-L','dependency='+str(directory)]
        for ext in ('.rlib','.rmeta'):args+=['--extern','semio_s_artifact_bim_model_viewer='+str(archive.with_suffix(ext))]
    if diagnose:
        import re
        plugin=next(a.partition('=')[2] for a in args if a.startswith('semio_plugin=') and a.endswith('.rmeta'))
        meta=subprocess.check_output([compiler,'-Zls=root',plugin],text=True)
        package=re.search(r'^\d+ (semio_plugin_window_config-[a-f0-9]+) hash',meta,re.M)[1]
        directory=next(Path(a.partition('=')[2]) for a in args if a.startswith('dependency=') and (Path(a.partition('=')[2])/('lib'+package+'.rlib')).exists())
        for ext in ('.rlib','.rmeta'): args+=['--extern','semio_plugin_window_config='+str(directory/('lib'+package+ext))]
        helper=next(Path('🧰️framework').glob('**/🧵️wp7-instance-tier2/🦀️.rs')).read_text().split('pub async fn restore_fixture_window_config',1)[1].split('//#endregion',1)[0]
        helper=helper.replace('crate::PluginApp','semio_plugin::PluginApp').replace('semio_plugin_testing_laws::','semio_plugin::')
        helper=helper.replace('for _ in','for turn in').replace('        fixture_renew_turn','        if turn < 7 || turn % 2048 == 0 { eprintln!("[DEBUG] turn={turn} admitted={} closing={} done={} phase={:?} failure={:?}",load.admitted,load.closing,load.done,load.load.as_ref().map(|l|l.phase()),load.failure.as_ref()); }\n        fixture_renew_turn').replace('let mut load = Default::default();','let mut load = semio_plugin_window_config::WindowConfigCommandLoad::default();').replace('turn_fuel(), grant)?;', 'turn_fuel(), grant).inspect_err(|e| eprintln!("[DEBUG] renew {e:?}"))?;').replace('entry, cx)?','entry, cx).inspect_err(|e| eprintln!("[DEBUG] advance {e:?}"))?')
        payload=(sources[source_name].read_text().replace('semio_plugin::restore_fixture_window_config','diagnose_restore')+'\npub async fn diagnose_restore'+helper).encode()
        args[3]='-'
    if not (target/(name+'.rlink')).exists():
        with (target/'codegen.log').open('wb') as log:
            subprocess.run([compiler,*args,'-Zno-link','-Copt-level=s','-Ccodegen-units=64'],env=env,cwd=test['cwd'],stdout=log,stderr=log,check=True,input=payload)
    rlink=next(target.glob('*.rlink'))
    with large_link(), (target/'link.log').open('wb') as log:
        subprocess.run([compiler,'--crate-name',name,'--test','-Zlink-only',str(rlink),*link],env=env,cwd=test['cwd'],stdout=log,stderr=log,check=True)
    with (target/'test.log').open('wb') as log:
        subprocess.run([str(target/name),'--test-threads=1',*(['window_commands_persist','--nocapture'] if diagnose else [])],env=env,cwd=pkg,stdout=log,stderr=log,check=True)
    print(name,(target/'test.log').read_text().strip().splitlines()[-1],flush=True)
