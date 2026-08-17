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

## Final state of this session

Framework mechanism is **built and green**; no plugin has been cut over yet (by design — the cutover is atomic
per plugin, and stdio's is blocked on D6).

| crate | result |
|---|---|
| `semio-framework-os-kernel --lib` | **996 / 996 pass** |
| `semio-framework --lib` | **148 / 148 pass** |
| `semio-framework-plugin --lib` | **230 run / 226 pass / 4 fail** — the 4 are the pre-existing baseline, unchanged; +5 net new tests, all passing |
| `semio-framework-plugin --target wasm32-wasip2` | clean |
| `semio-s-plugin-stdio` lib / wasm | 0 errors |
| repo-lib `🧪️index.test.ts` | 20 fail = baseline (2 taxonomy failures verified pre-existing) |
| new policies | 0 blocking breaches added; 2825 report-mode breaches = the measured migration backlog |

### The next agent should start here
1. **W1-D** — WIT + plugin host `IoRouter` + TS kernel mirror (disjoint from everything else; unblocks runtime io
   across the wasm boundary).
2. **Re-cut** `🔧️patches/w1b-discovery-io-native-codec-vocabulary.txt` against the corrected native-codec shape
   (`📓️design.md` §1 CORRECTION) before any subset moves codec files.
3. **W2** — stdio, all 36 artifacts in ONE cutover, deleting `📇️registry` (D6). Not per-artifact; the registry map
   makes per-artifact impossible without a forbidden dual-registration layer.
4. Re-run `carrier_native_is_raw` once the `26/08/16/FULL-STDIO-…` peer lands their 192-file stdio test-surface
   rewrite; until then the carrier law is proven by inspection only.
