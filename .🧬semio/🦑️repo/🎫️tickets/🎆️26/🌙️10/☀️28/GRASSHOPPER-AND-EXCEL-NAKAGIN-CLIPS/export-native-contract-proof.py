"""📜 Preserve current native contract envelopes and their complete recorded checks."""
import hashlib
import importlib.util
import json
import re
from datetime import datetime, timezone
from pathlib import Path
import sys

sys.dont_write_bytecode = True
sys.stdout.reconfigure(encoding="utf-8", errors="backslashreplace")
TICKET = Path(__file__).resolve().parent
SPEC = importlib.util.spec_from_file_location("native_contract_ledger", TICKET / "verify-native-ledger.py")
LEDGER = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(LEDGER)


def export(series, count):
    """🔐 Require terminal success and current source/runtime evidence before exporting."""
    directory = TICKET / "🗑️generated" / f"scenes-{series}-all-en-de"
    log = TICKET / "🗑️generated" / f"native-batch-{series}" / f"nc-{series}.log"
    paths = [directory / "result.json", directory / "checks.jsonl", log]
    raw = {path.name: path.read_bytes() for path in paths}
    result = json.loads(raw["result.json"])
    checks = [json.loads(line) for line in raw["checks.jsonl"].decode("utf-8").splitlines()]
    durations = re.findall(r"Ran (\d+) tests in ([\d.]+)s", raw[log.name].decode("utf-8"))
    if len(durations) != 1 or int(durations[0][0]) != count:
        raise RuntimeError("Native contract duration/count log is incomplete: " + series)
    envelope = LEDGER.accepted(result, "all-scenes") and result.get("tests") == count
    matching = bool(checks) and all(
        check.get("started_at") == result.get("started_at")
        and check.get("series") == series
        and check.get("provenance_scope") == "all-scenes"
        and check.get("outcome") == "success"
        and check.get("source_unchanged") is True
        and check.get("source_fingerprints") == result.get("source_fingerprints")
        and check.get("source_fingerprints_end") == result.get("source_fingerprints")
        and check.get("runtime_identity") == result.get("runtime_identity")
        for check in checks
    )
    stable = all(path.read_bytes() == raw[path.name] for path in paths)
    acceptance = {
        "exported_at": datetime.now(timezone.utc).isoformat(),
        "series": series,
        "expected_contract_tests": count,
        "recorded_contract_seconds": float(durations[0][1]),
        "declared_dependency_count": len(result.get("source_fingerprints", {})),
        "recorded_checks": len(checks),
        "terminal_envelope_accepted": envelope,
        "all_check_endpoints_match_current_terminal_source_and_runtime": matching,
        "input_files_unchanged_during_export": stable,
        "accepted": envelope and matching and stable,
    }
    text = [f"# Final Native {series.title()} Contract Proof\n",
            "This preserves the complete terminal presentation-contract envelope and every recorded check. "
            "Acceptance uses the native ledger's current semantic dependency and actual interpreter/package checks. "
            "Every check's source endpoints and runtime equal the accepted terminal envelope. "
            "The recorded configurations are preserved as recorded; this exporter does not claim its own configuration matches a native worker. "
            "This contract proof does not establish completion of the separately running native scene pairs.\n",
            "## Export Acceptance\n", "```json\n" + json.dumps(acceptance, ensure_ascii=False, indent=2) + "\n```\n"]
    for name, value in (("result.json", result), ("checks.jsonl", checks)):
        text.extend([f"## Complete {name}\n", "Raw SHA256: " + hashlib.sha256(raw[name]).hexdigest() + "\n",
                     "```json\n" + json.dumps(value, ensure_ascii=False, indent=2) + "\n```\n"])
    text.extend([f"## Complete {log.name}\n", "Raw SHA256: " + hashlib.sha256(raw[log.name]).hexdigest() + "\n",
                 "```text\n" + raw[log.name].decode("utf-8") + "\n```\n"])
    target = TICKET / f"final-native-{series}-contracts-proof.md"
    target.write_text("\n".join(text), encoding="utf-8")
    print("[DEBUG] " + json.dumps(acceptance))
    if not acceptance["accepted"]:
        raise RuntimeError("Native contract export is not accepted: " + series)


for series, count in (("grasshopper", 12), ("excel", 19)):
    export(series, count)
