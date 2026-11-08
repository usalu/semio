# 1107 Audit: open claims, staged patches and DEBUG census (lane aud-claims)

Read-only audit. No repo runs, no cargo, no sub-agents, no edits outside this folder.

## Method and limits

- Read in full: `1105b-claims.md` (675 lines, entries 11:27 to 21:42 plus out-of-order 00:xx to 07:xx entries). Each open item below was checked against the claims text and, where possible, against tracked source with `git grep` and `sed` reads of the named file.
- Result files (27 `1105b-result-*.md`): read the STATUS line of every file, the tails of the 17 larger ones, and grepped every file for LOCK NEEDED, staged, prepared, not applied, UNVERIFIED, DEBUG, NOT started. The full text of the runtime, hostcheck, aggregator, policy, nativetests and opbin results was not read line by line; only their open-item lines were read.
- Lock state: newest lock marker is `🗑️generated/1105b/lock-38.rc` (mtime Nov 6 22:13). 337 `Cargo.toml` files under `✏️s`, `🧰️framework` and `🌎️hub` are newer than it. Crate presence checked against tracked `Cargo.lock` files.
- DEBUG census: plain `grep -rn` over `✏️s`, `🧰️framework`, `🌎️hub`, `♻️mit-bestand` with node_modules, target, dist and .cache excluded. Raw output: `/private/tmp/claude-501/-Users-ueli--claude-spares-spare-4AHhVe/81aa81ee-3456-4db7-8ac0-58d6181adc0a/scratchpad/debug-census.txt`.
- Not verified by a run: every "UNVERIFIED" item below is still unverified. Source reads prove a symbol exists or is absent, not that behaviour is correct.

## 1. Staged patches not applied (confirmed in source)

| Patch | Where | State in tree |
|---|---|---|
| hostcheck ArtifactRef (`take_artifact_ref`) | `1105b-tools/hostcheck/apply-artifact.py go` (reference half), `go child` (child half) | NOT applied: `take_artifact_ref` absent from all tracked `.rs`. ArtifactChild staged, not applied. |
| store fn-pointer typed codec change | `1105b-tools/store/fnptr/apply.py` (dry-run 137 files, 0 manual) | NOT applied: `store/🚪️io/🦀️.rs:145` still `export: fn(..., &mut Option<IoResult<..>>, ...)`. |
| store bounded persisted-merge v2 | `1105b-tools/store/merge/` (design.md, follow-children-step.rs, apply.py) | NOT applied: `ChildFollowPlan` exists only in the ticket tools folder. |
| policy register() Result codemod | `🗑️generated/1105b/policy/register-result/` (stage_step1.py) | NOT applied: `RegistrationFault` absent from tree. On hold for "APPLY" (policy result section 7c). |
| prototype wasm debug=0 | `1102-tools/actinfra/patches/prototype-wasm-debug0.py` | Applied (hub result; prefreeze PASS "wasm-debug0"). Not open. |

## 2. Half-landed or unverified edits

- **Envelope decode never installs its replacement identity (BLOCKS PANE, all 8).** Store decode authorities now treat a bound identity as stale when none is installed (`job/context/🦀️.rs:261` calls `ReplacementIdentity::is_stale(stored, current)`, and `replacement/🦀️.rs:26` returns true when stored is Some and current is None). Ingress binds `envelope_decode_replacement(..)` (`document/📨️envelope-decode-and-ingress/🦀️.rs:6`). The driver `drive_envelope_decode_jobs` (`maintenance-ladder/📥️staging/🗝️envelope-ladder/🦀️.rs:184`, called at `maintenance-ladder/🦀️.rs:141`) has no `with_replacement_current` call, and neither has the envelope-decode-and-ingress file. Claim 389 asked runtime for this wiring; no FIXED line follows. The store-replacement driver does install it (`store-replacement-admission-and-member-drive/🦀️.rs:346`). Owner: runtime (ladder) with store.
- **setContributions lookup (claims 208, 228, 229).** Emission registration now falls back to `framework_registration` (`emit/🎟️original/🦀️.rs:31`), which matches the claimed edit. Bearbeiten's "unknown-key" rejection (command-routes actions:373) was not found in tracked source. Status: landed by reading, unproven; bearbeiten claim has no FIXED.
- **Result-page 48 B heap witness mismatch (claims 272, 273).** Runtime CLAIMED 23:42; no FIXED. Same class as generator take_fixture_result_page (proc W-proc-s13, claims 548 and 549, runtime-b added a HeapWitness, EDITED-unverified).
- **Lifecycle receipt vs whole producer receipt (claims 274, 275, 294).** Whole-receipt fix EDITED-unverified (01:44). Koordinator cad s6 and aussuchen smoke-12 both cited this fault; no later pane pass recorded.
- **Replay initializer staleness (claims 350, 353, 435).** Replay stale = `cx.is_cancelled() || replacement_is_stale`; producers bound in document/ladder (claim 384) but "replay initializer still stale=cx.is_cancelled() (ladder-driven via predicate)" per runtime result. Half-landed.
- **Aussuchen Retiring stall (claims 673, 674).** ChildContentReadAliases retain = Arc::strong_count > 1; EDITED-unverified; smoke rerun not recorded.
- **Aggregator typed-operation publication stall (claims 160, 351, 538, 566, 675).** FIXED-candidate only (ViewLease, close order, rung selector). Aggregator result (STATUS, s38): publication-stalled, probing (s39).
- **Generator frame size (claims 479, 508, 539, 563).** Resolved: Generation3dPreviewCommandWork 1,675,904 B to 952 B, GREEN (W-runtime-g3d2). The release-chunking question (claims 360) is moot.
- **Accidental claims-file edits.** Claims 136: an energie `sed` deleted a peer's last line; author asked to re-append. No re-append found. Claims 586: offclosure ran `git rm --cached` then `git reset` on one path. Index is currently empty (`git diff --cached` = 0 files), so no residue. Violates the no-modifying-git rule; coordinator should know.
- **Flow one-fuel quote law (claims 441, 447, 456, 459, 595, 657).** Several quote-exactness edits, all UNVERIFIED until flow-t20/t21/t22; law still red at last report.
- **Nativetests value-test closure and persistent/snapshot-read/tests-2 reds.** Test drift; result AMBER.
- **Offclosure draw/layout/viewer.** offclosure-54 FIXED-unverified; editor-store-preparation, presence, host-digest and viewer not green; layout engine background_scene needs a cursor driver.
- **Trinity graph io (claims 13:04).** Two receive sites UNVERIFIED (repo cargo could not build that standalone workspace); stdio trinity sqlite artifacts not started.
- **renames-1 cargo feature-resolver panic.** Open in renames result, no later note.

## 3. LOCK NEEDED not honoured

- `semio-plugin-tests-20`: registered in root and framework members at 16:41 (lock-33 rc 0 claimed). Not present in any tracked `Cargo.lock`. Recheck.
- `semio-plugin-declared-capture-tests` (hostcheck, 07:40, LOCK NEEDED): not in any tracked `Cargo.lock`. Open.
- Other LOCK NEEDED entries never closed in the notes: opbin hub foundation/space engine/space commands/space-app Cargo.toml (claims 521, 628, 637, 654, 671), opbin mutation-diff ordered dep (669), opbin dsl module-tests dev-dep (565), offclosure draw geometry crate and viewer dep (531, 534) (`semio-s-artifact-draw-drawing-scene-mounted` is in 2 locks, so partly done), offclosure layout engine FontHandle dep (468), process3d workspace trace dep (300), statik fem3d trace dep (323), cad and aussuchen trace dev-deps (303, 304), companion tests dev-dep (276). Per-crate lock state for these was not verified.
- Gate: `gates-lower1` refused by CargoLockRefused: `semio-framework-os-store-one-item` renamed to `semio-os-store-one-item`. Gates lane.

## 4. Named blockers by pane (from result and claims state)

- **generator (proc):** load-example idle-bound stall regressed at s15 (06:16 note). Catalogue-owner close panic and SIGABRT cascade. take_fixture_result_page heap (0,0) vs receipt (48,0). Owners: proc, runtime-b, runtime.
- **koordinator (cad):** lifecycle receipt fault (s6), then support tests; time-travel close retains undeclared session editor frontier. Owner: runtime.
- **aggregator (puzzle3d):** addObjectKind publication stall (4096 units without moving ladder), worker-close stall cleared (s38). Owner: aggregator, with runtime typed-operations.
- **energie (energy):** initializer-cancelled at load; RED at energie-t23. Owners: runtime, store (replay and replacement).
- **aussuchen (sourcing):** publication and Retiring stall, root ChildContentView alias. Owner: runtime-b (fix EDITED-unverified; smoke pending).
- **bearbeiten (process3d):** seeded settle WorkLimit (W-s2 0/4, W-s4 0/4); history-edit close UnsupportedOwner (undeclared session editor preview). setContributions unknown-key (see section 2). Owners: bearbeiten, runtime.
- **verfolgen (gis):** phase-loan fault on document load; general fix in ownership (claims 320, 567 law GREEN) but no pane rerun recorded after it. Owner: runtime, needs rerun.
- **statik (fem):** loads_applies fails at document-archive initializer-failed (W-statik-s8 at 05:30). Owners: runtime, store (replay initializer binding).
- **All eight:** envelope decode identity wiring (section 2, first item).

## 5. Classification of open items

**Blocks-activation**
1. E2E gate state (e2e result, prefreeze-2): slots held (slots.k=11, wextra=2 at that run), sync-5 without rc, dependencies lock to rerun, ext-ok older than hub space edits (13:45), channel-pin and descriptor INFO. Owner: coordinator (slots, sync, lock), hub lane (ext.sh rerun).
2. Lock: `semio-plugin-tests-20` and `semio-plugin-declared-capture-tests` not in tracked lock; 337 Cargo.toml newer than lock-38. Owner: coordinator.
3. actinfra: `repo run` component target and store publication NOT proven (actinfra STATUS). Owner: actinfra.

**Blocks-pane (named)**
4. Envelope decode identity wiring: all eight panes (runtime ladder).
5. generator (proc, runtime-b, runtime): load stall, close panic, 48 B page witness.
6. koordinator (runtime): lifecycle receipt.
7. aggregator (aggregator, runtime): typed-op publication stall.
8. energie (runtime, store): initializer-cancelled.
9. aussuchen (runtime-b): ChildContentView alias, fix unverified.
10. bearbeiten (bearbeiten, runtime): settle WorkLimit, history-edit UnsupportedOwner.
11. verfolgen (runtime): rerun needed.
12. statik (runtime, store): archive initializer.

**Post-e2e debt**
13. Flow laws: one-fuel quote, retained donation/Resolve (3 red). Owner: flow.
14. Nativetests test drift (value-tests, persistent, snapshot-read, tests-2). Owner: nativetests.
15. Store: tests-02 (4 red), tests-06 canonical sealer, fn-pointer typed codec change (prepared, not applied), merge v2 (design only). Owner: store.
16. Hostcheck: ArtifactRef and ArtifactChild staged not applied; ArtifactLink not started; global ArtifactPersistedOperation bound lands last (coordinator decision 01:00). Owner: hostcheck.
17. Opbin: mutation-diff KeyedMap (LOCK NEEDED, unverified); hub foundation tests and space-app tests stale (35 errors); hub camera BTreeMap to OrderedMap not done. Owner: opbin. Check first whether the hub space app is inside the demonstrator closure; if yes, this blocks activation.
18. Policy: register() codemod not applied; 4 unwrap/expect need design. Owner: policy.
19. Offclosure draw/layout/viewer, trinity graph io, wit reactor-context native laws (wit-t3 never run). Owners: offclosure, value, wit. Drawing is not in the demonstrator closure.
20. Gates lower-owned-equality lock break (crate rename). Owner: gates.
21. Actinfra rust-partition-size test (deferred), renames-1 resolver panic. Owner: actinfra, renames.
22. Value zero margin 32/32 files; session source-cap margin 1 percent; cad editor unit cannot build as external target. Owners: value, flow, cad.

## 6. DEBUG census (source, `[DEBUG]` lines)

Result: 42 lines in 22 files (15 under `✏️s`, 27 under `🧰️framework`). `🌎️hub` and `♻️mit-bestand`: 0 hits. Prefreeze at 13:50 Nov 6 counted 47 lines in 21 files.

- **Temporary aggregator-gate prints, production or test code (remove, aggregator lane): 18 lines in 8 files.** `add-object-kind-work` (W, K1 to K4), `pane-smoke` (V, U), `store-preparation` (PR), `worker-job-session` (C1), `worker/🎟️authority` (C2, C3), `typed-operations` (Q2), store `bundle` (BL, production store code), `raw-app-fixture` panic message.
- **Temporary test diagnostics (remove, owning lane): 11 lines in 7 files.** fem `native-tests.rs` (2), flow `publication/work` tests (2), flow `evaluation-source/read` comment (1, a `// [DEBUG] size probe` comment), instance `initialization` ephemeral test (1), registration tests (2, including a 1,048,576 iteration loop with eprintln), replay `supersession` tests (2), store `emission/receiving` test (1).
- **Protocol or tool strings (keep, or rename the tag): about 13 lines.** `process-arena-child-ready` test handshake; nx graph `[DEBUG] graph-root` and `[DEBUG] Fixed UDRW image` (parsed by production code; policy lane kept them on purpose); repo's own census module docstrings and tests (`docstring-and-debug-census`); repo custody and dependency TS tests with diagnostic `[DEBUG]` text (owner: repo lane to decide; these look like temporary diagnostics too).

## 7. Suggested next steps for the coordinator

1. Re-run the e2e prefreeze after the envelope identity driver lands. The envelope driver is the single change most likely to make all eight panes fail on load.
2. Confirm LOCK for the two untracked test crates. Confirm the hub space app is or is not in the demonstrator closure (item 17).
3. Remove the 29 temporary `[DEBUG]` aggregator and test prints before the freeze. Keep protocol strings or rename their tag.
4. Keep staged patches (hostcheck child half, fn-pointer, merge v2, register codemod) out of activation.
