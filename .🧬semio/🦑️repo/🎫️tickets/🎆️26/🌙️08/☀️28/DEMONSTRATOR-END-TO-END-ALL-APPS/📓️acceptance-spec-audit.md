# 📓️ Acceptance-Spec Audit — `🧪️demonstrator.acceptance.spec.ts`

Method: **static reading only**. No build, no `nx run ...:dev`/`:build`, no wasm-pack, no Playwright run
was executed for this audit — nothing below is "verified by running." Every claim is backed by grep/Read
of the file it cites, cross-checked against the ticket's own per-app research docs
(`📓️app-*.md`) and `📓️status.md` in this same ticket folder.

Files read in full: `♻️mit-bestand/🧺️demonstrator/🧪️demonstrator.acceptance.spec.ts`,
`♻️mit-bestand/🧺️demonstrator/🧪️playwright.config.ts`, `♻️mit-bestand/🧺️demonstrator/🟦️brand.ts`,
`♻️mit-bestand/🧺️demonstrator/🟦️.tsx`.

## 1. `brandPaneIds()` regex vs. current `🟦️brand.ts` shape

Static reading confirms the regex still matches. `DEMONSTRATOR_PANES` in `🟦️brand.ts:789-796` is:

```ts
export const DEMONSTRATOR_PANES: readonly DemonstratorPaneSpec[] = [
  { id: "generator", variant: "generator", brand: ENTWERFEN_MIT_BESTAND_GENERATOR_BRAND, ... },
  { id: "koordinator", ... },
  { id: "aggregator", ... },
  { id: "aussuchen", ... },
  { id: "bearbeiten", ... },
  { id: "verfolgen", ... },
];
```

Traced the regex `/export const DEMONSTRATOR_PANES[^=]*=\s*\[(.*?)\n\];/s` by hand against this text:
`[^=]*` consumes the `: readonly DemonstratorPaneSpec[] ` type annotation up to (but not including) the
first `=` — the `[]` inside the type annotation contains no `=`, so it does not break the class. `=\s*\[`
then matches `= [`, and the lazy `(.*?)\n\];` stops at the array's own closing `\n];` (no entry line ends
in `];`, since every pane is a single-line object literal, so there is nothing for the lazy match to
mis-stop on early). The subsequent `matchAll(/\bid:\s*"([^"]+)"/g)` then only matches the `id:` key
(never `variant:` or another field) on each entry line, in source order.

Result: `brandPaneIds()` yields exactly
`["generator", "koordinator", "aggregator", "aussuchen", "bearbeiten", "verfolgen"]`, in that order —
identical to `PANE_CASES.map(c => c.paneId)` in the spec (lines 146-193). **No regex change needed.**

## 2. Stale renamed-file references — found and fixed

`grep -rn "📦️index\.tsx\|🌐️index\.html"` and a broader `index\.tsx\|index\.html` sweep across the whole
`♻️mit-bestand/🧺️demonstrator/` directory (excluding `dist/`, `node_modules/`, `test-results/`) returned
**zero hits**. Those two literal old filenames are not referenced anywhere in the directory. The
`⚙️vite.config.ts` and `.storybook` mounts already say `🌐️.html`/`🟦️.tsx` correctly.

The one comment naming `🟦️.tsx` that the task flagged (spec header, line 6: `see \`🟦️.tsx\`'s
\`paneIdFromLocationHash\`/\`useSequentialPaneBoot\``) **is accurate** — both functions exist in
`♻️mit-bestand/🧺️demonstrator/🟦️.tsx` exactly as described (confirmed by reading that file in full).

However, broadening the same "stale renamed-file" check to every file citation in the spec (not just the
two literal names named in the task) turned up a **different, real** staleness class: seven comments cite
sibling framework component files as `.../🟦️component.tsx`, but every one of those files is actually
named `🟦️.tsx` (empty basename before the extension — the same naming convention the demonstrator's own
entry file uses, and per `📓️status.md`'s `~17:00` entry, commit `21fbcd3538` ran a repo-wide
`…component.tsx → 🟦️.tsx` rename sweep on 2026-09-02 that renamed the files without updating every
comment that named them). Verified each actual path and line number by grep against the real file, and
fixed all seven in place:

| Spec line (after fix) | Old (stale) citation | New citation | Verified against |
| --- | --- | --- | --- |
| 8 | `ShellHost/🟦️component.tsx` | `ShellHost/🟦️.tsx` | `#region 🔖️ReadinessBeacon` exists at lines 7025-7075 of that file |
| 54 | `framework/ui/elements/🆔️ElementId/component.tsx` | `framework/ui/elements/🆔️ElementId/🟦️.tsx` | `elementIdSegment` at line 27; spec's local mirror is a byte-identical copy |
| 92 | `ShellHost/🟦️component.tsx` | `ShellHost/🟦️.tsx` | same file as above |
| 127-128 | `ShellHost/🟦️component.tsx` lines ~6558-6591 | `ShellHost/🟦️.tsx` lines ~6687-6717 | `data-element-alias={childElementId(...)}` now at line 6717 (line numbers had also drifted, not just the filename — corrected both) |
| 208 | `World3dHost/🟦️component.tsx` ~line 4997 | `World3dHost/🟦️.tsx` ~line 5063 | `data-meshes-json`/`data-instances-json` now at lines 5063-5064 |
| 220 | `NodeGraph/🟦️component.tsx` line 1151 | `NodeGraph/🟦️.tsx` line 1154 | `data-fixture-json` now at line 1154 |
| 237 | `framework/ui/elements/📊️Table/🟦️component.tsx` lines 183/243 | `framework/ui/elements/📊️Table/🟦️.tsx` lines 201/262 | `data-row-id` now at lines 201/262 of `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📊️Table/🟦️.tsx` |
| 252 | `vite-elements-assets.ts`'s `createTileProxyMiddleware` | `framework/ui/🎨️styling/🟦️.ts`'s `createTileProxyMiddleware` | no file named `vite-elements-assets.ts` exists anywhere in the repo; `createTileProxyMiddleware` is actually defined (and used by `resolveGisMapTileServeMode`) in `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🟦️.ts`, the same module the demonstrator's own `⚙️vite.config.ts` imports `semioEmojiIndexHtmlVitePlugin` etc. from |

All seven edits are pure doc-comment fixes — no test logic, selector, or assertion changed. Confirmed
`git status --porcelain` on the spec file was empty before editing (no other session had it in flight),
and re-swept the file afterward for any remaining `component\.tsx`/`component\.rs`/old-filename hits —
none left.

One nuance deliberately **not** turned into a fix: the koordinator `note` string (spec lines 166 and
inline for the other three cad windows) says the `f394df99d4` fix is "NOT yet compile-verified
(`semio-s-plugin-cad` is blocked by peer `🏪️store` E0119 errors)". `📓️status.md`'s later entries in this
same ticket (`~14:1x` and `~16:00`, both same day) show `semio-s-plugin-cad` now compiles clean for
`wasm32-wasip2` and the demonstrator's whole dependency closure reached zero errors — i.e. the
"not yet compile-verified" framing is itself now stale, superseded by the ticket's own later status
entries. I left this narrative sentence untouched: it is a build-status claim, not a file-rename
reference (out of this task's stated scope), the ticket's status is still actively changing under
concurrent sessions (per the "plugin bundles are still building" framing given to me), and rewriting a
build-progress narrative I cannot verify by running anything risks planting a new, differently-stale
claim within the hour. Flagging it here so whoever finalizes this ticket can update that sentence once
the bundle rebuild actually lands and the four koordinator assertions get their first real run.

## 3. Navigation path

The spec navigates with `page.goto(\`/#${paneCase.paneId}\`, { waitUntil: "domcontentloaded" })` (line
301) — a request for `/` (root) with a client-side-only `#paneId` hash fragment, never sent to the
server. This is **correct for the app as it actually works today**, confirmed by reading
`♻️mit-bestand/🧺️demonstrator/🟦️.tsx`:

- `paneIdFromLocationHash()` (lines 63-67) reads `window.location.hash` — nothing else — to decide which
  pane to focus immediately on load.
- `focusPane()` (line 596) and the `hashchange` listener (lines 614-622) both read/write `#<paneId>`, not
  a path segment.
- There is **no path-based pane router** in this app (no `/generator` route handler) — a request to
  `/generator` (no hash) would 200 via the SPA-fallback rewrite in `semioEmojiIndexHtmlVitePlugin`
  (`🧰️framework/🔨️modules/🖱️ui/🎨️styling/🟦️.ts:625-663`, confirmed the plugin's `configureServer`/
  `configurePreviewServer` wire both a root rewrite and an SPA-fallback rewrite pointed at `🌐️.html`) but
  would land on the **overview** grid with nothing focused, since there is no hash to read.

So `/#${paneId}` is not a leftover from the old `/🌐️index.html` literal-path era — it is the one URL
shape that actually drives `paneIdFromLocationHash` to focus a single pane and take the documented "fast
path" (skip the 1.5s/35s sequential-boot queue) the header comment describes. No change needed here; the
suite already navigates the way the app is actually routed. `🧪️playwright.config.ts`'s `baseURL`
(`http://127.0.0.1:6029/` by default, from `MIT_BESTAND_DEMONSTRATOR_PORT`) resolves `/#generator` to
`http://127.0.0.1:6029/#generator`, matching the dev server already running on port 6029.

## 4. Per-test verdicts and the ready-now / needs-plugins split

Static reading confirms every `kindId`/`surface` pair in `PANE_CASES` (generator, koordinator,
aggregator, aussuchen, bearbeiten, verfolgen) against this ticket's own `📓️app-*.md` per-app fixture
audits — all six matched exactly (window kind ids, `SurfaceKind`, and the documented known-gap notes for
generator's `procedural-preview` and aussuchen's `sourcing-preview`). The four DOM surface helpers
(`worldContentCount`, `nodeGraphWidgetCount`, `tableRowCount`, `tiledMapHasVisibleContent`) were checked
against the real host components and all four `.semio-*-host`/`.semio-*-empty` class pairs and their
`data-*-json`/`data-row-id` attributes exist exactly as the docstrings (now) describe.

| # | Test name | Verdict | Why |
| --- | --- | --- | --- |
| 1 | `DEMONSTRATOR_PANES matches the pane ids this suite covers (drift guard)` | **ready now** | Pure static comparison of `PANE_CASES` ids against `brandPaneIds()`'s text-regex read of `🟦️brand.ts` — no page load, no server, no plugin involved at all. Regex confirmed correct in §1 above. |
| 2 | `demonstrator pane "generator": …` | **needs plugins** | Requires the `generator`/`generation3d` plugin to boot to a `data-shell-ready` outcome before any window assertion runs. Even once plugins load, the `procedural-preview` window's `expectContent: true` assertion is a documented KNOWN GAP (edit-mode `render()` always builds a fresh, never-ticked `FlowEvalSession`, so `eval_json` stays empty — `📓️app-generator.md` §3) — this sub-assertion is expected to keep failing until that gap is fixed, independent of the routing/bundle work. |
| 3 | `demonstrator pane "koordinator": …` | **needs plugins** | Same shell-boot dependency. All four cad windows (`cad-play-shape/building/energy/structure-classic`) assert non-empty content; per `📓️app-koordinator.md` and `📓️status.md`, the underlying fix (`f394df99d4`, wiring `cad_pane_working_scene` through `ArtifactChild::local_owner`) is committed and, per later status entries, now compiles clean — but this test still needs a fresh plugin bundle (not the stale Aug 27 one) actually served before it can be the "first real proof" the spec's note describes. |
| 4 | `demonstrator pane "aggregator": …` | **needs plugins** | `puzzle3d-main`'s split top/perspective instances need the puzzle plugin booted; no documented known gap for this pane, so once a fresh bundle is served this test is expected to pass outright. |
| 5 | `demonstrator pane "aussuchen": …` | **needs plugins** | Needs the sourcing plugin booted. `sourcing-curated` (`expectContent: false`, empty-by-design) and `sourcing-preview` (`expectContent: false`, documented framework-wide selection-threading gap, `📓️app-aussuchen.md` §5) are correctly pre-weakened — `sourcing-pool` and `sourcing-grid` are asserted non-empty and, per the research doc, should be populated once the plugin loads. |
| 6 | `demonstrator pane "bearbeiten": …` | **needs plugins** | Needs the process plugin booted for `process-workpiece`; no documented known gap — expected to pass once a fresh bundle is served. |
| 7 | `demonstrator pane "verfolgen": …` | **needs plugins** | Needs the gis plugin booted for `gis2d-main`, plus the same-origin tile proxy (`createTileProxyMiddleware`, now correctly cited — see §2) actually serving map tiles so the canvas paints more than one flat color; no documented known gap otherwise. |

**Split, stated plainly**: only test 1 (the drift guard) is exercising anything that exists right now
independent of plugin state — it reads a source file as text and does simple array comparison. All six
per-pane tests are gated on `waitForPaneShellOutcome` reaching `"ready"` (spec lines 93-108), which is a
property of the booted WASM plugin, not of the routing fix. The routing fix (🌐️.html /🟦️.tsx rename,
SPA fallback) makes `GET /` and the six `/#<paneId>` deep links reachable and hash-focusable, which is a
necessary precondition for tests 2-7 to even reach the shell-readiness wait — but it is not sufficient;
per `📓️status.md`'s running log, the plugin catalog bundle build was still in progress as of the latest
entries there (stdio failing inside a peer's live brep refactor as of `~17:15`), so tests 2-7 cannot be
expected to pass yet. This matches the task's own framing exactly — I did not independently discover this
state, I corroborated it against `📓️status.md`'s own entries via static reading.

## 5. Deliberately not touched, and why

- **The `brandPaneIds()` regex itself** — already correct against the current `🟦️brand.ts` shape (§1);
  changing it would be unmotivated churn.
- **`/#${paneId}` navigation** — already correct against the app's actual (hash-based) routing (§3);
  switching to a bare `/<paneId>` path would silently break every test (the app would land on the
  overview with nothing focused, and `waitForPaneShellOutcome` would time out against a shell that was
  never told which pane to boot).
- **The koordinator "NOT yet compile-verified" narrative sentence** — see the nuance note at the end of
  §2. Left as-is; flagged for whoever closes this ticket to update once the rebuilt bundle is actually
  served and the four assertions get their first real run.
- **Approximate line-number citations for `~4997`/`1151`/`183/243`/`~6558-6591`** were corrected as part
  of the filename fix (§2 table) since I was already re-verifying each target file; I did not go looking
  for *further* line-drift beyond the citations already touched by the `component.tsx` → `🟦️.tsx` rename
  sweep (e.g. did not re-audit every other line number cited elsewhere in the codebase outside this one
  spec file).
- **`playwright.config.ts` and `vite.config.ts`** — read in full, both already correct (`testMatch`,
  `baseURL`, port 6029 default, `🌐️.html` entry); no changes made or needed.
- **No test was skipped, weakened, or had its assertions changed** — per the task's constraint against
  running Playwright/builds, and because the header comment's own stated intent ("Known-defect windows
  are asserted exactly like every other window... so a real defect fails loudly") is a deliberate design
  choice of this suite, not a defect to fix.

## Files touched

- Edited: `♻️mit-bestand/🧺️demonstrator/🧪️demonstrator.acceptance.spec.ts` (7 doc-comment fixes, listed
  in the §2 table — no logic/selector/assertion changes).
- Read only, unchanged: `♻️mit-bestand/🧺️demonstrator/🧪️playwright.config.ts`,
  `♻️mit-bestand/🧺️demonstrator/🟦️brand.ts`, `♻️mit-bestand/🧺️demonstrator/🟦️.tsx`,
  `♻️mit-bestand/🧺️demonstrator/⚙️vite.config.ts`.
- Created: this file, `📓️acceptance-spec-audit.md`.
