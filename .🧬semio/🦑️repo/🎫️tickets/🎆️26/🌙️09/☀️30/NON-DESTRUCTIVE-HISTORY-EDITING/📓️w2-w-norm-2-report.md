# 📓️ W2-W-norm-2 — wire witnesses for EN 1996, DIN EN 16798, DIN V 18599 and DIN 4108

Ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`, executor W2-W-norm-2, 2026-09-30. Contract: `📋️design.md` §6 and §11
(incl. negative witnesses), `🧭️plan.md` "W2-W brief", `📓️w2-s-report.md` F10/F11/F16/F17, `📓️w1-d-report.md` §8.
Scope: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/{🪨️en1996, 🌬️din16798, ⚡️din18599, 🧱️din4108}` and the shared norm Python engine.

Verdict: DONE for the brief. Every leaf is witnessed by a committed vector that Rust produced and the independent Python
engine reached on its own. Both lints are 0 in scope, the crate tests and payload laws are green, and all four
feature cases pass their oracle, subject and parity phases.

## 1. Outcome

**Lints** (`🧪️test`: `bun ./📜️script.ts schema mutation-payloads|mutation-inputs --under ✏️s/🔌️plugins/📕️norm/🗿️artifacts/<a>`)

| Artifact | payloads before | payloads after | inputs after |
|---|---|---|---|
| EN 1996 | 59 `unwitnessed` (0/58 leaves) | **0**; 58/58 fixtures clean; 58/58 leaves witnessed | **0**; 135/135 inputs |
| DIN EN 16798 | 42 `unwitnessed` (0/41) | **0**; 41/41 clean; 41/41 witnessed | **0**; 72/72 |
| DIN V 18599 | 17 `unwitnessed` (2/19) | **0**; 20/20 clean; 19/19 witnessed; 1 negative witness rejected | **0**; 20/20 |
| DIN 4108 | 0; 71 fixtures, 28 of them on a stale model | **0**; 43/43 clean; 43/43 witnessed | **0**; 90/90 |

**Crate tests** (`cargo test --lib`, private `target-nde-w2w-norm-2`, `CARGO_INCREMENTAL=0`, gated, foreground; final run
`🗑️generated/w2w-norm-2/test-lib-final.txt`)

| Crate | Result |
|---|---|
| din16798 | 86/86 |
| din18599 | 115/115 |
| din4108 | 142/142, incl. 43 new canonical vector tests |
| en1996 | 143/143 |

- Every crate passes `semio_payload_law_<aggregate>` and the new `committed_vectors_are_this_implementations_answer`.
- The build emitted 300+ warnings, none in a file this WP wrote. That proves the code was type-checked.

**Feature cases** (`bun ./📜️script.ts oracle|subject|parity exhaustive --case <case>`)

| Case | oracle (Python) | subject (Rust) | parity |
|---|---|---|---|
| `🪨️mutate-en1996-1` | 117/117 | 117/117 | **117/117** |
| `🌬️mutate-din16798-1` | 83/83 | 83/83 | **83/83** |
| `⚡️mutate-din18599-1` | 39/39 (before: 3 passed, 24 errored) | 39/39 | **39/39** |
| `🧱️mutate-din4108-1` | 87/87 | 87/87 | **87/87** |

**Other checks**

- **Norm document contract** (`testNormDocumentContractOracle`): **OK**. It matched 38 native snapshots and 18 committed diffs, parsed by the TS twins and validated with Ajv.
- **Oracle-source ownership:** each of my four adapters meets every per-adapter expectation: one engine import, `definitions == ["adapter"]`, and handlers equal to the feature's scenario ids. The suite as a whole still fails 3/6, for reasons outside my scope (§5).

## 2. What was done

### 2.1 Committed vectors

Every kind has one closed bundle: `🧫️fixtures/🧬️mutations/<leaf>/<scenario>/{🦠️mutation, 📸️snapshot/⬅️before, 📸️snapshot/➡️after, 🎯️outcome, 🔺️diff}`.

**Pipeline** (`🧪️w2w-norm-2-vectors.py`):

1. `stage` writes the mutation, and a base where the kind needs one, from a per-kind spec.
2. A temporary Rust producer (`🗑️generated/w2w-norm-2/gen/*.rs`) writes the before, after, diff and outcome. It was mounted into each crate only while it ran, and every mount has been removed since.
3. `check` runs the independent Python engine against every bundle. It must reach the same after-snapshot, move the document, and restore the before-snapshot through its own inverse.
4. `install` pretty-prints the bundles. It keeps the Rust number lexemes and refuses to install on any disagreement.

**Counts per artifact:**

| Artifact | Bundles | Notes |
|---|---|---|
| EN 1996 | 58 | |
| DIN EN 16798 | 41 | |
| DIN V 18599 | 16 new, 2 regenerated | `🏢️reclassifies-the-building-as-an-office` and `📏️extends-net-floor-area-to-160-m2` were regenerated. The invariant refusal `🌧️refuses-a-negative-january-irradiance` is kept; it is a clean negative witness. |
| DIN 4108 | 43 | The 28 stale Python-oracle vectors were refreshed on the current snapshot model (`inclinationDeg`, `deltaUG/UF/UR`, two zones), keeping their descriptive names. `🏷️retags-as-wall` was a no-op on the current model, so it became `🏷️retags-as-opaque-frame`. 15 kinds were added. |

**DIN 4108 `🎫️fixtures` removed.** This was the obsolete testing category: 215 files, which caused 561 contract breaches. Its Rust suite was never mounted, so it was dead code. It is merged into `🧫️fixtures` and deleted.

**`update-climate` (DIN V 18599).** Applying it mints a content-addressed child handle that no document specifies, and the Python engine rightly refuses to guess it. Its applied form is therefore a payload-only witness, `🌦️update-climate/🧾️wire-witness/🦠️mutation/🔣️.json`, held by the payload law and the crate suite. Its feature row is the committed refusal, which both sides reject bit-identically.

### 2.2 The disagreeing side was fixed

**Rust: DIN 4108 lost undo.** 36 of 43 leaf inverses were `Vec::new()` stubs, so undo of every zone, window, element, layer and bridge edit was silently lost. They are rewritten with `🧪️w2w-norm-2-din4108-inverses.py`:

- setters restore the base value;
- insert and remove pair up at their position;
- reorder swaps back;
- a missing target yields no step.

The crate suite now proves that every inverse restores its before-snapshot.

**Python: the shared engine** (`✏️s/🔌️plugins/📕️norm/🔮️oracles/🏃️execution/🐍️.py`) could not read nested addressing. I added these, on top of norm-3's concurrent edits, which I re-read before every edit:

- **`address_steps` / `addressed_record` / `nested_arguments` / `nested_kind`.**
  - `<entity>Index` positions and `<entity>Id` native keys descend in wire order.
  - The collection is found by spelling, or as the one list that holds that id: `bridgeId` finds `thermalBridges`, `ventId` finds `ventSystems`.
  - The kind's noun drops the entities it descended through, so `change-member-label-en {memberId, newValue}` works.
  - Inverse steps keep the addressing, and partner kinds are mapped back to the full vocabulary.
- **Other engine changes:**
  - `specify` is a setter verb.
  - `collection_key` also tries the trailing noun segments.
  - `insert` prefers the record-valued argument.
  - The `remove` inverse falls through to the id-addressed branch when there is no `index`, and always restores the list position.
- **Regression check.** `🗑️generated/w2w-norm-2/engine-regression.py` compares every norm subset's vectors with and without my edits: **0 regressions** in 14 subsets.
  - Gains: en1996 +38, din4108 +24, din16798 +20, din18599 +4, en1995 +6.
  - en1997 could not be loaded (its adapter defines no `adapter()`; that is norm-3's scope).

**Asset: DIN V 18599 `🎬️demo/🗣️.dsl.semio` was not printer-canonical.**

- It had `heat-recovery-eta=0.80`, and `cooling= ` where the printer writes `cooling=`.
- It is replaced by the Rust printer's bytes. Both parse to the same document, proven by comparing the JSON of `decode(asset)` and `decode(reprint)`.
- The Python carrier reading reprints the new bytes exactly.

### 2.3 Tests and cases

- **Crate vector suite** `🧬️schema/🧬️mutations/🧪️tests/🔬️fixture/🦀️.rs`, in all four crates.
  - For each bundle it checks decoding, the after-snapshot, the diff, the outcome (a refusal must be raised with the committed fatal code), the inverse restoring the before-snapshot, and that every `KINDS` entry is covered by an applied vector or a wire witness.
  - EN 1996 had a placeholder there, which is replaced. DIN V 18599's three per-vector tests are kept.
  - It is newly mounted in DIN EN 16798 (the old smoke file was never mounted) and in DIN 4108.
- **DIN 4108 canonical tests.** Each of the 43 vectors has `<leaf>/🧪️tests/<scenario>/🦀️.rs` calling `assert_vector`. This replaces 31 dead, unmounted smoke tests.
- **Rust subject adapters.** All four case `🦀️.rs` files are now repo-test adapters (`pub fn adapter()`). They read exactly the vector URIs their scenario's steps name (`ctx.step_fixture_uris()`), so there is no per-kind table. They also carry the carrier identity round trip.
  - The EN 1996 crate no longer mounts its case file as a cargo test.
- **Python adapters, features and oracle manifests** are rendered by `🧪️w2w-norm-2-cases.py`.
  - The Python adapters carry `KINDS` and `VECTORS`, using the current vocabularies and existing bundles.
  - Each feature has a mutate row and an inverse row for every kind, plus `identity-round-trip`. EN 1996 gains one through its `loadbearing-wall` asset.
  - The oracle manifests are updated:
    - the catalog lists every kind, with vectors whose leaf directory renders the kind;
    - the owner manifest is built from the leaf descriptors (outcomes and variants);
    - coverage counts and the implementation path in the rationale are corrected;
    - DIN 4108's stray top-level `family`/`kinds` are removed.
  - All four catalogs pass `mutationCatalogProblems` with 0 problems.
- **Subsets components** (`🪆️subsets/🔣️.json`). DIN 4108's stale `envelope`/`layers` subsets had no directories and caused 43 `wildcard-subset-owner` breaches; the file now declares a single policy. The other three rationales now cite current kinds.
- **Document contract fixture:** the DIN V 18599 committed counts go from 6 snapshots / 2 diffs to 38 / 18.

## 3. Contract breaches in scope

The census is taken from `.🧬semio/🦑️repo/⚡️cache/breaches/testing.json`: **about 830 before, 263 after**.

| Removed class | Count |
|---|---|
| obsolete `🎫️fixtures` | 561 |
| missing fixtures | 88+ |
| invalid catalogs | 20 |
| `test-only-mutation` | 45 |
| `mutation-without-fixture` | 72 |
| `wildcard-subset-owner` | 43 |
| unregistered vocabularies | 2 |
| unclaimed catalog | 1 |
| missing adapter entry point | 1 |

**What remains (263):**

| Classes | Count | Cause |
|---|---|---|
| `manifest-only`, `runtime-only`, `mutation-outcome-mismatch` | 240 | The runtime-inventory cache dates from 09-25. The manifests are now written from the leaf descriptors, so they match what production dispatch reports. |
| DIN V 18599 `mutation-without-fixture` | 9 | See §5, item 3. |
| `binary-protocol-drift` | 2 | See §5, item 2. |
| DIN 4108 compliance-oracle entries | 2 | Pre-existing; see §5, item 5. |

## 4. Decisions

1. **Full bundles, not payload-only witnesses.** The Python oracles had to point at real vectors, so every kind got a Rust-produced bundle that the Python engine had cross-checked.
   - The only payload-only witness is `update-climate`'s applied form. Its effect depends on a content-addressed child that nothing specifies.
2. **One vector per kind.** Scenario directories reuse the leaf's existing canonical test directory, so the vector registry resolves; the exception is DIN 4108, which has its own new canonical tests.
3. **Engine generalisation is recursion, not per-subset tables.** Nested kinds run the unchanged verb logic inside the addressed record. As a result, norm-3's `index_path` branches in `apply_verb` and `inverse_mutation` no longer fire (they are subsumed). I left them in place because that is their active work; the owner may delete them.

## 5. Open items (outside this WP or needing a decision)

1. **Stale runtime-inventory cache.** Refresh it with `test inventory`. That builds `✏️s/🔌️plugins/📕️norm/🏭️bridge`, a standalone workspace linking all 15 norm crates, which I did not run.
2. **`binary-protocol-drift`** in EN 1996 and DIN V 18599: the binary mutation wire records predate commit 652. This is not wire-witness work.
3. **DIN V 18599 leaf directories.** Nine leaf directories do not render their kind: `🏷️use-class`, `🧮method`, `🧱attachment`, `🏠️building-category`, `📐️net-floor-area-m2`, `📦heated-volume-m3`, `⚖️geg-qp-factor`, `🌉delta-u-wb`, `🎛️automation-class`.
   - Their vectors exist and pass every phase.
   - The catalog cannot register them.
   - The taxonomy flags `mutation-payload-schema-authority-invalid`.
   - Renaming them touches the crate mounts, the leaf descriptors and the hot schema catalog.
4. **Taxonomy (`verify taxonomy report`).**
   - **`Δ` leaf directories.** DIN 4108's `Δchange-element-delta-u*` directories use a non-emoji identity, so their 3 vectors show as `projection-catalog-unrealized`.
   - **Path length is systemic.** Norm's subset prefix is about 150 bytes, so most bundle files exceed the 240-byte path budget: 368 of 806 in scope. The same class already exists for older norm bundles, and every leaf directory is already `directory-kind-unresolved`. This needs a taxonomy decision; scenario names alone cannot fit it.
5. **DIN 4108 `din4108-1-compliance-python`** has an empty `candidatesConsidered` and claims a report format that no manifest owns. Pre-existing.
6. **Oracle-source-ownership suite:** 3/6 failures, all outside my scope:
   - the en1997 adapter defines no `adapter()` (norm-3);
   - there is an extra `vdi3805` `⚖️compliance-vdi3805-1/🐍️.py`;
   - the launch rows lack `test-oracle-source`, which the coordinator owns.
7. **W3-CODES.** The DIN V 18599 refusal pins `mutation.invariant` in its `🎯️outcome` and in the leaf diff. If W3-CODES remaps it, the crate suite and the case catch any drift. I edited no code sites.

## 6. Files

**Ticket inputs** (ticket root):

- `🧪️w2w-norm-2-vectors.py` (specs, stage, check, install)
- `🧪️w2w-norm-2-cases.py` (adapters, features, manifests)
- `🧪️w2w-norm-2-din4108-inverses.py`

**Shared Python engine:** `✏️s/🔌️plugins/📕️norm/🔮️oracles/🏃️execution/🐍️.py`.

**Per artifact** (`<a>/🏅️standards/🔖️1/🪆️subsets/✳️any/…`):

| Artifact | Changes |
|---|---|
| All four | `🧫️fixtures/🧬️mutations/**` (new or refreshed bundles); `🧪️tests/<case>/{🦀️.rs,🐍️.py,🥒️.feature}`; `🔮️oracles/🔣️.json`; `🧬️schema/🧬️mutations/🧪️tests/🔬️fixture/🦀️.rs`; `../🔣️.json` (the subsets component) |
| DIN EN 16798 | `🧬️schema/🧬️mutations/🦀️.rs` (fixture mount) |
| DIN 4108 | `🧬️schema/🧬️mutations/🦀️.rs` (fixture mount); 36 × `🧬️schema/🧬️mutations/<leaf>/↩️inverse/🦀️.rs`; 43 × `<leaf>/🧪️tests/<scenario>/🦀️.rs` (31 dead smoke test directories removed); `🎫️fixtures/**` deleted |
| DIN V 18599 | `🧫️fixtures/🧬️mutations/🌦️update-climate/🧾️wire-witness/🦠️mutation/🔣️.json`; `🖼️assets/🎬️demo/🗣️.dsl.semio` |
| EN 1996 | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🦀️.rs` (case mount removed) |

**Norm fixtures:** `✏️s/🔌️plugins/📕️norm/🧫️fixtures/🪪️document-contract/🔣️.json` (committed counts).

**Scratch** (`🗑️generated/w2w-norm-2/`):

- lint, contract, phase and cargo logs;
- the staging tree;
- the generator sources;
- the engine counterfactual and regression harness;
- the catalog, registry and ownership checks.

The copied test binaries (188 MB) were deleted.
