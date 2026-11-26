"""🧾️ Identify staged release inputs and verify the copies served by the static site."""
import argparse
import gzip
import filecmp
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess

RECEIPT = "🧾️receipt.json"
BROWSER_CORES = "optimizedBrowserCores"
ROOT = Path(__file__).resolve().parents[7]
CASES = ROOT / "♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/🧫️cases.json"


def receipt_data(data, path, module):
    if path.name != RECEIPT:
        data = {key: value for key, value in data.items() if key != BROWSER_CORES}
    if (data["pluginId"], data["schema"], data["target"], data["profile"]) != (
        module, "semio.plugin.component-receipt/v1", "wasm32-wasip2", "release"
    ):
        raise ValueError(f"unexpected release identity at {path}")
    for key in ("wasmSha256", "semanticDescriptorSha256", "sourceTreeDigest", "provenanceSha256"):
        if not re.fullmatch("[0-9a-f]{64}", data[key]):
            raise ValueError(f"invalid {key} at {path}")
    return data


def read_receipt(path, module):
    return receipt_data(json.loads(path.read_bytes()), path, module)


def core_inventory(value):
    """🧬️ Admit a durable optimized browser inventory in the queue confirmation."""
    if not isinstance(value, dict) or value.keys() != {"version", "files"} or type(value["version"]) is not int or value["version"] != 1 or not isinstance(value["files"], list) or not value["files"]:
        raise ValueError("optimized browser core inventory is missing or invalid")
    files, names = value["files"], set()
    for row in files:
        if not isinstance(row, dict) or row.keys() != {"path", "sha256", "bytes"}:
            raise ValueError("invalid optimized browser core evidence")
        name = row["path"]
        if not isinstance(name, str) or not re.fullmatch(r"[A-Za-z0-9_.-]+\.wasm", name) or name in names or type(row["bytes"]) is not int or row["bytes"] <= 0 or not isinstance(row["sha256"], str) or not re.fullmatch("[0-9a-f]{64}", row["sha256"]):
            raise ValueError("invalid optimized browser core evidence")
        names.add(name)
    return {"version": 1, "files": sorted(files, key=lambda file: file["path"])}


def read_confirmation(path, module):
    data = json.loads(path.read_bytes())
    return receipt_data(data, path, module), core_inventory(data.get(BROWSER_CORES))


def persist_confirmation(path, receipt, inventory):
    path.parent.mkdir(parents=True, exist_ok=True)
    pending = path.with_suffix(f".{os.getpid()}.pending")
    try:
        pending.write_text(json.dumps({**receipt, BROWSER_CORES: core_inventory(inventory)}, ensure_ascii=False) + "\n")
        pending.replace(path)
    finally:
        pending.unlink(missing_ok=True)


def receipt_path(directory, module):
    paths = list(directory.glob(f"*️{module}/{RECEIPT}"))
    if len(paths) != 1:
        raise ValueError(f"expected one receipt for {module} in {directory}, found {len(paths)}")
    return paths[0]


def authored_release_digests(modules):
    """🧬️ Verify the current authored pairs with the production semantic descriptor codec."""
    if not modules:
        return {}
    helper = Path(__file__).with_name("authored-release-descriptors.ts")
    result = subprocess.run(
        ["repo", "tools", "exec", "--light", "bun", "--", str(helper), *modules],
        cwd=ROOT, encoding="utf-8", capture_output=True,
    )
    if result.returncode:
        raise ValueError(result.stderr.strip() or "authored semantic descriptor verification failed")
    digests = json.loads(result.stdout)
    if not isinstance(digests, dict) or digests.keys() != set(modules) or any(
        not isinstance(digest, str) or not re.fullmatch("[0-9a-f]{64}", digest) for digest in digests.values()
    ):
        raise ValueError("authored semantic descriptor verification returned an incomplete roster")
    return digests


def certify_browser_cores(requests):
    """🧷️ Bind each candidate's optimized cores to the raw release and browser probe."""
    if not requests:
        return []
    helper = Path(__file__).with_name("verify-browser-core-lineage.ts")
    result = subprocess.run(
        ["repo", "tools", "exec", "--light", "bun", "--", str(helper), json.dumps(requests, ensure_ascii=False)],
        cwd=ROOT, encoding="utf-8", capture_output=True,
    )
    if result.returncode:
        raise ValueError(result.stderr.strip() or "optimized browser core verification failed")
    rows = json.loads(result.stdout)
    if not isinstance(rows, list) or len(rows) != len(requests):
        raise ValueError("optimized browser core certification returned an incomplete roster")
    return [core_inventory({"version": 1, "files": row}) for row in rows]


def verify_recorded_cores(root, inventory):
    files = inventory["files"]
    expected = {file["path"] for file in files}
    paths = [p for p in root.rglob("*") if p.name.endswith((".wasm", ".wasm.gz.bin"))]
    actual = {p.relative_to(root).as_posix().removesuffix(".gz.bin"): p for p in paths}
    if expected != actual.keys() or len(actual) != len(paths):
        raise ValueError(f"Optimized browser core inventory differs in {root}")
    for file in files:
        path = actual[file["path"]]
        if path.is_symlink() or not path.is_file():
            raise ValueError(f"Invalid browser core: {path}")
        digest = hashlib.sha256()
        size = 0
        opener = gzip.open if path.name.endswith(".gz.bin") else open
        with opener(path, "rb") as source:
            for chunk in iter(lambda: source.read(1024 * 1024), b""):
                size += len(chunk)
                if size > file["bytes"]:
                    raise ValueError(f"Browser core exceeds its recorded length: {path}")
                digest.update(chunk)
        if size != file["bytes"] or digest.hexdigest() != file["sha256"]:
            raise ValueError(f"Browser core differs from its successful build: {path}")


def core_certification_request(staged, receipt):
    return {"stagedReceipt": str(staged.absolute()), "wasmSha256": receipt["wasmSha256"], "semanticDescriptorSha256": receipt["semanticDescriptorSha256"]}


def record_confirmation(staged, destination, module, expected=None):
    data = read_receipt(staged, module)
    if expected is not None and data != expected:
        raise ValueError(f"{module} changed before optimized browser core certification")
    inventory, = certify_browser_cores([core_certification_request(staged, data)])
    if read_receipt(staged, module) != data:
        raise ValueError(f"{module} changed during optimized browser core certification")
    verify_recorded_cores(staged.parent, inventory)
    persist_confirmation(destination, data, inventory)


def verify_deployed_module(staged, deployed):
    names = json.loads((staged / ".semio-artifact.json").read_bytes())["files"]
    copies = {p.relative_to(deployed).as_posix() for p in deployed.rglob("*") if p.is_file()}
    encoded = {name.removesuffix(".gz.bin") for name in copies if name.endswith(".wasm.gz.bin")}
    decoded = {name.removesuffix(".gz.bin") if name.endswith(".wasm.gz.bin") else name for name in copies}
    if not isinstance(names, list) or set(names) != decoded or len(names) != len(copies):
        raise ValueError(f"Deployed file inventory differs from the module marker in {deployed}")
    if not any(name.endswith(".wasm") for name in decoded):
        raise ValueError(f"Deployed module has no WebAssembly in {deployed}")
    bindings = {re.sub(r"\.core\d*\.wasm$", ".js", core) for core in encoded}
    scripts = bindings | {name for name in decoded if name.endswith(".js") and deployed.parent.name == "🧩️extension-modules"}
    uses_shard_gzip = False
    if scripts:
        helper = Path(__file__).with_name("verify-extension-imports.mjs")
        result = subprocess.run(["repo", "tools", "exec", "--light", "node", "--", str(helper), str(staged), str(deployed), *sorted(scripts)],
                                stdout=subprocess.PIPE, encoding="utf-8", check=True)
        uses_shard_gzip = json.loads(result.stdout)
        if not isinstance(uses_shard_gzip, bool):
            raise ValueError(f"Invalid binding transport verdict for {deployed}")
    for name in sorted(decoded - scripts - encoded):
        if not filecmp.cmp(staged / name, deployed / name, shallow=False):
            raise ValueError(f"Deployed module file differs from its staged build: {deployed / name}")
    return uses_shard_gzip


def verify_deployed_shard_worker(site):
    helper = Path(__file__).with_name("verify-shard-worker.ts")
    worker = site / "🔌️plugin-modules/🧵️shard/🟨️shard-worker.js"
    subprocess.run(["repo", "tools", "exec", "--light", "bun", "--", str(helper), str(worker)], check=True)


def roster(cases):
    return [(module, directory) for group, directory in (("pluginIds", "🔌️plugin-modules"), ("extensionIds", "🧩️extension-modules")) for module in cases[group]]


def missing_report(modules, evidence, cases):
    """📋️ List every roster module without a queue confirmation and say whether its release was at least staged."""
    rows = roster(cases)
    missing = [module for module, _ in rows if not (evidence / f"{module}.json").is_file()]
    lines = [f"{len(rows) - len(missing)} of {len(rows)} roster modules confirmed; {len(missing)} still missing:"]
    for module in missing:
        try:
            receipt_path(modules, module)
            state = "staged, awaiting confirmation"
        except ValueError:
            state = "no staged release"
        lines.append(f"  {module}: {state}")
    return missing, "\n".join(lines)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("operation", choices=("record", "verify", "status"))
    parser.add_argument("modules", type=Path)
    parser.add_argument("evidence", type=Path)
    parser.add_argument("module", nargs="?")
    parser.add_argument("--site", type=Path)
    args = parser.parse_args()
    if args.operation == "record":
        if not args.module or not re.fullmatch("[a-z0-9-]+", args.module):
            parser.error("record requires a module id")
        record_confirmation(receipt_path(args.modules, args.module), args.evidence / f"{args.module}.json", args.module)
        return
    cases = json.loads(CASES.read_bytes())
    missing, report = missing_report(args.modules, args.evidence, cases)
    if args.operation == "status":
        print(report)
        raise SystemExit(1 if missing else 0)
    if missing:
        raise ValueError(report)
    verified = []
    for group, directory in (("pluginIds", "🔌️plugin-modules"), ("extensionIds", "🧩️extension-modules")):
        for module in cases[group]:
            receipt = args.evidence / f"{module}.json"
            expected, inventory = read_confirmation(receipt, module)
            staged = receipt_path(args.modules, module)
            if read_receipt(staged, module) != expected:
                raise ValueError(f"{module} changed since its successful build")
            verified.append((module, expected, staged, directory, inventory))
    digests = authored_release_digests(list(dict.fromkeys(module for module, _, _, _, _ in verified)))
    uses_shard_gzip = False
    for module, expected, staged, directory, inventory in verified:
        digest = digests[module]
        if digest != expected["semanticDescriptorSha256"]:
            raise ValueError(f"{module}: authored semantic descriptor {digest} differs from its successful build {expected['semanticDescriptorSha256']}")
        verify_recorded_cores(staged.parent, inventory)
        if args.site:
            deployed = receipt_path(args.site / directory, module)
            if read_receipt(deployed, module) != expected:
                raise ValueError(f"{module} site differs from its successful build")
            verify_recorded_cores(deployed.parent, inventory)
            if verify_deployed_module(staged.parent, deployed.parent):
                uses_shard_gzip = True
    if uses_shard_gzip:
        verify_deployed_shard_worker(args.site)


if __name__ == "__main__":
    try:
        main()
    except (OSError, ValueError, KeyError, subprocess.CalledProcessError) as error:
        raise SystemExit(str(error))
