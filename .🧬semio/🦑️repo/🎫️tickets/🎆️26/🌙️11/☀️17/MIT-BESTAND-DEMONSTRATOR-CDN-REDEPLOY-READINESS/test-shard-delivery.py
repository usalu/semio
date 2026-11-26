"""🧪 Require a current static shard worker for an encoded factory delivery."""
import gzip
import json
from pathlib import Path
import runpy
import subprocess
from tempfile import TemporaryDirectory

ticket = Path(__file__).resolve().parent
tools = runpy.run_path(str(ticket / "release-receipts.py"))
generated = ticket / "🗑️generated"
generated.mkdir(exist_ok=True)
root = tools["ROOT"]

def reject(operation, *args):
    try:
        operation(*args)
    except (OSError, ValueError, subprocess.CalledProcessError):
        return
    raise AssertionError("Unusable compressed factory delivery was accepted")

with TemporaryDirectory(dir=generated, prefix="sw-") as scratch:
    directory = Path(scratch)
    staged = directory / "stage"
    site = directory / "site"
    deployed = site / "🔌️plugin-modules/🧩️fixture"
    staged.mkdir()
    deployed.mkdir(parents=True)
    core = "fixture.core.wasm"
    binding = "fixture.js"
    factory = "export function instantiate(getCoreModule, imports, instantiateCore, options) { return getCoreModule('fixture.core.wasm'); }\n"
    core_bytes = b"\0asm\1\0\0\0"
    (staged / ".semio-artifact.json").write_text(json.dumps({"files": [core, binding]}), encoding="utf-8")
    (staged / core).write_bytes(core_bytes)
    (staged / binding).write_text(factory, encoding="utf-8")
    (deployed / (core + ".gz.bin")).write_bytes(gzip.compress(core_bytes, mtime=0))
    (deployed / binding).write_text(factory.replace("fixture.core.wasm", "fixture.core.wasm.gz.bin"), encoding="utf-8")

    assert tools["verify_deployed_module"](staged, deployed) is True
    reject(tools["verify_deployed_shard_worker"], site)
    worker = site / "🔌️plugin-modules/🧵️shard/🟨️shard-worker.js"
    worker.parent.mkdir()
    source = subprocess.check_output(
        ["repo", "tools", "exec", "--light", "bun", "--", "-e",
         'import {shardWorkerSource} from "./🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🧵️shard-worker-source/🟦️.ts"; process.stdout.write(shardWorkerSource())'],
        cwd=root,
    )
    worker.write_bytes(source)
    tools["verify_deployed_shard_worker"](site)
    worker.write_bytes(source.replace(b"DecompressionStream", b"UnrecognizedStream"))
    reject(tools["verify_deployed_shard_worker"], site)
    (deployed / binding).write_text(factory, encoding="utf-8")
    reject(tools["verify_deployed_module"], staged, deployed)

print("Compressed factory delivery rejects a missing or stale worker and verifies exact rewritten bindings.")
