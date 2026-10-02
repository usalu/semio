# 🗒️ UI notes for the agent "deploy"

Written by the agent "ui" (React client, its tests, the site shell). Status at the bottom.

## 1. Content-Security-Policy: the client needs nothing beyond the policy you ship

Checked against the policy the coordinator described (`script-src 'self'` + boot hashes, `style-src 'self'` + boot hash,
`style-src-attr 'unsafe-inline'`, `img-src 'self' data:`, `font-src 'self'`, `connect-src` proctor https + wss,
`form-action 'self'`, `base-uri 'none'`, `object-src 'none'`). Read, not run in a sealed build:

| What | Finding |
|---|---|
| `<style>`/`<script>` elements created at runtime | None. Grep over the quiz target and the slim UI subset it imports (`@semio-tech/ui-react/chrome`, `/i18n`: Navbar, WindowChrome, OverviewCard, LayeredOverview, Icons, appearance, i18n port) for `createElement("style"|"script")`, `insertRule`, `adoptedStyleSheets`: no hit. The only `createElement` is a `<canvas>` (poster capture, unused by the quiz). |
| Styles set by script | Only through the CSSOM (`element.style.setProperty`, React `style` props, `classList`): the text-size preference (`--quiz-text-scale` on `<html>`), theme class `dark`, peer colours (`--quiz-peer`, `--quiz-peer-ink-light`, `--quiz-peer-ink-dark`), layered overview transforms, the drag ghost. CSP does not restrict CSSOM writes; nothing calls `setAttribute("style", …)`. |
| Markup injected as HTML | `ShellBrandLogo` and `Icon` set first-party SVG markup with `dangerouslySetInnerHTML`. The emblem and every generated icon contain no `<style>`, no `<script>` and no `style="…"` attribute (grep: 0), so neither `style-src` nor `script-src` is touched. |
| Connections | `fetch` to the baked proctor origin (`/commands`, `/queries`) and WebSockets to the same host (`/scopes/…/presence/ws`): covered by your `connect-src https://… wss://…`. Nothing else is contacted. |
| Images and fonts | Same origin only; `data:` is not needed by the quiz (posters are off). |
| Forms | One `<form>` (identity step) with `onSubmit` + `preventDefault`; it never navigates. `form-action 'self'` is more than enough. |
| Links | Imprint and privacy policy are plain `<a href target="_blank" rel="noopener noreferrer">` navigations; rendered only for an absolute `http(s)` URL (the client parses with `URL` and refuses every other scheme). |
| `document.title`, `<html lang>` | Set from script (`document.title = …`, `documentElement.lang`); not a CSP matter. |

Nothing to widen. One thing you could tighten later, not required: after §2 the built document has inline `style="…"`
attributes only in the loading placeholder and the `<noscript>` paragraphs of the shared host template; if the template moved
those two into its boot `<style>` block, `style-src-attr 'unsafe-inline'` could go. I did not do that because the boot style
is shared byte-for-byte with every other semio host.

## 2. What I change in the host document (your CSP hashing keeps working unchanged)

The site's vite config (`🎓️teaching/🏛️architecture/❓️quiz/🏗️builder/🌐️vite/🟦️.ts`) is edited by both of us; my edits are small and
anchored on the `semioHostHtmlVitePlugin({ … })` spec and on `siteTitle`:

- **No default language in the document.** The shared host template (`🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts`,
  `SemioHostHtmlSpec`) accepts each text either as one string (as today, `<html lang="en">`) or once per offered language
  (`[{ lang, text }, …]`); a document whose title is given per language carries **no `lang` on `<html>`**, a title joined with
  ` · `, and loading/noscript parts each marked with their `lang`. The quiz passes the catalog's own title pair, so your
  release plugin's `html.replace(/<html lang="[^"]*">/u, "<html>")` becomes a no-op (dev and release now agree). You can
  drop that replace; leaving it is harmless.
- **`siteTitle`** becomes the catalog title in every language (`Architecture and Technology Quizzes · Quizze zu Architektur
  und Technologie`), taken from `🔣️.json`; your manifest `name` follows it. I set the manifest `short_name` to `Quiz` (the
  same word in English and German) instead of the German-only `Quizze`. Say so if the manifest should differ.
- **`<noscript>`**: two short paragraphs (English, German). No new `<style>` or `<script>` block: the paragraphs carry
  `style="visibility:visible"` attributes (the boot style hides `<body>` until a script reveals it), which
  `style-src-attr 'unsafe-inline'` admits like the existing loading placeholder.
- No inline script and no inline style block is added anywhere; the set of blocks your plugin hashes is unchanged.

## 3. `site.legal` in `🚀️deploy/🔣️.json`

Your schema already has the shape I need: `site.legal: { imprint?: https URL, privacy?: https URL }`. I add `"legal": {}`
(both unset: the owner must supply the URLs). The site entry (`🟦️.ts`) passes `deployment.site.legal` to
`mountQuiz(…, { legal })`; the footer renders a link only when its URL is set, and the built-in notice "What is stored /
Was gespeichert wird" (a dialog, no URL of its own) is always there. A named import (`import { site } from …json`) keeps
the proctor image name out of the bundle; the proctor host is in the bundle anyway (`VITE_PROCTOR_URL`).

Please add to the operator README one line: the two URLs are the owner's one-time step before the public launch
(`site.legal.imprint`, `site.legal.privacy`), and a rebuild of the site is needed after setting them.

## 4. Things the client now relies on from the edge

- `429` with `Retry-After` (delay-seconds or an HTTP-date; capped at 120 s client-side): every retry path waits that long plus
  jittered backoff; without the header the client backs off on its own (0.5 s … 15 s for commands and loads, 10 s … 5 min for
  the leaderboard poll, 0.5 s … 15 s between presence rejoins).
- The client reads the `Retry-After` header of `429` and of `503` answers; "edge" already sends
  `Access-Control-Expose-Headers: Retry-After` and the CORS grant on both (its notes, §3). When the header is hidden or
  missing the client takes `retryAfterMs` from the JSON body edge sends, and without either it backs off on its own.
  A `429`/`503` without the CORS grant would look like "no answer" to the browser — the indicator would then say
  "Connection lost" instead of "Server busy"; keep the grant on those answers.

## Status

- 2026-10-02 02:10 — written. Host document and `site.legal` edits follow; I update this file when they are in.
- 2026-10-02 — final. Everything of §2 and §3 is in the tree: `quizHostDocument` (exported from the site's vite config),
  `site.legal: {}` in `🚀️deploy/🔣️.json`, `legal: site.legal` in the site entry. The node test
  `🧪️tests/📰️host-document/🟦️.ts` pins it (no `lang`, title pair, noscript, no additional inline block, legal URLs unset
  or `https://…`). Checked in a browser against the dev server (not against a sealed release build — that is your gate).
- For your runbook: a dev proctor refuses a data folder written before the storage format changed
  (`proctor.sqlite holds format … v1; this proctor reads … v2`); `.🧬semio/🎓️teaching/proctor-dev` on this machine is such a
  folder. I did not touch it; I ran my check with `PROCTOR_DATA` pointing at a scratch folder.
