import os
from pathlib import Path
import sys

original = Path(__file__).with_name("ui-transport-native.py").resolve()
target = sys.argv[1] if len(sys.argv) > 1 else "lib-semio_os_store_engine"
sys.argv[1:] = ["--preview", "--preview-api"]
source = original.read_text()
source = source.replace('generated / "ui-preview-api"', 'generated / "ui-actor-metadata"')
start = source.index("changed_packages =")
end = source.index("historical =", start)
source = source[:start] + '''changed_packages = {
    "semio-os-store-backbone", "semio-plugin-time-travel", "semio-plugin-reactor-app",
    "semio-plugin-instance-app", "semio-plugin-reactor-runtime",
}
''' + source[end:]
source = source.replace('target = "lib-semio_os_store_engine" if api_only else "lib-semio_os_store_engine_derived"', f"target = {target!r}")
source = source.replace('path.name if path else', 'path.with_suffix(".rmeta").name if path else')
source = source.replace('"--crate-type", "rlib", "-C"', '"--crate-type", "rlib", "--emit=metadata", "-C"')
source = source.replace('if dependency.with_suffix(".rmeta").is_file():', 'if dependency.suffix == ".rlib" and dependency.with_suffix(".rmeta").is_file():')
if os.environ.get("PUZZLE_WARM_PLUGIN_METADATA"):
    source = source.replace('compiled = {}', 'compiled = {}\nrefreshed = set()')
    marker = '    crate = fingerprint.name.removeprefix("test-")'
    source = source.replace(marker, '''    if path:
        warm = scratch / path.with_suffix(".rmeta").name
        if warm.is_file() and package not in {"semio-plugin-instance-typed-ops", "semio-plugin-instance-app"} and not refreshed.intersection(externs.values()):
            return warm
        refreshed.add(warm)
''' + marker)
exec(compile(source, str(original), "exec"), {"__file__": str(original)})
