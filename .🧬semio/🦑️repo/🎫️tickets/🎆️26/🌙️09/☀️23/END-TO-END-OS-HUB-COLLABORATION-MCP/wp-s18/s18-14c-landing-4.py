# -*- coding: utf-8 -*-
"""📝️ S18 §14c: landing rows for the hub-sweep harness fixes and the Home table viewport fix (idempotent)."""
import pathlib

LANDING = pathlib.Path("/Users/ueli/Documents/semio/.tmp-ticket/📓️landing.md")
ROWS = [
    "| S18 | 14c hub legs — hub-document-sweep harness: `stagedKinds` reads the kind choice's combobox listbox (the space index renders it as a combobox now; the sweep saw 0 of 36 kinds), `openSweepSpace` opens `/hub` and waits for the Space Browser's phase (the session survives a reload now, so the workspace never opened and every kind read \"no space listed\"): `🧑‍💻dev/🧪️tests/🗂️hub-document-sweep/🟦️.ts` | TS only (dev harness) | tsc 0 (`wp-s18/generated/s18-14c-tsc-harness-{5,6}.txt`); live on 7800/p24: 36 kinds offered, en 17/36 measured (11 PASS; reds = guest/host findings, `.🧬semio/🌐hub/s14-s18-logs/sweep-s18-14c-hub-en-3.txt`) | 28 21:2x |",
    "| S18 | 14c C12 P1 — windowed table viewport capped at the rows the guest serves (`tableWindowViewportCapRowsV1`, observer `onServed`), tree-window viewport excludes a sticky header declared as `scroll-padding-top`, keyboard row move no longer lands one row short: `💻️os/…/🧱️elements/🗣️Interpreter/🟦️.tsx` (TableView, `useTreeWindowObserver`, `treeWindowViewportMetrics`), law `🗣️Interpreter/🧪️tests/📊️table/🟦️.tsx`, fixture `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🪟️tree-window-served.json` (`tableViewportCaps`); codemods `wp-s18/s18-14c-table-viewport{,-2}.py` | TS only (host) | tsc 0 (`s18-14c-tsc-table-2.txt`), boot green (`s18-14c-boot-table-2.txt`), Interpreter laws 233/233 on variant 1 (`s18-14c-law-table-1.txt`; final variant re-run after the reboot); live 7800 user1 38 spaces en + de: wheel/End/PageDown reach every row, 0 pageerrors (`s18-14c-home-{rows-6,keys-en-US,keys-de-DE}.txt`) | 28 22:3x |",
]


def main() -> None:
    text = LANDING.read_text(encoding="utf-8")
    added = [row for row in ROWS if row[:70] not in text]
    if added:
        LANDING.write_text(text.rstrip("\n") + "\n" + "\n".join(added) + "\n", encoding="utf-8")
    print(f"added {len(added)}")


main()
