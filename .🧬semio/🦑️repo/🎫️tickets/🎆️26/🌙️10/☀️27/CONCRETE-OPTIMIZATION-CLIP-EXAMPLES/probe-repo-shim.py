"""🧪 Probe whether the ticket repo shim runs node through the renderer's own subprocess path."""
import importlib
import os
import sys
from pathlib import Path

TICKET = Path(__file__).resolve().parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
os.environ["SEMIO_REPO_BINARY"] = str(TICKET / "repo-shim.cmd")
runtime = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
runtime.run_command([os.environ["SEMIO_REPO_BINARY"], "tools", "exec", "--light", "node", "--",
                     str(ROOT / "🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧪️tests/🟨️metadata.mjs")], cwd=ROOT)
print("[DEBUG] shim ok", flush=True)
