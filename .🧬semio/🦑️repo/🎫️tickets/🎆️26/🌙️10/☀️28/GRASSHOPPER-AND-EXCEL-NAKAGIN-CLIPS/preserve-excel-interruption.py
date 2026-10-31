"""🧾 Preserve incomplete Excel evidence only after first-party process absence checks."""
from datetime import datetime, timezone
import hashlib
import json
from pathlib import Path
import subprocess
import sys

sys.stdout.reconfigure(encoding="utf-8", errors="backslashreplace")
TICKET = Path(__file__).resolve().parent
ROOT = next(path for path in TICKET.parents if (path / "🧰️framework").is_dir())
GENERATED = TICKET / "🗑️generated"
HISTORY = GENERATED / "historical-xl-0850"
old_pids = {55932, 49484, 10552, 11852, 37448, 44708, 45884, 53040, 10524, 28440, 34052, 37440, 42404, 56796}
doctor = subprocess.run(["repo", "doctor", "--only=processes", "--facts"], cwd=ROOT, capture_output=True, encoding="utf-8", check=True)
facts = json.loads(doctor.stdout)
processes = facts["processes"]
survivors = [row for row in processes if row["pid"] in old_pids or any(label in row.get("command", "") for label in ("final-native-excel", "nc-excel", "ns-excel-"))]
if survivors:
    raise RuntimeError("Original Excel process identity remains present: " + json.dumps(survivors))
if HISTORY.exists():
    raise RuntimeError("Interruption history already exists")
HISTORY.mkdir()
(HISTORY / "doctor-facts.json").write_text(doctor.stdout, encoding="utf-8")
scenes = json.loads((TICKET / "native-pairs.json").read_text(encoding="utf-8"))["series"]["excel"]["scenes"]
names = ["final-native-excel", "native-batch-excel", "nc-excel", "scenes-excel-all-en-de"]
names += [prefix + scene for scene in scenes for prefix in ("native-batch-excel-", "ns-excel-")]
names += ["scenes-excel-" + scene + "-" + language for scene in scenes for language in ("en", "de")]
preserved = []
for name in names:
    source = (GENERATED / name).resolve()
    target = (HISTORY / name).resolve()
    if not source.is_relative_to(GENERATED.resolve()) or not target.is_relative_to(HISTORY.resolve()):
        raise ValueError("Archive path escaped the ticket generated directory")
    if source.exists():
        source.rename(target)
        preserved.append(name)
log = HISTORY / "native-batch-excel" / "ns-excel-Beat1_Anatomy.log"
contract = HISTORY / "scenes-excel-all-en-de" / "result.json"
payload = {
    "observed_at": datetime.now(timezone.utc).isoformat(),
    "old_session": 31115,
    "old_capacity_root": 55932,
    "old_grant": "81JvA8QUI2Aq3x560xLXug",
    "known_owned_pids_absent": sorted(old_pids),
    "known_owned_command_labels_absent": ["final-native-excel", "nc-excel", "ns-excel-"],
    "doctor_facts_at_ms": facts["atMs"],
    "survivors": survivors,
    "preserved_directories": preserved,
    "worker_log_last_modified": datetime.fromtimestamp(log.stat().st_mtime, timezone.utc).isoformat(),
    "worker_log_sha256": hashlib.sha256(log.read_bytes()).hexdigest(),
    "worker_log_tail": log.read_text(encoding="utf-8")[-500:],
    "contract_result_sha256": hashlib.sha256(contract.read_bytes()).hexdigest(),
    "native_pair_result_exists": (HISTORY / "scenes-excel-Beat1_Anatomy-en" / "result.json").exists(),
    "batch_result_exists": (HISTORY / "native-batch-excel" / "result.json").exists(),
}
text = "# Excel Native Interruption Postmortem\n\nThe original owned exec session 31115 became unavailable after this agent ended its turn at root's request to free an audit slot. The original process tree and command labels are absent in the subsequent first-party doctor facts. The last worker log is ordinary progress at 5001 seconds with no terminal result; the run is an incomplete non-pass and establishes no native pair success. Agent-turn completion and disappearance are temporally correlated, but no termination journal establishes a definitive mechanism. No cancellation, signal, pause or private-cache mutation was issued by this worker.\n\nRoot authorized replacement only after absence verification and preservation. The original contracts remain a terminal 19-test pass, with complete permanent evidence in final-native-excel-contracts-proof.md. All fixed-path Excel output directories were preserved under generated/historical-xl-0850 before replacement, including the complete contract result/checks and unfinished worker outputs. No other owner's files were moved.\n\nThe corrective coordination rule is to retain the owning agent turn and original exec handle through the entire terminal batch, without ending the agent turn to free an audit slot. A fresh 24 GB official reservation is justified by the original approximately 19 GB observed native peak and must be admitted normally.\n\n```json\n" + json.dumps(payload, ensure_ascii=False, indent=2) + "\n```\n\n## Complete First-Party Process Facts At Absence Verification\n\n```json\n" + json.dumps(facts, ensure_ascii=False, indent=2) + "\n```\n"
(TICKET / "excel-native-interruption-postmortem.md").write_text(text, encoding="utf-8")
print("[DEBUG] Excel interruption preserved; owned identities absent; directories " + str(len(preserved)))
