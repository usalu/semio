# Tolerance and dry run (wave 2)

Nothing compiled; Python via heavy.sh.

## 1. Shared utilisation rule
- Shared (`⚖️compliance/✅️check-results/🦀️.rs`): new `within_limit(u) = u <= 1 + UTILIZATION_TOLERANCE` and `limit_status(u)`. `CheckBuilder::utilization` and the final stations promotion use them (was strict `<= 1.0` / `> 1.0`, so 1 + 1e-12 failed); `minimum` passes when `computed >= minimum` or the inverse ratio is within the limit (minimum > 0 only).
- Python: same constant plus `within_limit`/`limit_status` in the shared helper `🔮️oracles/🏃️harness/📏️tolerance/🐍️.py`; oracles load it by walking up to `🔮️oracles/🏃️harness` (`LIMIT`). Replaced: en1998 check-outcome (local constant; qLimit), en1996 compliance `relocation` (`1 + 1e-9`), en1999 compliance and check-outcome (`u > 1` status rows, 3 places).
- Rust: en1998 `direction-and-base-shear` uses `within_limit(q / q_max)`; en1994 beams eta tone and en1995 diagrams provided-vs-required tone use `norm::UTILIZATION_TOLERANCE` (were absolute `1e-9`).
- Kept unchanged: geometry, convergence, time-step, quadrature and grid epsilons, vdi3805 `kvs + 1e-12`, en1994 geometric limits (`thin`, `welded`, `ductile`), en1991 `assumed + 1e-9 >= required` (a catalogue input comparison, not a utilisation row).
- Boundary test: `⚖️compliance/🧪️tests/🎚️utilization-boundary/{🔣️.json,🐍️.py}` (1.0 and 1+1e-12 and 1+5e-10 pass; 1+2e-9 and 1+1e-6 fail); Python run 7/7 ok; Rust twin `utilization_boundary_cases_from_the_shared_fixture` in `🔬️unit/🦀️.rs` reads the same JSON (drives `limit_status` and the builder). Not compiled.

## 2. en1998 mutate dry run
Cause was the dry-run tool, not a missing handler: the `mutate` oracle adapter (`semio_norm_vocabulary`, `🔮️oracles/🏃️execution/🐍️.py`) already registers `mutate-<id>`/`inverse-<id>`; the harness looked up `outlineOf` (`mutate`) before the scenario id, did not put the execution folder on `sys.path` and did not resolve `asset://`. Fixed in `🧰️scripts/wave2-features-dryrun.py` (id first, outline fallback; execution path; `asset://` to `any/🖼️assets`). Result: en1998 mutate 579 ok, 0 failed (was 0 ok, 579 NO HANDLER); check-outcome 56/56, oracle-comparison 3/3, round-trip 3/3.

## 3. seismic-rc-frame round trip
`identity-round-trip` passes now (re-print byte-identical and reparse equal): the earlier failure was the harness not resolving the `asset://` DSL URI, the asset itself is current (carries en-ground-type/en-spectrum-type). No asset change needed.

## Side findings (other owners)
With the fixed harness: en1990 mutate 1 failure (`mutate-change-reliability-action-dist`: document did not move), en1995 mutate 2 failures (`inverse-change-deck-compression-c90`: forward mutation refused, plus one more). Check-outcome dry runs: en1990 84, en1996 40, en1999 119, all ok. Self-checks en1998/en1999 oracles run.

## Open
- Scene tone helpers `ratio > 1.0` (en1991/1993/1996/1997 diagrams, en1996 `kit.rs` status) still strict; they live in scene crates, decide with shared whether to route through `within_limit`.
- en1997 geotechnics self-check `utilization <= 1.0` left.

- ready for rebuild: tolerance-and-dryrun (en1994, en1995, en1996, en1998, en1999 latest.md appended)
