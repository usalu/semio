#!/usr/bin/env python3
"""⚙️ Splits the energy engine's `🧪️sim` (5,130 lines) into four crates along its own concerns (lane P1, slice S1d).

- `🧪️sim/🪜️retirement` — the close-turn vocabulary (grant cover, owner turn, rung/idle/done) and the bounded close
  ladders of every engine owner (results, time series, meters, summaries, sizing, model, config), `EnergyModelCloseCursor`.
- `🧪️sim/📡️wire` — the retained wire (packet, lease, queue, header, publication channels), the job stage/quality/preview
  vocabulary and the numerical admission census with its codes.
- `🧪️sim` — the numerical job authority, `EnergyJob`, its interactive-job protocol and the batch `Engine`.
- `🧪️sim/♻️restore` — checkpoint restore (`EnergyRestoreJob`, `EnergyCheckpointRejected`), above the job it rebuilds.
Moved items become `pub`; the unit tests split by the crate their subject lives in.
"""
import os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
exec(open(os.path.join(HERE, "⚙️sim-items.py"), encoding="utf-8").read())
SIM = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim"
RET = {"ENERGY_RUNG_DEMAND", "energy_grant_covers", "energy_guarded", "energy_turn", "energy_rung", "energy_one", "energy_idle", "energy_done",
       "close_string_vector_step", "close_results_step", "close_time_series_step", "close_meter_table_step", "close_summaries_step", "close_summary_rows_step",
       "close_sizing_step", "close_sizing_rows_step", "close_string_step", "summaries_are_terminal_empty", "close_model_step", "EnergyModelCloseCursor",
       "close_config_step", "model_is_terminal_empty", "config_is_terminal_empty", "energy_self_funded"}
WIRE = {"ENERGY_WIRE_MAGIC", "ENERGY_WIRE_VERSION", "ENERGY_WIRE_QUEUE_SLOTS", "ENERGY_WIRE_LEASE_SLOTS", "ENERGY_WIRE_HEADER_BYTES", "ENERGY_CHECKPOINT_BYTES",
        "energy_spend", "EnergyWireKind", "EnergyWireIdentity", "EnergyWireRejection", "EnergyWirePacket", "EnergyWireLease", "EnergyWireLeaseRecoverySlot",
        "ENERGY_WIRE_LEASE_RECOVERY", "EnergyWireQueue", "EnergyWireInFlight", "EnergyPublicationChannels", "EnergyWireBuild", "energy_wire_header",
        "EnergyQualityTier", "EnergyJobStage", "EnergyJobPreview", "EnergyJobCursor", "EnergyNumericalCensus", "EnergyNumericalDimension", "EnergyNumericalBounds",
        "observed_weather_records", "weather_copy_target", "checked_sum", "observed_identifier_bytes", "observed_model_bytes", "observed_vector_bytes",
        "energy_stage_from_code", "energy_tier_from_code", "decode_preview_packet", "fuel_code", "end_use_code", "unit_code", "close_retained_packet"}
REST = {"EnergyCheckpointRejected", "EnergyCheckpointRejectionReason", "EnergyRestoreJob", "EnergyRestoreAbandoned", "EnergyRestoreAbandonmentSlot",
        "ENERGY_RESTORE_ABANDONMENT_REGISTRY"}
ALIAS = {"retirement": "semio_s_plugin_energy_engine_sim_retirement", "wire": "semio_s_plugin_energy_engine_sim_wire", "sim": "semio_s_plugin_energy_engine_sim"}
lib_p = SIM + "/🦀️.rs"; test_p = SIM + "/🧪️tests/🔬️unit/🦀️.rs"
raw = open(lib_p, encoding="utf-8").read()
if "extern crate semio_s_plugin_energy_engine_sim_wire" in raw: sys.exit("already applied")
its = items(raw)
def target(name): return "retirement" if name in RET else "wire" if name in WIRE else "restore" if name in REST else "sim"
uses = "".join(raw[a:b] + "\n" for n, k, a, b in its if k == "use")
externs = [(n, raw[a:b]) for n, k, a, b in its if k == "extern crate"]
def publish(text):
    return re.sub(r"(?m)^((?:\s*(?:///[^\n]*|#\[[^\n]*\])\n)*)(?:pub(?:\([^)]*\))?\s+)?(?=(?:const fn|fn|struct|enum|const|static|type|trait)\b)", lambda m: m.group(1) + "pub ", text, count=1)
groups = {"retirement": [], "wire": [], "restore": [], "sim": []}
for n, k, a, b in its:
    if k in ("use", "extern crate"): continue
    t = target(n); text = raw[a:b]
    groups[t].append(text if t == "sim" or k == "impl" else publish(text))
names = {t: {n for n, k, a, b in its if k not in ("use", "extern crate", "impl") and target(n) == t} for t in groups}
def referenced(text, pool): return sorted(x for x in pool if re.search(r"(?<![A-Za-z0-9_])" + re.escape(x) + r"(?![A-Za-z0-9_])", strip(text)))
def header(doc, deps, body):
    ext = "".join(line + "\n" for n, line in externs if "#[cfg(test)]" not in line and re.search(r"\bas (\w+);", line) and re.search(r"\bcrate::" + re.search(r"\bas (\w+);", line).group(1) + r"::", body))
    test_ext = "".join(line + "\n" for n, line in externs if "#[cfg(test)]" in line)
    sib = "".join(f"extern crate {ALIAS[d]} as {d};\n" for d in deps)
    imports = "".join(f"use crate::{d}::{{{', '.join(referenced(body, names[d]))}}};\n" for d in deps if referenced(body, names[d]))
    return doc + "\n" + ext + test_ext + sib + "\n" + imports + uses
DOC = {"retirement": "//! 🪜️ Energy engine retirement: the close-turn vocabulary and the bounded close ladders of every engine owner a\n//! numerical job retains (results, time series, meters, summaries, sizing, model, config).\n",
       "wire": "//! 📡️ Energy engine retained wire: checkpoint, preview and commit packets with their lease and lossless queue, the job\n//! stage, quality and preview vocabulary, and the numerical admission census they carry.\n",
       "restore": "//! ♻️ Energy engine checkpoint restore: replays one retained checkpoint packet into a fresh numerical job.\n"}
DEPS = {"retirement": [], "wire": ["retirement"], "sim": ["retirement", "wire"], "restore": ["retirement", "wire", "sim"]}
DIR = {"retirement": SIM + "/🪜️retirement", "wire": SIM + "/📡️wire", "restore": SIM + "/♻️restore"}
# tests
traw = open(test_p, encoding="utf-8").read(); tits = items(traw)
upper_markers = {"EnergyJob", "Engine", "EnergyJobAuthority", "EnergyAdmissionRejected", "close_owned_job", "seen_step", "drive_energy_job", "drive_energy_job_with_fuel", "thermostatted_single_zone"}
tests = {"restore": [], "sim": [], "wire": []}
helpers = {n: traw[a:b] for n, k, a, b in tits if k != "fn" or not n.startswith(("p7c", "engine_", "ashrae", "invalid", "energy_", "full_", "hvac_", "run_period", "adversarial", "per_surface", "warmup"))}
for n, k, a, b in tits:
    if k == "use" or n in helpers: continue
    words = set(re.findall(r"[A-Za-z_][A-Za-z0-9_]*", strip(traw[a:b])))
    tests["restore" if words & REST else "sim" if words & upper_markers else "wire"].append(traw[a:b])
def test_file(t):
    body = "\n\n".join(tests[t])
    need, frontier = set(), body
    while True:
        new = {h for h in helpers if h not in need and re.search(r"(?<![A-Za-z0-9_])" + re.escape(h) + r"(?![A-Za-z0-9_])", strip(frontier))}
        if not new: break
        need |= new; frontier = "\n".join(helpers[h] for h in new)
    head = "".join(traw[a:b] + "\n" for n, k, a, b in tits if k == "use")
    return head + "\n" + "".join(helpers[h] + "\n\n" for h in sorted(need, key=lambda h: traw.index(helpers[h]))) + body + "\n"
for t in ("retirement", "wire", "restore"):
    body = "\n\n".join(groups[t])
    d = DIR[t]; os.makedirs(d + "/📦️packages/🦀️rust", exist_ok=True)
    text = header(DOC[t], DEPS[t], body) + "\n" + body + "\n"
    if t in tests and tests[t]:
        text += '\n#[cfg(test)]\n#[path = "🧪️tests/🔬️unit/🦀️.rs"]\nmod tests;\n'
        os.makedirs(d + "/🧪️tests/🔬️unit", exist_ok=True)
        open(d + "/🧪️tests/🔬️unit/🦀️.rs", "w", encoding="utf-8").write(test_file(t))
    open(d + "/🦀️.rs", "w", encoding="utf-8").write(text)
# upper sim: keep everything that is not moved, in order, plus aliases and imports of the moved names it uses
first_item = min(a for n, k, a, b in its if k not in ("use", "extern crate"))
kept = []
for n, k, a, b in its:
    if k in ("use", "extern crate") or target(n) != "sim": continue
    kept.append(raw[a:b])
body = "\n\n".join(kept)
doc = raw[:raw.index("extern crate")]
sib = "".join(f"extern crate {ALIAS[d]} as {d};\n" for d in DEPS["sim"])
imports = "".join(f"use crate::{d}::{{{', '.join(referenced(body + chr(10) + test_file('sim'), names[d]))}}};\n" for d in DEPS["sim"])
ext = "".join(line + "\n" for n, line in externs)
open(lib_p, "w", encoding="utf-8").write(doc + ext + sib + "\n" + imports + uses + "\n" + body + "\n")
open(test_p, "w", encoding="utf-8").write(test_file("sim"))
print({t: len(g) for t, g in groups.items()}, {t: len(v) for t, v in tests.items()})
