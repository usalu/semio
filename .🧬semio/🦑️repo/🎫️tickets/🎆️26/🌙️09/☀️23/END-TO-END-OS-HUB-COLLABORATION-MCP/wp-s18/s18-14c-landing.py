# -*- coding: utf-8 -*-
"""📝️ S18 §14c: appends this session's landing rows to 📓️landing.md (idempotent by row prefix)."""
import pathlib

LANDING = pathlib.Path("/Users/ueli/Documents/semio/.tmp-ticket/📓️landing.md")
ROWS = [
    "| S18 | 14c item 1 — program matrix drives revision-bound verbs (`set-cell`, `set-node`) through the program's rendered inline editor (its binding carries the document revision no staged rail argument can know): pins `pluginEdits`/`kindEdits` (`MatrixRenderedEdit {control, value}`, fill + Enter) replace the `stdio.set-cell` args and the json/xml `set-node` args; `foldVerbPins` (plugin → kind → subset) shared by args and edits; `driveRenderedEdit` + `resolvePins` exported; `readMatrixPins` runtime-neutral (`import.meta.url`, was Bun-only `import.meta.dir`); hub-document-sweep passes plugin edits; attempts record what was typed. Schema-first: `$defs.ProgramMatrixPinsV1` + `ProgramMatrixRenderedEditV1` (`🧑‍💻dev/🧬️schema/🔣️.json`); law `🧑‍💻dev/🧪️tests/🧮️program-matrix/🧮️reducers/🟦️.ts` (Ajv + jsdom oracles), registered in `🧑‍💻dev/🧪️tests/🎚️config/🟦️.ts`. Files: `🧑‍💻dev/🧪️tests/🧮️program-matrix/🟦️.ts`, `🧑‍💻dev/🧪️tests/🗂️hub-document-sweep/🟦️.ts`, `🧑‍💻dev/🧫️fixtures/🧮️program-matrix.json` | TS only (dev harness) | tsc 0 (`wp-s18/generated/s18-14c-tsc-harness-3.txt`), laws 4/4 + local-hub 27/27 in the native lane (`s18-14c-law-pins-{5,6}.txt`); live: csv/tsv `set-cell` now applies, undoes and redoes (edits [0,1,0,1]) — rows stay red on guest faults only (`.🧬semio/🌐hub/s14-s18-logs/matrix-s18-14c-stdio-en.txt`) | 28 17:4x |",
    "| S18 | 14c item 4 — identity gate follow-up: ShellHost's identity bootstrap reads the hub authority (`sessionPort.me`) at once and the persisted identity record BESIDE it (the 2 s `AbortSignal.timeout` wait for a record ran first and held every first sign-in's identity — and with it the input gate — 2 s behind the hub); the record only decides the write-back (an unchanged identity is not rewritten unless the authority moved; a returning human keeps its `issuedAtMs`): `💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx` identity-bootstrap effect | TS only (host) | tsc 0 (`s18-14c-tsc-gate-1.txt`), served module verified, boot Home 11 s 0 pageerrors (`s18-14c-boot-2.txt`); live (serve 6540 → catalog-less hub 8040, user1, 3 fresh sign-ins): input 122–495 ms after the session shows → guest dispatch +35–123 ms, dialog +140–230 ms, 0 refusals, 0 pageerrors (`s18-14c-identity-1.txt`); form closed → dialog 245–278 ms vs 2 052–2 448 ms before (`s18-14b-dialog-{4,5,6}.txt`) | 28 17:5x |",
]


def main() -> None:
    text = LANDING.read_text(encoding="utf-8")
    added = [row for row in ROWS if row[:60] not in text]
    if added:
        LANDING.write_text(text.rstrip("\n") + "\n" + "\n".join(added) + "\n", encoding="utf-8")
    print(f"added {len(added)}")


main()
