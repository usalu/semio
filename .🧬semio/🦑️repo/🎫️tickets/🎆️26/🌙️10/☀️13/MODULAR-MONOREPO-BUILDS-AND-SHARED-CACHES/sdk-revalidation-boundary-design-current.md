# SDK Revalidation Boundary Design

## Proposed First Change

Separate authentication of an exact private, live quote/projection and its immutable captured material from a complete physical source recheck. Authentication-only calls should not traverse the entire source tree. Retain physical validation at the owner of actual copy effects and at every distinct outer callback/drain boundary. Do not introduce a TTL, a successful-check memo, a caller-provided `alreadyValidated` flag or a stored assertion that mutable Source remains unchanged.

The first bounded implementation should reduce the sealed derivative path from eleven repeated projection rechecks to five explicit physical fences, while keeping the initial full content-hash capture, copy row checks, rooted source/destination scopes, actual base SH, registry EX, exact control and all accepted-work drains. This is a Source design for Root review; no implementation is authorized by this report. The full-clone branch should retain its current behavior until its independent copy/effect contract is separately traced.

No Original, test/build, image reader, Cargo command, publication or maintenance was started. Storage's guarded historical prototype remains outside this lane. All runtime timings below come from retained Root evidence and were not rerun here.

## Actual Evidence and Limits

The integrated prototype 26268 passed one full runtime law/21 assertions in 105.74 seconds under its unchanged 120-second Original. Its recorded capture/projection/quote/copy-settlement points were 15.407/20.678/33.349/91.097 seconds, followed by independent oracles and final settlement.

Release 66585 failed at the same unchanged deadline. Capture was 11.635 seconds; projection/quote/copy settlement were 19.302/34.845/117.008 seconds. The child was cancelled/SIGKILLed before independent verification and final settlement. This is not a release pass. The 82.163-second interval from quote-held to copy-settled contains multiple rechecks as well as actual materialization; those outer stage values do **not** independently measure each scan or prove the whole interval is validation cost.

These results justify targeted boundary instrumentation and removing demonstrated repeated work. They do not justify a deadline increase, another unchanged large retry, skipping hashes or relaxing capacity.

## Source Inventory

Paths below are relative to `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools`.

| Owner | Actual observed behavior |
|---|---|
| `📦️sdk/📥️source/🔗️derivative/🟦️.ts`: projection construction | Actual base SH is acquired before base reads; authenticated pristine inputs/marker and complete descriptor-content capture produce a privately mapped projection. Captured input bytes are copied, entry/extent facts are derived, and the owner retains SH through callback and accepted copy settlement. |
| Same file: `recheck(state)` | Checks live owner/store; rereads every captured dependency input and marker; traverses all captured stamps with rooted SourceTree, direct leaf `lstat`, stamp comparisons, exact symlink target checks and registered directory identities; checks sealed-index bytes. This is a complete captured-tree metadata/ancestry pass, not a repeat full content hash of every file. |
| Same file: `requireNxSdkDerivativeProjectionV1` | Private identity/workspace/store lookup currently invokes that full physical recheck before returning immutable entry/extent facts plus a recheck callback. |
| `📦️sdk/🎟️capacity/📥️physical/🟦️.ts`: `withPublicationQuoteV1` | Holds private quote state, material-derived bytes, single-publication/copy/input flags, actual accepted pending operations. Its `recheck` invokes the supplied physical verifier; it currently runs on issue, around clone, and after the quote body. |
| Same file: `requireNxSdkClonePublicationV1` | Private quote/store lookup then calls public `quote.recheck()` before returning the already derived byte bound. |
| Same file: `withAdmittedNxSdkClonePublicationV1` | Exact registry EX and private one-use claim; calls requireQuote and fresh statfs before any body effects, retains actual accepted-copy scope, closes births, drains pending operations and rechecks Source before returning success. |
| `🧹️retention/📥️physical/🟦️.ts`: publisher | Real registry EX; serial complete jobs, actual store checks; calls requireQuote to choose the immutable bound, performs existing bounded retirement/capacity observation, then enters admitted quote operation. |
| Derivative `copyNxSdkDerivativeV1` | Exact private accepted-copy custody and control; one-use claim before await; full physical precheck. Cold sealing releases SH, obtains real snapshot EX nonwaiting, rechecks/seals, returns to actual SH and rechecks. Complete per-row guarded copy/link operations and both rooted traversals settle; a full physical postcheck precedes copy success. |
| `📦️sdk/📥️source/🔗️derivative/🔏️index/🟦️.ts` | Initial capture discovers the complete actual namespace, bounds files/bytes, hashes every actual no-follow regular descriptor with pre/post identity checks, compares exact sealed index where present and completes rooted recensus. Keep this unchanged. |
| `📜️script.ts`: actual publisher | Uses privately captured immutable authorizer/compiler/package/patch/material inputs. It writes owned stage inputs, clones, patches owned Nx bytes, compiles/preflights authorizers, writes bounded receipts, validates installation, renames stage and validates destination. These later steps are distinct from copying the pristine Source tree. |

`requireResourceLeaseReadCustodyV1` exists in the actual lower lease owner and validates private live shared issuance. A private projection may authenticate its actual retained SH through that lower owner; it must not infer custody from public `lease.mode`/resource fields alone. Store/reservation/control authentication remains required independently of tree scanning.

## Exact Recheck Ladder

For a successful **already sealed** derivative publication through the current production publisher, with one `quote.clone` and no caller-requested extra recheck:

| # | Boundary | Why it currently traverses | Proposed first checkpoint |
|---:|---|---|---|
| 1 | `withNxSdkDerivativePublicationV1` → requireProjection | Get authenticated immutable entry/extent facts | Authenticate private owner/custody and return captured facts; no full traversal. |
| 2 | Initial `withPublicationQuoteV1.recheck` | Issue quote before caller body | Authenticate exact live quote/projection/store; no full traversal. No Source-derived effect occurs here. |
| 3 | Publisher → requireQuote | Select immutable required byte bound | Authenticate private quote/store/control; no full traversal. |
| 4 | `withAdmitted…` → requireQuote | Repeat bound lookup before actual fresh statfs | Same authentication; retain actual registry EX/statfs/reservation checks. No tree traversal for the number. |
| 5 | `quote.clone` precheck | Wrapper around derivative copier | Authenticate exact accepted operation and delegate physical precheck to the actual derivative copy owner. |
| 6 | `copyNxSdkDerivativeV1` precheck | Validate physical inputs before copying/linking | **Keep full physical precheck.** |
| 7 | Derivative copy postcheck | Validate physical Source after all copied rows and rooted scopes settle | **Keep full physical postcheck.** |
| 8 | `quote.clone` postcheck | Repeat immediately around the same delegated effect | Authenticate still-live exact quote/registry/store after delegated success; the actual copy owner already completed its postcheck. |
| 9 | Admitted publication after callback/pending drain | Callback can perform further writes/work after copying | **Keep full physical fence after all accepted operation work settles.** |
| 10 | Quote owner after callback | Quote body may do work after publication returns or start an accepted operation without awaiting it | **Keep one full fence after births close and accepted work drains.** Move the present pre-drain fence to this true final boundary rather than retaining both. |
| 11 | Projection owner after callback/copy drain | Projection body may continue after a quote scope, issue another quote or perform arbitrary accepted work | **Keep full physical fence.** |

The large `runtime` fixture adds an explicit successful requireQuote to verify the returned bound, so it currently adds a twelfth pass. Forged/foreign require calls refuse before scanning. Cold seal paths add their separate EX transition/preseal/reacquired-SH rechecks and seal-time content hashes; do not remove those in this first change.

This is a static success-path count from the cited Source, not an observed syscall counter. Refusal/failure/ready/full-clone paths have different counts. Five retained fences are deliberately conservative: outer user callbacks are independent effect boundaries even when the present normal caller happens to return immediately. Do not merge them using elapsed-time guesses or a `lastCheck` marker.

## Minimal Truthful Contract

### Private authentication and immutable facts

The projection require operation should validate exact private WeakMap identity, same workspace/store/control, live owning scope, actual retained matching base SH and real reservation/control. It may return copied/frozen captured material facts and detached bytes. It should not promise that current physical Source still matches those facts. A stale or mutated base must still fail at the next physical use fence.

The quote require operation should validate exact private WeakMap identity/store/control, live scope and the privately bound projection/material. It returns the bound derived at issuance. It must not return a structural caller-provided number or lower the cold/full-clone floor. Actual statfs/capacity, one-use registration and registry EX remain in the admitted effect operation at its actual start.

Prefer removing the derivative quote's public generic `recheck()` callback from authentication call paths. If a common full-clone quote still needs it internally, keep that verifier private to its owner/closed kind dispatch rather than making public method invocation the source of authenticity. No public `checked`, timestamp, accepted schema row or callback supplied by a client can attest to physical validation.

### Effect-owned physical validation

The derivative copy owner remains responsible for complete pre/post validation and each actual row's source/destination/descriptor checks. The private quote must accept only that exact projection and copier, retaining original lifetime/control and one-use semantics. A caller cannot substitute a copy implementation which returns success without those checks.

The common quote wrapper can therefore authenticate before/after its delegated derivative copy without running another identical traversal. **Do not apply this replacement automatically to sdk-clone.** Its separately inspected capture/clone owner must first establish the same delegated pre/post guarantee.

The admitted operation closes new clone/input births and drains all accepted tasks before its success fence. The quote scope must similarly close new publications, await its actual `publicationWork` and clone promises, then run its one final physical fence while quote/base/store/registry lifetimes are still genuinely retained as required. A callback error or `throw undefined` stays a real failure; accepted work still drains. No Error ledger is reset or converted to capacity DATA.

The projection owner remains the final independent fence after its caller's body and copy drainage. It must not let quote or copy completion mint a reusable “source still valid” capability. A second quote or nested projection has its own real lifetime and checks.

### Visibility/commit boundary must remain explicit

The production publisher renames staging to the final generation **inside** the callback, before the admitted/quote/projection final-return fences. Removing authentication scans must not be described as preventing every post-copy mutation from leaving any visible artifact. Existing post-copy checks guard clone completion, while installed/preflight validation and final outer checks cover different facts.

Before a later change consolidates outer fences further, the actual publish/rename owner must explicitly validate the required Source/installed-input facts immediately before visible commit and retain final post-drain validation. If a hostile mutation is discovered after visibility, success/activation must be withheld and cleanup must have exact owned artifact custody; one cannot erase a public directory merely because the callback failed. This is a proof requirement for any broader optimization, not an instruction to change publication behavior now.

## Required TDD Before Implementation

Use the existing genuine tiny owned-store fixtures and preauthored Original routes. The language-neutral contract should enumerate authentication versus physical-use boundaries, exact immutable facts, allowed event ordering and zero effects on invalid authority. Ajv validates the corpus; independent SQLite queries compare event counts/orders and actual Python SQLite/kernel attempts observe retained guards. A filesystem observer may count operations only while delegating every read/write unchanged; it grants no authority.

1. **Real duplicate-work RED:** In the tiny sealed production-shaped path, count full physical fence entries separately from every per-row hash/stat and immutable fact lookup. Current call ladder should exceed the authored five-fence contract. Do not count only a helper name or assert the implementation's own counter equals itself. Use delegated actual `lstat`/readlink observations plus independent corpus event counts; preserve complete content/inode/roster oracles.
2. **Pure identity boundary:** Repeated valid material/quote authentication returns the same captured bound and detached bytes without complete Source traversals or any write. Forged, copied, foreign-store/control, escaped and closed values still refuse before effects. Prove real shared/exclusive custody with actual lower issuer/SQLite observations; no public mode promotion.
3. **Mutation after authentication, before copy:** Replace a file, change bytes/size/mode, alter a symlink, add/remove namespace membership or swap an ancestor after requireQuote returns. Actual copy precheck must refuse; no copy-success marker or published generation. Include mutation between each authentication-only call and accepted effects to show no stored “valid” result is trusted.
4. **During-copy mutation:** Pause actual delegated file operations, mutate an already visited and a not-yet-visited source, then resume. The same complete per-file/rooted/postcopy checks refuse. Independent SHA and exact inode/roster observations remain; no file/hash skip or entry-cap increase.
5. **After-copy/body mutation:** Let clone complete, mutate Source during remaining callback work, then return. Admitted post-drain fence must refuse success. Repeat mutation after admitted publication returns but before the quote body returns, and after quote return before projection body return: the corresponding remaining outer fence must reject. These cases establish why the five fences cannot all be collapsed.
6. **Detached accepted work:** Callback starts actual clone/input work and returns early. Births close, accepted operation completes or fails under actual base SH/registry EX, and final validation occurs only afterward. An ignored rejected promise, Error, `throw undefined`, cancellation and late descendant all preserve complete drainage/failure propagation before loans release. Prove the whole root Original outcome, not only an inner catch.
7. **Cold seal and phase transitions:** Actual EX must conflict with a real independent base reader; no SH self-upgrade. Mutate before seal, between EX release/SH reacquisition and before copy; existing seal-time hashes and rechecks refuse. Keep updated privately owned stamps only after the authenticated seal.
8. **Limits and no new authority:** Exact-cap SourceTree root/nonroot case, 16-operation bounds, all file/byte/path ceilings, actual capacity DATA/no callback fake-green and wrong input/foreign link cases remain. No cold/full-clone minimum changes, new numeric exception or stale-generation reuse.

Qualify the focused contract/strict BOTH before running any full-size gate. Then retain bounded phase diagnostics: full-capture count/time; physical-fence count/time labeled by owning boundary; per-file copy/link counts/time; accepted-drain time; registry wait; final cleanup and Original settlement. A lower static count is not a speed claim. One-copy prototype/release must both complete under their existing 120-second Originals before normal fresh SDK/callback/Native/ordinary qualification continues.

## Scope and Protection Notes

This design does not weaken the initial content-hash capture or replace it with a previously written index. The existing metadata pre/post checks do not independently hash every file again; document that limit rather than calling them consumed-byte ABA proof. Actual shared SH excludes managed seal/retirement writers; it does not make arbitrary unadmitted filesystem mutations impossible. Preserve all existing no-follow/stamp/root checks and hostile laws.

Normal tooling retirement already allows unrelated generation readers when current reader-source authentication and candidate EX succeed. Historical copied formats still require image EX until complete historical opener evidence qualifies a narrower path. This revalidation proposal changes neither retirement authority nor that distinction.

No actual new Source/custody behavior is claimed. Approval is requested from Root for the bounded authentication/effect split only; broader commit-boundary or cross-version retirement changes need their own reviewed scope.

## Observed Source Digests

- `📦️sdk/🎟️capacity/📥️physical/🟦️.ts`: `cb658d801fc0c0b905e441f27da9e7529a2eba3c27d57fc66a8f3bee05d71433`
- `📦️sdk/📥️source/🔗️derivative/🟦️.ts`: `729dba66c6f845f3543806b79d749dbe24737747020fd06a61a54e64756369ed`
- `📦️sdk/📥️source/🔗️derivative/🔏️index/🟦️.ts`: `71721d022c90837470515368f101205bf6ee1d57e5c1d8ea33fb8816a0c755d3`
- `📜️script.ts`: `0fe0d04e28bdd116beaf48ac84021ece8ecc3b945962fcbc3084b50ceb42278d`

These are read-only current Source observations, not an authenticated installed SDK receipt or proof that an earlier process loaded these bytes. Related evidence: `sdk-async-ancestry-current.md`, `sdk-integrated-copy-independent-current.md`, `sdk-source-tree-capacity-current.md`, `tooling-concurrent-retirement-design-current.md`.
