# 📓️ Lease request — 3 pre-existing `unused_qualifications` warnings in `🧭️protocol/🦀️component.rs`

**Filed by**: terra, packet P1b-http-handles-bridge
**File**: `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧭️protocol/🦀️component.rs` — owned by P1a, frozen
for P1b (`📌️sol-P1b-packet.md` §1: "Do NOT touch `🧭️protocol`, `⚠️errors`, `🧬️schema`").

## What was found

Running the REAL workspace build for the first time (P1a's own acceptance was a standalone
throwaway-workspace build, per its report §4 — the crate had not yet become a workspace member when
P1a finished) surfaces 3 `unused_qualifications` warnings the standalone build never saw, because the
standalone `Cargo.toml` did not carry this repo's `[workspace.lints.rust] unused_qualifications =
"warn"` setting:

```
warning: unnecessary qualification
   --> 🧭️protocol/🦀️component.rs:1013:51  (super::META_PROTOCOL_VERSION_KEY)
warning: unnecessary qualification
   --> 🧭️protocol/🦀️component.rs:1014:37  (super::tests_support::request_with)
warning: unnecessary qualification
   --> 🧭️protocol/🦀️component.rs:1018:37  (super::tests_support::request_with)
warning: unnecessary qualification
   --> 🧭️protocol/🦀️component.rs:1022:43  (super::tests_support::request_with)
```

(4 call sites, all inside `mod long`'s `a_full_modern_session_lists_reads_and_subscribes_resources_end_to_end`
test — `super::` is redundant because the items are already in scope via `use super::*;` at the top of
`mod long`.)

## Requested change (mechanical, zero behavior change, test-only code)

In `🧭️protocol/🦀️component.rs`, inside `mod long`'s
`a_full_modern_session_lists_reads_and_subscribes_resources_end_to_end` test, replace:
- `super::META_PROTOCOL_VERSION_KEY` → `META_PROTOCOL_VERSION_KEY`
- `super::tests_support::request_with` → `tests_support::request_with` (3 occurrences)

This is a pure whitespace/path-prefix trim on already-in-scope names inside test code; it changes no
signature, no behavior, no other file. Blocks P1b's `cargo build -p semio-framework-os-mcp 2>&1 | grep
-c "^warning"` → 0 acceptance criterion, since the criterion is crate-wide and this file compiles into
the same crate.

**Status: pending as of this report** — P1b's own acceptance output (§3 of `📓️terra-P1b-report.md`)
reports the actual warning count including these 4, with this lease noted as the reason, rather than
silently editing a file outside `path_scope`.
