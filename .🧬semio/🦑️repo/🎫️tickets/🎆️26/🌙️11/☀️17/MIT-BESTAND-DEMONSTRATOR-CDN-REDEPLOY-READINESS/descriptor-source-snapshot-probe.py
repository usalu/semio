"""🧊 Exercise native Cargo input identity in an immutable mirror fixture."""
from pathlib import Path
import hashlib
import json
import os
import shutil
import subprocess
import sys
import tempfile


def write(root, name, content):
    path = root / name
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(content, encoding="utf-8")


def cargo(root, target, *args, environment=None):
    env = dict(os.environ)
    env.update(environment or {})
    return subprocess.check_output(["repo", "cargo", "--store=shared", "--configuration=prototype",
                                    "--parallelism=1", "--", *args, "--manifest-path", str(root / "Cargo.toml")],
                                   env=env, text=True)


def local_package_id(root, graph):
    packages = graph["packages"]
    paths = [(pkg["name"], Path(pkg["manifest_path"]).relative_to(root).as_posix()) for pkg in packages]
    return sorted(paths)


base = Path(tempfile.mkdtemp(prefix="src-", dir=sys.argv[1])).resolve()
live, frozen, target = (base / name for name in ["l", "f", "t"])
write(live, "Cargo.toml", 'cargo-features = ["trim-paths"]\n[workspace]\nresolver = "2"\nmembers = ["lib", "app"]\n[profile.prototype]\ninherits = "dev"\n')
write(live, "lib/Cargo.toml", '[package]\nname = "libowner"\nversion = "0.1.0"\nedition = "2021"\n')
write(live, "lib/src/lib.rs", 'pub fn source() -> &\'static str { include_str!("../words") }\n')
write(live, "lib/words", "old immutable payload")
write(live, "app/Cargo.toml", '[package]\nname = "descriptorfixture"\nversion = "0.1.0"\nedition = "2021"\n[dependencies]\nlibowner = {path = "../lib"}\n')
write(live, "app/build.rs", 'fn main() { println!("cargo:rerun-if-changed=build-msg"); println!("cargo:rustc-env=FIXTURE_BUILD_VALUE={}", std::fs::read_to_string("build-msg").unwrap()); }\n')
write(live, "app/build-msg", "old build input")
write(live, "app/src/main.rs", 'fn main() { println!("{}|{}|{}", libowner::source(), env!("FIXTURE_BUILD_VALUE"), env!("CARGO_MANIFEST_DIR")); }\n')
write(live, "Cargo.lock", 'version = 4\n[[package]]\nname = "descriptorfixture"\nversion = "0.1.0"\ndependencies = ["libowner"]\n[[package]]\nname = "libowner"\nversion = "0.1.0"\n')
shutil.copytree(live, frozen)
sources = {path.relative_to(frozen): hashlib.sha256(path.read_bytes()).hexdigest()
           for path in frozen.rglob("*") if path.is_file()}
for name in sources:
    (frozen / name).chmod(0o400)
write(live, "lib/words", "new live payload")
write(live, "app/build-msg", "new live build input")
manifests = lambda root: json.loads(cargo(root,target,"metadata","--offline","--locked","--format-version=1"))
original_graph, frozen_graph = manifests(live), manifests(frozen)
assert local_package_id(live, original_graph) == local_package_id(frozen, frozen_graph)
assert {p["id"] for p in original_graph["packages"]}.isdisjoint({p["id"] for p in frozen_graph["packages"]})
run = cargo(frozen, target, "run", "--locked", "--offline", "-p", "descriptorfixture")
expected = "old immutable payload|old build input|" + str(frozen / "app")
assert expected in run.splitlines(), run
assert all(hashlib.sha256((frozen / path).read_bytes()).hexdigest() == digest for path,digest in sources.items())
print(json.dumps({"owners":len(original_graph["packages"]), "inputs":len(sources), "compile":"owned fixture snapshot",
                  "buildScriptBody":"old build input", "sourceInclude":"old immutable payload",
                  "cargoIdChanged":True, "envManifestDirChanged":True},indent=2))
