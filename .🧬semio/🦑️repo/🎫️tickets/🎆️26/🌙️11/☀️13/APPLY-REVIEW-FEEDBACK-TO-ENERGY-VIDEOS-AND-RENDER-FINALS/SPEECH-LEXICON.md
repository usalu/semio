# Speech lexicon and inline math in narration (F11, F24)

A narration clause `(key, text_en, text_de)` may contain `$…$` TeX segments. The caption shows the real
symbol (`Δθ`), the voice says "Delta Theta", exports get plain Unicode. One text, three renderings.

## 1. Authoring rules

- Write every symbol, formula fragment and value-with-compound-unit between dollar signs, in both languages. Use raw
  strings (`r"…"`) wherever a backslash occurs.
  - `r"Treiber ist die Temperaturdifferenz $\Delta\theta$, innen minus außen."`
  - `r"Der Lüftungswärmestrom $\dot{Q}_{\mathrm{V}}$ ist $\dot{V}\cdot 0{,}34\,\mathrm{Wh/(m^3K)}\cdot\Delta\theta$."`
  - `r"… bei $\theta_i = 20\,^\circ\mathrm{C}$ …"`, `r"… $U = 0{,}24\,\mathrm{W/(m^2K)}$ …"` (English text: `0.24`).
- Never spell a symbol out ("Delta-Theta", "Q-Punkt", "c-Luft", "Phi-V") and never put bare Unicode math letters
  (Δ θ Φ ρ λ η ψ …) outside `$…$`. The guard (section 4) fails for both. A literal dollar sign is `\$`.
- Accepted TeX (the same subset the scenes use via `math_text`, plus the extras listed):
  - Letters, digits, `+ - = < > / ( ) [ ] , ; : .`, numbers with `{,}` (decimal comma) or `.`, `1\,000` (thousands).
  - Symbols from `📝️narration/🧮️tex/🔣️.json`: Greek letters (`\alpha … \omega`, `\Gamma … \Omega`, `\varphi`, `\vartheta`,
    `\varepsilon`), `\cdot \times \pm \approx \le \ge \to \Rightarrow \uparrow \infty \ldots \dots \cdots \sum \oint \perp \circ`.
  - Structure: `x_{i}`, `x^{2}`, `\frac{a}{b}`, `\dot{Q}`, `\sqrt{x}`, `\mathrm{kWh}`, `\text{plain words}`, `\left … \right`
    (delimiter only), spaces `\, \; \quad \!`.
  - Degree Celsius: `20\,^\circ\mathrm{C}` or `20\,°\mathrm{C}`; reciprocal units: `\mathrm{h^{-1}}`, `\frac{1}{\mathrm{h}}`.
  - Units are upright: put them in `\mathrm{…}` and separate them from a number with `\,`. A product of units needs
    a thin space or a script between the factors (`\mathrm{m^2K}` or `\mathrm{m\,K}`; `\mathrm{mK}` is read as the
    units m and K because the lexicon splits runs into known units, so both work).
  - Upright abbreviations that are not units (`\mathrm{JAZ}`, `\mathrm{WRG}`) must be listed under `abbreviations`.
  - Words inside a formula: `\text{ und }` is spoken literally; any other unknown TeX FAILS at plan time.
- Compound units and fractions: a `\frac` of two unit expressions or `1/\mathrm{h}` is spoken "… pro …"; the denominator
  is joined with "und" ("Watt pro Quadratmeter und Kelvin"). Any other `\frac{a}{b}` is "a durch b" (operands with
  `+ - =` are bracketed: "Klammer auf … Klammer zu").
- Indices: `_{\mathrm{V}}` "V", `_{si}` "s i", `_i` "i", `_{V,\mathrm{lat}}` "V latent", `g_\perp` "g senkrecht".
  Index words (`lat`, `sens`, `eq`, `min`, `max`) are read as words, every other index is spelled letter by letter.
- Numbers keep the language of the clause: `0{,}34` is "0,34" in German and "0.34" in English; "1" selects the singular
  of a unit ("1 Kilowattstunde"), every other value the plural.

## 2. Plain text (exports, read-time, logs)

`plain_text(clause)` / `math_to_unicode(tex)` in `📝️narration` give one deterministic rendering, never raw TeX:
Greek letters, `Q̇` (combining dot), `m³`, `h⁻¹` (Unicode superscripts only for digits and signs), every other script
ASCII-marked: `Q̇_V`, `R_si`, `x^n`, `Q̇_(V,lat)`. Fractions are `a/b`; an operand that is not a single atom is bracketed
(`W/(m² K)`, `Φ/(Δθ)`). Relations and binary operators get spaces (`Φ = U · A · Δθ`), `\,` becomes U+202F.
Checked against `pylatexenc` (test-only dependency, `pylatexenc==2.10` in the `test` group).

Consumers found: captions (`caption_bar`, `swap_caption`), reading time (`subtitle_read_seconds` counts printed words),
speech timing estimates (`narration_seconds` counts spoken words), `subtitle_narration_text`, per-clip speech modules.
There is no SRT/VTT export and the trace `trace/<identity>/<scene>/🔣️.json` stores only `start`/`window` per clause key,
so no further plain-text consumer exists today; any future one must call `plain_text`.

## 3. The lexicon (`🔊️speech/🗣️lexicon`)

`🔣️.json` (validated by `🧬️schema.json`) holds every pronunciation per language, no default language:

| key | meaning |
|---|---|
| `names` | spoken name of each TeX symbol (`Delta` → "Delta", `perp` → "senkrecht") |
| `characters`, `words`, `powers` | operators ("gleich", "minus"), phrases (`{}-Punkt`, "pro", "und"), exponent phrases |
| `indexWords` | subscripts read as a word |
| `abbreviations` | upright abbreviations (`JAZ` → "J A Z") |
| `units` | per unit `de`/`en` (string or `{one, other}`) and optional `powers` (`m` squared → "Quadratmeter") |
| `symbols` | whole-symbol overrides, keyed by canonical TeX; they win over composition at any position |
| `forbidden` | regex patterns and the Unicode class that the guard rejects outside `$…$` |

Add an entry: edit `🔣️.json` only, add a case to `🔊️speech/🧪️tests/🗣️lexicon/🔣️.cases.json`, run the lexicon tests. Override
keys are canonical: no spaces, no `\mathrm`, scripts braced (`R_{si}`, `\eta_{\mathrm{WRG}}` is stored as written and
canonicalised to `\eta_{WRG}` on load). A new TeX command needs a row in `📝️narration/🧮️tex/🔣️.json`
(`char`, `kind`, optional `alias`) AND a spoken name in `names`; the tests fail when one is missing. The caption
typesetter reads the same symbol table (`_MATH_SYMBOLS` in the visuals is derived from it).

Failures: `SpeechError`/`TexError` name the clause key and file (`spoken_clauses(narration, language=…, source=…)`),
and `synthesize_clause_audio` validates all clauses before the first request, so unknown TeX is never sent to the voice.

Voice language: unchanged, the voice reads the German clause (`subtitle_clauses`); `spoken_clauses(…, language="en")`
exists and is tested, so switching the voice to the caption language is one argument.

Recording identity: `synthesize_clause_audio` writes `identity.json` beside the clause files, one hash per clause of
spoken text + model + voice + speed + instructions. A recording is reused only when its hash matches (also without an
API key); changed pronunciations or lexicon entries re-synthesize exactly the affected clauses. Recordings without an
identity entry (made before this change) are re-synthesized once.

## 4. Guard

`narration-guard.sh` (this ticket folder) runs `📝️narration/🧪️tests/🛡️guard`, which loads every scene of the manifest
`⚡️energy/🎬️clip/🔣️.json` (all five videos) and reports each offender as `script:line [clause-key lang] message`:

```
bash .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/APPLY-REVIEW-FEEDBACK-TO-ENERGY-VIDEOS-AND-RENDER-FINALS/narration-guard.sh
NARRATION_GUARD_FILTER=heat-flow,volume bash …/narration-guard.sh     # only scripts whose path contains one of the substrings
```

Checks per clause and language: balanced `$`; every math segment speakable in de AND en, convertible to Unicode and
accepted by the caption typesetter; no spelled symbol names (`greek-name`, `dot-spelling`, `air-capacity`,
`balance-index`, `surface-resistance`) and no Unicode Greek letters outside `$…$`. It is registered in `📜️script.ts`
and red until every clip script is converted.

## 5. Files

- New: `📝️narration/🐍️.py` (segments, wrap clusters, line chunks, plain text), `📝️narration/🧮️tex/🐍️.py`, `🔣️.json`, `🧬️schema.json`
  (parser, symbols, Unicode), `🔊️speech/🗣️lexicon/🐍️.py`, `🔣️.json`, `🧬️schema.json`.
- Tests: `📝️narration/🧪️tests/🧮️tex`, `🪟️caption-layout`, `🛡️guard`, `🔊️speech/🧪️tests/🗣️lexicon` (each with `🔣️.cases.json`
  where data-driven), `🎨️visuals/🧪️tests/🌍️captions` (math clause case).
- Changed: `🎨️visuals/🐍️.py` (caption layout, symbol table from JSON, `\left`/`\right`, read-time and timing use printed/spoken words),
  `🔊️speech/🐍️.py` (spoken clauses, recording identity), `📜️script.ts` (test list),
  `📦️packages/🐍️python/pyproject.toml` + `uv.lock` (`pylatexenc==2.10`, test group only).
- Caption layout: words and formulas share one baseline; a formula never splits; punctuation glued to a formula stays with it;
  a third line still raises `ValueError` as before.
