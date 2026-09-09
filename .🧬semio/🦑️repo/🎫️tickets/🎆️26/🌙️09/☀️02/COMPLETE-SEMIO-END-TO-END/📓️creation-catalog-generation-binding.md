# Creation Catalog Generation Binding

## Defect

The Shell checks its captured and currently displayed catalog generation before
dispatch, but the create request drops that generation. Hub selects its own
current catalog at acceptance. A catalog rotation between dispatch and acceptance
can therefore create the same kind using different executable authority from the
one the user selected. Existing postaccept catalog checks do not cover this gap.

## Closed Contract

- Create canonical field order: `schema`, `requestId`,
  `expectedCatalogGenerationId`, `kindId`, `name`.
- Status canonical field order: `schema`, `requestId`, `spaceId`,
  `catalogGenerationId`, `phase`, and `ready` only for Ready.
- Both generation values are required, nonzero lowercase 64-character SHA-256
  identities. There is no default, optional legacy form, or caller-owned
  executable/factory/descriptor field.
- The existing Ready tuple and cancel request remain unchanged.
- Hub rejects expected/current mismatch before durable acceptance, random
  document identity minting, factory execution, or publication. The accepted
  intent must validate expected generation equals its retained catalog generation,
  and the sealed request digest includes the expectation.
- All statuses describe the accepted intent's generation, never a replacement
  current catalog. Worker status correlation and Shell owner/progress correlation
  must reject missing or substituted generations before Ready/open.
- Catalog rotation during the later private opening also requires an immutable
  origin check; the existing generic document-open protocol is not expanded merely
  to carry a creation-only field. This final opening boundary is under audit.

## Ownership And Order

Root owns the shared TypeScript creation schema, its JSON Schema and common
language-neutral corpus, Shell request/owner/progress/open admission, and renderer
neutral laws. WGPU owns worker request/response types and parsers, retained
operation/runtime correlation, OS wire corpus and tests. Home owns Rust/Hub
contracts, preclaim rejection, durable digest/status derivation, constructors,
transaction/HTTP fixtures and the actual composition launcher. Terra audits
read-only opening and error boundaries.

Native96646 is an already-running pre-change cohort and remains the sole fleet
Cargo process. Do not confuse its eventual result with qualification of this new
contract. The session startup/cancellation full-OS baseline precedes production
creation changes. No cohort or oracle is bypassed.

## Verification Frontier

The existing Shell catalog corpus now has current/rotated/missing status rows,
admitted independently through JSON Schema. Renderer59764 is terminal RED1/708
skipped against unchanged production: the request generation was undefined and
both rotated and missing Ready generations were admitted. This is the intended
behavioral failure, independently admitted through JSON Schema. No behavioral
pass or native no-fact/no-event proof is claimed yet.

Required follow-up coverage: exact first acceptance and retry, missing/zero/
uppercase/wrong-type generation, canonical/reordered/duplicate/Unicode wire forms,
G1-to-G2 rotation before acceptance with zero durable facts/rows/events, mismatched
worker status with zero opening, and rotation during private opening with exact
release and no publication. The actual trusted two-peer journey remains required.

## 2026-09-09 10:08 UTC Browser Packet

The session baseline is complete: full OS48804 GREEN333/333 and WGPU's bilingual
notice gate GREEN2/2. Root advanced the TS creation schema and shared JSON Schema
plus common corpus; source22231 is GREEN46 structured cases and14 raw JSON cases
through AJV, TypeScript and Pack. The common corpus has20 requests,19 statuses and
7 catalogs. Missing/zero/uppercase/number/null/array generation cases were added
without turning other hostile cases into accidental missing-field failures.

Root request/owner/progress generation correlation is implemented. Renderer37854
is GREEN11/435 skipped for the creation host-owner group, following59764 behavioral
RED. These are not actual Hub acceptance or two-peer browser proofs.

The audit found descriptor-compatible G2 may still mount after legitimate G1
acceptance: existing generic plan/grant checks validate current catalog, not the
creation's captured catalog. Root is adding a creation-only mount gate that waits
for an independently validated browser-actor-ui-mounted identity, not merely the
document port's ready promise. Six neutral mount/retirement cases and their schema
are authored;82573 failed during Nx graph discovery before test execution and is
not a behavioral baseline. A fresh registered retry follows the independently
repaired Trinity registry path. Native96646 remains
the pre-new-contract cohort; none of its17 selected laws reads the common creation
JSON corpus, as Home verified. Rust/Hub creation changes remain queued there.

## 2026-09-09 10:22 UTC Worker Verification

WGPU's generation packet passes the full current OS suite:333/333 tests across
five files, exit0, in `🗑️generated/artifact-creation-catalog-generation-full-os.log`.
Its focused gate is GREEN2/331 skipped. Required nonzero generations now survive
worker request sealing, retained operation correlation and synthetic statuses;
missing and rotated server statuses are refused before projection.

Hub currently maps multiple independent conflicts to bare HTTP409, including
unknown kind and request identity collision. An OS catalog-refresh signal must
not claim this proves a catalog mismatch. The planned separate exact-owner
refresh-required response preserves canonical creation phases and never retries
or auto-resubmits a create. Hub/Rust validation and genuine mounted-peer
acceptance remain unproven.

## 2026-09-09 10:31 UTC Private Mount Admission

Renderer8979 reached actual test execution and is behavioral RED1/446 skipped:
`createArtifactCreationCatalogMountV1` did not exist. Root then implemented the
private exact-generation mount gate and wired it into Shell document opening.
The validated current actor mount identity must satisfy the captured generation
before identity publication. Document attachment also waits the independent mount
promise; ordinary port readiness cannot commit creation. Close, bootstrap fault,
rebootstrap and snapshot replacement failure retire the gate. Opening checks gate
liveness again before commit, including retirement after an already-resolved mount.
Commit removes the creation-only constraint from later ordinary rebootstrap.

The neutral mount corpus now has seven cases, adding close-after-mount, and
requires an independent AJV admission plus fast-deep-equal outcome comparison.
Renderer99071 is the current registered creation-owner group rerun; no pass is
claimed until it terminates. The generic document-open protocol is unchanged.

## 2026-09-09 10:53 UTC Mount Composition And Rebootstrap

Renderer99071 is terminal GREEN12/435 skipped, then the seven-case composed
document-opening law passed with the creation group in90454: GREEN13/435 skipped.
WGPU's independent full renderer run is GREEN711/711 across19 files; full OS
after the separate refresh disposition is GREEN335/335 across five files.

Terra identified a liveness gap in the initial wiring: rebootstrap rejected only
the mount promise while attachment first awaited the document port. A missing
successor port could retain the private target until the60-second deadline.
Root added an eighth neutral case with a permanently pending port and required
the creation gate to own the concurrent wait for both port and mount.78926 is
the actual missing-attach behavioral RED1/447 skipped. The implementation now
uses Promise.all, so either failure releases promptly; ordinary non-creation
rebootstrap is unchanged. Its focused verification is running.

The current renderer type snapshot80331 is RED946; root does not claim a global
type pass. WGPU corrected an extracted creation test-harness contextual typing
seam, and Terra is auditing the broader script/runtime type boundary. Home's
native96646 is terminal RED in law2 after two passing laws and successful1h42
compilation; stale derived inference fixture identities are the diagnosed cause.
Home owns that correction and the queued Rust creation/replay changes.

The private-only bootstrap failure/rebootstrap and snapshot-failure paths now
call the existing exact-entry backbone failure closer immediately. This also
rejects a still-pending socket-actor waiter, which precedes document attachment;
concurrent mount/port waiting alone would not cover that earlier stage. Ordinary
non-creation rebootstrap behavior remains unchanged. Independent review is active.

Renderer18733 is terminal GREEN2/446 skipped for the eight-case mount and composed
opening laws. Two same-generation cases commit, six rejected/retired cases release
exactly once, and the pending-port case fails with the close reason rather than
the document-opening deadline. This is helper composition coverage, not a claim
of actual mounted-peer end-to-end acceptance.
