"""🧪️ Link the mounted Checks fixture against one coherent cached WASI unit graph."""

import os
import subprocess
import sys
from pathlib import Path


ticket = Path(__file__).resolve().parent
root = ticket.parents[6]
lane = root / ".cache/b/r/m/u/l" / sys.argv[1]
family = os.environ.get("NORM_CHECKS_FAMILY", "en1990")
build = lane / "wasm32-wasip2/prototype/build"
output = ticket / "🗑️generated"
name = "audit-norm-checks" if family == "en1990" else f"audit-norm-checks-{family}"
component = output / (name + ("-current.wasm" if len(sys.argv) == 2 else ".wasm"))
args = ["--edition=2021", "--target=wasm32-wasip2", "-Cpanic=abort", "-Copt-level=1",
        "-Cstrip=debuginfo", "-Clink-arg=-zstack-size=16777216", "--cfg", f'feature="{family}"', f"-o{component}", str(ticket / "audit-norm-checks.rs")]
args.extend(f"-Ldependency={p}" for d in (build, lane / "prototype/build") for p in d.glob("*/*/out"))
packages = {
    "audited_inspection": "semio-s-plugin-norm-app-surface-panels-inspection",
    "audited_model": f"semio-s-artifact-norm-{family}-model",
    "audited_report": f"semio-s-artifact-norm-{family}-inferences",
    "pack": "semio-framework-pack",
    "semio_framework_value": "semio-framework-value",
    "semio_plugin": "semio-plugin",
    "semio_s_plugin_norm_compliance": "semio-s-plugin-norm-compliance",
    "semio_ui_contract": "semio-ui-contract",
    "semio_ui_contract_values": "semio-ui-contract-values",
    "semio_ui_runtime": "semio-ui-runtime",
    "semio_ui_locale": "semio-ui-locale",
    "serde_json": "serde_json",
}
if family == "synthetic":
    for alias in ("audited_model", "audited_report", "pack"):
        del packages[alias]
for alias, package in packages.items():
    if alias == "audited_inspection" and len(sys.argv) > 2:
        library = Path(sys.argv[2]).resolve()
    else:
        library = max((build / package).glob("*/out/*.rlib"), key=lambda file: file.stat().st_mtime)
    for extension in (".rlib", ".rmeta"):
        args.extend(("--extern", f"{alias}={library.with_suffix(extension)}"))
flags = component.with_suffix(".args")
flags.write_text("\n".join(args) + "\n")
subprocess.run([os.environ["RUSTC"], "@" + str(flags)], cwd=root, check=True)
