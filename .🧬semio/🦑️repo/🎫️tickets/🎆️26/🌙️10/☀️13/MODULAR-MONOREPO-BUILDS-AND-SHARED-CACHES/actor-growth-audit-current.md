# Actor and Job Growth Audit

## Scope and Evidence

Read-only filesystem inspection on 2026-10-15. No builds, tests, lock probes, source edits or git mutations were performed. Concurrent decomposition is active; import observations below are snapshots, not settled qualification failures. Read the existing actor-crate-decomposition-current.md first. Paths below are relative to `🧰️framework/🔨️modules`.

## Immediate Job Frontier

`🧵️job/🦀️.rs` is 250327 bytes / 3878 physical lines before its actual production module closure. Its package manifest still has one implementation crate. Moving declarations into Rust modules alone cannot reduce that crate Source closure. Large bodies additionally belong to payload retirement (14956 bytes), worker admission plus preparation (24449), erasure (15081), caller-source parent plus child (26940), capture (10929), native decode (12489), failure/domain/projection (20379), builder/publication/fault/descriptor/loan (41986), and wake retirement (9376). Counts are physical bytes and lines, not the compiler active-line metric.

### Coherent Next Cut

First move the real retained worker-session family into a native worker crate depending downward on a core Job protocol crate. The existing root regions identify an unusually clear boundary: RetainedSessions begins at line 2036, covers BatchDriveConfig/BatchJobParams, WorkerJobAuthority, BatchJobSession, MountedWorkerJobSession, WorkerJobSession/Inner, retirement slots/pump, submission/outcome/rejection and ends before TortureJob at 3624. Keep their private strong ownership SCC together: session-return and its sealed SessionHandle, worker admission/preparation, session erasure, preview driver and retained worker-wake. `♻️session-return/🔐️handle/🦀️.rs:2` directly names the private WorkerJobSessionInner; splitting there would unnecessarily widen authority internals. `👷️worker/🔀️erasure/🦀️.rs:10` implements its sealed preview trait for BatchJobSession; keep those nominal owners together. Move the actual worker-specific tests and WORKER_SESSION_SLOTS_TEST_AUTHORITY with this family.

Core must retain StepContext, StepBudget, original caller/capture/park/decode, failure, payload authority/builder/outcome/retirement and InteractiveJob initially: they share private ledger/cancel references and exact receipt authorities. `🧵️job/🦀️.rs:927-998` is the key authority knot; a cosmetic payload crate with a reverse Job dependency would create a cycle. In particular, `⚠️failure/🦀️.rs:2` uses StepContext, while StepContext operations return these failures. `📬️outcome/🤝️loan`, descriptor/publication/builder and retirement consume the same core types. Model the core SCC explicitly before another cut.

The core root also contains independent candidates: identity/commit/fixed registry (141-388), structured child JobScope (1456-1804), progress vocabulary/channel routing (1806-2034), and TortureJob (3624 onward). A registry leaf can move with its own tests without depending on worker; a progress crate can depend on the core payload and nominal protocol, provided core does not import ProgressEvent in actual method signatures. TortureJob belongs in a real conformance/test-support owner rather than every downstream production Source closure. Root core tests presently add 66797 bytes retained-ownership, 37039 budget, 28706 worker authority and other fixtures; test closure must be censused separately after moves, not assumed bounded from production size.

The worker cut requires downstream users that need sessions to depend on the real worker owner. Do not make core depend back on worker to preserve old universal exports. A thin top-level assembly facade may depend on both only if downstream narrow consumers such as Actor retained context stop selecting that assembly. Reexports alone do not reduce compiled Source ownership.

## Actor Snapshot

Current manifests show an acyclic local direction: pack leaf; retained-turn -> pack; context -> retained-turn; vocabulary -> retained-turn + pack; job publication -> vocabulary + pack; runtime -> all five. No local manifest cycle or framework-to-product inversion was found in these inspected direct manifests. The body moves are real: root actor body is now 99656 bytes / 1792 lines; vocabulary 70197 /1515; publication 82568 /1822. Vocabulary canonically includes lifetime/page/return/cold-pair bodies; retained-turn canonically includes its actual source/execution/provider/batch/arguments/transport/park bodies. There is no copied second declaration of these types in runtime, which reexports their owners at actor/🦀️.rs:23-29.

At inspection the newly separate retained-context manifest omitted serde although its body uses serde derives at lines 85 and 91. This is a compile-visible manifest dependency gap if still present at the next actual qualification; coediting may already repair it. `protocol` references in vocabulary/publication are not automatically unresolved: dependency package semio-framework-replication may expose that library crate name, so alias text alone is insufficient evidence. No native compilation was run here.

Context contains a componentwise grant subset predicate at context/🦀️.rs:42. This has not been established as a duplicate canonical implementation: a repository-wide semantic comparison is needed before changing authority policy. Avoid replacing it with a wider helper until exact dimensional and equality laws are compared.

## Minimal Qualification Sequence

Finish the worker/core ownership schema and language-neutral dependency/canonical-body corpus first; move actual bodies and private authority tests together; update callers to the narrow owner; census each actual production and test Source closure under unchanged caps; then run the existing issued prototype/release route. This audit reports no passing tests or runtime behavior.
