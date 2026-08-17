# Engine Host Trait Dissolution Acceptance

## Baseline

- HEAD: `0727b80aa6a802cac1760f90fb7a148f74035413`.
- Coordinator-owned root `📜️script.ts` matched post-update SHA-256 `234006e405c100984edc6ec21cf055aaa35879ea9addd3f1844613cd819c98d8` and is read-only for this lease.
- `🧰️framework/🛍️products/💻️os/🔨️modules/⚙️engine/🦀️component.rs` matched SHA-256 `f0d51e0eca997b00df0f4c346064a80c3edd2059aa02200a241c7dae39487b8b`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts` was clean at SHA-256 `4d0943c9f4a18dd9f31814458a3930225bdb0e8eb30dc57959cd10ad5e924407`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖍️drawing/🦀️component.rs` was clean at SHA-256 `bb095112a0f408733e9c658dfed5fd21f3c348677c728ec03f0f15d3a32ebb14`.

## Implementation

- Deleted the zero-consumer `EngineHost` trait only. `Engine`, `EngineHandles`, `EngineCache`, and their behavior remain unchanged.
- Preserved the `BrepEngineHost` and `EngineCache` keys and detector patterns in the OS dev scanner, while replacing stale trait/implementation descriptions with concrete host-wrapper descriptions.
- Replaced Flow drawing’s speculative host-injection wording with the concrete WIT `engine-derive`/`engine-read` guest↔host boundary.
- No root script, plugin, WIT, stdio, glue, Cargo, lock, manifest, registrar, or compatibility source changed.

## Validation

- Active-scope `EngineHost` search, excluding compose, tickets, history, build output, dependencies, and Nx output, returned no matches.
- Ordinary and cached scoped `git diff --check` validations exited `0`.
- `bun nx run @semio-tech/framework-os-kernel:test-quick --skip-nx-cache` exited `0`: `904 passed`, `0 failed`, `0 ignored`, `0 measured`, `0 filtered out`.
- `bun nx run @semio-tech/framework-os-dev:host-handle-lint --skip-nx-cache` exited `0`. It retained its expected report-only `BrepEngineHost` finding for the CAD plugin; its `BrepEngineHost` and `EngineCache` detection is unchanged.
- No taxonomy or census was run.

## Final State

- HEAD remained `0727b80aa6a802cac1760f90fb7a148f74035413` throughout this lease after the coordinator’s prior advance.
- Engine SHA-256: `4cb1df39e581efa301ab41ec73a17a63631609d8621ebb84ff3189aadd94f5a1`.
- OS dev script SHA-256: `1314901d3d48164463f9699f288d91295d44666c1dd85d704f40b1d25612830a`.
- Flow drawing SHA-256: `d870df9fa6e23dfaf013854559d6e618a1257888798b2d0ec9d2f485b42c90bc`.
- Cached source diff is exactly engine `0` additions / `6` deletions, Flow drawing `3` additions / `3` deletions, and OS dev script `7` additions / `10` deletions.
- The three leased paths appeared index-staged after their clean baseline. No Git-mutating command was used; that externally controlled index state was preserved.
