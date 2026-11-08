# 🔍️ 1107 Audit Review: Plugin, Plugins and Hub Diff

Lane `aud-review-plugin`. Read-only review. No repo runs, no cargo, no edits outside this ticket folder. Scope: `git -c core.fsmonitor=false diff HEAD -- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin ✏️s/🔌️plugins 🌎️hub` (128 files, +18968/-1618, Cargo.lock excluded from reading). Items outside that pathspec are marked as such.

Method: diffs read per area, path dependencies and workspace keys checked by shell, `repo_text!` fixture paths checked on disk, cross-references checked with `git grep`. Nothing was compiled or run, so compile-level claims are static only.

## Summary

| # | Area | Verdict | Confidence |
|---|------|---------|------------|
| 1 | capsplit runtime-ops split (wire, document, surface) | No loss or duplication found; facade OK | high |
| 2 | capsplit untracked manifests | Commit hazard: tracked manifests depend on untracked crates | high |
| 3 | testing-laws, instance-app | No split defect; duplicate dep line removed safely | medium-high |
| 4 | bundle `consumedTopicsForApp` | Logic correct for the demonstrator; empty-row fallback can drop all topics | medium (code read, not run) |
| 5 | fixture lane host-faithful changes | No weakened assertions found; stricter host lifetime; debug residue left | high |
| 6 | envelope-decode replacement identity fix | Identity formula consistent with admission; fix is correct | high |
| 7 | energy snapshot split | Moved items fully re-homed, consumers updated; untracked crate hazard | high |
| 8 | fem mutation fixtures at runtime | Cwd-independent, all 39 paths exist; loses compile-time check | medium-high |

## 1. Runtime ops capsplit (wire, document, surface)

Files: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/🧰️ops/🦀️.rs`, `.../🧰️ops/🛡️wire-limits/🦀️.rs`, `.../📄️document/🦀️.rs`, `.../🖥️surface-render/🦀️.rs`.

- Moved bodies are unchanged apart from import lines. The old `#[path]` modules now are crate roots (`[lib] path = "../../🦀️.rs"`). No `crate::`, `super::` or `pub(crate)` paths remain in the three roots that point at sibling ops modules.
- Consumers of the old module names still resolve through `pub use semio_plugin_runtime_ops_{wire,document,surface} as ops_*`: `🎞️media/🦀️.rs:12`, `📥️ingress/🦀️.rs:16,18`, `🧾️typed-operations/🦀️.rs:21`. Every item they use (`find_instance`, `runtime_instance_cell`) is `pub`.
- `drive_self_waking_ready` was moved, not copied. One definition now lives in the wire crate (`🛡️wire-limits/🦀️.rs:253`); typed-operations re-exports it (`🧾️typed-operations/🦀️.rs:614`).
- No `cfg(feature = ...)` gates in the moved roots, so no code is silently dropped by a feature that moved crates.
- All path dependencies in the three new manifests and the ops manifest resolve to existing `Cargo.toml` files. The `package.workspace` key resolves to the repo root, the same pattern the ops manifest uses. Root `Cargo.toml` does not list the split crates explicitly, but they are path dependencies of a member, so Cargo adds them implicitly. The framework `Cargo.toml` (`🧰️framework/Cargo.toml:815-817`) lists them explicitly, as it does for ops.
- Every crate name referenced in the three roots is declared in its manifest. `document` has no `serde_json` use, and `wire` and `surface` declare what they use.
- Facade gap: none found for the ops crate.

Findings, all low:

- `🖥️surface-render/staged-tests.rs` is tracked but no `mod` references it. It imports `semio_framework_plugin_runtime_ops::ops_surface_render::staged_context`, a stale path. It is dead code, and it predates today's edits. Confidence high.
- `📄️document/🦀️.rs:51` has a doc link `[`crate::app::ArtifactCodecTableV1`]`. After the move, `crate::` refers to the document crate, so the intra-doc link is dangling. Doc-only. Confidence high.

## 2. Untracked manifests referenced by tracked files (commit hazard)

`git status` shows these as untracked (`??`), while tracked files already depend on them:

- `🧰️ops/📄️document/📦️packages/🦀️rust/Cargo.toml`
- `🧰️ops/🖥️surface-render/📦️packages/🦀️rust/Cargo.toml`
- `🧰️ops/🛡️wire-limits/📦️packages/🦀️rust/Cargo.toml`
- `🏪️store/📬️paged-one-item/📦️packages/` (whole package, outside the requested pathspec)
- energy: `🔋️model/.../📸️snapshot/🌳️structure/📦️packages/🦀️rust/Cargo.toml` and `.../🌳️structure/🦀️.rs` (see section 7)

Tracked manifests that need them: the ops `Cargo.toml`, `🧰️framework/Cargo.toml` (lines 622, 815-817), the energy model `Cargo.toml` and `Cargo.lock`. A commit that omits the untracked files will not build from a clean checkout. Confidence high.

## 3. testing-laws and instance-app

- `semio-plugin-testing-laws` (`🧪️testing/📦️packages/🦀️rust`) has lib root `🧪️testing/🦀️.rs`, the same file as before. It is not split. The `🎟️caller` crate now holds `OriginalWorkFixture`, `receive_*_fixture`, and the new `typing_run` module. The `#[path]` target `🎟️caller/👷️work/🦀️.rs` exists. The re-exports in `🧪️testing/🦀️.rs:34` cover the moved names. Confidence medium-high.
- `🧵️instance/📦️packages/🦀️rust/Cargo.toml` drops a line `semio-plugin-instance-stores` (with `features = ["artifact-app-testing"]`). The same path is still a dependency under `semio-framework-plugin-instance-stores` (line 52), and the `artifact-app-testing` feature still forwards to it (line 25). No code uses the removed name. Confidence medium-high.
- Not reviewed: `paged-one-item` contents (outside the pathspec). It exists and its manifest's lib path resolves. Its only consumers are the workspace and lock entries.

## 4. Bundle lane: `consumedTopicsForApp` (outside the requested pathspec)

The change is in `🧰️framework/🔨️modules/🎠️kernel/🧾️contributions-json/🟦️.ts` (new function, lines 48-66) and `.../🏛️ShellHost/🔁️contributions-refresh/🟦️.ts:182`. It is not in the pathspec, so I read it separately.

Logic: `owner.consumes ∩ receiver.consumes`, where the owner is the loaded non-receiver plugin whose manifest lists the app id.

- Demonstrator case: the standalone-parity test (`🌎️hub/.../🔬️standalone-parity/🦀️.rs:146`) asserts the bundle `consumes` row equals the union of its six hosts. The intersection therefore equals the owner's row. A cad, process3d or sourcing pane receives exactly its owner's `consumes`, which is what the function is for. Confidence medium-high.
- Miss risk 1 (medium): `registry.find(owner)?.consumes ?? []` returns an empty list when the owner has no registry row. The pane then gets no topics at all, where the old code gave it the receiver row. Expected to be rare, not verified.
- Miss risk 2 (medium): if the owner program is not in `loaded` (embedded in the bundle), `owner` is undefined and the result falls back to the receiver row, i.e. the union. The 555 KB pack problem that motivated the change returns in that case. The comment says the owner is loaded, which I did not verify.
- Miss risk 3 (by design, not a bug): a pane that acts on a topic missing from its owner's own `consumes` row never receives it. For generator (procedural owner), I could not confirm whether its owner row covers everything it uses.
- Consistency check passed: `programPluginIdV1` (`📦️program-sessions/🟦️.ts:56`) and the receiver lookup both use `catalogModule?.pluginId ?? handle.pluginId`.
- Unverified: the `consumes` rows of the plugins. They are not in `Cargo.toml` `[package.metadata.semio]` for these artifacts (git grep found no match), so I could not compare owner rows against pane needs.

## 5. Fixture lane: host-faithful changes and test assertions

Not weakened, as far as I could check:

- Pane smokes (cad, process3d, gis, sourcing) now load through `setActiveExample` via the new `assert_pane_smoke_by_example` (`🧪️artifact-app-laws/🪧️pane-smoke-law/🦀️.rs:35`) instead of pushing DSL text directly. This is closer to what the host does. The same assertions on the projection remain. Confidence high.
- `typing_run()` (`🎟️caller/⌨️typing-run/🦀️.rs`) keeps the final `assert_eq!` against the fixture's expected text, now inside the testing crate. Confidence high.
- `app-child-member-registry`: `if denied{assert_eq!(turn.work,0)}else{assert!(turn.work>0)}` is kept. Only the trait changed. Confidence high.
- `fixture_renew_turn` (`artifact-app-laws/🎟️fixture-dispatch.../🦀️.rs`) and `ensure_fixture_cancellation_issuer` in the command and response paths make the fixture stricter (host lifetime stock, births). No check is loosened. Confidence high.
- Removed `expect(...)` calls on `renew_turn` were replaced with fallible `fixture_renew_turn`, so errors now propagate through `?` instead of panicking in the law. Behaviour differs only on failure. Low.
- Sqlite native-output tests (`stdio/.../🛫️native-output/🦀️.rs`): the removed `is_err` and loop forms are re-expressed. The refusal case with `|_| false` is still asserted. I spot-checked; not every line was matched one-to-one. Confidence medium.

Problems:

- Leftover `[DEBUG]` lines (AGENTS requires removal before finish). Seven additions in four files:
  - `✏️s/.../🪵️sourcing/.../🪧️pane-smoke/🦀️.rs`: a new test `aussuchen_apply_probe` (lines ~35-53) with `eprintln!("[DEBUG] probe ...")` lines. It asserts nothing beyond `.expect`, so it is a probe, not a law. Confidence high.
  - `🧵️instance/.../📖️ref-reads`, `⏪️time-travel`: `[DEBUG] runtime time-travel residue` on a refusal path. Confidence high.
  - `♻️cancellation/🦀️.rs:285`: the `Drop` assert now formats `Backtrace::force_capture()` under a `[DEBUG] ledger-lease-drop` label. The backtrace is only evaluated when the assert fails, so there is no runtime cost on the happy path. Still must be removed. Confidence high.
  - `🧪️testing/.../🏗️registered-app-construction.../🦀️.rs`: `[DEBUG] rb-gate settle-idle`. Confidence high.

## 6. Envelope-decode replacement identity fix

- `envelope_decode_replacement(op, gen)` = `ReplacementIdentity::new(op.0, op, gen)` (`🗃️document/📨️envelope-decode-and-ingress/🦀️.rs:6-8`).
- Admission uses the same formula: `🗃️document/🔁️store-replacement-admission.../🦀️.rs:12` and `:85`. Decode begin uses the helper: `📄️docs/📥️staging/🚪️ingress/🦀️.rs:260,279`.
- The maintenance driver now installs that identity with `cx.with_replacement_current(current, ...)` around `active.drive(...)` (`🪜️maintenance-ladder/📥️staging/🗝️envelope-ladder/🦀️.rs:202-208`). Before the fix, the driver had no matching identity, so stale-slot checks compared against nothing. The fix is correct. Confidence high.
- Caveat: `with_replacement_current` restores the previous identity only on normal return. There is no drop guard, so a panic inside `run` leaves the identity installed. Low risk, since a panic aborts the turn anyway.
- Related change: `ArchiveWorkTurn` (`🗃️document/🗄️document-archive-load/🦀️.rs:6`) replaces `NativeDecodeWorkRecipient` with a `&dyn` turn and `with_loan`. Compile not verified.

## 7. Energy snapshot split

- Moved out of `📸️snapshot/🦀️.rs` into the new crate `semio-s-artifact-energy-model-structure` (`📸️snapshot/🌳️structure/🦀️.rs`): `STRUCTURE_TREE_SIBLING_CEILING`, `StructureOverviewGroup`, `energy_structure_overview`, `energy_structure_tree`. The snapshot root no longer declares or re-exports them.
- `git grep` for the four names finds no remaining references to the old location. Consumers were updated: editor and viewer `🌳️structure` modules and their tests, plus `Cargo.toml` deps in the edit and viewer crates. Confidence high.
- Signature change: `energy_structure_tree` gained a `locale` argument. All tracked call sites I found pass it. Confidence medium-high.
- Test ceiling assertions were kept, now referring to the new crate (`🪟️windows/🌳️structure/🧪️tests/🔬️unit/🦀️.rs:48,50`). Not weakened.
- Doc-only stale link: `👁️viewer/.../🌳️structure/🦀️.rs:28` refers to `crate::energy_structure...`. Low.
- Hazard: the new crate is untracked (see section 2).

## 8. Fem mutation test fixtures read at runtime

File: `🧪️tests/🔬️mutation-grammar-conformance/🦀️.rs`.

- 39 `include_str!` calls with relative `../../../` paths became `repo_text!("<repo-relative path>")` (`🧰️framework/🔨️modules/🧪️test/🧲️fixture-reader/🦀️.rs:149`). The macro reads at runtime, using `CARGO_MANIFEST_DIR` baked at compile time and walking up to the workspace root. The process cwd is not used.
- Under a built test binary's cwd: works from any directory, as long as the source checkout still exists at the compile-time path. I checked all 39 paths exist on disk. Confidence high.
- Lost: a rename now fails at test runtime, not at compile time. The three grammar `include_str!` calls in the same file were kept, so the file mixes both styles. Confidence high.

## Recommended follow-ups (not done, read-only task)

1. Remove the seven `[DEBUG]` lines and the `aussuchen_apply_probe` test, or convert the probe into a law with assertions (AGENTS).
2. Stage the untracked crates before any commit: three ops split manifests, `paged-one-item`, energy `🌳️structure`, and the `🌳️structure` untracked files.
3. Decide whether `consumedTopicsForApp` should fall back to the receiver row when the owner has no registry row (section 4, risk 1), and confirm the owner program is always in `loaded` for the demonstrator.
4. Delete `🖥️surface-render/staged-tests.rs` or wire it; fix the stale path.
5. Compile-verify the split crates and `ArchiveWorkTurn` change in the owning lanes; this review did not compile.
