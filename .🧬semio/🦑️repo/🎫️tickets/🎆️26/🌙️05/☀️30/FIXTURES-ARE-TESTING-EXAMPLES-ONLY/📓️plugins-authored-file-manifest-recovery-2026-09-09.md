# Plugin Authored File Manifest Recovery — 2026-09-09

## Recovery

The purged plugin manifest was restored byte-for-byte from the read-only Git index entry:

`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION/📓️plugins-authored-file-manifest-2026-09-09.md`

The recovered file is retained at the current ticket root as `📓️plugins-authored-file-manifest-2026-09-09.md`. The private repository MCP retention overlay passed before restoration, so the exact indexed document remains a single file for collector compatibility and existing report links.

## Exact Evidence

- Git blob: `c8e1f7a5565bee64158b52a8b22c320e09291ae2`
- Bytes: `13,622,727`
- SHA-256: `83c55435bec5d6d3dccb636f71ca406bf2ff961f17b8237f5af77a4a29429b68`
- Ledger records: `17,252`
- Unique nonempty source/destination paths: `29,848`
- Create records: `21`
- Modify records: `3,409`
- Move records: `13,822`

## Validation

`git hash-object` on the restored file reproduced the indexed blob ID. A filesystem byte count and SHA-256 reproduced the independently recorded values. Bun parsed the restored JSON block and reproduced all record, path, and kind counts. Recovery used `git show :<indexed-path>`; it did not modify Git, rescan repository source, or change semantic source files.

