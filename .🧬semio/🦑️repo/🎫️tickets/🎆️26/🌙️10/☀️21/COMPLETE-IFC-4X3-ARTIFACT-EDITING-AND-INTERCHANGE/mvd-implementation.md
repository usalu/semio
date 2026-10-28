# Published IFC4x3 View Scope

The IFC4x3 artifact now retains the complete informative ADD2 entity and concept scope listings: all 876 entities and 312 concept paths, with direct RV/AbRV badges, inclusion causes and universal product/type annotations. The source is pinned to official [Standards-builds commit 610ee3f95db9d0380b1eb8de4dc6e17833531518](https://github.com/buildingSMART/Standards-builds/tree/610ee3f95db9d0380b1eb8de4dc6e17833531518/IFC/RELEASE/IFC4_3/HTML). SHA-256 provenance is retained for each complete input document.

The owned schema-first catalog lives under IFC4x3 `🧬️schema/👁️mvd`. It has TypeScript and std-only Rust interfaces and no third-party runtime dependency. Both resolve included, universal, unlisted and unknown scope distinctly. An unlisted badge is not interpreted as a prohibition. TypeScript data is deeply frozen, results do not expose mutable catalog arrays, and view labels require explicit English/German selection.

The exact release identifiers are `ReferenceView` and `Alignment-basedReferenceView`; display labels use Reference View and Alignment Based View. IFC4x3 entity details show both views' published scope in the selected language while retaining entity editing and removal. No invented DTV/COBie view or legacy IFC4 rule adapter was introduced.

## Verification

- Language-neutral `🧪️tests/mvd.feature` and `mvd-cases.json` preceded the public implementations. The fixture is consumed by Bun and the written Rust tests.
- Bun: two view tests pass, seventeen assertions, including all 876 catalog identities against the normative EXPRESS catalog, universal and explicit scope, unlisted/unknown distinctions, explicit locale and mutation refusal on frozen metadata.
- Independent `mvd-oracle.ts` uses existing parse5 to parse both immutable HTML documents independently from Python's standard-library HTMLParser. All 1,188 complete rows match exactly, including names, paths, badges and causes. Existing Ajv validates the full output against the owned draft-2020-12 JSON Schema; input hashes match.
- `mvd-build.py` is retained ticket input. Its JSON and Rust static data are production declaration metadata; no compiler artifacts or source copies were put in the ticket.
- Rust modules, UI integration and written tests pass rustfmt syntax/format checks. Native execution remains unverified through the authentic compiler ingress; no enforcement or build infrastructure was bypassed.

## Conformance Boundary

The input annex is informative. `conformanceEvaluated` is explicitly false. This feature provides complete published scope metadata and localized editor context, not model-view constraint execution, certification, or complete IFC geometry. Full concept-template graph and parameter constraints, EXPRESS semantic execution and native end-to-end host checks remain open goal work. See `mvd-source-research.md`, `schema-implementation.md` and `geometry-integration.md`.

## Files

- New IFC4x3 `🧬️schema/👁️mvd/{schema.json,🔣️.json,🟦️.ts,🦀️.rs,data.rs}`.
- New IFC4x3 `🧪️tests/{mvd.feature,mvd-cases.json,mvd.test.ts}`.
- IFC4x3 Rust module mount, TypeScript facade, editor details/test and package test-suite registration.
- Ticket `mvd-build.py`, `mvd-oracle.ts` and this report.

The same wave normalizes omitted optional `complex` in structural TypeScript validation, matching the shared graph declaration under concurrent development.
