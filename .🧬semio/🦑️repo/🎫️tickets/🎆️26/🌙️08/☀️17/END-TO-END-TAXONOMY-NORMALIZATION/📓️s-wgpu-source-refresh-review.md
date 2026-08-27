# WGPU User-Owned Source Refresh Review

## Authority Status

Transaction `11918` committed and released the catalog/schema freeze. The reviewed one-row source refresh and explicit joined-path authority are now installed; the earlier proposed/dormant stages below are retained as historical evidence. The current browser-frame-transport source was changed independently; this lane preserves it. A hash-only refresh was insufficient: isolated canonical execution exposed the joined-path consumer gap fixed by the exact three-operand authority.

The preceding full integration run was **19 passed, 2 failed, 213 assertions, 67.17 s**. Both failures stem from the exact WGPU source-preimage mismatch; this is not a green full acceptance gate.

## Exact Proposed Source Row

Package: `wgpu-renderer`.

- Source suffix: `🧪️browser-frame-transport.test.ts` under the old WGPU Cargo package.
- Destination suffix: `🧪️tests/🧪️browser-frame-transport/🟦️.ts` under the semantic WGPU target owner, unchanged.
- Old SHA-256: `65b94ae37128fa193b4ce4c465564ba819a520dc73d89fbf73a2f30b054650e5`; 15,004 bytes.
- Current SHA-256: `0c424a9c55af60b7d16b2f0dd7d567a9c394115031bbd9fc21ac0abb660fe143`; 16,208 bytes.
- Role/disposition: `implementation`, unchanged.
- Exactly one of 32 source mapping rows changes. All four WGPU adapters, one derived registration, two source splices, package identities and structural mappings remain identical.

The new test replaces the stale `OsHost::into_retirement` assertion with a bounded admission/retirement predicate and four mutation checks. The production Rust worker source and all adapter inputs retain their prior catalog hashes.

Hash-only candidate projection catalog: `b084bcd8d4a538f310aed3df377c5d0250d34433e0e9a772fcaef459d6d3b158`.

Hash-only candidate purity catalog: `d1db4c5ba6507e74c5e770a0818273087e433613e8d580ac0f6dcefb53045f3e`.

The structural golden has no source-preimage hash and needs no change. A future accepted refresh must update the purity row, composed projection row, and taxonomy's composed-catalog pin together. The joined-path binding contract may change the final composed digest; the above digests are proposals, not installed authority.

## Runtime Evidence

Command:

```text
bun test /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/🧪️wgpu-source-refresh-review.test.ts
```

Final review packet: **2 passed, 0 failed, 112 assertions, 24.70 s**. Each runtime leg used the actual repository `runVitest` interface through uncached fixture Nx, the installed Vitest 4.1.10, one worker and an explicit absolute test selector. Fixture-only config includes the semantic leaf explicitly; it is not proof that the current WGPU package task/config discovery is canonical-ready.

- Actual source: **17/17 passed**.
- Current normalizer plan, old source leaves removed: **16 passed, 1 failed** with the exact old-directory `ENOENT`.
- Fixture-only three-operand correction, still without old source leaves: **17/17 passed**.
- Independent TypeScript parser reports no parse diagnostics for source, planned canonical, and proposed canonical forms.

The current plan correctly rebases the direct transport import and Rust worker read. It rewrites `join(root, "🟦️typescript")` to a relative path back into the old Cargo-package directory; it does not bind the two subsequent `join(typescript, ...)` operands to their moved files. The required source-owned bindings are:

| Operand | Canonical value |
| --- | --- |
| `typescript` directory operand | `../..` |
| boot read operand | `🧵️browser-boot/🟦️.ts` |
| frame-worker read operand | `🧵️frame-worker/🟦️.ts` |

The target identities are checked against the existing catalog, not inferred from basename. The retirement predicate and all other test logic remain byte-identical.

## Retained Red Evidence and Limits

The first review harness run was **0/2, 100 assertions, 12.96 s**: one test incorrectly supplied an ignored argument to zero-argument `loadTaxonomy`, and retaining source copies masked canonical reads. The fixture was corrected to parse/validate its own schema and remove only its copied source leaves before canonical execution.

The second run was **1/1, 107 assertions, 32.55 s**. It proved the real `ENOENT` gap, then rejected an incorrect hand-proposed boot identity (`renderer-boot` instead of the catalog's distinct `browser-boot`). The final proposal uses the exact catalog row and adds direct target-identity assertions.

Retained examples: `🧪️wgpu-source-refresh-TGegYz/📓️plan.json`, its three runtime Markdown logs, and `🧪️wgpu-source-refresh-2SQRRF/📓️candidate-digests.md`. No production leaf, generator output, or Git state was changed. Only disposable fixture copies were removed, and their original contents remain available in the earlier fixture and production source.

This proves the transport test and source-inspection predicate, not a full renderer native/wasm build. Full WGPU generator, task/config, and external-consumer closure is still required before any production apply.

## Dormant Joined-Path Authority Implementation

`semanticPackageJoinedPathReferenceAuthority` and its two internal-repository interfaces are implemented in discovery, under `🧵️Source-Owned Joined Paths`. The helper takes an explicit binding contract, an exact mapped package owner and source/canonical facts. It has no callers in production catalog loading or normalization yet; no production behavior, frozen catalog byte or taxonomy pin changed in this stage.

The language-neutral binding is separately preserved in `🔣️wgpu-joined-path-bindings.json`, independent from the external one-row source refresh in `🔣️wgpu-source-refresh-review.json`. It requires exactly one consumer mapping, exactly two distinct implementation read targets, Node's fixed imports and the reviewed consecutive local binding declarations. Source admission requires the complete catalog hash and byte count before any token can be returned. It derives only three literal operands, using POSIX repository-relative coordinates and existing mapping destinations. All other source logic is untouched.

Independent TypeScript AST traversal confirms the three reported UTF-16 spans are the second string arguments of the intended `join` calls, with the expected local identifier graph. Nine hostile source mutations and nine malformed/ambiguous contracts are rejected with no admitted tokens. The source variants include dynamic operands, changed spans, untrusted imports, altered root identity, fake readers, comment substitution and a shadowing declaration. Canonical dynamic-read mutation is also rejected.

TDD history:

- One unrelated shared import window initially failed before tests: missing `taxonomyWorkspaceRoot`, **0 passed, 1 error, 0.491 s**. Root and the other lane independently confirmed restoration; this lane did not change those functions.
- Intended missing-helper red: **0 passed, 2 failed, 2 assertions, 1.118 s**.
- Initial helper green: **2 passed, 0 failed, 71 assertions, 0.566 s**.
- Added missing-ID rejection red: **1 passed, 1 failed, 56 assertions, 0.443 s**; fixed by explicit string validation, not coercion.
- Final complete small packet: **4 passed, 0 failed, 192 assertions, 12.89 s**. Source **17/17**, current canonical plan **16/17 with expected old-directory failure**, helper-proposed canonical **17/17**. The helper-produced bytes match the independently recorded three-operand proposal exactly.

The existing separate canonical WGPU authority plus actual Rust macro relocation compiler packet was rerun:

```text
bun test /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/🧪️nested-cargo-package-integration.test.ts --test-name-pattern 'WGPU exact registration relocation|canonical WGPU authority'
```

Result: **2 passed, 0 failed, 20 assertions, 3.73 s**. The compiler executes the original and canonical real macro registration with two minimal interfaces; both print `7`. It is not a full renderer compile claim.

## Pending Production Integration

After the active transaction releases the catalog/schema freeze, the required binding registry and exact parser, normalizer token admission and canonical checks must be integrated atomically with the reviewed source refresh. No fallback or shadow old-layout leaves are permitted. Transaction rollback/retry remains pending; no unresolved plan was filtered or forged to manufacture an apply result.

The retained isolated candidate plan still has **31 moves, 85 edits, 0 accepted regenerations and 18 blockers**: 15 unsupported authored tokens (Cargo comments, WGPU task/config selectors, Rust/source commentary and plugin bridge commentary), graph preview fixture-owner absence, frame-worker producer preview failure and the mandatory WGPU adapter/registration/generation completeness guard. These fixture findings are not a fresh repository-wide reference inventory. No global scan was run during the hold.

## Activated Boundary and Final Focused Gates

The composed projection catalog SHA-256 is now `675132887bf9c4e1e8ba5e4640e892648f0b109aada443e6dbda4ea7d4b67a74`. Discovery and normalization consume required `joinedPathBindingCounts: [1, 0]`, with one exact WGPU binding and no JCO binding. Catalog loading validates count, unique binding identity and mapped target authority. Source full-file preimages and canonical statement shapes remain strict.

The activation test first failed **0 passed, 1 failed, 1 assertion, 0.697 s**, then passed **1 passed, 0 failed, 6 assertions, 4.78 s**. Final complete review packet: **6 passed, 0 failed, 201 assertions, 24.01 s** using the absolute test path above. Actual source and the integrated normalizer's canonical output both passed **17/17** with no old-layout shadow leaves. Nine hostile source and nine malformed authority cases remain rejected.

The full current nested-Cargo packet was also executed using its absolute test path: **21 passed, 0 failed, 234 assertions, 44.64 s**. This includes the genuine JCO rollback, same-ticket ordinal-two retry, commit, empty replan, Nx generate/check and Cargo metadata proofs. It does not establish a WGPU apply: its exact producer/retirement and remaining consumer closure are the next bounded dependency, with the WGPU completeness guard retained. No fixture process from these gates remains active.
