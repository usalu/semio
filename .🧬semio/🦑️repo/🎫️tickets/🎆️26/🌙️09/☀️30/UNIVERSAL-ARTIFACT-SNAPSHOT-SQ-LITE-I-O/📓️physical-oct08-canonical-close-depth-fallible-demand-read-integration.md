# Canonical Close Depth, Fallible Demand and Read Integration

## Current Outcome

The current retained-clone cursor contract requires `next_close_depth_demand`. Scalar, String, Vec, Option, Box, tuple, field, paged bytes/text/map/list, ordered-map and derive-generated cursors now delegate the actual next close branch. Pending typed retirement births report one depth; outstanding tickets delegate their measured depth; retained bindings expose their own exact depth. Inline child close transitions retain their existing depth semantics. Physical boxed/field cursor backing release now reports and requires one depth before freeing the whole measured scaffold; refusal retains that exact Box. This is not a compatibility restoration.

The erased retirement copy observer now returns `Result<usize, ValueError>`. The typed retirement cursor work observer and controlled retirement work observers propagate that same refusal. Factory copy-demand observation no longer panics through `expect`. The real ticket handoff test subsequently exposed child `Complete` being forwarded while its outer Box remained retained. The defining helper now validates the child receipt and terminal witness, reports Progress until the outer release is funded, and only returns Complete once its slot is absent. Fresh qualification after this correction is actual native run 92501: 31 tests passed, zero failures, 166 skipped in 0.503s. The stronger test now observes the public ticket demands, retains the spent child until the separately funded physical frame release, and requires an absent slot for Complete. The Store cursor handoff now accepts a full independent grant, validates the complete progress receipt against that grant, requires the exact terminal witness, and forwards all four demand observations without a sentinel. The Store String retirement now retains its original String in the defining typed controlled owner; logical byte work and whole backing release remain distinct.

## Read Provider Integration

The defining Value read file was physically empty except its tests when inspected. The original fixture retains 1024 slots, sparse return visitation, wraparound and live-reader starvation boundaries. While a new inline fixed-slot provider was being authored, another concurrent task supplied `ReadOwnershipRegistry` with generations, bitmap/free slots, a fixed admitted backing allocation and publication authority. The duplicate compile exposed both definitions. Only this owner's newly authored duplicate prefix was removed; the concurrent provider was preserved and adopted. Tests now bind its new public type, with original semantic vectors retained. The fixture/schema identifier is `framework.value.read-ownership`.

The independent audit is recorded in [the read-owner census](📓️semantic-independent-oct8-remodeling-native-edit-map-and-read-owner-census.md). New read ownership is not inferred from retained-source admission alone. The original constructor refusal, source refusal, original address preservation, source cancellation and complete physical heap-conservation tests remain enabled.

## Executed Evidence

| Run | Observed result |
|---|---|
| Original Value native 43140 | Inner command exit 1; one Value lib signature mismatch at concurrent PagedText provider; no runtime. |
| Original Value native 79947 | Inner command exit 1; Value lib progressed to three DSL Record encoding fallible-copy binding errors; no runtime. |
| Original Value native 88210 | Inner command exit 1; 25 coded test compiler diagnostics: 19 copy-observer bindings and six missing read provider references; no runtime. |
| Original Value native 30352 | Inner command exit 1 after test bindings; no runtime qualification. |
| Original Value native 6075 | Inner command exit 1; duplicate newly supplied read definitions; no runtime qualification. |
| Source oracle 28130 | One pass, zero failures, 11 expectations; DEBUG witness observed. |
| Source oracle 95386 | Two passes, zero failures, 17 expectations; independent JSON Patch sparse read and funded/refused birth witnesses observed. |
| Original Value native 92501 | Actual child exit 0 after original outer-slot completion fix; 31 passed, 166 skipped in 0.503s; real ticket demand/refusal/frame release witness ran. |
| Original Value native 40616 | Actual child exit 1; 30 passed, one failed. Stronger real factory handoff check exposed a false completion while the outer ticket remained retained. |
| Original Value native 68463 | Actual child exit 0 after exact boxed/field scaffold depth funding: Nextest 31 passed, 166 skipped in 0.574s. |
| Source oracle 57099 | Two passes, zero failures, 20 expectations after physical scaffold depth law. |
| Original VCS native 27639 | Actual child exit 1, 520 coded OS-kernel diagnostics; no Nextest runtime. One handoff module import diagnostic was repaired after this run. |
| Launch registry 5796 | Actual child exit 130; publication-input digest mismatch at `/nativeCodecs/9/protocolSourceSha256`; generator never published launch artifacts. |
| Original Value native 8483 | Actual child exit 0; Nextest ran 31 tests: 31 passed, 166 skipped in 0.952s. Both read laws and both new depth/refusal tests ran; DEBUG ownership and heap witnesses observed. |

Inputs and logs are under `📥️inputs/physical-oct08-canonical-close-depth-*` and `🗑️generated/physical-oct08-canonical-close-depth-*`. Compiler diagnostics from 88210 are preserved in `🗑️generated/physical-oct08-canonical-close-depth-after-dsl-diagnostics.json`. A Python orchestration process may exit zero while reporting its child's nonzero status; the table records the actual child result.

## Law and Checks

A new language-neutral `🧬️retained-clone/🧫️fixtures/🪜️close-demands/🔣️.json` defines idle, pending birth and terminal depth, underfunded birth refusal and copy observation refusal. Native tests observe real heap requests/releases, keep the original scalar after refused birth, and validate each complete independently granted close receipt. The independent Bun oracle uses the existing third-party JSON Patch and AJV implementations; it passed. Native run 8483 also executed the new laws: refused depth kept the original scalar with zero heap activity, admitted retirement receipts matched actual allocations/releases, and erased copy-demand refusal preserved the original owner. Read vectors executed three logical work grants (1, 3 and 64) over three texts with DEBUG physical conservation receipts. Original sparse/wrap/starvation vectors passed. The native artifact directory is `🗑️generated/root-native-authentic-current/semio-nextest-6xLreu`.

The launch seed includes the independent source oracle and original Value native selector at orders 206.1907 and 206.1908. Registry generation 5796 again stopped before publication at `/nativeCodecs/9/protocolSourceSha256`; generated launch publication is not claimed. No other-ticket artifacts, Git state, worktrees or AGENTS files were modified.

## Remaining Closure

Original VCS Source is separately qualified at 17 passes and 147 expectations. Original VCS Native and XLSX Native remain unqualified behind the real shared OS caller frontier. Existing legacy two-argument Store, SPR and VCS caller methods still require actual independent grant, deferred source birth, exact ownership-returning refusal and separately funded final frame release. The current original VCS 27639 reached Cargo Nextest build and returned 520 coded OS-kernel diagnostics, preserved in `🗑️generated/physical-oct08-vcs-after-canonical-depth-read-diagnostics.json`. The distribution is Store main 204, SPR history fold 45, document history hydration 42, member-open operation 41, config retained 41, snapshot-clone 21, and remaining concrete ownership providers. One handoff import diagnostic was corrected after this receipt. The new handoff does not erase that work. No Native qualification or ticket completion is asserted here.
