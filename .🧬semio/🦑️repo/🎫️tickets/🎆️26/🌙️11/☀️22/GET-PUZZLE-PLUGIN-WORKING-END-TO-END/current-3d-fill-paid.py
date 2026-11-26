from pathlib import Path
base = Path(__file__).resolve().with_name('current-5d-relink.py')
code = base.read_text().replace('current-5d-relink', 'current-2d-hist-relink')
a = code.index("name = ")
b = code.index("toolchain =", a)
code = code[:a]+'''name = "editor_surface"
registry_probe = False
fingerprint = next((cache/"semio-s-artifact-puzzle-3d-editor").glob("*/fingerprint/test-integration-test-editor-surface.json"))
meta = json.loads(fingerprint.read_text())
manifest = next(root.glob("✏️s/🔌*/🧩*/🗿*/🧊*/🏅*/🔖*/🪆*/✳*/✏*/📦*/🦀*/Cargo.toml")).parent
'''+ code[b:]
a = code.index("source = ticket/'ui-window")
b = code.index('args = ',a)
code = code[:a]+'''probe_name = os.environ.get("PUZZLE_3D_PROBE", "current-fill-host-probe.rs")
assert Path(probe_name).name == probe_name
source = gen / ("native-" + probe_name)
context = (manifest / "../../🧪️tests/🧰️context/🦀️.rs").resolve()
probe = (ticket/probe_name).read_text()
a = probe.index("mod context {")
b = probe.index("use context", a)
source.write_text(probe[:a] + f'#[path="{context}"] mod context;\\n' + probe[b:])
executable = scratch/"original-fill-host"
suite = os.environ.get("PUZZLE_3D_TEST_SUITE") or ("🔬️panels" if os.environ.get("PUZZLE_3D_PANELS") else None)
if suite:
    assert Path(suite).name == suite
    source = (manifest / "../../🧪️tests" / suite / "🦀️.rs").resolve()
    executable = scratch / ("registered-" + suite)
if os.environ.get("PUZZLE_OUTPUT"):
    suffix=os.environ["PUZZLE_OUTPUT"];assert Path(suffix).name == suffix
    executable=scratch/suffix
'''+code[b:]
code = code.replace("'-C','debuginfo=0'", "'-Z','threads=1','-C','debuginfo=0'")
exec(compile(code, str(base), 'exec'))
