# Terra Run Consolidated Current-HEAD Gate Revalidation

## Requested Gate

- Required HEAD: `5a1367dfcc90630c52dc2ec4de9526babe8d70f4`.
- Required Run component SHA-256: `2ab7ef2edcd150706e9165238e039d6274998087907f6848fdb7a3ce2324f57f`.
- Required Run binary SHA-256: `cd68b4d384ea8bcc675d38d4bab16c71fbaed860e8190d3fdf968fc5249444ec`.
- The gate applies only to the pinned Run component and binary paths; unrelated concurrent worktree state is quarantined.

## Evidence Captured Before This Record

- Current HEAD is `5a1367dfcc90630c52dc2ec4de9526babe8d70f4`, matching the required HEAD.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️component.rs` SHA-256 is `2ab7ef2edcd150706e9165238e039d6274998087907f6848fdb7a3ce2324f57f`, matching the required component fingerprint and `HEAD:<path>` content.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/📦️bin.rs` SHA-256 is `cd68b4d384ea8bcc675d38d4bab16c71fbaed860e8190d3fdf968fc5249444ec`, matching the required binary fingerprint and `HEAD:<path>` content.
- Both scoped Run paths have no ordinary or cached diff and no porcelain status.
- The broader worktree had concurrent entries, which are outside this isolated package gate.

## Direct Run Check

- Invoked once:

  ```text
  cargo check --manifest-path 🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/📦️packages/🦀️rust/Cargo.toml
  ```

- The command exited `101` after 4.41 seconds, before the Run target compiled.
- Exact external blocker: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:2707` patterns `AppFrame::Error { in_reply_to, fault }`, but `AppFrame::Error` now requires the `report` field. Rust emits `E0027` (pattern does not mention field `report`) while compiling the external `semio-framework-plugin-host` dependency.
- The check did not reach or pass Run, so the conditional direct Run library test was not invoked.

## Post-Check Integrity

- HEAD and both required Run SHA-256 fingerprints remain exact matches after the failed check.
- The scoped Run component and binary still have no ordinary or cached diff and no porcelain status.
- No source was edited; only this ticket record was updated.

## Prior Records Reviewed

- `📓️terra-run-file-media-cache-one-consumer-inline-acceptance.md`
- `📓️terra-run-test-only-memory-media-cache-acceptance.md`
- `📓️terra-compiler-run-consolidated-head-revalidation.md`
