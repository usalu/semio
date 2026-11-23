# 🧭️ Scene Surface Compile Sweep

Status: PARTIAL. Sweep stopped after 4 of 265 unchecked plugin workspaces. No migration errors found so far.

Command: the requested `cargo check --manifest-path <ws>/Cargo.toml --workspace --lib` with the shared native-target, nightly-2026-07-20. Full logs are in `🗑️generated/sweep/<n>.log`. Errors were extracted with `grep -E '(^|: )error'` because the requested `^(error|warning: unused)` pattern misses rustc short-format lines (`path:line:col: error[...]`).

| # | Workspace (under ✏️s/🔌️plugins/) | Result | Migration errors | Foreign errors |
|---|---|---|---|---|
| 1 | ✒️writer/🏭️bridge | FAIL | none | `error: no library targets found in package semio-writer-mutation-bridge` (binary-only package with `--lib`) |
| 2 | ➗️mathematical/🏭️bridge | FAIL | none | `error: no library targets found in package semio-mathematical-mutation-bridge` (binary-only package with `--lib`) |
| 3 | ➗️mathematical/🗿️artifacts/➗️equation/…/🧩️json/📦️packages/🦀️rust | OK | none | none |
| 4 | ➗️mathematical/🗿️artifacts/➗️equation/…/🔬️probes/📖️reader/📦️packages/🦀️rust | FAIL | none | `error: no library targets found in package semio-equation-oracle-probe` (binary-only package with `--lib`) |

Side effect (needs a decision): cargo rewrote tracked `Cargo.lock` files for workspaces 1, 2, 3 and 4 at 06:02:30 to 06:02:34. Diffstats: writer/bridge 349+/3-, mathematical/bridge 7734+/5865-. Working-tree content before the run is unknown. Restore from HEAD only after the owner confirms. Use `--locked` on future checks to stop lock writes.

Not swept: 261 workspaces. The notes' earlier "~66" figure understated the list.
