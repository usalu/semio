"""🧪 Run output contracts through the registered repository tool interface."""

from pathlib import Path
import os
import subprocess
import sys

WORKSPACE = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
GENERATED = Path(__file__).parent / "🗑️generated"
GENERATED.mkdir(exist_ok=True)
environment = {
    **os.environ,
    "UV_PROJECT_ENVIRONMENT": str(WORKSPACE / ".🧬semio/🎓️teaching/⚡️cache/python"),
    "SEMIO_TEACHING_OUTPUT_ROOT": str(GENERATED / "tests"),
    "PYTHONUTF8": "1",
    "PYTHONIOENCODING": "utf-8",
    "PYTHONDONTWRITEBYTECODE": "1",
    "SEMIO_TEST_MANIM": "1",
}
Path(environment["SEMIO_TEACHING_OUTPUT_ROOT"]).mkdir(exist_ok=True)
modules = sys.argv[1:] or ["🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.📁️output.🧪️tests.🐍️"]
command = ["repo", "tools", "exec", "uv", "--", "run", "--project", str(WORKSPACE / "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python"), "--python", "3.12", "--no-sync", "python", "-B", "-m", "unittest", *modules]
raise SystemExit(subprocess.run(command, cwd=WORKSPACE, env=environment).returncode)
