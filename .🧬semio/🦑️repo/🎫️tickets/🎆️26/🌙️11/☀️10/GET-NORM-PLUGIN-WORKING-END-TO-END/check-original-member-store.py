"""🫴️ Checks the current child-opening implementation against the compiled wasm32 ownership cohort."""
from pathlib import Path
import json
import subprocess
import sys

ticket = Path(__file__).resolve().parent
root = ticket.parents[6]
cohort = root / ".cache/b/r/m/u/l" / sys.argv[1]
lane = cohort / "wasm32-wasip2/prototype/build"
generation = sys.argv[2] if len(sys.argv) > 2 else "wMDTqyR"
graph = root / ".cache/b/r/m/components/runs" / generation / "l/unit-graph-prototype-00.json"
units = json.loads(graph.read_text())["units"]
unit = next(row for row in units if row["target"]["name"] == "semio_os_store_member_open_store"
            and row["platform"] == "wasm32-wasip2" and row["mode"] == "build")
output = ticket / "🗑️generated"
args = ["--edition=2021", "--target=wasm32-wasip2", "--crate-type=rlib",
        "--crate-name=semio_os_store_member_open_store", "-Cpanic=abort",
        "--emit=metadata", "-o", str(output / "member-store.rmeta"), unit["target"]["src_path"]]
args.extend(f"-Ldependency={path}" for path in cohort.glob("**/out") if "/build/" in str(path))
for dependency in unit["dependencies"]:
    source = units[dependency["index"]]
    alias = dependency["extern_crate_name"]
    folder = source["pkg_id"].rsplit("#", 1)[-1].split("@", 1)[0]
    artifact = max((lane / folder).glob("*/out/*.rlib"), key=lambda path: path.stat().st_mtime)
    args.extend(["--extern", f"{alias}={artifact.with_suffix('.rmeta')}"])
response = output / "member-store.args"
response.write_text("\n".join(args) + "\n")
compiler = subprocess.check_output(["rustup", "which", "rustc"], text=True).strip()
subprocess.run([compiler, "@" + str(response)], cwd=root, check=True)
print("[DEBUG] Current member-store source type checked against", sys.argv[1])
