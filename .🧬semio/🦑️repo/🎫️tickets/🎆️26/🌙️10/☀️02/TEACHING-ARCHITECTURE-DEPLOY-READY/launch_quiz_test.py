import os
import pathlib
import subprocess

root = pathlib.Path(__file__).resolve().parents[7]
probe = pathlib.Path(__file__).with_name("schema_short.py").read_text(encoding="utf-8")
schema_line = next(line for line in probe.splitlines() if line.startswith("schema_path = "))
namespace = {"root": root, "pathlib": pathlib}
exec(schema_line, namespace)
quiz = namespace["schema_path"].parents[1]
packages = next(path for path in quiz.iterdir() if path.name.endswith("packages"))
manifest = next(path for path in packages.iterdir() if path.name.endswith("rust")) / "Cargo.toml"
semio = next(path for path in root.iterdir() if path.name.endswith("semio") and path.name.startswith("."))
repo = next(path for path in semio.iterdir() if path.name.endswith("repo"))
cache = next(path for path in repo.iterdir() if path.name.endswith("cache"))
cargo = next(path for path in cache.iterdir() if path.name.endswith("cargo"))
target = cargo / "target-architecture-quiz"
env = os.environ.copy()
env["CARGO_TARGET_DIR"] = str(target)
print("manifest", manifest)
print("target", target)
raise SystemExit(subprocess.call(["cargo", "test", "--offline", "--manifest-path", str(manifest), "--lib", "--", "--test-threads", "8"], cwd=root, env=env))
