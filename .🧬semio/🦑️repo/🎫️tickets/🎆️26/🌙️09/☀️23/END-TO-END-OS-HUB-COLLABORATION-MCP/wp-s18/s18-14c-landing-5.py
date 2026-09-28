# -*- coding: utf-8 -*-
"""📝️ S18 §14c: landing rows for the /hub beacon, the hub-program board factory, and the guest-refusal cascade (idempotent)."""
import pathlib

LANDING = pathlib.Path("/Users/ueli/Documents/semio/.tmp-ticket/📓️landing.md")
ROWS = [
    "| S18 | 14c — readiness beacon on `/hub` (the hub workspace overlay) read `not-found` while the page rendered the hub: one predicate `notFoundPath` for page and beacon (`SHELL_HUB_ROUTE` excluded), memo deps follow: `💻️os/…/🧱️elements/🏛️ShellHost/🟦️.tsx`; codemod `wp-s18/s18-14c-hub-beacon.py` | TS only (host) | tsc 0 (`s18-14c-tsc-beacon-1.txt`); live: `/` ready, `/hub` ready + workspace shown, `/nowhere` not-found, 0 pageerrors (`s18-14c-beacon-{1..4}.txt`) | 28 23:0x |",
    "| S18 | 14c hub leg — a hub document's program (`<plugin>@<bundle sha256>`) joins its plugin's app-surface session factory (`resolveAppSurfaceSessionFactory` via `parseHubProgramIdV1`): hub-opened 2d puzzles threw \"The current app has no registered board session factory\": `💻️os/…/🧱️elements/🪪️WasmSessionLoader/🟦️.tsx`, fixture scopes `✏️s/🔌️plugins/🧩️puzzle/…/✏️editor/🌉️wasm/🧫️fixtures/🔣️session-factory.json` (+ hub program id rows); temporary `[DEBUG] s18 board` probe removed in the same pass; codemod `wp-s18/s18-14c-board-factory.py` | TS/JSON only (host + test fixture) | tsc 0 (`s18-14c-tsc-board-1.txt`), boot green; live 7800: `[DEBUG]` showed focused program `puzzle@fbe76999…` vs registrations `puzzle/…` (`s18-14c-board-2.txt`); after: hub sweep 2d.puzzle PASS [0,1,0,1] 0 faults, 2d.block PASS (`sweep-s18-14c-hub-en-puzzle.txt`); engine-contract laws queued (native lane) | 28 23:1x |",
    "| S18 | 14c (S20 relay) — one guest refusal no longer poisons a hub document: the store worker's browser actor answers EVERY typed-operation page of an action turn (a FAULT page too; `typedOperationPageAnswerV1` in `🧰️framework/🔨️modules/🎭️actor/🖼️wire-turn/🟦️.ts` + law over every declared lane in `🎭️actor/🧪️tests/🗞️typed-operation-page`), treats an `AppFrame::Error` reply the same way, drives the turn to its end and only then refuses with a typed `BrowserActorGuestRefusalV1` (actor stays open); the rejected reason carries the guest fault (`browserActorGuestRefusalReasonV1`, 256-byte bound; `🎯️action-handoff` fixture `guestRefusals` + schema + law; publication decoder carries `faultDisplayMessage`): `💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts`, `🔌️plugin/🌐️browser-bundle/🎯️action-handoff/{🟦️.ts,📤️publication/🟦️.ts,🧫️fixtures/🔣️.json,🧬️schema/🔣️.json,🧪️tests/…}`; codemods `wp-s18/s18-14c-guest-refusal-{ack,detail}.py` | TS/JSON only (host) | tsc 0 (`s18-14c-tsc-refusal-3.txt`), boot green (`s18-14c-beacon-4.txt`); live 7800 hub draw doc: example load refused WITH the guest's reason (`drawing.example.parse …`), next addLayer [0,1,0,1], 0 later refusals (`s18-14c-refusal-next-2.txt`); hub note PASS (was owner-mismatch cascade); laws queued (native lane) | 28 23:4x |",
]


def main() -> None:
    text = LANDING.read_text(encoding="utf-8")
    added = [row for row in ROWS if row[:70] not in text]
    if added:
        LANDING.write_text(text.rstrip("\n") + "\n" + "\n".join(added) + "\n", encoding="utf-8")
    print(f"added {len(added)}")


main()
