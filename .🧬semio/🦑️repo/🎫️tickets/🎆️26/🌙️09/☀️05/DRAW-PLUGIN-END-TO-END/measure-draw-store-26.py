"""📏 Measure each carved store crate through the granted compiler, using the captured native dependency set."""
import os
from pathlib import Path
import subprocess
import sys

ticket = Path(__file__).resolve().parent
capture = ticket / "🗑️generated" / "store-preparation-carve"
output = capture / "codegen-partitioned"
output.mkdir(exist_ok=True)
base = "semio_s_artifact_draw_drawing_editor_store_preparation"
wire = "semio_s_artifact_draw_drawing_editor_member_wire"
captured_wire = base + "_member_wire"
for recorded, crate in ((captured_wire, wire), (base, base)):
    args = (capture / (recorded + ".args")).read_text().splitlines()
    args[args.index("--crate-name") + 1] = crate
    args[args.index("--emit=metadata")] = "--emit=link"
    args[args.index("-o") + 1] = str(output / ("lib" + crate + ".rlib"))
    args.extend(["-Copt-level=2", "-Ccodegen-units=16", "-Cembed-bitcode=no", "-Zno-parallel-backend"])
    for i, arg in enumerate(args):
        if arg.startswith("--extern=") and arg.endswith(".rmeta"):
            key, _, previous = arg.rpartition("=")
            sibling = Path(previous).with_suffix(".rlib")
            if sibling.parent == capture:
                args[i] = key.replace(captured_wire, wire) + "=" + str(output / sibling.name.replace(captured_wire, wire))
    response = output / (crate + ".args")
    response.write_text("\n".join(dict.fromkeys(args)) + "\n")
    command = ["repo", "tools", "exec", "rustup", "--", "run", "nightly-2026-07-20", "rustc", "@" + str(response)]
    if sys.platform == "darwin":
        command = ["/usr/bin/time", "-l", *command]
    env = dict(os.environ, SEMIO_COMPILER_PARALLELISM="1", CARGO_BUILD_JOBS="1")
    with (output / (crate + ".log")).open("w") as log:
        result = subprocess.run(command, env=env, stdout=log, stderr=subprocess.STDOUT)
    if result.returncode:
        sys.exit(result.returncode)
