"""📝 Adds the x1 (cargo lease suites) finding, the tsdebt part-2 note and the x1 rows to result-runner.md (one-shot ticket helper)."""
import os
p = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "result-runner.md")
t = open(p, encoding="utf8").read()
anchor = "\n## Verdicts"
add = ("FOR coordinator: tsdebt GREW (part 2, 01:20; same 9 files, `edit_ingress.py`): `framework-process:test-cargo-build-custody` RED (rn-x1 00:48, rn-x1b 00:55, reproducible) — the raw-cargo negative-control fixture's `cargo build` exits 101 because "
       "its scratch lies below the checkout and the repository `.cargo/config.toml` (`rustc` = the compiler-ingress script, `rustc-wrapper = \"repo\"`) refuses it: \"Original compiler ingress requires the repository launcher and authentic enforcing store\" "
       "(reproduced by hand with the pinned nightly). Fix: the custody and unit-exclusion fixtures pin `RUSTC=<toolchain>/rustc` and `RUSTC_WRAPPER=\"\"`. Mirror proof with the scratch TMPDIR inside the mirror (`C/runner-tools/mchain.sh`, toolchain linked read-only): "
       "custody negative control red on base → green; unit-exclusion 0/1 → 1/1; tsdebt verify law1 0 / law2 0 again, scan 8/0. Still open in that suite: 'a queued custody wait under a real issued admission control…' settles within 1.5 s under the official envelope (not reproduced on the mirror, undiagnosed). "
       "Note: one mis-scoped helper call (red-g-tools/mrun.sh with an overridden TMPDIR) ran the custody file once from the live cwd at ~01:05 (scratch in the mirror, no tracked file written).\n")
t = t.replace(anchor, add + anchor, 1)
t = t.rstrip("\n") + "\n" + ("| rn-x1 / rn-x1b | framework-process :: test-cargo-build-lease,test-cargo-build-custody,test-resource-leases | 00:48 / 00:55 | 65 s / 60 s | 1,720 MB | 1 | RED `test-cargo-build-custody` 3/5: negative control (ingress refusal → tsdebt part 2) + queued custody wait settles early (open); lease suite and resource-leases did not run behind it (`--parallel=1`) |\n")
open(p, "w", encoding="utf8").write(t)
