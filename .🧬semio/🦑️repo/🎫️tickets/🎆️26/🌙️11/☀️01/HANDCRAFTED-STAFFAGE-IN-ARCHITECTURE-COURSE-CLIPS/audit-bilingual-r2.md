# 🌍 Bilingual Audit, Round 2 (read-only)

Scope: manifest `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧍️staffage/🔣️.json` and the 49 catalog assets it references (cast `asset` ids, plus alternatives reached from them). Paths below are relative to `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/`.

Scripts: `🧰️bl/audit_bilingual.py`, `🧰️bl/check_fragments.py`, `🧰️bl/dump_titles.py`, `🧰️bl/survey_keys.py`.

## Summary

| Check | Result |
|---|---|
| Manifest text fields | none (ids only), all 150 casts resolve, 49 asset ids found, 0 duplicate asset ids |
| Missing or empty `en` / `de` | 0 |
| Objects with only one language or extra keys | 0 |
| Plain (non-localized) human-facing strings | 0 |
| German stopwords in `en`, English stopwords in `de` | 0 |
| ASCII umlaut substitutes (`fuer`, `ueber`, `koennen`, ...) in `de` | 0 |
| ASCII eszett substitutes (`Strasse`, `gross`, ...) in `de` | 0 (all `ss` tokens checked, correct spelling) |
| `de` identical to `en` | 10, all accepted loanwords or units (see below) |
| Truncated or fragmentary text | 6 defects, 4 fragment groups (see table) |

Findings that need an edit are in the table. Accepted items are listed after it.

## Findings

| File | Field | Issue | Suggested text |
|---|---|---|---|
| `👤️humans/person-cooking/🔣️.json` | `.asset.representations.50.decisions.omitted` | en is broken ("camera lens of."), de is broken ("dess."); also a typo in de | en: `Pores, individual hairs, stitching holes, fabric fibres, the camera lens.` / de: `Poren, einzelne Haare, Stichlöcher, Gewebefasern, die Kameralinse.` |
| `👤️humans/standing-adult/🔣️.json` | `.asset.representations.50.decisions.omitted` | Same broken en/de pair as person-cooking | en: `Pores, individual hairs, stitching holes, fabric fibres, the camera lens.` / de: `Poren, einzelne Haare, Stichlöcher, Gewebefasern, die Kameralinse.` |
| `👤️humans/person-cooking/🔣️.json` | `.asset.representations.200.decisions.merged` (en and de) | Truncated sentence "hair tie into." / "Haargummi in." | en: `... sock into the sneaker, hair tie into the hair cap.` / de: `... Socke in den Sneaker, Haargummi in die Haarkappe.` (en and de keep the rest of the existing text) |
| `👤️humans/standing-adult/🔣️.json` | `.asset.representations.200.decisions.merged` (en and de) | Same truncation as person-cooking | en: `... sock into the sneaker, hair tie into the hair cap.` / de: `... Socke in den Sneaker, Haargummi in die Haarkappe.` |
| `👤️humans/person-cooking/🔣️.json` | `.asset.representations.50.decisions.thinElements`, `.100...`, `.200...` (en and de) | Fragments with leading space and no subject (`" is one 0.18 mm stroke ..."`, `" ist ein Strich ..."`). Subject is the element with the 148 mm length, not stated in the text | Add the subject the author intends, e.g. en: `The <element> is one 0.18 mm stroke ...` / de: `Der <Element> ist ein Strich ...`. Author must name the element; not guessed here |
| `👤️humans/standing-adult/🔣️.json` | `.asset.representations.100.decisions.thinElements`, `.200.decisions.thinElements` (en and de) | Same subject-less fragment as person-cooking, leading space | Same as above, name the subject |
| `🌳️vegetation/english-oak/🔣️.json` | `.asset.representations.50.decisions.thinElements` | en `stroke (0.2 mm, true length 148 mm) replaces the 10 mm slab; is a 0.1 mm line.` and de `strich (0,2 mm, ...) ersetzt ...` start without subject; de also starts lowercase | en: `The <element> stroke (0.2 mm, true length 148 mm) replaces the 10 mm slab; it is a 0.1 mm line.` / de: `Der <Element>-Strich (0,2 mm, wahre Länge 148 mm) ersetzt die 10 mm dicke Platte; er ist eine 0,1-mm-Linie.` |
| `🛋️furniture/office-swivel-chair/🔣️.json`, `👤️humans/seated-office-worker/🔣️.json` | `.asset.rig.bones[9].title` (office-swivel-chair), `.asset.rig.bones[10].title` (seated-office-worker) | de `Hinterer Rollen A` is ungrammatical (masc. adjective with fem./neuter noun, plural with singular "A") | de: `Hintere Rolle A` (en `Back castor A` unchanged) |
| `👤️humans/jogger/🔣️.json` | `.asset.title` | de `Jogger:in` uses gender colon while other titles have no gender forms; Low severity | Keep `Jogger:in` if the archive uses colon forms; otherwise `Jogger` (accepted German loanword) |
| `🔌️appliances/cooking-pot/🔣️.json`, `🔌️appliances/electric-kettle/🔣️.json`, other `Switched off` / `Off` pairs | `.asset.clips[0].phases[0].title` (cooking-pot, electric-kettle) | en `Switched off`, de `Aus`. Other assets use `Ausgeschaltet` for the same state. Consistency, Low | de: `Ausgeschaltet` |

## Accepted (checked, no change)

- Identical en/de, accepted German loanwords or units: `Denim` (materials, person-cooking, standing-adult), `Pause` (seated-office-worker), `Monitor` (dining desk, height-adjustable-desk), `Laptop (2 kg)` (height-adjustable-desk), `2000 W`, `1000 W`, `EPS 70 mm` (external-venetian-blind), `Laptop` and `Monitor` in long summaries (height-adjustable-desk).
- English-looking words in de that are normal German: `Laptop`, `Monitor`, `Server`, `Switch` (network switch), `Sneaker`, `Region`, `Silhouette`, `Transpiration`, `Thermostat`, `Toaster`, `Rack`, `Emitter`, `Phase`, `Pose`, `Clip` (Animation sense in technical text). `Jogger` likewise.
- Mixed translations for the same English word across assets (`Boost` → `Intensiv` / `Intensivlüftung`, `Turning` → `Dreht` / `Drehen`, `Boiling` → `Kochen` / `Kochend` / `Kocht`) are contextual, not errors.
- Short length-ratio hits (`Fan` → `Ventilator`, `High` → `Volle Flamme`, `Lit` → `Eingeschaltet`, `Off` → `Aus`) are correct translations, not truncations.
- `Asset`, `Klasse`, `Masse`, `Wasser`, `dass`, `muss` and other `ss` tokens are correct spelling in German.
- No `ae`/`oe`/`ue` substitutes and no ASCII eszett were found in any `de` string.

## Scope notes

- `decisions.*` (internal rendering notes) and `rig.bones`, `materials`, `anchors` titles were audited with the same rules, since they are localized and reach the UI.
- Descriptions and summaries were checked automatically (stopwords, verbatim copies, ratio, truncation). Long text was not re-read line by line, apart from the findings above.
- Fragment findings from `check_fragments.py` other than the ones listed were false positives (sentences ending in a noun, "switched on", "castor A").
