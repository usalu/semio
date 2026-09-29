"""🚧️ S18 §14c p33: an action the document actor refuses before it invokes the guest carries WHY (`action-refused: <admission
fault>`) instead of a bare `action-refused` — helper + fixture rows + schema + law + worker, applied as one set.
usage: python3 s18-14c-admission-refusal.py [--dry-run]"""
import json
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules")
HANDOFF = ROOT / "🔌️plugin/🌐️browser-bundle/🎯️action-handoff"
LAW = HANDOFF / "🧪️tests/🧪️browser-actor-action-handoff-validates-the-neutral-schema-and-exact-owne/🟦️.ts"
WORKER = ROOT / "🏪️store/👷️worker/🟦️.ts"
DRY = "--dry-run" in sys.argv


def swap(text: str, old: str, new: str, path: Path) -> str:
    if text.count(old) != 1:
        raise SystemExit(f"anchor count {text.count(old)} in {path}: {old[:80]!r}")
    return text.replace(old, new)


edits: dict[Path, str] = {}

source = (HANDOFF / "🟦️.ts").read_text(encoding="utf-8")
source = swap(source, '''/** 🚫️ The rejected-result reason of a guest refusal: `action-guest-refused`, then the guest's own fault as the shell displays
 * it (`code: message`) with control characters and runs of blanks folded to one space, cut on a character boundary to the
 * result's 256-byte text rule — so the refusal always parses and the human reads WHY (the fault used to be dropped).
 * Rows: `🧫️fixtures/🔣️.json` `guestRefusals`. */
export function browserActorGuestRefusalReasonV1(detail: string | null): string {
  const folded = (detail ?? "").replace(/[\\u0000-\\u001f\\u007f]/gu, " ").split(/\\s+/u).filter(Boolean).join(" ");
  let reason = folded.length === 0 ? "action-guest-refused" : `action-guest-refused: ${folded}`;
  const encoder = new TextEncoder();
  while (encoder.encode(reason).length > 256) reason = Array.from(reason).slice(0, -1).join("");
  return reason.trimEnd();
}''', '''/** 🧺️ `prefix`, then `detail` with control characters and runs of blanks folded to one space, cut on a character boundary to
 * the result's 256-byte text rule — so a rejected result always parses and the human reads WHY. */
function foldedRefusalReason(prefix: string, detail: string | null): string {
  const folded = (detail ?? "").replace(/[\\u0000-\\u001f\\u007f]/gu, " ").split(/\\s+/u).filter(Boolean).join(" ");
  let reason = folded.length === 0 ? prefix : `${prefix}: ${folded}`;
  const encoder = new TextEncoder();
  while (encoder.encode(reason).length > 256) reason = Array.from(reason).slice(0, -1).join("");
  return reason.trimEnd();
}

/** 🚫️ The rejected-result reason of a guest refusal: `action-guest-refused`, then the guest's own fault as the shell displays
 * it (`code: message`), folded and cut by {@link foldedRefusalReason} (the fault used to be dropped).
 * Rows: `🧫️fixtures/🔣️.json` `guestRefusals`. */
export function browserActorGuestRefusalReasonV1(detail: string | null): string {
  return foldedRefusalReason("action-guest-refused", detail);
}

/** 🚧️ The rejected-result reason of an action the document actor refused BEFORE it invoked the guest (owner, canonical form or
 * UI-quiescence admission): `action-refused`, then the admission fault, folded and cut by {@link foldedRefusalReason} (a
 * bare `action-refused` hid which admission rule refused every 3d.puzzle input on the p33 hub sweep).
 * Rows: `🧫️fixtures/🔣️.json` `admissionRefusals`. */
export function browserActorAdmissionRefusalReasonV1(detail: string | null): string {
  return foldedRefusalReason("action-refused", detail);
}''', HANDOFF / "🟦️.ts")
source = swap(source, '''  const { createBrowserActorUiIntentRequestV1 } = await import("./🧭️intent/🟦️.ts");
  await registerTests1(import.meta.vitest, { BROWSER_ACTOR_ACTION_PACK_MAXIMUM_BYTES, browserActorActionOwnerMatchesV1, browserActorGuestRefusalReasonV1,''', '''  const { createBrowserActorUiIntentRequestV1 } = await import("./🧭️intent/🟦️.ts");
  const { browserActorActionRefusalReasonV1 } = await import("./📮️requests/🟦️.ts");
  await registerTests1(import.meta.vitest, { BROWSER_ACTOR_ACTION_PACK_MAXIMUM_BYTES, browserActorActionOwnerMatchesV1, browserActorActionRefusalReasonV1, browserActorAdmissionRefusalReasonV1, browserActorGuestRefusalReasonV1,''', HANDOFF / "🟦️.ts")
edits[HANDOFF / "🟦️.ts"] = source

law = LAW.read_text(encoding="utf-8")
law = swap(law, "const { BROWSER_ACTOR_ACTION_PACK_MAXIMUM_BYTES, browserActorActionOwnerMatchesV1, browserActorGuestRefusalReasonV1,", "const { BROWSER_ACTOR_ACTION_PACK_MAXIMUM_BYTES, browserActorActionOwnerMatchesV1, browserActorActionRefusalReasonV1, browserActorAdmissionRefusalReasonV1, browserActorGuestRefusalReasonV1,", LAW)
law = swap(law, '''      expect(parseBrowserActorActionResultV1({ ...fixture.rejected, reason }).reason).toBe(reason);
    }
''', '''      expect(parseBrowserActorActionResultV1({ ...fixture.rejected, reason }).reason).toBe(reason);
      expect(browserActorActionRefusalReasonV1(new Error(reason)), reason).toBeNull();
    }
    for (const row of fixture.admissionRefusals) {
      const reason = browserActorAdmissionRefusalReasonV1(row.detail);
      expect(reason, JSON.stringify(row.detail)).toBe(row.reason);
      expect(parseBrowserActorActionResultV1({ ...fixture.rejected, reason }).reason).toBe(reason);
      expect(browserActorActionRefusalReasonV1(new Error(reason)), reason).toBe("dispatch-failed");
    }
''', LAW)
edits[LAW] = law

fixture_path = HANDOFF / "🧫️fixtures/🔣️.json"
fixture_text = fixture_path.read_text(encoding="utf-8")
if "admissionRefusals" in fixture_text:
    raise SystemExit("fixture already carries admissionRefusals")
long_prefix = "action-refused: queue full? owner retired — "
rows = [
    {"detail": None, "reason": "action-refused"},
    {"detail": "document browser actor: command owner mismatch", "reason": "action-refused: document browser actor: command owner mismatch"},
    {"detail": "document browser actor: noncanonical\tintent\n", "reason": "action-refused: document browser actor: noncanonical intent"},
    {"detail": "queue full? owner retired — " + "x" * 300, "reason": long_prefix + "x" * (256 - len(long_prefix.encode("utf-8")))},
]
block = ",\n".join("    {\n" + f'      "detail": {json.dumps(row["detail"], ensure_ascii=False)},\n      "reason": {json.dumps(row["reason"], ensure_ascii=False)}\n' + "    }" for row in rows)
tail = "\n  ]\n}\n"
if not fixture_text.endswith(tail):
    raise SystemExit("fixture tail anchor moved")
edits[fixture_path] = fixture_text[: -len(tail)] + "\n  ],\n  \"admissionRefusals\": [\n" + block + tail

schema_path = HANDOFF / "🧬️schema/🔣️.json"
schema_text = schema_path.read_text(encoding="utf-8")
schema_text = swap(schema_text, '''        "guestRefusals"
      ],''', '''        "guestRefusals",
        "admissionRefusals"
      ],''', schema_path)
schema_text = swap(schema_text, '''              "reason": { "type": "string", "minLength": 1, "maxLength": 256, "pattern": "^action-guest-refused(: .+)?$" }
            }
          }
        },
''', '''              "reason": { "type": "string", "minLength": 1, "maxLength": 256, "pattern": "^action-guest-refused(: .+)?$" }
            }
          }
        },
        "admissionRefusals": {
          "type": "array",
          "minItems": 1,
          "items": {
            "type": "object",
            "additionalProperties": false,
            "required": ["detail", "reason"],
            "properties": {
              "detail": { "type": ["string", "null"] },
              "reason": { "type": "string", "minLength": 1, "maxLength": 256, "pattern": "^action-refused(: .+)?$" }
            }
          }
        },
''', schema_path)
json.loads(schema_text)
json.loads(edits[fixture_path])
edits[schema_path] = schema_text

requests_path = HANDOFF / "📮️requests/🟦️.ts"
requests = requests_path.read_text(encoding="utf-8")
requests = swap(requests, '''/** 🧭️ Maps a mailbox or worker rejection onto the refusal vocabulary; `null` for errors this transport did not author. */
export function browserActorActionRefusalReasonV1(error: unknown): BrowserActorActionRefusalReasonV1 | null {
  const message = error instanceof Error ? error.message : typeof error === "string" ? error : null;
  if (message === null) return null;
''', '''/** 🧭️ Maps a mailbox or worker rejection onto the refusal vocabulary; `null` for errors this transport did not author. A
 * refusal carrying its WHY is read by its prefix first — the actor's own admission refusal (`action-refused: …`) is
 * `dispatch-failed`, a guest refusal (`action-guest-refused: …`) is not this transport's (`null`) — so words inside the
 * carried fault ("queue full", "owner retired") never re-classify it. */
export function browserActorActionRefusalReasonV1(error: unknown): BrowserActorActionRefusalReasonV1 | null {
  const message = error instanceof Error ? error.message : typeof error === "string" ? error : null;
  if (message === null || /^action-guest-refused(: |$)/u.test(message)) return null;
  if (/^action-refused(: |$)/u.test(message)) return "dispatch-failed";
''', requests_path)
edits[requests_path] = requests

worker = WORKER.read_text(encoding="utf-8")
worker = swap(worker, "import { BROWSER_ACTOR_ACTION_APP_CHANNEL_VERSION, BROWSER_ACTOR_ACTION_MUTATION_MAXIMUM, browserActorGuestRefusalReasonV1,", "import { BROWSER_ACTOR_ACTION_APP_CHANNEL_VERSION, BROWSER_ACTOR_ACTION_MUTATION_MAXIMUM, browserActorAdmissionRefusalReasonV1, browserActorGuestRefusalReasonV1,", WORKER)
worker = swap(worker, "   * queued behind itself. A patch deadline rejects through here (the action falls to `action-refused`). */", "   * queued behind itself. A patch deadline rejects through here (the action falls to `action-refused: <why>`). */", WORKER)
worker = swap(worker, ': invoked ? "action-state-unconfirmed" : "action-refused";', ': invoked ? "action-state-unconfirmed" : browserActorAdmissionRefusalReasonV1(error instanceof Error ? error.message : String(error));', WORKER)
edits[WORKER] = worker

for path, text in edits.items():
    if DRY:
        print("dry", path.name, len(text))
    else:
        path.write_text(text, encoding="utf-8")
        print("ok", path)
