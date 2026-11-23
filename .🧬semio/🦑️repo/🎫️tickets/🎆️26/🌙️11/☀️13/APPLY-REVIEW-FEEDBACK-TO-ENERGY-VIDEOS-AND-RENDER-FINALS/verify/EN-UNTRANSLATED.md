# 🌍 English video: untranslated on-screen literals

Static audit of the Manim clip scripts in the five energy families (`🧲️physics`, `🌬️ventilation`, `🔥️heating`, `❄️cooling`, `📊️demand`), `🐍️.py` files under `🎬️clip`, excluding `🧪️tests` and narration (`🔊️speech`) folders. Method: AST scan of every `Text`, `_m`, `math_label`, `chip`, `meter`, `_label`, `body_text` and similar call for string literals outside `tr(...)`, plus a scan of umlaut literals and raw `\text{...}` strings, with manual reading of each hit.

Paths are relative to `🎓️teaching/🏛️architecture/⚡️energy/`.

## Findings

| File | Line | Literal | Why language-specific | Suggested replacement |
|---|---|---|---|---|
| `🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py` | 715 | `Text("An", ...)` | German "An" (= "On"); shown on the power control in the English video. Known bug. | `Text(tr("On", "An"), ...)` |
| `📊️demand/🎬️clip/🧪️practice/🐍️.py` | 904 | `meter("JAZ", length=1.8, thickness=0.40, color=GOOD_GREEN)` | German acronym (Jahresarbeitszahl). The English video should show SPF. Line 910 already uses `tr(r"\text{SPF}\uparrow", r"\text{JAZ}\uparrow")`, so this label is inconsistent with it. | `meter(tr("SPF", "JAZ"), length=1.8, thickness=0.40, color=GOOD_GREEN)` |

Only these two literals are unwrapped and language-specific and rendered on screen.

## Checked and not reported

- **Umlaut literals on screen, all already paired.** Chapter and beat labels paired through `_USE_EN` (`📦️volume`), `_CARD_EN` (`🤝️handoff`), `_WALL_EN` (`🧱️conduction`) and `ENVELOPE_EN` (`🔁️recap`, `🧱️conduction`), or wrapped in `tr(...)` on the next line, as in `🧮️calculation` lines 487-492 and `🧲️fundamentals` lines 866-871 and 1080-1085.
- **Narration tables.** Bilingual `(id, English, German)` tuples in `🧲️fundamentals`, `🗺️roadmap` and `🏭️primary-energy` are the pair form and are excluded.
- **Identical in both languages, fine as is.** `"Winter"` (`🪟️natural` 1444, 1585, 1088, 385; `❓️need` 393, 515; `🧲️fundamentals` 1635), `"Wind"` (`🪟️natural` 844), `"Start"` (`🧲️fundamentals` 704), `"Transmission"` (`🧮️calculation` 458; `🔁️recap` 491), `"Thermostat"` (`🧮️calculation` 1284), `"Person"` (`⚖️heating-comparison` 299), `"Hygiene"` (`📐️system-sizing` 523, 560), `"Gas"` (`♨️system-losses` 1204), `"Toaster"` (`🧱️conduction` 890), `"Reserve"` (`⚡️demand-power` 1190), `"Jan" ... "Nov"` (`🧲️fundamentals` 860, only the non-identical months Mar, May, Oct, Dec are wrapped), `"Low-E"`, `"UV"`, `"West"` (`☀️solar-radiation` 297), and the cardinal letters `N`, `S`, `W`.
- **Mapped labels.** `"Horiz"` and `"O"` in `☀️solar-radiation` 320-327 are keys mapped through `tr(...)`, and `"Außenwand"`, `"Dach"`, `"Fenster"`, `"Boden"`, `"Haustür"` in `🧱️conduction` and `🔁️recap` are ids with `tr(...)` labels.
- **Proper names, norm ids and symbols.** `chip("Potsdam")` (`⚖️regulations` 1080), `DIN EN ...` / `VDI ...` ids (`⚖️regulations` 658-660, `⚡️demand-power` 688), `\text{(SCOP)}`, `\text{ · }`, `\text{in}` and similar LaTeX fragments.
- **Test fixture, not on screen.** `🌬️ventilation/🎬️clip/🧪️architecture/🐍️.py` line 99, `"Passive house with ventilation system "`, is a unit-test stub inside a clip folder.

## Dead German constants (not rendered)

These are assigned but never read, so they never reach the screen. No action is needed for the English video. If a chapter title is ever revived, it must be wrapped in `tr(...)`.

- `TITLE_DE` (or `TITLE_OPAQUE_DE`, `TITLE_LAST_DE` in `❄️cooling/💧️transmission-humidity`) is defined but unused in 22 clip files, including `🧲️physics/🧲️fundamentals` line 79, `🔥️heating/🧲️fundamentals` line 74, `🔥️heating/🧱️conduction` line 82, `📊️demand/🔁️recap` line 82 and `🌬️ventilation/⚙️mechanical` line 95.
- `STAGES` names in `📊️demand/♨️system-losses` line 113 (`"Übergabe"`, `"Verteilung"`, `"Speicherung"`, `"Erzeugung"`) are discarded (`_name`), so they are not drawn.
- Layer names in `🔥️heating/🧱️conduction` lines 103-106 and `🔥️heating/🧲️fundamentals` lines 1440-1445 (`"Innenputz"`, `"Vollziegel"`, `"Außenputz"`, `"Dämmung"`) are not drawn; the layer tuples are read only for thickness and colour.
