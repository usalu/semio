# W1b — `semio-s-plugin-space` (`🪐️space`) — `.artifact(declaration())` conversion

`apa-status: partial` — the conversion itself (Steps 1-4) is done and verified error-free in
isolation (zero errors anywhere touching `.artifact(declaration())`/`ArtifactDeclaration`/`.setup()`);
marked partial only because `cargo check -p semio-s-plugin-space --all-targets` is not currently
green — 8 `(lib)`/12 `(lib test)` errors remain, all measured (mtime + git log) as pre-existing,
unrelated to registration, and outside any file this session touched (see "Pre-existing, unrelated
errors" below). Not something I fixed blind or left silently — reported in full with evidence.

## Clearance

Read `📓️plugin-release-status.md` (SMO ledger, #2545). `🪐️space` appears only under **RELEASED**
(facet `🏠️home`, SMO's mutation-triad lane) — not under **HELD**. Per that ledger's own explicit
rule ("absence from this file means free, not held" / an entry under RELEASED, not HELD, blocks
nothing), `🪐️space` was free for this APA lane. Proceeded.

## What changed

### 1. `declaration()` — the artifact's own engine
`✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs`

- Deleted `register_artifact_schema()` (old :15-17) and `register_artifact_inference()` (old :27-31)
  — both side-effecting, zero call sites left after folding into the declaration (repo-wide grep
  confirmed their only caller was the root's `register_pilot_languages()`, itself deleted below).
- Deleted `register_io()` (old `//#region 🔖️IoFacet`, old :72-75) — confirmed **orphaned before this
  change**: `grep -rn "engine::register_io\(" ✏️s 🧰️framework` found zero call sites anywhere in the
  repo. This artifact's composer table was never actually pushed into the process-global `io`
  registry until this conversion — `.composers(...)` in the new `declaration()` is the first live
  wiring of `SHomeComposer`'s 5-entry table (1 import + 4 export), not a like-for-like replacement of
  working code.
- Kept `artifact_schema_registered()` unchanged (old :19-22) — a read-only query helper (checks
  `"s.space.home"` against the schema registry), not a registration side effect; nothing in the
  dispatch asks for query helpers to move.
- Added `pub fn declaration() -> semio_framework_plugin::ArtifactDeclaration`:
  ```rust
  ArtifactDeclaration::builder("s.home")
      .schema(crate::artifacts::home::schema::home_artifact_schema_descriptor())
      .inferences([crate::artifacts::home::standards::v1::subsets::any::schema::inferences::home_artifact_inference_descriptor()])
      .composers(crate::artifacts::home::standards::v1::engine::io_registry::entries())
      .languages(pilot_languages())
      .document_codec::<crate::apps::home::HomeApp>()
      .build()
  ```
  **`kind = "s.home"`** — matches `HOME_DIALECT.artifact_kind` (the composer table's own dialect,
  `io_registry` region below) and `S_HOME_DOCUMENT_SCHEMA`, which is what `register_all`'s "always
  enforced" ownership-check layer actually checks composer entries against. This artifact has **three
  different on-disk kind strings** for nominally the same thing — `"s.home"` (dialect/document
  schema), `"space.shome"` (the OS-level `ArtifactKindSpec.id` + 2 of the 5 DSL language ids), and
  `"s.space.home"` (the schema descriptor's own `id` field, matching `artifact_schema_registered()`'s
  hardcoded check string). This mirrors exactly the pre-migration inconsistency the W1 mechanism
  report documented for note's `"s.note"` — not fixed here (out of scope: renaming any of these three
  strings is a UCAS/SMO kind-canonicalization concern, not an APA declaration-mechanism concern), just
  measured and recorded so the choice of `"s.home"` isn't mistaken for arbitrary.
- Added `fn pilot_languages() -> &'static [dsl::LanguageSpec]` — the same 5 `LanguageSpec` literals
  (`space.shome`, `space.shome.op`, `space.shome.diff`, `home.pack`, `home.spr`) moved verbatim from
  the root's deleted `register_pilot_languages()`, built once behind the file's existing `OnceLock`
  import (reused, not re-imported) since `dsl::passthrough_hooks` isn't `const fn` — same pattern as
  note's own `pilot_languages()` helper.

### 2. Artifact root — dropped the now-superseded free function
`✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🦀️component.rs`

- Deleted `register_pilot_languages()` (old `//#region 🔖️Register`, old :13-68) in full — its only
  call site was `📦️glue.rs:477`, updated below. `artifact_kind()` (`//#region 🔖️ArtifactKind`) is
  untouched — a *different*, `PluginBuilder`-level mechanism (`.artifact_kind(ArtifactKindSpec)`),
  not part of `ArtifactDeclaration` at all (see finding below: this plugin's `plugin()` never actually
  calls it, pre-existing and unrelated to this conversion).
- The root's own `pub mod io_registry { entries()/compose()/register() }` (old :90-113, unchanged) is
  now **fully orphaned** — its only caller was `engine::register_io()`, deleted above; confirmed via
  `grep -rn "home::io_registry::" ✏️s/🔌️plugins/🪐️space` (zero hits outside the module's own
  definition). Left in place rather than deleted, exactly matching the note exemplar's own precedent
  for its analogous orphaned `io_registry` module — removing it is unrelated cleanup outside this
  wave's scope. Flagged here for whoever next touches `🏠️home`.

### 3. Plugin root — `.setup()` narrowed, `.artifact()` added
`✏️s/🔌️plugins/🪐️space/🦀️component.rs`

Before: `plugin()` called a bare `crate::register_s_exports();` **before** the `Plugin::builder`
chain even started — not even routed through `.setup()`, a more direct escape hatch than every other
plugin converted so far.

After:
```rust
pub fn plugin() -> Plugin {
    Plugin::builder("s")
        .label("S Studio")
        .version("0.1.0")
        .local_backbone_storage()
        .setup(crate::register_s_exports)
        .artifact(crate::artifacts::home::engine::declaration())
        .register_document_app::<crate::apps::home::HomeApp>(crate::apps::home::create_home_app())
        .register_document_app::<crate::apps::space::SpaceApp>(crate::apps::space::create_space_app())
        .build()
}
```

### 4. `register_s_exports()` — narrowed to the two things `ArtifactDeclaration` cannot express
`✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/📦️glue.rs:472-485`

Before (5 calls): `home::register_pilot_languages()`, both apps' `register_app_schema()`,
`register_document_codec_for_app::<HomeApp>("s.home")`, `register_document_codec_for_app::<SpaceApp>(OS_SPACE_SCHEMA)`.

After (3 calls):
```rust
fn register_s_exports() {
    apps::home::config::schema::register_app_schema();
    apps::space::config::schema::register_app_schema();
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<apps::space::SpaceApp>(semio_framework_os::OS_SPACE_SCHEMA);
}
```
The two `register_pilot_languages()`/`HomeApp` codec calls moved into `declaration()` (§1). The
**three** remaining calls stay in `.setup()` for two distinct, both-legitimate reasons:
- Both `register_app_schema()` calls — app-scope config/presence schema, the exact exclusion the M1
  mechanism report documents by name (`register_app_schema_descriptor` is not in the §6 artifact-scoped
  registrar set — see that report's field-mapping table).
- `SpaceApp`'s document codec — **a new exclusion class this plugin surfaces that note's exemplar
  didn't**: `SpaceApp` wraps the kernel-owned `WorkflowSnapshot` (`s.space`/`OS_SPACE_SCHEMA`) and owns
  **no `🗿️artifacts` node in this plugin at all** — this crate has exactly one (`🏠️home`), by design,
  documented in `📦️glue.rs`'s own module doc ("`🪐️space`'s own app owns no document type at all …
  there is only ONE `🗿️artifacts` node in this crate"). `.document_codec::<A>()` only exists on
  `ArtifactDeclarationBuilder`, i.e. it requires an owning declaration; `SpaceApp` has none to attach
  one to. Reported prominently as instructed, since it's a third reason (beyond app-schema) `.setup()`
  survives here.

## `.setup()` survives — exactly why, in full
Three calls, two reasons, both structural gaps in `ArtifactDeclaration` (not scope-creep): app-scope
config/presence schema (×2, matches the exemplar's own documented exclusion) and one app's document
codec for a kind it doesn't declare as its own artifact (`SpaceApp`/`s.space`, new to this report).

## Escape hatches (Step 4)

### `register_app_io` — verified: 1 production call site, kept and reported (not converted)
Grepped `register_app_io\(` across the whole plugin: **3 syntactic hits, only 1 is production code**:
- `🎛️apps/🪐️space/⚙️engine/🦀️component.rs:242`, inside `pub fn apply_app_registrations(json: &str)`
  — **production**, not inside `#[cfg(test)]`.
- `🎛️apps/🪐️space/📌️panels/🛍️catalogue/🦀️component.rs:120` and
  `🎛️apps/🪐️space/🦀️component.rs:605` — both are `seed_app()` test helpers inside
  `#[cfg(test)] mod tests` / `#[cfg(test)] pub(crate) mod testkit` respectively. Not production.

The plugin-specific note said "it had two — verify they are gone or convert them"; measured state
today is exactly **one** production call site (the other appears to have already been reduced to a
test helper before this session, or the "two" referred to something already resolved — not
reconstructable from git blame without risking a destructive command, so recorded as measured, not
assumed).

**Why the surviving one is not converted or deleted**: `apply_app_registrations` is a wasm-instance
boundary bridge, not domain-kind IO. Its own doc comment (unedited, already on-disk) explains: the
`🪐️space` app compiles to its own WASM component with a statically-linked, memory-isolated copy of
`semio_framework_os::APP_REGISTRATIONS`; a real browser host pushes every `{pluginId, app}` pair
`PluginHost::load_plugin`/`hot_swap_plugin` already registered natively as JSON, and this function
replays those exact entries into the wasm instance's own copy so `os_app_registration` resolves
inside the sandbox. It does not invent IO for a kind this plugin doesn't own — it forwards
already-registered `AppDefinition`s (arbitrary plugin/app pairs, not this plugin's own) verbatim from
host to sandbox. `register_app_io`'s own signature (`fn(plugin_id: &str, app: &AppDefinition)`, backed
by `APP_REGISTRATIONS: HashMap<(String, String), OsAppRegistration>` at
`🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs:4231-4236`) has no relationship to
`ComposerEntry`/`Dialect` at all — it is not the mesh/solid/dwg-exporter shape Step 4 targets, and
`ArtifactDeclaration` has no field that could express "mirror arbitrary other plugins' app
registrations into my own wasm sandbox." Left as-is; flagged here per the dispatch's "report it
prominently" instruction rather than silently kept or riskily deleted without a replacement
mechanism.

The two OTHER `register_os_media_export_handler_kind("2d.drawing", …)` /
`register_dwg_import_handler("2d.drawing", …)` calls in
`🎛️apps/🪐️space/🎮️commands/🖼️media/🦀️component.rs:121,126` — checked, also **test-only**
(`#[cfg(test)] mod tests`, seeding a fake handler for the export/import media dispatch test). Zero
production calls to either. Not touched.

### `register_studio_port` — already excluded by design, unchanged
`🎛️apps/🏠️home/🦀️component.rs:110-118` (called from `🎛️apps/🏠️home/🎮️commands/🏙️studio/🦀️component.rs`).
Already named as an exclusion in the M1 mechanism report's own field-mapping table
(`"register_studio_port | none | excluded by design — not a framework SDK fn at all; pub(crate) to
🪐️space's own 🏠️home app, registers into a plugin-local static"`) — this plugin IS `🪐️space`, so
that exclusion was written specifically about this call site. No action needed; confirmed unchanged.

### `semio_framework_os` dependency — NOT purged
`grep -c "semio_framework_os::" ✏️s/🔌️plugins/🪐️space --include='*.rs'` → **87 hits across 27
files**. This plugin's own module doc calls it "the OS host plugin" — it legitimately depends on deep
`semio_framework_os` internals throughout both apps (workflow graph ops, media export/import,
backbone ports, app registry). Left `semio-framework-os` in `📦️packages/🦀️rust/Cargo.toml` per Step
4's own instruction ("otherwise leave it and file the needed SDK re-export"). No SDK re-export filed:
87 call sites spanning workflow/media/backbone/registry APIs is not a short, curatable list a
handful of re-exports could replace — this is this plugin's core function as the OS host, not a
leaked implementation detail.

### Missing `.artifact_kind()` call — pre-existing, unrelated, reported not fixed
`crate::artifacts::home::artifact_kind() -> ArtifactKindSpec` (root file, `id: "space.shome"`) is
defined but **never called anywhere in this plugin** — not from `plugin()`'s `PluginBuilder`-level
`.artifact_kind(...)`, not from either app's own `App::builder(...).artifact_kind(...)`. This is a
pre-existing gap, orthogonal to the M1 conversion (`.artifact_kind()` is a `PluginBuilder`/`AppBuilder`
method, not an `ArtifactDeclaration` field — confirmed by reading both signatures at
`🔌️plugin/🦀️component.rs:1308` and `:7307`). Not fixed here: out of this dispatch's Steps 1-6, and
fixing a apparently-dead registration path without understanding what (if anything) currently
compensates for it risks an unreviewed behavior change. Reported for SMO/APA follow-up.

## Step 3 — plugin root already closed
`✏️s/🔌️plugins/🪐️space/` top level: `🎛️apps/`, `🗿️artifacts/`, `📦️packages/`, `🦀️component.rs` —
nothing else. No `🛂️manifest/`/`🎟️capabilities/`/`🔧️setup/` dirs exist. No action needed.

## Step 5 — inventory

**`thread_local!`**: exactly one repo-wide, `🎛️apps/🪐️space/🦀️component.rs:568`
(`STUDIO_TEST_APP: RefCell<SpaceApp>`), inside `#[cfg(test)] pub(crate) mod testkit`. Test-only, not
production app state.

**Interior-mutable / host-handle statics** (`static … OnceLock<…>`), all repo-wide hits:
| location | holds | class |
|---|---|---|
| `🦀️component.rs:24` `FIXTURES: LazyLock<()>` | nothing (unit) — a `Once`-style registration guard for `register_os_fixture_json` | registration idempotency guard, not state |
| `🎛️apps/🏠️home/🦀️component.rs:69` `PORT: OnceLock<Arc<MemoryBackbonePort>>` (`temp_catalog_port_concrete`) | an in-process `MemoryBackbonePort` instance | **host/engine-handle class** — an I/O port engine, session-local ephemeral by its own doc comment |
| `🎛️apps/🏠️home/🦀️component.rs:74` `REGISTRY: OnceLock<Arc<Mutex<HashMap<String, Arc<dyn OsBackbonePort>>>>>` (`shared_studio_ports`, backs `register_studio_port`) | a map of `Arc<dyn OsBackbonePort>` — host port trait-object handles, keyed by space id | **host/engine-handle class** — the distinct violation class the dispatch asks to count separately from mere mutability |
| `🎛️apps/🪐️space/🦀️component.rs:116` `REGISTRY: OnceLock<Arc<Mutex<HashMap<String, HashMap<String, SPresencePeerLocal>>>>>` (`shared_presence_peers`) | live multi-user cursor/selection presence data, refreshed on `SetClient`/`SetSelection`/`Snapshot` config mutations, 15s staleness window | **ephemeral shared** app state (per CLAUDE.md's local-only/shared × persisted/ephemeral matrix) — genuinely not derivable from the document snapshot, so it does NOT belong in an inference; it is presence, not a cache |
| `🗿️artifacts/🏠️home/🦀️component.rs:38` `ENTRIES: OnceLock<Vec<&'static ComposerEntry>>` | pure, deterministic composer-entry references | build-once static data, not app state |
| `⚙️engine/🦀️component.rs:46` `LANGUAGES` / `:150` `ENTRIES` (both new/pre-existing `OnceLock<Vec<…>>`) | pure, deterministic declaration data | build-once static data, not app state |

Two genuine host/engine-handle-class statics found, both in `🎛️apps/🏠️home/🦀️component.rs` (the
`register_studio_port` mechanism), already named as an accepted exclusion elsewhere in this report —
counted here per the dispatch's request, not newly flagged as a violation to fix.

**`std::fs`/`std::env`/`std::process`/`Command::new` outside `#[cfg(test)]`**: zero hits repo-wide in
this plugin (`grep -rn` returned nothing at all, test or production).

## A trivial, compiler-suggested pre-existing fix (mirrors the note exemplar's precedent)

`cargo check --all-targets` reaching this plugin's real code for the first time in this session (see
§Step-6 retry log below) surfaced 3 real, pre-existing, unrelated compile errors — same situation the
M1 mechanism report documented for note ("first time it ever linked far enough to reach them").
Fixed the one that was a safe, compiler-exact one-liner; left the rest (see next section).

**Fixed**: `🎛️apps/🏠️home/🎮️commands/🏙️studio/🦀️component.rs:7` and
`🎛️apps/🏠️home/🎮️commands/🗂️vfs/🦀️component.rs:7` both imported
`change_catalog_generation` from `crate::artifacts::home::op::{…}` — the DSL text-grammar shim
(`op` = `pub use …schema::mutations::text::*`), which never actually contained this mutation
constructor. Compiler's own suggestion (`E0432`, "consider importing this module through its public
re-export instead: `crate::artifacts::home::mutations::change_catalog_generation`") applied verbatim
in both files:
```rust
use crate::apps::home::config::{HomeConfig, HomeConfigMutation};
use crate::artifacts::home::mutations::change_catalog_generation;
use crate::artifacts::home::op::SHomeMutation;
use crate::artifacts::home::SHomeSnapshot;
```
Confirmed pre-existing, not caused by this session: both files' mtime is `Aug 12 14:37` (hours before
this session's ~15:57+ edits) and `git log --oneline -3` on both shows only unrelated commits
(`…492`/`…480`/`…469`). Confirmed the fix worked: this exact error class (3 hits) is present in
`scratch-w1b-space-check-3.txt` (before) and absent from `scratch-w1b-space-check-4.txt` (after),
with the plain-lib error count dropping from 10 → 8 between those two identical-otherwise runs.

## Pre-existing, unrelated errors found — NOT fixed (reported, per note's own precedent for real bugs)

Two consecutive `cargo check --all-targets` runs (`scratch-w1b-space-check-3.txt`,
`scratch-w1b-space-check-4.txt`) show the **same stable 8 `(lib)` / 12 `(lib test)` errors**, none
touching any file this session edited, all with mtimes ≤ `Aug 12 10:50` (hours before this session).
None are about registration/declaration mechanics — all are external type-shape changes this
plugin's `🏠️home` IO leaves and one kernel test helper haven't been updated to match:

1. **`dsl::ArtifactEngine` no longer exists** (`grep -rln "trait ArtifactEngine" 🧰️framework` → zero
   hits anywhere in the framework tree) — used only inside this artifact engine's own
   `#[cfg(test)] mod tests` (untouched by this conversion), backing `SHomeEngine::apply/snapshot/
   artifact()`. The compiler's only "similar name" suggestion, `ArtifactLink`, is a different trait
   (composition links, not mutation-apply) and not a safe substitution — this looks like a trait this
   plugin's `SHomeEngine` test pattern depended on that was removed/renamed elsewhere in the kernel by
   another session; RELEASED puzzle (an SMO-migrated plugin) uses a completely different test pattern
   (`app.snapshot()` on the `ArtifactApp`, not a bespoke `*Engine::apply/snapshot/artifact` trait) with
   zero `dsl::ArtifactEngine`/`ArtifactLink` references anywhere in its tree — corroborating this is a
   retired pattern, not a rename I could 1:1 substitute. Left as-is; not mine to redesign.
2. **`OsAppRegistration` lost its `document` field** —
   `🎛️apps/🪐️space/📌️panels/🛍️catalogue/🦀️component.rs:79` (`row.document.clone()`); compiler lists
   `id, label, breadcrumb, controller_id, inputs, … +7 others` as the current fields. A kernel-side
   struct redesign; guessing which of the 12 real fields now means what `document` used to would be
   an unreviewed behavior change to the app catalogue tree, not a mechanical fix.
3. **`CsvSnapshot` was redesigned** (`headers`/`rows` → `has_header`/`records`) — breaks home's own
   csv import deserializer AND export serializer (4 errors:
   `🚪️io/📥️import/…/csv/🔖️rfc4180/✳️any/🦀️component.rs:9` reads `.headers`/`.rows`;
   `🚪️io/📤️export/…/csv/🔖️rfc4180/✳️any/🦀️component.rs:11` constructs with `headers, rows`). A real
   data-shape migration (`Vec<String> + Vec<Vec<String>>` → `bool + Vec<CsvRecord>`-shaped, unverified)
   this plugin's csv leaf never got updated for — substantive domain logic, not a rename.
4. **`JsonSnapshot.value` is now a `JsonValue` newtype, not `serde_json::Value`** — breaks home's json
   import/export leaves (3 errors: `🚪️io/📥️import/…/json/…/🦀️component.rs:10,21`;
   `🚪️io/📤️export/…/json/…/🦀️component.rs:10`), each a `serde_json::Value ⇄ JsonValue` boundary the
   leaf never got a conversion for.

All four are **measured** (mtime + git log, not assumed) to predate this session and to be untouched
by any file this conversion edited. None is a registration/`ArtifactDeclaration` concern — fixing
them means understanding and rewriting real CSV/JSON serialization logic and a kernel trait's
replacement, squarely out of this dispatch's Steps 1-6. Reported per the note exemplar's own
precedent for a genuine domain bug found-but-out-of-scope ("Left unfixed … a real domain bug, out of
M1's scope").

## Step 6 — verification

1. **`#[path]` resolution** — scripted (Python, resolves every `#[path = "..."]` in `📦️glue.rs`
   relative to its own directory): **150 attributes checked, 0 missing.**
2. **`include_str!`/`include_bytes!` resolution** — scripted, walked every `.rs` file under the
   plugin, re-resolved each target against the real file (not pattern-substituted): **63 targets
   checked, 0 missing.**
3. **`cargo metadata`**:
   ```
   $ cargo metadata --no-deps --format-version 1 >/dev/null && echo OK
   OK
   ```
4. **`cargo check -p semio-s-plugin-space --all-targets`** (`RUSTC_WRAPPER=""`,
   `CARGO_TARGET_DIR=".../🎯️target"`) — retry log (concurrent-churn protocol, matching the note
   exemplar's own precedent of retrying rather than patching a shared dependency mid-churn):
   - **Attempt 1** (`scratch-w1b-space-check-1.txt`, last 100 lines): failed entirely inside
     `semio-s-plugin-stdio` (`couldn't read …🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs`) —
     zero mentions of `🔌️plugins/🪐️space` anywhere in the output. SMO's own ledger names this exact
     failure mode (stdio mid-rename, `SetSnapshot`).
   - **Attempt 2** (`scratch-w1b-space-check-2.txt`): stdio still red, different error this time
     (`SemioDrawingMutation: OpBinary` not satisfied) — errors changing between retries while never
     mentioning space, the same converging-churn signature the note report documented. Zero mentions
     of `🔌️plugins/🪐️space`.
   - **Attempt 3** (`scratch-w1b-space-check-3.txt`): stdio finally compiled clean — first real look at
     `semio-s-plugin-space` itself: **10 `(lib)` / 14 `(lib test)` errors**, all in files this session
     never touched (see previous two sections for the categorized breakdown and fix).
   - **Fixed** the one safe compiler-exact one-liner (previous section).
   - **Attempt 4** (`scratch-w1b-space-check-4.txt`, current state):
     ```
     $ RUSTC_WRAPPER="" CARGO_TARGET_DIR=".../🎯️target" cargo check -p semio-s-plugin-space --all-targets
     ...
     error[E0432]: unresolved import `dsl::ArtifactEngine`
     error[E0609]: no field `document` on type `&OsAppRegistration`
     error[E0609]: no field `headers` on type `&CsvSnapshot`
     error[E0609]: no field `rows` on type `&CsvSnapshot`
     error[E0308]: mismatched types (JsonValue/Value, ×2)
     error[E0560]: struct `CsvSnapshot` has no field named `headers`
     error[E0560]: struct `CsvSnapshot` has no field named `rows`
     error[E0308]: `?` operator has incompatible types (JsonValue/Value)
     error[E0599]: no method named `apply` found for struct `SHomeEngine`
     error[E0599]: no method named `snapshot` found for struct `SHomeEngine`
     error[E0599]: no method named `artifact` found for struct `SHomeEngine`
     error: could not compile `semio-s-plugin-space` (lib) due to 8 previous errors; 18 warnings emitted
     error: could not compile `semio-s-plugin-space` (lib test) due to 12 previous errors; 29 warnings emitted
     ```
     **8 `(lib)` / 12 `(lib test)` errors remain, all pre-existing and unrelated** — see previous
     section for the full per-error breakdown and evidence. **Zero errors anywhere in
     `.artifact(declaration())`, `ArtifactDeclaration`, `.composers()`, `.languages()`,
     `.document_codec()`, or `.setup(crate::register_s_exports)`** — every symbol and call this
     conversion introduced compiles clean; the only errors are in unrelated pre-existing csv/json IO
     leaves, one kernel test trait, and one app catalogue panel field.

   This crate is **not currently fully green**, but not because of anything this conversion did —
   every remaining error is measured, categorized, and traced to code this session never touched,
   blocked on external type redesigns (`CsvSnapshot`, `JsonValue`, `OsAppRegistration`,
   `dsl::ArtifactEngine`) owned by other sessions/tickets.

## sharedFileRequests

None with an ask attached — every file touched is inside `✏️s/🔌️plugins/🪐️space/` (this plugin's own
tree). No framework files, no other plugin's files, no `🧬️mutations/**`, no artifact-kind-id renames.

For visibility only (not this plugin's to fix): the 4 pre-existing blockers above trace to
`semio_framework_os_kernel` (`dsl::ArtifactEngine` removed/renamed) and to whatever owns
`CsvSnapshot`/`JsonValue`/`OsAppRegistration`'s current shape (stdio and/or the OS kernel) — flagged
in full in "Pre-existing, unrelated errors" above so whoever owns those tickets can pick them up.

## apa-status: partial
