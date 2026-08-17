# W1-D Report — Rust↔TS Parity Reconciliation + wgpu Shell Role Awareness

Lane 1-D of `26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET`. Contract: `📋️contract-freeze.md` §1,
§2.3, §3, §5.

## Deliverable 1 — Rust↔TS `AppRouter`/`OpeningResolver` parity reconciliation

Diffed the real Rust `AppRouter`/`OpeningResolver`
(`💻️os/🔌️plugin/🖥️host/🦀️component.rs:1708-2135` as landed by lane 1-A) against the TS twin lane
1-B wrote against the frozen contract text alone (`🎠️kernel/🟦️component.ts`, regions after :268).
Two real drifts found, both fixed on the TS side (Rust authoritative, per the ticket's own rule):

1. **How "owner plugin" is computed (the ordering rule's first key).** Rust's `register_manifest`
   claims ownership of an `artifact_kind` from EITHER a manifest's own `artifact_kinds` declaration
   OR — if still unclaimed — the first app that references it, and does this in a SINGLE pass,
   interleaved per manifest, in registration order (`state.owners.entry(...).or_insert_with(...)`,
   `🦀️component.rs:1760,1763`). The TS `AppRouter.build` computed ALL `artifactKinds`-declared
   ownership in one pass across every manifest BEFORE processing any app, and never let an app claim
   ownership at all — silently diverging from Rust's own test
   `step3_first_entry_when_the_owner_has_no_surface_for_this_role` (`🦀️component.rs:2101`), where a
   contributor with zero declared `artifact_kinds` becomes the owner by being first to register a
   surface. Fixed: `AppRouter.build` (`🎠️kernel/🟦️component.ts:426-499`) now does one pass over
   `manifests`, and for each manifest first claims from its own `artifactKinds`, then lets each app
   claim its dialect's kind if still unclaimed — byte-for-byte the same algorithm as Rust.
   Also reordered the per-app check order to match Rust exactly: contribution gate
   (`surface.contribution-not-permitted`) checked BEFORE the duplicate check (`surface.conflict`),
   not after — Rust's `register_manifest` returns on the gate failure before ever reaching the
   `registered_refs.insert` duplicate check (`🦀️component.rs:1766,1775`); the TS code had these
   reversed.
2. **`FaultOrigin::Framework`.** Rust added it (`🗣️dsl/⚠️diagnostic/🦀️component.rs:149`, additive,
   lane 1-A). TS's `FaultOrigin` union (`🎠️kernel/🟦️component.ts:727`) didn't have `"framework"` yet
   — lane 1-B had worked around this with a `"framework" as unknown as FaultOrigin` cast. Fixed: added
   `"framework"` to the union; `surfaceFault()` now writes the literal directly.

Also renamed `AppRouter.assertOwnedSurfacesComplete()` (threw on the FIRST breach) to
`ownedSurfaceGaps()` (returns `readonly Fault[]`, collects every breach, never throws) to match
Rust's `owned_surface_gaps()` exactly (`🦀️component.rs:1836` — a pure, total, non-panicking W1 soft
gate). No product code called the old name yet (verified by search), so the rename carries no
migration burden.

While tracing the duplicate-check ordering fix I found and fixed one unrelated bug: a literal NUL
byte (`\x00`) had ended up inside a template-literal string (`` `${ref.pluginId}\x00${ref.appId}` ``,
`🎠️kernel/🟦️component.ts` — the space between the two interpolations had been corrupted to a NUL
somewhere upstream of my edits, confirmed via `python3 -c "content.count(chr(0))"` → 1 hit before the
fix, 0 after). Repaired to a plain space; this was silently corrupting every `refKey` used for
duplicate detection.

### Parity test — run on both sides

Added the SAME ordered fixture (owner surface, two contributed surfaces from different plugins, a
duplicate, an unknown dialect) to both:

- **Rust**: new `#[test] fn w1_d_parity_fixture_owner_two_contributors_duplicate_and_unknown_dialect`
  in `app_router_tests` (`💻️os/🔌️plugin/🖥️host/🦀️component.rs`).
- **TS**: new standalone script `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET/🧪️w1-d-parity.ts`.

Both assert: owner (`cad`) first, then contributors (`aec-building`, `norm`) sorted `pluginId`
ascending; a duplicate `AppRef` faults `surface.conflict`; an unresolvable dialect faults
`surface.unknown-dialect`.

**Rust output** (`cargo test -p semio-framework-plugin-host --lib -- app_router_tests
opening_resolver_tests`, full log in `🧪️w1-d-parity-rust-output.txt`):

```
running 11 tests
test component::app_router_tests::contribution_without_a_declared_dependency_is_rejected ... ok
test component::opening_resolver_tests::step2_and_step3_collapse_to_the_owner_surface_when_default_is_stale ... ok
test component::opening_resolver_tests::step3_first_entry_when_the_owner_has_no_surface_for_this_role ... ok
test component::app_router_tests::contribution_with_a_declared_dependency_is_admitted_and_sorted_after_the_owner ... ok
test component::opening_resolver_tests::step1_explicit_default_still_in_router_wins ... ok
test component::app_router_tests::unregister_plugin_drops_its_surfaces_but_keeps_its_ownership_claim ... ok
test component::app_router_tests::owned_surface_gaps_reports_the_missing_role_only ... ok
test component::app_router_tests::duplicate_app_ref_is_a_conflict ... ok
test component::app_router_tests::owner_surface_sorts_first_then_plugin_id_then_app_id ... ok
test component::app_router_tests::w1_d_parity_fixture_owner_two_contributors_duplicate_and_unknown_dialect ... ok
test component::opening_resolver_tests::step4_unknown_dialect_when_the_router_has_nothing ... ok

test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 29 filtered out
```

**TS output** (`bun run 🧪️w1-d-parity.ts`, `🧪️w1-d-parity-ts-output.txt`):

```
[ok] owner first, then pluginId ascending (aec-building < norm)
[ok] duplicate AppRef -> surface.conflict (code=surface.conflict)
[ok] unknown dialect -> surface.unknown-dialect (code=surface.unknown-dialect)

All checks passed
```

Also re-ran and updated lane 1-B's own scratch verifier (`🧪️w1-b-verify.ts`, renamed-method call
sites updated) — 21/21 still passing. Full-repo `bunx tsc --noEmit -p tsconfig.json`: same 19
pre-existing errors as lane 1-B's own report (four files this lane never touched), zero new. `bun nx
run @semio-tech/framework:test`: 150/150 passed, unchanged.

### Parity table

| contract rule | Rust location | TS location | agreed?/how pinned |
|---|---|---|---|
| `(ArtifactDialect, AppRole) -> Vec<AppRef>` | `AppRouter::surfaces_for` (`🦀️component.rs:1790`) | `AppRouter.entriesFor` (`🟦️component.ts:496`) | ✅ identical shape |
| owner-first, then `plugin_id`/`app_id` ascending | `surfaces_for` (`:1790-1804`) | `AppRouter.build`'s final sort (`:474-491`) | ✅ identical algorithm, verified by the shared fixture |
| owner claimed from `artifact_kinds` OR first app to touch an unclaimed kind, single interleaved pass | `register_manifest` (`:1755-1785`) | `AppRouter.build` (`:426-499`, fixed this lane) | ✅ fixed to match — was the #1 drift |
| duplicate `AppRef` ⇒ `surface.conflict` | `register_manifest` (`:1774-1781`, checked SECOND) | `AppRouter.build` (checked SECOND, fixed this lane) | ✅ fixed check order to match |
| non-owner contribution without `dependencies` ⇒ `surface.contribution-not-permitted` | `register_manifest` (`:1764-1773`, checked FIRST) | `AppRouter.build` (checked FIRST, fixed this lane) | ✅ fixed check order to match |
| `OpeningResolver::resolve` 4-step precedence | `OpeningResolver::resolve` (`:2039-2054`); steps 2/3 collapse by construction | `resolveOpeningApp` (`:612-624`); steps kept literally separate | ✅ same outputs (verified by fixture + existing 4-precedence tests on both sides); Rust documents the collapse, TS keeps 4 explicit checks — a stylistic, not behavioral, difference |
| owned-subset-missing-both-roles diagnostic | `AppRouter::owned_surface_gaps` (`:1836`, pure, returns `Vec<Fault>`, never panics) | `AppRouter.ownedSurfaceGaps` (`:516-528`, renamed from throwing `assertOwnedSurfacesComplete` this lane) | ✅ fixed to match — was throwing on first breach instead of collecting all |
| `FaultOrigin::Framework` for all five `surface.*`/`viewer.*` codes | `dsl::diagnostic::FaultOrigin::Framework` (`⚠️diagnostic/🦀️component.rs:149`, lane 1-A) | `FaultOrigin` union (`🟦️component.ts:727`, added this lane) | ✅ fixed to match — was a type-assertion workaround |
| `surface_app_id`/`parse_surface_app_id`, `ArtifactDialect::to_coordinate`/`parse_coordinate` | `🛂️manifest/🦀️component.rs:2678-2688`, `🚪️io/🦀️component.rs:67-81` | `surfaceAppId`/`parseSurfaceAppId`, `dialectCoordinate`/`parseDialectCoordinate` (`:290-351`) | ✅ already byte-exact ports (lane 1-B), no drift found |

## Deliverable 2 — wgpu shell role awareness (contract §5)

Lease: `🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/{🦀️shell.rs,🦀️chrome.rs,🦀️component.rs}` (framework,
domain-neutral) and `💻️os/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/**` (OS product).

**Key finding before writing any code**: neither directory had ANY role/session/viewer/editor
vocabulary yet (confirmed by a repo-wide search of both trees for `SEMIO_APP_ROLE`, `AppRole`,
`Session`, `ContextMenu` role terms — zero hits) — lane 1-C's React shell and this lane started from
the same blank slate concurrently, as the brief said.

### What landed — `ui_wgpu` (domain-neutral framework, real + tested)

`🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️component.rs`, new `pub mod role_chrome` (inserted
between `pub mod utilities` and `pub mod ui`, ~230 lines):

- `ChromeRole { Viewer, Editor }` — local wire-compatible mirror of `semio_framework::AppRole`/the TS
  host's own mirror (this crate cannot depend on `semio_framework` — domain-neutral boundary, see the
  `wgpu` feature's `Cargo.toml` deps). `as_str()`, `is_read_only()`, `from_boot_env(Option<&str>)`
  (contract §5's `SEMIO_APP_ROLE`/`VITE_SEMIO_APP_ROLE` semantics: `"viewer"` → Viewer, everything
  else including `None` → the frozen default `Editor`).
- `role_title_chip_text(role, is_de)`, `open_with_label_text(is_de)`,
  `set_as_default_label_text(is_de)` — the three frozen en/de string pairs, English first, mirroring
  the pre-existing `ribbon_parent_label(category, is_de)` idiom already used for this crate's other
  framework-owned two-locale strings.
- `OS_OPEN_ARTIFACT_WITH`/`OS_SET_DEFAULT_VIEWER`/`OS_SET_DEFAULT_EDITOR`/`OS_CLEAR_DEFAULT_APP`
  (contract §3) and `PALETTE_OPEN_ARTIFACT_WITH_VIEWER`/`_EDITOR` (contract §5) — frozen id constants.
- `OpenWithEntry { plugin_id, app_id, label, role, is_default }` + `open_with_menu_item(entries,
  is_de) -> ContextMenuItemSpec` — builds the "Open with…" submenu, entries grouped by role (viewer
  group first, then editor — `AppRole`'s own declaration order), each headed by a labeled separator
  (reusing this file's own `context_menu_is_header` convention), each entry carrying a nested "Set as
  default" toggle child that dispatches `OS_SET_DEFAULT_VIEWER`/`_EDITOR` when turning default ON or
  `OS_CLEAR_DEFAULT_APP` when turning it OFF (the toggle direction is resolved here, not left to the
  host, since no single OS command flips a boolean). Purely a renderer of already-resolved entries —
  never talks to a real `AppRouter`/`ConfigStore` (same domain-neutral boundary as everything else in
  this crate).
- `filter_shell_menu_actions_for_role(actions, role)` — drops every `ShellMenuAction` whose `kind ==
  "Mutation"` for `ChromeRole::Viewer` (contract: "hides every `Mutation`-kind action").
- `apply_role_to_utilities(utilities, role)` — forces `disabled: Some(true)` on every
  `UtilityCategory::History` utility (undo/redo/checkpoint) for `ChromeRole::Viewer` (contract:
  "disables undo/redo"); every other utility passes through unchanged.
- 7 unit tests (`role_chrome_tests`) covering all of the above — see Verification.

`🦀️shell.rs`:

- `Shell` gained `window_roles: HashMap<String, ChromeRole>` and `locale: Locale` fields, plus
  `set_window_role`/`clear_window_role`/`window_role`/`set_locale` (each re-runs
  `set_window_layout` on the current layout if one is set, so a role/locale change applied after a
  layout is already up repaints immediately, without the caller re-supplying the whole layout).
- `build_root`/`build_axis`/`build_stack`/`build_window` now thread a small `ShellPaintContext`
  (bundling `window_kind_icons`, `window_roles`, `locale`) instead of the bare
  `window_kind_icons` map they threaded before.
- `build_window`: a window whose id resolves a `ChromeRole` gets the frozen title-chip text appended
  to its tab label (`"{title} · Viewer"` / `"{title} · Betrachter"` etc.), and — for
  `ChromeRole::Viewer` only — its icon swaps to `IconName::Lock`, standing in for the read-only badge
  (this widget has exactly one icon slot per window cap).
- 5 new tests covering: no-role paints nothing extra, viewer role appends chip + swaps to lock icon,
  editor role appends chip but keeps its own window-kind icon, a role set AFTER the layout already
  repaints immediately, and German locale resolves "Betrachter".

`🦀️chrome.rs`: `push_read_only_badge(draw, icons, theme, rect)` — a small lock-icon chip pinned to a
rect's top-right corner, for whoever later paints a window's own in-canvas title chrome (not this
lease) and wants the same badge there; distinct from `shell.rs`'s tab-icon swap, which lives one
level up in the declarative `WindowLayout` vocabulary.

### What landed — `semio-framework-os-renderer-wgpu` (OS product, real but NOT compile-verified — see gap)

`📦️glue.rs`, new `//#region 🔖️RoleBoot` (next to the existing `ICON_ATLAS_RUNTIME` thread_local,
same idiom): `BOOT_APP_ROLE` thread-local defaulting from `SEMIO_APP_ROLE` on native
(`std::env::var`, mirrors the pre-existing `SEMIO_PLUGIN_MODULES` read in `📦️bin.rs`) / `Editor` on
wasm boot (wasm has no env access); `#[wasm_bindgen(js_name = semioWgpuSetAppRole)]
semio_wgpu_set_app_role(role: String)` lets the JS boot script set it; `boot_app_role()` reads it
back. Deliberately additive — does not change `run_native`'s or `semio_wgpu_mount`'s existing
signature (see gap below for why).

`🟦️typescript/🟦️boot.ts`: `resolveBootAppRole()` (mirrors the pre-existing `resolveBootLocale()`'s
own "no shell state exists this early in boot" reasoning) reads `import.meta.env.VITE_SEMIO_APP_ROLE`
defensively — **this target is Trunk-served (`Trunk.toml`), not Vite-bundled**, so that read is a
harmless `undefined` unless a deployment wraps this boot module through a Vite dev server; a `?role=`
URL param is the always-available fallback, mirroring the file's own pre-existing `?plugin=`
(`bootVariant`) idiom. Called (guarded, `bindings.semioWgpuSetAppRole &&`) right before
`semioWgpuMount`, so the very first `Shell::set_window_layout` a real wiring would trigger already
carries the role.

### Verification

Full log: `🧪️w1-d-cargo.txt`.

- **`ui_wgpu` (`semio-framework-ui`) — `cargo check -p semio-framework-ui --lib --features
  wgpu-engine`: clean, 0 errors**, confirmed 3 times across the session (including once immediately
  after every shell.rs/component.rs/chrome.rs edit). This is the feature that actually compiles the
  three files touched — `default = []` skips them entirely, so this is the correct verification
  target, not a guess.
- **`cargo check -p semio-framework-ui --all-targets --keep-going --features wgpu-engine`: 89
  pre-existing errors, ZERO from this lane's code** (grepped the full output for `role_chrome`,
  `ChromeRole`, `ShellPaintContext`, `set_window_role`, `only_window_button` — no hits). Every error
  is `label_impl::Label: From<&str> is not satisfied` across 12 files this lane never touched
  (`engine.rs`, `draw.rs`, `paint.rs`, `scene_slots.rs`, `tree.rs`, `widgets.rs`, `cursor.rs`,
  `events.rs`, `flex.rs`, `reconcile.rs`, plus two spots in `component.rs` far from the `role_chrome`
  region) — confirmed pre-existing via `git status`: none of those 12 files show as modified this
  session, meaning the breakage is already baked into the last commit (a `Label` compile-time-checked-
  label migration, tickets 26/08/03/COMPILE-TIME-CHECKED-UI-LABELS… and
  26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE, in flight elsewhere). **This means the new
  `role_chrome_tests`/`shell.rs` role tests are typechecked but NOT actually executed** — the crate's
  whole test binary cannot link while those 89 unrelated errors exist. Honestly reported as such, not
  claimed as passing.
- **`semio-framework-os-renderer-wgpu` — could not get ANY check to run**, not even a bare `cargo
  check -p semio-framework-os-renderer-wgpu --lib`: it fails several dependency layers down, first in
  `semio-framework-os-kernel` (`unresolved import crate::os_pack::index` — transient, self-resolved on
  retry), then in `semio-s-plugin-puzzle` (a direct dependency, 3 real errors from a `dsl::Mutations`
  derive macro) which in turn depends on the SAME live stdio-plugin refactor lane 1-A's own report
  named ("FULL-STDIO ticket") — confirmed via `git status -- ✏️s/🔌️plugins/🗄️stdio/`: **70+
  uncommitted files under `🗿️artifacts/🧊️gltf/…`**, all live edits by another session, none of them
  touched by this lane. The renderer crate's own doc comment (written by an EARLIER session, `📦️glue.rs`
  ~line 1211, predating this lane's edits) independently confirms: "this crate does not currently
  build clean (a concurrent, unrelated `dsl`/`store` import break)". Given this, the `📦️glue.rs`/
  `🟦️boot.ts` boot-role addition is real, carefully hand-verified against the crate's own working
  precedents (`ICON_ATLAS_RUNTIME`'s identical thread_local idiom; `upload_icon_atlas`'s identical
  `#[wasm_bindgen(js_name = …)]` idiom; the external-path resolution `ui_wgpu::wgpu::component::
  role_chrome::ChromeRole` independently traced through `🖱️ui/📦️packages/🦀️rust/📦️glue.rs`'s
  `#[path = "🎯️targets/🧊️wgpu/📦️glue.rs"] pub mod wgpu;` mount point to confirm the path is correct)
  but **NOT compiler-verified**, for reasons entirely outside this lease.
- **`🟦️boot.ts`**: `bunx tsc --noEmit -p tsconfig.json` — same 19 pre-existing errors as deliverable
  1's report (four files, none of them `boot.ts`), zero new.

### What is NOT done, and why (most material gap first)

1. **No real session ever calls `Shell::set_window_role`/`set_locale`, and no real `AppRouter` data
   ever reaches `open_with_menu_item`.** The actual orchestration that knows "this window is bound to
   this `(artifact_ref, AppRef)` at this role" lives in `🧑️‍🎨️engine/🧱️elements/Shell/🧊️component.rs`
   — mounted into `📦️glue.rs` as `pub mod shell` but its physical file sits OUTSIDE this lease's
   granted path (`🎯️targets/🧊️wgpu/**` only; `🧱️elements/**` is a sibling directory four levels up).
   This lane built real, tested, ready-to-call primitives at every layer up to that boundary
   (`ChromeRole`, the role-aware `Shell` API, `open_with_menu_item`) and a real boot-role resolution
   that a caller inside that file can read via `boot_app_role()` — but did not cross into a file this
   lease does not own, per the ticket's own "stop and report, don't improvise" rule (lane 1-A's own
   precedent for `PluginHost`/`semio-framework-os` was the same shape of gap).
2. **The new role-aware tests in `ui_wgpu` are typechecked, not executed** — see Verification. Not
   claimed as passing.
3. **The renderer product's boot-role wiring (`📦️glue.rs`, `🟦️boot.ts`) is not compiler-verified at
   all** — the crate's entire dependency graph is currently unbuildable for reasons unrelated to this
   lane (live FULL-STDIO plugin refactor + a transient `os_pack` hiccup). Hand-verified as carefully as
   possible without a compiler; flagged, not silently claimed as passing.
4. **`chrome.rs`'s `push_read_only_badge` has no caller yet** — it's a primitive for whoever paints a
   window's own in-canvas chrome, which (like item 1) lives outside this lease.

## Files touched

- `🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts` — `AppRouter.build` ownership/check-order fix,
  `FaultOrigin` gained `"framework"`, `surfaceFault` cast removed, `assertOwnedSurfacesComplete`
  renamed to `ownedSurfaceGaps`, one stray NUL-byte fix (deliverable 1).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs` — new parity test in
  `app_router_tests` (deliverable 1).
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️component.rs` — new `pub mod
  role_chrome` region (deliverable 2).
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️shell.rs` — role-aware `Shell`
  API + window-cap chrome (deliverable 2).
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️chrome.rs` — `push_read_only_badge`
  (deliverable 2).
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs`
  — `//#region 🔖️RoleBoot` (deliverable 2).
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🟦️typescript/🟦️boot.ts`
  — `resolveBootAppRole` + mount-time wiring (deliverable 2).
- `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET/🧪️w1-d-parity.ts` — new,
  TS half of the parity test (kept, ticket scratch convention).
- `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET/🧪️w1-d-cargo.txt` — new,
  full cargo verification log.
- `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET/🧪️w1-d-parity-rust-output.txt`,
  `🧪️w1-d-parity-ts-output.txt` — new, captured parity test outputs.
- `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET/🧪️w1-b-verify.ts` — updated
  two call sites for the `ownedSurfaceGaps` rename; re-ran, still 21/21.
