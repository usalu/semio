# Lane 1-A finisher report — TS twin for manifest controls (C8.1)

## Status: TS twin written, regenerated, verified — one real regression found and fixed along the way

## Changed files

- `🧰️framework/🔨️modules/🛂️manifest/🟦️component.ts`
  - Added `AppRole as GeneratedAppRole, AppRef as GeneratedAppRef, ArtifactDialect as GeneratedArtifactDialect`
    to the existing `🧬️GeneratedMirror` import block (ts-rs mirror import surface).
  - New region `//#region 🔖️HostResolvedArgs`, placed right after `PluginManifest` (mirrors the Rust
    region's position right after `PluginManifest` in `🦀️component.rs`), before `//#region AppManifestProtocol`:
    - Module-**private** (not exported — see "Regression found and fixed" below) `AppRole`/`AppRef`/
      `ArtifactDialect` type aliases over the generated ts-rs types.
    - `ArtifactKindChoice { kindId, schema, dialect, label: { en, de } }` and
      `SurfaceAppChoice { app, role }` — exported types, TS twins of the Rust structs.
    - `encodeArtifactKindChoice`/`decodeArtifactKindChoice`, `encodeSurfaceAppChoice`/
      `decodeSurfaceAppChoice` — hand-written JSON codecs mirroring Rust's `serde_json::json!` key
      order exactly (kindId, schema, dialect, label.en, label.de / pluginId, appId, role).
    - `resolveNativeLabel(label: unknown)` — a tiny private helper that reads the `native` cell off
      the manifest wire shape `{ native: { en, de }, reuse: { en, de } }` (Rust `LocalizedLabel`'s
      `Serialize`), since `AppDefinition.label` is `unknown` on the generated type (no ts-rs mirror
      for `LocalizedLabel` exists yet — pre-existing, documented gap, out of scope here).
    - `artifactKindChoices(manifests, roles)` — the pure resolver, TS twin of Rust
      `artifact_kind_choices`: dedupes by dialect coordinate (`Map`, first-manifest/app wins, mirroring
      Rust's `BTreeMap::entry().or_insert_with()`), sorted by coordinate (`.sort()` on the coordinate
      strings — matches Rust `Ord` for ASCII dialect-grammar strings), filtered to
      `role ∈ roles && io.documentSchema !== ""`.
    - An `if (import.meta.vitest)` block with 7 tests: the pinned-fixture byte-identical encode test
      (exact string from contract §C8.1), a decode-inverts-fixture test, a decode-throws-on-missing-field
      test, a `SurfaceAppChoice` round-trip test, a decode-throws-on-bad-role test, and an
      `artifactKindChoices` test over three fake manifests proving dedup-by-coordinate with
      owner-manifest-first-wins semantics, sort order, and role filtering (editor-only / editor+viewer /
      viewer-only).
  - `PluginManifest.apps` was **not** widened to `AppDefinition[]` — it stays
    `readonly Record<string, unknown>[]` (pre-existing, documented as pending a different ticket's "C1"
    work in `🎠️kernel/🟦️component.ts`'s own `AppRouterManifest` comment); `artifactKindChoices` reads
    each `app` through a local structural cast instead, matching the established
    `AppRouterManifest`-narrow-cast idiom already used elsewhere in this codebase for the same reason.

- `🧰️framework/🔨️modules/🛂️manifest/🤖️generated/🟦️manifest.ts` — regenerated (see "Commands run" below).

## Regression found and fixed (read this)

My first draft `export`ed `AppRole`, `AppRef`, `ArtifactDialect` as public bare-name type aliases (the
natural thing to do, matching the file's own `export type X = GeneratedX;` convention used everywhere
else). This is **wrong** and I caught it only because I proactively ran a real `tsc --noEmit` against
`🟦️glue.ts` (there is no `typecheck`/`tsc` nx target for `@semio-tech/framework` — `bun nx run
@semio-tech/framework:test` runs vitest, which transpile-strips types via esbuild/oxc and would **not**
have caught this):

```
🧰️framework/📦️packages/🟦️typescript/🟦️glue.ts(13,1): error TS2308: Module
"../../🔨️modules/🛂️manifest/🟦️component.ts" has already exported a member named 'AppRef'. Consider
explicitly re-exporting to resolve the ambiguity.
```
(same for `AppRole`, `ArtifactDialect`)

Cause: `🎠️kernel/🟦️component.ts` **already** hand-declares structurally identical `AppRole`/`AppRef`/
`ArtifactDialect` types (its own `🔖️AppRouter` region, lines ~305-343), with a doc comment explicitly
explaining why it duplicates rather than imports from this file ("that twin's `apps` field is still
`Record<string, unknown>[]` pending the ts-rs regen for contract freeze §1 C1 ... same idiom as the
🔖️PluginDependency/🔖️ArtifactContribution regions above"). Both modules are `export *`-ed into the same
`🟦️glue.ts` barrel, so a second public export of the same bare names is an ambiguous-export error at
that barrel — not a new type any caller needs, since kernel's copy already flows through
`@semio-tech/framework`.

**Fix**: dropped the `export` keyword — `AppRole`/`AppRef`/`ArtifactDialect` are now **module-private**
aliases inside `🛂️manifest/🟦️component.ts`, used only by `ArtifactKindChoice`/`SurfaceAppChoice`/
`artifactKindChoices` internally. `ArtifactKindChoice`/`SurfaceAppChoice`/`artifactKindChoices` etc. stay
publicly exported (unique names, no collision). Re-verified with the same `tsc --noEmit` invocation
against `🟦️glue.ts`: zero `AppRole`/`AppRef`/`ArtifactDialect`/"already exported" errors afterward.

## A second, pre-existing gap found while verifying (not fixed — outside this task's lease)

Regenerating (`bun nx run @semio-tech/framework-rs:generate`) does fix P3's literal sharedFileRequest:
`UiPresence` in `🤖️generated/🟦️manifest.ts` now carries `color: number | null` and
`peers: Array<UiPeerMark>` (confirmed by direct `grep`/`sed` inspection of the regenerated file).

**But** the regenerated bundle is still not fully self-consistent as real TypeScript. I checked this by
running `tsc --noEmit` directly against `🤖️generated/🟦️manifest.ts` in isolation (realistic compiler
flags: `--strict --lib DOM,ESNext --target ESNext --module ESNext --moduleResolution bundler
--isolatedModules`) and diffing declared-vs-referenced type names. Result: **15 type names are
referenced in the generated bundle but never declared inside it** (the consolidator's own
`stripTsRsBoilerplate` step in `📜️script.ts` deliberately strips every per-type file's `import type
{...}` line before concatenating, so the bundle is only self-consistent if literally everything it
transitively references is also `::export()`-ed by the `exports_typescript_bindings` test). Of the 15,
**two are germane to this ticket's own C7.6/C8.1 work**:

- `AppIo` — referenced by `AppDefinition.io: AppIo` (line 200) — needed by `AppDefinition`, which
  `🛂️manifest/🟦️component.ts`'s own `AppDefinition` type (`Omit<GeneratedAppDefinition, ...> & {...}`)
  transitively resolves.
- `UiPeerMark` — referenced by `UiPresence.peers: Array<UiPeerMark>` (line 1173) — the exact field P3's
  sharedFileRequest was about; `UiPresence` is directly re-exported (`export type UiPresence =
  GeneratedUiPresence;`, line 151).

Both structs **do** carry `#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]` already (verified by
reading their definitions: `AppIo` in `🛂️manifest/🦀️component.rs:4883`, `UiPeerMark` in
`🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️component.rs:85`) — they are simply never called via
`::export()` in `🛂️manifest/🦀️component.rs`'s `exports_typescript_bindings` test
(`🦀️component.rs:6761+`). **`bun nx run @semio-tech/framework-rs:check` does not catch this class of
bug** — it only byte-compares the freshly-rebuilt bundle against the committed one, never type-checks
the bundle's own content, so `:check` passing (it does, see below) does not mean the mirror is valid
TypeScript.

**Not fixed by me**: `🛂️manifest/🦀️component.rs` (the Rust file housing `exports_typescript_bindings`) is
outside this finisher task's stated lease (`🟦️component.ts` + `🤖️generated/**` only — the brief's "State
you inherit" section marks the Rust half as already landed/closed for this task). Filing as a
`sharedFileRequest` below rather than touching it, per worker-brief rule 2 and this task's own
instruction 4 ("if the generator does not cover it, say so precisely").

The other 13 pre-existing dangling names (`Label`, `Locale`, `Terminology`, `StyleSpec`, `UiMenuRef`,
`WindowStackCorner`, `UiTreeActionPlacement`, `TopicContribution`, `ComposerEntryDescriptor`,
`FileTypeContribution`, `IoEntryDescriptor`, `ConfigSpec`, `CommandGrammar`) predate this ticket
entirely, are unrelated to C7.6/C8.1, and are not touched — named here only so whoever looks at this
class of bug next has the full list instead of rediscovering it piecemeal. In practice none of these
are fatal at runtime because nothing consumes the generated bundle's own orphaned duplicate
`Ui*Node`/`AppIo`/`ConfigSpec`/etc. declarations directly — `🛂️manifest/🟦️component.ts` hand-defines its
own `Label`/`UiMenuRef`/`StyleSpec`/etc. instead of aliasing the generated ones for most of these — but
`AppIo` and `UiPeerMark` **are** live (reached through `AppDefinition`/`UiPresence`, both re-exported),
so those two are the ones that actually matter here.

## sharedFileRequests

1. **File**: `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs`, function `exports_typescript_bindings`
   (`#[cfg(test)]`, starts at line 6761).
   **Exact change requested**: add two lines, matching the file's own existing convention for
   same-file/sibling types (compare `crate::ui::ArtifactKindSpec::export().unwrap();` /
   `crate::ui::MediaPortSpec::export().unwrap();` at lines 6950/6957 for a same-file type, and
   `ui_wgpu::wgpu::UiPresence::export().unwrap();` at line 6796 for a sibling `ui_wgpu::wgpu` type):
   - Near the `ui_wgpu::wgpu::UiPresence::export().unwrap();` line (~6796):
     `ui_wgpu::wgpu::UiPeerMark::export().unwrap();`
   - Near the `crate::ui::AppDefinition::export().unwrap();` line (~6916):
     `crate::ui::AppIo::export().unwrap();`
   **Why**: without these, `🤖️generated/🟦️manifest.ts` — even freshly regenerated — has two dangling
   type references (`AppIo` in `AppDefinition.io`, `UiPeerMark` in `UiPresence.peers`) that make the
   bundle invalid TypeScript in isolation; see the section above for the exact `tsc --noEmit` evidence.
   Both structs already carry the `ts_rs::TS` derive; this is purely two missing `::export()` calls, not
   a Rust-side type change. This is a Rust-file change so I did not apply it myself (outside this
   finisher task's lease — see "State you inherit" in the brief).

2. (Carried forward from `📓️1-a-report.md`, still unresolved as of this check — re-verified live in the
   tree, not applied by any other lane yet) **File**:
   `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`, function `validate_arg_defs`
   (currently line 364). **Exact change requested**: alongside the existing `Select`-with-no-options
   assertion, add an equivalent arm for `ActionArgControl::ArtifactKind { roles }` and
   `ActionArgControl::SurfaceApp { roles, .. }` asserting `!roles.is_empty()`. **Why**: contract §C8.1 /
   worker brief item 3 — this file's `validate_arg_defs` is owned by lane P3 per the ownership table, not
   mine to edit.

## What was already done before this finisher session (verified, not re-done)

- `ActionArgControl::ArtifactKind { roles }` / `SurfaceApp { roles, dialect_arg }` Rust variants,
  `ActionArgDef::artifact_kind`/`::surface_app` constructors, the `🔖️HostResolvedArgs` Rust region
  (`ArtifactKindChoice`/`SurfaceAppChoice`/codecs/`artifact_kind_choices`), and `PluginBuilder::editor`/
  `::viewer` schema stamping (C8.2) — all landed and committed, confirmed by direct reads of
  `🛂️manifest/🦀️component.rs:3807-3901` and `🔌️plugin/🏗️builder/🦀️component.rs:333-397`.
- The TS side of `ActionArgControl`/`ArgFormat` (the `artifactKind`/`surfaceApp` variants) and
  `argControl()`'s derivation of them from `ArgFormat` were **already present** in both
  `🤖️generated/🟦️manifest.ts` and the hand-written `argControl()` function in `🟦️component.ts` (lines
  605-619) before I touched anything this session — someone (packet P3-manifest-schema, per the inline
  comment) landed that half already. I did not need to add anything there.

## Commands run + results (real tails, nothing fabricated)

1. `bun nx run @semio-tech/framework-rs:generate` (the brief says `@semio-tech/framework:generate`, but
   the only nx project that actually owns `generate`/`check` targets is `@semio-tech/framework-rs` — the
   ts-side project `@semio-tech/framework` only has `test`/`test-quick`/`test-long`/`test-exhaustive`;
   confirmed by reading both `📋️project.json` files). Took ~5m15s (heavy concurrent load — 90-120
   rustc/cargo processes observed throughout this session). Tail:
   ```
   test manifest::app_label_tests::exports_typescript_bindings ... ok
   test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 157 filtered out; finished in 0.97s
   framework typescript mirror refreshed -> .../🛂️manifest/🤖️generated/🟦️manifest.ts
   NX   Successfully ran target generate for project @semio-tech/framework-rs
   ```

2. `bun nx run @semio-tech/framework-rs:check` — **PASS**. Tail:
   ```
   test manifest::app_label_tests::exports_typescript_bindings ... ok
   test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 157 filtered out; finished in 1.44s
   framework typescript mirror is fresh.
   NX   Successfully ran target check for project @semio-tech/framework-rs
   ```
   (Reminder: this only proves the committed bundle matches a fresh rebuild byte-for-byte — see the
   "second gap" section above for why it does not prove the bundle type-checks.)

3. `cargo test -p semio-framework --lib` — **PASS, 157/157** (0 failed, 0 ignored). Took ~5 minutes under
   heavy concurrent load (90-120 rustc/cargo processes observed throughout, per the peer
   `MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME` ticket). Full tail saved to
   `🧪️1-a-finish-cargo-test-semio-framework.txt`:
   ```
   test result: ok. 157 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
   ```

4. **Not in the brief's "Verify" list, but I ran it anyway to actually exercise my new tests** (see
   "TS test wiring gap" below for why `bun nx run @semio-tech/framework:test` alone doesn't run them): a
   throwaway vitest config at `$T/🧪️1-a-finish-probe.vitest.config.ts` (`includeSource`/`include` pointed
   directly at `🛂️manifest/🟦️component.ts`, not committed/applied anywhere real) —
   **12/12 passing** (6 tests × 2, vitest's own dual-run — I did not investigate why the dual-run happens,
   it's the runner's own behavior on this repo, unrelated to my code). Log:
   `🧪️1-a-finish-ts-instrument-probe.txt`.

5. `bunx tsc --noEmit` scoped to `🟦️glue.ts` — used to catch and confirm-fix the `AppRole`/`AppRef`/
   `ArtifactDialect` collision (see above). Also used on `🤖️generated/🟦️manifest.ts` directly to find the
   `AppIo`/`UiPeerMark` gap.

6. `bun nx run @semio-tech/framework:test --skip-nx-cache` (the framework's real vitest gate,
   `🟦️glue.ts`'s own in-source tests) — **152/152 passing**, unaffected by any of my changes (my new
   tests are not part of this run — see next section).

## TS test wiring gap (found, reported, not fixed — outside lease)

`📦️packages/🟦️typescript/🧪️vitest.config.ts`'s `include`/`includeSource` list **only** `"🟦️glue.ts"` —
the single barrel file, not a glob. `import.meta.vitest` in-source blocks only actually execute for
files vitest's in-source plugin has been told to instrument; since `🛂️manifest/🟦️component.ts` isn't in
that list, my new `if (import.meta.vitest) {...}` test block (and, I discovered, `🎠️kernel/🟦️component.ts`'s
**pre-existing** `describe("expandPluginRegistry", ...)`/`describe("IoEntryGraph", ...)` blocks too —
this is not new, not mine, not scoped to this ticket) never runs under the real
`bun nx run @semio-tech/framework:test` gate. I verified this concretely: ran the real gate, grepped its
verbose output for `HostResolvedArgs`/`expandPluginRegistry`/`IoEntryGraph` — none appear, despite both
files having those exact `describe(...)` blocks in source.

I did **not** fix `📦️packages/🟦️typescript/🧪️vitest.config.ts` (outside this finisher task's lease) and
did **not** move my tests into `🟦️glue.ts` (also outside lease). Instead I independently verified my
tests pass via a throwaway, non-committed vitest config (see item 4 above) — real runtime verification,
just not wired into the checked-in gate. Filing as a third, lower-priority sharedFileRequest:

3. **File**: `🧰️framework/📦️packages/🟦️typescript/🧪️vitest.config.ts`. **Exact change requested**: widen
   `include`/`includeSource` from `["🟦️glue.ts"]` to also cover every module's `🟦️component.ts` that
   carries an `import.meta.vitest` block — at minimum add `"../../🔨️modules/🛂️manifest/🟦️component.ts"`
   and `"../../🔨️modules/🎠️kernel/🟦️component.ts"` (the two files that currently have dead in-source
   tests), or more robustly a glob (`"../../🔨️modules/**/🟦️component.ts"`) so this class of gap can't
   recur silently. **Why**: currently `bun nx run @semio-tech/framework:test` silently skips every
   `import.meta.vitest` block outside `🟦️glue.ts` — a real, pre-existing (not introduced by this ticket)
   test-coverage gap that made my own new tests (and kernel's two existing `describe` blocks) invisible
   to the real CI gate.

## Blockers

None outstanding. `cargo test -p semio-framework --lib` and `bun nx run @semio-tech/framework-rs:generate`
each took several minutes under heavy concurrent load (~90-120 rustc/cargo processes observed
throughout, from the peer `MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME` ticket) but both completed cleanly —
run once each, per the brief's instruction, real tails reported above.

## What is NOT done

- The two Rust sharedFileRequests above (`exports_typescript_bindings` missing `AppIo`/`UiPeerMark`
  exports; `validate_arg_defs` missing the `ArtifactKind`/`SurfaceApp` roles-non-empty checks) —
  Rust-file changes outside this finisher task's stated lease.
- The vitest-config sharedFileRequest above — outside lease.
- Widening `PluginManifest.apps`'s type beyond `Record<string, unknown>[]` — deliberately left alone; a
  different, larger, cross-module concern ("contract freeze §1 C1" per kernel's own comment), not this
  ticket's C8.1 scope, and touching it would ripple far beyond my lease.
