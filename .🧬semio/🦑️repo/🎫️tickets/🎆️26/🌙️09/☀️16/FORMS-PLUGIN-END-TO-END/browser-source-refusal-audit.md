# Source Refusal Audit and Fix

The previous source action mapper threw when the configured JSON-chunk budget was exceeded. Surface delivery had already appended echo.pending before awaiting it; the outbox swallowed the rejection, so no parent action occurred but local echo bookkeeping and feedback were incorrect. A generic refused outcome would have triggered session rollback and discarded the correction text.

The owned mapper now returns schema-described dispatch/local/refused-size results. It checks encoded UTF-8 size before chunk splitting and does not reinterpret programming faults as size failures. Change-commit sources use their controlled local draft and the existing serialized latest-value outbox, independently of normal editor echo pending/typing publication. Local changes update the draft immediately, even while an older parent action is in flight. Only admission of the same latest text clears size feedback; its parent publication then releases the local override. Explicit discard reverts to published text without mutation.

Feedback is localized through the canonical UI labels in English and German, associated with the textbox through aria-describedby and aria-invalid, and offers the existing localized discard control. The shared encoded boundary32770/32771 is unchanged. The generic65536 ceiling and ordinary answer16384 budget are unchanged.

Added pure lifecycle/schema cases and mounted EN/DE actual TextEditorHost tests using the existing Testing Library adapter. They cover withheld invalid actions, exact controller/window/chunks, a newer refusal surviving older admission, publication-before-admission, later foreign publication and explicit revert. The browser suite now additionally authors a real graph, refuses32771 locally, commits32770, reopens it to verify the stored value, restores valid graph source and explicitly discards a later oversized edit, in both locales. This makes14 authored browser cases. No new browser discovery or live execution result is claimed.

Validation is pending existing focused owner43179 (normal registered Bun/Vitest, no duplicate) and primary renderer21435/browser8166. Earlier five-test green applies to the prior mapper; the typed refusal and mounted tests require their own actual result.


The follow-up scope audit is addressed: identity now combines the exact shell LocalDocumentOwner.id, mounted window key and semantic edit route. Presentation labels/settings are excluded. Optional editIdentityArguments lets the caller exclude changing baseline payload from identity while dispatch retains complete editArguments; provider Try controls author that record, and Forms built-in Try arguments are already stable IDs. Owner retirement, owner changes and unmounts reject stale completion delivery. A local revert behind an older in-flight edit is retained and sent after it, rather than skipped because it matches an older published value. Matching newer scene publication releases an admitted local override.

Mounted tests now include locale changes while invalid, exact document owner replacement/retirement, latest-value outbox delivery without invalid parent commands or typing publication, and a local revert behind older pending admission. The hook-level outbox test uses an injected minimal session adapter to exercise actual React hook/outbox code; it does not claim real Wasm execution. The actual browser case exercises the real attached Wasm session and remains pending.


Read-only installed Vitest CLI audit confirms filename filters use substring matching; the existing queued filter ending in 🟦️.ts includes the co-located 🟦️.tsx mounted suite, and the ticket config explicitly includes both. A second focused test owner is unnecessary. This is runner inspection, not an execution result.


The scoped Play test owner explicitly declares the new external TextEditor fixture as a freshness input, matching its existing external Flow/answer fixture declarations. This changes no dependency manifest and does not start another owner.


## Actual Source Editor Recovery Green

Authoritative focused session43179 completed exit0 through normal registered Bun/Vitest capacity admission:2testfiles,13passed,0failed,6.27seconds total,303ms assertions. Both the eight pure cases and five mounted cases executed. Mounted coverage uses actual TextEditorHost/React/Testing Library for explicit EN/DE labels, invalid32771→correct32770 recovery, locale changes without draft loss, accessible descriptions/invalid state, exact controller/window/chunks, stale admission isolation, publication ordering, foreign publication, owner retirement, explicit discard and pending revert. The hook-level source outbox test confirms latest local correction wins without invalid parent dispatch or false typing state. It uses a minimal injected session adapter and does not claim real Wasm execution.

The earlier five-test result is superseded for the production feedback/identity implementation by this actual13-test result. Integration is now converting shared graph/answer literals to exact compact recipes to satisfy the actual native source-byte budget; after its fixture/consumer stable notice, one fresh full13-test run will validate that representation change. No duplicate focused owner is live. Actual browser/native completion remains required. Log: generated/source-editor-command-boundary.log.


## Final Compact-Recipe Source Editor Green

The single post-recipe focused rerun69935 completed authoritatively with exit0:2files,13passed,0failed,8.29seconds total and306ms in tests. It used ordinary registered Bun/Vitest admission with2048MB reserved and consumed the stable compact graph/answer recipes plus all updated consumers. Raw/encoded graph and answer sizes/hashes are asserted by the updated pure tests, and the complete mounted EN/DE recovery/identity/queue/revert corpus passed again. Log: generated/source-editor-recipes-final.log.

This supersedes the pre-recipe13-test checkpoint for the current fixture representation. No focused rerun remains live and no further repetition is needed absent a new code change or failure. The full source-demand runtime36-test result remains green. Renderer21435 and scoped browser8166 remain retained; actual tsc/service/browser execution is still separate and has no pass claim here.
