# -*- coding: utf-8 -*-
"""S18 §14c (S20 relay, step 2 on top of s18-14c-guest-refusal-ack.py): a guest refusal reaches the shell WITH the guest's
own fault, and no refusal ends a turn early. The action-reply `AppFrame::Error` threw mid-turn exactly like the fault page
did (same parked-page poison), and both dropped the fault text — the shell only ever read `action-guest-refused`. Now the
publication decoder carries the fault as the shell displays it (`faultDisplayMessage`: `code: message`), the worker records
every refusal of the turn (fault pages and Error replies) while it answers the whole turn, refuses the action afterwards
with a typed `BrowserActorGuestRefusalV1`, and the rejected result's reason reads `action-guest-refused: <fault>` bounded
by `browserActorGuestRefusalReasonV1` to the result's 256-byte text rule (schema-first: fixture `guestRefusals`). Idempotent."""
import json
import pathlib

HANDOFF = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🎯️action-handoff")
MODULE = HANDOFF / "🟦️.ts"
PUBLICATION = HANDOFF / "📤️publication/🟦️.ts"
FIXTURE = HANDOFF / "🧫️fixtures/🔣️.json"
SCHEMA = HANDOFF / "🧬️schema/🔣️.json"
LAW = next((HANDOFF / "🧪️tests").glob("*/🟦️.ts"))
WORKER = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts")

LONG = "space.home.directory-projection: " + "Überlänge — " * 40
REFUSALS = [
    {"detail": None, "reason": "action-guest-refused"},
    {"detail": "extension.missing", "reason": "action-guest-refused: extension.missing"},
    {"detail": "stdio.xml.invalid-source: xml parse:\ntrailing content\tafter root", "reason": "action-guest-refused: stdio.xml.invalid-source: xml parse: trailing content after root"},
    {"detail": "   ", "reason": "action-guest-refused"},
]


def bounded(detail):
    text = " ".join("".join(" " if ord(c) < 32 or ord(c) == 127 else c for c in (detail or "")).split())
    reason = "action-guest-refused" if not text else f"action-guest-refused: {text}"
    while len(reason.encode("utf-8")) > 256:
        reason = reason[:-1]
    return reason.rstrip()


REFUSALS.append({"detail": LONG, "reason": bounded(LONG)})
for row in REFUSALS:
    assert bounded(row["detail"]) == row["reason"], row

EDITS = [
    (MODULE,
     "function text(value: unknown, path: string): string {",
     """/** 🚫️ The rejected-result reason of a guest refusal: `action-guest-refused`, then the guest's own fault as the shell displays
 * it (`code: message`) with control characters and runs of blanks folded to one space, cut on a character boundary to the
 * result's 256-byte text rule — so the refusal always parses and the human reads WHY (the fault used to be dropped).
 * Rows: `🧫️fixtures/🔣️.json` `guestRefusals`. */
export function browserActorGuestRefusalReasonV1(detail: string | null): string {
  const folded = (detail ?? "").replace(/[\\u0000-\\u001f\\u007f]/gu, " ").split(/\\s+/u).filter(Boolean).join(" ");
  let reason = folded.length === 0 ? "action-guest-refused" : `action-guest-refused: ${folded}`;
  const encoder = new TextEncoder();
  while (encoder.encode(reason).length > 256) reason = Array.from(reason).slice(0, -1).join("");
  return reason.trimEnd();
}

function text(value: unknown, path: string): string {"""),
    (PUBLICATION,
     'import { decodeAppFrame, decodeInvocationResultPacks, decodePackValue, encodeAppFrame, encodePackValue } from "../../../../../🟦️.ts";',
     'import { decodeAppFrame, decodeInvocationResultPacks, decodePackValue, encodeAppFrame, encodePackValue, faultDisplayMessage } from "../../../../../🟦️.ts";'),
    (PUBLICATION,
     'export type BrowserActorIntentPublicationV1 = { readonly kind: "emit" } | { readonly kind: "error"; readonly reason: string } | BrowserActorUnsolicitedPublicationV1;',
     'export type BrowserActorIntentPublicationV1 = { readonly kind: "emit" } | { readonly kind: "error"; readonly reason: string; readonly detail: string } | BrowserActorUnsolicitedPublicationV1;'),
    (PUBLICATION,
     'export type BrowserActorCommandPublicationV1 = { readonly kind: "invocation"; readonly projection: BrowserActorCommandMutationProjectionV1; readonly historyPatch: BrowserActorHistoryPatchBytesV1 } | { readonly kind: "error"; readonly reason: string } | BrowserActorUnsolicitedPublicationV1;',
     'export type BrowserActorCommandPublicationV1 = { readonly kind: "invocation"; readonly projection: BrowserActorCommandMutationProjectionV1; readonly historyPatch: BrowserActorHistoryPatchBytesV1 } | { readonly kind: "error"; readonly reason: string; readonly detail: string } | BrowserActorUnsolicitedPublicationV1;'),
    (PUBLICATION,
     '  if ("Error" in frame && frame.Error.in_reply_to === null) return { kind: "error", reason: "action-guest-refused" };',
     '  if ("Error" in frame && frame.Error.in_reply_to === null) return { kind: "error", reason: "action-guest-refused", detail: faultDisplayMessage(frame.Error.fault, decodePackValue) };'),
    (PUBLICATION,
     '  if ("Error" in frame && frame.Error.in_reply_to === actionSequence) return { kind: "error", reason: "action-guest-refused" };',
     '  if ("Error" in frame && frame.Error.in_reply_to === actionSequence) return { kind: "error", reason: "action-guest-refused", detail: faultDisplayMessage(frame.Error.fault, decodePackValue) };'),
    (WORKER,
     'type BrowserActorActionPublication = { readonly kind: "ui-intent" | "app-command"; readonly sequence: number; frames: number; refusals: number; readonly hostEffects: (readonly number[])[]; readonly historyPatches: (readonly number[])[] };',
     '''type BrowserActorActionPublication = { readonly kind: "ui-intent" | "app-command"; readonly sequence: number; frames: number; readonly refusals: string[]; readonly hostEffects: (readonly number[])[]; readonly historyPatches: (readonly number[])[] };

/** 🚫️ The guest refused the action (a typed-operation fault page or its `AppFrame::Error` reply) — raised only after the
 * whole turn was answered, so the actor stays open; `detail` is the guest's first fault as the shell displays it. */
class BrowserActorGuestRefusalV1 extends Error {
  constructor(readonly detail: string | null) {
    super("action-guest-refused");
  }
}'''),
    (WORKER,
     "frames: 0, refusals: 0, hostEffects: [], historyPatches: [] };",
     "frames: 0, refusals: [], hostEffects: [], historyPatches: [] };"),
    (WORKER,
     '        if (publication.refusals !== 0) throw new Error("action-guest-refused");',
     '        if (publication.refusals.length !== 0) throw new BrowserActorGuestRefusalV1(publication.refusals[0] ?? null);'),
    (WORKER,
     """      const explicitRefusal = error instanceof Error && error.message === "action-guest-refused";
      if (invoked && !explicitRefusal) this.close();
      const reason = error instanceof Error && /^(action-owner-mismatch|action-child-unavailable|action-guest-refused)$/u.test(error.message) ? error.message : invoked ? "action-state-unconfirmed" : "action-refused";""",
     """      const explicitRefusal = error instanceof BrowserActorGuestRefusalV1;
      if (invoked && !explicitRefusal) this.close();
      const reason = explicitRefusal ? browserActorGuestRefusalReasonV1(error.detail) : error instanceof Error && /^(action-owner-mismatch|action-child-unavailable)$/u.test(error.message) ? error.message : invoked ? "action-state-unconfirmed" : "action-refused";"""),
    (WORKER,
     "Promise<Readonly<{ receipts: readonly Uint8Array[]; mutations: number; publications: number; refusals: number; hostEffects:",
     "Promise<Readonly<{ receipts: readonly Uint8Array[]; mutations: number; publications: number; refusals: readonly string[]; hostEffects:"),
    (WORKER,
     "    let mutations = 0,\n      publications = 0,\n      refusals = 0,\n      artifactPages = 0,\n",
     "    const refusals: string[] = [];\n    let mutations = 0,\n      publications = 0,\n      artifactPages = 0,\n"),
    (WORKER,
     "          if (answer.refused) refusals += 1;\n",
     "          if (answer.refused) refusals.push(faultDisplayMessage(Array.from(page.payload), decodePackValue));\n"),
    (WORKER,
     "          if (publication.kind === \"error\") throw new Error(publication.reason);\n",
     """          if (publication.kind === "error") {
            refusals.push(publication.detail);
            continue;
          }
"""),
    (WORKER,
     "          publication.refusals += routed.refusals;\n",
     "          publication.refusals.push(...routed.refusals);\n"),
]


def insert_fixture_rows() -> None:
    raw = FIXTURE.read_text(encoding="utf-8")
    if '"guestRefusals"' in raw:
        return
    body = json.dumps(REFUSALS, ensure_ascii=False, indent=2).replace("\n", "\n  ")
    end = raw.rstrip().rfind("}")
    FIXTURE.write_text(raw[:end].rstrip() + ",\n  \"guestRefusals\": " + body + "\n}\n", encoding="utf-8")
    json.loads(FIXTURE.read_text(encoding="utf-8"))


def insert_schema_rows() -> None:
    raw = SCHEMA.read_text(encoding="utf-8")
    if '"guestRefusals"' in raw:
        return
    raw = raw.replace('        "commandViewState",\n        "publication"\n      ],', '        "commandViewState",\n        "publication",\n        "guestRefusals"\n      ],', 1)
    anchor = '      "properties": {\n        "request": {\n          "$ref": "#/definitions/request"\n        },'
    rows = ('      "properties": {\n        "guestRefusals": {\n          "type": "array",\n          "minItems": 1,\n          "items": {\n            "type": "object",\n'
            '            "additionalProperties": false,\n            "required": ["detail", "reason"],\n            "properties": {\n'
            '              "detail": { "type": ["string", "null"] },\n              "reason": { "type": "string", "minLength": 1, "maxLength": 256, "pattern": "^action-guest-refused(: .+)?$" }\n'
            '            }\n          }\n        },\n        "request": {\n          "$ref": "#/definitions/request"\n        },')
    assert raw.count(anchor) == 1
    raw = raw.replace(anchor, rows, 1)
    json.loads(raw)
    SCHEMA.write_text(raw, encoding="utf-8")


def main() -> None:
    texts: dict[pathlib.Path, str] = {}
    for path, old, new in EDITS:
        text = texts.get(path) or path.read_text(encoding="utf-8")
        if new not in text:
            assert text.count(old) == 1, (path.name, old[:90])
            text = text.replace(old, new)
        texts[path] = text
    worker = texts[WORKER]
    handoff = "import { BROWSER_ACTOR_ACTION_APP_CHANNEL_VERSION, BROWSER_ACTOR_ACTION_MUTATION_MAXIMUM, parseBrowserActorActionRequestV1,"
    if "BROWSER_ACTOR_ACTION_MUTATION_MAXIMUM, browserActorGuestRefusalReasonV1," not in worker:
        assert worker.count(handoff) == 1
        worker = worker.replace(handoff, "import { BROWSER_ACTOR_ACTION_APP_CHANNEL_VERSION, BROWSER_ACTOR_ACTION_MUTATION_MAXIMUM, browserActorGuestRefusalReasonV1, parseBrowserActorActionRequestV1,")
    if "\n  decodePackValue,\n  faultDisplayMessage,\n" not in worker:
        assert worker.count("\n  decodePackValue,\n") >= 1
        worker = worker.replace("\n  decodePackValue,\n", "\n  decodePackValue,\n  faultDisplayMessage,\n", 1)
    texts[WORKER] = worker
    module = texts.get(MODULE) or MODULE.read_text(encoding="utf-8")
    registration = "await registerTests1(import.meta.vitest, { BROWSER_ACTOR_ACTION_PACK_MAXIMUM_BYTES, browserActorActionOwnerMatchesV1, createBrowserActorAppCommandRequestV1,"
    if "browserActorActionOwnerMatchesV1, browserActorGuestRefusalReasonV1, createBrowserActorAppCommandRequestV1," not in module:
        assert module.count(registration) == 1
        module = module.replace(registration, "await registerTests1(import.meta.vitest, { BROWSER_ACTOR_ACTION_PACK_MAXIMUM_BYTES, browserActorActionOwnerMatchesV1, browserActorGuestRefusalReasonV1, createBrowserActorAppCommandRequestV1,")
    texts[MODULE] = module
    law = LAW.read_text(encoding="utf-8")
    if "guestRefusals" not in law:
        law = law.replace(
            '    expect(parseBrowserActorActionResultV1(fixture.rejected).reason).toBe("action-refused");\n',
            '    expect(parseBrowserActorActionResultV1(fixture.rejected).reason).toBe("action-refused");\n'
            '    for (const row of fixture.guestRefusals) {\n'
            '      const reason = browserActorGuestRefusalReasonV1(row.detail);\n'
            '      expect(reason, JSON.stringify(row.detail)).toBe(row.reason);\n'
            '      expect(parseBrowserActorActionResultV1({ ...fixture.rejected, reason }).reason).toBe(reason);\n'
            '    }\n', 1)
        law = law.replace("const { BROWSER_ACTOR_ACTION_PACK_MAXIMUM_BYTES, browserActorActionOwnerMatchesV1,", "const { BROWSER_ACTOR_ACTION_PACK_MAXIMUM_BYTES, browserActorActionOwnerMatchesV1, browserActorGuestRefusalReasonV1,", 1)
        assert "browserActorGuestRefusalReasonV1(row.detail)" in law and "browserActorGuestRefusalReasonV1," in law
        texts[LAW] = law
    for path, text in texts.items():
        path.write_text(text, encoding="utf-8")
    insert_fixture_rows()
    insert_schema_rows()
    print("ok")


main()
