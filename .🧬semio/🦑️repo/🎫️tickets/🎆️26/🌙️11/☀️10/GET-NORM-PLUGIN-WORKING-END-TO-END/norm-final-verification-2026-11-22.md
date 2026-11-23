# Norm Final Verification, 2026-11-22 Evening

This report covers the remaining steps from `handoff-2026-11-22.md`.

## `norm` rebuild and base routes

- `norm` was rebuilt from the current source; the build exited 0 and the core was published at 19:21. Log: `🗑️generated/rebuild-norm-h1.log`.
- The strict Play probe then passed all four base routes: din4108, din16798, iso16757 and vdi3805, each with outcome 0. Flags: `VERIFY_SIGNED_MODULES`, `PLAY_TEST_PICKER_ROUNDTRIP`, `PLAY_TEST_OBSERVE_ARCHIVES`, `PLAY_TEST_REQUIRE_DRAW`. Log: `🗑️generated/routes-h1.log`.
- The `svc pre` print is gone. The core still contains 5 `[DEBUG]` strings from other agents' active diagnostics in shared framework code:
  - FEM cleanup failure
  - `poll_wit` authority refusal
  - natural-file worker
  - retained close
  - unresolved input receipt

  All five sit on refusal or failure paths and stayed silent during the runs. They belong to those agents' tickets and are not removed here.

## JS and contract targets (rerun)

| Target | Result |
|---|---|
| `@semio-tech/norm-js:test` | 12051 pass, 0 fail |
| `@semio-tech/norm-js:test-wire-twins` | 11914 pass, 0 fail |
| `verify-norm-document-contract` | pass |
| `verify-norm-results-window-ownership` | pass |

## Example documents

`check-norm-example-schema.py` checked all 79 example documents of the 15 families; see `norm-example-schema-audit-2026-11-22.md`. Eight EN 1990 documents had stale `reliability-analyses` and `test-series` headers, and they now carry the current columns.

## Official Rust test, family example crates, guest rebuilds, full sweep

The chain is driven by `finish-chain.sh`, which uses `test-family-examples.sh` and `rebuild-and-probe.sh`.

### Disk

The first official test run failed with `No space left on device` in the Rust image; the host had 25 GiB free. The stale ticket `26/08/29/AI-MCP-END-TO-END` held 19 GiB in private cargo target directories under `🗑️generated`: `cg`, `host-spill`, `native-parser-test` and `native-guests`. They had been idle since 2026-11-21 and no process held them open. Their own ad-hoc scripts set `CARGO_TARGET_DIR`, which bypasses the shared store. I removed them and kept that ticket's logs and reports. The host now has 53 GiB free.

### Family example crates, first pass

| Family | Result | Fix |
|---|---|---|
| en1990 | 1/1 | 6 example tests were unmounted |
| en1991 | 2/2 | `🧬️subjects` unit test unmounted |
| en1993 | 3/3 | `heb240-compliant` test unmounted |
| en1994 | 3/3 | |
| en1995 | 11/11 | |
| en1997 | 0/0 | `demo` test unmounted |
| en1998 | 6/6 | |
| en1999 | 1/1 | |
| din4108 | 4/4 | |
| din16798 | 0/0 | no example tests |
| vdi3805 | 4/4 | |
| din18599 | 14/15 | `cooled-office` asset claimed cooling failures it did not produce. Setpoint is now 24 °C and the south facade has 30 m² of unshaded glazing; see `din18599-cooled-office-example-2026-11-22.md`. Now 15/15 |
| iso16757 | 3/4 | The `demo` test demanded `complies()`, although `iso16757.5.6.1.ifcView` is always unverified. It now asserts no failing checks and only that unverified check; see `iso16757-demo-example-2026-11-22.md`. Now 4/4 |
| en1992 | 5/7 | The `compliant` examples produce 2 and 10 failing checks; see `en1992-compliant-examples-2026-11-22.md` |
| en1996 | 0/2 | The compliant wall does not pass, and a failing check has no remedies; see `en1996-examples-2026-11-22.md` |

Mounting all example tests: see `norm-example-test-mounting-2026-11-22.md`.

### Example tests after mounting

The verification agents' reports are `norm-example-verification-2026-11-23.md` and `en1999-compliant-purlin-2026-11-23.md`.

| Family | Tests | Result | What changed |
|---|---|---|---|
| en1990 | 14 | pass | |
| en1997 | 3 | pass | Pile investigation depth raised to 20 m in the `demo` and `compliant` assets and their pack twins |
| din16798 | 3 | pass | New tests |
| en1996 | 2 | pass | |
| en1999 | 2 | 1 fail | The compliant purlin failed 10 checks; the asset is being redesigned |

### Describe fuel regression (`norm-eurocode`)

`describe()` exhausted the 32e9 fuel cap. The cause was the earlier fix that moves `publish_declared_catalogs()` before `extend_contributions()`. After it, mutation-input `$ref` lookups resolved for real. Each lookup ran `registered_input_schema_document`, which scanned and re-parsed every matching JSON schema text, so the cost grew as mutations × documents.

That lookup is now a process-wide `$id` index in `🧰️framework/🔨️modules/🛂️manifest/🔡️mutation-inputs/🦀️.rs`, with the test `registered_lookup_is_repeatable_and_indexed`. The crate passes 12/12. `norm-eurocode` was rebuilt and published at 00:51 on 2026-11-23. Details: `describe-fuel-norm-eurocode-2026-11-22.md`.

The interpreter file `🧠️interpreter/🏛️instance-state/🦀️.rs` holds a temporary sampler in the git index, although its working tree is clean. The archivist must re-stage that path before committing.

### Component rebuilds, 2026-11-23

| Component | Result |
|---|---|
| `norm-eurocode` | Published at 00:51 on the second attempt; the first ended with exit 75 |
| `norm-eurocode-materials` | Not published after three attempts: exit 75, then lease timeouts (`resource-directory-admission`), then a compile error in another agent's `begin_original_settlement` change. That agent has since fixed its callers |
| `norm-eurocode-structural` | Published at 05:11 and includes the EN 1992 asset fix. It still contains another agent's work-admission `[DEBUG]` prints, which have since been removed from the source |

### First 15-route sweep, 2026-11-23 05:21

Strict Play probe, same flags as above. Log: `🗑️generated/routes-sweep.log`. 12 of 15 routes pass:

- **Pass:** din4108, din16798, iso16757, vdi3805, en1990, en1991, en1997, en1998, en1992, en1995, en1996, en1999.
- **din18599 fails.** The shell never publishes its ready state. The faults are `shell.documentTransfer.load-failed norm-energy#1 original-turn.native-return-refused: semantic`, and the `setActiveExample` input is refused with the same error. See `din18599-route-refusal-2026-11-23.md`.
- **en1993 and en1994 fail.** Every fault is a `[DEBUG] admits/admit/end/scope` print from another agent's work-admission diagnostics compiled into the 05:11 structural core: 10737 faults for en1993 and 7146 for en1994. Those prints are no longer in the source, so a rebuild of `norm-eurocode-structural` clears them.

The en1995, en1996 and en1999 routes ran against the materials core from 2026-11-22.

### Example crates after the reruns, 2026-11-23

| Family | Result |
|---|---|
| din4108 | 4/4 |
| din18599 | 15/15 |
| iso16757 | 4/4 |
| vdi3805 | 21/21 after the crate split; see `vdi3805-source-boundary-2026-11-23.md` |
| en1997 editor | The `integration` target does not compile; see `en1997-editor-test-2026-11-23.md` |
| en1991 | 3/3 after regenerating the `de-office-compliant` and `multi-fail-noncompliant` assets and pack twins, which had an empty `cases` block and stale `NUM` columns; see `en1991-subjects-test-2026-11-23.md`. `norm-eurocode` must be rebuilt |
| vdi3805 failing examples | `blatt-5-fail`, `blatt-16-fail` and `blatt-19-fail` produce fewer than two failing checks; being fixed |

### Structural rebuild and second probe, 2026-11-23 07:17

The 07:10 `norm-eurocode-structural` build fixes the `[DEBUG] admits` noise. All three structural routes, en1992, en1993 and en1994, now fail with the same `original-turn.native-return-refused: semantic` refusal as din18599. The refusal happens at document load. This is a regression in the current tree between about 04:15 and 06:30, not only a stale guest; see `din18599-route-refusal-2026-11-23.md`.

### `norm-energy` rebuild, 2026-11-23 09:13

`norm-energy` was rebuilt from the current tree and published on the first attempt. The din18599 route still fails: the shell never publishes its ready state, with the same `native-return-refused: semantic` fault. So a fresh guest does not fix it, which confirms the host-side regression. `chain-4.sh` re-runs the replay crate tests every 15 minutes. Once they pass, it rebuilds the four affected components, sweeps all 15 routes and runs the official plugin test.

### EN 1993 examples with real assertions, 2026-11-23 10:00

4 passed, 0 failed, verified in `🗑️generated/en1993-assert.log`. The `heb240-compliant` test now asserts no failing check and at least one passing check. Earlier runs had failed to compile because the examples crate lacked a dev-dependency on `semio-s-artifact-norm-en1993-io`, which is now added. No `[DEBUG]` dumps remain in the en1993, en1994 or en1998 example tests.

### VDI 3805 family, 2026-11-23 10:43

31 passed and 0 failed across the four crates (`examples`, `examples-blatt-3-to-8`, `examples-failing`, `failing-blatt-4-to-19`), verified in `🗑️generated/examples-vdi3805.log`. The `blatt-5-fail`, `blatt-16-fail` and `blatt-19-fail` assets now drop the mandatory identity record (family 100). Each one fails both the record-count check and the Part 1 structure check, and each failing check has a remedy; see `vdi3805-source-boundary-2026-11-23.md`. These assets ship in `norm`, so `chain-5.sh` rebuilds `norm` after chain 4 and re-probes the four base routes.

### EN 1999 compliant purlin, 2026-11-23 11:00

The examples crate passes 4/4, verified in `🗑️generated/en1999-examples.log`. The final `aluminium-roof-purlin` design:
- Span 5 m.
- R30 fire with a 30 mm protective board: member temperature about 143 °C, utilisation 0.11.

The asset, its pack and JSON twins, and the reference snapshot are regenerated from one snapshot. New tests assert that the DSL and both twins equal it. Report: `en1999-compliant-purlin-2026-11-23.md`.

At 10:08 `semio-s-artifact-norm-en1999-mutations` did not compile: 28 serde trait errors on the new fire-insulation and sheet-profile commands. That blocks the editor crate and the materials component. The fix is tracked in `en1999-mutations-compile-2026-11-23.md`.

### EN 1997 editor and `semio-hub-norm` depth limit, 2026-11-23 11:20

`semio-hub-norm` and the en1997 editor `integration` target now compile. The futures in the `dyn_enum` delegate macro and in the `artifact-app-laws` fixture drivers are now boxed, so the compiler's type-layout query stops at the pointer; the recursion limit was not raised.

Run 16 results: integration 41/51, up from 29; lib 17/31, up from 22.

Fixed:
- The four en1997 assets and their packs were regenerated: `soil-type` is now ENUM.
- The command wire hex was re-pinned in all 12 families, because `Evaluate` was removed from the command enum.
- The history law fixtures root was fixed in 14 families.
- The ownership orphans `gi-01.points` and `gi-01.spacing` were fixed.
- A stale NDP read leak in the incremental laws was fixed.
- The Results and Viewport window configs now declare the disposer birth.

Remaining in the en1997 content:
- mutation schema labels and UI metadata
- 40 remedy-law findings
- a scene `layer-exists` violation
- live acceptance

Remaining in other agents' in-flight framework work:
- JSON parse work quote not admitted
- a leftover allocation counter
- the history driver close

Details: `en1997-editor-test-2026-11-23.md`.

RESULTS_PENDING
