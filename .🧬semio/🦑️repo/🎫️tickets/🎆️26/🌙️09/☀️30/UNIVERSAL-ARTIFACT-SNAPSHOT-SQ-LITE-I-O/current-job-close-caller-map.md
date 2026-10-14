# Current Job Close Caller Map

Read-only current concurrent source, 2026-10-13. No edits/builds. High owns canonical migration; paths are source frontiers, not fresh compiler-qualified errors.

Canonical Job InteractiveJob and erased-close APIs now require `&mut StepContext` and return `Result<InteractiveJobCloseStep,OriginalTurnFailure>`. InteractiveJobCloseStep currently has Pending/Blocked/Complete only. Old Refused{kind,progress} conversion truncates actual owned cause.

| Actual owning path under Plugin | Stale boundary | Original context status |
|---|---|---|
| `🧵️retained-command/🦀️.rs:368,821` | controlled_close_step strips cause to removed Refused; true InteractiveJob close_step still bare grant | Scheduling trait already lends actual StepContext; migrate close signature and pass it into real child closure/debit helper |
| `🖥️host/🦀️.rs:3606` | actual job close bare grant; S::Refused aliases for writer/source failures | Actual InteractiveJob close entry now must receive original StepContext; preserve writer/source original receipts and whole failure |
| `📜️declaration/📥️natural-file/🦀️.rs:282,291` | job close bare grant; Refused kind only | Current local signature lacks cx; canonical trait supplies original context on migration |
| `♻️lifecycle/🦀️.rs:~502–512` | erased job close helpers/Refused conversions | Distinguish snapshot-disposer grant API (still separate) from interactive job closure; actual caller must pass original context only at job boundary |
| `⚛️reactor/🏃️runtime/🦀️.rs:1230,1236,1369` | runtime_live_cleanup_pump_one uses mounted close grant and removed Refused after rejected.close_step; cleanup similarly truncates outcome | No StepContext in current helper signature; trace actual admitted parent callback to thread it. Do not create internal fresh context/clock/deadline |
| `🧪️tests/🔬️tool-run/🦀️.rs:235,254` | test InteractiveJob impls still bare-grant close | Existing test original StepContext setup should supply close; no fake compatibility overload |

Current Actor provider cancel/step uses canonical OriginalTurnFailure, and Plugin reserved close at `⚛️reactor/💼️jobs/🏛️reserved/🦀️.rs:135` already receives actual cx and closes original source/output through current close_original. No stale Refused match found in that entry.

Job owning tests `⏱️budget/🧪️tests/⏱️budget/🦀️.rs:368` and `♻️retirement/👷️authority/🧪️tests/🦀️.rs:75,78,79` still call actual migrated session.close_step(grant). They require actual caller context setup/loan, not invoking new method with old grant. High-owned spans must remain untouched by this audit.

Do not blanket-change every WorkerJobCloseStep::Refused or close_step(grant): actual OriginalPreviewSession trait in `👷️worker/🔀️erasure/🦀️.rs` remains a grant-bearing worker abstraction; lower cancellation/payload/wake retirement APIs also retain independent grant contracts. Current Job internals `close_original_worker_params:2065`, worker_payload_step:2927 and wake retirement:74/79 still collapse owned errors to worker kind/progress, a distinct remaining custody frontier when exposed at canonical cx boundary. That is not automatically a removed-variant syntax error; preserve whole original failure when migrating owning boundary.

No original caller authority should be synthesized from demands. Debit actual unrecorded failed child receipt once, keep primary and receipt_refusal, schedule/fuel once at original boundary, retain them through paid close/publication. New Semio Text source qualification remains separate; parent reports Source8046 strict3/79 green and first owning Native90438 pending, not a Job caller pass.
