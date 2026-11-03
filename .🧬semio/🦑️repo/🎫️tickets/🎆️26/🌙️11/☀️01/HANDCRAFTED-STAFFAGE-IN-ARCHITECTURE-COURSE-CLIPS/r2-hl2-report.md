# R2 language fix (agent hl2)

Scope: the three humans assets (running-child, wheelchair-user, seated-reader), the catalog root manifest, and a full scan of all 150 catalog `🔣️.json` files (17564 `{en, de}` pairs).

## Findings

- The three main titles were already German when checked: `running-child` "Rennendes Kind" (kept, correct German for sprinting), `wheelchair-user` "Rollstuhlnutzende Person" (kept, as suggested), `seated-reader` "Lesende Person im Sitzen" (changed, see below).
- The real English leaks were in the position-marker symbols of the same three assets. In each, the English description sat in `title.de`, and the German title sat in `description.en`. The same swap was already fixed in walking-commuter earlier.

## Fixes (Edit, text only)

1. `👤️humans/running-child/🔣️.json` symbol `running-direction-marker` (around line 2805): `title.de` = "Rennendes Kind als Marke"; `description.en` = "Position dot with a direction arrow for crowded overview plans at 1:500 and smaller."
2. `👤️humans/wheelchair-user/🔣️.json` symbol `wheelchair-position-marker` (around line 3689): `title.de` = "Rollstuhlnutzende Person als Marke"; `description.en` = "Position dot with a direction arrow for crowded overview plans; the dot marks the rear axle."
3. `👤️humans/seated-reader/🔣️.json` symbol `seated-person-marker` (around line 3431): `title.de` = "Sitzende Person als Marke"; `description.en` = "Position dot for crowded overview plans; no direction arrow because a seated person does not travel."
4. `👤️humans/seated-reader/🔣️.json` (line 9) and root manifest `🔣️.json` (around line 313): `title.de` "Lesende Person im Sitzen" to "Sitzende lesende Person", the form requested in the brief, matching the "<adj>e Person" pattern.

## Catalog scan

Script: `🧰️hl2/de_scan.py` (ticket-local, run with PYTHONIOENCODING=utf-8). It flags identical en/de pairs, empty values, German function words in `en`, and English function words plus a short list of English-only content words in `de`.

- 17564 pairs in 150 files, after the fixes: 0 de prose leaks found.
- Identical pairs (33 hits, all accepted): proper names, materials and trade terms (Denim, Leggings, Jeans, Fleece, Ollie, Pause, Eco, Tour, Turbo (250 W), Maximum, Monitor, Laptop (2 kg), Agave, Caudex, HDPE), units and profile codes (2000 W, HEA 160, IPE 160, EPS 70 mm, RHS 100x50x4).
- Remaining hits, all accepted: English identifiers in overlay and merged decision text (`frame-timber`, `box-and-rails`, `monitor-housing`, `lamp-housing`, `standing-adult`); the English word "die" in "die-cast" (en side, correct German "Druckguss" on de side); "Wood Handbook" (proper name); "Walking-Stiefel" and "Kneeling" (accepted German transit/orthopaedic terms, left unchanged); "Dreisitzer-Sofa" and "Sofa" (valid German).
- Open judgement: "Kneeling" is used as a German term in city-bus pose and clip titles (e.g. `🚜️vehicles/city-bus/🔣️.json`, poses[2].title "Kneeling mit offenen Türen"). It is a standard German transit term, so I did not change it. A native reviewer may prefer "Absenkung".

## Catalog check

`repo tools exec bun -- 🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🩺️check/🟦️.ts` (no arguments), rerun after the edits:

- `✓ manifest`, 147 asset checks, 148 `✓` lines in total, 0 `✗`, exit 0.
- A first run exited 0 but stopped after 55 assets, so it was not counted. The rerun above is the valid result.

## Still open

- Nothing in the three named assets is still English in `de`.
- The German review is by reading, not a native speaker check.
