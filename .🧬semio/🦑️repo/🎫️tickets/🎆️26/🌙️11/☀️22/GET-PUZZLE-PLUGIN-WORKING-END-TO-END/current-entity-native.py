from pathlib import Path
base = Path(__file__).resolve().with_name('current-5d-relink.py')
s=base.read_text().replace('current-5d-relink','current-2d-hist-relink')
a=s.index('name = '); b=s.index('toolchain =',a)
s=s[:a]+'''name = 'semio_plugin_tool_run'
registry_probe = False
fingerprint = next((cache/'semio-plugin-tool-run').glob('*/fingerprint/test-lib-semio_plugin_tool_run.json'))
meta = json.loads(fingerprint.read_text())
manifest = next(root.glob('🧰*/🛍*/💻*/🔨*/🔌*/⏯*/📦*/🦀*/Cargo.toml')).parent
'''+s[b:]
a=s.index("source = ticket/'ui-window");b=s.index('args = ',a)
s=s[:a]+'''source = manifest/'../../🦀️.rs'
executable=scratch/'tool-run-entity-test'
'''+s[b:]
exec(compile(s,str(base),'exec'))
