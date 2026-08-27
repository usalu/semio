# Metadata Source Facts Independent Review

## Initial Rejection

The initial inspector was not accepted despite its five-assertion focused test. The coordinator's independent sixteen-case preflight executed through the workspace-scoped Nx router and found thirteen mismatches. Evidence is `🧪️metadata-facts-root-preflight/🧪️root-first-retry.log` and retained `🧪️metadata-facts-root-preflight/🧫️run-T52FSk/🔣️results.json`. The neutral input/expected projections are retained beside the harness.

The failures cover exact derive paths, unqualified and qualified crate aliases, identifiers containing `as`, grouped imports, inherited conditional modules, `cfg_attr` metadata evidence, valid trailing commas and numeric identifier suffixes, malformed absolute paths, and duplicate metadata attributes. Extern aliases and two lexical-decoy cases passed. This is a source-facts failure, not a production mutation finding census.

The first harness invocation failed before execution because `bun nx exec` inherited the package lifecycle name `nx`. The corrected root-router invocation executed all sixteen cases. Both logs remain.

Ownership was transferred to a fresh Terra High executor, `terra_metadata_facts_v2`, after the original lane repeatedly returned incomplete work. The revised implementation must use token ranges and structural parsing, add schema-validated neutral tests, and demonstrate independent syn parity. Root policy activation is not authorized by this packet.

## Extended Review

The first token-based revision passed the original sixteen cases, but the coordinator's eight new cases all failed. The 24-case run is retained at `🧪️metadata-facts-root-preflight/🧫️run-9Nnfsk`, with log `🧪️root-extended-first.log`. It exposed absolute/root-group/self import handling, qualified attribute decoys, forbidden contract path keywords, and inner-module cfg propagation.

The proposed syn oracle compared only declaration/alias counts, not the facts themselves. It was rejected as insufficient parity evidence. The executor is extending it to exact declaration, metadata, alias, and implementation projections with retained bounded compiler/runtime evidence. Lint diagnostics outside this region are not established as pre-existing without a baseline; no package-lint pass is claimed.

## Final Bounded Acceptance

The coordinator reran the corrected source through three independent entrypoints, all exit 0:

- `🧪️metadata-facts-root-preflight/🧪️root-extended-final.log`: all 24 adversarial projections passed; retained run `🧫️run-xmLy7c`.
- `🧪️metadata-facts-syn-oracle/🧪️root-final.log`: exact full-fact equality with the independent syn parser, covering 8 declarations, 15 aliases, and 1 manual implementation; retained run `🧫️run-QhPd9O` has compiler/runtime argv and output plus full comparison JSON.
- `🧪️metadata-facts-registered-root.log`: registered scoped repo-library test, 1 passed, 5 deep assertions, 293 filtered, 0 failed.

This accepts metadata syntax facts only. Conditional/malformed/ambiguous evidence is not active ownership. Package identity, actual wrapped declaration resolution, manual-alias rejection, and high-severity policy activation remain open. No package lint or global mutation census acceptance is inferred from these tests.
