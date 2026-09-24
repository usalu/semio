# WP-R1 — Native Test Green: OS Frontend + Plugins

Slice: R1 (session 10). Captures: `.tmp-ticket/wp-r1/generated/`. Private cargo: `.tmp-ticket/wp-r1/target`.
Inherits: O1d, O1b, O1c, P1, P3, P2b, audits plugins-a/b, os-frontend.

## Status

| Item | State | Evidence |
|------|-------|----------|
| framework-renderer-wgpu:test 0 fail | PASS — cargo 1372/1372, vitest 393/393 (38 files), EXIT 0 | `wp-r1/generated/wgpu-nx-test-3.txt` |
| space native suite | 87/88; only `descriptor_is_fresh` left (waiting on w1) | `wp-r1/generated/space-nextest-9.txt` |
| flow close-step stall (1-byte grant) | PASS — flow 4/4 (incl. descriptor_is_fresh) | `wp-r1/generated/flow-nextest-3.txt`, `fw-kernel-space-flow-nextest-4.txt` |
| extensions cad×4 / imperative×5 / playbook×1 / sourcing×3 (native) | PASS — 50/50 after playbook-procedural rework | `wp-r1/generated/extensions-nextest-1.txt`, `playbook-proc-3.txt` |
| stdio IFC/STEP/PNG third-party oracle parity (test platform) | PASS long: PNG 4/4 (png 0.18), STEP 7/7 (ruststep 0.4), IFC 7/7 (IfcOpenShell 0.8.4 differential + ruststep). Exhaustive: PNG 68/68 (parity 34/34), STEP 194/194 (parity 97/97), IFC 196/196 (parity 98/98; 2x3 base rerun 38/38 after control-row registration) | `wp-r1/generated/stdio-*-parity-long-*.txt` |
| every plugin test-quick semantics, native (`nextest --profile quick -- --skip long:: --skip exhaustive::`, 34 plugin crates) | 301 laws: every non-descriptor law PASS. Only `descriptor_is_fresh` fails (sourcing, lowpoly, space, procedural, demonstrator, stdio), waiting on w1's describe chain. norm's window-roster law was fixed | `wp-r1/generated/all-plugins-nextest-1.txt`, `norm-law.txt` |
| stdio artifact crates png/step/ifc/semio (native) | 3006/3011, then all fixed or jitter: IFC 2x3 exact_native 5/5 after canonical-layout fixture; semio fillet 200 ms budget law failed at 359 ms under load avg ~30 and passes isolated | `stdio-artifacts-nextest.txt`, `ifc-exact.txt`, `semio-fillet-rerun.txt` |
| framework-os / os-mcp / host-rs vitest / os-dev | PASS 373/373, 52/52, 373/373, 163 (+28 skipped) at 01:0x | `wp-r1/generated/framework-os-test.txt`, `os-mcp-test.txt`, `host-rs-vitest.txt`, `os-dev-test.txt` |
| plugin-registry (vitest run directly) | R1: 58/61 with 2 stale `🚀️launch` laws. W1 fixed them: 60/61, 1 skip (W1-measured) | `wp-r1/generated/plugin-registry-test.txt`, `wp-w1/generated/registry-test.txt` |

## Fixes

### renderer-wgpu (O1d continuation)

- Root cause of the O1d "84–89 failures": serial `cargo test --test-threads=1` runs all 1372 laws in ONE process, so process-global retained state (resident roots 64/64, sealed input candidates) leaks between laws. The official runner is nextest (one process per law): `cargo nextest run -p semio-framework-os-renderer-wgpu` = 1372/1372 (`wgpu-nextest-1.txt`). Presented-input "could not be sealed" was the same cascade.
- nx `test` was killed by the fundamental 15 s budget (suite ~65–105 s): `TestScript`/`NativeTestScript` now floor at `long`, like `WgpuUnitTestScript` (`wgpu-nx-test-1.txt` → `-2`).
- 12 vitest source-contract failures, each against a deliberate HEAD change:
  - `🧩️wgpu-module-routes` fixture: the `🪞️vendor` fontRoot mount was removed in HEAD (fonts come from `moduleRoot/🪞️vendor`); fixture row dropped.
  - `🖼️wgpu-raster-witness`: `reserve_engine_texture` takes the raster content `identity`.
  - `🖌️wgpu-document-owner-move`: the frame build no longer runs a caller-side drive loop (HEAD 643: one retained Worker turn per callback). The law now checks the present loop budget in winit, `try_step_on_worker`, and no `drive_deadline_us`. The terminal-cursor law names the six chrome walks and checks that every release site sits in one of them. The orphaned `BROWSER_FRAME_BUILD_DRIVE_US` docstring on `batch_params` is replaced.
  - `🧩️package-integration` (2 laws): the new `📡️replication/👕️peer-overlay/🟦️.ts` (C3) wasn't in the wgpu browser profile's `sourceModulePaths` or `inputPatterns` in `🔣️taxonomy.json`; added to both.
  - `🔗️hub-projection`: `SHELL_HUB_ROUTE` constant and `status.remote`.
  - `🌳️tree-row-rects`: customizable `treeRowShellClassName` (CSS vars defaulting to `--size-workbench`), `live_tree_item_height`. The "real React capture" law read a transient ticket `🗑️generated/tree-style.json` that no longer exists and has no producer; it now pins the density fixture to React's own `domSizePx("treeRowUiSpacing")`. The live-capture oracle is listed as a gap. `[DEBUG]` logs removed from that test.

### space (P3 continuation): 88 laws, 13 failing → 87/88

- **spawnApp publication stall** (`interactive-job.publication-stalled` after 4096 units, stage Publishing, pending Config lane). The config preparation always answered `Blocked`: commit 590 raised `SPACE_CONFIG_MAXIMUM_BYTES` to 65 536 and priced every config turn at `4×max+1024 = 263 168` bytes, but the host's typed-operation grant is a fixed 4 KiB page. Fix: `space_config_candidate_bytes(base, mutation)` prices the candidate by its real footprint (post + inverse + forward), and a unit law pins one real session turn at ≤ 4 KiB. The envelope limits stay as they were.
- **set_active_example** ArtifactEnvelope Drop: the test's parsed pack dropped its envelope. It now uses `ParsedDocumentText::into_snapshot`.
- **set_app_registrations**: `engine::apply_app_registrations` was a no-op stub ("AppDefinition has no FromValue"; it has one now). It now decodes `[{pluginId, app}]` through `pack::from_json_str` into an `AppRegistrationRow` and calls `register_app_io`. A malformed roster returns the fault `s.space.app-registrations-malformed`.
- **deleteSelection** was `BatchOnlyPendingRewrite`, so the UI refused it (`interactive-job.not-ui-safe`). It is now Migrated: tool ids, publication contract Artifact+Config, proofs, manifest, studio retained fixture, plugin `🧬️schema` contract tuple (16), and catalog counts 16/24. The retained reducer reads the live `graph` selection the same way `openInstance` does.
- **Window action scoping** ("Media VFS must NOT expose spawnApp"): P3's staged AppBuilder loop fanned every app action, including ones another window explicitly owns, onto every window kind. That contradicted `window_kind_actions` (manifest), which `AppActionRegistry::from_definition` already uses. Loop removed and registry doc corrected. The catalog test helper now reads `definition.actions` + window rosters.
- **Close faults** "artifact store has no owner-supplied bounded disposer": TC4 made store *disposers* framework-default, but the *owner catalogs* those disposers drive stayed `None`, so every app without explicit owners faulted on close (space draft store, home/index). Fix: `ArtifactApp` and `ArtifactEditor` default `build_{document,config,draft}_store_owners` to the bounded catalogs.
- Tests that dropped live apps now settle retained ops (`context::dispatch_settled`, `settle_framework_reserved_admission` for commit/checkout checkpoint), retire parsed envelopes, and call `close_registered_fixture_app`.
- Demo example edge `edge-draw-1-to-draw-2`: its stored contract `wire_schema=document` no longer matches the negotiated `Document{schema:"artifact"}`. Example updated, and the validation assert now prints its errors.
- Workflow scene law: the component doc is now binary (`doc.bytes`). The law decodes the `NodeGraphScene` (`capabilities_json` engine flow, `host_snapshot_json` schema).
- Home retained catalog: the fixture and positional schema were stale for S4 (`applyDirectoryEventPage`/`createStudio` Migrated) and C6 (`promoteToHubSpace` HostOnly, `persistLocally` Artifact). Now 18 routes and 13 proofs; the TS oracle in `📜️script.ts` is updated.

### flow close stall (P1 continuation)

- `flow_actual_surface_factories_close_all_owners_under_neutral_grants` @ bytes=1: 96 k idle turns. I instrumented it (instrumentation since removed): `CompositionGraph::close_step` and `AppActionRegistry::close_step` both *refused* any identifier longer than the grant (`Pending{0,0}` forever). That contradicts the plugin's own page-never-refuse law (`close_retained_string_page`). Both now detach one row whole (`Pending{1,0}`) into a `retiring: Vec<Vec<u8>>` and page identifiers by at most the grant per turn, popping on the turn that empties them. New law `composition_graph_retires_under_a_one_byte_grant` (kernel). `retained_field_maximum_and_maximum_plus_one_are_language_neutral` was rewritten from "max+1 refuses" to "max+1 pages" (the refusal was the livelock).
- Framework plugin crate 833/833 (`plugin-fw-space-nextest-3.txt` + `fw-kernel-space-flow-nextest-4.txt`). The overlay-append 2 ms timing law failed once under load avg 41 and passed on rerun. `two_hundred_publications_…` compared against a baseline taken before the reconcile runtime backing registers lazily on the first reservation. It now subtracts `fixed_backing_bytes()` on both sides.
- Peer compile breaks fixed in plugin tests: missing `published_window_config` (4 initializers) and a duplicated `active_tool` field.

### extensions

- 13 crates natively: 41/50, then 50/50. `semio-s-plugin-playbook-procedural` tests predated retained ops: the app is now bound to the fixture instance, migrated actions settle (`act`) and undo goes through `settle_history_verb`. Apps close through `close_registered_fixture_app`. The standalone store installs owners and closes. Action lookup covers `definition.actions` (after the fan-out removal). Params-body laws read `component.type`/`component.role` (projection shape) instead of the stale `stack`/top-level `type`.

### stdio oracle parity (repository test platform, `bun ./📜️script.ts parity <level> --owner …`)

- Commit 599 split stdio into per-artifact crates and rewrote every `semio_s_plugin_stdio::artifacts::<x>::` to `crate::`, including the test-platform adapters that the generated host compiles as a *bin* linking the subject crate by name. None of those adapters compiled. For png/step/ifc (16 case adapters), `crate::` is now the subject crate name.
- Explicit re-exports where the subject's public API needs them (AGENTS.md re-export rule): png root `pub use semio_framework_os_kernel::ArtifactDsl`, ifc `engine::part21` (from step). STEP base (10 leaves) and IFC (30 leaves / 47 fields) mutation payload fields changed `pub(crate)` → `pub`: they are the public mutation constructors.
- Host materialization: two ancestors contributing the same oracle crate produced a duplicate Cargo key (cc6). Contributions now merge features, and a divergent path is reported as a problem.
- The feature ids for the control row are `no-mutation-baseline-mutate`/`-inverse` (the `✉️mutate-semio-base` convention). The STEP adapters registered `mutate-no-mutation`, so 28 exhaustive scenarios errored with "no registration". They now use `scenario_id(kind, verb)`, and the cc inverse skips the inverse walk for the control row.
- `law::scenario_id(kind, verb)` now lives in the stdio oracle crate (`🔮️oracles/⚖️law`), and every STEP/IFC mutate adapter plus `✉️mutate-semio-base` registers through it (one definition instead of a copy per adapter). IFC 2x3 base now also registers the `no-mutation` control row its feature declares.
- Subjects rejected the identity baseline `no-mutation`: STEP `apply_and_encode` re-encodes without a mutation, and the IFC subsets map it to `SetSnapshot(base)` exactly as `🧱️mutate-ifc-2x3` does.

### plugin crates (native quick)

- norm `every_norm_editor_action_is_migrated_onto_the_shared_owned_factory` read `window.actions` directly (it depended on P3's fan-out). It now resolves each window through `semio_framework_plugin::window_kind_actions`, the roster a window actually presents. This was P3's original norm symptom, fixed at the reader instead of by duplicating actions into every window.
- IFC 2x3 base `exact_native_*` (4 laws): the original 409k-entity EDM fixture `temp/wellness-center-sama.ifc` is gone. P2b's street-level substitute is another tool's export, and git's `* text=auto eol=lf` LF-normalizes it (`.ifc` is missing from the `.gitattributes` binary list documented for exactly this hazard). So no writer can reproduce its bytes (193 915 vs 196 428, first difference at byte 13, where the writer emits CRLF). Exactness is now proven against the codec's canonical layout, which is asserted to be a fixed point of decode→encode. Recommend a repo-tooling follow-up: add `*.ifc`/`*.stp` to the binary list.

## Blockers

- **Descriptor freshness** (w1): 6 plugins (`sourcing`, `lowpoly`, `space`, `procedural`, `demonstrator`, `stdio`) fail only `descriptor_is_fresh`. W1's describe chain died at ~01:08 (all its detached processes vanished at once, load avg 276). My framework edits are announced to w1 and settled: the fan-out removal changes every descriptor. Requests are in `wp-w1/requests/r1.txt`.

- `✉️mutate-semio-base` (semio artifact, outside the PNG/STEP/IFC scope, touched for `scenario_id`): it didn't compile before (the same `crate::` drift, a folded `::mutation::` facet, `pub(crate)` payload fields). It now compiles and runs 36/43. The 7 subject failures are fixture-plan drift: the adapter reads `asset://🏅️standards/…/📸️set-snapshot/🧪️tests/✉️replaces-…/📸️snapshot/⬅️before/🔣️.json`, but the feature declares `shared://🧬️mutations/📸️set-snapshot/✉️replaces-…`. Owner decision needed on the fixture's canonical location. `stdio-semiobase-parity-exhaustive-4.txt`.
- Kernel `document_codec_of_round_trips_dsl_and_pack_and_edit_text` expects one header line + one op line, but `print_edit_lines` now also emits inverse+metadata (C2b change). Reported to h2; not mine. The two sync laws were fixed by g4 (61/61).

## Processes (pids)

None left running. The detached renderer/nextest/parity runs were started by R1 and have exited. One detached exhaustive parity chain died with the ~01:08 machine-wide process loss and was rerun in the foreground.

## Files changed (R1)

- framework: `🔌️plugin/🦀️.rs` (fan-out removed, owner defaults, registry close paging, doc), `🏪️store/🦀️.rs` (CompositionGraph paging), `🏪️store/🧪️tests/🔬️unit/🦀️.rs` (one-byte law), plugin tests (`plugin-runtime-plugin-builder-contract`, `app-typed-command-full-operation`), `⚛️reactor/🩹️patches` test.
- renderer: wgpu `📜️script.ts`, `🧵️frame-job/🦀️.rs` (doc), tests + fixtures `🧩️wgpu-module-routes`, `🖼️wgpu-raster-witness`, `🖌️wgpu-document-owner-move`, `🔗️hub-projection`, `🌳️tree-row-rects`. Also `📚️library/🔣️taxonomy.json` (peer-overlay module).
- space: engine `🦀️.rs` (config pricing, deleteSelection migration, retained selection), `⚙️engine/🦀️.rs` (apply_app_registrations), commands `📇️set-app-registrations`, `🗑️delete-selection`, tests (engine unit, set-active-example, spawn-app, delete-selection, workflow, compiled-dag, set-active-panel-tab, interactive-job-catalog), `🧬️schema/🔣️.json`, studio + home retained fixtures, home `🧬️schema/🔣️.json`, space `📜️script.ts`, os `🪐️space/📚️examples/♻️reuse/…/🧬️.semio`.
- norm test `🖥️app-surface`. playbook-procedural test.
- stdio: png/ifc/step/semio artifact roots and mutation leaves, 16 + 1 test-platform adapters, oracle `⚖️law/🦀️.rs`, IFC 2x3 mutations unit test. Test platform `🖥️host/🏗️materialization/🟦️.ts`.
