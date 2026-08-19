# 📓️ terra — alltargets-green baseline defense report

Packet: `alltargets-green`. `CARGO_TARGET_DIR=/private/tmp/claude-501/-Users-ueli-Documents-semio/e6a44461-bab7-421f-8a53-65123a5e9482/scratchpad/target-at-green` for every command below. All commands run FOREGROUND, `-p <crate>` (never bare workspace cargo).

## Result: no regressions. All six baselines hold.

| crate | `--lib` | `--all-targets` | `cargo test` | test count | notes |
|---|---|---|---|---|---|
| `semio-framework-replication` | EXIT 0 | EXIT 0 | EXIT 0 | **184 passed** | no test-count baseline was given (table only specified exit 0/0) |
| `semio-framework-pack` | EXIT 0 | EXIT 0 | EXIT 0 | **44 passed** | matches baseline exactly |
| `semio-framework-geometry` | EXIT 0 | EXIT 0 | EXIT 0 | **57 passed** | matches baseline; **0 warnings**, matches baseline |
| `semio-framework-math` | EXIT 0 | EXIT 0 | EXIT 0 | **191 passed** | matches baseline |
| `semio-framework-async` | EXIT 0 | EXIT 0 | EXIT 0 | **17 passed** | matches baseline |
| `semio-framework-dispatch-macros` | EXIT 0 | EXIT 0 | EXIT 0 | **28 passed** (22+3+1+1+1 across lib+4 integration test binaries) | matches baseline |

Every exit code above is the **actual cargo exit code**, captured as `$?` immediately after the command with output redirected to a file (never piped through `tail`/`echo` in a way that would report a different command's status). Raw logs: `terra-alltargets-green-<crate>-{lib,all,test}.txt` in this ticket folder.

Test counts were cross-checked two ways per crate: the `test result: N passed` summary line(s) (all binaries, including doctests where present) and a raw count of `^test .* \.\.\. (ok|FAILED|ignored)` lines, which agreed in every case — no hidden `#[cfg(test)] mod tests` unwrapping, no `#[test] async fn` illegal-signature suites silently not compiling.

## Trap checks performed
- **Feature unification trap**: checked each crate's `[features]` table for a non-default feature a consumer might enable that a bare `-p` check would skip. `semio-framework-async` declares `typegen` (off by default, gates `ts_rs::TS` derives) and `testkit`; `semio-framework-pack` declares `default = ["deflate"]` plus optional `ureq` — these were exercised as-is (defaults), matching what a plain `cargo test -p <crate>` runs, which is what the baseline table specifies. Nothing here hid feature-gated code the way the `wgpu` case did for `semio-framework-ui`.
- **Silent-suite trap**: grepped every crate's raw `--lib`/test output for `mod tests` brace balance and for any `#[test] async fn` compile errors; none found. Reporting **unique** counts per binary target, summed, as shown above.

## Finding (pre-existing, not a regression): `typegen` cfg undeclared in `semio-framework-replication`
`🧰️framework/🔨️modules/📡️replication/📡️wire/🦀️component.rs` has 20 `#[cfg_attr(feature = "typegen", ...)]` attributes, but `semio-framework-replication`'s `Cargo.toml` `[features]` table (only `default = []`, `deflate`) never declares `typegen` — unlike `semio-framework-async`, which declares and documents it. This produced 20 `unexpected cfg condition value: typegen` warnings, folded into the 59-warning total below. **Not a baseline violation** (the baseline table specifies exit codes only for this crate, no warning count) and **not something I fixed**: a proper fix needs `ts-rs` added as an optional dependency, which touches the workspace `Cargo.lock` — a registrar-only file outside my `path_scope`. Filing as a `lease-request` below instead of touching it.

## Change made (within owned paths, R7-sanctioned, verified before/after)
Two of the six crates were missing the `#![allow(async_fn_in_trait)]` crate-root attribute that **R7** prescribes verbatim ("Add `#![allow(async_fn_in_trait)]` at crate root, with a one-line comment pointing at R3 and R7") — `semio-framework-pack`, `semio-framework-math`, and `semio-framework-dispatch-macros` already had it; `semio-framework-replication` and `semio-framework-async` did not, and were emitting the `async_fn_in_trait` lint as plain warnings.

Added the attribute + R3/R7-pointing comment to both crate roots:
- `🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/📦️glue.rs`
- `🧰️framework/🔨️modules/⏳️async/📦️packages/🦀️rust/📦️glue.rs`

Did **not** touch the `-> impl Future + Send` suggestion rustc prints (R7 explicitly forbids taking it) and did not make any trait method sync.

**Before → after, re-measured with the same `--lib`/`--all-targets`/`cargo test -p` commands, freshly re-run and exit codes recaptured (not reused from the first pass):**

| crate | warnings before | warnings after | exit codes after | tests after |
|---|---|---|---|---|
| `semio-framework-replication` | 59 (39 `async_fn_in_trait` + 20 pre-existing `typegen` cfg, see finding above) | **20** (all `typegen` cfg, unrelated to this change) | 0 / 0 / 0 | 184 passed (unchanged) |
| `semio-framework-async` | 6 (the exact set R7 itself measured and cited as the reference case) | **0** | 0 / 0 / 0 | 17 passed (unchanged) |

No behavior change (attribute-only), no test-count change, no exit-code change — pure warning-count improvement plus R7 compliance. `semio-framework-pack`, which depends on `semio-framework-replication`, was re-verified after the dependency's edit: `--lib`/`--all-targets`/`test` all still EXIT 0, 44 tests unchanged.

## `lease-request`
```lease-request
file: Cargo.lock (workspace root, registrar-only)
also: 🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/Cargo.toml (in my path_scope, but the fix requires
      adding `dep:ts-rs` as an optional dependency + a `typegen` feature, which touches Cargo.lock)
requested_by: terra (alltargets-green packet)
reason: semio-framework-replication references `#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]` 20 times
        in 📡️wire/🦀️component.rs but never declares the `typegen` feature (unlike the sibling
        semio-framework-async crate, which does). Produces 20 "unexpected cfg condition value: typegen"
        warnings on every --lib/--all-targets build. Not a regression, not blocking the exit-0 baseline —
        flagging so a registrar pass can add the feature + optional ts-rs dep (mirroring async's
        Cargo.toml) in the same Cargo.lock-touching batch as other pending registrar work.
```

## Nothing else regressed
No other file in the six owned modules was modified. No `lease-request` beyond the one above. No sibling crate outside the owned paths was touched.
