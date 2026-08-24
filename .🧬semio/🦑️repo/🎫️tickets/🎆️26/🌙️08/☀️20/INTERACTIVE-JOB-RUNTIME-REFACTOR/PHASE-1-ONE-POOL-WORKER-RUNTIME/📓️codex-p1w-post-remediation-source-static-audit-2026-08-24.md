# P1w Post-Remediation Source/Static Acceptance Audit

Date: 2026-08-24  
Auditor: Codex independent read-only audit  
Verdict: **GREEN — no remaining P1w source/static counterexample found.**

## Scope And Method

Read completely:

- repository `AGENTS.md` and applicable OS instructions;
- the original P1w caller census, Terra's prior RED audit, and Sol's remediation report;
- the live `CatalogBootstrapCas` production region, root `Database::open_with` cutover, terminal authorities, and the P1w law region in `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs`;
- the complete P1w root-verifier rule and hostile-mutation suite in `📜️script.ts`.

This was a fresh source/static audit. No production or verifier source was edited. Cargo, Nx, Wasm, browser, release/native matrix, and runtime matrix were not run.

## Prior RED Closure

The old cancellation window is closed by an independent atomic driver authority:

- `schedule` is the only `Idle → Queued` claimant ([component.rs:2824](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:2824)); a non-idle requester records a wake instead of enqueuing a competing driver ([component.rs:2829](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:2829)).
- The queued callback atomically claims `Queued → Driving` before clearing the observational `scheduled` mirror ([component.rs:2901](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:2901)). It releases `Driving → Idle` only after its body has finished, then consumes the retained wake ([component.rs:2922](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:2922)).
- `cancel` and public-future `Drop` only publish cancellation plus a wake and invoke the same authority-aware scheduler; neither branches on the polling mirror ([component.rs:3389](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:3389), [component.rs:3418](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:3418)).

Therefore a Handoff-to-first-poll cancellation that observes `scheduled == false` observes `Driving`, cannot acquire a second callback, and is rechecked by that same driver before its first backend poll ([component.rs:2948](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:2948), [component.rs:3026](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:3026)). No competing poll-driver execution trace remains.

## Rechecked Acceptance Properties

- Handoff retains unpolled `storage` and `pages`; the backend future is mounted only in `work.poll` on the I/O callback ([component.rs:3003](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:3003), [component.rs:2640](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:2640)).
- Work is republished before polling-gate release for pre-poll cancellation, Pending, Ready, and panic paths ([component.rs:3037](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:3037), [component.rs:3048](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:3048), [component.rs:3059](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:3059), [component.rs:3070](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:3070)). The polling flag remains an observation, not driver authority.
- Queue refusal retains the exact returned job before `Queued → Retry`; the callback must reclaim `Retry → Queued` before resubmission ([component.rs:2851](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:2851), [component.rs:2875](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:2875)).
- The check-register-recheck public future and publication-before-waker order remain intact ([component.rs:3398](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:3398), [component.rs:3153](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:3153)).
- Admission is generation/byte qualified, result handback releases only after all public parts are transferred, and retained terminal cleanup releases one owned authority per worker opportunity before generation-qualified registry removal ([component.rs:2239](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:2239), [component.rs:2365](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:2365), [component.rs:3213](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:3213)).
- Exact initial-fence validation preserves backend errors and turns only a returned unexpected epoch into `DbError::Fenced`; it never retries `EpochFence::INITIAL` ([component.rs:3133](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:3133)).
- The fresh-root `None` caller branch now awaits the retained operation and returns its exact storage/epoch; all four public open surfaces converge on this private path ([component.rs:5127](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:5127), [component.rs:5157](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:5157)). The scoped semantic-equivalent sweep found only this initial CAS mount; the three remaining production `db_actor::block_on` sites are the census's create-document, compaction, and sync-hello waits.

## Law And Verifier Fidelity

The deterministic four-worker law freezes a real driver after `Queued → Driving`, cancels from the facade thread while `scheduled == false`, proves zero backend polls, then verifies one driver, exact page/storage identity, single admission release, and registry removal ([component.rs:7522](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:7522)). It repeats the driver-release race for Pending, Ready, panic, and result-drop retirement ([component.rs:7576](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:7576), [component.rs:7617](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:7617)).

The root verifier now checks authority transition order, cancellation placement, owner publication before release, retry publication/order, terminal authority, and all fourteen law bodies. Its 51 hostile mutations specifically reject the previous boolean/mirror authority, claim-after-clear, release-before-body, retry ordering, cancellation-through-polling, pre-claim cancellation check, wake-consumption, shallow race law, and retirement/identity omissions ([📜️script.ts:9794](/Users/ueli/Documents/semio/📜️script.ts:9794), [📜️script.ts:9978](/Users/ueli/Documents/semio/📜️script.ts:9978)). No remaining static-verifier false-green equivalent was found in this source audit.

## Executed Isolated Gates

| Gate | Result |
| --- | --- |
| `bun ./📜️script.ts verify interactivity p1w` | PASS — live source and all hostile mutations clean |
| `bun ./📜️script.ts verify interactivity p1q-b1-b6` | PASS — retained P1q gate remains clean |
| `rustfmt --edition 2021 --check --config skip_children=true …/db/engine/🦀️component.rs` | PASS |
| scoped `git diff --check` on DB engine and root verifier | PASS |

Runtime, target, allocation-pressure, and timing evidence remains outside this read-only static acceptance audit and is still required by the serialized final matrix.
