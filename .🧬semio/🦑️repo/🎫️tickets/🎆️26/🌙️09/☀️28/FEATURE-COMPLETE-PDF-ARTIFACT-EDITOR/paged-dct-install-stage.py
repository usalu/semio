"""🗃️ Stages the native DCT Pack event codec and a probe against existing first-party build outputs."""
import json
import runpy
from pathlib import Path

TICKET = Path(__file__).resolve().parent
GRAPH = runpy.run_path(str(TICKET / "paged-jpeg-graph-stage.py"))
GRAPH["synthesize"]()
OUT = TICKET / "🗑️generated" / "paged-dct-install"
BASE = GRAPH["BASE"]
SOURCE = GRAPH["OUT"]


def stage():
    OUT.mkdir(parents=True, exist_ok=True)
    for target, name in [("🗃️install-pack.rs", "paged-dct-install-pack.rs"), ("probe.rs", "paged-dct-install-probe.rs")]:
        (OUT / target).write_text((TICKET / name).read_text())
    (OUT / "📖️dct-view.rs").write_text((SOURCE / "📖️dct-view.rs").read_text())
    identity = (SOURCE / "identity.rs").read_text()
    (OUT / "identity.rs").write_text(identity[:identity.index("pub(crate) fn qualified_image_dictionary")])
    foreign = (SOURCE / "foreign-artifacts.rs").read_text()
    (OUT / "foreign-artifacts.rs").write_text(
        GRAPH["one"](foreign, "pub mod dct_lookup;", 'pub mod dct_lookup;\n\n#[path = "🗃️install-pack.rs"]\npub mod install_pack;'))
    manifest = (BASE / "🧬️schema/💡️inferences/📦️packages/🦀️rust/Cargo.toml").read_text()
    (OUT / "inferences.Cargo.toml").write_text(GRAPH["one"](manifest, "pack = { workspace = true }\n",
        'pack = { workspace = true }\nsemio-framework-pack-source = { path = "../../../../../../../../../../../../../🧰️framework/🔨️modules/🎒️pack/🚰️source/📦️packages/🦀️rust" }\n'))


def cached_args():
    root = Path(".cache/b/r/m/u/l/pe73647/prototype/build")
    paths = {p.name[3:-5]: p for p in root.glob("*/*/out/*.rlib") if p.name.startswith("lib")}
    base = next(path for key, path in paths.items() if key.startswith("pack-"))
    deps = json.loads(base.with_suffix(".rmeta.semio-deps").read_text())["dependencies"]
    keys = {name for name, _ in deps}
    args = ["--edition=2021", "-C", "opt-level=1", "-o", str(OUT / "probe"), str(OUT / "probe.rs"),
            "--extern", f"pack={base.with_suffix('.rmeta')}", "--extern", f"pack={base}"]
    for name in ("semio_framework_artifact_reference", "semio_framework_pack_source", "semio_framework_value", "semio_framework_hash"):
        candidates = [path for key, path in paths.items() if key.startswith(name + "-")]
        selected = [path for path in candidates if path.name[3:-5] in keys] or candidates
        if len(selected) != 1:
            raise ValueError((name, selected))
        args.extend(["--extern", f"{name}={selected[0].with_suffix('.rmeta')}", "--extern", f"{name}={selected[0]}"])
    linked = {p.parent for p in root.glob("*/*/out/*") if p.suffix in (".rmeta", ".rlib", ".so", ".dylib")}
    args.extend(f"-Ldependency={path}" for path in sorted(linked))
    (OUT / "probe.args").write_text("\n".join(args) + "\n")


if __name__ == "__main__":
    stage()
    cached_args()
