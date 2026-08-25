# P9-A1 Owned ABI Kernel Second Independent Audit

## Verdict

**GREEN — accept the four-P0 remediation for P9-A1.** The owned ABI kernel now satisfies the original P9-A1 byte/message contract and the three blockers from the first independent audit: retained cancellation is terminal for writer progress, generation identities cannot wrap or alias, and direct-indexed ledgers plus reader admission preflight remove the unbudgeted/pre-admission paths.

This was a read-only production audit. It changed no production source, manifest, lockfile, package script, Nx configuration, Wasm target, browser state, or product file. Ticket-local audit executables and this report are the only audit outputs.

## Scope Compared

- P9-A scout: `📓️sol-high-p9a-rust-browser-wasm-abi-packet-scout-2026-08-25.md`
- implementation report: `📓️p9a1-owned-byte-message-abi-kernel.md`
- first RED audit: `📓️codex-p9-a1-owned-abi-kernel-independent-audit-2026-08-25.md`
- implementation: `🧰️framework/🔨️modules/🌉️abi/🦀️component.rs`, schema, ledger, limits, and the two added mount/re-export lines in `🧰️framework/📦️packages/🦀️rust/📦️glue.rs`

## Reproduced P0 Traces

| Required trace | Independent evidence | Result |
| --- | --- | --- |
| Cancellation after admission and after partial copy | Permanent law offers `credits`, copies 2 bytes, then returns exactly the original 7-byte page, admitted credits `7`, copied count `2`; later `write_step` returns `Cancelled` and retained bytes remain `cr`. The separate hostile harness offers `page`, cancels, then proves `write_step` is `Cancelled` with no copy. | GREEN |
| Incremental close and terminal witness | Writer retirement is budget-limited (`Advanced(1)`, then `Complete` in the partial-cancel trace); reader and writer laws prove interrupted close does not mutate and terminal empty close is idempotently `Complete`. | GREEN |
| Generation exhaustion/no ABA alias | Reuse increments with `checked_add`; closing a `u32::MAX` slot quarantines it. The near-max law reaches maximum, retires it, and rejects a fully quarantined 64-slot table with `Err((GenerationExhausted, 999))`. | GREEN |
| Direct handle/reply lookup | Handle reuse uses a free-slot stack and indexed `get`/`get_mut`; the reply ledger selects exactly `request_id % 256`. Production-source scan found no `iter`, `find`, or wrapping increment. Active collisions return `Busy`; loss, late, cross-generation, and duplicate replies are covered. | GREEN |
| Reader preflight/non-consuming rejection | `read_step` calls budget admission before reservation, target-length assignment, cursor movement, or copy. The law checks zero credit, cancellation, interruption, and deadline each leave `(source_cursor, target_len, staging.len, staging.capacity) == (0, 0, 0, 0)`. | GREEN |

## Executed Gates

| Gate | Result |
| --- | --- |
| `rustfmt --check` on A1 component | GREEN |
| direct dependency-free `rustc --edition=2021 --crate-name semio_framework_abi_second_audit -D warnings --crate-type lib` | GREEN |
| direct debug test | GREEN — 14 passed, 0 failed |
| direct optimized test (`-O`) | GREEN — 14 passed, 0 failed |
| independent hostile cancellation harness (`-A dead_code -D warnings`) | GREEN — 15 passed, 0 failed |
| Bun standard-library schema/fixture parser | GREEN — 15 schema definitions, 8 reconstructed ledger records, 10 consecutive max/max+1 laws |
| owned-module external ABI/runtime deny-list | GREEN — zero `JsValue`, `Promise`, `Function`, `web_sys`, `js_sys`, `wasm_bindgen`, or serde runtime matches |
| exact added glue-line deny-list | GREEN — zero prohibited matches |
| public-surface review | GREEN — public A1 signatures are primitives, standard owned containers, or owned A1 types; `AbiPort` exposes only owned messages/budgets/results |
| production scan/collision/order review | GREEN — no wrapping increment or linear lookup; canonical version/tag/order/little-endian ledgers reconstruct exactly |
| tracked glue `git diff --check` plus no-index checks for the four untracked A1 files | GREEN |
| A1-scoped forbidden-file status | GREEN — no Cargo/package/lock/project/script/launch/product path in the A1 implementation scope |

`rustc` received explicit ASCII crate names because the repository’s emoji filename is not a valid inferred Rust crate name; this is only a direct-compiler invocation detail, not a diagnostic from the component.

## Contract Review

The schema remains version `1`, fixed little-endian, `u32`-length, declaration-order wire data. The parser reconstructed request, reply, event, both page examples, and all control subtags from the fixture rather than trusting only row counts. The ten fixed limits agree with code and fixture values, including the `u32::MAX + 1` generation-exhaustion law. Maximum-plus-one constructors retain the rejected owned allocation/page; successful reader admission reserves only the permitted copied byte extent.

The two glue additions mount and re-export the owned module only. They introduce no browser/runtime type and do not broaden the domain contract.

## Scope Note

The shared worktree contains unrelated concurrent modifications, including product and script paths. The P9-A1-scoped status contains only the owned module/schema/fixtures, the two glue lines, and ticket-local material; it contains none of those forbidden path classes. No Cargo, Nx, Wasm, or browser command was run.

## Blockers

None for P9-A1.
