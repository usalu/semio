# L33 Localized Inspector Flag Values (2026-11-24)

Follows up the two de risks in `l20-locale-audit.md`.

## Finding
All three editor inspectors showed `hidden`/`locked` rows with `pressed.to_string()`, so the value read `true`/`false` in every locale. The 2D time-travel gap test `g4-lock…` passed only because it matched `/\btrue\b/` against that untranslated text.

## Fix
- New terminology labels `yes` ("Yes"/"Ja") and `no` ("No"/"Nein") in the 2D, 3D and 5D `✏️editor/🗣️terminology/🦀️.rs` (`app_labels!`, native and reuse).
- The inspection panels (`✏️editor/📌️panels/🔍️inspection/🦀️.rs`) in 2D (`flag_row`, `region_flag_row`), 3D (`flag_row`) and 5D (`flag_row`, `volume_flag_row`) take the labels and render the yes/no text. The action payload is unchanged (`value: Bool(!pressed)`).
- Puzzle time-travel copy `✏️s/🔌️plugins/🧩️puzzle/🧪️tests/⏪️time-travel/🌍️copy/🟦️.ts` gains `flagOn`/`flagOff` regexes per locale. `🧮️gaps/🟦️.ts:427,433` uses them.
- `🌄️drag-journey/🟦️.ts:198`: the no-editing-peer roster regex is now `/⏪|is editing|bearbeitet\b.*\bVerlauf\b/`. It covers all four de presence notes, including the beginner "bearbeitet gerade den Verlauf", which the old pattern missed.

No Rust test asserted the raw `"true"`/`"false"` description. Compile and test verification comes with the next native batches (3D: `l32-verify.sh`).
