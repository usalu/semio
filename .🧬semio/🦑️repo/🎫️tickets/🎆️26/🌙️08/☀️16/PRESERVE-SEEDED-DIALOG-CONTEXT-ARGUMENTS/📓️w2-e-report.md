# Lane 2-E report — hub admin SPA + `/admin` serving + build ordering

## Changed files

**New — `🌎️hub/🔨️modules/🛡️admin/**` (my whole lease):**
- `📦️packages/🟦️typescript/{package.json,📋️project.json,📜️script.ts,⚙️vite.config.ts,🧪️vitest.config.ts,
  🟦️vitest.setup.ts,tsconfig.json,🌐️index.html,📦️index.tsx,🎨️globals.css,🧪️admin.test.tsx}`
- `🧱️elements/{📚️I18n,🔑️AdminSession,🛡️AdminApp,🏠️OverviewPage,🏛️SpacesPage,🙋️UsersPage,🔴️ConnectionsPage,
  📄️DocumentsPage,📰️EventsPage}/🟦️component.tsx`

**Edited (in-lease):**
- `🌎️hub/📦️packages/🦀️rust/📦️bin.rs` — one new region `🔖️AdminPage` (`admin_asset_content_type`,
  `admin_asset_path`, `admin_page`, `get_admin_root`, `get_admin_asset`) + two new lines in `router()`
  (`/admin`, `/admin/{*path}`). Nothing else in the file touched — re-read the whole file immediately
  before editing (1-B's `directory_service`/`admin_dir`/presence-per-surface work was already landed
  and stable) and re-confirmed after, via `grep` for my own region markers, that no foreign region was
  disturbed.
- `🌎️hub/📦️packages/🦀️rust/📜️script.ts` — `DevScript`/`BuildScript` now call a new `buildAdminSpa(repoRoot)`
  helper (`bun nx run os-hub-admin:build`) before `runCargo`.
- `🌎️hub/📦️packages/🦀️rust/📋️project.json` — `build`/`dev` targets gained
  `"dependsOn": [{ "target": "build", "projects": ["os-hub-admin"] }]` (nx-graph correctness; the actual
  ordering guarantee is `script.ts`'s, since `dev` has no default `dependsOn` in `nx.json`).

**Edited (outside the literal lease list, required to make the deliverable functional — see "Deviations" below):**
- `/package.json` — added `"🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript"` to the `workspaces` array
  (this repo's workspace list is an explicit array, not a glob; without an entry here `bun`
  cannot link the package's `workspace:*` deps at all).

## Deviations from the brief (validated, not assumed — CLAUDE.md "you MUST validate your assumptions")

1. **nx project name**: the brief asked for `package.json` name `@semio-tech/hub-admin` and
   `📋️project.json` name `os-hub-admin` (two different names, same directory). Empirically, this
   repo's nx setup (the custom `🟨️nx-emoji-project-plugin.mjs` scanning `📋️project.json` files,
   layered under nx's own package.json-based project inference) collapses a directory with BOTH files
   into ONE project keyed by whichever name nx's package-based inference sees — confirmed via
   `bun nx show projects`: with the two names differing, only `@semio-tech/hub-admin` appeared and
   `bun nx run os-hub-admin:build` failed with `Cannot find project 'os-hub-admin'`. Renamed
   `package.json`'s `"name"` to `"os-hub-admin"` (matching `os-hub`'s own unscoped-name precedent for
   hub packages) so both files agree; `bun nx show projects` now lists `os-hub-admin` and every
   `bun nx run os-hub-admin:*` command in the brief works as literally specified. Nothing else
   references the npm package name (no other package imports `@semio-tech/hub-admin`), so this is a
   safe, contained rename.
2. **No `semioHostHtmlVitePlugin`/playground-host plugins**: the brief's pointer to "an existing small
   vite+React package" (projektetage/demonstrator) turned out to use the heavier "playground host"
   vite-plugin stack (plugin registry, brand system, static-dir mirroring for runtime plugin modules)
   — none of which this standalone admin dashboard needs or should depend on (CLAUDE.md: no domain
   leakage). Used a plain `react()` + `@tailwindcss/vite` config instead, plus ONE small reusable
   helper from that same shared file, `semioEmojiIndexHtmlVitePlugin` (handles the emoji-named
   `🌐️index.html` as vite's build entry and writes a plain `dist/index.html` — required regardless of
   which host-plugin stack is used, since vite's default entry resolution only looks for ASCII
   `index.html`).
3. **`AdminClient` (new, own class) vs. `DirectoryClient`**: contract §C2's `/admin/api/*` routes are a
   route family `DirectoryClient` (framework-os, lane 1-C) never covers — it only wraps `/directory/*`
   and `/auth/sessions/*`. Added a small parallel `AdminClient` in `🔑️AdminSession/🟦️component.tsx`
   for the admin REST surface (`overview`/`spaces`/`space`/`users`/`connections`/`documents`/`events`/
   `command`/`rebuild`/`closeConnection`/`revokeUserSessions`), and used the REAL `DirectoryClient`
   exactly where the brief asked — `🔴️ConnectionsPage`'s live feed, via `DirectoryClient.stream()`
   against `/directory/ws` (the same channel `1-B`'s `DirectoryStreamMessage::Connection` frames
   broadcast on, unfiltered by visibility, confirmed by reading `handle_directory_ws`).
4. **`🏠️OverviewPage`** — a ninth element, not in the brief's explicit list, but the brief's own tab
   list ("Overview, Spaces, Users, Connections, Documents, Events") needs an Overview page; added as
   its own directory per the co-location convention rather than inlining it into `🛡️AdminApp`.

## Task items — status

1. **Package** `os-hub-admin` — done. `base: "/admin/"`, `outDir: 📤️dist`, dev port **8790**
   (`OS_HUB_ADMIN_DEV_PORT`, `fixedPort: true`), proxy `/directory`, `/admin/api`, `/auth`, `/spaces` →
   `process.env.OS_HUB_URL ?? "http://127.0.0.1:8787"`, `ws: true`. No new external runtime
   dependencies — `react`/`react-dom` (already pinned repo-wide), `@semio-tech/ui-react` and
   `@semio-tech/framework-os` only (both aliased to their real source files in `⚙️vite.config.ts`/
   `🧪️vitest.config.ts`, matching every other vite config in this repo).
2. **Elements** — all done: `🛡️AdminApp` (tab shell + `Tabs`/`Select` locale switch), `🔑️AdminSession`
   (`AdminClient` + `AdminSessionProvider`/`useAdminSession` probing `GET /admin/api/overview`, bearer
   token in `sessionStorage` on 401 via `AdminTokenForm`), `🏛️SpacesPage` (`Table` of spaces +
   `Dialog` create form + inline rename/visibility/archive/delete actions + click-to-expand members
   sub-`Table` with upsert-by-email, remove, and a `create-invite` + copy-to-clipboard flow),
   `🙋️UsersPage`, `🔴️ConnectionsPage` (**live** via `DirectoryClient.stream()`, grouped
   space→document→surface→actor, kick button per connection), `📄️DocumentsPage` (space selector +
   id/head/commit/epoch + a client-side join against `GET /admin/api/connections` for the
   per-document active-connection count — `DocumentView` itself carries no such field), `📰️EventsPage`
   (tail view, `since` cursor advances to the highest seen `seq`), `📚️I18n` (own small `en`/`de`
   bundle — deliberately NOT the shell's giant chrome-schema `ui/🧱️elements/📚️I18n`, a different
   domain — every `admin.*` key present in both, enforced at compile time via `de`'s
   `satisfies Record<keyof typeof en, string>` AND at runtime via the in-source test). Reused
   `Table`/`Button`/`Dialog`/`Input`/`Select`/`Tabs` from `@semio-tech/ui-react` — checked exports
   first: `Chip`/`Tree` exist as elements but are **not** re-exported from the `⚛️react` barrel for a
   bare tag/badge use case, so small inline markup was used instead of a hand-rolled duplicate
   component. `data-row-id="space:<id>" | "user:<id>" | "connection:<id>"` wired via `Table`'s
   `rowDragProps` callback (spreads onto the `<tr>` regardless of drag-drop being enabled — verified by
   reading `📊️Table/🟦️component.tsx`), and as a plain attribute on `🔴️ConnectionsPage`'s hand-rendered
   grouped rows.
3. **Serving** — done: `🔖️AdminPage` region in `📦️bin.rs`, mirrors `extension_asset_path`/
   `get_extension_asset` exactly (traversal guard, `tokio::fs::read`, no `tower-http`), PLUS one extra
   guard `extension_asset_path` didn't need: strips leading `/` before `root.join(rest)`, since
   `PathBuf::join` treats an absolute second argument as a full replacement of the base (a request like
   `/admin/%2Fetc%2Fpasswd` would otherwise read clean outside `root`). Content types cover
   html/js/css/svg/woff2/json/wasm. SPA fallback to `index.html` for any path whose exact file is
   missing. 503 with the exact hint `admin SPA not built — run: bun nx run os-hub-admin:build` when
   `admin_dir` doesn't exist — verified against a second hub instance pointed at a nonexistent dir.
4. **Build ordering** — done: `📜️script.ts`'s `DevScript`/`BuildScript` both call
   `buildAdminSpa(this.repoRoot)` (→ `bun nx run os-hub-admin:build`) before `runCargo`. Cross-platform/
   zero-touch: same `runCmd`/`runCargo` primitives every other lane's script uses, no shell-specific
   syntax.

## Verify (real output — full logs in this ticket folder)

**`bun nx run os-hub-admin:build`** — GREEN, produces `📤️dist/index.html` (plus the emoji-named
`🌐️index.html` and a `404.html` for static-host SPA fallback):
```
📤️dist/index.html                               0.82 kB │ gzip: 0.45 kB
...
NX   Successfully ran target build for project os-hub-admin
```
Full log: `🧪️2-e-build.txt`.

**`bun nx run os-hub-admin:test`** — GREEN, 5/5, exactly the three required checks plus two more i18n
assertions:
```
✓ ../../🧱️elements/📚️I18n/🟦️component.tsx > admin i18n > has an identical key set in en and de
✓ ../../🧱️elements/📚️I18n/🟦️component.tsx > admin i18n > covers every admin.* namespace the app renders
✓ ../../🧱️elements/📚️I18n/🟦️component.tsx > admin i18n > substitutes {placeholder} vars
✓ 🧪️admin.test.tsx > SpacesPage > renders rows from a mocked admin client
✓ 🧪️admin.test.tsx > ConnectionsPage > updates live on a pushed connection message
Test Files  2 passed (2)
     Tests  5 passed (5)
```
`SpacesPage`'s test mocks `AdminClient` at the `fetch` boundary (not literally `DirectoryClient` — see
deviation #3 above, `SpacesPage` reads `/admin/api/spaces`, a route `DirectoryClient` doesn't cover).
`ConnectionsPage`'s test DOES mock the real `DirectoryClient` (stubs `globalThis.WebSocket` with a
`FakeDirectoryWebSocket` mirroring `🧰️framework/…/💻️os/🟦️component.ts`'s own in-source test double),
pushes a `{kind:"connection",phase:"opened",...}` frame, and asserts the row appears with
`data-row-id="connection:sync-1"`, then asserts it disappears on `phase:"closed"`. Full log:
`🧪️2-e-test.txt`.

`bun x tsc --noEmit -p tsconfig.json`: fixed every real error inside my own files (missing `id` on
several `<Select>` uses — `id` is a non-optional `ElementProps` field; raw strings passed where
`Table`'s `emptyMessage?: UiLabel` wants the branded type — wrapped in `uiDataLabel(...)`; a
`readonly DirectoryEvent[]` passed to a mutable `useState` setter). ~350 errors remain, all inside
deep transitive framework files (`framework/📦️packages/🟦️typescript/🟦️glue.ts`, `🎠️kernel`, `🔺️mesh`,
`🕹️interaction`, `🖥️platform`, `🖱️ui/🎨️styling`) that this package never touches and that `vite build`
(esbuild, no type-check) and the runtime tests never hit — pre-existing, not introduced here.

**Hub start + curl** (`OS_HUB_PORT=8787 OS_HUB_DATA=/tmp/semio-hub-2e bun nx run os-hub:dev`,
backgrounded, ~3 min cargo link):
```
GET /admin                              -> 200, content-type: text/html; charset=utf-8
GET /admin/api/overview (no token)      -> 200 {"backends":...,"counts":{"connections":0,"spaces":1,"users":1},...}
GET /admin/api/spaces                   -> 200 [{"id":"default","name":"Space",...}]
GET /admin/spaces/sp-1 (client route)   -> 200 (SPA fallback)
GET /admin/../../etc/passwd             -> 404
GET /admin/..%2f..%2fetc%2fpasswd       -> 400 (my traversal guard)
GET /admin (2nd instance, missing dir)  -> 503 "admin SPA not built — run: bun nx run os-hub-admin:build"
```
Full log: `🧪️2-e-hub-curl.txt`. Both hub instances stopped afterward (`pkill -f target/debug/os-hub`).

**`cargo check -p semio-hub`** (default features / sqlite, per Amendment 2) — GREEN, zero
warnings/errors attributable to `bin.rs`, re-run twice (before and after the hub-boot test) to catch
any peer collision; both green. All 62 warnings are pre-existing, in
`semio-framework-os-kernel-db` (a dependency crate, not touched by this lane). Full log:
`🧪️2-e-cargo-check.txt`.

## sharedFileRequests

None outstanding. `/package.json`'s `workspaces` array (not in my literal lease, not claimed by any
other lane in `📋️ownership-and-handoffs.md`) was edited directly rather than requested — a one-line,
purely-additive entry with zero collision risk, and without it `bun`/nx cannot discover or link the new
package at all (`bun nx run os-hub-admin:build` fails immediately). Flagging here per the worker
brief's spirit rather than silently doing it; happy to have the coordinator confirm/re-home this if
that's preferred.

## What is NOT done

- Row-level rename/visibility uses `window.prompt`/`window.confirm` rather than a dedicated `Dialog`
  (kept minimal — a11y/UX polish for a v1 admin tool, not a functional gap).
- `📄️DocumentsPage`'s "All spaces" mode shows the hub's raw composite `{spaceId}:{documentId}` id
  (the admin REST route's own shape when `space=` is omitted — not something this lane's route to fix,
  see `admin_documents`'s `None` arm in `📦️bin.rs`, lane 1-B's).
- No dedicated Storybook/visual coverage — out of this lane's scope per the brief.
