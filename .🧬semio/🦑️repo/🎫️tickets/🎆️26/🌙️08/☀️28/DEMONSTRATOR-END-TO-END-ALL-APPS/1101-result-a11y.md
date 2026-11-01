STATUS: EDITS DONE, PARTLY VERIFIED. Bun transpile of all six edited TS/TSX files parses; a bun render check of the footer markup passes in en and de. vitest, tsc and the browser pass are UNVERIFIED (no repo commands in this lane). Standalone dev-shell partner credits NOT added (see Open).

Root `D` = `♻️mit-bestand/🧺️demonstrator`. Lane `a11y`, brief `1101-brief.md`. Fixes the uifix leftovers M5, L1, L2, M2 from `1101-audit-ui.md` and the dev-shell window title.

## What changed

- M5 landmarks and heading (`D/🟦️.tsx`): the navbar brand is now `<h1 data-slot="app-name">` (was a `<span>`, `m-0` keeps the look). The overview is wrapped in `<main className="h-full w-full">`.
- L1 small text: card tagline and paragraphs `text-xs` to `text-sm` (`D/⚛️demonstrator-card.tsx`). Footer credit captions `text-2xs` to `text-xs` (`D/⚛️footer.tsx`).
- L2 clipping (`D/🌐️.html` body and `D/🏗️builder/🌐️vite/🟦️.ts` `bodyClass`): `overflow-hidden` to `overflow-auto`, so content that does not fit the viewport scrolls instead of being cut.
- M2 partner table dedupe (`D/⚛️footer.tsx`): the hard-coded LUH/UdK/ZukunftBau logo paths, URLs and the 50-line inline Zukunft Bau SVG are gone. Both credits now read `demonstratorFooterItems(locale)` from `D/🪧️brand.ts`, the single partner table. Logo, link and alt text come from that table. The Zukunft Bau credit now uses the shared PNG logo (light and dark) instead of the inline SVG, so it shows a dark variant too. Exports `LUH_LOGO_URL`, `UDK_LOGO_URL`, `LUH_URL`, `UDK_URL`, `ZUKUNFT_BAU_PROJECT_URL` and the logo components were removed (L4).
- Test `D/🧪️tests/🏛️shell-brand/🟦️.ts`: the URL constants it imported are now derived from the shared table at the top of the file. Its assertions are unchanged.
- Standalone dev shell window title (`D/🐚️shell/🟦️.ts`): after boot, `document.title` is the pane's localized `windowTitle` (for example "Entwerfen mit Bestand · Energie") when the viewer's language is resolved (stored choice or browser language). Without a resolved language it stays the neutral project name. No language is assumed.

## Verification

- `bun build --no-bundle` on `⚛️footer.tsx`, `🟦️.tsx`, `⚛️demonstrator-card.tsx`, `🐚️shell/🟦️.ts`, `🧪️tests/🏛️shell-brand/🟦️.ts`, `🏗️builder/🌐️vite/🟦️.ts`: all parse ("Transpiled file"). The ENOENT after it is bun failing to write its output path, not a parse error. Logs: `C/🗑️generated/1101/a11y-transpile-*`.
- Bun render check (script `scratchpad/a11y/check-footer.ts`): `aProjectOfLuhUdkFooterItem` and `fundedByZukunftBauFooterItem` render "A project of", "Funded by", a `<button>`, and "(opens in a new tab)" labels in en. In de icon-only mode the LUH label is "Leibniz Universität Hannover (öffnet in einem neuen Tab)" and the caption is absent. No `text-2xs` left in the footer.
- Locale fixture `D/🧪️tests/🧪️demonstratorlocale/🧫️cases.json` forbidden patterns checked by reading: no literal `aria-label`/`title` attributes added, no `windowTitle: "` literal, no `locale = "de"` defaults, `<title>Entwerfen mit Bestand</title>` unchanged.
- Not run: `repo tools exec tsc`, vitest, the Playwright acceptance spec, serve/test-e2e. Type-level claims (for example `logo.darkSrc` optional, `logo.alt` string on `ShellFooterItem`) are UNVERIFIED until tsc runs.

## Open / not done

- Standalone dev-shell partner credits: NOT added. The pane brands carry `footerItems` (`D/🪧️brand.ts:896`), but no consumer of `footerItems` was found under `🧰️framework/🔨️modules` or `🧰️framework/🛍️products/💻️os/🔨️modules`. The static dev-shell brands (`DEMONSTRATOR_SHELL_BRANDS`, also used by the vite config) have no footer, and the shared credit alt text is locale-bound. Whether the shell renders credits at all needs the framework owner; not a small change in this lane.
- Landmarks: the footer credits sit inside `<main>` (LayeredOverview renders its chrome inside it), so they are generic groups, not a `contentinfo`. The navbar is not a `banner`. The `<h1>` and `<main>` are the scope of this fix.
- Rendered behaviour unverified in a browser: `<h1>` styling, `<main>` layout with `h-full`, body scrolling on short viewports, card text at `text-sm` not overflowing the 1fr rows, Zukunft Bau PNG at `h-tiny`.
- The uifix open item "M2 partner-table dedupe" is done (see above). M3 (mobile icon-only credits) was already wired by uifix through `viewportMobile`.
