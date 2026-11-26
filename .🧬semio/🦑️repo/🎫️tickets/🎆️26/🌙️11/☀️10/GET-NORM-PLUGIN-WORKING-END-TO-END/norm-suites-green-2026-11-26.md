# Norm suites run, 2026-11-26 (agent norm-suites)

## JS/TS contracts (repo test-file, bun)
- Plugin `✏️s/🔌️plugins/📕️norm/🧪️tests`: 29 files, 28 passed in the first run, about 12,070 test cases, 0 failing assertions. wire-twins alone ran 11,956.
- `🧬️schema-typecheck` was the one file reported failed with no output (host memory pressure during a 175 s run). A solo rerun passed (1 test, 26.7 s). No source defect found, nothing changed.
- `✏️s/🧑‍💻dev/📕️norm/🧪️tests` and `🌎️hub/🧩️compositions/📕️norm/🧪️tests/🔬️rust-package-commands`: both files pass (0 test cases, load-only).
- The 12,093 / 49-file total in the history includes the artifact TS packages, which were not rerun here.

## Rust
- `repo show project` and `show projects` refuse without a sealed graph (policy current, no inference allowed), so there was no target discovery.
- `repo cargo test -p semio-hub-norm` from the root failed: the package is not in the root workspace. Run it with `--manifest-path 🌎️hub/🧩️compositions/📕️norm/📦️packages/🦀️rust/Cargo.toml`.
- Two attempts at that command were admitted at 1536 MB. Both ended in the host warden cancelling the cargo ingress helper (a bun --eval process of about 1.1 GB) after proc-macro2/syn started compiling. The cause was critical host memory pressure while another agent's native-exec (2-3 GB) and command jobs were running. Nothing compiled to completion, so there are no Rust results.
- This is not a Norm defect. It is host contention with other agents' jobs, and the plugin workspace Rust crates were not run.
