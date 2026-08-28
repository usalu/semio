# Shared Source Admission Contract Proposal

## Status And Purpose

This is a proposed schema-first boundary, not an implemented API or a complete repository census. The root read the taxonomy lane's [API review](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/📓️mutation-source-admission-api/📝️.md) and the current normalizer union and discovery filesystem view. No root-script, normalizer, discovery, package-export or taxonomy-schema source was changed for this proposal.

Mutation discovery and the full normalizer must consume one admission authority. Package discovery, semantic active roots and the catalog filesystem view are not substitutes: each answers a different question. The projection must not create another area list, dot-directory filter or basename-based build/cache exclusion.

## Proposed Neutral Result

The language-neutral result contains a version, normalized scope, schema identity, ordered candidate rows and ordered diagnostics. Each row retains its physical repository-relative `sourcePath`, observed node kind, index mode/object identity when present, explicit-directory status, all admission origins, and matching generator contract/output-root identities. Physical spelling is preserved; NFC is used for scope comparison, not to silently merge distinct physical paths. Rows are deduplicated by exact physical spelling and sorted by UTF-8 bytes.

Admission origins are the closed set `tracked`, `nonignored-untracked`, `ignored-generator`, and `explicit-ticket`. They record independent facts; a tracked generated file is not relabeled authored merely because Git knows it. A generator match is derived from the schema's declared output roots and is returned even when admission came from Git or the explicit ticket. Generated rows remain visible in the projection; mutation consumers can audit them separately from authored declarations instead of silently dropping them.

The existing union's row precedence is preserved where compatible: stage-zero index identity is retained, and an explicitly observed directory carries its directory kind. Overlapping origins are accumulated rather than overwritten. Ancestor directories needed by the full normalizer are derived once from admitted rows, using the same scoped ancestry semantics as its current implementation.

No result claims a stable content snapshot. It reads no admitted leaf content, computes no source-content hashes, discovers no references and parses no package manifests. A downstream complete mutation census must additionally capture source bytes, revalidate them and compare final candidate membership before asserting completeness.

## Admission And Safety Laws

1. Git stage-zero entries and nonignored untracked paths are considered regardless of root-level location, hidden directories or familiar build-like basenames.
2. Ignored paths enter only through schema-owned ignored generator output roots or the explicitly supplied ticket scope. No filesystem-wide fallback walk is allowed.
3. Configured opaque prefixes are rejected lexically before filesystem observation. The active task's case-insensitive `compose` segment exclusion is an additional non-negotiable lexical boundary, including nested segments; real excluded paths are never used as test probes.
4. Scope matching preserves the current normalizer's normalized scope/ancestor semantics and conservative Unicode-safe Git pathspec construction. Lexical escape, absolute path, invalid separator and nonregular-node conditions cannot be interpreted as authored files.
5. A symlink leaf may be reported as a symlink node without dereferencing it. A symlink or non-directory ancestor prevents descendant access and is reported, never followed. Repeated reads must not rely on a memoized earlier ancestry observation as current proof.
6. Missing tracked paths are explicit absent observations retaining exact index identities, not active source leaves and not unconditional structural errors. Unresolved index stages are ambiguity diagnostics and cannot select source content. Neither condition may silently disappear from the public projection.
7. Overlapping generator roots retain all exact matching authority identities. Neither schema-output membership nor a filename suffix proves authored ownership.
8. Cancellation is checked before and after Git enumeration and during each admitted tree walk. Existing progress reporting is reused for bounded enumeration phases and work counts; cancellation returns no partial-success certificate.
9. Errors and source changes are surfaced. A consumer may not convert a failed admission phase to an empty set and report success.

## Bounded Implementation Footprint Proposed For Review

- **Normalizer contracts region:** add the neutral admission row/result/options types, using repository-owned types only. Options are the existing repository/scope/ticket/cancellation/progress/taxonomy inputs, not another root list.
- **Normalizer inventory helper region near `CandidatePath`, `gitRows`, `worktreeCandidate`, `explicitTicketRows`, and `ignoredGeneratorRows`:** introduce one internal collector that reuses those existing authorities and retains provenance. Add a single public `inventoryTaxonomySources` wrapper. The full inventory calls that same internal collector with its already loaded taxonomy, avoiding a second schema load or parallel membership algorithm.
- **Normalizer full inventory setup:** replace only the inline candidate-union block with the shared collector result. Directory classification, payload projection, reference extraction, transaction/recovery code and `referenceCoordinateRoots` progress work remain untouched.
- **Discovery filesystem primitive:** no edit proposed. Its public catalog view remains a useful reference for no-follow ordering, but its memoized kinds do not become fresh admission witnesses. Existing normalizer ancestry helpers should be reused/hardened within the collector's exact boundary where necessary.
- **Neutral schema/fixtures:** add the admission contract and test vectors adjacent to the normalizer's domain owner after schema location and diagnostics semantics are agreed. The current ticket contains preparation only.
- **Library TypeScript package export:** expose only the new public admission entry point and owned contract types if the actual root import requires that join. Do not export the private Git/tree helpers.
- **Root script:** after the shared API has direct tests, replace only mutation-source admission in `policyFindAllMutationsDirs` and `mutationTaxonomySourceFiles`. Their semantic analyses remain separate consumers. Do not change unrelated scaffold helpers or normalizer reference traversal.

## Acceptance Before A Source Release

Neutral cases cover all four origins and overlaps; root/hidden/build-named authored files; tracked generated files; ignored undeclared outputs; exact ticket scope; Unicode physical spelling and scope; missing/conflicted candidates; symlink leaves and ancestors; opaque-before-observation traces; cancellation; deterministic ordering and membership changes. A third-party JSON Schema validator validates the neutral contract independently of the implementation. Actual read-only workspace enumeration must agree between the extracted full-inventory admission and the public projection without executing generators or reading excluded content.

The source extractor, its consumers and the existing taxonomy regression selection must be tested together. A lightweight admission pass alone does not prove that every mutation has a canonical leaf, that source metadata is correct, or that all languages/consumers compile.

## Coordination Needed Before Mounting

The taxonomy lane should review the exact missing/conflicted diagnostic semantics, contract/schema location, and internal-collector/export footprint. Its current `referenceCoordinateRoots` progress work is disjoint and remains preserved. This proposal does not request a blanket hold, a pin refresh, a restoration, a generator publication or a cleanup.

## Reviewed Absence And Conflict Semantics

The root read the taxonomy lane's complete contract review. A legitimate worktree move can leave its old path in the intentionally unmodified shared index. Therefore every indexed path receives an explicit observation: present with its actual node kind, or absent with its original mode/object/stage identity. Absence does not fabricate zero-byte source, establish deletion intent, synchronize the index or prevent an otherwise valid empty replan. The full inventory consumes only the present population, preserving that existing behavior; the public admission result retains the absent observations for audit.

Nonzero index stages are different: retain every exact stage/mode/object tuple and emit an index-conflict ambiguity diagnostic. No stage is chosen for content, and a consumer cannot claim unambiguous coverage while that diagnostic remains. Keep existing `gitRows` reference/transaction callers stage-zero-only; a bounded underlying parser can expose all stages to admission without changing those callers' population.

Malformed or escaping paths, nonregular nodes, unsafe ancestry, failed Git commands, observation drift and cancellation remain failures, not absent-source observations or empty-success results. Missing declared active-output authority remains governed by the existing taxonomy loader; this API does not bypass or soften it.

The accepted schema location is `📚️library/🧹️normalization/🧬️schema/🔣️.json` with a named source-admission definition. Canonical library test-case leaves will hold its independent neutral vectors/schema and TypeScript test; their actual paths must pass taxonomy validation before mounting. The normalizer's current named source gates are still running, so implementation waits for their short explicit release. Ticket-only vector preparation continues.

## Mounted Projector And Incomplete IO Integration

The canonical schema, neutral vectors and TypeScript test are now mounted under the normalizer domain owner. The actual exported projector first passed fourteen neutral projections and three malformed-input checks at source SHA `202e887f54b33d2980d56d531f7e4416b667a352b766febcdda47a41aba49d30`. The canonical test subsequently ran through the registered source controller: eighteen tests, thirty-eight assertions, no failures, at N `0704c9528d3c4a6fd78f608ea30017ec22c87a5ab656546b0754432a2da1e5bf`. Exact output and input hashes are retained in [run-cDQygP](./🧪️source-admission-51/🧫️canonical-runs/run-cDQygP/📝️.md). These are projection checks, not filesystem or global mutation acceptance.

The corrected IO controller reached the actual missing `inventoryTaxonomySources` export before fixture allocation; [run-WPYiBP](./🧪️source-admission-io-51/🧫️runs/run-WPYiBP/🔣️.json) retains the genuine API red and unchanged source hashes. A prior controller syntax failure is separately recorded in its preparation report.

An initial mounted IO implementation passed its twenty-seven fixture checks, but root source review rejected release: full inventory still performed the old union before calling the new public wrapper, causing duplicate traversal and schema loading. The initial wrapper also lacked complete raw-path/parser/walker/error and membership-digest safeguards. The historical green fixture result is retained, not promoted to shared admission readiness. The agent is replacing the old union with one loaded-taxonomy internal collector and strengthening the corresponding source and IO tests. Root mutation discovery has not yet been switched to that incomplete boundary.

## Root IO Correction Boundary

The root now owns the collector correction; the former implementation agent is restricted to ticket-only executable regressions. The currently inspected source still normalizes raw index paths, silently filters malformed untracked paths, catches unrelated filesystem errors as unsafe ancestry, labels every non-directory leaf a file, omits the explicit-ticket nested-Git recursion boundary, and rereads the taxonomy to report its hash. Full inventory now uses one union, but its schema load still precedes the admission-specific option guards. None of those defects is covered by the earlier narrow green fixture.

Both public callers will validate all raw option paths before filesystem access, then validate repository-root ancestry before loading the taxonomy. Native absolute repository-local option paths remain supported; source identities and scope use strict relative slash paths. Git NUL records must decode losslessly, contain no empty interior record, retain exact spelling, and reject invalid paths rather than reducing the population. The untracked command must use directory-pruning ignore patterns for opaque roots, including the task's case-insensitive opaque segment, as well as output pathspec exclusion. Indexed enumeration can inspect the complete index without opening indexed source paths, retaining scoped ancestor identities without unsafe filesystem probes.

The collector will receive the one already-loaded taxonomy. Its reported schema hash must come from that loader's retained input snapshot. Candidate observation distinguishes absent, regular, directory, symlink, nonregular and typed unsafe-ancestor outcomes; unrelated IO errors propagate. Tree walks validate lexical membership before probing, preserve the schema-owned nested-Git boundary, check cancellation during traversal, and verify directory identity around enumeration. These checks are not an atomic content snapshot; downstream content and membership endpoint recapture remains mandatory.

The Markdown parser declarations and reference-coordinate implementation are outside this correction. The taxonomy task owns its independently reproduced parser defect and may change those disjoint slices; whole-file drift must be reported separately from admission-slice stability.
