# Terra P1p Store-Sync Retained Actor-Turn Re-Audit — 2026-08-23

## Verdict

**ACCEPT — source packet only.** The quiet-runner ownership defects in the prior Terra rejection are closed in the live source. This is not an acceptance of Phase 1 or of compiled/runtime/DB-I/O behavior.

## Independent source findings

- `ArtifactHostState` owns each active native runner strongly through `OpenDocument.runner`, and owns a closing runner strongly in its generation-keyed `closing` map. A quiet actor may therefore remain `Idle { deadline: None }` without the last runner owner disappearing. Reopening replaces the active generation; close moves its exact generation into `closing` before requesting close.
- `ArtifactChannels` exposes `ArtifactActorRunnerTicket`, not a cloneable runner handle. The ticket contains only the generation, a `Weak<ActorRunner>`, a returned bit, and an optional host-state `Arc`. Issuance increments the runner ticket count; return is one-shot, upgrades only the matching generation, and returns the exact count. A stale ticket cannot revive or decrement a replacement generation. Terminal completion requires that count to reach zero.
- `ActorRunner` has a strong `self_retained` owner until the terminal-empty predicate is met: no scheduled/retry/mailbox/actor-or-future/terminal-turn work remains and no external tickets remain. Its final retained value is taken only after terminal completion, leaving no deep actor/turn owner in the final drop.
- Host close first removes the active entry, records the strong closing entry, then sends the close request and rechecks already-complete terminal state. The terminal callback removes only its matching generation. Close closes ingress. The native close branch processes at most one `close_one` mailbox owner or one backbone opportunity per actor turn.
- Public `ArtifactHost::closing_runner` exposes a strong terminal-control handle. `ArtifactActorRunnerHandle` provides exact `take_terminal_job`, `resume`, and `close`; the public terminal-job owner restores an unresolved job rather than losing it.
- The prior retained mailbox conditions remain in the live path: fixed 64-item/1 MiB limits, recursive byte preflight and exact full/byte/closed/stale owners, one command or one rotating actor opportunity per `drive_one`, one-poll generation waker, finite generation-coalesced retry, no production Tokio runtime/spawn/block-on, and no status-drain loop.

## Fixture and verifier review

The implementation report's fixtures are present and materially exercise the requested cases: quiet idle then late send, one-shot self-retain wake, ticket held/dropped before and after close, pending detach, public terminal-job take/resume/close, host close, missing handle, strong external ownership, ticket terminal gate, terminal callback race, and generation ABA. The interactivity verifier has corresponding positive conditions and adversarial mutations for missing strong host retention, quiet-runner drop, an exposed strong external channel handle, missing ticket terminal gate, and a missing host terminal callback; these mutations fail the verifier rather than merely matching fixture names.

## Executed structural gates

| Command | Result |
| --- | --- |
| `rustfmt --edition 2021 --check 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🦀️component.rs 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🧊️component.rs` | PASS |
| `rustfmt --edition 2021 --check *.rs` from `🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services` | PASS |
| `bun ./📜️script.ts verify interactivity` | PASS; inherited test-only blocking bridge record remains, with no P1p addition |
| `bun ./📜️script.ts verify interactivity --self-test` | PASS; all adversarial mutations rejected |
| Production forbidden scans for `block_on`, Tokio runtime/spawn, unbounded `ArtifactActorMsg` sender, remote drain, and status drain | PASS after test-span exclusion; raw hits are confined to `#[cfg(test)]` mock-hub and Shell fixture code |
| Scoped `git diff --check`, `git diff --cached --check`, and `git diff HEAD --check` across P1p source/verifier paths | PASS |

At the final check, whole-tree `git diff --check` was clean. `git diff --cached --check` and `git diff HEAD --check` each reported only the concurrent P10 report `OWNED-UI-AND-TOOLING-STACK/📓️p10-owned-eslint-plugin-react-hooks-retirement-2026-08-23.md:3`, whose `Date: 2026-08-23` line has two trailing spaces. This is outside the P1p source scope and is not a P1p acceptance blocker; no concurrent file was modified by this audit.

## Residual evidence required

Not run by audit scope: Cargo compilation and Rust tests; real WorkerPool scheduling/timer races; native folder, watcher, hub, and database I/O; cancellation/close timing under load; Wasm/browser behavior; Nx tasks and root lint. Those remain evidence required for Phase 1's broader runtime acceptance.
