import sys
from importlib import import_module
from pathlib import Path

root = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(root))
runtime = import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
for key, value in runtime.profile_environment("preview", sys.argv[1]).items():
    print(f"export {key}='{value}'")
for token in runtime.profile_arguments("preview"):
    print("ARG", token)
