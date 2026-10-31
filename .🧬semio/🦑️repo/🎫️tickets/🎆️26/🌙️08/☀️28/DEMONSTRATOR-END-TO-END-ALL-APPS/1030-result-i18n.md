# 1030 result i18n
STATUS: DONE. Rust notice rows not compiled: kernel crates are not workspace members so `cargo test -p` is refused; array length verified by hand (96 rows = declared 96).

## How the codebase localizes
- Rust plugins: `LocalizedLabel::native(en, de)` and a wire schema (`semio:locale/localized-label`) that requires every (terminology x locale) cell, so a missing German cell cannot compile or validate. Fault notices: `FRAMEWORK_FAULT_NOTICE_LABELS` (Rust, TS twin, fixture `kernel/🧪️tests/🧫️fixtures/🧫️framework-notices`) plus per-app `fault_notices()`. A fault with no notice shows the shell's generic localized refusal, never a raw code.
- TS page chrome: `registerUiTranslationBundles({en, de})`.

## Findings and fixes
1. Demonstrator page texts were inline bundles plus `locale === "de" ? .. : ..` ternaries in footer.tsx (duplicated against brand.ts captions). FIXED: one schema-first catalog `🔨️modules/🌐️locale/🔣️.json` (+ schema, typed accessor `🟦️.ts`, registration `⚛️.ts`); landing, card, footer and brand footer items read it. DEMONSTRATOR_LOCALE derives from the catalog's `lockedLocale`.
2. Accessibility: card button now has a short accessible name ("Demonstrator X öffnen"); footer links/buttons carry localized aria-labels incl. "opens in a new tab"; credit group has role=group; decorative logo imgs/svg are alt=""/aria-hidden (name comes from the link); partner names have real English names; project name is a constant. Icon-only footer keeps its name via aria-label (shell-brand test updated, long-level only, not run).
3. Framework fault notices added (Rust + TS + fixture, 87 -> 96 rows): task-resume-command/queue/delivery, admission-capacity, publication-stalled, stale, media-output-limit, artifact-envelope ingress-saturated and stale-publication. German uses the neutral infinitive; existing table mixes "Versuche" (du) and infinitive - style debt, not touched.
4. Audit result with no change needed: the three composed manifests (demonstrator, energy, fem): 0 blank/malformed label cells (~3900 cells); 974 `LocalizedLabel::native` pairs in the 8 pane plugin trees: 0 blank, 0 untranslated-identical.

## Open gaps (need a framework schema change, not done: wire types shared with TS/generated schemas, peers editing, cargo-heavy)
- `IntroductionInteraction.label` is a plain String: puzzle3d manifest ships English-only "Zoom", "Pan", "Orbit", "Add an object", "Activate Transform" (the demonstrator brand tour overrides them with German, other shells show English).
- `MediaPortSpec.label` ("Kit Catalog", puzzle/block) and `ArtifactPresentation.name` ("3D Puzzle", "Sourcing Curation") are plain Strings.
- `WindowLayoutWindowNode.title` ("Structure Classic", "Energy simulation", "Energy results") is a plain English fallback; the renderer overrides it with the localized window-kind label, so not user-visible today.
- Brand content (`🪧️brand.ts` introduction, pane label/tagline/description, tutorial) is German-only plain strings: allowed by the explicit German lock, but an English visitor can never be served; needs localized `IntroductionDefinition`/`ShellBrand` text.
- Framework i18n instance has `fallbackLanguage: "en"` and `detectShellLocale` defaults to en (implicit default); the demonstrator is protected by its explicit lock only.
- ~240 internal `interactive-job.*`/`artifact-envelope.*`/`artifact-store.*` invariant codes have no notice (generic refusal shown); deliberate.

## Tests (language-agnostic, one `🧫️cases.json`)
- `🧪️tests/🧪️demonstratorlocale/🟦️.ts` (vitest, registered in `📜️script.ts`; Ajv oracle for the catalog schema) and `🐍️.py` (`python3 -I`, stdlib; jsonschema oracle if installed): every catalog key has en+de, same placeholders, normal/beginner pairs, html lang == lock, no literal alt/aria-label/label or locale ternary in components, manifests' label cells, framework notice rows, plugin pair scan with minimum counts (no silent zero).
- Registered in project.json (both entry lists, test inputs now include the plugin `*.rs`, manifests, notice fixture) and compileclosure cases + schema (maxItems 16 -> 17).

## Verification
- Python: 8 tests, OK (1 skipped: jsonschema absent).
- `repo test @semio-tech/mit-bestand-demonstrator`: 72 of 73 pass incl. all 9 new tests and the compile-closure entry test. The one failure ("does not link whole puzzle/procedural/gis crates") is the peer-owned `🌎️hub/🧩️compositions/🎪️demonstrator/📦️packages/🦀️rust/Cargo.toml` listing semio-hub-puzzle/procedural; not mine.
- Cargo (Rust notice table size/rows): see last line.
- Cargo: kernel-history lib test queued (detached, rc marker scratchpad ck.rc); first two launches failed on CLI syntax only, third is queued.
