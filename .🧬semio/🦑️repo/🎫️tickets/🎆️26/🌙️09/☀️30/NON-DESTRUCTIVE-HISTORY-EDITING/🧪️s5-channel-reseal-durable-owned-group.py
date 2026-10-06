#!/usr/bin/env python3
"""🗄️ S5-CHANNEL: re-seals the durable-owned-group corpus after wave B (design §20.7 / §22.4).

`🏪️store/🧩️composition/🗄️durable-group/🧫️fixtures/🔣️.json` pins, per member, the Store-owned unbound outcome pack
(`unboundOutcomePackHex` + its SHA-256) and, over the three hashes, the decision's canonical unsigned JSON and its SHA-256.
The pack embeds the member edit's canonical JSON, which lost `description` with wave B, so the law
`durable_store_prepared_outcome_derives_and_verifies_exact_unbound_bytes` now derives other bytes than the corpus holds.
The pack is Store-owned (compressed) bytes no second implementation produces, so the only source of the new bytes is the
law itself: this script reads one failing run of that law (`left` = derived pack, `right` = the corpus pack it replaces),
re-derives the member's hash, the unsigned JSON and the decision hash exactly as the TypeScript twin
(`🏪️store/🧪️tests/🗄️durable-owned-group/🟦️.ts`) composes them, and rewrites those values in place (exact occurrence
counts, formatting untouched). Before anything changes it proves that its own composition reproduces the corpus as it is.
One member per failing run (the law stops at the first mismatch): run law → `--from-log … --write` → repeat until green.
Usage: python3 🧪️s5-channel-reseal-durable-owned-group.py --from-log <cargo test output> [--write]
"""
from __future__ import annotations

import hashlib
import json
import pathlib
import re
import sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio")
FIXTURE = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🧫️fixtures/🔣️.json"
LAW = "durable_store_prepared_outcome_derives_and_verifies_exact_unbound_bytes"
MAGIC = "8953454d0d0a1a0a"


def compact(value) -> str:
    return json.dumps(value, ensure_ascii=False, separators=(",", ":"))


def unsigned_json(fixture: dict) -> str:
    def member(role: str) -> dict:
        row = fixture["members"][role]
        return {
            "role": row["role"],
            "reference": row["reference"],
            "owner": row["owner"],
            "expectedGeneration": row["expectedGeneration"],
            "expectedRevision": list(bytes.fromhex(row["expectedRevisionHex"])),
            "recoverySchema": row["recoverySchema"],
            "unboundOutcomeSha256": row["unboundOutcomeSha256"],
        }
    return compact({"schema": fixture["schemas"]["decision"], "anchor": fixture["anchor"], "parent": member("parent"), "drawing": member("drawing"), "value": member("value")})


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def replace_exact(text: str, old: str, new: str, count: int, label: str) -> str:
    found = text.count(old)
    if found != count:
        raise SystemExit(f"[DEBUG] {label}: found {found}x, expected {count}x — nothing written")
    return text.replace(old, new)


def main() -> None:
    if "--from-log" not in sys.argv or sys.argv.index("--from-log") + 1 >= len(sys.argv):
        raise SystemExit("usage: --from-log <cargo test output> [--write]")
    log = pathlib.Path(sys.argv[sys.argv.index("--from-log") + 1]).read_text()
    text = FIXTURE.read_text()
    fixture = json.loads(text)
    before_json = unsigned_json(fixture)
    if before_json != fixture["expected"]["unsignedJson"] or sha256(before_json.encode()) != fixture["expected"]["decisionSha256"]:
        raise SystemExit("[DEBUG] this script's composition does not reproduce the corpus as it is — nothing written")
    for role, row in fixture["members"].items():
        if sha256(bytes.fromhex(row["unboundOutcomePackHex"])) != row["unboundOutcomeSha256"]:
            raise SystemExit(f"[DEBUG] {role}: the corpus pack does not hash to its own SHA-256 — nothing written")
    panic = re.search(r"thread '[^']*" + LAW + r"'[^\n]*panicked at[^\n]*\n[^\n]*assertion `left == right` failed\n\s*left: \"([0-9a-f]+)\"\n\s*right: String\(\"([0-9a-f]+)\"\)", log)
    if panic is None:
        raise SystemExit(f"[DEBUG] the log holds no pack mismatch of {LAW} — nothing to re-seal")
    derived, replaced = panic.group(1), panic.group(2)
    roles = [role for role, row in fixture["members"].items() if row["unboundOutcomePackHex"] == replaced]
    if len(roles) != 1 or not derived.startswith(MAGIC) or derived == replaced:
        raise SystemExit(f"[DEBUG] the mismatch names {len(roles)} corpus members or is no Store pack — nothing written")
    role = roles[0]
    old_sha, new_sha = fixture["members"][role]["unboundOutcomeSha256"], sha256(bytes.fromhex(derived))
    text = replace_exact(text, replaced, derived, 1, f"{role} pack")
    text = replace_exact(text, old_sha, new_sha, 2, f"{role} pack hash (member row + unsigned JSON)")
    resealed = json.loads(text)
    after_json = unsigned_json(resealed)
    if resealed["expected"]["unsignedJson"] != after_json:
        raise SystemExit("[DEBUG] the unsigned JSON did not follow the member hash — nothing written")
    old_decision, new_decision = fixture["expected"]["decisionSha256"], sha256(after_json.encode())
    text = replace_exact(text, old_decision, new_decision, 1, "decision hash")
    json.loads(text)
    print(f"[DEBUG] {role}: pack {len(replaced) // 2} → {len(derived) // 2} bytes, sha {old_sha[:12]}… → {new_sha[:12]}…; decision {old_decision[:12]}… → {new_decision[:12]}…")
    if "--write" in sys.argv:
        FIXTURE.write_text(text)
        print("[DEBUG] corpus written")
    else:
        print("[DEBUG] dry run: nothing written")


if __name__ == "__main__":
    main()
