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

RESULTS_PENDING
