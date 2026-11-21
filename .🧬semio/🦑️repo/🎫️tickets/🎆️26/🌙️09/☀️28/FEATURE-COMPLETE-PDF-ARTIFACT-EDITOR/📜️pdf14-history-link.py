"""🔗 Link PDF history fixture discovery against the retained governed Rust dependencies."""
import os
from pathlib import Path
import subprocess

ticket = Path(__file__).resolve().parent
generated = ticket / "🗑️generated"
cohort = generated / "sl" / "cohort"
fixture = generated / "pdf14-history"
names = ("semio_plugin", "semio_plugin_testing_acceptance", "semio_s_artifact_stdio_pdf_schema_5", "semio_os_kernel", "semio_framework_value", "semio_framework_pack_json")
for variant in ("a", "x"):
    path = fixture / variant / "case" / "🦠️mutation" / "🔣️.json"
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_bytes((ticket / f"🔣️pdf14{variant}-history.json").read_bytes())
target = fixture / ("probe.exe" if os.name == "nt" else "probe")
args = ["--edition=2024", "--test", "--crate-name", "pdf14_history_profile_fixture", str(ticket / "📜️pdf14-history-law.rs"), "-o", str(target)]
for name in names:
    candidates = list(cohort.glob(f"*/*/out/lib{name}-*.rlib"))
    assert len(candidates) == 1, (name, candidates)
    archive = candidates[0]
    args.extend(("--extern", f"{name}={archive.with_suffix('.rmeta')}", "--extern", f"{name}={archive}"))
for directory in sorted(cohort.glob("*/*/out")):
    args.extend(("-L", f"dependency={directory}"))
response = fixture / "args"
response.write_text("\n".join(args) + "\n", encoding="utf8")
subprocess.run(["repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20", "rustc", "@" + str(response)], check=True)
subprocess.run([str(target), "--nocapture"], env={**os.environ, "SEMIO_PDF14_HISTORY_PROBE": str(fixture)}, check=True)
