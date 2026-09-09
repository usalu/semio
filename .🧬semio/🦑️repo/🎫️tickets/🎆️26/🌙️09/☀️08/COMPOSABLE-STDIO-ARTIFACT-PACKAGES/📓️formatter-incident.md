# Formatter Incident

## 2026-09-09 10:11 CEST

This was the second formatter-expansion incident recorded for the ticket. An attempted exact-file formatting command incorrectly included `cargo fmt --all --` before the three intended Semio paths. Cargo expanded the invocation to the Rust workspace instead of limiting rustfmt to those paths. The earlier distinct `cargo fmt -- <file>` incident remains summarized in `📓️multi-artifact-execution.md`.

The command shell was PID 59812, its Cargo child was PID 59816, and rustfmt was PID 59887. The shell and rustfmt were first sent `TERM`; after rustfmt remained live, Cargo and rustfmt were sent `KILL`. A process snapshot then confirmed PIDs 59816 and 59887 were absent.

The erroneous command prefix was:

```text
cargo fmt --all -- '.../📦️object/🧬️schema/📸️snapshot/🦀️.rs' '.../🧰️kit/🧬️schema/📸️snapshot/🦀️.rs' '.../🖊️drawing/🧬️schema/🧬️mutations/💾️binary/🦀️.rs'
```

The first post-`TERM` process snapshot showed the shell defunct and rustfmt still running:

```text
59812 11644 Z 00:31 <defunct>
59887 59816 R 00:29 .../bin/rustfmt <workspace Rust inputs> --edition 2021 <three intended paths>
```

The follow-up `kill -KILL 59887 59816` command exited 0. Its immediate `ps -p 59816,59887` emitted no rows.

The incident was bounded before preserving work. These checks were observed in the terminal transcript; they were not redirected to a separate raw receipt:

- A repository-wide Rust-file modification-time query for the incident window produced no unexpected Rust paths.
- A narrowed five-minute query below the Semio artifact root listed only the three intended files, whose changes preceded the formatter attempt.
- Representative early workspace input `✏️s/🔌️plugins/✒️writer/📦️packages/🦀️rust/🦀️.rs` retained its 2026-09-09 06:52:01 modification time.
- The intended Object snapshot retained its 2026-09-09 10:10:54 modification time from the preceding patch.
- No revert or other modifying Git command was used, so concurrent edits were preserved.

The three owned paths were subsequently formatted one at a time with direct `rustfmt --edition 2021 <exact-path>`; all three commands exited 0. A path-scoped `git diff --check` also exited 0.
