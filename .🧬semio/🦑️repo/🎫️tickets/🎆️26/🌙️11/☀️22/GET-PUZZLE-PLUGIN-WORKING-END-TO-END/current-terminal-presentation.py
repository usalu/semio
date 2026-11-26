from pathlib import Path
base=Path(__file__).resolve().with_name('current-entity-native.py')
wrapper=base.read_text()
marker="exec(compile(s,str(base),'exec'))"
insert="""s=s.replace("executable=scratch/'tool-run-entity-test'", "executable=scratch/'terminal-presentation'")
point=s.index('dependencies = meta')
s=s[:point]+'''
plugin_meta=meta
name='semio_framework_tool_run_presentation'
manifest=next(root.glob('🧰*/🔨*/⏯*/📊*/🪟*/📦*/🦀*/Cargo.toml')).parent
key=next(dep[3] for dep in plugin_meta['deps'] if dep[1]==name).to_bytes(8,'little').hex()
path=artifacts[key].resolve()
meta=json.loads((path.parent.parent/'fingerprint'/f'lib-{name}.json').read_text())
dev={'serde_json','semio_framework_trace','semio_framework_value_testing_effect_bank','semio_framework_value_testing_allocator'}
rows={row[1]:row for row in plugin_meta['deps'] if row[1] in dev}
for alias in dev-rows.keys():
    matching=[(link.resolve().stat().st_mtime_ns,key) for key,link in artifacts.items() if link.name.split('-')[0]=='lib'+alias]
    key=max(matching)[1]
    rows[alias]=[0,alias,False,int.from_bytes(bytes.fromhex(key),'little')]
meta['deps']+=list(rows.values())
args[1]=str(manifest/'../../🦀️.rs')
args[args.index('--crate-name')+1]=name
'''+s[point:]
exec(compile(s,str(base),'exec'))"""
exec(compile(wrapper.replace(marker,insert),str(base),'exec'))
