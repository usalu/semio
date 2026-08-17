# W1-B — TS Host Parity (AppRouter, OpeningResolver, AppChannelClient, opening-preferences fold)

Lane 1-B of `26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET`. Scope: contract-freeze.md §1–§4.

## What landed

### 1. `AppRouter`/`OpeningResolver` TS twins — NEW, my exclusive lease

`🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts`

- `//#region 🔖️AppRouter` (inserted after `//#endregion 🔖️ArtifactContribution`, line 268):
  - `ArtifactDialect`, `AppRole`, `AppRef` types (:278, :319, :328) — local duplicates of the Rust
    wire shapes, not imports off `🛂️manifest/🟦️component.ts`'s generated `AppDefinition` (see
    "Drift risks" below for why).
  - `dialectCoordinate`/`parseDialectCoordinate` (:290/:299) — byte-exact port of Rust
    `ArtifactDialect::to_coordinate`/`parse_coordinate` (`🔨️modules/🚪️io/🦀️component.rs:67,74`):
    `@` splits at its FIRST occurrence, the LAST `/` splits standard from subset.
  - `surfaceAppId`/`parseSurfaceAppId` (:334/:340) — port of Rust `surface_app_id`/
    `parse_surface_app_id` (`🛂️manifest/🦀️component.rs:2678,2683`): LAST `#` splits the role.
  - `SURFACE_FAULT_CODES` (:365) — the five frozen strings from contract-freeze §2.3
    (`viewer.read-only`, `surface.unknown-dialect`, `surface.contribution-not-permitted`,
    `surface.conflict`, `surface.missing-owner-surface`).
  - `class AppRouter` (:427) — `AppRouter.build(manifests)` groups every loaded manifest's
    `(dialect, role)` surfaces, orders each group owner-first-then-`pluginId`-then-`appId`
    ascending, and throws `SemioFaultError` with `surface.conflict` on a duplicate `AppRef` or
    `surface.contribution-not-permitted` when a non-owner plugin registers a surface for a kind
    it doesn't list in its own `dependencies`. `entriesFor`, `ownerPluginId`,
    `assertOwnedSurfacesComplete` (throws `surface.missing-owner-surface`) round out the API.
- `//#region 🔖️OpeningResolver` (right after):
  - `DefaultApp`, `OpeningPreferences`, `EMPTY_OPENING_PREFERENCES` — local twins of the Rust
    `🎚️config/🧬️schema` types.
  - `decodeOpeningPreferences` — narrows a JSON value into a whole `OpeningPreferences` snapshot
    (this facet's `Mutation::diff` is whole-record, not incremental — see docstring).
  - `OpeningConfigMutation` + `decodeOpeningConfigMutation` — mirrors the Rust
    `#[serde(tag = "mutation", rename_all = "camelCase")]` wire shape
    (`💻️os/🎚️config/🧬️schema/🧬️mutations/🦀️component.rs:15`).
  - `applyOpeningConfigMutation`/`foldOpeningPreferences` — event-sourced fold, **never a mutable
    map**: every step is `base.defaults.filter(...)` (+ `.push(...)` for `setDefaultApp`),
    recomputing a fresh array, byte-for-byte the same filter-then-push / filter-only logic as
    Rust's `🔺️diff` leaves (`set-default-app/🔺️diff/🦀️component.rs:9`,
    `clear-default-app/🔺️diff/🦀️component.rs:9`).
  - `resolveOpeningApp(router, dialect, role, prefs)` — the four-step precedence from
    contract-freeze §3, literally as four sequential checks (not collapsed into three, even
    though step 2's owner entry and step 3's "first entry" coincide whenever an owner is known —
    kept separate to match the frozen contract text and stay correct when they diverge, e.g. no
    owner known but contributor entries exist).

**Caveat, stated in the code and here**: task "W1-A Rust hosts AppRouter/OpeningResolver" had not
landed a concrete Rust `struct AppRouter`/`struct OpeningResolver` as of this write (confirmed:
`grep -rn "AppRouter\b" 🧰️framework` / `OpeningResolver\b` found zero struct definitions anywhere
in the tree). This TS code implements the FROZEN CONTRACT TEXT (§3), not a Rust source file. It
must be diffed against the real Rust `AppRouter`/`OpeningResolver` the moment lane 1-A ships them.

### 2. `AppChannelClient` — three new methods

`🧰️framework/🛍️products/💻️os/🟦️component.ts`, `//#region 🔖️AppChannelClient` (after
`detachBackbone`, before `contextMenu`):

- `openArtifact(artifactRef, role, pluginId = "", appId = "")` → `AppCommand::OpenArtifact`.
- `setDefaultApp(artifactKind, standard, subset, role, pluginId, appId)` → `AppCommand::SetDefaultApp`.
- `clearDefaultApp(artifactKind, standard, subset, role)` → `AppCommand::ClearDefaultApp`.

**The codec itself (`encodeAppCommand`/`decodeAppCommand` tags 27–29) was already fully landed**
before I started — both the encoder AND decoder (contract-freeze's own note said "verify the
decoder half exists; finish it if not" — verified, it was already there, field-order-identical to
`🔨️modules/📡️spr/🧵️channel/🦀️component.rs:551-576,662-689`). I only added the three client
convenience methods on top; the codec region itself was untouched.

### 3. Opening-preferences config-lane attach

`🧰️framework/🛍️products/💻️os/🟦️backbone-worker.ts`, new `//#region 🔖️ConfigLane` (after
`//#endregion 🔖️DocumentState`):

- `OPENING_PREFERENCES_SCHEMA = "os.config.opening"`.
- `openingPreferencesActorConfig(actor)` — builds the `ArtifactActorConfig` that opens this facet
  through the SAME generic actor mechanism every other document uses. `bindings: []` is not a
  stub: `folderBinding`/`hubBinding` (this file, :119-127) already return `null` on an empty
  `bindings` array, so `openArtifact` (this file's TS-fallback actor) already skips folder
  watch/hub websocket setup for an empty-bindings config — i.e. "persisted local-only" (contract
  freeze §4) falls out of the EXISTING generic mechanism with zero schema-specific branches
  anywhere in this file. Verified by reading `openArtifact`/`folderBinding`/`hubBinding` — did not
  invent new sync logic.
- `foldOpeningPreferencesEvent(base, event, decodePayload)` — reduces one `ArtifactEvent` onto a
  materialized `OpeningPreferences`: for a `remoteMutations` event, each envelope's
  `diff.payload` (this facet's whole-record diff) IS the next full snapshot, so folding is
  "last envelope wins," not a replay of individual mutations. Every other event kind passes
  `base` through unchanged.
- Added `import type { OpeningPreferences } from "@semio-tech/framework"` — confirmed this is the
  correct dependency direction (`🛍️products/💻️os/📦️packages/🟦️typescript/package.json` already
  lists `"@semio-tech/framework": "workspace:*"`), and confirmed no name collision: grepped every
  module `🟦️glue.ts` re-exports (`action-bus`, `schema`, `platform`, `mesh`, `manifest`, `kernel`,
  `machine`) for `ArtifactDialect`/`AppRole`/`AppRef`/`DefaultApp`/`OpeningPreferences`/
  `AppRouter`/etc. before adding them — zero hits anywhere else.

### 4. Golden vectors — consumed, not duplicated

`🧰️framework/🛍️products/💻️os/🧫️fixtures/📡️channel/app-command-opening.json` and
`channel-version.json` already existed (someone else's prior pass, referenced from that facet's
own Rust docstring as `📓️w0-c-report.md`) with all four opening-command hex vectors
(`OpenArtifactResolve`, `OpenArtifactExplicit`, `SetDefaultApp`, `ClearDefaultApp`) and
`channelVersion: 10`. A full cross-language parity test consuming them already existed too
(`os/🟦️component.ts`, `"matches the shared cross-language opening fixture vectors, byte-for-byte"`,
encoding AND decoding every vector). I read the directory first per instructions and wrote nothing
new there.

## Commands run and results

```
bun nx run @semio-tech/framework:test
  Test Files  2 passed (2)
  Tests       150 passed (150)
```

```
bun nx run @semio-tech/framework-os:test
```
Submitted, but got stuck behind ~300 concurrent identical `nx run @semio-tech/framework-os:test`
invocations from other live sessions in this shared repo (`ps aux | grep framework-os:test | wc -l`
→ 322 at one point) — a known contention pattern (see project memory "Concurrent Cargo Workspace
Churn"). I did not kill any process (not mine to kill in a live multi-session repo) and did not
wait indefinitely. Instead I ran the exact same vitest config directly, uncontended:

```
cd 🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript
bunx vitest run --config 🧪️vitest.config.ts
  Test Files  4 failed (4)
  Tests       4 failed | 290 passed (294)
```

The 4 failures are two distinct, **pre-existing, environmental** causes, both confirmed absent
from disk (not caused by any TS source change):
- `ENOENT …/🧫️fixtures/📡️wire/📦️client-hello.bin` — that whole fixtures directory doesn't exist
  on disk (`ls` confirms). Needs a Rust-side fixture generation step.
- `Cannot find module …/🦀️rust/pkg/semio_framework_os.js` — the wasm build output directory
  (`🧰️framework/🛍️products/🦀️rust/pkg/`) doesn't exist on disk either. Needs a `wasm-pack build`
  that hasn't run in this checkout.

Neither failure touches `openArtifact`/`setDefaultApp`/`clearDefaultApp`, the opening fixture
parity test, or anything in `🔖️AppChannelClient`/`🔖️ConfigLane`; both are reproduced identically
run after run. Full output: `🧪️w1-b-tests.txt` in this folder.

Also ran, standalone (glue.ts's vitest `includeSource` is scoped to itself only — see
`🧰️framework/📦️packages/🟦️typescript/🧪️vitest.config.ts:21-23` — so it does NOT scan
`🎠️kernel/🟦️component.ts` directly; the new `AppRouter`/`OpeningResolver` exports therefore get
zero coverage from the nx target above until glue.ts's own inline suite is extended, which is
outside this lease):

```
bun run 🧪️w1-b-verify.ts   (in this ticket folder)
  21/21 checks passed
```

Covers: `dialectCoordinate`/`parseDialectCoordinate` round trip (plain + dotted-standard dialect),
`surfaceAppId`/`parseSurfaceAppId` round trip, `AppRouter` owner-first + `pluginId`/`appId`
ascending ordering, `ownerPluginId`, `assertOwnedSurfacesComplete` (pass + `surface.missing-owner-
surface` throw), `surface.conflict` throw, `surface.contribution-not-permitted` throw,
`surface.unknown-dialect` throw, all four `resolveOpeningApp` precedence steps (pinned-and-present,
owner-fallback, stale-pin-falls-through-to-owner, no-owner-falls-to-first-entry), and
`foldOpeningPreferences` (set, set-then-clear round-trips to empty, purity — two folds of the same
ops from the same base are structurally equal but not the same object).

Also ran a full-repo `bunx tsc --noEmit -p tsconfig.json`: 19 pre-existing errors, all in files I
never touched (`✏️s/🔌️plugins/🔱️trinity/…/🧠️lsp/🟦️component.ts`,
`✏️s/🔌️plugins/🗄️stdio/…/🧬️schema/🟦️component.ts` ×2, `…vscode/…/🟦️extension.ts`) — zero errors in
any of my three touched files.

## Not done, and why

- **The Rust `AppRouter`/`OpeningResolver` structs don't exist yet** (lane 1-A, task pending as of
  this write). This TS code is my best-effort, fully-reasoned implementation of contract-freeze
  §3's TEXT — re-diff it against the real Rust the moment it lands. The biggest risk of drift is
  **how "owner plugin" is computed**: I used `PluginManifest.artifact_kinds` (plugin-level,
  `🛂️manifest/🦀️component.rs:3218`, doc-commented "library plugins with zero apps declare kinds
  here") as the ownership signal — first plugin in load order to list a given `artifactKind.id`
  wins. This is a reasoned inference (no other candidate field exists; ArtifactKindSpec is
  otherwise per-app produces/consumes metadata, which is ambiguous once `ArtifactContribution`
  lets a non-owner register a surface on a kind it doesn't own), not something I could verify
  against real Rust ownership-resolution code, because none exists yet. **Action for lane 1-A**:
  confirm or correct this against the real Rust `AppRouter::build`.
- **`FaultOrigin::Framework`/`"framework"` does not exist yet** on either side (Rust
  `dsl::diagnostic::FaultOrigin`, `💻️os/🔨️modules/🗣️dsl/⚠️diagnostic/🦀️component.rs:137`: only
  `Edge/Renderer/Os/Module/Plugin/App/Extension`; TS `FaultOrigin` in `🎠️kernel/🟦️component.ts:362`
  mirrors that same set), yet contract-freeze §2.3 pins `FaultOrigin::Framework` for all five
  surface/viewer fault codes. Both enums live outside this lease (Rust: the dsl diagnostic crate;
  TS: this file's pre-existing `FaultOrigin` type, not one of my two new regions), so I carry the
  frozen wire string via `"framework" as unknown as FaultOrigin` in `surfaceFault()` rather than
  editing a shared union under concurrent edit. **Action**: whichever lane owns
  `⚠️diagnostic/🦀️component.rs` needs to add the `Framework` variant (and its TS mirror), at which
  point `surfaceFault()`'s cast becomes a plain literal.
- **Real ConfigStore wiring for `os.config.opening` doesn't exist anywhere yet** (Rust OR TS) — the
  schema file's own docstring says as much ("NOT YET wired into any crate's `📦️glue.rs`"). My
  `foldOpeningPreferencesEvent`/`openingPreferencesActorConfig` in backbone-worker.ts are therefore
  forward-looking plumbing built on the EXISTING generic actor mechanism (verified it needs no
  schema-specific change), not a connection to a live store — nothing calls them yet. Wiring an
  actual caller (who opens this artifact on shell boot, who persists it to disk/IndexedDB given
  `bindings: []` means no folder/hub target) is future work, likely lane 1-C or a dedicated
  packet.
- **`glue.ts` vitest coverage for the new kernel exports** — out of my lease (glue.ts isn't one of
  my three files), and it's the only file glue's `vitest.config.ts` scans for
  `🎠️kernel/🟦️component.ts`'s inline tests. Compensated with the standalone
  `🧪️w1-b-verify.ts` script (21/21 passing, kept in this ticket folder) instead of touching
  glue.ts.
- **`os.config.opening`'s Rust `Mutation::inverse`** (undo support) has no TS twin — not needed by
  `OpeningResolver.resolve`, which only ever reads forward-folded state; out of scope for this
  deliverable.

## Files touched

- `🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts` — added `//#region 🔖️AppRouter` and
  `//#region 🔖️OpeningResolver` (new regions only; no existing region touched).
- `🧰️framework/🛍️products/💻️os/🟦️component.ts` — added three methods inside the existing
  `//#region 🔖️AppChannelClient` (codec region untouched).
- `🧰️framework/🛍️products/💻️os/🟦️backbone-worker.ts` — added new `//#region 🔖️ConfigLane` and one
  import line.
- `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET/🧪️w1-b-verify.ts` —
  standalone verification script (kept per ticket-folder scratch convention).
- `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET/🧪️w1-b-tests.txt` —
  captured command output.
- Read only (no changes): `🧰️framework/🛍️products/💻️os/🧫️fixtures/📡️channel/*.json`,
  `🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/*` (Rust + TS), `📋️contract-freeze.md`.
