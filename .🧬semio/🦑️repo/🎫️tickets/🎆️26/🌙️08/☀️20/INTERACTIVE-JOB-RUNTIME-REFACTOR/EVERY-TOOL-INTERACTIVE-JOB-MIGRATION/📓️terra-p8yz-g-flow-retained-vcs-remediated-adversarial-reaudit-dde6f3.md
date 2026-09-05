# P8yz-g Flow Retained VCS Remediated Adversarial Re-Audit

Date: 2026-08-26  
Auditor: Terra (independent source/static audit)  
Verdict: **RED — the retained implementation remedies the prior route/cursor blockers, but the mandatory fixture/oracle evidence is still materially incomplete.**

## Scope

Read and compared:

- the original Interactivity-First plan at `/Users/ueli/.codex/attachments/2225dd4d-c3b6-4564-b4b1-f552928e8ff3/pasted-text.txt`;
- the repository `AGENTS.md` and the all-app acceptance contract;
- the prior RED audit `📓️terra-p8yz-g-flow-retained-vcs-cursor-adversarial-audit-2026-08-26.md`;
- the implementation report `📓️codex-p8yz-g-flow-retained-vcs-source-static-implementation-2026-08-26.md`;
- the complete live `//#region 🌊️RetainedVcs` route, lines 959–2756 of `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🦀️component.rs`; and
- its three language-neutral fixture files.

No production source, scripts, caches, Cargo, Nx, Wasm, browser, or runtime target was modified or run.

## Remediation Findings

| Required source/static property | Result | Evidence |
| --- | --- | --- |
| Complete retained route has no iterator/recursive/whole-clone/whole-digest/slot scan in census, digest, or operation-slot paths | GREEN | The complete 1,799-line region has no `.iter()`, `.position(`, `.find(`, `for `, `while `, `.clone()`, legacy whole digest/census, or monolithic apply spelling. Admission and initial digest are scalar metadata at `component.rs:1409`, `1509–1572`, and `2709–2755`; four operation slots and surface resource counts are explicitly unrolled. |
| One bounded cursor unit per polling grant | GREEN (static) | Scan, shift, mutation, replacement transfer/reversal, redo retirement, history transfer, surface transfer, visibility, page, and rollback are explicit persistent phases. No whole collection mutation token is present in the route. |
| Exact rollback of redo/history/surface/visibility/edit/semantic ownership | GREEN (static) | `cancel`/`fault` enter rollback for every changed ownership flag and `redo_retired > 0` (`1676–1718`). `close_operation_step` reverses visibility → surface → history → redo → semantic owner, then edit ownership (`1740–1811`). Redo ownership is restored one owner per close grant. This repairs prior RED-2. |
| Full grant envelope on grant-bearing poll/control/close routes | GREEN (static) | `poll`, `cancel`, `fault`, `panic_fault`, `close_operation_step`, and nonterminal `close_retired_step` reject a non-permitted grant. `permits_work` requires fuel, item capacity, non-interruption, unexpired deadline, and a deadline window no larger than eight milliseconds (`1073–1075`). This repairs prior RED-3. |
| Split semantic publication and reverse rollback | GREEN (static) | `transfer_history_cursor`, `transfer_surface_cursor`, `publish_visibility_cursor`, and `publish_page_cursor` are separate phases (`2080–2160`). Cancellation/fault remains valid before the terminal page phase and close reverses the first three independently. This repairs prior RED-4. |

The remediated source is materially different from the prior RED packet. I found no new source/static counterexample to the five former retained-route defects.

## Blocking Defect

### RED-1 — the claimed 13-feature third-party oracle parity is a self-authored label comparison, not an oracle comparison

The all-app contract requires a test-only third-party oracle to produce the same semantic result as the owned implementation through an owned interface. The implementation claims this for all thirteen retained features, but `retained_vcs_all_thirteen_features_match_owned_third_party_oracle_matrix` does not derive any of the thirteen `actual` semantic results from the live `FlowRetainedVcs` document, page, or event output.

- `SerdeJsonFlowOracle::feature_cases` only parses the `feature` and `semantic` strings from JSON (`component.rs:3008–3025`). It does not model any Flow VCS operation or produce a semantic result.
- Each real operation is driven through ACK and close, but the test then appends a literal fixture label such as `"widgetCount+1"`, `"widgetOrderChanged"`, or `"redoPublished"` (`3143–3211`). No post-operation widget/synapse/layout/version/history/page state is converted into `actual`.
- The final equality at `3213` therefore proves only that the test's hard-coded labels repeat the fixture's labels. A broken feature that still returns a page would pass this matrix so long as the literal push remains unchanged.
- The separate small-layout test does compare one layout result (`3113–3134`), but it cannot establish the claimed all-feature parity.

This is a direct failure of the plan/acceptance contract's independent-oracle law and of the implementation report's statement that the matrix compares the canonical ordered semantic result of all thirteen real retained features with the third-party oracle.

### RED-2 — the language-neutral fixtures do not contain the mandated hostile boundary data

The JSON fixtures structurally name 13 features, 30 laws, 24 cursor boundaries, 24 cancellation labels, a 22-field fingerprint, and 62 owner categories. They do not supply per-feature input/output ledgers for the required edge and hostile data:

- no empty, single, maximum, or maximum-plus-one operation vectors;
- no explicit `1/3/16/17` capacity/boundary vectors;
- no multibyte/UTF-8 source or expected semantic result;
- no stale/wrong/ABA source vector, malformed/omitted fixture value, deadline/zero-fuel grant fixture, or exact post-close handback vector; and
- no fixture-owned expected document/page/history output for each feature through ACK and close.

The fixtures instead provide category names and descriptive law strings. The executable Rust tests contain several constructed values, but that does not turn the language-neutral artifacts into deterministic input/output ledgers. A direct fixture-only census found no `multibyte`, `utf8`, `1/3/16/17`, or quoted `"1"`, `"3"`, `"16"`, `"17"` test-vector label.

This violates the all-app contract's mandatory language-neutral fixture coverage and leaves RED-1 untestable even if the test assertion is rewritten.

## Safe Gates Re-Run

| Gate | Result |
| --- | --- |
| `rustfmt --edition 2021 --check` on the Flow VCS component | PASS |
| `git diff --check` on the component and three fixture paths | PASS |
| Standalone Bun complete-route source law | PASS — 1,799 route lines; 23 forbidden spellings absent; 10 required retained tokens present; 5 full grant-envelope checks found |
| Standalone Bun fixture parser/cardinality law | PASS structurally — 13 features, 30 laws, 24 boundaries, 24 cancellation labels, 22 fingerprint fields, 62 owners, 13 oracle cases, terminal protocol `pageReady` |
| Fixture vector census for the demanded explicit values/UTF-8 data | FAIL — no multibyte/UTF-8 or `1/3/16/17` vector exists |

Cargo/unit execution, Nx, Wasm, browser, native, timing, and cache-touching gates were intentionally not run.

## Required Remediation

1. Make the test-only oracle independently evaluate each fixture-defined operation and emit owned canonical semantic/page/history results; derive the implementation-side result from the retained session after ACK and close, then compare those values instead of matching literal labels.
2. Extend the three language-neutral fixtures with deterministic input and expected-output ledgers for every one of the thirteen features through ACK and close, including empty/single/max/max+1, explicit 1/3/16/17 vectors, multibyte text/identifiers, stale/wrong/ABA, malformed/omitted data, cancel/fault at every transfer, expired/over-window/zero-fuel/interrupted grants, and exact terminal handback fingerprints.
3. Add hostile mutation tests that prove removing an oracle result extraction or any required ledger vector fails the source/fixture gate.

Until these are present, the packet cannot be accepted as GREEN at the required source/static boundary despite the repaired retained cursor implementation.
