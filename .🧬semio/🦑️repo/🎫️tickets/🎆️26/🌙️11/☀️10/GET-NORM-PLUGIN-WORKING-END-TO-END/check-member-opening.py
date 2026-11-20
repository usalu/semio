"""🧩️ Relink the production replacement ladder and mounted DIN climate-child probe."""
from pathlib import Path
import os, subprocess, sys

ticket = Path(__file__).resolve().parent
root = ticket.parents[6]
out = ticket / '🗑️generated'
temp = out / 'member-opening'
temp.mkdir(exist_ok=True)
rustc = os.environ.get('RUSTC','rustc')
def compile(name, args):
    flags = temp / (name + '.args')
    flags.write_text('\n'.join(args) + '\n')
    subprocess.run([rustc,'@'+str(flags)],check=True)
def amended_lib(name, replacements):
    args = (out / (name+'-live.args')).read_text().splitlines()
    i=args.index('--emit=metadata')
    del args[i:i+3]
    for i,arg in enumerate(args):
        for alias,lib in replacements.items():
            if arg.startswith(alias+'='):args[i]=alias+'='+str(lib.with_suffix('.rmeta'))
    for arg in args[:]:
        if '=/' in arg and arg.endswith('.rmeta'):
            alias,path=arg.split('=',1)
            link=Path(path).with_suffix('.rlib')
            if link.is_file(): args.extend(('--extern',alias+'='+str(link)))
    library=temp/('lib'+name+'.rlib')
    args+=['--emit=link='+str(library)+',metadata='+str(library.with_suffix('.rmeta')),'-Cpanic=abort','-Copt-level=0']
    compile(name,args)
    return library
ladders=amended_lib('semio_plugin_instance_ladders',{})
instance=amended_lib('semio_plugin_instance_app',{'semio_plugin_instance_ladders':ladders})
family=sys.argv[1]
src=out / (family+'_archive_context.rs')
dest=out / (family+'_member_probe.rs')
text=src.read_text().replace('PluginApp, VcsArtifactApp};','PluginApp};\n    use updated_instance::VcsArtifactApp;')
anchor="                let mut mounted = MountedOwnerTurnV1::admit(1, identity, phase, policy, cx)?.unwrap();"
assert text.count(anchor)==1
dest.write_text(text.replace(anchor,'''                if !isolated {
                    use updated_mounted::{MountedOwnerIdentityV1 as Identity, MountedOwnerPhaseV1 as Phase, MountedOwnerPolicyV1 as Policy, MountedOwnerTurnV1 as Turn};
                    assert!(<EditorApp<PlayApp> as semio_plugin::ArtifactApp>::mounted_jobs_terminal_is_empty(1));
                    assert_eq!(phase, MountedOwnerPhaseV1::Maintenance);
                    let identity = Identity { instance_id: 1, operation: cx.operation(), generation: cx.generation() };
                    let mut turn = Turn::admit(1, identity, Phase::Maintenance, Policy::INTERACTIVE_COMPONENT, cx)?.unwrap();
                    turn.advance_lifecycle(|same, grant| self.app.maintenance_step(same, grant))?;
                    return Ok(());
                }
'''+anchor))
args=(out/(family+'-archive-context.args')).read_text().replace(str(src),str(dest)).splitlines()
component=temp/(family+'.wasm')
args[args.index('-o')+1]=str(component)
args.append('-Ldependency='+str(temp))
for suffix in ('.rmeta','.rlib'):
    args.extend(('--extern','updated_instance='+str(instance.with_suffix(suffix))))
    args.extend(('--extern','updated_mounted='+str(out/'mounted-return'/('libsemio_plugin_mounted_owner'+suffix))))
compile(family,args)
subprocess.run([root/'node_modules/.bin/jco','run','--jco-dir',temp/'js',component,'wire','replacement'],check=True)
