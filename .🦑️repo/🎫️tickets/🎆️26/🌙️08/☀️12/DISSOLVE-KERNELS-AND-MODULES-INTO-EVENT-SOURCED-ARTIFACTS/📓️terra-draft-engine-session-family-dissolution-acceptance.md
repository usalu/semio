# Draft Engine Session Family Dissolution Acceptance

## Baseline

- HEAD: `07873f842a5a99ac2f69c1648c21f36ebf260bdb`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/⚙️engine/🦀️component.rs` was clean at SHA-256 `32b7ecd98ddbb9858e91a7060bd72b299c7fb1dcb3245ac92e9c709042eb7237`.
- `🧰️framework/🔨️modules/🗺️surface/🎨️paint/🦀️component.rs` was clean at SHA-256 `74b9544a145819e73bbc92516fba3072335c7ef903906de56987498bc4888e52`.
- Production referrer evidence identifies no caller for `DraftBaseHash`, `DraftEngineSessionStats`, or `DraftEngineSession`; same-file tests and the paint documentation analogy are excluded.

## Implementation

- Removed the complete `🔖️DraftEngineSession` region: `DraftBaseHash`, `DraftEngineSessionStats`, `DraftEngineSession`, and their private cache/session logic.
- Removed the base-hash helper and all draft-session-only tests. The independent `EngineRep` fixture and `engine_rep_build_is_deterministic` test remain.
- Removed the `EngineRep` documentation cross-reference to the deleted session family.
- Rewrote the paint scratch-buffer documentation directly: it is droppable at any instant without losing committed user data.
- Cache, handles, `EngineHost`, `EngineRep` runtime behavior, and all non-leased source paths are unchanged.

## Validation

- Repository-wide stale-symbol search for `DraftBaseHash|DraftEngineSessionStats|DraftEngineSession`, excluding ticket/history, returned no matches.
- Ordinary and cached scoped `git diff --check` validations exited `0`.
- `bun nx run @semio-tech/framework-os-kernel:test-quick --skip-nx-cache` exited `0`: `904 passed`, `0 failed`, `0 ignored`, `0 measured`, `0 filtered out`.
- No taxonomy or census was run: the structural engine scope remains mixed and is not released by this deletion lease.

## Final State

- Engine source SHA-256: `f0d51e0eca997b00df0f4c346064a80c3edd2059aa02200a241c7dae39487b8b`.
- Paint source SHA-256: `1aed5375e89371234006bc6c57d34159338f59a9e92cfebda4536ca5dfa6df87`.
- Cached source diff is exactly engine `0` additions / `146` deletions and paint `1` addition / `2` deletions.
- Both leased paths appeared index-staged after the edit despite their clean baseline. No Git-mutating command was used; that externally controlled index state was preserved.
