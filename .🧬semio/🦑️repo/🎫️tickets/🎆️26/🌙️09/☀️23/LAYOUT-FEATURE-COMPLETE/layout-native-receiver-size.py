import json
import re
import subprocess
import sys
from pathlib import Path

ticket = Path(__file__).resolve().parent
generated = ticket / "🗑️generated"
log = Path(sys.argv[1]).resolve()
objects = generated / "layout-native-size-deps"
objects.mkdir(parents=True, exist_ok=True)
crates = {}
for line in log.read_text().splitlines():
    try:
        event = json.loads(line)
    except json.JSONDecodeError:
        continue
    if event.get("reason") != "compiler-artifact":
        continue
    for name in event["filenames"]:
        current = Path(name)
        if not current.is_file() or current.suffix not in (".rmeta", ".rlib", ".dylib", ".so"):
            continue
        if "wasm32-wasip2" not in current.parts and current.suffix in (".rmeta", ".rlib"):
            continue
        target = objects / current.name
        target.unlink(missing_ok=True)
        target.symlink_to(current)
        if current.suffix == ".rmeta":
            crates[event["target"]["name"]] = target

layout = "semio_s_artifact_layout_layout_mutations"
value = "semio_framework_value"
derive_log = Path(sys.argv[2]).resolve() if len(sys.argv) > 2 else None
required = (value,) if derive_log else (layout, value)
missing = [key for key in required if key not in crates]
if missing:
    raise SystemExit(f"WASI compiler artifacts still required: {', '.join(missing)}")
source = generated / "layout-native-size.rs"
if derive_log:
    derive = "semio_framework_value_derive"
    macro = None
    for line in derive_log.read_text().splitlines():
        try:
            event = json.loads(line)
        except json.JSONDecodeError:
            continue
        if event.get("reason") != "compiler-artifact" or event["target"]["name"] != derive:
            continue
        for name in event["filenames"]:
            candidate = Path(name)
            if candidate.is_file() and candidate.suffix in (".dylib", ".so", ".dll"):
                macro = candidate
    if macro is None:
        raise SystemExit("The selected derive proc-macro output is still required")
    externs = ["--extern", f"{derive}={macro}"]
    for library in macro.parent.rglob("*.dylib"):
        target = objects / library.name
        target.unlink(missing_ok=True)
        target.symlink_to(library)
    sections = ["use semio_framework_value::{DslValue, NativeReceive};\n"]
    for kind, count in (("small", 2), ("wide", 58)):
        name = kind.capitalize()
        variants = "\n".join(f"    Value{index} {{ text: String }}," for index in range(count))
        sections.append(
            "#[derive(semio_framework_value::RetireOwned, semio_framework_value_derive::NativeReceive)]\n"
            f"enum {name} {{\n{variants}\n}}\n"
            f"#[unsafe(no_mangle)] pub fn {kind}_receiver_bytes() -> usize {{\n"
            f"    size_of::<<{name} as NativeReceive<(String, DslValue)>>::Receiver>()\n"
            "}\n"
        )
    source.write_text("\n".join(sections))
else:
    externs = ["--extern", f"{layout}={crates[layout]}"]
    source.write_text(
        "use semio_framework_value::{DslValue, NativeReceive};\n"
        "use semio_s_artifact_layout_layout_mutations::LayoutMutation;\n"
        "#[unsafe(no_mangle)] pub fn layout_native_receiver_bytes() -> usize {\n"
        "    size_of::<<LayoutMutation as NativeReceive<(String, DslValue)>>::Receiver>()\n"
        "}\n"
    )
ir = generated / "layout-native-size.ll"
subprocess.run(
    ["repo", "tools", "exec", "rustup", "--", "run", "nightly-2026-07-20", "rustc",
     "--edition=2024", "--crate-name=layout_native_size", "--crate-type=lib",
     "--target=wasm32-wasip2", "--emit=llvm-ir", "-Copt-level=1",
     "-L", f"dependency={objects}", *externs,
     "--extern", f"{value}={crates[value]}", str(source), "-o", str(ir)],
    check=True
)
sizes = dict(re.findall(
    r"^define[^\n]* @(layout_native|small|wide)_receiver_bytes\(\)[^\n]*\{\nstart:\n  ret i32 (\d+)\n\}",
    ir.read_text(), re.MULTILINE,
))
if derive_log:
    assert sizes["small"] == sizes["wide"], sizes
    print(f"Two- and 58-variant WASI receivers each occupy {sizes['small']} bytes")
else:
    print(f"LayoutMutation WASI native receiver bytes {sizes['layout_native']}")
