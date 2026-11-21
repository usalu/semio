"""🔬 Run the paged ingress laws against repository-built value owners."""

import json
import os
from pathlib import Path
import subprocess
import sys
from runpy import run_path

TICKET = Path(__file__).resolve().parent
OUT = TICKET / "🗑️generated" / "paged-media-receiver"
CACHE = Path(sys.argv[1])
OUT.mkdir(parents=True, exist_ok=True)
target = OUT / ("check.exe" if os.name == "nt" else "check")
args = [
    "--edition=2021", "--test", "--crate-name", "original_media_receiver_test", "-Copt-level=2",
    str(TICKET / "📜️original-media-paged-receiver-test.rs"), "-o", str(target),
]
owners = dict.fromkeys(("semio_framework_value", "semio_framework_value_persistent", "semio_framework_replication_command_ingress", "semio_framework_job"), "*")
if "--product-module" in sys.argv:
    run_path(str(TICKET / "raw-picker-product-stage.py"))["stage"]()
    args += ["--cfg", "raw_picker_product_module"]
if "--reactor-product" in sys.argv:
    subprocess.run([sys.executable, str(TICKET / "raw-reactor-wire-build.py"), str(CACHE)], check=True)
    args += ["--cfg", "raw_picker_product_module", "--cfg", "raw_reactor_product_module"]
if "--issuer-product" in sys.argv:
    run_path(str(TICKET / "raw-pdf-picker-stage.py"))["stage"]()
    run_path(str(TICKET / "raw-pdf-picker-oracle.py"))["verify"]()
    args += ["--cfg", "raw_pdf_issuer_module"]
if "--original-fixture" in sys.argv:
    plugin = TICKET.parents[6] / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin"
    source = plugin / "⚛️reactor/🏃️runtime/🎞️command/📥️custody/🎟️source/🦀️.rs"
    production, marker, _ = source.read_text(encoding="utf8").partition("#[cfg(test)]")
    if not marker or "pub struct OriginalCommandOrigin" not in production:
        raise ValueError("Command origin production source boundary drifted")
    if "--live-command-origin" not in sys.argv:
        production = run_path(str(TICKET / "raw-picker-origin-stage.py"))["stage"](production)
    (OUT / "command-origin.rs").write_text(production, encoding="utf8")
    browser = json.loads((TICKET / "original-media-react-cases.json").read_bytes())
    request = browser["request"]
    loan = request["loan"]
    revision = list(bytes.fromhex(loan["baseRevision"]))
    (OUT / "browser-law.rs").write_text(
        'use crate::ingress::OriginalMediaLoan;\n'
        f'pub const RAW_KIND: u8 = {request["kind"]};\n'
        f'pub const LIMIT: usize = {request["maximumFileBytes"]};\n'
        f'pub const SEQ: u64 = {int(browser["seq"])};\n'
        f'pub const OPERATION_HEX: &str = {json.dumps(loan["operationId"])};\n'
        f'pub const GENERATION_HEX: &str = {json.dumps(loan["generation"])};\n'
        f'pub const REVISION_HEX: &str = {json.dumps(loan["baseRevision"])};\n'
        "pub const LOAN: OriginalMediaLoan<'static> = OriginalMediaLoan {\n"
        f'    operation_id: {int(loan["operationId"], 16)}, generation: {int(loan["generation"], 16)},\n'
        f'    instance_id: {loan["instanceId"]}, document_id: {json.dumps(loan["documentId"])}, base_revision: &{revision!r},\n'
        '};\n',
        encoding="utf8",
    )
    args += ["--cfg", "original_media_source_fixture"]
    owners.update({
        "semio_framework_value": "86eb0cf0fad7ae66",
        "semio_framework_value_persistent": "d19346c8b0514097",
        "semio_framework_replication_command_ingress": "8cf2689dbcc20a4d",
        "semio_framework_job": "689697170f9e2e90",
        "semio_framework_job_testing_caller": "382719391b729040",
    })
if "--reactor-product" in sys.argv:
    del owners["semio_framework_replication_command_ingress"]
    args += ["--extern", f"semio_framework_replication_command_ingress={OUT.parent / 'rm-wire/libsemio_framework_replication_command_ingress.rlib'}"]
for name, suffix in owners.items():
    [library] = CACHE.glob(f"*/*/out/lib{name}-{suffix}.rlib")
    args += ["--extern", f"{name}={library.with_suffix('.rmeta')}", "--extern", f"{name}={library}"]
args += [
    f"-Ldependency={directory}"
    for directory in sorted({path.parent for path in CACHE.glob("*/*/out/*") if path.suffix in (".rmeta", ".rlib", ".dylib", ".dll", ".so")})
]
response = OUT / "args"
response.write_text("\n".join(args) + "\n", encoding="utf8")
subprocess.run(["repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20", "rustc", "@" + str(response)], check=True)
subprocess.run(["repo", "capacity", "run", "--memory=256", "--", str(target), "--nocapture"], check=True)
