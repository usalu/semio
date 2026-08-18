# 📓️ Status — Clean Artifact Standard Subset Mechanism

**Coordinator single-writer file.** Agents: read this, never write it. Put your findings in your own `📓️w*-report.md`.

Ticket start commit: `101a6b4ea83acc82d6fbdc0607e6ae5d876825ae` (2026-08-17 15:59:36 +0200).
Anything newer than that in a file you did not touch belongs to a peer session — attribute with
`git log --date=iso -- <path>`, never by reading commit message text (auto-commit dates are fake templates).

## ✅️ W0 recon — DONE

- Ticket opened (`🪆️`, goal `🎯r2602🎯runningsketchpad`). ⚠️ `ticket_open` wrote to `.🦑️repo/…` (MCP cwd bug);
  canonical copy is this folder. Close with the explicit canonical path.
- `📌️important.md` + `📓️design.md` written. Inventory: `📓️w0-a-inventory.md`.

### Verified baselines (measured, not assumed)

| gate | baseline |
|---|---|
| `cargo check -p semio-framework-plugin --all-targets` | clean, 9 warnings (all from `derive_artifact_facets!`/`subset!` sites) |
| `cargo nextest -p semio-framework-plugin --lib --no-fail-fast` | **225 run / 221 pass / 4 fail** — 4 pre-existing at start commit, confirmed twice independently |
| `cargo nextest -p semio-framework-os-kernel --lib` | 996 / 996 pass |
| `cargo nextest -p semio-framework --lib` | 148 / 148 pass |
| repo-lib `bun test ./🧪️index.test.ts` | 188 run / **20 fail** (2 of them taxonomy: `osChildDirs` already carried `🎚️config`; `snapshotChildDirs` never existed — both pre-existing) |
| `bun ./📜️script.ts policy` | high-priority total dominated by a pre-existing grammar-spec collision from 2026-08-10, unrelated |

The 4 pre-existing SDK failures: `artifact_definition_contract_tests` ×3 + `plugin_builder_contract_tests::merge_channel_commands_…` ×1.

## ✅️ W1 framework F0 — DONE (3 agents, disjoint boundaries)

**W1-A io mechanism** (`📓️w1-a-report.md`) — vocabulary split into `🚪️io/🧬️schema/🦀️component.rs`, mounted ONCE in
os-kernel and re-exported by `semio_framework`; new `🔖️IoMechanism` region with `Serializer`/`Deserializer`/`IoEntry`/
`io_register`/`io_route`/`io_run`/`io_identify`/`io_entries` + 4 entry constructors; carrier law implemented in
`io_identify`. **8 law tests pass** (incl. a real failure→pass transition that exposed `same_io_entry` ignoring
`fidelity`). os-kernel 996/996, framework 148/148.
- ⚠️ Registry half still double-compiles (`io` + `os_io`) — unchanged from before, tracked as D2, shrinkable at W6.
- Deviation: `Deserializer::CONFORMANCE` is an associated const, not a constructor parameter — bare `fn` pointers
  cannot capture. Justified in the report.

**W1-B taxonomy + policies** (`📓️w1-b-report.md`) — taxonomy v6 allow-half (`🔨️modules` legal at plugin/artifact/
standard roots); 7 new report-mode policies; `new artifact|standard|subset` scaffolders. **0 blocking breaches added.**
Coordinator applied their `w1b-index-test-schema-version.txt` patch → repo-lib back to baseline.
- Deferred with a prepared paired patch: io native-codec vocabulary keys need the discovery walker taught the new
  shape first (`🔧️patches/w1b-discovery-io-native-codec-vocabulary.txt`) — **W2 prerequisite**.

### 📊️ Migration backlog (report-mode breach counts = the size of the remaining work)

| policy | breaches |
|---|---|
| `io-exclusivity` | **1132** |
| `subset-isolation` | **1117** |
| `owner-mounts-children` | **344** |
| `io-declaration` | **112** |
| `subset-standalone` | **61** |
| `module-consumer-count` | **59** |
| `declaration-tree` | 0 (dormant until subsets declare) |
| **total** | **2825** |

**W1-C SDK declaration tree** (`📓️w1-c-report.md`) — `app::declarations::{ArtifactDeclaration, StandardDeclaration,
MediaDeclaration, SubsetDeclaration, SchemaDeclaration, IoDeclaration, NativeCodecs, SurfaceDeclaration}` +
`editor_surface`/`viewer_surface` + `PluginBuilder::declare_artifact()` walking the tree + `SnapshotBuilder<S,M>` +
3 testkit laws + a two-standard/three-subset end-to-end fixture at **`crate::app::declarations::fixture`** —
that fixture is the executable spec every plugin wave copies from. **230 run / 226 pass / 4 fail** (same 4
pre-existing) and `cargo check --target wasm32-wasip2` clean.
- Coordinator follow-up: **deleted `IoDeclaration.conformance`**, which shipped inert "for shape parity" — dead API
  is forbidden here. `Deserializer::CONFORMANCE` is now the single conformance mechanism. Re-verified 230/226/4.
- Naming debt: the new builder method is `.declare_artifact()` because the old `ArtifactDeclaration` still owns the
  name `.artifact()`. **W6 renames it** once the old type is deleted → debt D5.


## ✅️ W2-P pilot (stdio carriers `💾️binary` + `📄txt`) — DONE, with two design-level findings

`📓️w2-p-report.md` + **`📓️recipe-subset.md`** (the template the ~40 fan-out agents follow).

### 🎯️ The central hypothesis is CONFIRMED on real code

`BinarySnapshot::encode_pack_with` wrapped the raw bytes in a `SemioEnvelope` (`BINARY_MAGIC` header + token) —
so every exported `.bin` was an unopenable `.semio` pack container instead of honest file bytes. `TxtSnapshot`'s
DSL prepended a preamble line. **This is exactly the `registry_export_media` class of bug the whole ticket exists
to remove, and it was sitting in the two dialects every other io leaf routes through.** Both codecs are now the
identity on their content (verified by reading
`💾️binary/…/🧬️schema/📸️snapshot/🦀️component.rs:73-82`), fixtures regenerated, and a `carrier_native_is_raw` law
test written for both artifacts.

### ⚠️ Finding 1 — the import/export mirror was NOT implementable (design corrected)

A single trait (`ArtifactDsl` = parse+print, `ArtifactPack` = decode+encode) can be impl'd exactly once per type in
Rust, so putting "the real native codec" under BOTH `📥️import/🧩️deserializers/📸️snapshot/📝️text` and
`📤️export/🧵️serializers/📸️snapshot/📝️text` forces either duplicated codec logic or a hollow re-export.
**`📓️design.md` §1 is corrected**: import/export expresses *direction*, which only exists for FOREIGN dialects;
the native codec is one bidirectional thing and sits directly at `🚪️io/<facet>/<representation>/`, unsplit.
User decision #2 stands (all bytes cross in `🚪️io`); only the internal shape changed. The physical file relocation
was therefore correctly NOT attempted by the pilot — it must be re-planned against the corrected shape, and
`🔧️patches/w1b-discovery-io-native-codec-vocabulary.txt` must be re-cut to match.

### ⚠️ Finding 2 — the registry wall (blocks EVERY stdio artifact, not just the pilot)

`🗄️stdio/📇️registry/🦀️component.rs` holds a rigid **36-artifact** `BTreeMap` of `fn` factories, cross-checked by
`schema_keys_and_runtime_factories_are_exact` against a 36-entry `SOURCES` array of `📜️artifact-definition.json`
includes. `plugin()` only ever reaches artifacts through that map, so a subset cannot become live on the new
declaration tree one at a time. The pilot offered an additive Option A (register through BOTH channels).
**Coordinator rejected Option A** — a second parallel registration channel is precisely the compatibility layer
CLAUDE.md forbids. **W2 must cut all 36 stdio artifacts over in one pass and delete `📇️registry` outright**, which
is what `📓️design.md` §2 already mandates. The new `artifact()`/`standard()`/`subset()` roots for binary and txt
are built, mounted and compiling — they are simply not yet the live channel, by design.

### Verification (honest)

| gate | result |
|---|---|
| `cargo check -p semio-s-plugin-stdio --all-targets --keep-going` (lib) | **0 errors** |
| `cargo check --target wasm32-wasip2` | clean |
| `cargo nextest -p semio-s-plugin-stdio` (test target) | ❌️ **cannot run** — 267 compile errors in the *test* target |
| `carrier_native_is_raw` | written + compiles; **NOT executed** |

The 267 errors are **not ours**: they are in `🧿️semio` (86), `🖊️dwg` (53), `🧊️gltf` (41), `📜️docx` (24), `📕️xlsx` (23),
`🏗️ifc` (15) — zero in `💾️binary`/`📄txt` — and `git status` shows **192 stdio files staged mid-flight** by the live
`26/08/16/FULL-STDIO-…` peer session. Per this ticket's own rule, we did not chase a moving target. **The carrier
law is proven by code inspection, not by a test run — that distinction is deliberate and must not be reported as
"verified by test" until the peer's stdio test surface lands.**

## Known live peer sessions (do not fight)

| ticket | area | evidence |
|---|---|---|
| `26/08/17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS` | rust warnings/errors repo-wide | own ticket folder, active today |
| `26/08/17/FINISH-HUB-SPACES-COLLABORATION-END-TO-END` | hub/spaces | active today |
| `26/08/16/FULL-STDIO-ARTIFACT-STANDARDS-CODECS-INFERENCES-AND-MUTATIONS` | stdio codecs/mutations | active 08/16–17 |

`🌎️hub/**` is read-only for this ticket until W6.

## Wave ledger

| wave | slice | agent | state |
|---|---|---|---|
| W0 | recon + baselines | coordinator + Haiku | ✅️ done |
| W1-A | framework io mechanism + vocabulary split | Sonnet | ✅️ done |
| W1-B | taxonomy v6 allow-half + 7 report-mode policies + scaffolders | Sonnet | ✅️ done |
| W1-C | SDK declaration tree + PluginBuilder walk + testkit + e2e fixture | Sonnet | ✅️ done |
| W1-D | WIT (`list-io-entries`/`io-run`/`io-sniff`, host `io-routes`/`io-run`/`io-identify`) + plugin host `IoRouter` + TS kernel mirror | Sonnet | ⏸️ **next** |
| W1b | OS host F1 (catalog on dialects, media via routes, shells) | Sonnet | ⏸️ |
| W2-P | pilot: stdio carriers `💾️binary`+`📄txt` + `📓️recipe-subset.md` | Sonnet | ✅️ done |
| W2 | stdio all 36 artifacts in one cutover + delete `📇️registry` (D6) | Sonnet ×6 | ⏸️ |
| W3 | exemplars shooting / norm-en1990 / cad | Sonnet ×3 | ⏸️ |
| W4 | ~29 plugin fan-out | Sonnet ×29 | ⏸️ |
| W5 | serializer (patches, catalog, launch.json, sweep) | Sonnet | ⏸️ |
| W6 | F2 deletion + policy ratchet | Sonnet ×2 | ⏸️ |
| W7 | verify + close | coordinator | ⏸️ |


## ✅️ W1-D WIT & host io routing — DONE

`📓️w1-d-report.md`. Guest exports `list-io-entries` / `io-run` / `io-sniff`; host imports `io-routes` / `io-run` /
`io-identify`; `resolve-artifact-link` untouched; every old WIT name kept (D3, W6 deletes).

- **`from` is a reserved WIT keyword** — wire params are `source`/`target`. Rust/TS internals keep `from`/`into`.
  Recorded as D10 so nobody "fixes" it back.
- Payload wire encoding: **JSON**, matching this interface's own existing precedent (`WireComposeSource` /
  `artifact-compose` already carry `IoPayload` as JSON). The pack-wire alternative was evaluated and rejected because
  `dsl::DslValue` has no `Bytes` variant, so `Vec<u8>` would serialize as one number per byte anyway. The resulting
  JSON-array blowup for large binaries is real and recorded as **D9**.
- **Reentrancy guard**: `IoRouter::run_io` resolves the whole route, then scans it for any hop owned by the calling
  plugin *before executing anything* — refuses the whole call, never partially executes. Stronger than the old
  one-hop self-refusal it generalizes.
- **Determinism across plugin load order**: `BTreeMap` graph + full-candidate-set sort that is never short-circuited.
  Proved by a 2-plugin fixture registered in both orders. `nextest` could not execute it (pre-existing compile
  blocker in `🎚️config/**`), so the algorithm was additionally run standalone via `rustc`: **13/13 checks pass**;
  the TS parity script over the same fixture: **11/11 pass**.
- Guest wasm target checked with `--features component-guest` — **the bare wasm command silently skips the guest
  code**, which is worth knowing for every later wave.

## ⚠️ W1b OS host — PARTIAL (Tasks 1-2 landed, Task 3 not)

`📓️w1b-report.md`. `registry_export_media`/`registry_import_media` now run through `io_identify` + `io_route` +
`io_run` to/from the carriers, and the host catalog is keyed on `ArtifactDialect` instead of the legacy
`"2d.shooting"`-style `ArtifactKindSpec` ids, so W6 can delete that type.

**Task 3 (shell wiring) was NOT completed** and the agent was right not to fake it: a real "Export as…" menu needs a
host-WIT→TS bridge and a plugin `shell_action` declaration, both outside its boundary. **No en/de strings were
invented for a menu that does not exist.** Consequence: `os_reachable_export_dialects`/`os_reachable_import_dialects`
exist with **no caller** → **D8: wire them in the shell wave or delete them.**

> The precedent from W1-C stands: dead API does not ship. The distinction here is that `IoDeclaration.conformance`
> was a *second copy of a live mechanism* (harmful, deleted immediately), whereas these two are the query surface of
> an explicitly unfinished task, recorded with an owner. If the shell wave does not land, they must be deleted.


## ✅️ W1-E taxonomy corrected to the real native-codec shape — DONE

`📓️w1-e-report.md`. W1-B's deferred patch encoded the **rejected** import/export mirror; it has been replaced.

- **No new walker code was needed.** The existing `ioSemanticCollectionDirNames` mechanism — already how
  `🧬️mutations`/`💡️inferences` sit directly under `🚪️io`, live on disk in gltf — just gained `📸️snapshot`/`🔺️diff`.
  Reusing the proven pattern instead of adding a branch to `artifactFacetChildLevel` is the better answer and it is
  what the corrected shape actually calls for.
- Also fixed **W1-B's `new subset` scaffolder**, which was still generating the rejected mirror — it would have
  stamped the wrong shape into every subset created from here on.
- `verify taxonomy enforce`: **10887 → 10789** (decreased). All seven `clean-mechanism/*` policy counts identical
  before/after (2830 total, **0 high-priority**).
- Coordinator applied their `w1e-index-test-io-semantic-collection-dirs.txt` patch (the one repo-lib assertion that
  legitimately had to change).

Two peer problems it hit and reported rather than absorbed: `plugin-registry:check` is broken by an off-by-two `../`
path bug from a third session, and `LANGUAGE-NEUTRAL-TAXONOMY-AND-PACKAGE-PURITY` left `discovery/component.ts`
uncompileable with a duplicate `const` (removed, as it blocked all verification).

**Resume step 2 is therefore done** — subsets can now move their native codecs without breaking taxonomy enforce.


## ▶️ W4 plugin fan-out — STARTED (the gate lifted)

**The peers landed.** HEAD moved to `abd29c08d0` (2026-08-18 10:38); `semio-framework` is green again (0 errors).
Working **in conjunction with** the ongoing peer work now rather than waiting on it.

### ✅️ D8 closed for real
`cargo nextest -p semio-framework-os --features os-host-full` → **110 run / 103 pass / 7 fail** — exactly the
predicted baseline. The dead `os_reachable_*` deletion is now compile-and-test verified, not just grep-verified.

### ✅️ `🎬️sequence` — FIRST COMPLETE PLUGIN CUTOVER, GREEN

`📓️w4-sequence-report.md`. **16 pre-existing compile errors → 0. Tests: could not run at baseline (crate did not
compile) → 146 run / 146 pass / 0 fail.** wasm32-wasip2 clean. Clean-mechanism breaches 18 → 10.

This is the proof the whole mechanism works end to end on a real plugin: declaration tree built, plugin root
**atomically** cut over to `.declare_artifact(...)` with the old `.artifact()`/`.editor()`/`.viewer()` channel
deleted in the same edit, all 8 foreign io leaves rewritten as typed `Serializer`/`Deserializer` impls with honest
`IoFidelity`, native codec physically relocated to `🚪️io/` per the CORRECTION, forbidden plugin-level shapes
(`🎟️capabilities`/`🔧️setup`/`🛂️manifest`) deleted.

**Two real bugs found only because the crate compiled for the first time:**
1. `sequence_snapshot_mutations` emitted a redundant `DisconnectSteps` for edges already cascade-deleted by
   `DeleteStep`, breaking the delete action in `RemoveStep`/`DeleteSelection`/`NodeGraphEdit`.
2. The CSV importer decoded incoming bytes as a `SequenceSnapshot` pack instead of a `CsvSnapshot` pack — it would
   have silently misbehaved on any real CSV import.

Both are the kind of defect that only surfaces when dead code becomes live code. Expect more of them.

### ▶️ In flight (parallel, disjoint trees)
`🌿️vcs` · `📋️forms` · `🖍️draw` · `🗒️note` — each briefed with `📓️w4-sequence-report.md` as the worked example and
its `## recipeGaps` as the trap list.

### Standing instruction for every fan-out agent
A plugin that is red **before** the cutover is the agent's to fix — those files are inside its boundary. Exit
condition is **0 errors + green tests**, not "no worse than baseline". `🎬️sequence` proved that is achievable.

## Final state of this session

The **framework mechanism is built end to end and green**. **No plugin is cut over.** That is not a scheduling
slip — it is a gate, described below.

### Confirmed green (measured, this session)

| gate | result |
|---|---|
| `semio-framework-os-kernel --lib` | **1003 / 1003 pass** (was 996; peers added 7, all pass) |
| `semio-framework --lib` | **148 / 148 pass** |
| `semio-framework-plugin --lib` | **246 run / 242 pass / 4 fail** — the 4 are byte-identical to the W0 baseline list; the suite grew 225→246 |
| `semio-framework-plugin --target wasm32-wasip2 --features component-guest` | 0 errors |
| `semio-framework-os --features os-host-full` | **110 run / 103 pass / 7 fail** — first ever recorded; all 7 pre-existing |
| **export-bug proof** | ✅️ **1/1 PASS** — exported bytes are raw, not a pack container |
| `semio-s-plugin-stdio --lib` + wasm | 0 errors |
| repo-lib `🧪️index.test.ts` | 20 fail = baseline (2 taxonomy failures verified pre-existing) |
| new policies | **0 blocking breaches added**; 2825 report-mode breaches = the measured backlog |
| io laws | 8/8 pass · io-router algorithm 13/13 · TS parity 11/11 |

### ✅️ THE EXPORT-BUG PROOF PASSES — executed, not inferred

```
cargo nextest run -p semio-framework-os --features os-host-full \
  -E 'test(export_via_io_mechanism_writes_raw_bytes_not_a_pack_container)'
PASS (1/1) host_core::workflow::tests::export_via_io_mechanism_writes_raw_bytes_not_a_pack_container
Summary  1 test run: 1 passed, 109 skipped
```
It calls the real production entry point `registry_export_media` and asserts the exported bytes are byte-identical to
the raw content and do **not** start with the pack magic `[0x89,'S','E','M',0x0D,0x0A,0x1A,0x0A]`. Evidence:
`🧪️w1b-export-proof.txt`. **The ticket's central claim is now verified by an executed test.**

Two things had to be fixed to get there, both worth keeping:
- `--lib` alone reports **"0 tests run"** for this crate — the `workflow` module sits behind
  `#[cfg(feature = "os-host-full")]`. Anyone measuring this crate without that feature is measuring nothing.
- The lib-test target would not compile: a peer commit (`5ac47258a6`, 21:07, after our start) dropped the import
  that brought `ConfigFieldShape` into scope in this file's own test helper. Fixed by qualifying it
  `semio_framework::ConfigFieldShape::`, exactly as its sibling on the same lines already was (4 lines).

### 📋️ First-ever recorded baseline for `semio-framework-os`

`cargo nextest run -p semio-framework-os --features os-host-full --no-fail-fast` → **110 run / 103 pass / 7 fail**
(`🧪️w1b-os-host-suite.txt`). **All 7 are pre-existing**, not this ticket's: the suite had never executed (feature flag
+ the compile error above), so nobody had seen them. Spot-checked the most suspicious one,
`mesh_exporter_registrar_round_trips_a_box_through_glb` ("unknown mesh export format kind `glb`") — it fails in
isolation too, and `git log -S` dates its failing line to `dbcc4fa462`, **2026-08-16 03:32, a day before this ticket
opened**. The other six are `space::`/`workflow::` fixture tests untouched by this work.

### ❗️ One test is WRITTEN BUT NOT EXECUTED — do not report it as verified

1. **`carrier_native_is_raw`** (stdio binary/txt). Blocked: the stdio *test* target has 267 errors in
   `🧿️semio`/`dwg`/`gltf`/`docx`/`xlsx`/`ifc` — **zero in binary/txt** — with **192 stdio files staged mid-flight**
   by the live `26/08/16/FULL-STDIO-…` session. The carrier fix itself is confirmed by reading the code
   (`💾️binary/…/📸️snapshot/🦀️component.rs:73-82`: `encode_pack_with` is now identity; it previously emitted a
   `SemioEnvelope` + `BINARY_MAGIC` header).
### 🩹️ Two peer collisions repaired (both in files this ticket owns)

Peer commit `5ac47258a6` (2026-08-17 21:07, after our start) changed `semio_framework`'s re-export surface and
renamed `HostEffect`→`Effect`. It left two of our files uncompilable, each of which silently hid a whole test suite:

1. **`semio-framework-os` lib-test** — `ConfigFieldShape` lost its import in that file's own test helper.
   Qualified to `semio_framework::ConfigFieldShape::` (its sibling on the same lines already was). Without this the
   crate's suite could not build at all, which is why the export-bug proof appeared unrunnable.
2. **`semio-framework-plugin` lib-test** — our `schema_stamping_tests` imported `LocalizedLabel`/`SurfaceKind` from
   `semio_framework` (no longer re-exported) and `Fault` from `crate::app` (now private). Repointed to
   `ui_wgpu::wgpu::{LocalizedLabel, SurfaceKind}` and `semio_framework::Fault`, matching how the SDK's own code
   imports them.

**Lesson for later waves**: a green `cargo check -p <crate>` says nothing about the *test* target, and this repo's
test targets are where the peer collisions land. Always run `--lib --no-fail-fast` (and for `semio-framework-os`,
`--features os-host-full`) before believing a crate is healthy.

### ✅️ D8 resolved — dead API deleted, not documented-and-kept

`os_reachable_export_dialects`/`os_reachable_import_dialects` are gone. They had no caller, and inspection showed
they were also **weaker than they claimed**: one-hop filters over `io_entries()`, so a shell built on them would
under-report exportable formats — real reachability is `io_route`, which the framework already exposes. Keeping a
subtly-wrong unused wrapper "for the shell wave" would have been worse than deleting it.

⚠️ **Verified by zero-reference grep, NOT by a compile.** A peer's *uncommitted* edit to
`🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs` (brand-new `ArgFormat`, unsatisfied serde bounds, ` M` in the
working tree at 23:12) reds `semio-framework` and therefore every crate below it. Re-run
`cargo nextest -p semio-framework-os --features os-host-full` when that clears — expected **110 / 103 / 7**.

### 🚧️ THE GATE — why no plugin was cut over

The cutover is **atomic per plugin** (a second parallel registration channel is the compatibility layer CLAUDE.md
forbids — see the Rejected approaches table in `📌️important.md`). Three independent conditions block it today:

1. **stdio is still red — measured after the peer committed.** The `FULL-STDIO-…` peer went from 192 staged files
   to 0 (committed) during this session, and `cargo check -p semio-s-plugin-stdio --all-targets --keep-going` still
   reports **268 errors — the identical count as before the commit**. The lib alone is clean (0 errors); it is the
   **test target** that is broken, which is exactly what blocks verifying any stdio migration. The earlier
   attribution (same 268) was `🧿️semio` 86 · `🖊️dwg` 53 · `🧊️gltf` 41 · `📜️docx` 24 · `📕️xlsx` 23 · `🏗️ifc` 15 —
   **zero in `💾️binary`/`📄txt`**, i.e. none in this ticket's files.
   Its 36 artifacts must cut over in one pass (D6), which cannot be verified while the test target does not build.
2. **Plugins are already red before we touch them.** `semio-s-plugin-sequence` has 17 errors whose file last changed
   at `1d71198c19` (14:44), *before* this ticket started — i.e. pre-existing, not peer-in-flight. That is exactly what
   the concurrent `26/08/17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS` ticket exists to fix.
   A full crate-health survey was started and abandoned: it was itself adding to the contention.
3. **The native-codec relocation shape changed mid-flight** (the ⚠️ CORRECTION in `📓️design.md` §1). The taxonomy
   patch `🔧️patches/w1b-discovery-io-native-codec-vocabulary.txt` must be **re-cut** against the corrected shape
   before any subset moves codec files, or `verify taxonomy enforce` breaks repo-wide.

Migrating 29 plugins + 36 stdio artifacts into that would produce thousands of unverifiable edits across files two
other sessions are actively rewriting. That is the opposite of what this ticket is for.

### Resume order for the next session

1. Finish `carrier_native_is_raw` once the stdio peer lands (the only unexecuted test left).
2. ~~Re-cut the native-codec vocabulary~~ — **done in W1-E.**
3. Resolve **D8** — wire `os_reachable_export_dialects`/`os_reachable_import_dialects` into a real "Export as…"
   shell action (with en+de strings), or delete them. They must not ship unread.
4. Wait for a green plugin baseline. `FULL-STDIO-…` committing did **not** fix stdio's test target (still 268
   errors), so W2 stays blocked on it and on `ZERO-WARNINGS-…`. The gate is `cargo check -p semio-s-plugin-stdio
   --all-targets` reaching 0 — check that first, before spending any agent on W2.
5. Then **W2 stdio** (all 36 artifacts + delete `📇️registry`, D6), then **W3/W4** plugin fan-out using
   `📓️recipe-subset.md`, then **W5** serializer, **W6** deletion + policy ratchet (debts D1-D7), **W7** verify + close.

### 📋️ W4 dispatch ledger

| plugin | baseline errors | state |
|---|---|---|
| `🎬️sequence` | 16 | ✅️ **done — 0 errors, 146/146 pass, wasm clean** |
| `🌿️vcs` | ? | ▶️ running |
| `📋️forms` | ? | ▶️ running |
| `🖍️draw` | ? | ▶️ running (also: `🔄️fsm` module decision, artifact-level `📚️examples` move) |
| `🗒️note` | ? | ▶️ running (also: `assembly-failed` manifest check) |
| `🪵️sourcing` | 0 | ▶️ running (also: manifest check, `🧩️extensions` decision) |
| `🕸️dag` | 1 | ▶️ running |
| `➗️mathematical` | 1 | ▶️ running (also: manifest check, non-canonical artifact id `a` → `s.mathematical.…`) |
| `🎞️animate` `📖️playbook` `💡️reasoning` `✒️writer` | 1 each | queued |
| `📜️imperative` | 3 | queued |
| `📏️layout` | 9 | queued |
| `🖨️raster` | 16 | queued |
| flow/process/fem/gis/lowpoly/remodel/puzzle/block/architect/energy/space/trinity/demonstrator/norm/shooting/cad/procedural | surveying | queued |
| `🗄️stdio` (36 artifacts, atomic, deletes `📇️registry`) | 268 (test target) | ⛔️ blocked — D6 |

**Health survey result (batch 2): most plugins are nearly green** — sourcing 0, animate/dag/mathematical/playbook/
reasoning/writer 1 each, imperative 3, layout 9, raster 16. The fan-out is far more tractable than the earlier
"plugins are already red" reading suggested; only stdio is genuinely hard.

### ⚠️ COORDINATION HAZARD FOUND — a `🔨️modules` promotion that is a Cargo workspace member breaks EVERY crate

The batch-3 survey reported "1 error" for all 17 remaining plugins. That number is **not plugin health** — it is a
single workspace-level failure:

```
error: failed to load manifest for workspace member `✏️s/🔌️plugins/🖍️draw/📦️packages/🦀️rust`
  Caused by: failed to load manifest for dependency `semio-s-plugin-draw-fsm`
```

`🖍️draw`'s `🔄️fsm` is its **own crate and a root-`Cargo.toml` workspace member**. While the draw agent is mid-move
(promoting it to `🖍️draw/🔨️modules/🔄️fsm` per design.md §4), the workspace manifest cannot resolve — so
`cargo check -p <anything>` fails for **every** crate in the repo, including crates that agent never touches.

**Consequences, binding for the rest of this wave:**
1. **`🧪️plugin-health-batch3.txt` is invalid** — measured during that window. Re-survey after draw lands; the real
   baselines are almost certainly 0.
2. A plugin whose module promotion touches a **workspace member** is NOT boundary-isolated the way the ownership
   table assumes. `🖍️draw` (`🔄️fsm`), `🔱️trinity` (`🔨️modules/🔌️jack/{🐚️shell,🧠️lsp}`) and `🔋️energy`
   (`🔨️modules/⚡️simulation`) all have their own crates — **these three must run alone**, never concurrently with
   other plugin agents, and the root `Cargo.toml` `members` edit is coordinator-owned.
3. Concurrent agents seeing this error must report `blocked-peer` and wait, not "fix" the workspace — it is
   transient and self-resolving.

This is exactly the class of hazard the hot-file ownership table exists for; the table just did not model
*workspace membership* as a shared resource. It does now.

### ▶️ W4 in flight (10 concurrent, disjoint trees)

`🌿️vcs` · `📋️forms` · `🖍️draw` · `🗒️note` · `🪵️sourcing` · `🕸️dag` · `➗️mathematical` · `✒️writer` · `💡️reasoning` · `🎞️animate`

**Deliberately NOT dispatched concurrently** (workspace-member hazard above — each must run alone):
`🖍️draw` was already in flight when the hazard was found, so it stays; `🔱️trinity` and `🔋️energy` are held back
until the field is clear.

Remaining after this batch: playbook, imperative, layout, raster, flow, process, fem, gis, lowpoly, remodel,
puzzle, block, architect, space, demonstrator, norm, shooting, cad, procedural, trinity, energy — plus stdio (D6).

### ▶️ stdio unblock dispatched (D6 critical path)

The `FULL-STDIO-…` peer's last commit is `d9542d156a` (2026-08-18 12:22) and the stdio tree is **clean** — so the
268 test-target errors are no longer anyone's in-flight work; they are simply **unowned breakage nobody is fixing**.
Waiting for them to clear was therefore the wrong call, and that is why the earlier "gate" framing was too passive.

Dispatched a dedicated agent with a **narrowly scoped** mission: make the stdio *test target* compile and its suite
run green. Explicitly **not** the declaration-tree cutover — no `subset()`/`standard()`/`artifact()` roots, no
`📇️registry` deletion, no `declare_artifact`. That stays W2, and it stays atomic (D6).

Rationale for the split: the cutover is unverifiable while the test target does not build, so repairing the test
target is a strict prerequisite and a much smaller, safer change. The error profile supports that — `E0422`/`E0425`/
`E0433`/`E0599` dominate, i.e. test code calling moved/renamed APIs, not deep logic breakage.

The agent is instructed to fix in descending order of error count and produce a real descending curve, and that
**silently deleting a test to make the suite green is forbidden** — any deletion needs a `## deletedTests`
justification.
