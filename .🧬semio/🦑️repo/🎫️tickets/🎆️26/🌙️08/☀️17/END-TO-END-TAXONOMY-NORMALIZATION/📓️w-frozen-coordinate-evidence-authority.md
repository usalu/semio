# Frozen Coordinate Evidence Authority

## Current Boundary

The CAD/Draw projection fixture was not previously registered as frozen evidence. The distributed CAD manifest contract and inline Draw vectors did not bind that separate fixture's bytes. The new explicit registry now binds its exact path, full-byte SHA-256, version, and typed coordinates; neither a filename convention nor the word `fixture` supplies authority.

The root task's 236-document transaction durably committed before the schema boundary was released. The registry is now connected to the full validated taxonomy and both planning and terminal reference closure. A fresh load returns zero validation problems with CAD/Draw and README registrations. The first full CAD evidence lifecycle and digest-drift negatives pass; the expanded regression packet is still in progress at this checkpoint. No production CAD/Draw or artifact move has been applied by this lane.

## Exact Contract

The prepared `frozenCoordinateEvidenceContracts` map associates a stable contract id with:

- one exact non-opaque repository-relative JSON document `path`;
- its full-byte `sha256` and an exact document `schemaVersion` presence/value declaration;
- a nonempty list of `coordinates`, each with an exact JSON `pointer` and `kind` equal to `source` or `destination`.

The only wildcard is a complete pointer segment `*`, and it must select an array index. Object-wide wildcards, duplicate paths, duplicate or overlapping selectors, missing coordinates, non-string values, escaped value spans, duplicate JSON keys, undeclared absolute values, schema-version drift, and byte-digest drift are rejected. Source/destination coordinates are historical facts, not runtime fallback paths.

Relative coordinates retain the exact `{pointer, kind}` grammar. A causally proven absolute observation instead requires both `representation: "recorded-repository-absolute"` and `recordedRepositoryRoot`. That root is an exact lexical POSIX or drive-qualified prefix; the value must have a nonempty normalized repository-relative suffix after it. Root-only values, prefix lookalikes, escaping/doubled segments, drive changes, case changes, backslash/UNC substitutions, missing root declarations, and unknown representations are rejected. The recorded root is never resolved, traversed, or treated as the current workspace. Eleven language-neutral cross-platform value cases and eleven malformed declarations cover this boundary.

A positive `schemaVersion` requires that exact property and value. Explicit contract `schemaVersion: null` instead requires the document property to be absent; an actual null-valued property is rejected. This is explicit authority for proven unversioned observations, not a fallback for unregistered or older schemas. No historical document is changed to add a version field. Eligibility of the additional unversioned Draw observations remains under a separate causal producer/consumer audit.

The declaration does not exclude the document from inventory, candidate discovery, or taxonomy normalization. Only matching value spans are protected. A registered document with bad bytes or selectors aborts with `frozen-coordinate-evidence-invalid`. A relevant path-bearing token outside declared spans in a valid bound document produces `frozen-coordinate-evidence-unowned`, never a content edit that silently invalidates the registered digest. Ordinary neighboring JSON retains normal incoming-reference rewriting.

The implementation validates observed registered documents before the literal incoming-reference admission filter, so deleting old path text cannot hide digest drift. An observed registered node must remain a no-follow regular file. Exact spans are cached once per read buffer, document path, and loaded taxonomy; a separate taxonomy-local path index prevents hashing/parsing the entire document for each of its 451 tokens. Existing exact catalog and canonical self-digest plan authorities remain separate and unchanged.

## Prepared Documents

The CAD/Draw document is the existing `🧪️cad-draw-path-projection/🔣️.json` fixture under the repository library's TypeScript fixtures. Its 122,279 bytes remain unchanged, SHA-256 `1410a74ccc87561fd4a4b91db7d503614fe21ddce8bc78dee923d8237820f3e0`.

Its six typed selectors identify 451 values: 227 source coordinates and 224 destination coordinates. They cover projection source/destination roots, mapping source/destination paths, recorded consumer path identities, and the two Draw reference-edit destination paths. The CAD move itself affects exactly 210 historical source values: one source root plus 209 mapping sources. Draw has twelve analogous source values. Partial strings and prose are not declared coordinates.

The README lane prepared its actual language-neutral authored-correction vector with SHA-256 `35db5a0ae52f6e4779453782708ee65efbda63e40439bc1124e022f78cd183b0` and exact `/authority/sourcePath` and `/authority/destinationPath` locations. Its earlier proposed-change audit is not granted a new exemption; that lane converted its source reference to an explicit historical catalog pointer.

The language-neutral registry contract and negative cases are retained in `🧪️frozen-coordinate-evidence/🔣️.json` under the repository library's TypeScript fixtures. The CAD/Draw and README source vector bytes themselves remain unchanged.

## Executed Tests

The pure ticket packet `🧪️frozen-coordinate-evidence.test.ts` passed **4 tests, 506 assertions, 0 failures in 0.841 seconds**. Ajv independently validates the contract grammar and five language-neutral version-presence cases. `jsonc-parser` independently reproduces every one of the 451 exact value spans. The runtime negative cases cover changed bytes, document schema version, missing/non-array/non-string coordinates, empty/drive/escaping values, duplicate JSON keys, escaped values, and overlapping selectors. Initial missing-export red was observed; a test's JSON-parser array-index conversion was corrected before parity. The explicit absent-version addition separately failed the old positive-only validator before implementation: **0 passed, 1 failed, 2 assertions**.

The new exact CAD integration case failed as expected: **0 passed, 1 failed, 3 assertions in 14.86 seconds**. Its read-only scoped plan proposes edits inside the unchanged golden because production wiring is still absent. The fixture and source bytes are retained. A neighboring ordinary `sourcePath` consumer is part of the same test and must remain rewritten once the evidence boundary is connected.

After live wiring and explicit recorded-root support, the pure packet passes **6 tests, 542 assertions, 0 failures** (latest run 3.37 seconds). The new recorded-absolute feature first failed its validator test with **0 passed, 1 failed, 1 assertion** before implementation. The independent JSON parser reproduces the exact accepted POSIX and drive-qualified spans as well as all 451 original CAD/Draw spans.

The first registered CAD lifecycle passes in 87.94 seconds: the read-only source plan proposes no golden edits, an ordinary neighboring JSON consumer is rewritten, injected after-edit failure rolls back, retry commits, the canonical replan is empty, and golden bytes are unchanged throughout. Digest drift and erased-target negatives also pass. Unowned-coordinate, no-follow, concurrent-drift, and full compatibility regressions are being completed before release. The README lane is running its separate nine-case packet against the shared implementation.

Scoped `git diff --check` passed for the normalizer and discovery. No production file was moved or edited by this evidence lane; actual Compose roots remain opaque.

## Remaining Verification

Complete the expanded CAD/Draw and README packets plus generic incoming-closure regressions, record exact final digests, and request root approval before any new real global capture or apply. The normalizer consumes the full validated discovery authority directly; no partial-schema cast or fallback was introduced for this map. Additional historical observations require the causal writer/consumer audit and explicit row approval; executable retained payloads and the live package-purity test are not frozen by this change.

The earlier full CAD result had 337 edits versus 75 declared consumers. The 210 historical CAD values explain part of that difference, but the remaining 52 are not classified without a complete fresh plan body. They are not assumed to be either errors or exemptions. The Draw capture is now retained in full and audited in [the exact plan-body report](📓️s-draw-full-ticket-reference-plan.md). It contains twelve CAD/Draw-golden coordinates, four package-purity-golden coordinates, 77 other-ticket JSON edits, and eighteen unsupported-syntax findings in addition to runtime/build and policy consumers. Those additional historical/executable distinctions must be proven; the new registry is not permission to freeze them indiscriminately.
