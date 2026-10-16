# Result: wave 4, `semio-framework` facade carved into 21 producer crates

Executor: Sonnet 5.5, foreground, no sub-agents, 2026-10-15 (start 14:45, live apply 16:02, fixes until about 16:30). Slug `FRAMEWORK-FACADE`, tool prefix `w4fw`. Nothing was committed, staged, stashed or checked out by me. Tools: `🪓️monolith-split/w4fw-tools/` (engine `cut.py`/`apply.py`, oracle `asm.py`, probe `probe.py`, scans `privacy_run.py`, `dupcheck.py`, `pubnames.py`, `lossless.py`, `regions_check.py`, `static_checks.ts`). Logs: `🗑️generated/w4fw-*`. Ownership note for the parallel relocation session: `w4fw-OWNERSHIP-NOTICE.md`.
`F` = `🧰️framework`, `M` = `F/🔨️modules`.

## 0. Verdict

- The facade (`F/📦️packages/🦀️rust`, tier 3) was a 30-file, 863,332-byte, 15,141-line source closure (124 files / 1,827,372 bytes / 24,783 lines with tests). It is now a **1-file, 24,232-byte, 152-line** lib root of explicit `pub use` items (plus the `FRAMEWORK_RESERVED_CONTRIBUTIONS` constant and two test mounts) over **21 new producer crates**. Every crate and every test selection is inside the compiler-ingress gate (4000 lines / 32 files / 262144 bytes), and inside the carving target (3500 / 28 / 230000) except the facade's own test selection (247,640 bytes, section 8).
- The kernel source file `M/🎠️kernel/🦀️.rs` has exactly one owner: `semio-framework-kernel`. It no longer is `#[path]`-mounted by the manifest (nor by anyone). The other regions of the old kernel file moved to four sibling crates.
- The census now reports the facade and all 21 crates as `overLimit false`. `closureCompilable` stays false for 17 of them, only because their dependencies `semio-framework-replication` and `semio-framework-ui-contract` are oversized peer-owned tier-0 crates.
- **Not verified by a compiler:** the launcher is down (`Finite original maintenance authority required`; the Nx producer route fails earlier with `Canonical native dependency owner required: ...◻️2d/⚙️engine/...`). No `cargo check` or `cargo test` result exists. Everything below is static analysis plus the rustc name-resolution oracle. Do not read it as "tests pass".
- The product vocabulary was handled differently from the brief, see section 4.

## 1. Census (canonical `readRustSourceClosureV1`, `🗑️generated/w4fw-census-after.log`, host cfg)

| | Lib files / bytes / active lines | Test files / bytes / active lines |
|---|---|---|
| `semio-framework` before (14:47, first measurement) | 30 / 863,332 / 15,141 | 124 / 1,827,372 / 24,783 |
| `semio-framework` after | 1 / 24,232 / 152 | 5 / 247,640 / 2,443 |

| Crate | Node (`<node>/🦀️.rs`, package `<node>/📦️packages/🦀️rust`) | Lib files / bytes / lines | Test files / bytes / lines | First-party deps | Extra dev deps |
|---|---|---|---|---|---|
| `semio-framework-kernel-identity` | `🎠️kernel/🪪️identity` | 1 / 31,336 / 693 | 2 / 45,449 / 972 | - | - |
| `semio-framework-kernel-history` | `🎠️kernel/📜️history` | 1 / 51,548 / 771 | 12 / 143,052 / 1,030 | - | - |
| `semio-framework-kernel-ui` | `🎠️kernel/🖼️ui` | 1 / 44,145 / 973 | 5 / 80,334 / 1,542 | - | - |
| `semio-framework-kernel-media-jobs` | `🎠️kernel/🎞️media-jobs` | 1 / 41,573 / 824 | 3 / 48,472 / 872 | `kernel-identity` | - |
| `semio-framework-kernel` | `🎠️kernel` | 5 / 54,992 / 1,246 | 24 / 163,573 / 2,681 | `kernel-history`, `kernel-identity`, `kernel-media-jobs`, `kernel-ui`, `manifest-media`, `manifest-view-model` | - |
| `semio-framework-abi` | `🌉️abi` | 2 / 45,149 / 1,118 | 5 / 64,349 / 1,466 | - | - |
| `semio-framework-action-bus` | `🎯️action-bus` | 2 / 48,538 / 910 | 6 / 94,712 / 1,612 | `manifest-actions`, `manifest-app-surface` | `manifest-arg-schema`, `manifest-media` |
| `semio-framework-source-projection` | `📁️filesystem/📷️snapshot` | 1 / 4,524 / 92 | 3 / 21,256 / 120 | - | - |
| `semio-framework-interaction` | `🕹️interaction` | 6 / 47,415 / 378 | 10 / 75,719 / 649 | - | - |
| `semio-framework-manifest-address` | `🛂️manifest/🧭️address` | 1 / 12,619 / 302 | 1 / 12,619 / 302 | - | - |
| `semio-framework-manifest-media` | `🛂️manifest/🎞️media` | 1 / 23,344 / 427 | 2 / 26,762 / 477 | - | - |
| `semio-framework-manifest-view-model` | `🛂️manifest/🖼️view-model` | 1 / 27,662 / 475 | 15 / 76,974 / 989 | - | - |
| `semio-framework-manifest-contribution` | `🛂️manifest/🧩️contribution` | 1 / 26,945 / 554 | 2 / 29,839 / 595 | - | - |
| `semio-framework-manifest-arg-schema` | `🛂️manifest/🧮️arg-schema` | 1 / 64,795 / 1,176 | 3 / 78,305 / 1,236 | `kernel-identity` | - |
| `semio-framework-manifest-mutation-inputs` | `🛂️manifest/🔡️mutation-inputs` | 3 / 81,265 / 933 | 7 / 202,992 / 1,148 | `manifest-arg-schema` | - |
| `semio-framework-manifest-actions` | `🛂️manifest/🕹️actions` | 2 / 68,418 / 1,165 | 2 / 68,418 / 1,165 | `kernel-identity`, `kernel-ui`, `manifest-address`, `manifest-arg-schema` | - |
| `semio-framework-manifest-app-surface` | `🛂️manifest/🧱️app-surface` | 1 / 62,008 / 1,183 | 10 / 152,742 / 2,434 | `interaction`, `kernel-history`, `kernel-identity`, `manifest-actions`, `manifest-address`, `manifest-arg-schema`, `manifest-media`, `onboarding` | `kernel`, `kernel-ui` |
| `semio-framework-manifest-plugin` | `🛂️manifest/🧾️plugin-manifest` | 1 / 38,758 / 660 | 4 / 56,104 / 1,011 | `kernel-identity`, `manifest-actions`, `manifest-app-surface`, `manifest-arg-schema`, `manifest-contribution`, `manifest-media` | - |
| `semio-framework-onboarding` | `🎓️onboarding` | 1 / 69,649 / 1,310 | 3 / 100,764 / 1,811 | `manifest-actions`, `manifest-address`, `manifest-arg-schema` | - |
| `semio-framework-time-travel-actions` | `⏪️time-travel/🕹️actions` | 1 / 19,039 / 255 | 4 / 38,856 / 374 | `manifest-actions`, `manifest-address`, `manifest-app-surface`, `manifest-arg-schema` | - |
| `semio-framework-tool-run-actions` | `⏯️tool-run/🕹️actions` | 1 / 7,552 / 108 | 3 / 29,491 / 311 | `manifest-actions`, `manifest-app-surface`, `manifest-arg-schema` | `interaction` |

Largest lib closure: onboarding, 1,310 lines. Largest test closure by bytes: manifest-mutation-inputs, 202,992 bytes (the 99 KB mutation corpus fixture); by lines: kernel, 2,681. Deepest new file path: 132 bytes (limit 240).

## 2. Crate graph (downward DAG, checked by `static_checks.ts`: no cycle over normal and dev edges)

Layers, low to high (a crate depends only on crates above it in this list):

1. No first-party deps: `kernel-identity`, `kernel-history`, `kernel-ui`, `abi`, `source-projection`, `interaction`, `manifest-address`, `manifest-media`, `manifest-view-model`, `manifest-contribution`.
2. `kernel-media-jobs` (identity), `manifest-arg-schema` (identity), `manifest-mutation-inputs` (arg-schema).
3. `manifest-actions` (identity, ui, address, arg-schema), then `onboarding` (actions, address, arg-schema).
4. `manifest-app-surface` (interaction, history, identity, actions, address, arg-schema, media, onboarding). `action-bus` (actions, app-surface; also owns the `platform` shell as a mounted child).
5. `manifest-plugin` (identity, actions, app-surface, arg-schema, contribution, media), `time-travel-actions` and `tool-run-actions` (actions, address, app-surface, arg-schema), `kernel` (history, identity, media-jobs, ui, media, view-model).
6. `semio-framework` facade (all of them).

Crate ids are `<id>`, package `semio-framework-<id>`, Nx project `@semio-tech/<id>-rs`, package dir `<node>/📦️packages/🦀️rust`, lib path `../../🦀️.rs`. Folders: `🪪️identity`, `📜️history`, `🖼️ui`, `🎞️media-jobs` under `M/🎠️kernel/` (the kernel root file stays `M/🎠️kernel/🦀️.rs`); `🧭️address`, `🧩️contribution`, `🔡️mutation-inputs` are new under `M/🛂️manifest/`; the other manifest crates reuse the existing child folders (`🧮️arg-schema`, `🕹️actions`, `🧱️app-surface`, `🧾️plugin-manifest`, `🖼️view-model`, `🎞️media`); abi, action-bus, snapshot and interaction keep their module roots; onboarding, history-edit and tool-run actions are the three modules the relocation session had mounted into the facade.

Partition rationale (item-level, from `dag.py`): an entity-level reference graph over 719 top-level entities (types, fns, consts, macros, with impls and macro calls attached to their type) was built from tokens, duplicate names (`Platform`, `ActionInvocation`, `CommandInvocation`) resolved by file family, and clustered by hand. The file-level kernel/manifest cycle is not an entity-level cycle: the kernel needs only `MediaType`, `ViewModel` and the fault notice definitions from the manifest, so those moved below the kernel.

## 3. How the cut was done

- Files stay where they are; each crate root is an existing module file (rewritten from its entities) or a new file beside a new `📦️packages/🦀️rust` folder. Existing child modules stay mounted by `#[path]` from the one crate that owns them: kernel `return_content`, `return_message`, `cold_pair`, `clipboard_envelope`; interaction `gesture`, `schema`; actions `initial_window_utility`; action-bus `platform`. Every module file has one owning crate (census lists, `asm.py` ownership map).
- Each root starts with an emoji docstring (unique per file), then explicit imports generated from usage (cross-crate names from the owning crate, external names from the original `use` lines and the old parent header, `kernel::X` and `crate::ui::X` qualifiers resolved to bare names plus imports), then the entities in original order with the original `//#region` stacks regenerated, then test mounts. `pub(super)` became `pub(crate)`; `ActionArgDef::with_schema` and `plain_string` became `pub` (used from the mutation-inputs crate).
- `include_str!`/`#[path]` literals of moved code were rewritten for the new directory depth (`🔣️input-labels.json`, value schema, test mounts). Test files stay in place and only had their `crate::ui::...`/`super::...` paths rewritten to the owning crate.
- Test structure: tests moved with the crate that owns the tested items, or the highest crate that can see everything they use (kernel owns the effect/import/export tests, app-surface owns the app-label suite). Three cross-cutting adjustments: the `app_with` helper of the app-label suite became `semio_framework_manifest_app_surface::testkit::app_with` (feature `testkit`, dev-dependency feature in `tool-run-actions`); the `exports_typescript_bindings` test moved with `schema_metadata` into the facade (`🧬️schema/📽️projection/🧪️tests/🦀️.rs`, module `typegen_tests`); the two "the framework hosts this contribution" asserts moved into the facade test `🛂️manifest/🧪️tests/🧩️framework-reserved/🦀️.rs`. The three crates whose tests call `semio_framework_trace::observe_heap_allocations_on_this_thread` (action-bus, mutation-inputs, view-model) now install `HeapWitness` as test global allocator, replacing the facade's old `RequestedAllocator`, which forwarded to the same witness.
- The facade root keeps the old module paths as aliases of the crates (`abi`, `action_bus`, `platform`, `interaction`, `source_projection`, `history_edit`, `tool_run`, `onboarding`) and two hand-listed modules (`kernel`, `manifest`, with `ui` as alias), all explicit `pub use` lists computed from the public items of each crate (no glob). The root-level surface was reproduced by simulating Rust's explicit-over-glob shadowing.
- God-object rule: not triggered. The largest inherent impls are `InputSchemaReader` (462 lines, mutation-inputs), `ActionDefinition`, `ActionBus` (251) and `UiTurnPatches` (173); each lives next to its type in a crate of at most 1,310 lines. No extension trait was introduced.

## 4. Product vocabulary (`🧳️product-vocabulary`)

- The brief said to relocate it. A second session ("Relocate wrong-direction code", `📓️RELOCATION-BRIEF.md` slice A) was doing exactly that at the same time, and the coordinator then told me to leave it to that session and, if needed, keep the vocabulary as one unit. By the time I could cut (16:02) that session had already dissolved the file: `product_vocabulary` is no longer mounted, the file is deleted, and its items are in `onboarding`, `time-travel/🕹️actions` (history-edit), `tool-run/🕹️actions`, `ReservedContribution` in the manifest actions file, and the media enums, `PackageRole`, `AppRole`, `ActionKind` and `InteractiveJobClassification` in the manifest children. So I created **no** vocabulary crate (my earlier two-crate plan, `manifest-product-vocabulary` low and `manifest-product-actions` high, was dropped before any file was written) and relocated **nothing** myself.
- What I did with their result: the three new modules became crates (`onboarding`, `time-travel-actions`, `tool-run-actions`) so no framework crate carries them in the facade; `ReservedContribution` and its `reserved_*` helpers (they take `&AppDefinition`) sit in `manifest-app-surface`; `AppRole` moved from the surface file down into `manifest-arg-schema` (arg-schema uses it in `ArgFormat`, the surface depends on arg-schema); `MediaClass`, `MediaForm`, `OsMediaCapability` are in `manifest-media`, `PackageRole` in `manifest-plugin`; `ActionKind` and `InteractiveJobClassification` in `manifest-actions`.
- Not relocated, still framework-resident (for that session to decide): the closed enums `ActionKind`, `InteractiveJobClassification`, `AppRole`, `PackageRole`, `OsMediaCapability`, `MediaClass`, `MediaForm` (hundreds of consumers: 206, 280, 310, 9, 118, 215 and 216 files respectively), and `AppDefinition.introduction`/`tutorials`, which still embed the onboarding types (so `manifest-app-surface` depends on `onboarding`). Replacing the enums by open string-keyed registration needs the wire shape and all consumers decided first.

## 5. Consumers and wiring changed

- Facade: `F/📦️packages/🦀️rust/🦀️.rs` (rewritten), `Cargo.toml` (dependencies pruned to what the root and its integration test use, plus the 21 crates), `📜️script.ts` (new `FRAMEWORK_PACKAGES`; every `testCargo`/`captureCargo`/lint call now names the owning package and a module-path-free filter, the `test`/`lint` routes run all 22 packages).
- Workspace: `F/Cargo.toml` (21 members inserted by byte order), `📋️build-owners.json` (21 rows), `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` (126 routes each, unused order keys, inserted after the pets routes), 21 `Cargo.toml` and 21 `📋️project.json`. `Cargo.lock` files not refreshed (launcher down).
- Source-text readers: root `📜️script.ts` (the manifest reader now reads the actions root; the interactivity audit slices the `TurnResult` region of the kernel root and of the new `🖼️ui` root; `INTERACTIVITY_AUDIT_KERNEL_UI_FILE` exported), `📖️source-access/🟦️.ts` (the deleted manifest root left `POLICY_SPLIT_SOURCE_ROOTS`). Syntax of all three files checked with `bun build --no-bundle`.
- Deleted: `M/🛂️manifest/🦀️.rs` (parent), `🌲️canonical/🦀️.rs`, `📇️space-index/🦀️.rs` (merged into `🧩️contribution`). Edited: every old child root, `🖥️platform/🦀️.rs` (two `use` lines), the test files of the moved suites (path rewrites), `🛂️manifest/🧪️tests/🔬️app-label/🦀️.rs` (helper and typegen test extracted). New test files: `🧰️testkit`, `🧩️framework-reserved`, `🧬️schema/📽️projection/🧪️tests`.
- External consumers of the facade: none edited. The 978 distinct `semio_framework::...` paths used by the repository's Rust files (`probe.py`, list in `scratchpad/consumer-paths.json`) resolve identically before and after (0 new-only failures against the original facade; the old/new failure sets are the `Type::assoc_fn` imports and unresolved external crates).

## 6. Verification (every command run; results real)

| Check | Result |
|---|---|
| Census, both cfgs, 21 crates + facade | table in section 1; all `overLimit false` |
| Entity audit (every top-level unit emitted once or intentionally dropped) | clean; intended drops: the 6 mount-and-glob pairs of the old manifest parent, `pub mod kernel`, the canonical mount, `test_allocation` |
| Public-name preservation (`pubnames.py`) | 706 public names in the old closure, 708 in the new one; only `kernel` (the mount) missing, new: `platform`, `schema`, `testkit` mounts |
| Lossless line multiset old vs new (`lossless.py`) | missing lines are only mounts, old imports, module docs, path rewrites, the dropped `test_allocation` file; no code line lost |
| Region markers (`regions_check.py`) | 120 marker lines before, 140 after; missing region names `Children`, `Kernel` (their mounts no longer exist); `Tests`, `SchemaMetadata` restored around their mounts; every file balanced except `🗣️return-content-dialects/🦀️.rs`, unbalanced at HEAD already |
| Multi-crate name-resolution oracle (`asm.py`: each crate mounted as a module, `crate::` and sibling crate names rewritten; `rustc +nightly-2026-07-20 --emit=metadata -o /dev/null`) | native: 1,919 errors, 2 non-external (1 absolute `::replication::` path, 1 abort line); `--test`: 2,894 / 5; `--cfg test`, wasm32-wasip2, wasm32-wasip2 `--test`, wasm32-unknown-unknown: same classes. Every non-external error is an artifact of the simulation (`::replication` unresolved, `#[macro_export] tree_pin!` path, `CARGO_PKG_VERSION`). Baseline original facade: 1,798 / 2 native, 2,703 / 9 `--test` |
| Oracle sensitivity | removing one cross-crate `use` gave 7 E0425; making `ActionDefinition` private gave 2 E0603; removing `CapabilityAudience` from the facade lists gave 4 new-only probe failures |
| Definition scan (`privacy_run.py`: private fields, tuple fields, inherent methods used from another crate) | 8 candidates, all name collisions (`Deserialize`, `From`, `Node`, `runs`, `cursor`, `length`); the real hits of the first run (`with_schema`, `plain_string`) were widened |
| Duplicate-import scan (`dupcheck.py`) | clean after fixing a real bug it targets (a cfg(test) import repeating a lib import; unresolved external imports hide that from rustc; sensitivity shown with a seeded duplicate) |
| Orphan-rule scan (`orphan.py`) | every impl in a crate names a local type or trait |
| rustfmt parse of every crate root (`+nightly-2026-07-20 --emit stdout skip_children`) | all parse |
| `static_checks.ts` | `STATIC CHECKS PASSED`: 21 `Cargo.toml` parse with Bun TOML, names, exact three boundary keys, `workspace` resolves to `F`, `[lib] path`, workspace membership, every path dependency resolves to the right package name, owner rows, 126 launch routes in both files (commands exact), unique order keys, no cycle, facade depends on all 21 |
| `bun repo cargo ... check -p semio-framework-abi` | `error: Finite original maintenance authority required` (before the compiler) |
| `bun repo run @semio-tech/manifest-address-rs:check ...` | `Canonical native dependency owner required: 🧰️framework/🔨️modules/◻️2d/⚙️engine/📦️packages/🦀️rust` (peer 2d move) |
| TS tests | none run: the changed TS files only had string/array edits (syntax checked); the heavy gates (`workspace-contract`, policy script) were not executed |

## 7. Wrong-direction items I found but did not move

- `semio-framework-ui` (`ui_wgpu`) is a dependency of kernel, address, actions, app-surface, interaction, onboarding for `IconName`, `UiNode`, `NamedLayout`, `SurfaceKind`, `WindowLayout`, `WindowOptions`, `ActionDescriptor` (UI product types in the manifest, section W3 of the seam map). Unchanged.
- `AppDefinition` embeds `IntroductionDefinition` and `TutorialDefinition` (section 4).

## 8. Residual risks

1. **No compiler ran.** Type-phase problems are invisible to the oracle: missing trait imports for method calls (only `to_value`/`from_value` are handled by a heuristic), field or method privacy beyond the definition scan, derive output, trait coherence, `async`/`Send` effects, unused-import warnings (imports are generated from tokens, so some unused ones are expected). Run the real `check`, `--tests`, wasm32-wasip2 and wasm32-unknown-unknown first.
2. **Lock files** (`Cargo.lock`, `F/Cargo.lock`) lack the 21 packages and the changed facade edges; a `--locked` resolve fails until `bun repo cargo -- update --workspace --offline` runs.
3. **`semio-framework-abi` is still mounted by two ui crates** (`ui-host` and `ui-backend-webgpu`, `#[path] pub mod abi`), as before. Making them depend on the new crate also needs `🧪️tests/🔬️workspace-contract` (line 655 asserts the mount) and the `package-language-kind-handoff/🖥️ui-host-package` fixture changed, which belong to peers, so I left them.
4. **Facade test selection is 247,640 bytes** (limit 262,144, target 230,000): `schema_metadata` (124,759) and the generated manifest TypeScript (97,613) are embedded by the typegen test. Moving that pair into its own tiny test crate would fix it; then the `generate`/`preview-generated` scripts need `-p` for that crate.
5. **Test global allocator:** the facade no longer installs `test_allocation::RequestedAllocator`; the mount was unused inside this closure (io/control mounts its own copy). If a hidden consumer relied on it, tests that measure requested bytes would read zero.
6. **Cross-crate test helpers:** `testkit` is a cargo feature of `manifest-app-surface`; it must be enabled by dependants' dev-dependencies (done for `tool-run-actions`). A new dependant that forgets it fails with an unresolved import.
7. **`Cargo.toml` hygiene:** crates use the facade's dependency specs verbatim (`ui_wgpu` with feature `wgpu`, workspace-inherited crates); `🔒️dependencies.json` (external usage lock) and `verify dependencies literal-external` were not run and do not list the new manifests.
8. **Source-text gates not run:** the root policy script, `readModuleTreeSource` readers and the TS twin comments that still name `🛂️manifest/🦀️.rs` (comments only) were checked by grep, not executed. A concurrent session was editing the same tree throughout (manifest, builder, tool-run, time-travel); the cut ran once, from live state, at 16:02 after 14 minutes without changes in the closure, and was re-synced for three later engine fixes (region wrapping, one dropped re-export in `tool-run-actions`, duplicate test imports) only where the live file equalled my previous output up to those lines.
9. **Peer sweeps:** a peer sweep that strips qualified paths could break `semio_framework_manifest_app_surface::testkit::app_with` style test paths; they are used on purpose instead of bare names because the test files have no matching imports.
