# WP-S19 — Plugin Extensions: Contributions Framework, i18n, Flow Extensions; Norm Fixtures/Decoders

Session 14 slice S19 (2026-09-27 18:2x, Opus 5.5). Continues [S17](📓️wp-s17.md) and [N1](📓️wp-n1.md). Ports 8110–8119 /
6610–6619. Scripts `wp-s19/`, expendable captures `wp-s19/generated/`, binaries `CARGO_TARGET_DIR=wp-s19/target`, durable
logs `.🧬semio/🌐hub/s14-s19-logs/`. Native cargo only through the `native` lane (build-fleet-b); overlay builds only through
the `overlay` lane with a private build-dir inside the overlay.

### Session 14b

| # | Item | Status |
|---|---|---|
| 0 | Reconcile the predecessor's in-flight host TS edits | DONE — live tree carries ZERO S19 edits (below) |
| 1 | Verify S17 tool-run retirement + N1 test-only fixes on the current tree | S17: native `--lib --tests` 14:26 — kernel lib, SDK lib + tests, gen3d lib + tests compile (4/86+440/20+259 warnings); kernel LIB TEST red = peer's `🏪️store/🧬️retained-clone` test E0618 (SH2, rule 22); gen3d law queued (native). N1 norm part in the overlay proof |
| 2 | S17 2b-2 contributions framework (framework-reserved non-ledgered `setContributions` job) | NOT STARTED — sequenced after `flow-extensions` lands (its new flow route is the 9th per-artifact route 2b-2 migrates); needs the SDK + 9 artifacts in one window-3 pass |
| 3 | S17 2c flow Catalogue "Extensions" empty (sets `flow-extensions` guest + host scope) | PREPARED on the 28-09 tree (31-entry payload, dry run 0 conflicts); overlay tsc os/framework 0 new errors; vitest + cargo queued in the overlay lane |
| 4 | S17 1b `LocalizedLabel` migration | waits on T14's H9-L |
| 5 | N1 norm: fixtures, vdi3805/iso16757 decoders, norm reds, undeclared `path` arg | PREPARED `norm-examples` + NEW `norm-args` (MCP-callable arguments, 15 editors + law); overlay proof queued; emitter build queued; fixture regen + decoders open |
| 6 | Delete norm's `En1994Artifact` @deprecated alias | PREPARED (set `norm-cleanup`, 0 importers) — window 3 |
| 7 | generation3d seated example never evaluates in the served app | **ROOT-FIXED, LANDED (host TS)** — live: evaluates + STL export (three.js oracle) |
| 8 | 26 plugin extensions load in `s` with en + de | MEASURED: 26/26 `loaded` en + de, 0 faults; closure reach flow 1/9, procedural 2/9 (→ set `flow-extensions`), imperative 1/5 (4 built-ins, F1) |
| 9 | (main 13:4x, S20 relay) generation3d Import Document (archive load) traps the guest; generation2d + demonstrator/generation3d `initializer-failed` | ROOT CAUSE found; PREPARED set `gen-archive-load` (gen3d + gen2d + laws), overlay proof queued (green + red run) |

#### Session 14b Log

- 12:0x successor started; read AGENTS.md, preamble 14 (+14b), fleet tail, this report. Chain launched 12:02:46 → GUEST FREEZE ON.
- 12:1x **item 0 reconciliation (measured):** the predecessor edited ONLY the scratch overlay `.🧬semio/🌐hub/s14-s19-overlay`
  (codemods `s19-norm-examples.py`, `s19-flow-extensions.py`, `s19-contributions-scope.py` = the "host-side scope codemod", run
  20:06 on the overlay; `wp-s19/generated/tsc-os-overlay-1.txt` 20:45 = its last overlay `tsc`, red on stale overlay files).
  `s19-stage.py status`: 26/30 payload paths `tree=base`, 4 `tree=moved` (kernel `🟦️.ts`, ShellHost `🟦️.tsx`, wgpu Shell
  `🦀️.rs`, plugin-runtime test) — all four moved by the overnight peer (mtimes 04:10–04:55; kernel = peer's extraction of the
  external-slot resolver into `🧩️extensions/`), none by S19; no `apply --write` ever ran (`s14-s19-backup/` absent); payload
  holds only `.old` bases (never captured). → nothing to finish or revert in the live tree; no frozen path touched. Next:
  re-sync the overlay onto the current tree (peer's 1 870 files), drop the obsolete value-derive pins, re-run the three
  codemods, capture, overlay proofs.
- 12:1x overlay refreshed onto the 28-09 tree: `s19-overlay.py` gained prune + `node_modules` mirror (`@semio-tech/*` relative
  links into the overlay) + `--reset-staged` (staged paths back to the tree, `.old` rewritten); value-derive pins dropped
  (`s19-overlay-protect.txt` = root `Cargo.toml` + emitter member only). 79 243 files, 2 323 re-placed, 0 pruned. The three
  codemods re-ran clean on the new tree (0 anchor misses; idempotent: second run 0 edits). `s19-contributions-scope.py` +
  manifest `🦀️.rs` docstring (last `exampleArtifactSources` mention). Payload captured: `flow-extensions` 21, `norm-examples` 10,
  `norm-cleanup` 1 (NEW `s19-norm-cleanup.py`: en1994 snapshot TS alias, 0 importers by `git grep`); `s19-stage.py apply` dry
  run: 32 to apply, 0 conflicts.
- 12:2x overlay tsc (host half of `flow-extensions`): os package 88 errors vs live 85 — delta = only overlay-missing gitignored
  build outputs (puzzle `pkg/`, jcoprobe bundle) + union ordering; 0 errors in any touched file
  (`generated/tsc-os-{overlay-2,live-1}.txt`); framework package 7 errors, none in kernel (`tsc-framework-overlay-1.txt`).
- 12:4x **item 7 root cause (measured):** baseline live probe on a local-only `s` serve 6610 (`s19-serve.sh`, HMR off):
  generation3d opened from Home, contributions installed into instance 2, preview "Computing 0/1 (0%)" after 75 s
  (`s14-s19-logs/live/gen3d-3`). The install's own re-arm (`owe_attached_previews_carrying` → `flowEvalTick` run start) came
  back as `requestedEffects`; `publishContributions` dispatched them as `{ ...sessionRef.current, pluginId, instanceId }` — in
  `s` the primary is Home, so `makeEffectDispatchOne` saw `flowEvalTick` absent from HOME's commands, sent it as a Home action,
  the guest refused, and `.catch((error) => undefined)` swallowed it; the guest kept its run "started" forever.
  Fix (host TS, open): `contributionsReceiverSessionV1` (`🪟️spawned-program`), `dispatchDeferredEffects(receiver, effects)`,
  install passes its receiver (own app), failures via `logUnlessRetiredV1`; fixture + law in `🪟️spawned-program-session`.
  tsc os 84 errors, 0 new (`tsc-os-live-2.txt`). Live after (`gen3d-4`, `gen3d-5`): boot `ready:s`, 0 pageerror; preview
  "Finalized · 7 nodes evaluated, 3 meshes tessellated" (hexagonal column); Actions rail `exportDocument` → `generation3d.stl`
  3 810 B; three.js `STLLoader` (`s19-stl-oracle.ts`): 20 triangles, bounds x ±0.5, z 0..6 = the example's radius/height.
  Landing row written. Law run queued (`generated/lane-vitest-1.txt`, overlay lane, same hold as the overlay suites).
- 12:5x overlay build-dir seeded with 4 378 third-party units cloned (APFS, read-only source) from build-fleet-b (path crates
  still build in the overlay); cargo holds capped at 28 min (`perl alarm`, re-run continues). `s19-overlay-proof.sh` queued
  (pid 30804): norm `--lib --test compliance_gate`, flow artifact `--lib` → `s14-s19-logs/ov-proof-1-{norm,flow}.txt`.
  `cargo metadata` in the overlay: procedural consumes `[forms.questionKind, flow.extension]`, demonstrator + `flow.extension`.
  Window-3 note: the host `consumes` rows come from the gitignored generated `📇️registry/🤖️generated/🧩️plugins/🟦️.ts`
  (registry script) — regenerate it with the Cargo.toml edits.
- 13:0x **item 8 measured** (`wp-s17/s17-extension-probe.ts` against 6610, `s14-s19-logs/live/ext-{en,de}-1.json`): en + de,
  seated `de=de`: all 26 extension plugins `loaded` (35 parent/extension rows), 0 faults. Pushed closure per parent: process
  4/4, sourcing 3/3, cad 4/4, playbook 1/1, imperative 1/5 (control/text/logic/math are parent built-ins, F1 — only `effect`
  contributes an `imperative.module`), flow 1/9 + procedural 2/9 = the operator-reachability cut that set `flow-extensions`
  removes (and flow's editor owns no `setContributions` yet — the "pushed" witness counts a publish whose install loop skipped
  flow). Extension payload texts English in de = item 4 (1b).
- 13:1x **item 5 `path` arg (measured on the tree):** the norm manifest DOES declare `path` — as OPTIONAL text, and `index`/
  `remedyIndex` as TEXT, while the handlers need a path and read ordinals with `as_u64`; `insertItem.value` is read
  (`value_arg_json`) but undeclared. So a schema-following agent sends `{}` (→ `INTERNAL "path must not be empty"`, G11) or a
  string ordinal (→ 0). The coverage battery (`🌉️mcp/🧪️tests/🧩️plugin-coverage`) fills only REQUIRED inputs. Set `norm-args`
  (NEW `s19-norm-args.py`, overlay, 15 editors): `path`/`checkId` required text, `index`/`remedyIndex` required
  `ActionArgDef::index` (integer ≥ 0), `setField.value` required typed value (`any` — the handler re-serialises a typed value,
  so `json_text` would double-encode), `insertItem.value` optional typed value, `setSnapshot.snapshot` required `json_text`;
  law `📕️norm/🧪️tests/🎯️action-args` ×15 (mounted from the plugin root). NOTE: `norm-examples` and `norm-args` share the
  en1997/din4108 editor files (tracked under `norm-examples`) → land both sets together; the manifest `🔣️.json` /
  descriptor regenerate from the builders (chain rebuild-all "descriptors" step). Stage: 47 entries, dry run 0 conflicts.
- 13:1x queued: native lane `s19-item1.sh` → `s14-s19-logs/item1-2-{check,law}.txt` (S17 kernel/SDK/gen3d `--lib --tests` +
  gen3d law, pid 53249); overlay lane emitter build → `ov-emitter-1.txt` (pid 60862, for N1's fixture regeneration).
- 13:19 overlay-lane vitest run 1 (`generated/lane-vitest-1.txt`): kernel `scopeContributionsJson` **9/9 passed** (overlay);
  the two engine invocations collected no file — the react engine config includes its listed suites only when
  `SEMIO_TEST_LEVEL` is above `fundamental`/`quick` → re-queued with `SEMIO_TEST_LEVEL=standard` (`lane-vitest-2.txt`, pid 73965,
  adjacent to my proof ticket).
- 13:2x item 5 decoder drift, measured: 7 unit enums carry test-serde `camelCase` but no `#[value(rename_all)]` (vdi3805
  `VdiQuantityKind`/`SchemaStatus`/`Domain`/`Severity`/`EditionProfileChoice`, en1995 `MemberRole`/`SupportType`), so
  production writes PascalCase. en1995's committed fixtures already carry production spelling (`"Floor"`, `"SimplySupported"`);
  vdi3805's carry `"legacy"`/`"current"` (38 rows) against production `Legacy`; its jsonschema leaves `editionProfile` values
  free strings. Decision (N1's plan, production = truth): regenerate the fixtures from production through the emitter
  (`n1-vectors.py` → `n1-materialize.ts`) once `ov-emitter-1` is built, then tighten the schema to the production spelling —
  no codec rename (it would only move the drift into every committed en1995 fixture).
- 13:2x dev serve 6610 stopped (process group 90538) — idle until window 3.
- 13:4x **item 9 (main relay from S20)** — root cause (source-measured): generation3d and generation2d editors declare
  `REQUIRES_DOCUMENT_STORE_PUBLICATION_AUTHORITY`, their `validate_document_store_publication` demands a lease from the
  process-wide publication table, but outside tests nothing admits one for a replacement the HOST began (archive import,
  whole-document load) → publication fails `*-publication.authority-missing`; gen2d surfaces it as `initializer-failed`,
  procedural gen3d as the guest trap (a fault-path drop of an unretired `OrderedMap` — the masking panic; not reached once
  the publication succeeds). process3d carries the fix since 09-16 (app self-grant lease). demonstrator/generation3d is the
  same `Generation3dPlayApp` → covered. Set `gen-archive-load` (NEW `s19-gen-archive-load.py`, idempotent, 6 files): port of
  process3d's self-grant verbatim into gen3d + gen2d binaries (`*_app_publication_lease`, host-first/app-second lookups,
  admit in the initializer's `new()`, release at validation/cancel/fault/close) + editor `validate_document_store_publication`
  releases the grant + law `a_document_archive_loads_into_a_fresh_instance_through_the_import_door` per artifact (real door:
  `document_archive` → fresh instance `begin/poll/acknowledge_document_archive_load` → Ready + identical widget ids).
  S20's `s20-patch-initializer.py --dry-run`: 69 files, none procedural/demonstrator → no overlap (gen2d/gen3d keep their own
  initializer). Proof `s19-gen-archive-proof.sh` queued (one hold: gen3d + gen2d laws green, then gen3d red on the unfixed
  binary/editor) → `s14-s19-logs/gen-archive-proof-1.txt`. The fault-path masking drop stays a separate latent defect (noted).
- 14:26 item 1 native check (`s14-s19-logs/item1-2-check.txt`, build-fleet-b, 13:11–14:26 incl. lane wait): EXIT 101 from ONE
  target — `semio-framework-os-kernel (lib test)`: `🏪️store/🧬️retained-clone/🧪️tests/🔬️unit/🦀️.rs:311` E0618 `RetainedCloneGrant`
  (peer fallout, SH2 owns under rule 22). Everything S17 landed compiles: kernel lib (4 warnings), SDK lib (86) + lib tests
  (440), gen3d lib (20) + lib tests (259) + `incremental-eval`. gen3d law queued in the native lane (`item1-2-law.txt`).
- 14:27 overlay lane reached: `ov-proof-1` running (norm, then flow), then `gen-archive-proof-1`, then `lane-vitest-2`, emitter.

## Session 14

| # | Item | Status |
|---|---|---|
| 1 | Verify S17's window-2 tool-run snapshot retirement + N1's test-only norm fixes on the current tree | in progress |
| 2 | S17 2b-2 contributions framework (`setContributions` → framework-reserved, non-ledgered job; 8 artifacts, ShellHost, TransientStore) | open |
| 3 | S17 2c flow Catalogue "Extensions" group empty (operator-keyed reachability cut) | open |
| 4 | S17 1b i18n `LocalizedLabel` catalogue text (after T14's H9-L) | open |
| 5 | N1 norm: production-derived fixture vectors, vdi3805/iso16757 decoder drift, norm reds, norm in `s` + hub | open |
| 6 | 26 plugin extensions: load in `s`, artifacts exposed, en + de | open |

### Session 14 Log

- 18:2x started; read AGENTS.md, preambles 14/13/12, `fleet-14-agents.md` (no "CHAIN LAUNCHED" yet), `wp-s17.md`, `wp-n1.md`,
  `audit-s13-window3-inventory.md`, fleet-13 log 14:00–16:0x. Lanes free, 0 rustc, disk 141 GiB, swap 7.6/9.2 GB.
- 18:2x predecessor captures read: S17 `land-tool-run-1-check.txt` (kernel + SDK `--lib --tests`) **EXIT 0 16:22**, wasm32
  kernel + SDK + procedural **EXIT 0 17:03** — both on the 15:2x–16:2x tree. N1 `wp-n1/generated/lane-run-1.txt` (16:01–16:12):
  norm `--lib` **29 passed / 1 FAILED** (`en1992_set_active_example_loads_liquid_retaining_fem_anchor`: "must load the example
  through setSnapshot"), `--test compliance_gate` **FAILED** (10/15 families: "no applicable remedy cleared the Fail"; the test
  prints a `[DEBUG] compliance-gate fleet:` line), emitter build **E0432** (`protocol::value::{FromValue, ToValue}` moved).
  So N1's window-2 fixes compile but the two tests are still red → item 1/5 work.
- 18:36 item-1 chain `wp-s19/s19-item1.sh` (native lane, build-fleet-b; kernel + SDK + gen3d `--lib --tests`, gen3d law, norm
  `--lib` + `compliance_gate`) → `.🧬semio/🌐hub/s14-s19-logs/item1-1-*.txt`. 18:48 check **EXIT 101**: NOT S17's code — a peer's
  in-flight value-derive edit (`🌱️value/✨️derive/⚙️expansion/🦀️.rs` +904 lines, `🔁️codec` +225, untracked `🧪️tests/🧭️typed-path/`,
  files written 18:35–18:49) breaks `semio-framework-3d` (E0282 `🥽️mesh` 231), `semio-framework-actor` (8× E0282 `🚪️lifetime`,
  `📤️return`) and the kernel (5× E0034 "multiple `from_value`" in `🏪️store/🧩️composition/🗄️durable-group`, E0282 store 2892).
  Stopped my queued law/norm steps (they compile the same crates; killed my 43205 + waiter 59083, ticket removed). Re-run when
  the peer's edit compiles.
- 18:4x overlay `.🧬semio/🌐hub/s14-s19-overlay` (APFS clonefile of 78 384 tracked + untracked-unignored files + 18 gitignored
  `🤖️generated` dirs, 5 min; `wp-s19/s19-overlay.py <root>`, re-run = sync changed files); `cargo metadata` resolves.
- 18:5x item 5 census `wp-s19/s19-norm-example-census.py`: the N1 surface red is a PRODUCT defect, not a stale test —
  en1992's editor offers 5 examples in its navbar (`examples()`), `setActiveExample` resolves only 2 (hand-kept match), so
  picking "Liquid-retaining tank" / both prestressed beams is a silent no-op. Same drift: en1994 (2 of 3 missing:
  `composite_floor_beam`, `…_failing`), en1998 (2 of 4: `seismic_multipart`, `…_fail`), iso16757 (`broken`). Root fix =
  every family resolves `setActiveExample` through its own `examples()` roster (en1995 already does) + a law over EVERY roster
  example of all 15 families (the current surface law checks one example per family).
- 19:0x set `norm-examples` written in the overlay (codemod `wp-s19/s19-norm-examples.py <root>`, idempotent; staged by
  `wp-s19/s19-stage.py` = whole-file `.old`/`.new` pairs + three-way-merge apply, dry run default, backups under
  `.🧬semio/🌐hub/s14-s19-backup/`): shared `app_surface::roster_example_snapshot` (empty id → empty doc, id outside the
  roster → no-op, unparsable body → named fault `norm.set-active-example-invalid`, never a fallback); en1992/en1994/en1997/
  en1998/iso16757/din4108 `setActiveExample` resolve through `<Editor as ArtifactEditor>::examples()`; en1997's silent
  `decode(...).unwrap_or(fallback)` is gone; rosters completed (en1997 + `compliant`, `noncompliant`; din4108 +
  `failing-thin-insulation` — mounted example modules the picker hid); law `🔬️surface` `<family>_roster_examples_all_load`
  ×15 (every roster example loads via `setActiveExample`; roster ids == the `pub const ID`s of `📚️examples/*/🦀️.rs`).
  Overlay builds pin the Codex peer's in-flight value-derive files to HEAD inside the overlay only (`s19-overlay-pin.sh`,
  `s19-overlay-protect.txt`). First overlay run died on a gitignored generated source (`🔤️tokens/🦀️.rs`) → overlay now also
  clones ignored `🤖️generated*`/`🦀️.rs`/`🔤️tokens`/`🕸️bindings` entries; a sync clobbered the unstaged edits once → the sync
  now skips staged + protected paths and every edit is a re-runnable codemod.
- 19:1x N1's emitter copied to `wp-s19/emitter/` (import `protocol::{FromValue, ToValue}` — the E0432) and built as an
  overlay workspace member (`.s19-emitter`, shares the overlay build-dir). Overlay proof chain `s19-norm-overlay.sh` pid 52330
  → `s14-s19-logs/norm-ov-2-{test,check,emitter}.txt`.
- Matrix fact (S16, `🧮️program-matrix/🟦️.ts` `normSnapshot`): a norm row stages the FIRST mutation fixture case that has
  `📸️snapshot/➡️after/🔣️.json`; 12/15 rows fail because those snapshots are stale against the schema or absent → the
  production-derived fixture regeneration (N1's `n1-vectors.py` → `n1-materialize.ts`) is what makes norm 15/15 in the matrix.
