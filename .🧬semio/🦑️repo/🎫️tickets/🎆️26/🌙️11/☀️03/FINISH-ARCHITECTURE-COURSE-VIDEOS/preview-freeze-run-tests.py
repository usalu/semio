"""🧪 Regenerate the independent oracle and run the bounded freeze tests."""
import subprocess
import sys
from pathlib import Path

here = Path(__file__).resolve().parent
(here / "🗑️generated").mkdir(parents=True, exist_ok=True)
subprocess.run(["repo", "tools", "exec", "--light", "node", "--", str(here / "preview-freeze-oracle.cjs")], check=True)
subprocess.run(["repo", "tools", "exec", "--light", "uv", "--", "run", "--no-project", "--python", sys.executable, "python", "-B", str(here / "preview-freeze-test.py")], check=True)
