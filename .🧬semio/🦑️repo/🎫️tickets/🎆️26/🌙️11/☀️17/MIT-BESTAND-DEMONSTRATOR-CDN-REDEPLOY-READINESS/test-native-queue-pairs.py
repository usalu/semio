"""🧪️ Exercise the explicit two-root queue and verify its launches and confirmations with SQLite."""
import json
import os
import runpy
from pathlib import Path
import shlex
import sqlite3
import subprocess
import sys
import tempfile


ROOT = Path(__file__).resolve().parent
FIXTURE = ROOT / "🔣️native-queue-pairs.json"
RECEIPT = "🧾️receipt.json"


def write_receipt(module, root, certified=True):
    receipt = dict(pluginId=module, schema="semio.plugin.component-receipt/v1", target="wasm32-wasip2", profile="release")
    receipt.update(dict.fromkeys(("sourceTreeDigest", "wasmSha256", "provenanceSha256", "semanticDescriptorSha256"), "a" * 64))
    folder = root / ("🧩️" + module)
    folder.mkdir(parents=True, exist_ok=True)
    (folder / RECEIPT).write_text(json.dumps(receipt))
    if certified:
        (folder / "component.core.wasm").write_bytes(b"\x00asm-pair")


def driver(arguments):
    fixture = json.loads(FIXTURE.read_text())
    step, name, cwd, repo, script, command, action, *selection = arguments
    assert (step, cwd, repo, command, action, selection[-2:]) == (
        "step", "", "repo", "components", "describe", ["--facet", "browser"]
    ), arguments
    assert script == "./📜️script.ts"
    roots = selection[:-2]
    assert os.environ["FORCE"] == "1" and 1 <= len(roots) <= 2
    logs, modules = (Path(os.environ[key]) for key in ("QUEUE_GENERATED", "QUEUE_MODULES"))
    with (logs / "attempts.jsonl").open("a") as handle:
        print(json.dumps(roots), file=handle)
    if len(roots) == 1:
        module, = roots
        if module == "n":
            assert not any((logs / "confirmed" / (member + ".json")).exists() for member in ("n", "o"))
        if module in fixture["singleFailures"]:
            return 1
        write_receipt(module, modules)
        (logs / (name + ".log")).write_text("[native-build] 3 unit(s), 1 component(s) staged, 0 failure(s) in 1 s; report /fixture/r/report.md\n")
        return 0
    pair = next(row for row in fixture["pairs"] if row["roots"] == roots)
    assert name == "browser-pair-" + "+".join(roots)
    for module in pair["receipts"]:
        write_receipt(module, modules, certified=module not in pair.get("uncertified", []))
    lines = [f"[native-build] 123 unit(s), {count} component(s) staged, 10 stage run(s), {failures} failure(s) in 20 s; report /fixture/report.md" for count, failures in pair["summaries"]]
    (logs / (name + ".log")).write_text("\n".join(lines) + "\n")
    return pair["status"]


def main():
    fixture = json.loads(FIXTURE.read_text())
    roots = [root for pair in fixture["pairs"] for root in pair["roots"]] + [fixture["tail"]]
    with tempfile.TemporaryDirectory(prefix="queue-pair-", dir=ROOT / "🗑️generated") as directory:
        scratch = Path(directory)
        fake = scratch / "driver"
        fake.write_text("#!/usr/bin/env bash\nexec " + shlex.quote(sys.executable) + " " + shlex.quote(__file__) + ' --driver "$@"\n')
        fake.chmod(0o755)
        receipt_fixture = runpy.run_path(str(ROOT / "fixture-release-receipts.py"))
        receipt_fixture["install_python_shim"](scratch)
        env = dict(os.environ, PATH=str(scratch) + os.pathsep + os.environ["PATH"], JOBS="2", QUEUE_DRIVER=str(fake), QUEUE_GENERATED=str(scratch), QUEUE_MODULES=str(scratch / "modules"))
        base = ["bash", str(ROOT / "native-queue.sh"), "--pairs"]
        assert subprocess.run(base + ["a", "a"], env=env, capture_output=True).returncode == 2
        assert subprocess.run(base + roots, env=dict(env, JOBS="3"), capture_output=True).returncode == 2
        assert not (scratch / "attempts.jsonl").exists()
        evidence = scratch / "confirmed"
        write_receipt("f", scratch / "modules")
        evidence.mkdir()
        (evidence / "f.json").write_text("{}")
        result = subprocess.run(base + roots, env=env, capture_output=True, text=True)
        print(result.stdout, end="")
        assert result.returncode == 1, (result.returncode, result.stderr)
        attempts = [json.loads(line) for line in (scratch / "attempts.jsonl").read_text().splitlines()]
        confirmed = sorted(p.stem for p in evidence.glob("*.json"))
        with sqlite3.connect(":memory:") as db:
            for key, actual in (("attempts", attempts), ("confirmed", confirmed)):
                expected = fixture["expect"][key]
                expected_json, actual_json = json.dumps(expected), json.dumps(actual)
                mismatch = db.execute(
                    "select exists(select key,value from json_each(?) except select key,value from json_each(?)) or json_array_length(?) != json_array_length(?)",
                    (expected_json, actual_json, expected_json, actual_json),
                ).fetchone()[0]
                assert not mismatch, (key, actual, expected)
            width = db.execute("select max(json_array_length(value)) from json_each(?)", (json.dumps(attempts),)).fetchone()[0]
            assert width == 2
        assert not list(scratch.glob("pair.??????"))
        for module in confirmed:
            record = json.loads((evidence / (module + ".json")).read_text())
            assert record["pluginId"] == module
            assert record["optimizedBrowserCores"] == receipt_fixture["synthetic_certification"]([{"stagedReceipt": str(scratch / "modules" / ("🧩️" + module) / RECEIPT)}])[0]
        print("paired queue fixture passed")


if __name__ == "__main__":
    if len(sys.argv) > 1:
        raise SystemExit(driver(sys.argv[2:]))
    main()
