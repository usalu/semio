# Lane 0-A (manifest spine) — Report

Ticket: `26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET`. Contract: `📋️contract-freeze.md` §1 C1.
Lease: `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs` (crate **`semio-framework`**, confirmed via
`🧰️framework/📦️packages/🦀️rust/Cargo.toml:2`, lib path `📦️glue.rs` which mounts this file at
`#[path = "../../🔨️modules/🛂️manifest/🦀️component.rs"] pub mod manifest;`).

## What landed

All in `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs` unless noted.

1. **`AppRole`** — `Copy, Eq, Hash, Serialize, Deserialize, rename_all = "camelCase"`, ts-rs under
   `typegen`, in new `//#region 🔖️Surface` (~:2627, right before `AppDefinition`).
   `as_str()` → `"viewer"`/`"editor"`; `impl std::str::FromStr` (`Err = String`) accepts only those
   two spellings.
2. **`AppRef { plugin_id: String, app_id: String }`** — same derive set, camelCase, same region.
3. **`AppDefinition`** gains two REQUIRED fields, no `#[serde(default)]`: `pub role: AppRole` and
   `pub dialect: ArtifactDialect` (~:2696-2700, right after `pub id: String`). Imported
   `ArtifactDialect` at file top via `use crate::ArtifactDialect;` (~:15-18) — confirmed it is the
   owned/`String`-based type from `🧰️framework/🔨️modules/🚪️io/🦀️component.rs:50`, re-exported at
   the crate root in `📦️glue.rs:83-93`, per contract §7.1 (not the `&'static str` `Dialect`).
4. **`surface_app_id`/`parse_surface_app_id`** in the same `//#region 🔖️Surface`. Built on
   `ArtifactDialect::to_coordinate()`/`::parse_coordinate()` (already `"<kind>@<standard>/<subset>"`)
   plus a `#<role>` suffix/strip via `rsplit_once('#')`. Grammar matches contract examples exactly
   (`s.cad.cad@1/*#editor`).
5. **`PanelTabKind::SettingsDefaultApps`** appended after `SettingsTheme` (~:2582-2586);
   `id_str()` arm → `"framework.settings.default-apps"` (~:2601).
6. **Manifest-side builder**: confirmed `AppBuilder::build_definition` exists ONLY in
   `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:4371` (crate
   `semio-framework-plugin`, lane 0-B's lease — grepped, not touched). There is no manifest-side
   builder to change. Per the ticket's fallback instruction: `surface_app_id` is `pub fn` at the
   crate root of `semio-framework` and reachable as `semio_framework::surface_app_id` /
   `crate::ui::surface_app_id` (re-exported via `pub use manifest::*;` in `📦️glue.rs:96`).
   **0-B must call `semio_framework::surface_app_id(&E::DIALECT.into(), E::ROLE)` inside
   `AppBuilder::build_definition` to set `id`, and reject a hand-written `id`** — not done by this
   lane, SDK file is out of lease.
7. Unit tests, new `//#region 🔖️SurfaceTests` inside the existing `mod app_label_tests`, right
   before `exports_typescript_bindings`:
   - `surface_app_id_round_trips_through_parse_surface_app_id` — fixture set: subset `*`
     (`s.cad.cad@1/*#editor`), dotted standard `1.7` (`s.stdio.png@1.7/a#viewer`), hyphenated
     artifact kind (`s.stdio.dwg-2d@1/cc6#editor`).
   - `parse_surface_app_id_rejects_missing_hash_and_unknown_role`
   - `app_role_serde_wire_strings_are_exactly_viewer_and_editor` — asserts literal `"viewer"`/`"editor"`.
   - `app_role_as_str_and_from_str_round_trip`
   - `panel_tab_kind_settings_default_apps_id_str`
   - `app_ref_serde_round_trips_as_camel_case` (extra, not explicitly required but covers C1's other new type)
8. **Typegen wiring**: added `crate::ui::AppRole::export().unwrap();` and
   `crate::ui::AppRef::export().unwrap();` to the `#[cfg(feature = "typegen")] fn
   exports_typescript_bindings` test (right before the existing `AppDefinition::export()` call,
   ~:5735-5737) so `bun nx run @semio-tech/framework:generate` will pick both new types up once the
   crate-wide typegen build succeeds (see "NOT done" below).
9. **In-file test fixtures fixed** (required — these ARE inside my lease file):
   `app_with` (the shared `AppDefinition` builder used by ~15 tests in `app_label_tests`) now sets
   `role: AppRole::Editor` and `dialect: ArtifactDialect { artifact_kind: "s.test.a", standard: "1",
   subset: "*" }`.
10. **Out-of-lease-file-but-same-crate fixtures fixed**: `🧰️framework/🔨️modules/🖥️platform/🦀️component.rs`
    lines 99-101 (`adds_first_app_as_active`) and 154-156 (`minimal_app` helper) — both are test
    fixtures inside `semio-framework` itself, matching the ticket's explicit carve-out ("tests,
    fixtures, defaults... they are in your crate"), and `🖥️platform/🦀️component.rs` is not listed
    as contended by any peer in `📋️ownership-and-handoffs.md`. Added `role:
    crate::ui::AppRole::Editor` and a synthetic `dialect: crate::ArtifactDialect { .. }` to both,
    fully qualified (no new `use` needed). This was the only way to get `cargo check -p
    semio-framework --all-targets` to a clean exit — the two files are compiled into the same crate,
    so leaving them broken would have kept the whole crate (including my own new tests) uncompilable.

## Commands run and results

```
RUSTC_WRAPPER="" cargo check -p semio-framework --all-targets --keep-going
```
Iterated 3 times while wiring up imports (`AppRole`/`AppRef`/`ArtifactDialect`/`surface_app_id`/
`parse_surface_app_id` needed adding to the `app_label_tests` mod's explicit `use crate::ui::{...}`
list — that mod does NOT `use super::*;`, it name-lists everything). **Final run: `EXIT:0`, zero
errors, only pre-existing warnings** (two `dead_code` warnings at manifest ~:1463/1472 that predate
this change, unrelated). Full output captured at
`🧪️w0-a-cargo.txt` in this ticket folder (overwritten each run; last run is the clean one).

```
RUSTC_WRAPPER="" cargo test -p semio-framework --lib app_label_tests::
```
`68 passed; 0 failed` — includes all 6 new tests listed above, all `ok`.

```
RUSTC_WRAPPER="" cargo test -p semio-framework --lib platform::
```
`5 passed; 0 failed` — confirms the `🖥️platform/component.rs` fixture fix didn't just compile, it
still asserts correctly (`adds_first_app_as_active` etc.).

One transient, unrelated failure observed mid-session and NOT chased: a `cargo test -p
semio-framework --lib` run hit `E0004 non-exhaustive patterns` in
`🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️component.rs:404` over
`AppCommand::{OpenArtifact,SetDefaultApp,ClearDefaultApp}` — that's lane 0-C's live, in-progress
contract §3 channel work (tags 27-29) in a file I do not own and `📋️ownership-and-handoffs.md`
assigns to 0-C. Re-running the mandated `cargo check -p semio-framework --all-targets --keep-going`
seconds later came back clean (`EXIT:0`), consistent with a peer session mid-edit rather than
anything in my lease. Not touched.

## Broken construction sites for W2 (authoritative + supplementary)

**Authoritative (from the compile-clean `cargo check -p semio-framework` gate above → 0 remaining in
that crate; the 2 that existed were `🖥️platform/component.rs:99,154`, both fixed by this lane per
item 10 above).**

**Supplementary, repo-wide `grep -rn "AppDefinition {" --include="*.rs" .` (unverified by compile —
each is a separate crate outside `semio-framework`, not run by my gate):**

| file:line | crate (best-effort) | note |
|---|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:4371` (`build_definition` itself) and `:4665` | `semio-framework-plugin` | lane 0-B's lease |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🧊️component.rs:5574` | os renderer (part of `semio-framework-os-kernel` module tree — not independently confirmed) | test fixture `test_app` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Dock/🧊️component.rs:1410` | same as above | test fixture `sample_app` |
| `🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs:978,1036,1080,1147,1277,4357,4466` | `semio-framework-os` (`🖥️host/📦️packages/🦀️rust/Cargo.toml:2`) | mixed: production app registration (978, 1036, 1080, 1147, 4357) + test fixtures (1277, 4466) |

Excluded from the table: matches under `.🦑️repo/🎫️tickets/**` — those are historical/scratch
snapshots from other tickets (`before/`, `pre-patch.rs`, `ws-b-iso/` copies), not live source.

W2 should re-grep at its own start rather than trust this table verbatim — it is a snapshot taken at
commit `dbcc4fa462` and other lanes are live-editing concurrently.

## NOT done, and why

- **`bun nx run @semio-tech/framework:generate` was not run.** It shells out to `cargo test
  --features typegen exports_typescript_bindings`, which compiles the WHOLE `semio-framework` crate
  test target under the `typegen` feature — including `semio-framework-os-kernel` as a dependency.
  At the moment of my gate runs that dependency intermittently failed to compile due to lane 0-C's
  in-flight channel work (see "transient failure" above), which is entirely outside my lease and
  crate. Consequently `🧰️framework/🔨️modules/🛂️manifest/🤖️generated/🟦️manifest.ts` was **not
  regenerated** — the new `AppRole`/`AppRef` TS types and the two new `AppDefinition` fields are not
  yet reflected there. The export calls are wired (item 8 above) so regeneration is a single command
  away once the crate compiles cleanly under `--features typegen`: run
  `bun nx run @semio-tech/framework:generate` from repo root.
- **0-B's `AppBuilder::build_definition` was not changed to call `surface_app_id`** — that file is
  explicitly out of my lease (`🔌️plugin/🦀️component.rs`, two peer sessions live in it). Recorded as
  a blocking dependency for 0-B in item 6 above.
- **The renderer/host/plugin construction sites in the supplementary table are not fixed** — by
  design (W2's job per the ticket's explicit "do NOT chase" instruction), and also outside my crate
  (`semio-framework-os`, not `semio-framework`).
- **No `FromStr`-based error type richer than `String`** was added for `AppRole` (contract doesn't
  specify one; `parse_surface_app_id` itself returns `Result<_, String>`, matched for consistency).

## Files touched

- `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs` (in-lease)
- `🧰️framework/🔨️modules/🖥️platform/🦀️component.rs` (same crate, test-fixture carve-out — see item 10)
- `🧪️w0-a-cargo.txt` (this ticket folder, scratch — final gate output, `EXIT:0`)
