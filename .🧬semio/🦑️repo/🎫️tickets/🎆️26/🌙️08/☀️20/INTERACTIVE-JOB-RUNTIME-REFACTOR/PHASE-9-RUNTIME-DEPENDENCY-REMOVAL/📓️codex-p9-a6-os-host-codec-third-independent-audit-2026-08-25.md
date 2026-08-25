# P9-A6 OS-Host Codec Third Independent Audit

## Verdict

**GREEN for the bounded P9-A6 public-route remediation.** The two P0 paths in
the prior audits are absent from the current public service. `OsHostCodecInput`
has exactly three retained structural variants:

- `Workflow(WorkflowStructuralCursor)`;
- `Filter(FilterKindsStructuralCursor)`; and
- `Normalize(NormalizeKindStructuralCursor)`.

There is no `Bytes(Vec<u8>)` variant, accumulated raw request, post-seal whole
input parser, batch workflow backend, `ArtifactPack`, `ArtifactDsl`, or external
browser ABI edge in the 1,264-line production `codec_abi` region. The public
`OsHostCodecService::new` owns only `RegisteredOsHostFormatResolver`; the
workflow structural remediation remains intact.

## Source Findings

`FilterKindsStructuralCursor` retains only a fixed three-byte header, a
two-byte current length, a 1,024-byte current kind, UTF-8 state, scalar item
state, and resolved output. It validates version/count/length during byte
admission, rejects a 257th item or a 1,025th current-kind byte before that byte
is copied, and resolves one completed kind at its final admitted-byte
opportunity. `NormalizeKindStructuralCursor` has the fixed 1,024-byte
current-kind state and resolves through the registered resolver only on the
last admitted byte. Normalizer max+one input is rejected by `begin` before a
session or copy is created.

Workflow accepts the WFP1/version/canonical-length framing one byte at a time,
copies canonical DSL directly into the eventual retained output, and seals with
scalar structural checks only. Filter and normalize seal by completing their
cursor state and moving already-resolved output into A1 paging. The bounded
`str::from_utf8` views occur while resolving the current fixed item during its
final admission, never on an accumulated raw request after seal.

The exercised laws cover all named routes for every byte/field split,
cancelled partial page exact return, zero-credit/interruption/deadline
non-advance, malformed/truncated/unknown/invalid-UTF-8 replies, preflight
maximum-plus-one, handle loss, stale generation, duplicate ACK, ACK-gated
output, and interrupted one-unit-at-a-time close. This includes public-service
workflow, filter, and normalization cursor paths in the feature suite.

## Executed Gates

| Gate | Result |
| --- | --- |
| Current-source dependency-free debug retained ABI binary | GREEN — 30 passed, 0 failed |
| Current-source dependency-free optimized retained ABI binary | GREEN — 30 passed, 0 failed |
| Feature-enabled public `OsHostCodecService` binary | GREEN — 32 passed, 0 failed |
| Feature rlib external wrapper link and execution | GREEN — exit 0 |
| Hostile raw `Bytes(Vec<u8>)` injection | GREEN — expected rejection, exit 101 |
| Hostile whole-slice `input.bytes()` injection | GREEN — expected rejection, exit 101 |
| Hostile source/batch-edge law against current source | GREEN — exit 0 |
| Focused `rustfmt --edition 2021 --check` | GREEN — exit 0 |
| Focused `git diff --check` | GREEN — exit 0 |
| Bun schema/ledger/fixture-pair parser | GREEN — 4 operations, 9 errors, 10 ledger rows, 7 limits, 5 DSL/SPK name pairs |
| Direct host manifest `wasm-bindgen` / `serde-wasm-bindgen` census | GREEN — 0 rows |
| Host-tree browser ABI (`wasm_bindgen`, `serde_wasm_bindgen`, `JsValue`, `web_sys`, `js_sys`) census | GREEN — 0 matches |
| Codec production whole-input/batch deny census | GREEN — 0 for all 11 denied edges |

The strict debug/optimized/feature binaries were produced immediately after the
current host source timestamp; they and the current-source hostile law were
executed in this audit. A direct standalone `rustc --test component.rs` is not
an applicable substitute: the complete host component intentionally imports its
workspace-only crates and fails before reaching the bounded codec module. No
Cargo workspace/package, Nx, Wasm, or browser command was run.

## Census And Scope

The current unstaged production diff census is 1,802 additions / 40 deletions
in the host component, 0 / 4 in the host Rust manifest, and 6 / 31 in the OS
TypeScript component. The new codec schema and hostile fixture are untracked
ticket-packet files. Focused diff checking is clean. This shared worktree also
contains unrelated concurrent ticket artifacts; no global clean-worktree claim
is made.

The generated renderer worker still has exactly two old workflow-export calls:
`parseWorkflowFixtureDsl` and `decodeWorkflowFixturePack`. Both are inside its
`if (import.meta.vitest)` test block; the non-generated OS source has no live
call edge (the remaining names are the new schema operation labels and a
source-law string). They are derived Vitest-only calls, not a public production
route. Regeneration remains intentionally deferred because this packet forbids
Nx/Wasm/browser work.

## Blockers

None for bounded P9-A6 source acceptance. Full Cargo integration and generated
worker regeneration remain explicitly deferred integration/artifact gates and
are not claimed by this audit.
