# Save and Close Verification

Reproduced production failures before fixes: unknown status yielded flushed; cancelled flush asked again; lost status after flush closed the document; retired generation flush accepted discard and closed.

Saved-state now requires positive acknowledgment after a flush, keeps a cancelled close, and checks generation ownership before and after every await. Close requests abort superseded prompts; stale completion cannot clear a newer prompt. Folder detach forgets its binding only after accepted close.

Language-neutral saved-state fixtures include five acknowledgment cases, four close decisions, and two retirement cases. Ajv independently computes acknowledged status. Focused real Vitest: 18 tests across derivation and actual close hook lifecycle passed (see saved-state verification report).

Dock close now awaits document guards before teardown, serializes close gestures, tracks accepted sibling closures until layout catches up, checks latest spawned instance identity, logs close before destruction, destroys before panel removal, and preserves unrelated panel changes. UI async admission is covered by ten fixture-driven tests. Host canvas integration tests are being authored. Canvas TSX transpilation passed.

Broader host suite remains queued under the repository capacity gate, handle 57027. Runtime and Rust compilation remain separately unproven; no end-to-end completion claim.

## Focused Rendered Dock Verification

Actual React Mode rendered in jsdom, language-neutral cases accept/refuse/reject: pending repeated clicks remain two tabs and call the owner once; accept removes exactly the target, refusal and rejected save keep both. Command: SEMIO_LANGUAGE=en repo tools exec --light bun -- node_modules/vitest/vitest.mjs run --config .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️04/WELL-KNOWN-ARTIFACT-EDITOR-COMPLETION/vitest.mode-close.config.mjs. Exit 0, 3 tests passed, 1 file, duration 3.65 seconds.

Host useCanvas integration: 4 passed, verifies Keep has no panel/plugin/log side effects, attachment teardown before destroy, serialized sibling closes invoke final guard, and generation replacement is retained. The focused close coverage is 35 tests total (18 saved-state + 4 host canvas + 10 UI admission + 3 rendered Mode).

The broad suite request was changed from 2048 MB to a declared 768 MB one-worker capacity reservation (session 95925, PID 3261846), preserving the host governor. Original own queued PID 3225813 cancelled by exact PID. No other owners were interrupted.
