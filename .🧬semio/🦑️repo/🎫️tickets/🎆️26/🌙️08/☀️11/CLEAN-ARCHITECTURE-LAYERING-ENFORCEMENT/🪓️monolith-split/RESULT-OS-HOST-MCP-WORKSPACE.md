# Result: OS host (C) and MCP workspace (D) monolith split

Seam map: `SEAM-MAP-OS-RUST-TIER2.md` sections C and D. Rules followed: `📓️EXECUTOR-RULES.md`, `📓️RESOLUTION-ORACLE.md` (including the sandbox patches).

Status: **both splits applied and verified with the full-resolution differential (native, `--test`, wasm32-unknown-unknown, wasm32-wasip2).** `cargo check` through the repo launcher was not possible (see section 7). Nothing was committed or staged.

Two findings change the instructions and are decided here, not in the map:

1. **The two `*_legacy` functions in C are not legacy. They are the only production path for every artifact that has no `IoEntry` yet, so they were renamed, not deleted** (section 2).
2. **The probe document in D is not a test fixture. Production code (`artifact_create`, inference discovery, the artifact byte/provenance/resource lanes, the hub surface binding) depends on it, so it was isolated in its own child and left in production** (section 3.4). `cfg(test)`-gating it would remove a shipped gateway feature and was not done.

## 1. Result at a glance

| | Before | After |
|---|---|---|
| C `🖥️host/🦀️.rs` | 4,671 lines, 266,024 bytes | parent 61 lines (51 active), 8 child files |
| D `🏠️workspace/🦀️.rs` | 5,831 lines, 384,935 bytes | parent 161 lines (108 active), 12 child files |

### C children (folder beside the parent, file `🦀️.rs`)

| Folder | Module | Lines | Active | Bytes |
|---|---|---|---|---|
| `🔌️plugin-host` | `host` | 1360 | 988 | 77,709 |
| `🗃️document-backbone` | `backbone` | 165 | 128 | 9,884 |
| `🧱️instance` | `instance` | 453 | 334 | 21,801 |
| `🖼️media-export-raster` | `media_export_raster` | 219 | 160 | 12,772 |
| `🧾️media-export-simple` | `media_export_simple` | 81 | 68 | 4,020 |
| `🔀️workflow` | `workflow` | 817 | 572 | 52,672 |
| `📡️codec-abi` | `codec_abi` | 1013 | 918 | 40,530 |
| `🗂️registry` | `registry` | 502 | 360 | 25,610 |

### D children (under `🏠️workspace/`)

| Folder | Module | Lines | Active | Bytes |
|---|---|---|---|---|
| `🧭️plugin-paths-and-descriptor` | `plugin_paths_and_descriptor` | 92 | 54 | 6,135 |
| `🔬️probe-document` | `probe_document` | 294 | 210 | 17,237 |
| `🔖️plugin-binding-and-activation` | `plugin_binding_and_activation` | 133 | 88 | 7,732 |
| `🧊️compiled-components` | `compiled_components` | 404 | 293 | 23,283 |
| `📨️plugin-artifact-channel` | `plugin_artifact_channel` | 1203 | 765 | 82,485 |
| `🧩️artifact-app-frames` | `artifact_app_frames` | 605 | 402 | 39,926 |
| `🔀️routing-and-shell-routed` | `routing_and_shell_routed` | 694 | 469 | 44,374 |
| `🗂️guest-document-codec` | `guest_document_codec` | 397 | 287 | 31,125 |
| `🤝️hub-relay` | `hub_relay` | 360 | 248 | 20,241 |
| `🪪️headless-workspace-state` | `headless_workspace_state` | 303 | 202 | 21,470 |
| `🛂️gateway-backend` | `gateway_backend` | 1188 | 839 | 84,317 |
| `💡️inference-facade` | `inference_facade` | 213 | 177 | 12,402 |

All 20 children are at most 988 / 839 active lines (cap about 2,500) and at most 84,317 bytes (gate 262,144). Folder names follow `<emoji>️<kebab>` with U+FE0F, all 20 are RGI (checked with `/^\p{RGI_Emoji}$/v`), none repeats a sibling folder, and every emoji already prefixes other folders in the repo. Longest path is far below 240 bytes (`verify_paths.py` checks it). Each child starts with a `//!` docstring whose first character is the folder emoji (for `media_export_simple` the old `🖼️` was changed to `🧾️` because it duplicated the raster child; `codec_abi` had no docstring and got one).

## 2. Task C: OS host

### 2.1 What was done

- The 8 inline `pub mod X { ... }` blocks became `#[path = "<folder>/🦀️.rs"] pub mod X;`. Module paths are unchanged (`crate::host`, `crate::workflow`, ...), the `pub use` block of the parent is unchanged, and no visibility was widened.
- Bodies were moved verbatim and de-indented by 4 spaces (a lexer guards multi-line string literals; none occurred). The inner `//!` docstring of each block moved above its `// #region` marker.
- 9 relative literals gained `../`: 8 `include!("../🧪️tests/...")` and 1 `include_str!("../🧬️schema/🔣️.json")`.
- The packages-root macro `test_identity!` is still declared before `mod host_core;`.
- Test module paths such as `workflow::tests::svg_path_extraction_preserves_transformed_geometry` are unchanged because module names did not change.

### 2.2 The two `*_legacy` functions

Investigation (the code and its callers; the compiler cannot be asked, see section 7):

- `registry_export_media_legacy` and `registry_import_media_legacy` resolve a workflow node through the typed composer registry (`dialects_for`, `resolve`, `IoKey`, `io_dispatch`) and are called last by `registry_export_media` / `registry_import_media`, after the `io_mechanism` route (`io_route` + `io_run`) returned `None`.
- That composer registry is live. `register_composer_entries` is called by the stdio artifact crates (for example `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🦀️.rs:227`) and by the plugin SDK macros (`🔌️plugin/🦀️.rs:1086,1131`).
- Artifacts registered only on that registry exist: `✏️s/🔌️plugins/📐️cad/.../🚪️io/🦀️.rs` defines `ComposerEntry` rows and **zero** `IoEntry` (checked with a recursive search of the cad plugin). For cad, the "legacy" function is the only registry route for export and import.
- `registry_shared_stdio_dialect` (workflow validation) uses the same registry (`dialects_for`), so deleting the two functions would also leave an inconsistent half.
- The only callers are the two entry points above; the test `workflow-unit` calls `registry_export_media`, not the legacy functions.

Decision: the name is misleading and the code is the current path for non-migrated artifacts. Renamed, all callers updated at once:

| Old | New |
|---|---|
| `registry_export_media_legacy` | `registry_export_media_via_composer_registry` |
| `registry_import_media_legacy` | `registry_import_media_via_composer_registry` |

Touched places: the two definitions, the two call sites, one doc comment in `🔀️workflow/🦀️.rs`, and the assertion message in `🧪️tests/🔬️workflow-unit/🦀️.rs` ("legacy/handler-map paths" became "composer-registry/handler-map paths"). The doc comments that say "W6 deletes this function" are still true and were left alone. Deleting them is correct only after every artifact has moved to `IoEntry` (cad is the visible blocker). The stringly `register_os_media_{export,import}_handler_kind` map that these functions fall through to is also called "legacy" in comments; it was not touched.

### 2.3 Wrong-direction items (recorded, not fixed)

- `seed_builtin_artifact_kinds` in `🗂️registry/🦀️.rs:102` hard-wires `space`, `workflow`, `collection`, `parameter.value` and `data`. It depends on `crate::space`, `crate::workflow` and the artifact crates `semio_framework_artifact_workflow_workflow` / `semio_framework_artifact_space_space`. Doc comments in `🔀️workflow` name plugin kinds `cad`, `puzzle3d`, `gismap`.
- `🖼️media-export-raster/🦀️.rs:18` (`rasterize_svg_to_png_base64`, `svg_to_polylines`) is a native SVG rasterizer. `resvg`, `tiny-skia`, `usvg` are native dependencies of `semio-framework-os`. It belongs in a renderer-side or media plugin.
- Product defaults: `SPACE_FOLDER_DOCUMENT_ID = "studio"` (`🗃️document-backbone/🦀️.rs:21`) and `OS_SPACE_BACKBONE_URI_PREFIX = "space://"` (`🔌️plugin-host/🦀️.rs:1054`).
- `registry_export_media_via_composer_registry` / `..._import_...` stay as the compatibility path described above; see 2.2.

## 3. Task D: MCP workspace

### 3.1 Where the plan changed from the seam map

- Cuts were located by content anchors, not line numbers (the map was off by one in several places: `app_command_seq_mut`, `shared_plugin_runtime`, the `HeadlessWorkspace` regions).
- **`dyn_enum_close!` is invoked three times**, not zero as map D.6 says: `ArtifactChannels` twice (`#[cfg(not(test))]` and `#[cfg(test)]`, in C07) and `GatewayBackends` (in C11). See 3.5 for the expansion hazard.
- **`impl crate::actions::HistoryUndoPort for HeadlessWorkspace` (5 lines, production code) was assigned to the parent by the map; it moved to the inference child.**
- The `//#region 🧪️Tests` wrapper stays in the parent around the two test mounts. The production inference code that sat inside it lives in the inference child under its own `//#region 💡️Inference`.
- `use semio_framework_os_kernel::os_directory::DocumentScope;` (at the head of the inference section) is used by `HeadlessWorkspace` methods in two children, so it moved into the parent import block (a moved line, not a new one).
- The anonymous trait imports `GuestRuntime as _` and `SpaceMember as _` can no longer live in the parent (they would be unused there and invisible to children). See 3.5.

### 3.2 Layout

- Parent (161 lines): module doc, the `workspace_identity` test mount, `is_repository_root`, the import block, `pub mod remote` and its `use`, `now_ms`, `workspace_worker_pool`, wiring, and the two test mounts (`quick`, `long`, paths unchanged). Original region markers `ArtifactChannel`, `HeadlessWorkspace`, `Tests` stay balanced here.
- Wiring per child: `[cfg] #[path = "<folder>/🦀️.rs"] mod <snake>;`, explicit `pub use <mod>::{...};` for the old public API, `pub(crate) use` only for `pub(crate)` names that other files in the crate use (searched over the whole `🌉️mcp` tree because the crate root re-exports `crate::workspace::*`), and one `#[cfg(test)] use <mod>::{names};` per child for the private names the `quick` and `long` tests reference. A child whose items are all native-gated has its `mod` line gated the same way. There are no glob re-exports.
- Every child imports what it uses as `use super::{names};` (names that live in the parent: shared `use` items, `now_ms`, `remote`, ...) and `use super::<sibling_mod>::{names};` (names defined by a sibling). Imports carry the `cfg` of the definition plus the common `cfg` of every use site (item attributes, impl-member attributes and block-level `#[cfg]` are all read), so nothing is imported where it would be unused or missing.
- Mounts inside a moved chunk gained `../`: `probe_json` (`../🚪️io/🔤️json/🦀️.rs`), `probe_sqlite` (`../🪶️sqlite/🦀️.rs`) and `guest_sqlite_original` (`../🪶️sqlite/🧪️tests/guest-input-slot.rs`). `remote`, `workspace_identity`, `quick`, `long` are unchanged. All 19 `#[path]`, `include!` and `include_str!` literals resolve on disk (0 missing).
- Test-module paths of the moved probe/guest mounts move one level (`workspace::probe_document::probe_sqlite::...`). Test function names are unchanged and are what the repo selects (`probe_sqlite_original_*`).

### 3.3 Visibility

Rule used: anything referenced from another chunk (code, the `quick`/`long` tests, or the mounted `guest-input-slot` test) is widened to `pub(super)`, the narrowest visibility that reaches the whole `workspace` subtree. It is not widened to `pub(crate)`. Items already `pub` or `pub(crate)` are unchanged.

Counts (name-based over-approximation, no compiler available): 42 `fn`, 6 `struct`, 3 `enum`, 2 `const` at top level; 41 inherent methods and associated consts; 64 named struct fields; 2 tuple-struct fields (`ProbeSnapshot`, `ProbeDiff`). The 38 child-defined names of map Appendix A (its other 3 live in the parent and need no widening) are all `pub(super)` or already `pub` in the output; the extras come from the two test files, which the map did not count. Private types named in the signature of a widened item or field were widened too (`CompileFlightState`, `PluginInferenceRoute`, `HubRelayState`), so no `private_interfaces` warning is introduced (`privcheck.py`: 0).

### 3.4 Probe fixture: not moved

Map D.7 and the task say the probe document is a fixture living in production code. That is not true of the current code:

- `HeadlessWorkspace.open_probes: Mutex<HashMap<String, ProbeStore>>` is a production field, drained in `Drop`, read by `workspace_artifact_ids`, `read_artifact_bytes`, `untrusted_artifact_provenance` and `read_artifact_resource` (its `schema` and `history` lanes).
- `🗿️artifact/🦀️.rs:287,289,343,349,385` creates, recognises and lists the probe kind (`artifact_create` with `kind == PROBE_SCHEMA`, `ensure_probe_artifact`, `authenticated_probe_document_is_known`).
- `💡️inference/🦀️.rs:28,198` discovers it (`PROBE_SCHEMA`).
- `WorkspaceOrigin::persistence_binding` mints the hub surface with `PROBE_SURFACE_ID`.
- `🌎️hub/📦️packages/🦀️rust/📜️script.ts:662,678` names `os.agent.probe/v1` as a hub test schema, and the credential-source-order analyzer pins `PROBE_SURFACE_ID`, `PROBE_PACK_SCHEMA_HASH`, `authenticated_probe_document_is_known` and `probe_record_spec` in the workspace source.

So `ensure_probe_codec_registered` is part of how the gateway opens its generic `os.agent.probe/v1` document in production. Putting the 290 lines under `cfg(test)` would delete that feature from `artifact_create`, inference discovery and the resource lanes; putting them under `🧪️tests/🧫️fixtures` while still compiling them in production would hide production code from the test-owned-path gates (oracle purity, dependency classification skip `🧪️tests`). Neither was done. What was done: the probe is isolated in `🔬️probe-document` (210 active lines) with its two codec mounts, so removing it later touches one child plus the places listed above. Decision needed from the owner: delete the production probe kind (and rewrite the 26 `quick`-test uses and the hub script pin), or keep it and rename it to what it is (a built-in scratch document kind).

### 3.5 Hazards the oracle cannot see, and how they were handled

The oracle compares resolution-level errors. It cannot see type-check-level problems or macro expansions (the proc-macro and external crates are absent). These were checked by reading the code:

- **Trait method scope.** Anonymous or named trait imports in the parent made trait methods callable everywhere. Real method calls were searched with each trait's method set and the receivers were read. `GuestRuntime as _` (native-gated, `use semio_framework_plugin_host::GuestRuntime as _;`) is now imported in C03, C04, C05, C11 (calls `instantiate`, `execute_turn`, `compile`, `drop_instance` on `GuestRuntimes`). `SpaceMember as _` is imported in C02 (`close_owned_step`, `close_owned_terminal_is_empty`) and C11 (`undo`, `redo`). `use super::ArtifactChannel as _;` is added to C11 (`exchange`, `activate`); `GatewayBackend`, `ArtifactChannel` are named in the children that implement them. The `codec_*` calls in C08 are inherent methods of the guest runtime, so they need no import. Doc comments of the two moved trait imports travel with the first child that uses them (C05, C11).
- **`dyn_enum_close!`.** The expansion emits a bare `__semio_dispatch_<Trait>!` invocation resolved by `macro_rules` textual scope (the crate root declares `#[macro_use] pub mod actions;` and `#[macro_use]` on the protocol module before `pub mod workspace;`), which children inherit. It also copies the trait's method signatures as literal tokens, so every type in `ArtifactChannel` (`AppCommand`, `AppFrame`, `Fault`, `ActivationScope`) and `GatewayBackend` (`ContextSummary`, `GatewayError`, `SearchHit`, `PreparedActionReport`, `RevisionStamp`, `InvocationReport`, `ResourceContent`, `Resource`) must be in scope in C07 and C11. Both traits have no provided methods and C07/C11 contain `impl`s naming exactly those types, which the import analysis picked up (verified in the generated import lists).
- **`file!`, `module_path!`, `type_name!`, `include*!`** do not occur in D; the host has only the 9 `include*!` (all rewritten).
- Derives are std prelude or fully qualified paths, so moving code between modules does not change derive resolution.
- Field and method privacy (E0616/E0624) is not reported by the oracle. `defscan.py` re-scans the written files: every non-`pub` item, inherent method or named field that is mentioned in another written file (or the `quick`, `long`, `guest-input-slot` tests) is flagged. Result: 103 definitions checked, 1 hit, and it is a false positive (`fn compiled_components` in C04 has the same identifier as the parent's `mod compiled_components;`; they are in different namespaces and the fn is not referenced from outside).

## 4. Readers, policy and registries

| File | Change | Check |
|---|---|---|
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🟦️.ts` | `POLICY_SPLIT_SOURCE_ROOTS` gained the C parent (`💻️os/🖥️host/🦀️.rs`) and the D parent (`🌉️mcp/🏠️workspace/🦀️.rs`) | `bun test .../🔬️rust-policy-source-evidence/🟦️.ts`: 6 pass, 0 fail (includes "every declared split source root names a real file that reads as one text") |
| `📜️script.ts` (root) | allowlist entry for `block_on` re-keyed from `🖥️host/🦀️.rs` (`lineHint` 1985) to `🖥️host/🔌️plugin-host/🦀️.rs` (`lineHint` 1359); `mcpWorkspace` now `policyReadModuleTreeSource` | see below |
| `🌎️hub/🧩️compositions/🗄️stdio/📦️packages/🦀️rust/📜️script.ts:850` | module assertion `^pub mod X \{` became `^pub mod X;` | regex check below |
| `🌎️hub/🧩️compositions/🪐️space/📦️packages/🦀️rust/📜️script.ts:319` | `osHost` reads `readModuleTreeSource(host)` plus the `workflow-unit` test file | check below |
| `🌎️hub/🔐️auth/🧪️tests/🧭️credential-source-order/🟦️.ts:208` | `workspace` read through `readModuleTreeSource` | check below |
| `🌎️hub/🧪️tests/🧱️foundation-source/🟦️.ts:398` | same | `bun test .../🟦️.ts -t "source definition boundaries"`: 1 pass |
| `🧰️framework/🔨️modules/🚪️io/⏱️control/🛫️snapshot/🧪️tests/🟦️.ts:85` | probe slice read through `sliceModuleTreeSource` | check below |
| `🌎️hub/📦️packages/🦀️rust/📋️project.json:65` and `🌎️hub/🧪️tests/🧫️fixtures/🧱️foundation-source/🔣️.json:371` | `🏠️workspace/🦀️.rs,🏠️workspace/🔗️remote/🦀️.rs` became `🏠️workspace/**/🦀️.rs` (a glob, so later splits do not drift) | the foundation-source test file runs; its 3 other failures are listed in section 7 |
| `🌎️hub/🧩️compositions/🗄️stdio/🔮️oracles/🔣️.json:75` and `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/.../🔮️oracles/🔣️.json:24,56` | `productionDebt.reachableFrom` now names `🖥️host/🖼️media-export-raster/🦀️.rs` (where the rasterizer is) instead of `🖥️host/🦀️.rs` | JSON valid |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` | 8 names added to `members-of-members-of-operating-system.memberNames` and 12 to `members-of-members-of-members-of-modules.memberNames` | `verify taxonomy report`, section 5 |
| docs naming moved functions | `🖨️raster/.../🚪️io/🦀️.rs:16,913`, `🪐️space/⚙️engine/.../🦀️.rs:23`, `🏪️store/✉️envelope/🦀️.rs:16`, `🪐️space/🫀️core/🦀️.rs:121` now name the child file | comment-only |

Reader evidence (`hostmcp-tools/readers-check.ts`, run with bun):

- The MCP credential-source analyzer: old monolith text conforms (true); the parent file alone, as the old reader would now read it, fails (false); the module tree conforms (true). Module tree of D is 566,357 bytes because it includes the `remote` subtree; the added text only adds content, and the analyzer's substring and ordering checks all sit inside single functions.
- Snapshot reader: `indexOf("impl store::ArtifactPackReceiving for ProbeSnapshot")` in the parent alone is -1 (the old reader would slice an empty string); `sliceModuleTreeSource` returns a 456-byte slice that is byte-identical to the slice of the old monolith.
- `toolJobPagedIngressExact` (root script) uses only `includes` on the MCP text: all 8 substrings are present in the new tree exactly as in the old file.
- Stdio reader: new regex true and old regex false for all five modules (`host`, `backbone`, `instance`, `workflow`, `registry`).
- Space reader: `fn svg_path_extraction_preserves_transformed_geometry()` is found in tree + test text (false in the parent alone).
- Parse check (`Bun.Transpiler`) of all 7 edited TS files: ok. Snapshot test file: `bun test` 6 pass, 1 fail; the failure is `copied_bytes:std::mem::size_of::<Option<O>>()` at line 71, which reads the `🛫️snapshot` crate, not D.

Left unchanged on purpose: `🌎️hub/🧩️compositions/🗄️stdio/🧪️tests/🧫️fixtures/🖊️dwg-artifact-ownership/🔣️.json:12` (a path-predicate case, no file is read); the `🌊️flow` crate's own `🖥️host` (`#[path = "../../🖥️host/🦀️.rs"]` in `🌊️flow/📦️packages/🦀️rust/🦀️.rs:43`) is a different file; the distribution manifest lists only A and B.

## 5. Verification run (actual results)

Tooling is in `hostmcp-tools/` (inputs: `orig-host.rs.txt`, `orig-workspace.rs.txt`, the splitters `split_host.py`, `wsgen.py`, `wsan.py`, `wscommon.py`, lexers, `sbx.py`, `oracle.py`, `lossless.py`, `lossless2.py`, `verify_paths.py`, `defscan.py`, `privcheck.py`, `readers-check.ts`, `em.ts`). Logs are in `🗑️generated/hostmcp-*.log`.

### 5.1 Full-resolution differential

Method: `oracle.py` builds two sandboxes that mirror the repo (real copies of `🖥️host` or `🌉️mcp`, symlinks elsewhere): the original snapshot and the live split tree. Both get the oracle's patches (missing `extern crate` lines removed, external glob imports replaced by `const _: () = ();`) and, for D, `dyn_enum_close!` is stubbed to a plain `enum` so the closed enums exist. `rustc +nightly-2026-07-20 --edition 2021 --crate-type lib --emit=metadata -o /dev/null --error-format=json`, run from the real lib root (`🖥️host/📦️packages/🦀️rust/🦀️.rs`, `🌉️mcp/📦️packages/🦀️rust/🦀️.rs`). Diagnostics are keyed by level, code, message, file (children mapped to the monolith), source text and label, with line numbers stripped.

| Crate | Config | Errors (orig / split) | Difference |
|---|---|---|---|
| host | native | 435 / 435 | none |
| host | `--test` | 828 / 828 | 1 message text (my assertion string change in `workflow-unit`, same error) |
| host | wasm32-unknown-unknown | 399 / 399 | none |
| host | wasm32-unknown-unknown `--test` | 788 / 788 | same single message-text difference |
| host | wasm32-wasip2 | 337 / 337 | none |
| host | wasm32-wasip2 `--test` | 682 / 682 | same |
| mcp | native | 2336 / 2336 | warning only (below) |
| mcp | `--test` | 3569 / 3569 | warning only |
| mcp | wasm32-unknown-unknown | 1717 / 1717 | warning only |
| mcp | wasm32-unknown-unknown `--test` | 2992 / 2992 | warning only |
| mcp | wasm32-wasip2 | 1717 / 1717 | warning only |

Zero new E0412, E0425, E0422, E0423, E0432, E0433, E0603, E0616, E0624 in any configuration. The single warning difference in all five D configurations is `unused import: untrusted_content`: the original reports it once (parent), the split twice (the two children that use it). It is an oracle artifact: both uses are inside `serde_json::json!{}` arguments (lines 5290 and 5498 of the original), which the oracle cannot expand because `serde_json` is missing. All other warnings (3 to 8 per configuration) are identical in both trees. In an earlier iteration the same checks caught a missing macro-site import, an unused `remote` import under wasm (block-level `#[cfg]`), unused globs, and a cfg(test) import list that ignored `cfg(not(wasm32))` definitions; each was fixed and re-run.

Negative controls (so the oracle is known to see these classes): removing `Fault` from C07's import list produced 21 new E0422/E0425 errors; making `app_frame_tag` private produced one E0603 (at the `use super::artifact_app_frames::{...}` import); making the host's `resolve_kernel_future` private produced 41 new E0603 errors (476 against 435); breaking one `include!` path aborted the run with "couldn't find file".

### 5.2 Other static checks

| Check | Result |
|---|---|
| Losslessness C (`lossless.py`, multiset of trimmed non-blank lines) | 4,294 original lines vs 4,295: only differences are the 8 module openers/closers, the 8 `#[path]`+`pub mod X;` pairs, the 2 renamed functions and their 2 call sites, the doc line that names a renamed function, the re-emoji'd `media_export_simple` doc line and the new `codec_abi` doc line |
| Losslessness D (`lossless2.py`) | 5,472 original lines vs 5,652: 182 added lines are wiring, imports, region markers and docstrings, 2 lines are not present any more; the only removed content is the `GuestRuntime as _,` entry in the parent import (moved to children) and a normalisation artifact for the untouched `workspace_identity` path line |
| Paths (`verify_paths.py`) | C: 17 literals, 0 missing; D: 19 literals, 0 missing; none exceeds 240 bytes |
| rustfmt parse (`rustfmt --edition 2021 --emit stdout < file`) | all 9 C files and all 13 D files exit 0 |
| Brace balance and `//#region` balance (per file) | balanced in all 22 files |
| Emoji | 20 new folders RGI (with FE0F), no duplicates among siblings |
| Active lines per child | max 988 (C), 839 (D) |
| Definition scan and private-in-public scan (D) | see 3.5; privcheck 0 |
| `verify taxonomy report --scope 🖥️host` (`hostmcp-taxonomy-host.log`) | `errors=78`; none concerns the 8 new folders or their files (the 78: `reference-syntax-unsupported` in other plugins, five `collision-*` between `📦️packages/🦀️rust/🦀️.rs` and `🦀️.rs` of the host, 12 `reference-edit-required` on the host's `pub use` lists and `Cargo.toml`/`📜️script.ts`, `directory-kind-unresolved` on `🧪️tests/🧫️fixtures/🧾️actor-context`); no baseline run, so these are attributed by path, not by diff |
| `verify taxonomy report --scope 🏠️workspace` (`hostmcp-taxonomy-ws.log`) | `errors=28`; no `directory-kind-unresolved` for the 12 new folders. The one finding in a moved file is `reference-edit-required` for the `#[path]` to `🪶️sqlite/🧪️tests/guest-input-slot.rs` in `🗂️guest-document-codec` (the same mount was flagged in the monolith because the taxonomy wants that test file renamed; the other 27 are in `🔗️remote` and `🪶️sqlite`) |

## 6. Deliberate behaviour and structure notes

- No runtime behaviour changed. Compile-time behaviour that changed: the two renamed functions (C) and `pub(super)` visibility (D). Neither alters public API.
- The parent of D is not "wiring only": it keeps the small shared helpers (`is_repository_root`, `now_ms`, `workspace_worker_pool`) and the import block, because children import them through `super`.
- `#[path]` modules declared inside `guest_document_codec` and `probe_document` point outside their own folder (`../🪶️sqlite/...`). The repo's module-tree reader skips mounts that leave the declaring directory, so `readModuleTreeSource(workspace)` does not pull `probe_json`/`probe_sqlite` into the workspace text, exactly like the old single file.
- A cfg-gated child (`plugin_artifact_channel`, `artifact_app_frames`) has its `mod` declaration gated with the common attribute of its items; `compiled_components` and `guest_document_codec` mix gated and ungated items and are always declared.

## 7. Not verified or not done

- **No `cargo check`.** `bun repo cargo --store=shared --configuration=prototype -- check -p semio-framework-os-mcp --message-format=short` fails before any compile at `cargo fetch --locked` ("cannot update the lock file ... --locked was passed"), caused by the shared `Cargo.lock` that peers keep changing, not by this work. It was run once for D and not retried for C because the compiler-ingress gate also refuses both crates (source-boundary limits, `📓️EXECUTOR-RULES.md`). No scratch workspace or private target dir was used. Consequently type-check-level correctness (method/field privacy, trait scope, the `dyn_enum_close!` expansion, unused warnings inside macro arguments) rests on the reading in 3.5, the definition scan and the widening rule, not on a compiler run. First real compile should expect, at worst, a missed `pub(super)` on a member whose name only occurs inside a macro token tree, a missed trait import on a method whose receiver type was not obvious from the text, or a few over-widened names (generic method names such as `new`, `get`, `record` were widened wherever the name appears in another chunk).
- **No Rust tests were run** (`cargo test` is not possible); only the TS tests listed in section 4 were run. In `foundation-source/🟦️.ts` four tests fail in this tree for causes outside this change: "Hub consumes retained OS document mounts" (marker not found in the hub bootstrap source), "package, target and input registrations bind only the moved owners" (`readHubDirectorySource` is exported but not in the fixture list), and two actor tests that need `SEMIO_TEST_ARTIFACT_DIR`. I did not run them before my edits, so this attribution is by the failing assertions, not by a before/after diff.
- `verify interactivity` could not run: it aborts earlier in an unrelated puzzle sub-command ("Canonical native dependency owner required: `◻️2d/⚙️engine`"). The only allowlist entry for the host was re-keyed by reading the code (the `block_on` calls are in the `host-unit` test file, `include!`d at line 1359 of the plugin-host child); the scanner skips `🧪️tests` and the entry is `expectedNeverToMatch`.
- `verify dependencies literal-external`, `repo` dependency-direction and dependency-cruiser gates were not run; no dependency or manifest edge changed.
- The probe document is still compiled in production (3.4). The two other items from D.7 (plugin names in doc comments; `HeadlessWorkspace` implementing both the gateway and the inference client) are unchanged except that the inference facade now has its own child.
- Region markers: `//#region ArtifactChannel` was split across children; each child that holds only a slice of it carries a synthetic `//#region`/`//#endregion` pair so every file stays balanced (these synthetic lines are counted in the 182 added lines).
- No ticket was opened or closed (subagent rule); no `🗑️generated` folder was touched except my own `hostmcp-*.log` files there.

## 8. Files

Created (20 folders, one `🦀️.rs` each): under `🧰️framework/🛍️products/💻️os/🖥️host/`: `🔌️plugin-host`, `🗃️document-backbone`, `🧱️instance`, `🖼️media-export-raster`, `🧾️media-export-simple`, `🔀️workflow`, `📡️codec-abi`, `🗂️registry`; under `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/`: `🧭️plugin-paths-and-descriptor`, `🔬️probe-document`, `🔖️plugin-binding-and-activation`, `🧊️compiled-components`, `📨️plugin-artifact-channel`, `🧩️artifact-app-frames`, `🔀️routing-and-shell-routed`, `🗂️guest-document-codec`, `🤝️hub-relay`, `🪪️headless-workspace-state`, `🛂️gateway-backend`, `💡️inference-facade`.

Edited: `🖥️host/🦀️.rs`, `🖥️host/🧪️tests/🔬️workflow-unit/🦀️.rs`, `🏠️workspace/🦀️.rs`, plus every file in the first column of section 4 and the four doc-comment files named there.

Tooling and records: `hostmcp-tools/`, `🗑️generated/hostmcp-taxonomy-host.log`, `🗑️generated/hostmcp-taxonomy-ws.log`, `🗑️generated/hostmcp-interactivity.log` (failed run, kept as evidence).
