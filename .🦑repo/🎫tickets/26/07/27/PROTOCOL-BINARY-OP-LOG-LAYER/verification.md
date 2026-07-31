# Wave 4 — Final verification

Single agent. Prerequisite: Wave 3 complete, `POLICY_PROTOCOL_COMPLETENESS_ALLOWLIST` empty.

## Checklist

1. `cargo build --workspace` clean.
2. `cargo test -p vcs -p protocol_core -p protocol_format -p protocol_history -p
   protocol_materialize -p protocol_io -p protocol -p protocol_testkit -p protocol_cli` — all
   green, including the law-based tests (encode/decode identity, canonical stability, ops⇄binary
   fixpoint, streamed=buffered, resume=continuous, recovery-truncates-to-commit at exhaustive
   tier, chain-detects-tamper, compaction identity, zero-copy pointer sweep,
   materialize-fast-path==full-replay, corruption fuzz with zero panics).
3. `cargo clippy` on every touched crate, clean (or only pre-existing unrelated warnings — note
   which).
4. `bun ./script.ts lint` — protocol-completeness allowlist empty and firing correctly, no
   committed `.spr` files, region-format lint clean on all new crates.
5. CLI smoke test: export a real app document via the os shell (or a vcs test fixture) to `.pack`+
   `.spr`, run `protocol inspect`, `protocol verify --level=full`, `protocol decompile` → `protocol
   compile` round trip byte-identical, `protocol materialize` (or the vcs-level equivalent) prints
   the same projection the app's own dsl round trip produces.
6. Bench sanity (don't need full criterion HTML report, just confirm they run and the shape is
   right): materialize time roughly flat as history grows past a few checkpoints; append time
   roughly constant per edit regardless of file size.
7. `cargo test -p semio-framework-sync -p vcs` (hub/sync convergence tests from Wave 2b) green.
8. Rename verification: grep sweep confirms zero remaining references to the old Blockly-tech
   `protocol` identifiers anywhere except the never-rename list (see rename-instructions.md);
   `playbook` crates build and test clean.

## Report back

Full pass/fail table against the checklist above, and a consolidated **human-todo list**:
- Rewrite `playbook/AGENTS.md` content (frontmatter + stale internal references — AGENTS.md edits
  are forbidden for agents).
- Any items flagged by Wave 2b as needing human review (e.g. browser WS binary-frame client code).
- The ULID/id-collision hazard noted in the design (ids like `edit-N` from a process-local counter
  can collide across processes editing the same document concurrently — pre-existing bug,
  independent ticket, out of scope for this rollout).

Close the ticket via `ticket_close` with an explicit path
(`26/07/27/PROTOCOL-BINARY-OP-LOG-LAYER`) and the full summary + every file touched across all
waves once this checklist is green.
