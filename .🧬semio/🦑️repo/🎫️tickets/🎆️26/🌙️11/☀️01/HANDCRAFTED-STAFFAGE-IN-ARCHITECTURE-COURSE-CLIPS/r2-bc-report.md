# r2 bc: full-catalog bilingual audit and fixes

Scope: every `🔣️.json` under `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/` (147 assets plus the root index). Scripts in `🧰️bc/`: `audit_full.py` (all checks, writes `🗑️generated/bc/findings.tsv`), `ids_in_text.py`, `shared_tokens.py`, `tok_ctx.py`, `get.py`, `show.py`, `show2.py`, `titles.py`, `where.py`, `perfile.py`, `map.tsv` + `apply_map.py`, `apply_pairs.py`. It extends bl's `audit_bilingual.py` with: lowercase starts, trailing function words, subject-less fragments, doubled words and articles, German adjective-noun agreement, unbalanced parentheses, raw kebab-case ids in titles, and the new sibling-label consistency check (same lowercased English label inside one asset kind, different German wording).

## Result of the first run
7,577 raw findings, almost all heuristic noise (`ss`-tokens, loanwords, `Klaue`/`Quer`/`Braue` as "ue" hits). Triage: no missing or empty en/de, no ASCII umlaut or eszett substitutes, no English stopwords or German stopwords that are real errors (the `of` hits are citation titles), no truncated sentences left, no subject-less fragments. Real findings, all fixed:

| Group | Finding | Fix |
|---|---|---|
| Agreement | `Linker Hand` / `Rechter Hand` (22 assets each), `Linker/Rechter Ferse` (15 each) | `Linke/Rechte Hand`, `Linke/Rechte Ferse` |
| Agreement | `Linke/Rechte Ballen` (4) | `Linker/Rechter Fußballen` |
| Agreement | animals: `Fernes/Nahes Unterarm`, `Oberarm`, `Oberschenkel`, `Unterschenkel` (goat, pig, red-fox, sheep, herring-gull), `Ferne/Nahe Unterarm/Oberarm` (red-squirrel, dairy-cow), `Ferne/Nahe Schulterblatt`, `Fernes/Nahes Vorder-/Hinterpfote`, `Fernes/Nahes Mittelhand/Mittelfuß` | masculine/feminine/neuter forms corrected; `Vorderpfote (fern)` unified to `Ferne Vorderpfote`; `Hinterfuß (ferne) am Boden` to `Hinterfuß fern am Boden` |
| Truncated/mismatch | bar-stool `decisions.omitted` rep 200: de lacked the leg part of the English sentence | de now translates the full sentence |
| Typo | glass-balustrade provenance: "ein ein Meter breiter Streifen" | "ein 1 m breiter Streifen" |
| Missing word | horse rep 500 overlays: "der Kopf Kopf, Maul und Auge" | "der Kopf trägt Kopf, Maul und Auge" |
| Unquoted pose name | evaporative-humidifier: "in der Pose Trocken trocken" | "in der Pose „Trocken“ trocken" |
| Elided verb | shopping-trolley rep 200 overlays: "wheels wheels" / "Räder wheels" | "the wheels wheels" / "die Räder wheels" (region ids stay verbatim) |
| Raw ids as text | courier-with-hand-truck and pregnant-person contact events `heel-l on`/`ball-r off` (en and de) | `Left heel contact`/`Linker Fersenkontakt`, `Right ball lifts`/`Rechter Fußballen hebt ab`, etc. (10 events each, ids and shapes from event `kind` and `target`) |
| Raw ids as text | ivy-on-trellis pose loads `Wind drag on fol-1-lo`, `Snow on the leaf mat fol-4-hi` (en and de, 3 poses) | `Wind drag on leaf mat 1, lower`/`Winddruck auf Blattteppich 1, unten` (names from the bones `fol-N-lo/hi`) |
| Capitalisation | `limitingFeature` rep 200/500 in evaporative-humidifier, expansion-valve, finned-heat-exchanger, plate-heat-exchanger, scroll-compressor, sorption-wheel, electric-kettle, gas-hob, cooking-pot started lowercase in en and de | capitalised; evaporative-humidifier de also lacked the "0.5 mm" of en: now `Die 0,5 mm großen Tropfen ...` |
| Capitalisation | bone titles `shaded lower core`, `lower west foliage`, ... in apple-tree, japanese-maple, london-plane-pollarded, small-leaved-lime (en); de `beschatteter Kern unten` etc. | capitalised in both languages |
| Titles | en titles `Kneeling gardener`, `Parent carrying a toddler`, `Seated guitarist` | title case like their siblings |
| Titles | `Jogger:in`, `Skateboarder:in` (only gender-colon titles in the archive; also in the root catalog index) | `Joggende Person`, `Skateboardfahrende Person` (like `Radfahrende Person`) |
| Consistency | `Aus` (cooking-pot, electric-kettle) vs `Ausgeschaltet` | `Ausgeschaltet` |
| Consistency | `Fahrschleife start` (5 vehicles) | `Fahrschleife Start` (noun capital) |
| Consistency | `Linke/Rechte Fersenabhebung` (worker-carrying-box) vs `Linke/Rechte Ferse hebt ab` | `... Ferse hebt ab` |
| Wording | stair-flight pose `Person in Lauf-Mitte` for "Person mid-flight" | `Person mitten im Treppenlauf` |

## Sibling consistency (new check)
96 label groups differed. Unified (de): `Doppelstütz` to `Doppelstandphase`, `Linke/Rechte Einzelstütz` to `Einbeinstandphase`, `Nackte` to `Unbedeckte Hände/Arme`, `Rücken an der Lehne` to `... Rückenlehne`, `Baumwollslip` to `Baumwollunterhose`, `Jeansstoff` to `Denim` (denim) / `Jeans` (denim jeans), `Nasenöffnungen` to `Nasenlöcher` (also `Atem aus den Nasenlöchern`), `Turnschuhe` to `Sneaker`, `Kopf mit Kurzhaarschnitt` to `Kopf mit kurzem Haar`, `Pulloverärmel` to `Hoodie-Ärmel`, `Hinteres Radpaar` to `Hinterradpaar`, `Volle Last` to `Volllast`, `Gewölbte Abdeckung` to `... oben`, `Windstoß` to `Böe`, `Laubmasse Wipfel` to `Laubmasse Kronenspitze`, `Beschattetes Innenlaub` to `Beschattete Innenlaubmasse`, tram `Seiten- und Stirnwände` to `Seiten- und Endbleche`, `Sohle des ... Hufes` to `Hufs`.

Left as is, species- or context-specific terms (documented, not errors): `Löffel`/`Schaufel` (excavator vs wheel loader), `Kabine`/`Fahrerhaus`, `Abgasrohr`/`Auspuffrohr` (generator vs vehicles), `Ohren`/`Lauscher`, `Nasenlöcher`/`Nüstern` (horse), `Schwanz`/`Rute`/`Ringelschwanz`, `Hufe`/`Klauen`/`Schalen`, `Hufhorn`/`Klauenhorn`, `Fell der Vorder-/Hinterbeine` vs `-läufe` (roe deer), `Schaft`/`Stamm`, `Heizbetrieb`/`Heizen`, `Öffnen`/`Öffnet`, `Dreht`/`Drehen` (state vs action), the `(Speiche)`/`(Tibia)`/`(Femur)` suffix variants, and the plural/singular tyre and wheel patch labels (different counts).

## False positives (checked, no change)
`Linkes/Rechtes Bein` (neuter, correct), `die die`/`der der` as relative pronouns, `A-Gestell`/`A-Rahmen` ("an A"), `gehen ... auf`, `fährt ein`, `ruht ein` (separable verbs), `Switched on`, `Key on`, `Hooking on`, `castor A`, `Approach a`/`Anflug a` (phase letter), id tokens in `decisions.overlays` of shopping-trolley, stepladder, bookshelf, patio-heater, bar-stool (region ids are quoted verbatim in both languages), `Wood Handbook` and other citation titles, `Slip`/`Short`/`Overall`/`Jeans`/`Denim`/`Fleece`.

## How the edits were applied
- Edit tool: bar-stool, glass-balustrade, evaporative-humidifier, horse, stair-flight, shopping-trolley (6 one-off sentences), re-read just before editing.
- Deviation, flagged: the 200-odd mechanical word swaps and the pair rewrites (events, ivy loads, capitalisations, titles) went through `🧰️bc/apply_map.py` and `🧰️bc/apply_pairs.py` instead of 250 Edit calls. Each run re-reads the file, replaces only exact `"de": "OLD"` tokens or adjacent `en`+`de` pairs, validates with `json.loads`, aborts the write if the file changed meanwhile, writes LF with a truncating write. Windows raised sporadic `Errno 22` on write (about a third of the files, always passing on retry); no partial writes happened. A file mid-edit by ap/av would have been skipped, none was.

## Verification
See the bottom of this file for the catalog check and the library vitest.

## Open
- Male-form human titles (`Gitarrist`, `Pendler zu Fuß`, `Läufer`, `Kurier`) and `Gärtnerin`/`Smartphone-Nutzerin` (female) mix gendered and neutral forms; changing them alters the described persona, so they are left to the content owner.
- The root index `🗂️catalog/🔣️.json` carried the two colon titles and was fixed with the same script.

## Verification results
- Full catalog check (`🔨️modules/🗂️catalog/🩺️check/🟦️.ts`, no arguments), run after all edits: exit 0, 147 assets listed with a check mark, only the expected `unavailable` scale notes.
- Staffage library vitest (q2 alias config): 56 files, 1125 passed, 11 skipped, 2 failed. The two failures are `export-performance > equal the pinned digests` (`radiator-transient/0000.svg`) and `front-projection > sizes a front cast to the front and a side cast to the section`. They are the same two that `r2-av-report.md`, `r2-cr2-report.md` and `r2-fm-report.md` attribute to agent ap's concurrent panel-radiator wall removal; panel-radiator is not among the files touched here.
- Re-audit after the edits: findings dropped from 7,577 to about 7,190 raw heuristic hits, with 0 remaining agreement errors (`Linkes/Rechtes Bein` is correct), 0 lowercase-start labels, 0 raw ids in event or load titles, and the remaining sibling groups only the contextual ones listed above.
- Generated output (`🗑️generated/bc`) deleted.
