# 💻️ Wave E1: `semio-framework-os-kernel` decoupling (2026-10-13)

Scope: E1 of `📓️2026-10-13-scalable-build-architecture.md` §10, using the family method of
`📓️2026-10-13-stdio-semio-split-plan.md` (decouple inside the crate first, then cut).

- **Step 1 was done:** decoupling inside the crate.
- **Step 2 was not done:** the crate cut. Its prerequisites are not met (§8). A precise cut plan is in §8.

Hard rules kept throughout:

- No git-modifying command and no worktree. No ticket operation and no sub-agent.
- Every cargo call ran in the foreground through `🚦️cargo-slot.sh`, with private `CARGO_TARGET_DIR` and `CARGO_BUILD_BUILD_DIR` under `⚡️cache/e1/os-kernel`. That directory is deleted at the end.
- `🗑️generated` was not touched.

---

## 1. Outcome

| | Before | After |
|---|---|---|
| Kernel modules that can be cut away from the Store with no seam | none: one cycle through 14 facets | **11 base crates, 22.6 k lines, 0 seams** (§4) |
| Store root file `🏪️store/🦀️.rs` | 31,901 lines, one module | 13,104 lines, plus 19 new module files grouped by target crate |
| Store layer seams to cut before the Store can be split | not measurable: regions inside one file | **9 node pairs, 19 item references**, each classified with its fix (§5) |
| Target crate graph | the plan's axis, unverified | 20 crates checked against the exact reference graph (`e1-os-kernel/🎯️target-crates.json`) |
| os-kernel `--lib` tests, one process per test | 1465/1465 | **1465/1465** |

The plan's E1 axis (kernel core / canonical edit / history / retirement / laws) does not match the code (§6). The measured axis has two parts:

- **Horizontal base layers** (spr, pack, vcs, dsl, directory, …). They are already one-directional.
- **A vertical stack inside the Store engine:** base → codec → io → engine → history, preparation, composition → sync.

---

## 2. Method (exact, compiler-derived)

All inputs live in `e1-os-kernel/`.

1. **`🔬️redump.sh` + `🔬️hir-owner-graph.py`.**
   - The dump runs `cargo rustc -p semio-framework-os-kernel --lib --profile check --features sync,ureq -- -Zunpretty=hir-tree`, which is about 3.8 GB.
   - The parser builds an owner-level graph: every HIR owner (item, impl or assoc item) and every resolved path inside it (`res: Def(..)`, `SelfTyAlias`).
   - Module and extern-crate path segments are skipped.
   - Each impl owner records its self type and trait.
   - This is exact name resolution, not text matching. A first text-based attempt (`🕸️kernel-module-graph.py`) is kept only as the module-tree walker, because name collisions made its edges unreliable.
2. **`🧭️facet-map.py`.** Puts owners into nodes: their module, or the `//#region` of the store root they start in.
   - Inherent impls and foreign-trait impls follow their self type, because of the orphan rule.
   - Impls of local traits, and `From`/`PartialEq`-style impls, stay where they are.
   - Commands: `summary` (facet edges and SCCs), `layers K` (Tarjan condensation plus an Eades–Lin–Smyth greedy feedback-arc-set order, sliced by lines), `edge`, `nodes`.
3. **`🪜️layer-closure.py`.** Item-level closures. It answers "what else must move with these items?" and "why does X pull Y?".
4. **`🧮️crate-assignment.py` + `🎯️target-crates.json`.** The target crate graph: crates bottom-up, allowed dependencies, and node rules. It reports per-crate lines and every reference that crosses into a crate it may not depend on (a seam).
   - `🎯️derive-store-targets.py` regenerates the Store section of the target file from a layering.
5. **Refactoring tools.**
   - `🚚️move-items.py`: moves named items, items selected by region, or items from a child file into a new module. It carries docs, attributes, cfg-gated re-exports and the import preamble.
   - `🔁️fix-loop.py`: applies only what rustc names: widening to `pub(crate)`, private fields, tuple fields, and suggested imports.
   - `🧹️prune-imports.py`: `--force-warn unused-imports` on the kernel unit only.
   - `🧽️tidy-imports.py`.
   - `🏃️apply-move.sh`: move, then fixes in five configurations, then prune, then tidy, then fixes again.
   - Move specs: `🔀️moves/*.json` and `a1`–`a3` `.py`.
   - `🧱️rust_items.py` is a copy of the E2 splitter. Braces inside generic arguments are now skipped.
6. **Verification helpers.**
   - `🧪️each-test.sh`: one process per test. Additions: sharding, and `SEMIO_TEST_ARTIFACT_DIR` as a private directory.
   - `🌎️hub-check.sh`: the hub check in bounded, resumable slices.

The native analysis does not see browser-only `cfg(all(wasm32, not(p2)))` modules (`store::worker`, the browser directory bridge). Every move was still compile-checked on `wasm32-unknown-unknown`.

---

## 3. Dependency map before

This is the exact graph of the tree at session start, plus A1 and A2.

### 3.1 Facet graph (production references)

```
store              -> vcs:950 spr:110 retirement:62 store-composition:44 io:40 pack:26 semio:24 canonical-edit:19 history:17 dsl:2
store-composition  -> store:359 semio:45 vcs:33 spr:20 history:8 canonical-edit:4 pack:3
history            -> store:135 store-composition:42 vcs:17 io:15 spr:7
retirement         -> store:116 vcs:7
canonical-edit     -> store:36
io (os)            -> store:78 directory:16 spr:8
spr                -> pack:38 vcs:7 io:6 store:2        <- back-edge
dsl                -> semio:6 pack:3 store:2            <- back-edge
directory          -> spr:2 identity:1
inference          -> spr:5 io:3
extension          -> semio:9
vcs, pack, semio, identity, list-delta -> (nothing in-crate)
```

- **Facet SCC:** canonical-edit, directory, dsl, history, inference, io, pack, retirement, root, spr, store, store-composition and vcs form one cycle.
- **Module-level SCC:** 75 nodes, 36.7 k lines.

### 3.2 Back-edges from the base layers into the Store (the whole cycle)

| From | To | Symbols |
|---|---|---|
| `os_spr::command` | store `📬️OneItemPublication` | `ARTIFACT_STORE_ONE_ITEM_MAXIMUM_WORK_ITEMS`, `ArtifactStoreOneItemFootprint` (in the declared-rows law) |
| `os_dsl::tagged_value_binary` | store `pack_rt` | `encode_wire_value`, `decode_wire_value` |
| `os_vcs` types | store `♻️retirement` | `RetireOwned` for `HistoryPageStack`, `ArtifactHistoryKey`, `ArtifactGroupVisibility`, `Author` |

`spr` → `io::control` and `inference` → `io::text` are not back-edges. Those two modules are framework modules (`🧰️framework/🔨️modules/🚪️io/{⏱️control,📝️text}`) with no in-crate dependencies.

### 3.3 Store internals

The 31.9 k-line root mixed the vocabulary (snapshot-read owners, one-item publication, schemas, command format, replay, ledger, codec seam) with the `ArtifactStore` engine in one module.

- **Item closures (`🪜️layer-closure.py`).** What canonical edit needs from "store" pulls in `ArtifactStore`. The chain: `ArtifactStoreOneItemLiveAuthority → ArtifactStoreOneItemPrepared → ArtifactPack → ArtifactCodec → ArtifactStore`.
- **Remaining pulls.** The cursor revision accumulator and the edit-message ledger are referenced from every lower region.

---

## 4. What changed

### 4.1 Base layer, made cut-ready

**A1: one-item admission vocabulary.**

- The ceilings, `ArtifactStoreOneItemFootprint`, `…Grant`, `…Checkpoint`, `…PreparationStep` and `validate_original_preparation_step` left the store root for `🏪️store/🎮️one-item/🦀️.rs`.
- The declared-rows law `mutation_inverse_rows_declaration_failures` moved from `📡️spr/🎮️command/🦀️.rs` to the same file. It now sits beside the footprint it proves.
- The derive keeps the root path `::semio_framework_os_kernel::mutation_inverse_rows_declaration_failures`, so no consumer changed.

**A2: pack runtime bridge.**

- `pack_rt` moved from the store root to `🎒️pack/🌉️bridge/🦀️.rs`, mounted as `os_pack::pack_rt`. It forwards only to os pack and the replication byte sink (`protocol::io::binary::operation_bytes`).
- All 44 repository files that spelled `os_store::pack_rt` now say `os_pack::pack_rt`. These include the `dsl_derive` expansion, hub tests and about 25 plugin `🔺️diff` files. No re-export was left behind.
- The `store::pack_rt` / `semio_framework_os_kernel::pack_rt` paths still resolve through the crate root.

**A3: version-graph retirement.**

- The four `RetireOwned` impls and their cursors moved to `🌿️vcs/♻️retirement/🦀️.rs`.

**Result:** zero seams for the base crates (`🧮️crate-assignment.py`, §5.1).

### 4.2 Store root decomposed (all inside one crate, behaviour unchanged)

| Move | New file (under `🏪️store/`) | Lines | Target crate |
|---|---|---:|---|
| B1 ledger | `📒️ledger/🦀️.rs` | 1,150 | store-base |
| B12 edit retirement | `📒️ledger/♻️retirement/🦀️.rs` | 235 | store-base |
| B2 snapshot reads and owner catalogs (engine-held close view and disposers kept back) | `🔗️read/🦀️.rs` | 2,527 | store-base |
| A1 one-item vocabulary | `🎮️one-item/🦀️.rs` | 204 | store-base |
| B3 one-item publication protocol, batch source, member wire | `🎮️one-item/📬️publication/🦀️.rs` | 1,117 | store-base |
| B4 schemas, history lane, stamps, events | `🧬️schema/🦀️.rs` | 1,114 | store-base |
| B5 command format and `.ops` header grammar | `📬️command/🦀️.rs` | 1,053 | store-base |
| B6 effective forwards, edit replay, pair merge | `🔁️replay/🦀️.rs` | 1,211 | store-base |
| B7 `ArtifactChild` vocabulary and child/owner/link DSL records | `🪆️child/🦀️.rs` | 776 | store-base |
| B8 backbones | `🔗️backbone/🦀️.rs` | 583 | store-base |
| B9 space history document | `🧬️schema/📜️space-history/🦀️.rs` | 254 | store-base |
| B10 serialization seam (`ArtifactDsl`, `ArtifactPack`, SQLite and native snapshot capability, the erased `ArtifactCodec` table) | `🚪️io/🦀️.rs` | 278 | store-base |
| B11 document materialization | `🧾️document/🦀️.rs` | 203 | store-base / codec |
| B13 presence peers and lane receipts | `👥️presence/🦀️.rs` | 312 | store-base |
| C1 owned schema decoding | `📦️codec/🧬️owned-schema/🦀️.rs` | 5,278 | store-codec |
| C2 document text format | `🚪️io/📝️text/🦀️.rs` | 1,437 | store-codec |
| C3 codec registry, bounded owners, dialect migration | `📦️codec/🦀️.rs` | 684 | store-codec |
| D1 composition coordinator | `🧩️composition/🦀️.rs` | 878 | store-composition |

Mechanics:

- **Re-exports in the root.** The store root re-exports each moved `pub` item with `pub use`. Each moved crate-private item is re-exported at `pub(crate)`, carrying the item's `cfg` gate. Every existing path inside and outside the crate therefore still resolves; no consumer changed.
- **Engine-only impls stay in the root.** `ArtifactCodec`'s generic constructors (`of`, `bare`, `assemble`) remain an inherent impl in the store root, with the engine. See seam S1.

Visibility debt:

| Count | Value |
|---|---:|
| `pub(crate)` in the Store files before (root at HEAD) | 15 |
| `pub(crate)` in the Store files after (root plus the new files) | 830 |

- Every widening was named by rustc.
- At the cut these widenings become crate-public API. §8 says which of them must instead become constructors or accessors.

### 4.3 Source-scan laws moved to the owning files (behaviour-preserving)

Some tests and policies read the store root's text. Every one that my moves broke was re-pointed at the file that now owns the fact.

- **TypeScript tests:**
  - replication `📨️messages/♻️retire`
  - plugin `♻️lifecycle/🏪️stores`
  - `🪟️mounted-owned-child`
  - plugin `🌱️genesis`
  - `⏪️time-travel/♻️retirement`
  - store `🔬️canonical-error-progress`
  - store `📦️owned` emission
- **Rust:** the store unit law `one_item_publication_source_denies_generic_or_unbounded_shortcuts` now reads the store root plus `🎮️one-item/📬️publication`.
- **Scripts:**
  - hub `document-check-in` oracle: `replay_envelopes_onto_pair` is now in `📦️codec`, so the oracle has 6 boundaries.
  - os `script.ts` batch publication handoff.
- **Root `📜️script.ts`.** Its tool-job policies scan 109 Store literals across 16 functions. They now read `policyReadOsStoreSource` / `policyReadOsStoreEvidence`, the root plus the decomposed files in `OS_STORE_ROOT_SOURCES`.
  - That list is exactly the old root text, re-split. Verified: 0 literals missing and 0 newly present against the pre-move root.

Baselines for the scan tests were taken by swapping the pre-move store root in for about 1 minute. The file was guarded by mtime and restored with `cmp`. Each affected test file is now back at its baseline:

| Test | pass / fail |
|---|---|
| canonical | 13/2 |
| owned | 19/6 |
| lifecycle | 7/1 |
| time-travel | 8/13 |
| flow | 1/1 |
| trinity | 42/16 |

The remaining failures all predate this work.

---

## 5. Dependency map after

### 5.1 Target crate graph (`🎯️target-crates.json`, checked on the final HIR)

| Crate (bottom-up) | Non-blank prod lines | Depends on |
|---|---:|---|
| `semio-framework-io-control` (`io::control`, `io::text`) | 275 | – |
| `semio-framework-os-vcs` | 1,903 | – |
| `semio-framework-os-pack` (incl. `pack_rt`) | 4,215 | – |
| `semio-framework-os-list-delta` | 691 | – |
| `semio-framework-os-identity` | 138 | – |
| `semio-framework-os-semio` | 389 | – |
| `semio-framework-os-spr` | 7,523 | pack, vcs, io-control |
| `semio-framework-os-dsl` | 1,024 | pack, semio |
| `semio-framework-os-directory` (+ plugin module schema) | 5,661 | spr, identity |
| `semio-framework-os-inference` | 639 | spr, io-control |
| `semio-framework-os-extension` | 212 | semio |
| `semio-framework-os-store-base` | 12,688 | spr, dsl, directory, inference, extension, list-delta |
| `semio-framework-os-store-codec` (+ `io::bindings`) | 7,466 | store-base |
| `semio-framework-os-io` | 2,174 | store-codec |
| `semio-framework-os-store` (engine) | 17,646 | io |
| `semio-framework-os-store-history` (space history SQLite) | 985 | store |
| `semio-framework-os-store-preparation` | 7,533 | store |
| `semio-framework-os-store-composition` | 1,382 | history, preparation |
| `semio-framework-os-store-sync` | 4,816 | composition |
| `semio-framework-os-kernel` (facade, then deleted at the cut) | 110 | – |

- Total is 77.5 k non-blank production lines (native, `sync,ureq`).
- Every crate is within the L3 15 k target, **except the engine** at 17.6 k. That is under the 25 k fail line.
- All names are 48 characters or fewer (L4).

### 5.2 Remaining seams (the work list before the Store crates can be cut)

| # | From → to | Items | Fix |
|---|---|---|---|
| S1 | store-base `serialization` → codec `text_format` / `codec_registry`, engine `ArtifactStore`; codec `codec_registry` → engine | `ArtifactCodec::{of,bare,assemble}` instantiate parse, print and replay; `close_codec_reduction_store`, `funded_bounded_artifact_store_owners`, `replay_envelopes_onto_pair`, `ArtifactStoreCursorDisposer` | **L8 seam.** The erased `ArtifactCodec` table stays in base. The generic constructors become engine-side free functions (`artifact_codec::<P, M>(schema)` / `bare_artifact_codec`), together with the three helpers. This changes the consumer API: 292 call sites in 195 files plus the TS code generators in root and stdio `📜️script.ts`. It fits the cut's consumer rewrite (§8), not before. |
| S2 | base `one_item_publication` → engine `durable_group` | `DurableOwnedGroupDecisionError`, `DurableStorePreparedOutcomeV1`, `DurableStorePreparedOwnerV1` (closure pulls `ArtifactStore`, 9 k lines) | **Inversion.** The publication carries its durable outcome through a base trait or associated type. The engine's durable group implements it. |
| S3 | base `snapshot_read` → engine | `ArtifactStoreOwnedDisposer` (a trait naming `ArtifactStore`) | Split the trait: a base disposal contract without the engine type, plus the engine disposer. |
| S4 | base `snapshot_read` (`MemberStoreOwner`) → engine `member_open::operation` | `MemberSnapshotOpenOperation` (closure pulls member open plus owned schema decode) | **Inversion.** `MemberStoreOwner`'s open hook returns an erased `Box<dyn …>` operation declared in base. Member open implements it. |
| S5 | base `serialization` (`ArtifactPack::publish_native_snapshot`) → io | `ArtifactAssemblyRegistryError` (closure: the 16-type registry error family, 114 lines) | Move the registry error family to store-base. Or let `publish_native_snapshot` return a base error that io converts. |

S1 is the only seam that touches consumers. S2 to S5 are local to the crate.

### 5.3 Facet view

The old directory-based facets still show one SCC: canonical-edit, history, io, retirement, store and store-composition. The reason is that the facet rules key on directories: for example `🔗️read/` counts as "retirement" and `🔁️replay/` as "history".

The authoritative view is §5.1. The base layers no longer take part in any cycle.

---

## 6. Correction to the plan's E1 axis

The plan proposed splitting on: kernel core (store, ledger) / canonical edit / history and time travel / retirement and lease / laws (test-only).

Measured against that:

- **Canonical edit, history and retirement are not leaves above a core.**
  - The engine calls them: store → canonical-edit 19, history 17, retirement 62 references.
  - They depend on vocabulary that used to sit in the same root file as the engine.
  - The workable order is vertical:
    1. vocabulary (one-item, ledger, snapshot reads, schemas, command format, replay, canonical edit),
    2. codec,
    3. io,
    4. engine,
    5. leaves above the engine (preparation, history SQLite, composition coordinator),
    6. sync.
- **The ledger is vocabulary, not engine.** It (66 items, about 1.2 k lines) has no path to `ArtifactStore`.
- **"History and time travel" is mostly inherent `ArtifactStore` methods.** The regions `EventSourcedHistory`, `TimeTravelReads`, `DeferredReprojection`, `Supersede` and `ToolTransactions` live inside the engine's impl blocks. They can leave the engine crate only as extension traits, which is a consumer-API change. That is the remaining lever for the engine's 17.6 k lines.
- **The biggest immediate win is horizontal, not in the Store.** 22.6 k lines of base layers have no seams today, and they cut first.
- **"Laws (test-only)" is not a crate.** Laws stay with each crate's tests.

---

## 7. Verification (all run after the last source edit unless stated)

**Kernel checks:**

- `cargo check -p semio-framework-os-kernel --lib` passed natively, on `wasm32-wasip2`, on `wasm32-unknown-unknown`, with `--features sync,ureq`, and with `--profile test`. All are green; the only warnings are the same 3 pre-existing ones (`store` unreachable and offset, `dsl` type-alias bounds).

**Kernel tests:**

- `--lib` tests, one process per test (6 in parallel, 150 s alarm, `RUST_MIN_STACK` 64 MiB, private `SEMIO_TEST_ARTIFACT_DIR`): **1465/1465 ok**.
  - The first sweep showed one test failing only because `SEMIO_TEST_ARTIFACT_DIR` was unset. The runner now sets it.
- Integration tests:
  - `artifact_command_retirement`: 2/2.
  - `sqlite_snapshot_native_admission`: 66 pass, 4 fail ("no available explicit retirement slot"). These 4 fail identically on a HEAD snapshot (`git archive HEAD` into the scratchpad, private dirs), so they are not caused by this work.

**Hub:**

- The hub wasm workspace check (`cargo check --manifest-path 🌎️hub/Cargo.toml --workspace --exclude semio-hub --exclude semio-hub-auth-client --target wasm32-wasip2`) is **green**: 349 units in 7 min 51 s, private dirs.

**Consumers whose files A2 rewrote:**

| Result | Crates |
|---|---|
| green | `semio-framework-plugin`, artifact `flow`, `dag`, workflow `run`, space `space` and `collection` |
| red | `os-mcp` (lib) and `os-renderer-wgpu` (lib test) |

The two red crates fail only on unrelated drift that predates this work: argument counts, `Budget.retained`, `TurnFault::ActorCall`, and unexported `os_directory` items. A2 also listed this drift as inherited breakage. Neither touches any moved item.

**TypeScript:**

- The 7 affected TS scan tests are at baseline (§4.3).
- `tsc` reports no new errors in the edited root, hub and os scripts.
- Plugin `test-retained-raw-source` is 17/20 (E2 reported 16/20). Its 3 failures name plugin source (`ToolCancellationLeaseRetirement`, `store::LaneItemReceipt` in the VCS app) or are baseline failures. None scans the Store.

**Not measured:** unit timing and RSS before and after, because there was no cut.

---

## 8. Step 2 (the crate cut): why it waits, and the plan

### 8.1 Why it waits

- **Consumer rewrite.** 1,489 consumer files with 4,003 `semio_framework_os_kernel::` paths must name the owning crate. There are no re-export shims (split-plan rule).
- **Lockfiles and workspaces.** 378 manifests and 176 lockfiles name the kernel. Without WS1, every new crate also enters about 270 nested workspaces and lockfiles. WS1 has not landed: the root workspace has no `feature-unification` setting, and the 273 nested workspaces are still there.
- **Seams.** The Store's seams S1 to S5 must be cut first.
- **The base cut** (§8.2, step 1) is clean on its own. It still needs the same consumer rewrite and lockfile churn, so it belongs to the WS1 wave.

### 8.2 Cut script (in order, compile-atomic)

1. **Base crates** (seam-free now). Create one `📦️packages/🦀️rust/Cargo.toml` per crate in §5.1 at its semantic node. Each lib root `🦀️.rs` mounts the existing files by `#[path]`, as the kernel root does today. No file moves.

   | Crate | Package directory | Mounts |
   |---|---|---|
   | `semio-framework-os-spr` | `🔨️modules/📡️spr/📦️packages/🦀️rust` | `📡️spr` |
   | `semio-framework-os-pack` | `🔨️modules/🎒️pack/📦️packages/🦀️rust` | os pack facade and `🌉️bridge` |
   | `semio-framework-os-vcs` | `🔨️modules/🌿️vcs/…` | |
   | `semio-framework-os-dsl` | `🔨️modules/🗣️dsl/…` | |
   | `semio-framework-os-directory` | `🔨️modules/📇️directory/…` | plus `🔌️plugin/📇️registry/📦️deployment/🧬️schema` |
   | `semio-framework-os-inference` | | |
   | `semio-framework-os-extension` | | |
   | `semio-framework-os-identity` | | |
   | `semio-framework-os-list-delta` | | |
   | `semio-framework-os-semio` | | |
   | `semio-framework-io-control` | `🧰️framework/🔨️modules/🚪️io/⏱️control/📦️packages/🦀️rust` | `⏱️control` and `📝️text` |

   - Each crate replicates the `extern crate self as …` aliases its code uses (`spr`, `dsl`, `vcs`, …). The derives' emitted paths become owner-crate paths, for example `::semio_framework_os_spr::Mutation`. `dsl_derive` emits about 30 such paths.
2. **Cut S2 to S5 inside the kernel** (local, no consumers).
3. **Store crates** per §5.1: base, then codec, io, engine, history, preparation, composition, sync.
   - S1 converts `ArtifactCodec::{of,bare}` to engine free functions in the same wave as the consumer rewrite.
   - Privacy: these `pub(crate)` items must not become `pub`, because the authority laws (for example `one_item_publication_source_denies_generic_or_unbounded_shortcuts`) forbid raw fabrication fields. They get engine-minted constructors and accessors instead, or the type moves to the engine crate:
     - `ArtifactStoreOneItemLiveAuthority` fields (minted by the engine);
     - `ArtifactStoreOneItemPrepared` seal fields.
4. **Consumer rewrite, generated from the HIR map.**
   - For every consumer path `semio_framework_os_kernel::X` / `store::X`, the owners JSON gives X's defining module, and `🎯️target-crates.json` gives its crate.
   - The rewriter emits `semio_framework_<crate>::X` and the manifest dependencies.
   - The `use super::…` preambles of the decomposed Store files become explicit imports from their crates, from the same map.
5. **Delete the `semio-framework-os-kernel` facade.** Regenerate the lockfiles once, under WS1.
6. **Gates:**
   - G1/G2: 1 unit per crate, no non-component cdylib. Drop `crate-type = ["rlib","cdylib"]` from today's kernel manifest (L5).
   - G3: the line budget.
   - os `--lib` sweeps at 1465.
   - The hub wasm check.
   - Measure the M0 unit timing and RSS here.

---

## 9. Follow-ups

- **Engine size (17.6 k).** Turn the engine-method facets (time travel reads, deferred reprojection, supersede, tool transactions, event-sourced history) into extension traits. This is a consumer-API change, so do it in the cut wave.
- **Narrow the `pub(crate)` debt (830)** while cutting S2 to S5, in the same spirit as E2's C4.
- **Pre-existing failures:**
  - the 4 `sqlite_snapshot_native_admission` failures (nested native-codec slot);
  - the `os-mcp` / `os-renderer-wgpu` drift;
  - the TS scan-test failures listed in §4.3.
- **`🔣️codec/🧵️send/🧫️fixtures`** (plugin) still lists the store root as a scan source, but nothing consumes it.

## 10. Files

**New:**

- `🧰️framework/🛍️products/💻️os/🔨️modules/`:
  - `🎒️pack/🌉️bridge/🦀️.rs`
  - `🌿️vcs/♻️retirement/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/`:
  - `🎮️one-item/🦀️.rs`, `🎮️one-item/📬️publication/🦀️.rs`
  - `📒️ledger/🦀️.rs`, `📒️ledger/♻️retirement/🦀️.rs`
  - `🔗️read/🦀️.rs`, `🔗️backbone/🦀️.rs`
  - `🧬️schema/🦀️.rs`, `🧬️schema/📜️space-history/🦀️.rs`
  - `📬️command/🦀️.rs`, `🔁️replay/🦀️.rs`, `🪆️child/🦀️.rs`
  - `🚪️io/🦀️.rs`, `🚪️io/📝️text/🦀️.rs`
  - `🧾️document/🦀️.rs`
  - `📦️codec/🦀️.rs`, `📦️codec/🧬️owned-schema/🦀️.rs`
  - `🧩️composition/🦀️.rs`
  - `👥️presence/🦀️.rs`

**Changed in the kernel:**

- `🏪️store/🦀️.rs`, `🏪️store/♻️retirement/🦀️.rs`, `🏪️store/🧪️tests/🔬️unit/🦀️.rs`
- `📡️spr/🦀️.rs`, `📡️spr/🎮️command/🦀️.rs`
- `🎒️pack/🦀️.rs`, `🌿️vcs/🦀️.rs`
- `🗣️dsl/🦀️.rs`, `🗣️dsl/✨️derive/🦀️.rs`
- The A2 path rewrite (`os_store::pack_rt` → `os_pack::pack_rt`) in 44 files:
  - kernel: `🏪️store/🔄️sync`, `🧩️composition/🗄️durable-group`, `📡️spr/🧵️channel`, `🗣️dsl` tests;
  - outside the kernel: `🌉️mcp`, the wgpu renderer elements, the space, flow, dag and workflow artifacts, the plugin test fixtures, the hub trusted-catalog and its tests, and about 25 `✏️s` plugin `🔺️diff` files.

**Changed scripts and tests:**

- `📜️script.ts` (root)
- `🌎️hub/📦️packages/🦀️rust/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📜️script.ts`
- The 7 TypeScript tests in §4.3.

**Inputs (kept):** `e1-os-kernel/`, holding the tools of §2, `🔀️moves/`, `🎯️target-crates.json`, `🧪️each-test.sh` and `🌎️hub-check.sh`.
