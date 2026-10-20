# result-gran-keys: Rust and schema cache keys become per-crate derived keys under nx and Bazel (staged, nothing applied)

READY: staged set complete 10-20 21:05 in `C/gran-keys-tools/` (`apply.py` dry run: 170 operations, no anchor drift against the live tree; `--apply` after the coordinator's GO; `--undo` restores). Executor `gran-keys`, wave 1020e. Stage-only: nothing outside `C/` was written; every run used `/private/tmp/gk/` (sandbox clone of the library plus all 1,471 manifests), one process at a time, `nice -n 10` after the coordinator's load note. No `bun repo`, Bazel or cargo over the real repository; cargo ran only on a 25 MB-class overlay copy (one `cargo check`, `-j 2`).

## 1. What the audit got right, what it got wrong (verified in code and in 3,826 real manifest records)

| Audit claim | Measured |
|---|---|
| nx `cargo` named input hashes into nearly every Rust target | Used by one project (`@teaching/proctor`). The real whole-workspace Rust key is elsewhere (next rows). |
| `schemaSources` hashes into nearly every Rust target | Six consumers: five root `schema *` targets and `repo-test-domain:test-schema`. All validate or generate every schema scope by design. No Rust crate target consumes it (a crate keys the schema files it `include_*!`s, exactly). |
| root `Cargo.toml` / `.cargo/config.toml` / `rust-toolchain.toml` in every Rust key | True, but through `toolchains.cargo.files` (policy): `nativeCommandInputs` adds them to every native target, `inferProjectInputs` to `default`/`production` of every cargo project. |
| (not in the audit) | **139 projects author an identical `cargoToolchain` named input = root `Cargo.toml` + the whole root `Cargo.lock` + config + toolchain + rustfmt**, referenced by 824 cached targets. It defeats the lock slice nx already computes (any root lock edit re-keyed all of them). |
| (not in the audit) | **Nested workspace manifests are in no key** (`🧰️framework/Cargo.toml` etc.): lints, `workspace.package`, inherited dependency entries, profiles. Edits there reuse stale results today. |
| (not in the audit) | **Bazel declares the nearest `Cargo.lock` file** for a lock marker: whole file, and for **565 of 3,374 crates the wrong file** (a stray gitignored per-crate lock, or `🧰️framework/Cargo.lock` for a crate whose workspace is the root). 193 crates have a stale lock (no local package); that is the existing nx behaviour (hash fails), Bazel now records the refusal in the key and the action fails under `--locked`. |

## 2. Design (schema-first, language-agnostic, one function for both adapters)

New member `🏗️build/📥️inputs/🦀️resolution/`: engine `🟦️.ts` (no I/O of its own), closed schema, policy document `🔣️.json` (EN/DE texts), fixture + strict fixture schema + independent Python oracle.

**Resolution identity of a crate** = fold of: its locked dependency slice (existing `cargoLockClosure`, unchanged), **a Merkle fold over its path-dependency closure of what each crate inherits from its own owning workspace** (`workspace.package` fields it names, `[lints] workspace`, the `workspace.dependencies` entries it uses, plus the workspace's `cargo-features`, resolver, profile, patch, replace; membership and comments never count; dev-dependencies only at the root of a test run; cycles and escaping paths refused), the canonical `rust-toolchain.toml` (no comments, sorted components) and the canonical `.cargo/config.toml` (no comments, minus `build.target-dir`/`build.build-dir`). `composeCargoResolutionV1` is the single folding function.

* **nx**: the existing `SEMIO_CARGO_LOCK_<key>` marker keeps its plumbing; its value (computed in `taskEnvironments`) is now the resolution identity under one shared reading state. The marker name stays (renaming touches six modules; follow-up).
* **Bazel**: a lock marker declares only the crate manifest and stays in the task inputs; `taskFingerprint` folds the same digest (`selectedCargoResolutionDigests`, one batch per plan). The whole `Cargo.lock` file and `workspaceBazelLockFilesV1` are gone.
* **Policy**: `toolchains.cargo.files` keeps only `rustfmt.toml` and the driver script; the three root files move to the new `toolchains.cargo.identity` (still watched, never hashed raw). nx.json and `🏗️build/🔣️policy.json` drop the `cargo` group for a base `cargoToolchain` group.
* **`cargoToolchain` is generated per Rust project** in `inferProjectInputs` (raw tool files + the crate's marker; markers only in persisted graphs, so the live source-watch path is untouched). The 139 authored definitions are removed (text surgery, each verified by parsing before and after); every authored reference keeps working. Flow browser package, os-hub and proctor replace their raw root-lock/manifest leaves by it.
* **Input precision**: a Cargo read now carries the marker of the crates it names instead of raw root Cargo files (150 root `workspace:*` verify laws had them; they also had no lock key at all).
* **Law** (`⚖️laws/🔒️cache-correctness`): new rule `whole-workspace-rust-key` forbids root `Cargo.toml|Cargo.lock|rust-toolchain.toml|.cargo/config.toml` and the patterns `{workspaceRoot}/**/Cargo.toml|Cargo.lock|*.rs|🧬️schema/**/*` in any cached target, expanded through named inputs, unless a `whole`/`assumed` precision decision, a recorded command (`native cargo metadata`) or a recorded exempt target excuses it. Exempt list = 25 ids with reasons, and a **ratchet test** (`staleRustKeyExemptionsV1`) refuses an entry that no longer needs it. 13 language-agnostic corpus cases. The graph assembly also refuses (fail closed) a native target whose inferred command inputs carry such a key.

## 3. Proofs

* **No missed inputs, four independent ways.**
  1. rustc dep-info (`cargo check --offline`, overlay of the `semio-framework-value-paged-list` closure, lib root outside the package dir): rustc read 22 local sources over 4 units; the graph's declared sources for the same crates (26) contain **0 missing**, 2 declared-but-unread are `cfg(test)` modules of dependencies. A second and third overlay (io-schema with `include_str!` schema leaves, schema) could not be compiled because modules of the closure live in sibling trees outside the copied units (`../../../🚪️io/🦀️.rs`): not run, not claimed.
  2. `cargo metadata --no-deps` (real Cargo) on the same 82-crate overlay vs the engine: 82 packages, 474 path-dependency declarations by kind and resolved path, owning workspace, inherited `edition`/`version`/`rust-version`, inherited requirements: **0 mismatches**.
  3. Fixture (`🧪️tests/🔣️.json`): TypeScript engine == Python oracle byte for byte (digests, closure moves for 12 manifest edits, 5 toolchain/config edits, key predicate, canonical JSON); stored trees equal what `Bun.TOML` and `@iarna/toml` parse; refusals agree.
  4. Live adapter code vs independent parser: nx batch path == Bazel plural path == engine over `Bun.TOML` on a fixture workspace on disk; comment-only edits of root manifest, toolchain and config move **no** crate; an inherited `[workspace.lints]` edit moves exactly the inheriting crate; an unrelated lock entry moves none; and the live batch reader agrees with the independent parser on **25 of 25 random real crates**.
* **Schema consumers**: a crate keys its `include_*!` closure (existing source analysis, covered by the dep-info check above) and generator-contract inputs; the six `schemaSources` consumers are whole by design and recorded as such.
* **Tests (sandbox clone, patched vs unpatched baseline)**: resolution 10/10; Bazel closure 8 pass (updated: lock marker kept, no lock file declared), fingerprint 3/3 (new: resolution folded, missing identity refused), Bazel cargo 7/7, precision 12 pass (+1 new test: markers additive, raw root files gone), cache-correctness corpus and ratchet green. Law over the 3,826 real records: **855 cached targets violate before (patched law on current manifests), 0 after**; the only findings left are the 2 known peers (`uncached-without-reason`). Baseline failures (workspace-negation-closure, cargo-transaction-command-source, graph admission, digest suite needing the runner envelope) are identical before and after; one baseline negation-closure failure disappears.

## 4. Before / after: targets invalidated by one edit (model over the real repo: 240 workspaces, 3,374 crates, 12,134 targets = 3 inferred + authored cached)

| Edit | Before (crates / targets) | After (crates / targets) |
|---|---:|---:|
| new crate appended to root `[workspace].members` | 3,374 / 12,134 (+150 root laws) | **0 / 0** (and 0 root laws) |
| root `Cargo.toml` comment or formatting only | 3,374 / 12,134 | **0 / 0** |
| `.cargo/config.toml` comment, or `[build].target-dir` only | 3,374 / 12,134 | **0 / 0** |
| `rust-toolchain.toml` comment or ordering only | 3,374 / 12,134 | **0 / 0** |
| root workspace dependency used by one crate | 3,374 / 12,134 | **1 / 3** |
| root workspace dependency used by 9 crates (a path crate) | 3,374 / 12,134 | 2,590 / 9,249 (their transitive dependents) |
| root `[workspace.lints]` / `[workspace.package]` / `[profile]` | 3,374 / 12,134 | 2,740 / 9,794; 2,724 / 9,716; 2,788 / 9,940 (every crate that compiles with an inheriting crate) |
| `[build].rustflags` or toolchain channel | 3,374 / 12,134 | 3,374 / 12,134 (correct) |
| nested `🧰️framework/Cargo.toml` lints or a dependency entry | **0 / 0 (stale hit)** | 3,258 / 11,745 and 2,274 / 8,098 (**correctness fix**) |
| `Cargo.lock`: package in 562 crates' slices (proc-macro2) | nx 642 / 2,546; Bazel 564 / 2,554+ | 562 / 2,134 (both tools) |
| `Cargo.lock`: package in 113 crates' slices | nx 245 / 1,176; Bazel 564 / 2,885 | 113 / 433 |
| `Cargo.lock`: package in one crate | nx 139 / 827; Bazel 564 / 2,964 | **1 / 5** |
| source edit, leaf / median / p90 / hub crate | 1/3, 21/75, 2,233/7,956, 3,080/11,136 | unchanged (intrinsic fan-in: the value-tower split is `gran-value`) |
| authored cached targets with a whole-workspace Rust key | 855 (139 projects) | **0** (25 recorded exemptions, 7 of them debt) |

## 5. Not done / risks

* **First live exercise is `bun repo show projects`**: the native branch of the assembly with a real `cargoLockInput` (needs graph-writer custody) cannot run offline; the sandbox exercised the plugin load, the source-watch path, the law over real records and every unit layer. `--undo` is exact while nothing moved.
* **Cost**: the identity reads every manifest of the closure once per batch. Measured through the live reader under load 24: 1 crate 6.4 s, 10 crates 11 s, 100 crates 41 s, 400 crates 55 s. A persisted receipt keyed by manifest digests (like the precision program receipt) would remove it; not built.
* Residual whole keys: 150 root laws now carry markers, but `assumed` precision rules (`hub-stdio-oracles`, `plugin-composition-test`) keep inherited inputs; the 25 exempt targets (schema family 6, repo laws 6, membership tests 5, toolchain provenance 1, debt 7: hub-compositions native, stdio-js package-*, stdio-plugin native check, architecture-course check/test/typecheck) are listed with reasons in `🔣️.json`.
* `bazelCargoToolchainDigestV1` still hashes the raw bytes of `rust-toolchain.toml` plus the installed channel manifest (a toolchain digest; a comment edit re-keys Bazel cargo actions only).
* Marker name `SEMIO_CARGO_LOCK_` now stands for the resolution identity; rename is a follow-up.
* `schemaSources` stays for the six whole-by-design consumers; a per-scope partition of the schema registry would be needed to key them narrower.

## 6. Files

`C/gran-keys-tools/`: `apply.py` (dry run default, `--apply`, `--undo`, `--list`), `layout.py`, `patchlib.py`, `patches_core.py` (lock reader, digest, policy and schema, inference, assembly), `patches_bazel.py`, `patches_law.py`, `patches_named.py`, `patches_manifests.py`, `patches_precision.py`, `staged/` (engine `resolution.ts`, `schema.json`, `policy.json`, `test.ts`, `fixture.json`, `fixture-schema.json`, `oracle.py`), `go.sh`, `NOTES.md`, `verify/` (model, census, sandbox builders, suites results, adapter and scale checks, dep-info scripts and outputs).

## 7. READY2 (10-20 21:40): persisted per-crate resolution receipts

* **What**: member `🦀️resolution/💾️receipt/` (codec: sealed, brotli, closed schema `definitions.Leaf`) and `rr/<24 hex>.json` as a third optional graph cache leaf beside `np/` and `ip/` (storage schema regex and graph fixture extended). One receipt per crate: per run kind the files its identity read with their digests (manifest, probed ancestors, owning workspace manifest), what it inherits, what its workspace fixes, the path dependencies with the identities they had and the folded identity; plus the locked slice with the digest of the lock file. A receipt answers while every recorded digest holds; a changed dependency moves only the identities above it, without parsing; a corrupt, foreign, unsealed or schema-violating leaf is no receipt.
* **Who writes, who reads**: the graph build refreshes receipts in `inferProjectInputs` under the graph writer's custody (plain reads, no observation, one shared reading state per graph construction). The hasher (nx `taskEnvironments`, Bazel plan) only reads them and never writes (a consumer has no writer custody), so a stale receipt costs one derivation per run until the next graph build rewrites it. Selection- or dataset-scoped builds write under their own `selections/` or `contexts/` prefix, like `ip/`.
* **Measured, 400 real crates (2,909 distinct crates in their closures), nice -n 10**: graph build cold 8.1 s (3,197 manifests parsed once each, 2,909 receipts, 5.4 MB, 1.85 KB per crate), warm **1.3 s, 0 identities derived**; hash time through the live batch reader cold 16.5 s (each manifest read once, shared across crates), warm **7.3 s, 0 derived, 0 parsed** (3,195 file digests are the whole cost). Identities are equal in all four runs; the lock part equals the legacy `selectedCargoLockDigest` on 25 of 25 real crates.
* **Tests**: resolution 15/15 (warm run: 0 derived, 0 manifest reads; comment-only edit re-derives one crate and moves no identity; a dependency edit moves exactly the dependents; a workspace-manifest edit re-derives that workspace only; lock slice reused, unrelated lock entry moves nothing, missing lock, stale lock refused, tolerant mode; corrupt and foreign receipts); the earlier suites are unchanged (Bazel closure and fingerprint, precision, law, cargo).
* **Graph round**: my 174 operations touch none of conc-graph's 10 files (disjoint paths). In a clone with my set applied, `conc-graph-tools/apply.py check` shows 28 pending / 0 drift, `apply --write` applies, and its five suites pass (generations 22, environment 20, load 26, protocol 33, sealed-reuse 13). Order is free; conc-graph invalidating receipts means one cold graph build rewrites the `rr/` leaves. graph-speed's fact table was not staged yet: nothing of mine reads or writes its table (re-run the clone check when it lands).
