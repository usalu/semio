from pathlib import Path
import json
import re
import subprocess

ticket = Path(__file__).resolve().parent
generated = ticket / "🗑️generated"
inputs = generated / "mesh-inputs-9f403ac70c06ea23"
previous = generated / "authored-intersection-native"
output = generated / "longitudinal-slab-authored"
output.mkdir(exist_ok=True)
mesh = Path("🧰️framework/🔨️modules/🧊️3d/🥽️mesh")
source = mesh / "🧪️tests/🔬️modeling/🦀️.rs"

def artifact(name):
    candidates = list(inputs.glob(f"lib{name}-*.rmeta"))
    if len(candidates) != 1:
        raise RuntimeError(f"expected exactly one retained compiler artifact for {name}")
    return candidates[0]

def compile(name, path, dependencies, mode, target):
    print(f"[DEBUG] compiling {name}", flush=True)
    command = ["repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20",
               "rustc", "--edition=2024", "--crate-name", name, mode, str(path), "-o", str(target)]
    for directory in (inputs, previous, output):
        command += ["-L", f"dependency={directory}"]
    for dependency, path in dependencies.items():
        command += ["--extern", f"{dependency}={path}"]
    subprocess.run(command, check=True)

value = {"semio_framework_value": artifact("semio_framework_value")}
clipping_deps = {
    **value,
    **{name: previous / f"lib{name}.rlib" for name in
       ("semio_framework_3d_mesh_contract", "semio_framework_mesh_engine")},
}
clipping = output / "libsemio_framework_3d_mesh_clipping.rlib"
compile("semio_framework_3d_mesh_clipping", mesh / "🛠️modeling/✂️clipping/🦀️.rs",
        clipping_deps, "--crate-type=rlib", clipping)
native = output / "native-longitudinal-clipping"
compile("longitudinal_clipping", mesh / "🛠️modeling/✂️clipping/🧪️tests/🧱️longitudinal.rs",
        {**clipping_deps, "semio_framework_3d_mesh_clipping": clipping,
         "serde_json": artifact("serde_json")}, "--test", native)
subprocess.run([str(native)], check=True)

document = output / "libsemio_framework_3d_mesh_document.rlib"
derive = [path for path in inputs.glob("*semio_framework_value_derive-*")
          if path.suffix in (".dylib", ".so", ".dll")]
if len(derive) != 1:
    raise RuntimeError("expected exactly one retained value derive library")
compile("semio_framework_3d_mesh_document", mesh / "📄️document/🦀️.rs",
        {**clipping_deps, "semio_framework_pack_json": artifact("semio_framework_pack_json"),
         "semio_framework_value_derive": derive[0]}, "--crate-type=rlib", document)
reconstruction = output / "libsemio_framework_3d_mesh_reconstruction.rlib"
compile("semio_framework_3d_mesh_reconstruction", mesh / "🛠️modeling/🔁️reconstruction/🦀️.rs",
        {**clipping_deps, "semio_framework_3d_mesh_document": document}, "--crate-type=rlib", reconstruction)
facade_deps = {
    **clipping_deps,
    "semio_framework_3d_mesh_clipping": clipping,
    "semio_framework_3d_mesh_document": document,
    "semio_framework_3d_mesh_reconstruction": reconstruction,
    **{name: previous / f"lib{name}.rlib" for name in
       ("semio_framework_3d_brep_vector", "semio_framework_value_source_hold")},
}
facade = output / "libsemio_bevel_modeling.rlib"
compile("semio_bevel_modeling", mesh / "🦀️.rs", facade_deps, "--crate-type=rlib", facade)

original = source.read_text()
start = original.index("#[test]\nfn partitioned_difference_projects_authored_cutter_faces")
stop = original.index("#[test]\nfn authored_convex_intersection", start + 1)
proof = original[start:stop]
proof = re.sub(r'include_str!\("([^"]+)"\)', lambda match:
    "include_str!(" + json.dumps(str((source.parent / match[1]).resolve()), ensure_ascii=False) + ")", proof)
helpers = (previous / "proof.rs").read_text().split("\nfn assert_edit_surface(", 1)[1]
test = output / "proof.rs"
test.write_text("use semio_bevel_modeling::*;\n" + proof + "\nfn assert_edit_surface(" + helpers)
executable = output / "authored"
compile("longitudinal_slab_authored", test,
        {**value, "semio_bevel_modeling": facade, "serde_json": artifact("serde_json"),
         "semio_framework_mesh_engine": clipping_deps["semio_framework_mesh_engine"]}, "--test", executable)
subprocess.run([str(executable)], check=True)
