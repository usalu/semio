from pathlib import Path

ticket = Path(__file__).resolve().parent
base = ticket / "ui-history-frozen.py"
shared = base.read_text().split("test_deps = dependencies(test_fingerprint)", 1)[0]
shared = shared.replace("mounted = '--refresh-mounted' in sys.argv", "mounted = True")
shared = shared.replace("variant = 'mounted' if mounted else 'frozen'", "variant = 'paged-list'")
shared = shared.replace("output = ticket / f'🗑️generated/ui-history-{variant}'", "output = ticket / '🗑️generated/law-projection'")
exec(compile(shared, str(base), "exec"))

def one(name):
    return next(key for key, (crate, _) in artifacts.items() if crate == name)

model = "semio_framework_dsl_record_model"
model_key = one(model)
refresh_names = {model, "semio_framework_value_paged_text"}
dsl_binding = root / "🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🪆️binding"
test_source = dsl_binding / "📋️persistent/🧪️tests/🦀️.rs"
source = output / "probe.rs"
source.write_text(f'use semio_framework_dsl_record_model::*;\n#[path = "{test_source}"]\nmod laws;\n')
keys = {model: model_key,
        "semio_framework_value_persistent": dependencies(fingerprints[model_key])["semio_framework_value_persistent"],
        "semio_framework_test_fixtures": one("semio_framework_test_fixtures"),
        "serde_json": one("serde_json")}
executable = compile_source("paged_list_law", dsl_binding, source, fingerprints[model_key], {model: refresh(model_key)}, test=True, selected_deps=keys)
raise SystemExit(subprocess.run([str(executable), "--nocapture"], check=False).returncode)
