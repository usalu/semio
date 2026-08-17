# Wave 0 Frozen-Tree Luna Audit

## Verdict

**FAIL.** Narrow structural checks pass, but the Wave 0 contract is not frozen and Wave 1 must not start. Three independent read-only Luna extra-high audits found definition, identity, codec-support, atomicity, runtime, CQRS, and evidence-honesty blockers. No auditor edited the repository or ran a heavy build.

## Narrow passes

- The catalog and TypeScript facade enumerate 36 artifacts.
- EPW is MIME-unregistered while TXT owns `text/plain`.
- The format, composer, subset-validator, inference-service, and document-codec changes reject several conflicting registrations rather than overwrite them.
- Repository-owned codec result/diagnostic interfaces exist.
- Explicit locale has no default.
- Projection results carry revision/generation stamps and reject covered stale results.
- Stdio Nx and launch entries exist and the short structural ledger commands execute.

## P0 remediation blockers

1. **Identity grammar is wrong.** The ledger emits `stdio.<artifact>`, slash dialects, and arrow codec IDs. Framework dialect coordinates use a second `artifact@standard/subset` grammar. The frozen grammar is `s.stdio.<artifact>...standard...profile...dialect...codec...`.
2. **The 250-codec claim is invalid.** The script labels IO component-file existence and import/export paths as codec definitions without executable registration, selectors, support status, fidelity, provenance, validators, or vectors.
3. **There is no definition authority.** Stdio never assembles or consumes the framework `ArtifactDefinition`; it derives a minimal TypeScript ledger from the catalog plus directory scans.
4. **Cardinality is wrong.** `ArtifactDeclaration` carries `Vec<ArtifactDefinition>` and stdio still builds only 27 declarations for 36 artifacts, with legacy artifact-kind/setup registrations and duplicate PDF declarations. The contract requires exactly one definition per artifact with plural contents.
5. **Plugin assembly is non-atomic.** Setup and app-schema side effects occur before validation, declarations allocate independent definition registries, failure panics/asserts, and registration is sequential rather than a typed preflight/commit transaction.
6. **TypeScript parity is nominal.** The facade exports 36 namespaces, but representative artifact modules are `export {}` and the gate tests names only.
7. **Mutation vocabulary remains generic.** Fifty snapshot-replacement command trees, `NoMutation`, `SetSnapshot`, generic setters, clamping, and silent target no-ops remain.
8. **Current stdio Rust compile is unverified.** The earlier GLTF planning-path blocker is stale against a subsequent in-flight rename; no current-tree Nx Rust pass exists.

## P1 contract blockers

- The support ledger lacks normative source, publication date, checksum, redistribution, clauses/features, profiles/conformance classes, registered code points, read/write/lossless/canonical status, validator identity, mutation/inference identities, and fixtures.
- The manifest is a hand-maintained 36-row table with unproved neutral/binary/name claims; catalog dependency data is duplicated.
- Optional MIME handling is not representable through the artifact-definition builder.
- ArtifactDefinition/document-codec duplicate identity behavior rejects identical idempotent registration instead of accepting it only when descriptor and executable identity match.
- Child-store factory and dialect-migration registries still overwrite.
- Codec limits omit recursion depth, default to unlimited, and depend on voluntary manual charging; canonical/lossless and anchored lexical/opaque preservation are not executable requirements.
- Store mutation validation is not called before replay/event persistence, and reset can panic on malformed caller history.
- Projection causes omit replay, policy, and external-resource changes and are not a uniform automatic projection subscription.
- Native/WIT inference remains cold-only, rejects non-empty policy, and lacks source dialect, budgets, cancellation identity, prior state/cache-mode input, and typed diagnostics.
- Root inference construction still owns validity/quality/diagnostics/provenance for representative leaves.
- Script “runtime”, “fuzz”, and “cross-platform” targets are thin aliases, not proof of the claimed behavior.

## Remediation ownership

- Terra R0-A: exact one-definition-per-artifact contract, canonical IDs, optional MIME, typed plugin preflight/commit, open capability execution, and WIT/native inference request contract.
- Terra R0-B: enforceable codec budgets/fidelity/resource resolution, remaining conflict registries, mutation validation/reset safety, complete projection causes/subscription seams, and identical-registration rules.
- Terra R0-C: schema-owned machine-readable definition/ledger data, removal of filesystem support inference and manual manifest duplication, real TS parity gates, honest gate naming/evidence, and current GLTF glue integration repair without discarding concurrent GLTF changes.

Wave 0 is re-audited after remediation; primitive implementation does not begin before all P0 findings close.
