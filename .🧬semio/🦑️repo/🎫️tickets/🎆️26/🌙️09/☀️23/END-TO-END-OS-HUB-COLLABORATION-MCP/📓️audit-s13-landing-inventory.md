# Session 13 Landing Inventory — Everything Prepared But Not Landed From Sessions 11–12

Auditor A13-land, read-only, foreground. Snapshot taken 2026-09-26 ~19:20 (session 13 started ~19:0x; LA/LB/LC/LD/H11/T13/C11/S16/DB1
had each logged only their first "read the handovers" entry, all rows "pending"/"in progress" with no evidence, when this
snapshot was taken — but §3 below shows several sets are **already applied to the uncommitted working tree**, ahead of what the
agents' own report tables say). Sources read: `📓️session-13-preamble.md`, `📓️fleet-13-agents.md`, `📓️work-packages.md` (§ "Post-
`--packages all` landing window"), `📓️landing.md` (only a Session-11 table + a just-opened, still-empty "# Session 13 Landing
Window" header — no session-12 rows were ever added), all `📓️wp-*.md` reports (r8, d1, h10, p8, f1, u5, wg7, wg8, h9, c10, g10,
s15, t12, w2, plus the started session-13 la/lb/lc/ld-absent/h11/t13/c11/s16/db1), `wp-w1/requests/*.txt`, `wp-w2/requests/wg7.txt`,
and the live `git diff` against HEAD (201 non-ticket files touched, uncommitted).

## 0. How to read this

Each item: **source** (slice + report row) → **command** → **dry-run state** → **targets** (crates; Guest-linked Y/N; touches
kernel-derive inputs `taxonomy.json`/`nx.json`/`project.json`/root `Cargo.toml`/`Cargo.lock`/`.cargo/config.toml` Y/N) →
**dependencies** → **session-13 slice** → **live tree state** (confirmed via `git diff` symbol/file match, this snapshot).

---

## 1. Framework / host-core / UI (→ slice **LA**)

| # | Source | Command | Dry run | Targets | Guest-linked | Kernel-derive input | Deps | Live tree state |
|---|---|---|---|---|---|---|---|---|
| LA-1 | R8, `📓️wp-r8.md` row S12-7 (§S12-7, l.202) | `wp-r8/nx-narrowed-inputs.py` | clean, re-verified | `nx.json` global `production` (inlines `default`, drops the lone `!{workspaceRoot}/**/🧫️fixtures/**/*` negation), every plugin-generated `production` in `🟨️.mjs`, component `describe` inputs → native closure, `🕸️graph` generator inputs | N (build-graph config, not compiled) | **Y — `nx.json`** (named explicitly as frozen once W3 announces REBUILD START, preamble rule 4) | needs a fresh Nx re-plan + the cache-input structural law + a new "no `!{workspaceRoot}` negation via reference" law after | LA | **CONFIRMED APPLIED**, uncommitted: `git diff -- nx.json` shows exactly this edit (`"production": ["{projectRoot}/**/*", "sharedGlobals", …]`, negation gone). Must land (verify + record in `📓️landing.md`) **before** W3's REBUILD START or it will be frozen mid-flight. |
| LA-2 | F1, `📓️wp-f1.md` row F1-5 (already landed, this row is the leftover) | os tsc error in `⌨️text-input-oracle` | n/a (bug, not a patch) | TS only, framework-os typecheck | N | N | none | not independently re-checked; F1's report marks the editor Rust+TS side "landed… + live PASS", so this may already be moot — LA should just re-run `tsc` |
| LA-3 | S15, `📓️wp-s15.md` row S12-3k | exact JSON hand-edit (not a script): add `"waiting"` to `$defs.ArtifactCreationProgressCopyV1.required` + `properties` in `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🔣️.json` | n/a, one hand-specified hunk | directory schema (TS + Rust twins regenerate) | N (host directory schema, not guest ABI) | N | none — was already landed once (session 12, commit before `f7791a96178`) and lost to a peer's overwrite | LA | **CONFIRMED NOT APPLIED** — direct read: `required` still lists only `heading, cancel, cancelling, opening, phases, catalog`; `waiting` is absent from both `required` and `properties`. Re-apply verbatim. |
| LA-4a | H10, `📓️wp-h10.md` row 1 / §Post-publish landing plan item 3 | `zsh .tmp-ticket/wp-h10/q1-land.sh` (self-reverting; dry run → apply → `cargo check -p semio-framework-plugin-host --tests` → interpreter laws → `--sweep` identity check → hub `artifact_authority::` laws) | clean, re-verified after the 14:59 desktop restart | `semio-framework-plugin-host` (interpreter) | N directly, but **changes `owned_engine_identity()`** → every hub re-verifies every guest once | N | run before LA-4c (codec-app-resolution) so the fuel/identity sweep compares apples to apples | LA | **NOT APPLIED** (symbol check: `owned_engine_identity` diff absent from working tree) |
| LA-4b | H10, row 2 / item 2 | `patches/q2-sha256-hardware.py --apply`; `cargo test -p semio-framework-hash` | clean | `semio-framework-hash` | **Y — every framework dependent recompiles** (every guest) | N | none, but recompiles all guests → sequence before the consolidated restage | LA | NOT APPLIED |
| LA-4c | H10, row 1 / item 4 | `patches/codec-app-resolution.py --apply`; `cargo test -p semio-framework-plugin --lib --codec_calls_construct app_declarations` + wasm32 check | clean (11 plugin hunks, 3 builder hunks, 1 law) | `semio-framework-plugin` + every plugin declaration (AppFactory/SurfaceDeclaration) | **Y — every guest** (no WIT/wire/descriptor change, but every component recompiles) | N | needs W2/W3's consolidated restage + republish to take effect in a hub | LA | NOT APPLIED (symbol check: `codec_calls_construct`/`artifact_codec_owner` absent) |
| LA-5 | R8, `📓️wp-r8.md` §S12-2 (l.112) | `wp-r8/ui-retirement-item-metered.py` (+ `-law.rs.txt`) | clean; measured ui-contract 197/197, ui-runtime 125/125, replication 292/292 in an isolated scratch workspace | `semio-framework-ui-contract` (`release_empty_page`, the byte-gating law renamed to `…meters_backing_as_items`) | **Y — every UI guest links ui-contract** | N | "lands in the window after `--packages all`" (R8's own words) — needs restage for guests | LA | NOT APPLIED (symbol check: `release_empty_page`/`meters_backing_as_items` absent from the diff; the ui-contract files currently touched in the tree are an unrelated Tree-inline-toolbar feature, not this patch) |
| LA-6 | R8, `wp-w1/requests/r8.txt` (23:3x) | `bun 🌊️flow/🫀️core/📦️packages/🦀️rust/📜️script.ts wasm` (regenerate stale flow_core wasm-pack bindings; source is 1 day older than the bindings) | n/a, a rebuild not a patch | `flow` plugin only | Y (flow guest) | N | none, safe "after `--packages all` publish is fine" per R8 | LA | not checked live; low risk, cheap |
| LA-7 | (suites) | plugin-host identity sweep, hash digest oracle, ui-contract/ui-runtime nextest, framework-os typecheck | — | — | — | — | runs after LA-4/LA-5 apply | LA | — |

## 2. Plugin open kinds, capability descriptions, oracles (→ slice **LB**)

| # | Source | Command | Dry run | Targets | Guest-linked | Kernel input | Deps | Live tree state |
|---|---|---|---|---|---|---|---|---|
| LB-1 | T12, `📓️wp-t12.md` §S12-1 (l.337–358) | `python3 .tmp-ticket/wp-t12/postpublish-open-kinds.py --write` (7 parts A–G; then `cargo check -p semio-s-plugin-stdio --features full-app-catalog --lib --tests`, `-p semio-s-plugin-gis`, `-p semio-hub --lib --tests --features native-artifact-execution` + wasm32 check → W2/W3 describe-all + generate → census law) | **clean, 79 files / 0 problems**, re-verified 04:4x | Part A: 35 `stdio.<x>`→`s.stdio.<x>` ids (35 `artifact_kind()` + 25 artifact-definition + 25 native-codec receipts + hub fixture/bin-unit + TS trusted-stdio-catalog). Part B: txt/tsv/html/csv/json×2/xml×2/md editor kind declarations. Part C: gis terrain kind. Part D: hub fence `local-stdio-gis-open-v1` (drops the hard-coded target list, validates every target by descriptor). Part E: census law. Part F: bootstrap `opensDocuments`. Part G: rotation fixup | **Y** — Part A/B/C are guest crates (stdio, gis); Part D/E/F are hub-native | N | needs W2/W3's describe-all + generate to regenerate `🔣️.json`/`🛂️.descriptor.semio` **before** the census law (Part E) can pass | LB | **CONFIRMED IN PROGRESS**: `git diff` shows Part A applied across 68 stdio files (`id: "s.stdio.csv".into()` etc.), `📇️registry/📜️native-codec-factories.json` (50 lines) applied, Part C applied (gis terrain `+22`, `gis/🦀️.rs +2`), **Part D applied** (hub `local-stdio-gis-open-v1` fence rewritten — error text now reads "closed two-package native-codec closure opening every package target", exact match). Parts B/E/F/G not individually verified but very likely mid-apply too. **Descriptors are NOT yet confirmed regenerated via a live `describe` run** — but the committed sidecar `✏️s/🔌️plugins/🗄️stdio/🔣️.json` already shows `"id": "s.stdio.csv@rfc4180/*#editor"` / `"artifactKind": "s.stdio.csv"`, so the descriptor text is at least as far along as the Rust source. Run describe-all regardless before trusting the census law (Part E). |
| LB-2a | T12 §3b (fem3d) | `wp-t12/fem3d-oracles/patch.py` | clean, 200/200 rows validated | fem3d plugin (Python oracle only) | N | N | none | not checked |
| LB-2b | T12 §3b (outline rows) | `wp-t12/outline-row-ids/patch.py` | clean, 9 files / 0 problems (incl. a platform parser change that must land with its 3 features) | energy/procedural platform + features | partially (platform parser is shared) | N | must land as one unit (platform + 3 features + 4 adapters together) | not checked |
| LB-2c | T12 §3b (example-geometry) | `wp-t12/example-geometry-paths.py` | clean, 2 files / 0 problems | generation3d | N | N | none | not checked |
| LB-2d | T12 §3b (energy) | `wp-t12/energy-reference-drift.py` | clean, 3 files (2 are Rust dead-code deletions → `cargo check -p` the energy crate after) | energy plugin (Rust + fixture) | Y (energy guest) | N | `cargo check -p` energy after apply | not checked |
| LB-2e | T12 §3b (kit pack) | `wp-t12/kit-tower-pack.py` | clean, 1 regenerated binary fixture | kit-tower fixture (writer byte-faithful regenerate) | test-fixture only | N | none | not checked |
| LB-2f | T12 §3b (brep) | `wp-t12/brep-reference-carrier.py` | clean, 4 files; then `test-parity --case 🧊️mutate-semio-brep` | brep reference/feature/DSL/pack | test-fixture only | N | none | not checked |
| LB-2g | T12 §3b (txt) | `wp-t12/txt-oracle-refusal.py` | clean, 1 Rust case adapter; then `test-parity --case 📝️mutate-txt-utf-8` | stdio-txt test adapter | N | N | none | not checked |
| LB-3 | D1, `📓️wp-d1.md` l.143–150 | `wp-d1/d1-frozen.py --apply` | clean, re-verified 00:44 + 00:53 | **gis** `create_gis2d_app` (10 descriptions + 8 Chrome audiences), gisterrain (2 + setCamera), **vcs** (5 + noMutation), **stdio** (`set_active_example_description()` + 62 `.action_describe` call sites) — all pure description text, **no ABI change** | **Y — gis/stdio/vcs are guest crates** (frozen specifically because of that) | N | then `cargo check -p` gis/stdio/vcs/demonstrator + describe those four; **also needs W2/W3's rebuilt `semio-os-mcp`** (filed as `wp-w1/requests/d1.txt`) before the live MCP shows any new description | LB | **NOT APPLIED** (vcs plugin untouched in the diff; gis changes present are T12's terrain kind, not D1's descriptions) — **collides on the same files as LB-1** (see §5 Conflicts) |
| LB-4 | (LB's own charter item, no single source report found) | cross-plugin verb-arg census → framework law + plugin fixes | — | Likely a broader version of T12's already-landed `note-verb-args.py` (session-12, applied 01:3x, 400/400 passed) generalized to every plugin; no dedicated script found under any `wp-*/` folder as of this snapshot | — | — | — | **written nowhere yet** — treat as design work, not a prepared patch |

## 3. Plugin handlers, editors, preferences, creation labels/progress (→ slice **LC**)

| # | Source | Command | Dry run | Targets | Guest-linked | Kernel input | Deps | Live tree state |
|---|---|---|---|---|---|---|---|---|
| LC-1 | P8, `📓️wp-p8.md` row 17 (l.34) | `python3 .tmp-ticket/wp-p8/p8-land.py --write --test` (order: `p8-agent-lane` → `p8-law` → `p8-flow` (23) → `p8-cad` (4) → `p8-space-studio` (8) → `p8-space-home` (3); each backed up + gated by `cargo check -p` of its own crates; a failing gate restores only that set) | dry run of all 6 default sets clean on the tree (all 6, re-verified after every session-12 restart) | `semio-framework` SDK, `🔌️plugin/🧵️retained-command`, `🎯️action-bus` (agent-lane); flow (23 files), cad (4), space-studio (8), space-home (3) | **Y** — SDK + flow/cad/space are guest crates | N | `p8-agent-lane` first (moves the architect law's agent pin: `exportProgram`/`exportRegistersCsv`/`importProgramRequest` refused by name) | LC | **CONFIRMED IN PROGRESS**: `git diff` shows `🔌️plugin/🧵️retained-command/🦀️.rs` gained `preview_emit` ("the agent lane's prepare phase…") and `🎯️action-bus/🦀️.rs` gained `ToolPayload::into_inner` ("the agent lane's prepare phase reads a retained route's work without dispatching it") — **`p8-agent-lane` is applied**. Flow/cad/space-studio/space-home not independently confirmed. |
| LC-1b | P8, row 15 (HELD, not default) | `p8-orphan.py` (`--only orphan`) | clean in the clone, but turns the reasoning law red until its class fix lands | reasoning + 11 other plugins that mint content-addressed children (dag, writer, sequence, animate, playbook, imperative, trinity jack, raster, note, cad, flow) | Y | N | **must land together with the content-addressed-child class fix** (flow's pattern or an SDK rule) — explicitly held back by P8 itself | LC (held) | not checked; do not land alone |
| LC-2 | F1, `📓️wp-f1.md` row F1-6 (l.20) + `wp-w1/requests/f1.txt` | `.tmp-ticket/wp-f1/patches/typing-coalescing/apply.py --dry-run` → (when opened) apply, `cargo test`, wasip2 checks | **clean** (5 files + 1 new fixture) | Component-changing: `semio-s-artifact-trinity-jack` (coalesce_key), `semio-s-artifact-vcs-vcs` (`Emit::amend`). Test-only, no component change: `semio-framework-plugin` (`typing_run()` + fixture), `semio-s-artifact-writer-writer` (law only) | **Y — jack + vcs restage needed**; writer/plugin are test-only | N | no ABI/WIT/pack/codec change — safe to land any time after the freeze lifts | LC | NOT APPLIED (jack/vcs unchanged in the diff) |
| LC-3 | U5, `📓️wp-u5.md` §12-4 (l.330–357) | `.tmp-ticket/wp-u5/patches/u5-preference-lane-apply.py` (`--check`: all anchors present in 12 files) | clean (anchors present) | `semio-framework-ui-runtime`, `semio-framework-plugin` (every guest links it, no ABI change), `semio-s-artifact-space-space`, `semio-s-artifact-space-home`, `semio-s-plugin-space` + hub `DirectoryEventLaneV1`/`HubAccessPolicyV1` (`preference.record`/`preference.read`) + directory schema `user.preference-recorded` event | **Y — space guest package** | N | needs only `semio-s-plugin-space` rebuilt + restaged for U5's live proof | LC | NOT APPLIED (only the ticket-local patch-script line count changed in the diff, not the target files) |
| LC-4 | H9, `📓️wp-h9.md` §L (l.147) | `wp-h9/codemods/kind-label-patch.py --apply` | **clean**, 184 `ArtifactKindSpec` literal sites in 93 files, 6 exact hunks | `ArtifactKindSpec.name: String` → `label: LocalizedLabel` across **semio-framework, semio-framework-plugin, semio-framework-os-host, semio-hub, and every `s` plugin crate** (descriptor bytes change for all of them) | **Y — every guest, largest blast radius in this inventory** | N | H9's own note: "W2 then rebuilds all guests once" — this is a describe-all + full restage trigger by itself | LC | NOT APPLIED (no `label: LocalizedLabel` in the diff) |
| LC-5 | H9, §C (l.38, l.226) | `wp-h9/codemods/creation-progress/os-kernel-and-creation.diff` (`patch -p1`) + `hub-creation-progress.py [--dry-run\|--apply]` | both clean, re-verified 18:3x | `semio-framework-os-kernel` schema (guest-linked) + 9 anchored hub edits + oracle `creation-progress-oracle.ts` | **Y — os-kernel is guest-linked** | N | root cause of the underlying stall (residency release) is routed to H10/LA — this patch is UI/progress reporting only, not the fix for "stuck 2d.puzzle" | LC | NOT APPLIED |

## 4. Guest store re-announce, opaque concurrency, field precision (→ slice **LD**)

| # | Source | Command | Dry run | Targets | Guest-linked | Kernel input | Deps | Live tree state |
|---|---|---|---|---|---|---|---|---|
| LD-1 | C10, `📓️wp-c10.md` l.38, l.311 | **No script exists yet** — only a root-cause note: guest `🏪️store/🦀️.rs` `flush_apply_outbound` never drains `pending_report.outbound`, so the next flush re-announces an Accepted op and the hub rejects it | n/a — not written | `semio-framework` store (guest-linked, every guest) | **Y** | N | blocks collab-e2e STEPs 4/8/11/12/13 (writer typing) — **on the critical path for outcome 3** | LD | **UNWRITTEN.** This is the single highest-priority gap in the whole inventory: a known, reproduced, root-caused defect on the collaboration critical path with no patch script anywhere under `.tmp-ticket/`. LD must write it, not just land it. |
| LD-2 | H9, `📓️wp-h9.md` l.39, l.400–419 | `zsh .tmp-ticket/wp-h9/codemods/opaque-concurrency/land.sh` (`--dry-run` → applies `store-causal-dependencies.py` + `db-opaque-concurrency.py` + `hub-vigilant-law.py`, then db nextest + os-kernel store laws + hub laws + wasm check through the fleet mutex) | **clean now; explicitly "not compile-verified"** (rule 20/24 blocked it; anchors verified against the 17:00 tree only) | `ArtifactStore::causal_head()` in **os-kernel store** (guest-linked, `replay_mutations` stamps the head) + hub db grading (`mutation.clamped`) + `hub-vigilant-law` fixture — native db/hub halves, kernel half is guest | **Y (kernel half only)** | N | **Field precision is an explicit, separate follow-up** (wire field on `MutationEnvelope`, 31 Rust + 11 TS files, spr op-meta bit 7, per-plugin schema declarations) — not in this patch; until it lands, opaque concurrent writes on *different* fields are still refused conservatively | LD | NOT APPLIED (symbol check: `mutation.clamped`/`causal_head`/`durable_group_edit_ids` absent from the diff) |

## 5. WG7's legacy items → slice **WG9** (wasm32 wgpu shell)

| # | Source | Command | Dry run | Targets | Guest-linked | Kernel input | Deps | Live tree state |
|---|---|---|---|---|---|---|---|---|
| WG9-1 | WG7, `📓️wp-wg7.md` l.60–67, l.190 (`s12-echo-suppression-kernel-patch.py`), Landing plan step 1 | `python3 wp-wg7/s12-echo-suppression-kernel-patch.py --apply` + `python3 wp-wg7/s12-link-expiry-patch.py --apply` (one window) → native kernel `--lib -- document_echo_suppression_tests document_link_shortage_tests os_store::sync` + TS twin laws | both clean, dry run + rustc-over-fixture verified standalone | kernel `admit_remote_envelopes`/`note_authored_envelopes`/`applied_op_ids` (region 🔁️DocumentEchoSuppression) + link-shortage expiry fix (cuts > ~31.5 s expired the link wrongly) | **Y — kernel, every guest** | N | React lane's half (`admitRemoteEnvelopes` etc.) already landed session 12 | WG9 | **CONFIRMED IN PROGRESS**: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs` (+57/-…), `🔄️sync/🦀️.rs` (+78), `🔄️sync/🧪️tests/🔬️document-link-shortage/🦀️.rs`, schema `document-link-shortage/🔣️.json`, and fixture `document-link-shortage-v1/🔣️.json` (**+429 lines**) are all touched in the diff — file names match this patch set exactly. |
| WG9-2 | WG7, l.65–67 (`s12-echo-suppression-hub-patch.py`), step 2 | `python3 wp-wg7/s12-echo-suppression-hub-patch.py --apply` → hub `bin-unit` catch-up laws + hub rebuild for 8050 (and 7800 via W3) | clean, H9 informed | hub `HUB_CATCH_UP_ORIGIN = "hub.catch-up"` passed to `state.db.hello`/`handle_frontier_advertise` instead of the socket actor | N (native hub) | N | none beyond a hub rebuild | WG9 | **CONFIRMED APPLIED**: `🌎️hub/🏗️bootstrap/🦀️.rs` diff shows the exact `HUB_CATCH_UP_ORIGIN` constant and both call sites rewritten, docstring citing "ticket 26/09/23 session 12, run s12i" verbatim. |
| WG9-3 | WG7, l.38 (`s12-hub-sign-in-off-interaction.py`), step 3 | `python3 wp-wg7/s12-hub-sign-in-off-interaction.py --apply` → renderer laws (`hub_projection_workspace_tests`, `ui_prefs_themes_i18n_tests`, `document_relay_tests`, `browser_input_wire_tests`) | clean | renderer (native + wasm32-unknown-unknown, browser wgpu shell) | **Y (browser renderer target)** | N | step 4 (`wasmshort` check + `s12-release-direct.sh`) depends on this | WG9 | not independently confirmed; several `🖱️ui/🎯️targets/🧊️wgpu/…` and `🐚️Shell`/`🛰️Dock` files ARE touched in the diff (a11y credit fix matches WG7 S12-4) but the sign-in patch specifically wasn't grepped for a distinctive symbol |
| WG9-4 | WG7 S12-4 (l.28), a11y | fix already in source per WG7's own table ("Tab from the canvas killed the shell → fixed"; native AccessKit half waits on Cargo.lock freeze) | — | browser a11y credit admission | Y | N | native half needs the Cargo.lock freeze lifted — same freeze window as everything else | WG9/WG10 | `🐚️Shell/🧪️tests/🎨️wgpu-theme-editor-and-accessibility/🦀️.rs` (+60) present in diff — consistent |
| WG9-5 | WG7 l.24, late-joiner root cause | "React lane landed, kernel + hub patches prepared" — same patch sets as WG9-1/WG9-2, no separate script | — | — | — | — | folded into WG9-1/WG9-2 above, not a separate item | WG9 | see above |

## 6. Native wgpu shell (→ slice **WG10**)

| # | Source | Command | Dry run | Targets | Guest-linked | Kernel input | Deps | Live tree state |
|---|---|---|---|---|---|---|---|---|
| WG10-1 | WG8, `📓️wp-wg8.md` l.285–289 | `wp-wg8/kernel-patch-transport-deadline.py --apply` → kernel native tests + wasm32 checks | **written, "NOT applied, NOT compiled"** (rule 20 explicit) | native ureq transport in `📇️directory/🔌️client` — bounds only the connect today; patch makes the overall timeout follow the caller's `OperationContext` deadline (120 s backstop) | kernel crate is guest-linked overall, **but the diff is inside `cfg(not(wasm32))` native transport code** — patch note itself still asks for a wasm32 check as a sanity pass | N | root cause of the native shell's 15 s hard-cut sign-in reload cutting a 30 s deadline (cross-shell run 8/9) | WG10 | NOT APPLIED |
| WG10-2 | WG8 (already landed 16:3x–16:51, no action needed) | `spaceRowsAfterEventsV1` read-your-writes fold | landed | renderer + shared contract | Y | N | — | landed (not part of the gap list, noted for completeness) |

## 7. Cross-platform (→ slice **Z3**)

| # | Source | Command | Dry run | Targets | Guest-linked | Kernel input | Deps | Live tree state |
|---|---|---|---|---|---|---|---|---|
| Z3-1 | Z2 (`wp-z2/pending/winit-linux-backends.py`); H10 defers to it explicitly (`📓️wp-h10.md` l.27–32, l.209–210: "Z2's B4 patch is the one to land… mine marked superseded") | Z2's `winit-linux-backends.py`, then any non-`--locked` cargo run re-resolves `Cargo.lock` (+11 crates), then `cargo check -p semio-framework-ui --features wgpu-engine` | clean (both copies) | `🧰️framework/🔨️modules/🖱️ui` Cargo.toml (adds `[target.'cfg(target_os = "linux")'.dependencies] winit = {… features = ["x11","wayland","wayland-dlopen"]}`) + root `Cargo.lock` re-resolution | N directly (native winit backend selection); unblocks Linux builds of the hub **and** the native wgpu shell | **Y — touches root `Cargo.lock`**, one of the explicitly-frozen kernel-derive-input files once W3 announces REBUILD START | none | Z3 | **CONFIRMED APPLIED**: `🖱️ui/📦️packages/🦀️rust/Cargo.toml` diff shows exactly this target table; root `Cargo.lock` (+123 lines) and 9+ plugin `🏭️bridge/Cargo.lock` files (writer, wfc, procedural, flow, gis, sequence, reasoning, forms, layout, playbook, trinity, dag — all +200-ish lines) carry the resolved `x11-dl`, `smithay-client-toolkit`, `wayland-cursor`, `calloop-wayland-source`, `as-raw-xcb-connection`, etc. **This must be recorded + `cargo check`-verified before W3 freezes Cargo.lock**, or it risks being caught mid-resolve. |
| Z3-2 | H10 item 3, `📓️wp-h10.md` row 3 | `zsh wp-h10/docker-build-context.sh … 1 h10-winit` then `zsh wp-h10/docker-run-drill.sh h10-winit 8136 …` | image "not built yet: honest status" | Dockerfile / `.dockerignore` (already landed), needs Z3-1 in the image first | N | N | depends on Z3-1 | Z3 | not re-attempted since 10:58 OOM (Docker VM 8 GB, `CARGO_BUILD_JOBS=1` retry queued) |
| Z3-3 | (new slice item, work-packages.md "NEW SLICES" (e)) | Z2 cross-platform + devcontainer audit | — | — | — | — | — | Z3 | design-stage, no script found |

## 8. Plugin correctness debt (→ slice **T13**)

| # | Source | Command | Dry run | Targets | Guest-linked | Kernel input | Deps | Live tree state |
|---|---|---|---|---|---|---|---|---|
| T13-1 (F10) | T12, `📓️wp-t12.md` l.301–322 | no single script named yet — census done (`wp-t12/` census tooling), fixes are: 7 missing `adapter()` + 11 missing TS adapters + `txt-oracle-refusal.py` (the one already-written adapter, = LB-2g) + a contract rule so a case can't silently never run | census measured (2 154 executed / 2 131 pass / 23 fail / 16 not-exercised); fixes for 10 of the 16 not-written yet | plugin test-parity adapters across ~18 cases | N (test infra) | N | none | T13 | not checked; mostly unwritten design + one written patch (LB-2g) |
| T13-2 (F9) | T12 l.412–414, l.260–271 | `hash::content_id(prefix, canonical_json)` = SHA-256 first-16-hex, replacing `std::collections::hash_map::DefaultHasher` at **~18 minting sites**; regenerate **661 committed carriers** in one sweep; add a law forbidding `DefaultHasher` for persisted ids | **not written as a runnable patch yet** — this is a specified design (T12: "one specified `hash::content_id`… main messaged"), not a dry-run-clean script | ~18 Rust minting call sites across multiple plugins (dag, writer, sequence, animate, playbook, imperative, trinity jack, raster, note, reasoning, cad, flow, en1990/norm) + 661 fixture carriers | Y (guest plugins) | N | must land BEFORE any further fixture regeneration touches those carriers, else double-drift | T13 | **UNWRITTEN, confirmed twice**: `fn content_id` in the repo today only matches 3 unrelated wgpu/renderer functions (canvas/draw content ids, not `hash::content_id`); repo-wide `DefaultHasher` usage is **345 hits** across `✏️s/🔌️plugins`, none yet migrated to the new helper. |
| T13-3 | T12 §S12-4 item 3, plugin lib-test debt | wfc bitmap solve laws under load, F4 `assert_viewer_never_mutates`, other `--lib` reds | — | various plugin crates | Y | N | — | T13 | not enumerated in source reports beyond the label; T13 needs to build its own census first |

## 9. Semio MCP (→ slice **G11**)

| # | Source | Command | Dry run | Targets | Guest-linked | Kernel input | Deps | Live tree state |
|---|---|---|---|---|---|---|---|---|
| G11-1 | G10, `📓️wp-g10.md` row S7 (l.40) + `wp-w1/requests/g10.txt` | `wp-g10/g10-preview-effect-refusal.py --apply` → cargo check native + wasm32 → full describe/restage | clean | agent-lane preview refusal for `Effect::LoadDocument` (note `setFixtureJson`/`setActiveExample`) with `interactive-job.preview-document-load` | **Y — plugin framework, every guest** | N | MCP side (no-change answer) already landed session 12; this is only the guest half | G11 | NOT APPLIED |
| G11-2 | G10 l.322, "quartet hub reds" | `inference_approve` PLUGIN_UNAVAILABLE after a directory event; hub tools failing after a directory event | not a patch, an open red — "waits for the post-publish 7800 carrying H9's gates" | hub | N | N | blocked on H11 landing H9's in-tree hub fixes first | G11 | open bug, no script |
| G11-3 | G10, `wp-g10/g10-adjacency-probe.py` (new file, uncommitted) | untested by this audit — file exists under `wp-g10/` but no report row cites it yet | — | — | — | — | — | G11 | new/uninventoried; flag for G11 to explain |

## 10. Build/publish structural items (→ slice **W3**, informational — W3 is not a landing slice per se but every item above ultimately serializes through it)

| # | Source | Note |
|---|---|---|
| W3-1 | W2, `📓️wp-w2.md` l.159–166, l.220–223 | SDK exact-pin narrowing (`VersionReq` admits only `Exact`; 22 extension declarations + demonstrator; emitter refusal) is **designed, not landed** — deliberately deferred because `VersionReq` lives in `semio-framework`, forcing a recompile of all 34 packages. W2's own words: "lands in the window, step 2." Land it as ONE step of the consolidated `rebuild-all`, not piecemeal. |
| W3-2 | W2, l.192 | Structural follow-up, **not started**: the kernel derive macro should read a narrow generated input instead of the whole `taxonomy.json`/`nx.json`/`project.json` — currently editing any of those three recompiles the kernel from the top. No script; design note only. |
| W3-3 | (fresh, session 13) | Publish 4's failure (`browser actor artifact: unsupported import interface`, imperative package, jco `generate`) has **no prepared fix yet** — `wp-w3/w3-import-scan.sh` exists but `📓️wp-w3.md` itself doesn't exist yet at this snapshot. This blocks the whole chain (rebuild-all → publish → 7800) and is W3's own first job, not a "landing" item from sessions 11–12. |

---

## (a) UNASSIGNED items

None of the 15 session-13 slices in `📓️fleet-13-agents.md` explicitly own these. Flag to the coordinator:

1. **LD-1 — guest store `flush_apply_outbound` fix (C10's find).** No patch script exists anywhere under `.tmp-ticket/`. LD's
   charter mentions "guest store re-announce" but only H9's already-scripted opaque-concurrency set has a `land.sh`; this one
   still needs to be *written*. This sits on the critical path for collaboration outcome 3 (blocks collab-e2e STEPs 4/8/11/12/13).
2. **The "rename slice"** (T12 §S12-4, `📓️wp-t12.md` l.390–421; R8's S12-6 plan, `📓️wp-r8.md` l.147–153, "not started, by design").
   Bundles: (i) trinity F1 rewriting-vocabulary rename, (ii) a root Cargo-workspace member move for 3 dev-benchmark test owners
   (touches `🔣️taxonomy.json`, library + dev `📋️project.json`, `.vscode/launch.json`, a fixture — **explicitly a kernel-derive-input
   move**, must NOT run during W3's REBUILD window), (iii) pdf `remove-pattern` reader/lift decision ("pdf owner, post-publish" —
   no owner named). None of LA–LD, T13, or Z3 claim this in their charter.
3. **T13-2 (F9, `hash::content_id`)** is assigned to T13 by name in `fleet-13-agents.md`, but as of this snapshot it is still pure
   design — no script. Listed here too because the 661-carrier regen is large enough that T13 should confirm it before starting
   anything else that touches those carriers.
4. **LB-4 (cross-plugin verb-arg census)** — named in `work-packages.md`'s landing-window list under LB's charter but no source
   report names a script; likely still to be authored from scratch by LB.
5. **G11-3 (`wp-g10/g10-adjacency-probe.py`)** — a file exists in G10's ticket folder with no corresponding report row; G11 should
   read it before assuming it is dead weight.

## (b) Recommended landing order

Given rule 4 (wasm mutex + kernel-derive-input freeze starts the moment W3 announces REBUILD START) and the confirmed live-tree
state in §1–9, the sequencing that minimizes rework:

1. **Right now, before W3's announcement** — land/verify everything that already touches a soon-to-be-frozen file:
   - LA-1 (`nx.json`) — **already applied**; LA should run its cache-input laws and record the row NOW.
   - Z3-1 (root `Cargo.lock` via the Linux winit target table) — **already applied**; Z3 should `cargo check -p semio-framework-ui
     --features wgpu-engine` and record it NOW, before Cargo.lock is frozen mid-resolve.
   - Any part of the "rename slice" that must touch `taxonomy.json`/`project.json` (UNASSIGNED item 2) — either land it in this
     same pre-freeze window or explicitly defer it to after PUBLISH DONE; it cannot straddle the freeze.
2. **Guest-linked sets that need only a compile check, not a describe/regenerate, before restage** — LC-1 (P8 agent-lane already
   applied; land flow/cad/space-studio/space-home next in P8's own declared order), LC-4 (H9 kind-label — largest blast radius,
   land early so its one describe-all absorbs everyone else's guest edits too), LA-4b/4c (H10 Q2 + codec-app-resolution), LA-5
   (R8 ui-retirement), LC-2 (F1 typing-coalescing), LC-3 (U5 preference lane), LD-2 (H9 opaque-concurrency kernel half).
3. **LD-1 must be written before it can land** — flag to main now; it is the only item on the collaboration critical path with
   zero code written.
4. **LB-1 (T12 postpublish-open-kinds)** — Parts A/C/D are already applied; land B/E/F/G, then run ONE describe-all + generate
   (shared with everything above) so the census law (Part E) and the T12 §S12-1 `descriptor_is_fresh` law both see fresh
   descriptors in the same pass.
5. **LB-3 (D1 frozen descriptions)** lands in the SAME describe-all pass as LB-1 because both touch gis/stdio (see Conflict 1
   below) — apply D1 first (pure text, zero risk of clashing with T12's kind-id rename), THEN T12's id rename, THEN one
   `cargo check` + one describe, not two.
6. **LA-4a (H10 Q1 interpreter)** — self-reverting one-command lander, safe any time; run before the final identity sweep so the
   sweep measures the landed interpreter, not the old one.
7. **G11-1 (G10 preview-effect-refusal)** — small, isolated, land any time in this window.
8. **WG9-1/2/3** — already substantially applied (echo-suppression + link-expiry + hub catch-up origin); WG9's first job this
   session should be to run the compile checks WG7's own report specifies (native kernel + hub bin-unit + renderer wasm32) and
   record the row, not re-derive the patch.
9. **WG10-1 (WG8 transport deadline)** — independent of everything above, land any time.
10. **T13-1/T13-2** — F10's written half (txt-oracle-refusal) already covered under LB-2g; the unwritten 10 cases and F9's
    content-id migration are design work T13 should scope before touching the 661 carriers.
11. **Only after all of the above are compile-green** — W3's ONE consolidated `describe → generate → check → activate-s →
    verify-s` (rebuild-all) → the imperative-codegen fix (still unsolved, W3's own first job) → `--packages all` → 7800 restart.

## (c) Conflicts (two sets touching the same file/symbol)

1. **D1's frozen set (LB-3) vs T12's postpublish-open-kinds (LB-1), both on `gis`, `stdio`, and (T12 only) `vcs`-adjacent files.**
   D1 touches `create_gis2d_app`/gisterrain descriptions and 62 stdio `.action_describe` call sites; T12 touches the SAME
   `artifact_kind()` files (kind-id rename) and the SAME stdio artifact-definition files. Both are dry-run-clean *independently*,
   but neither dry run was taken against a tree carrying the other's hunks. **Land D1's pure-text hunks first** (lowest risk of
   textual collision with an id rename), re-derive T12's dry run against that result, then apply T12. Do not apply both from
   stale dry runs.
2. **Z3-1 (Linux winit, root `Cargo.lock`) vs W3's REBUILD-window Cargo.lock freeze (preamble rule 4).** Root `Cargo.lock` is
   already re-resolved in the working tree with the winit backend crates. If W3 announces REBUILD START before this is
   `cargo check`-verified and recorded, it becomes frozen mid-resolve and Z3 loses its landing window until PUBLISH DONE.
   **Time-sensitive — resolve today's ordering with W3 before it announces.**
3. **H10's own `wp-h10/patches/linux-winit-backends.py` vs Z2's `wp-z2/pending/winit-linux-backends.py`.** Both are the identical
   fix; H10's report explicitly marks its own copy "superseded" (l.209–210) in favor of Z2's. The tree currently carries Z2's
   version (confirmed above). No action needed beyond noting H10's copy should not be separately applied.
4. **LC-4 (H9 kind-label, `ArtifactKindSpec.name`→`label`) touches every plugin crate; LB-1 Part A (T12's stdio id rename) also
   touches every stdio `artifact_kind()` call site.** Not a textual collision (different fields of the same struct literal), but
   both require their own full describe-all + guest restage. **Sequence them into the SAME describe-all pass** (land both source
   edits, one `cargo check` sweep, one restage) rather than two separate restages — this is also W2/W3's own stated intent
   ("W2 then rebuilds all guests once" — H9; "the next catalog carries it" — WG7's analogous framework-plugin change).
5. **LD-2 (H9 opaque-concurrency, kernel `causal_head`) vs a currently-uncommitted, unrelated refactor already sitting in
   `🧰️framework/🔨️modules/📡️replication/🔗️causal/🦀️.rs`** (extracts `edit_operation_mutation_id`, no relation to causal-head
   stamping — verified by reading the diff, not a false match). Not a real conflict today, but LD should re-derive its dry run
   against the CURRENT causal.rs (already modified by someone else) before applying, per landing rule 2 ("re-derive any hunk that
   no longer applies").

---

**Note on staleness:** §1–9's "Live tree state" column reflects `git diff` against HEAD at ~19:20. LA/LB/LC/LD/WG9/Z3 are actively
applying patches as this report is written (their own status tables already read as stale relative to the tree within minutes of
being written). Treat the confirmed-applied rows as a floor, not a ceiling — re-run the git-diff checks before acting on this
report if more than ~15 minutes have passed.
