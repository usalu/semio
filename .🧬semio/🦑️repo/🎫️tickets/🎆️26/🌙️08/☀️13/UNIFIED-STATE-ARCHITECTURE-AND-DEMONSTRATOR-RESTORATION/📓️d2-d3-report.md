# 📓️ D2 + D3 — Demonstrator IO-registration ownership & pane dissolution

Scope: waves **D2** (move foreign-kind IO registrations to their owning plugins, ratchet the policy
from downgrade to ban) and **D3** (dissolve `🎪️panes/` into `🎛️apps`, regenerate the plugin registry).

---

## 1. D2 — IO-registration ownership

### 1.1 Verification of the premise (grep, before touching anything)

`grep -rn --include='*.rs' -E "register_mesh_exporter|register_mesh_importer|register_mesh_dwg_export_handler|register_dwg_import_handler|register_solid_exporter|register_solid_importer|register_2d_export_handlers" "✏️s/🔌️plugins"`

Confirmed the premise exactly:

- `🎪️demonstrator/🎪️panes/📐️koordinator/🦀️component.rs:20-29` held **all ten** `"3d.cad"`
  registrations (3 solid exporters, 3 solid importers, mesh exporter, mesh importer,
  mesh-dwg-export, dwg-import). `📐️cad` registered **nothing** anywhere — grep over the whole
  `✏️s/🔌️plugins/📐️cad` tree returned zero registrar hits.
- `🎪️demonstrator/🎪️panes/🗺️verfolgen/🦀️component.rs:19-20` held **both** `"2d.map"` registrations
  (`register_2d_export_handlers`, `register_dwg_import_handler`). `🌍️gis` registered **neither**.

So these were sole registrations sitting in the wrong layer, and the latent bug is real: a standalone
`cad-play` / `gis2d-play` booted outside the demonstrator bundle had **no** solid/mesh/dwg/media IO at
all. Fixing ownership fixes that too.

The already-fixed panes (`🌱️generator`, `🏭️bearbeiten`) were confirmed as the shape to copy: a single
`register_document_codec_for_app::<App>(SCHEMA)` line each.

### 1.2 Owner-side shape (mirrors 🌀️procedural, no new `⚙️engine` dir)

`🌀️procedural`'s own self-registration is `apps::procedural3d::register_dwg_mesh_bridge()`, an
imperative fn reached from the plugin root's `.setup(register_exports)` — `ArtifactDeclaration` has no
field for these registrars (they sit outside APA §6's covered set). That plumbing is mirrored, with
the fn body placed in the owner's artifact `🚪️io` facet as instructed:

| owner | new fn | file | called from |
|---|---|---|---|
| 📐️cad | `register_host_io()` (10 registrations) | `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs` → new `//#region 🔌️HostIoRegistration` | `✏️s/🔌️plugins/📐️cad/🦀️component.rs` → new `register_exports()` + `.setup(register_exports)` |
| 🌍️gis | `register_host_io()` (2 registrations) | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs` → new `//#region 🔌️HostIoRegistration` | `✏️s/🔌️plugins/🌍️gis/🦀️component.rs` → new `register_exports()` + `.setup(register_exports)` |

No `⚙️engine` directory was created. `register_dwg_import_handler`'s `fn(&DwgDrawing)` signature was
**relocated, never changed** — the sibling ticket that owns it
(`26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS` G2b) is cited in the new gis
docstring.

`CAD_KIND`/`CAD_FORMAT` and `GIS_MAP_KIND`/`GIS_MAP_FORMAT` moved from private pane consts to `pub
const`s on the owning artifact's io facet.

`📐️cad/📦️packages/🦀️rust/Cargo.toml` gained `semio-framework-os` (with a comment naming the ticket
and the APA/M3 SDK-re-export path that will retire it). `🌍️gis` already had it.

### 1.3 Owner-side registration tests (extended existing test modules, no new files)

- **cad** — `cad_owns_the_host_io_registration_for_its_own_kind`, added to the existing
  `#[cfg(test)] mod tests` in cad's `🚪️io/🦀️component.rs` under a new `//#region 🔌️HostIoRegistration`.
  Asserts `CAD_KIND == crate::artifacts::cad::artifact_kind().id` (registrant == declarer), then
  `semio_framework_os::solid_exporter_for(CAD_KIND, fmt)` for `obj`/`stl`/`step`, then re-runs
  `register_host_io()` to pin idempotence. Runs **entirely inside the cad crate** — which is the
  property that was missing before.
- **gis** — `gis_owns_the_host_io_registration_for_its_own_kind`, added to the existing
  `#[cfg(test)] mod tests` in `🗿️artifacts/🗺️gismap/🦀️component.rs` (the gismap io facet has no test
  module; no new one was created). Asserts `GIS_MAP_KIND == artifact_kind().id`, runs
  `register_host_io()` twice, and exercises the DWG bridge fn it hands the OS
  (`gis2d_document_json_from_dwg`) on a one-point drawing, asserting the point lowers into a
  position feature.

  ⚠️ The first version of this test asserted on the **svg** bridge instead and **failed** (§3.3).
  `gis2d_document_json_to_svg` renders through `io_dispatch`, whose drawing→svg composer entry is
  registered by 🗄️stdio's plugin build — which never runs in a bare gis unit test. Asserting on it
  measured another plugin's registration, not gis's ownership. The rewrite to the DWG bridge (pure,
  gis-local) is the correct assertion and passes; the reason is recorded in the test docstring so
  nobody re-adds the svg assertion.

**Honest limitation, documented in both test docstrings:** the solid registry has a public predicate
(`solid_exporter_for`), the OS **media** handler map does not. The only reader is
`export_os_app_instance_media_kind`, which takes a `WorkflowNode` gated behind the `os-host-full`
feature (`default = []`), so plugins cannot reach it. Adding such a predicate would mean four edits
across `🖥️host/🦀️component.rs`'s cfg-split registries — a hot file another session had committed to
25 minutes earlier — so the media half is pinned by kind-ownership + bridge-execution assertions
instead of registry membership. Recommended follow-up: an
`os_media_export_handler_for(kind, format) -> bool` twin of `solid_exporter_for`.

### 1.4 Policy ratchet (root `📜️script.ts`, surgical edits, region re-read before each)

1. `POLICY_PLUGIN_DEP_OS_SYMBOLS["🎪️demonstrator"]` → `[]` with a comment stating the ban is a
   ratchet, not a downgrade.
2. `semio-framework-os` **and** `semio-framework-3d` dropped from
   `🎪️demonstrator/📦️packages/🦀️rust/Cargo.toml` (grep confirmed zero remaining
   `semio_framework_os` / `semio_framework_3d` references in the whole demonstrator tree). Replaced
   with a comment explaining why they must not come back.
3. `POLICY_PLUGIN_DEP_OS_SYMBOLS` rows for the new owners corrected to the real symbol lists —
   `"📐️cad"` from `[]` to its six registrars, `"🌍️gis"` extended with the two it gained. These are
   the factual data the breach `solution` text quotes; they are not carve-outs.
4. `policyRegistrationBreach`'s violation `reason` no longer asserts that 🎪️demonstrator registers
   kinds it never declares — it now records that half of the exemplar as RESOLVED by this ticket, so
   a future demonstrator hit reads as a regression rather than known backlog.
5. `policyDeclarativeRegistrationBreaches`'s docstring dropped `🎪️panes/` from its violation-site
   list (D3 removed the repo's only pane facet).

**There was no demonstrator-specific priority downgrade to remove.** The task brief pointed at
`📜️script.ts:~5487-5495`; that range is `policyRegistrationBreach`'s violation return, whose
`priority` is an unconditional `"medium"` with no plugin-keyed branch anywhere in the function. The
only demonstrator-specific content in that region was the stale `reason` exemplar, handled as (4).

---

## 2. D3 — Pane dissolution (not relocation)

`POLICY_PLUGIN_CLOSED_SHAPE_DESTINATIONS`' row proposing `🎪️panes/` → `🎛️apps/<app>/📌️panels/` was
**removed**, not followed: the six panes host FOREIGN apps, and a
`📌️panels/entwerfen-mit-bestand-*` under 📐️cad or 🌍️gis would push demonstrator identity into
plugins that must not carry it. That reasoning is recorded in the new `🎛️apps` module docstring so
the decision survives the deleted policy row.

- `✏️s/🔌️plugins/🎪️demonstrator/🎛️apps/🦀️component.rs` (was a one-line docstring stub) now carries
  `pub fn bundle(bundle: Plugin) -> Plugin` with all six export+app pairs inlined. **Order preserved:**
  all six export registrations run before any `register_document_app`, and the comment explaining why
  (process-global OS registries) was carried over into the fn docstring.
- The three bundle tests moved verbatim from `🎪️panes/🦀️component.rs` into the new file's
  `//#region 🧪️Tests`: identity, the six app ids in order
  (`["procedural3d-play","cad-play","puzzle3d-play","sourcing-curate","process3d-play","gis2d-play"]`),
  and non-empty document schemas.
- `🎪️panes/` deleted — all 7 files (`🦀️component.rs` + the six pane components).
- `📦️packages/🦀️rust/📦️glue.rs`: the `//#region 🎪️Panes` block (a 7-mod `#[path]` mount) was rewritten
  as `//#region 🎛️Apps` mounting the single `🎛️apps/🦀️component.rs`.
- `🎪️demonstrator/🦀️component.rs`: `crate::panes::bundle(plugin)` → `crate::apps::bundle(plugin)`,
  docstring updated.

Grep confirms no dangling `panes::` references anywhere in the crate, and the only remaining
`🎪️panes` string in the repo outside ticket folders is the policy docstring note added in §1.4/5.

---

## 3. Verification — exactly what was run

All cargo invocations used `RUSTC_WRAPPER=""` and `--all-targets`.

### 3.1 Baselines measured FIRST — and what they showed

`scratch-d2d3-baseline-check.txt`, `scratch-d2d3-baseline-errors.txt` (16:07 and 16:13).

**The baseline was red at the root, for every crate.** Not the brep blocker named in the brief (that
one had already cleared) but a newer one from a peer session: `semio-framework-os-kernel` failed with
`E0599: no variant … named Persistent/SharedUi/LocalUi/Preview/Effect/Inferred found for enum
wire::codec::StateClass` in `🔨️modules/📡️spr/🎮️command` and `🧾️wire`.

Attribution settled with a live predicate, not a derived artifact:
`git status --porcelain -- "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr"` showed four files
**uncommitted and mid-edit** (`🦀️component.rs`, `🧾️wire`, `📜️history`, `🧪️testkit`) — a peer wave
renaming the `StateClass` variants, consumers updated ahead of the enum. Per the wait-and-repoll rule
this was left alone. **Consequence: no per-plugin test baseline could be measured before editing** —
nothing compiled. That is a genuine gap in this report, caused by tree state, not skipped.

The blocker cleared on its own by 16:21 (the peer landed the enum), and every later run below is green.

### 3.2 `cargo check --all-targets` — after (`scratch-d2d3-check1.txt`, 16:21)

| crate | errors |
|---|---|
| `semio-s-plugin-cad` | **0** |
| `semio-s-plugin-gis` | **0** |
| `semio-s-plugin-demonstrator` | **0** |

`--all-targets` means both new tests compile.

### 3.3 Test suites

Scratch: `scratch-d2d3-tests1.txt` (first pass), `scratch-d2d3-tests2-gis.txt` (gis re-run after the
test rewrite), `scratch-d2d3-tests3-gis-svg.txt`, `scratch-d2d3-tests4-demonstrator.txt`.

| crate | result |
|---|---|
| `semio-s-plugin-cad` | ✅ **140 passed, 0 failed, 1 ignored** |
| `semio-s-plugin-gis` | ⚠️ **170 passed, 2 failed** — both failures pre-existing and peer-caused, see below |
| `semio-s-plugin-demonstrator` | ⛔ **blocked** — cannot build test binaries, see below |

**cad.** Green on the first run. The new test was confirmed to have actually executed, by name:

```
test artifacts::cad::standards::v1::subsets::any::io::component::tests::cad_owns_the_host_io_registration_for_its_own_kind ... ok
test result: ok. 45 passed; 0 failed; 1 ignored; 0 measured; 95 filtered out
```

**gis — my test.** The first run had it as the suite's ONLY failure (`171 passed; 1 failed`),
panicking at `🗿️artifacts/🗺️gismap/🦀️component.rs:310:108` — the `.expect("registered svg bridge
renders")` on `gis2d_document_json_to_svg`. Diagnosed as the io_dispatch/stdio-registration issue in
§1.3, rewritten onto the DWG bridge, and re-run. It now passes, confirmed by name:

```
test artifacts::gismap::component::tests::gis_owns_the_host_io_registration_for_its_own_kind ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 171 filtered out
```

**gis — the 2 remaining failures are NOT mine.**
`relocated_engine_tests::svg_export_of_an_empty_document_still_renders_a_bare_canvas` and
`svg_export_renders_real_svg_text_through_the_stdio_drawing_bridge`. They **passed** in the first
run of this wave (`171 passed; 1 failed` — only mine failed) and started failing between the two
runs. Root-caused from source, not inferred from timing:

The first panics at `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🦀️component.rs:94:5`, inside
`ArtifactKindId::parse(concat!("s.stdio.semio.", …)).expect("canonical semio subset kind")`.
`git diff HEAD` on that file shows the **entire enclosing function is new, uncommitted code** — a
peer session's `//#region 🔖️ChildStoreFactories` / `register_child_store_factories()`, added to
stdio's `register()` while this wave was running. The panic site did not exist at HEAD. The second
failure is the same svg path one frame further down. Neither touches anything D2/D3 changed — this
wave never edited stdio, `ArtifactKindId`, or any semio subset. Per the wait-and-repoll rule these
were left alone.

Confirmed experimentally as well as from source — run with a filter that excludes this wave's test
entirely (`cargo test -p semio-s-plugin-gis --lib relocated_engine_tests::svg_export`,
`scratch-d2d3-tests3-gis-svg.txt`), both still fail with nothing of mine in the process:

```
test …relocated_engine_tests::svg_export_renders_real_svg_text_through_the_stdio_drawing_bridge ... FAILED
test …relocated_engine_tests::svg_export_of_an_empty_document_still_renders_a_bare_canvas ... FAILED
test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 170 filtered out
```

**demonstrator — blocked, and by a different peer wave.** `cargo check --all-targets` was **green**
(§3.2). `cargo test` cannot get as far as demonstrator's own tests: it fails building the dependency
`semio-s-plugin-procedural` (lib) with **93 errors** — `unresolved import
crate::artifacts::procedural2d::widget_index` ×12, `cannot find module create_generation/
delete_generation/replace_widget/…`, `no variant Generation found for enum Procedural3dMutation`,
`no variant SetWidget`, `the name change_schema is defined multiple times`, …. That is a peer
session mid-refactor of procedural's mutation module tree (`git status` shows a large batch of
staged/modified files under `✏️s/🔌️plugins/🌀️procedural`). Nothing in D2/D3 touches procedural.

Re-polled ~50 minutes later with a direct probe
(`cargo check -p semio-s-plugin-procedural --lib`, `scratch-d2d3-tests4-demonstrator.txt`): still
broken, **91 errors** — the peer's refactor is still in flight, so the blocker stands rather than
having been a transient.

So the three bundle tests migrated into `🎛️apps/🦀️component.rs` are **compile-verified but not
run-verified**. They are the same three assertions, moved verbatim from a file that was passing, and
`cargo check --all-targets` proves they still typecheck against the new `bundle()`. **Re-run
`RUSTC_WRAPPER="" cargo test -p semio-s-plugin-demonstrator` once procedural compiles again** — that
is the one outstanding verification of this wave.

> Note on the build queue: the machine was saturated throughout (load average peaked at 118, six
> peer `cargo test` invocations queued on the same `target/debug/.cargo-lock`), so each of these runs
> took 10-40 minutes wall-clock. That is environment, not a signal about the change.

### 3.4 Registry regeneration

`bun "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/📇️registry/📜️script.ts" generate`
→ `plugin registry catalog refreshed (59 plugin crates, 58 playgrounds, 22 framework packages)`.

- `🤖️generated/🟦️playgrounds.ts` — **diff EMPTY**, as required. So are the other seven generated
  outputs (`🔣️framework.json`, `🔣️playgrounds.json`, `🔣️plugins.json`, `🟦️framework.ts`,
  `🟦️plugins.ts`, `🦀️artifacts.rs`, `🦀️hosts.rs`): `git status --porcelain` on the whole
  `🤖️generated/` dir came back empty. Playground variants/brands/ports live in `Cargo.toml`
  `[[package.metadata.semio.playground]]` rows, which were untouched — only the `[dependencies]`
  section changed.
- `.vscode/launch.json` — **diff NOT empty: 44 lines deleted.** Investigated rather than accepted.

**Root cause of the launch.json diff, and why it is not D2/D3's:** the regen removed four
`⚖️gate*` entries — `gate🪆️subset-conformance`, `gate📚️subset-examples`, `gate🧹phantom-standards`,
`gate🪞subset-ts-parity` (presentation group `4_gate`, orders 410.11–410.14). Those four exist in the
committed `.vscode/launch.json` but **not** in `.vscode/🧩️launch.seed.jsonc`, which is what the
generator reads. `grep` for `4_gate` across the repo hits only those two files; the generator
(`🖥️launch.ts`) contains no gate-name logic at all. So a previous session hand-edited the GENERATED
`launch.json` without adding the entries to the seed — the regen is correctly deleting rows the seed
never declared. The four `policy*Breaches` functions those gates call all still exist and are still
exported (`📜️script.ts:6060, 6107, 6135, 6183`), so the gates are wanted; only the seed is stale.

**Action taken:** `launch.json` was restored byte-for-byte to its `HEAD` content
(`git show HEAD:.vscode/launch.json > .vscode/launch.json` — a read-only git plumbing read plus a
file write; no `checkout`/`stash`/`commit`), so the four gates are **not** lost. D2/D3 needed nothing
added to `launch.json` (the playgrounds diff being empty proves it), so restoring is complete, not a
partial revert.

**Left for the owning session:** add those four gate entries to `.vscode/🧩️launch.seed.jsonc` so the
next `generate` stops deleting them. Not fixed here — it is another ticket's content and guessing at
their intended ordering/naming would be worse than reporting it.

> ⚠️ Concurrency note: between the regen and the restore, the repo's auto-commit staged the
> regenerated (gate-less) `launch.json` into the index, so `git status` reports `MM`. The **worktree**
> holds the correct gate-bearing content; the next auto-commit cycle (`git add -A`) re-stages it and
> the index self-heals. No `git add`/`reset` was run.

### 3.5 Policy breach counts (`scratch-d2d3-policy-after.txt`)

`bun ./📜️script.ts policy` — ran clean against the edited script (which is itself the syntax check
for the five `📜️script.ts` edits). 24 097 high-priority breaches across 31 rules, headed by 22 274
`handcrafted-grammar/spec-distinctness`.

**No high-priority row for any APA rule names 🎪️demonstrator, 📐️cad or 🌍️gis** — verified by
`grep -E "🎪️demonstrator|📐️cad|🌍️gis" | grep -E "plugin-dependency|plugin-registration|plugin-closed-shape|plugin-purity"`
returning nothing. So no ratchet regression was introduced:

- `plugin-dependency-os-host` ceiling **10** — my change is net-zero (demonstrator −1, cad +1), so
  still 10, no promotion to high.
- `plugin-dependency-allowlist` ceiling 105, **measured 118 → 13 regressions**. All 13 are
  `semio-framework` / `-geometry` / `-graph` / `-hash` / `-os-infinite` / `-mesh-engine` deps in
  🖨️raster, 🗄️stdio, 🧩️puzzle, 🪐️space, 🪵️sourcing (+2 of its extensions). **Pre-existing, none of
  them mine** — this wave touched neither those crates nor those dependency names.

`plugin-registration-*` carries no ratchet ceiling, so it is always `medium` and never printed by the
`policy` census (which reports high-priority only). Counted directly instead, replicating the rule's
own predicates (`POLICY_REGISTRATION_FAMILY_FNS` regex + `\bsemio_framework_os::`, comments stripped,
no `⚙️engine` site among these paths), `git show HEAD:<f>` for **before** vs worktree for **after**:

| file | before (family + os-path) | after |
|---|---|---|
| `🎪️demonstrator/🎪️panes/🦀️component.rs` | 0 + 0 | *deleted* |
| `🎪️demonstrator/🎪️panes/🌱️generator/…` | 1 + 0 | *deleted* |
| `🎪️demonstrator/🎪️panes/📐️koordinator/…` | **11 + 10** | *deleted* |
| `🎪️demonstrator/🎪️panes/🧩️aggregator/…` | 0 + 0 | *deleted* |
| `🎪️demonstrator/🎪️panes/🗂️aussuchen/…` | 1 + 0 | *deleted* |
| `🎪️demonstrator/🎪️panes/🏭️bearbeiten/…` | 1 + 0 | *deleted* |
| `🎪️demonstrator/🎪️panes/🗺️verfolgen/…` | **3 + 2** | *deleted* |
| `🎪️demonstrator/🎛️apps/🦀️component.rs` | 0 + 0 | **5 + 0** |
| `📐️cad/…/🚪️io/🦀️component.rs` | 0 + 0 | **10 + 12** |
| `🌍️gis/🗺️gismap/…/🚪️io/🦀️component.rs` | 0 + 0 | **2 + 2** |
| `🌍️gis/🗺️gismap/🦀️component.rs` | 2 + 0 | 2 + 0 *(pre-existing, untouched)* |

Roll-up:

| kind | 🎪️demonstrator before → after | 📐️cad | 🌍️gis |
|---|---|---|---|
| `plugin-registration-violation` | **29 → 5** | 0 → 22 | 0 → 4 |
| `plugin-registration-setup-callback` | 0 → 0 | 0 → 1 | 0 → 1 |
| `plugin-dependency-os-host` | **1 → 0** | 0 → 1 | 1 → 1 |

**Two results that do NOT match the brief's expectation, stated plainly:**

1. **Demonstrator registration breaches did not reach zero — they went 29 → 5.** The five survivors
   are the `register_document_codec_for_app::<ForeignApp>(FOREIGN_SCHEMA)` calls the brief itself
   specifies each pane shrinks to. They are structurally unavoidable in this bundle: 📐️cad's and
   🌍️gis's own `declaration()`s already carry `.document_codec::<…>()`, but the demonstrator never
   calls those plugins' `plugin()` — only their app factories — so nobody else registers those
   codecs for the bundled component. Retiring these five needs a mechanism change (a declaration
   form for "adopt a foreign app's codec"), not a relocation. **Dependency** breaches did reach zero.
2. **The rule's total breach count across the three plugins is roughly flat: 29 → 33.** The moved
   registrations keep breaching, because `policyRegistrationIsEngineSite` only exempts
   `🗿️artifacts/<kind>/…/⚙️engine/…`, and this wave was told (correctly) not to create one. The +4
   is 2 `.setup(` callbacks and 2 `semio_framework_os::solid_exporter_for` mentions inside cad's new
   test. What actually changed is the thing the rule's own `reason` cares about: **no plugin
   registers a kind it does not own any more.** No new breach *kind* appeared.

---

## 4. Files touched

**📐️cad**
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs` — new `//#region 🔌️HostIoRegistration` (consts + `register_host_io`), new test in the existing tests module
- `✏️s/🔌️plugins/📐️cad/🦀️component.rs` — `register_exports()` + `.setup(...)`
- `✏️s/🔌️plugins/📐️cad/📦️packages/🦀️rust/Cargo.toml` — `semio-framework-os` added

**🌍️gis**
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs` — new `//#region 🔌️HostIoRegistration`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🦀️component.rs` — new test in the existing tests module
- `✏️s/🔌️plugins/🌍️gis/🦀️component.rs` — `register_exports()` + `.setup(...)`

**🎪️demonstrator**
- `✏️s/🔌️plugins/🎪️demonstrator/🎛️apps/🦀️component.rs` — `bundle()` + the three migrated tests
- `✏️s/🔌️plugins/🎪️demonstrator/🦀️component.rs` — `apps::bundle`
- `✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🦀️rust/📦️glue.rs` — `🎪️Panes` region → `🎛️Apps`
- `✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🦀️rust/Cargo.toml` — `semio-framework-os` + `semio-framework-3d` removed
- **deleted:** `✏️s/🔌️plugins/🎪️demonstrator/🎪️panes/` (7 files)

**root**
- `📜️script.ts` — 5 surgical edits (closed-shape row removed, os-symbols table, registration `reason`, rule docstring)
- `.vscode/launch.json` — restored to HEAD after the regen; see §3.4

**scratch (this ticket folder)**
`scratch-d2d3-baseline-check.txt`, `scratch-d2d3-baseline-errors.txt`, `scratch-d2d3-check1.txt`,
`scratch-d2d3-tests1.txt`, `scratch-d2d3-tests2-gis.txt`, `scratch-d2d3-tests3-gis-svg.txt`,
`scratch-d2d3-tests4-demonstrator.txt`, `scratch-d2d3-policy-after.txt`

---

## 5. Outstanding / handoff

1. **Re-run `RUSTC_WRAPPER="" cargo test -p semio-s-plugin-demonstrator`** once
   `semio-s-plugin-procedural` compiles again (91 errors as of the last probe, peer wave in flight).
   That is the only verification this wave could not complete. `cargo check --all-targets` on the
   crate is green, so the three migrated bundle tests typecheck; they just have not been executed.
2. **`.vscode/🧩️launch.seed.jsonc` is missing four `⚖️gate*` entries** that exist only in the
   generated `launch.json` (§3.4). Until they are added to the seed, every `registry generate` run
   silently deletes them. Belongs to whichever session added those gates.
3. **Optional framework follow-up:** an `os_media_export_handler_for(artifact_kind, format) -> bool`
   twin of `solid_exporter_for`, so owner-side registration tests can assert media-registry
   membership directly instead of via kind-ownership + bridge execution (§1.3).
4. **Not fixed here, pre-existing:** `plugin-dependency-allowlist` is 13 over its 105 ratchet ceiling
   from other plugins' framework deps (§3.5).
