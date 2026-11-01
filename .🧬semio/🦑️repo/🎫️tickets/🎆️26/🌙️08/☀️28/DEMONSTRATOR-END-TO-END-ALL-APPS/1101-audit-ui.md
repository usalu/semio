# 1101 Audit: UI of `♻️mit-bestand/🧺️demonstrator` (lane aud-ui)

Read-only audit. No source files edited, no cargo, no servers, no sub-agents.
Scope: landing page, footer, demonstrator card, brand and pane definitions, locale catalog, pane-row consistency, launch entries.
Root for paths below: `/Users/ueli/Documents/semio/♻️mit-bestand/🧺️demonstrator/` (abbreviated `D/`).

## Verdict

The demonstrator violates the multi-language rule in its core: the whole page and all eight panes are hard-locked to German, there is no English runtime path, and two places carry a German default. The catalog itself is complete (126 keys, every key has an `en` and a `de` cell, no empty cells, English stored first). Pane rows agree across `brand.ts`, the runtime catalog and the acceptance spec. Two of the brief's cross-check targets do not exist: the hub `Cargo.toml` has no per-pane playground rows, and there is no `.vscode/launch.json`.

## Findings (file:line)

### High: no multi-language support, German lock and German default

H1. Whole demonstrator is locked to German.
- `D/🔨️modules/🌐️locale/🔣️.json:3` `"lockedLocale": "de"`.
- `D/🔨️modules/🌐️locale/🟦️.ts:16-17` `DEMONSTRATOR_LOCKED_LOCALE` is derived from that single value; `:23-25` `demonstratorText` has no fallback, so the lock is the only language the UI can show.
- `D/🪧️brand.ts:33-36` `DEMONSTRATOR_LOCALE = DEMONSTRATOR_LOCKED_LOCALE`, used by all 8 brands at `:366, :526, :577, :632, :677, :728, :796, :847` (`locks: { locale: DEMONSTRATOR_LOCALE, ... }`) and by all 8 pane rows (`:921` to `:994`).
- `D/🟦️.tsx:49-51` and `:51` `initUiLocaleSync(DEMONSTRATOR_LOCALE)` pins the landing page to German before first render.
- `D/README.md:3` documents the page as "a German-locked landing page".
- `D/🔨️modules/🌐️locale/🧬️schema/🔣️.json:5` and `:11` schema makes `lockedLocale` an enum over `en`/`de`, so the lock is part of the contract, not a leftover.
- Consequence: the English cells in the catalog are dead data at runtime. No language switch exists (see M6).

H2. German default parameters in the footer.
- `D/⚛️footer.tsx:112` `fundedByZukunftBauFooterItem(key, locale: UiLocale = "de", ...)`.
- `D/⚛️footer.tsx:138` `aProjectOfLuhUdkFooterItem(key, locale: UiLocale = "de", ...)`.
- Violates "no default language". Both are called with `DEMONSTRATOR_LOCALE` at `D/🟦️.tsx:150-151`, so the default is never used today, but it is a latent German default.

H3. Hard-coded German-only literals that bypass the catalog.
- `D/🪧️brand.ts:363, :523, :574, :629, :674, :725, :793, :844` `windowTitle: "Entwerfen mit Bestand · <Pane>"` (German suffixes such as "Koordinator", "Aussuchen", "Verfolgen", "Statik" are literal strings, not `demonstratorLabel` cells).
- `D/🪧️brand.ts:364, :524, :575, :630, :675, :726, :794, :845` `shortWindowTitle: "EmB · Agg" / "EmB · Gen" / ... / "EmB · Statik"`. These also feed `shortLabel` in the landing (`D/🟦️.tsx:97`, `:99`), so the short labels stay German even if the locale changed.
- `D/🪧️brand.ts:61` `ENTWERFEN_MIT_BESTAND_LOGO_SVG` carries `aria-label="Entwerfen mit Bestand"` as a literal. The project name is a proper noun (`:58`), acceptable as a literal, but the SVG aria-label is not localized anywhere else.

H4. Document language and title are German-only.
- `D/🌐️.html:2` `<html lang="de">` is static.
- `D/🌐️.html:6` `<title>Entwerfen mit Bestand · Demonstrator</title>` has no English variant.
- Once a second locale exists, `lang` must follow the active locale; today it is hard-wired.

### Medium

M1. The acceptance spec and locale test enshrine the German lock.
- `D/🧪️tests/🎭️acceptance/🟦️.ts:479` `toContainText("Demonstrator öffnen")` asserts the German card label only. No case exercises English.
- `D/🧪️tests/🧪️demonstratorlocale/🟦️.ts:67-68` asserts `DEMONSTRATOR_LOCKED_LOCALE === catalog.lockedLocale` and `<html lang="${lockedLocale}">`, i.e. it tests the lock as the intended behavior.
- `D/🧪️tests/🧪️demonstratorlocale/🐍️.py:65-66` same assertion, cross-language.
- Positive: `:762` (`/^(Play|Abspielen)$/`) and `D/🧪️tests/🎭️acceptance/🟦️.ts:262` (`/Keine Auswahl|No selection/`) already accept both languages, so the spec is bilingual where it matters; only the card label is German-only.

M2. Footer partner data is duplicated and partly hard-coded.
- `D/🪧️brand.ts:22-30` `DEMONSTRATOR_FOOTER_ITEMS` (used by all 8 shell brands at `:361, :521, :572, :627, :672, :723, :791, :842`) and `D/⚛️footer.tsx:64-102` (LUH/UdK/ZukunftBau logo paths and URLs) describe the same partners twice. Drift risk.
- `D/⚛️footer.tsx:64, :66, :83, :85` hard-code `/♻️mit-bestand/🧺️demonstrator/🖼️asset/...` instead of using `DEMONSTRATOR_ASSETS_DIR` from the runtime catalog (`D/🔨️modules/🧩️runtime/📇️catalog/🟦️.ts:4`, re-exported by `D/🪧️brand.ts:5-6`). The catalog's `assetsDirectory` and the footer can diverge silently.
- URLs repeated: `D/⚛️footer.tsx:68, :87, :102` and `D/🪧️brand.ts:24, :25, :28, :105`.

M3. Footer mobile behavior is documented but not implemented.
- `D/⚛️footer.tsx:110` and `:136` say "On mobile the credit shrinks to just its logo(s)".
- `D/⚛️footer.tsx:112` and `:138` take `iconOnly = false` by default; neither footer applies a responsive class, so the only way to shrink is the caller.
- `D/🟦️.tsx:150-151` always passes `false`. On a phone both credit groups show full captions, and the reserved bottom inset `D/🟦️.tsx:70` (`5.5rem`, "two rows of logos on a phone") was sized for that. Either implement the shrink or fix the comments.

M4. Tablet layout is not detected from the viewport.
- `D/🟦️.tsx:111` `viewportMobile = useMediaQuery(UI_MOBILE_MEDIA_QUERY)`.
- `D/🟦️.tsx:116` `device: viewportMobile ? "mobile" : readStoredUiChromeLayout(...) === "tablet" ? "tablet" : "desktop"`. A tablet (not mobile by the query) is reported as `desktop` unless the user stored `tablet` in browser storage. There is no tablet breakpoint in the demonstrator.
- `D/🟦️.tsx:112` `DEMONSTRATOR_SWIPE_MEDIA_QUERY` (mobile and touch) decides swipe mode; any touch tablet above the mobile breakpoint gets the 4x2 strip (`D/🟦️.tsx:191`).
- The breakpoint value (`UI_MOBILE_MEDIA_QUERY`) was not located in this lane (ui-react source not inspected), so the exact tablet range is UNVERIFIED.

M5. Landing has no landmark or heading structure.
- `D/🟦️.tsx:159-177` the navbar brand is a `<span data-slot="app-name">` (`:168`), not a heading; no `<h1>` and no `<main>` on the landing.
- `D/🟦️.tsx:184-196` the card grid is the framework `LayeredOverview`, whose grid is `role="group"` with `aria-label` (`D/../🧰️framework/🔨️modules/🖱️ui/🧱️elements/🥞️LayeredOverview/🟦️.tsx:55`). Screen-reader users get no page-level heading to navigate by.
- Footer partner groups are `role="group"` with labels (`D/⚛️footer.tsx:143`), which is good; the funded-by control is a `<button>` with an "opens in a new tab" label (`D/⚛️footer.tsx:117-120`), also good.

M6. Customization is not exposed on the landing.
- `D/🟦️.tsx:43` and `:115-117` read stored appearance, layout and driver from browser storage only.
- `D/🟦️.tsx:159-177` the navbar offers only the label; `:175` `showFullscreenToggle={false}`. There is no visible control for appearance, layout, driver or language.
- Language cannot be customized at all (H1).

M7. Keyboard parity for the hover emphasis is missing.
- `D/🎨️globals.css:6-9` paragraph emphasis is `:hover` only.
- `D/🎨️globals.css:11-18` title-chip emphasis is driven by `:has(...:hover)` only. No `:focus-visible` or `:focus-within` equivalent, so keyboard focus on the card button does not reveal the same emphasis.

### Low

L1. Small text sizes.
- `D/⚛️demonstrator-card.tsx:37` tagline `text-xs`; `:43` paragraphs `text-xs text-muted-foreground`.
- `D/⚛️footer.tsx:121`, `:143` credit text `text-2xs`.
- `--text-2xs: 0.6rem` (about 9.6 px) is defined at `D/../🧰️framework/🔨️modules/🖱️ui/🎨️styling/🖌️ui/🎨️.css:886`. Footer credit captions at 9.6 px are below a comfortable reading size, especially on phones.
- `text-xs` and the contrast of `text-muted-foreground` against the background were NOT verified in this lane.

L2. Page root clips overflow.
- `D/🌐️.html:16` `<body class="h-screen w-screen overflow-hidden ...">`. Combined with a fixed 2-row grid (`D/🟦️.tsx:62`, `minmax(min(10rem, 100%), 1fr)`) this can clip content on short or zoomed viewports. Only the strip mode scrolls (`D/🟦️.tsx:105`). UNVERIFIED in a browser (not run).

L3. Pane naming differs between sources.
- Brand ids are `energie` and `statik` (`D/🪧️brand.ts:948`, `:988`); their runtime variants are `energy` (`:949`) and `fem3d` (`:989`).
- README lists the eight panes by variant (`D/README.md:3`: `energy`, `fem3d`), while the acceptance spec and the URL hash use the brand ids (`D/🧪️tests/🎭️acceptance/🟦️.ts:225`, `:276`; `/#aussuchen` at `:552`).
- Runtime catalog order differs from brand order: JSON lists `energy` seventh (`D/🔨️modules/🧩️runtime/🔣️.json:30-33`), brand lists `energie` fourth (`D/🪧️brand.ts:947`). Not a functional bug (`pane roster` test compares the acceptance list to brand order only).

L4. Unused exports and stale header.
- `D/⚛️footer.tsx:13` `ZukunftBauLogo`, `:71` `LuhLogo`, `:90` `UdkLogo` are exported but only used inside `footer.tsx`; `:64-68`, `:83-87` URL and logo constants are exported as well. Remove the exports (or the constants) to keep the surface honest.
- `D/⚛️footer.tsx:2` describes the file as "Aggregator's shell-chrome" credits; it is the landing footer for all panes.

## Pane-row cross-check (8 rows)

Source of truth: `D/🪧️brand.ts:916-997` (`DEMONSTRATOR_PANES`).

| Pane id (brand `:918-988`) | Brand variant | Runtime `variant` / `runtimeVariant` (`🔣️.json`) | Brand `id` (`ENTWERFEN_MIT_BESTAND_*_BRAND`) | Acceptance `PANE_CASES` | README |
|---|---|---|---|---|---|
| generator | generator | generator / generation3d | entwerfen-mit-bestand-generator | yes (`:195`) | yes |
| koordinator | koordinator | koordinator / koordinator | entwerfen-mit-bestand-koordinator | yes (`:207`) | yes |
| aggregator | aggregator | aggregator / aggregator | entwerfen-mit-bestand-aggregator | yes (`:221`) | yes |
| energie | energy | energy / energy | entwerfen-mit-bestand-energie | yes (`:225`) | as `energy` |
| aussuchen | aussuchen | aussuchen / aussuchen | entwerfen-mit-bestand-aussuchen | yes (`:254`) | yes |
| bearbeiten | bearbeiten | bearbeiten / bearbeiten | entwerfen-mit-bestand-bearbeiten | yes (`:273`) | yes |
| verfolgen | verfolgen | verfolgen / verfolgen | entwerfen-mit-bestand-verfolgen | yes (`:274`) | yes |
| statik | fem3d | fem3d / fem3d | entwerfen-mit-bestand-statik | yes (`:276`) | as `fem3d` |

Result: all 8 variants match between `brand.ts` and `🔣️.json` (`D/🔨️modules/🧩️runtime/🔣️.json:6-37`); the acceptance roster equals brand order, enforced by `D/🧪️tests/🎭️acceptance/🟦️.ts:451-453`, which parses `brand.ts` as text (`:24-29`). `ENTWERFEN_MIT_BESTAND_BRAND_IDS` (`D/🪧️brand.ts:39-48`) lists the same 8 brand ids. Runtime build-time guards (`D/🔨️modules/🧩️runtime/🟦️.ts:16-17`, `:48`) throw on unknown variants. Naming and order drift only (L3).

README pane count and test count agree: README says twenty Playwright cases (`D/README.md:75`); the spec has 12 static `test(` blocks plus the 8-case `for` loop at `:486-487`, which is 20.

## Launch entries and hub Cargo

- `.vscode/launch.json`: NOT FOUND. `/Users/ueli/Documents/semio/.vscode` does not exist, `git ls-files` contains no `.vscode` path, and a depth-3 search found no `launch.json`. The brief's demonstrator launch entries therefore cannot be checked; either they live in an uncommitted location or the brief is out of date.
- `🌎️hub/Cargo.toml` (`/Users/ueli/Documents/semio/🌎️hub/Cargo.toml`): no per-pane playground rows. Demonstrator-related lines are only `:69-70` (`🧩️compositions/🎪️demonstrator` members), `:100-101` (energy composition), `:282` (`semio-s-artifact-demonstrator-playground`), and the fem/energy/generation3d artifact crates (`:310-313`, `:326`, `:330-339`). The per-variant `PLAYGROUND_BUILD_TARGETS` registry is generated (`@semio-tech/s-dev-catalog-ts/generated/playgrounds`) and is not tracked in git, so the playground side is UNVERIFIED.

## Positive checks

- Locale catalog: 126 keys, all with non-empty `en` and `de`, schema-enforced (`D/🔨️modules/🌐️locale/🧬️schema/🔣️.json:20-32`).
- Every pane description has exactly 2 paragraphs in both languages, matching the acceptance expectation at `D/🧪️tests/🎭️acceptance/🟦️.ts:478`.
- Every catalog key referenced in code exists (checked by literal scan; the `demonstrator.landing.*` and `demonstrator.card.*` keys resolve through the `.label.normal` / `.label.beginner` suffix).
- Partner logos ship light and dark variants and use `alt=""` only where the enclosing link carries an `aria-label` (`D/⚛️footer.tsx:74-75`, `:145-163`).
- Card: accessible name via `openNamed` label (`D/⚛️demonstrator-card.tsx:22`, `:30`), and the icon carries a title (`:32`).

## Not verified in this lane

- Rendered layout, contrast, focus rings and real breakpoints (no browser run; read-only lane).
- `UI_MOBILE_MEDIA_QUERY` value, `text-xs` size, theme colours (defined in ui-react and theme CSS outside this lane).
- Generated playground registry and icon names in `DEMONSTRATOR_PANES` (`IconName` type check needs a build).

## Suggested fixes (not applied)

1. Replace `lockedLocale` with a real locale state (default none, en first, de second per the brief); remove `DEMONSTRATOR_LOCKED_LOCALE`, the `"de"` defaults at `footer.tsx:112,138`, and the German-only `lang`/`title` in `🌐️.html`.
2. Move `windowTitle`/`shortWindowTitle` (`brand.ts:363-845` range) into `demonstratorLabel` cells; localize the SVG aria-label.
3. Derive footer logo paths from `DEMONSTRATOR_ASSETS_DIR` and reuse one partner table for `brand.ts` and `footer.tsx`.
4. Either implement mobile icon-only credits or drop the claim at `footer.tsx:110,136`; match the `5.5rem` inset to the result.
5. Add a tablet breakpoint or viewport-based `device`; add a visible language and appearance control on the landing; add a page `<h1>`/landmark; mirror hover emphasis on `:focus-visible`.
6. Add an English acceptance case for the card label (`acceptance/🟦️.ts:479`) once the lock is gone.
