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
