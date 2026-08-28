# Runtime R51 — Cancelled Accidental Coverage Routing

The intended exact runtime selector was launched with `exhaustive` but I omitted the established `SEMIO_COVERAGE=0` override. The task therefore selected cargo-llvm-cov and a release coverage build beneath the existing master target, rather than the intended existing native test profile. No runtime test executed and no result is claimed.

After reading the actual process tree and target paths, I interrupted only this invocation's exact Cargo/nextest/coverage processes (4993, 4930, 4844). The remaining compiler children (5574, 5571, 5570) finished before the subsequent interrupt attempt; the next process census found no compiler remaining for this target. Session 49227 exited 1. Existing files and caches were preserved; no cleanup was performed. No unrelated peer process was targeted.

The next exact gate restores explicit `SEMIO_COVERAGE=0`. Raw output remains `🧪️member-runtime-canonical-grants-r51-native-2026-08-27.txt`.
