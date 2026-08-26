# S-GENERATOR-PREVIEW-ENGINE

## Outcome

The normalization engine now consumes the mandatory schema-owned generator preview protocol end to end. Planning invokes only the exact owner-local `bun ./📜️script.ts preview-generated` command, validates one canonical JSON document, freezes expected bytes/modes/stale removals into `TaxonomyRegeneration`, and remains read-only. Apply runs the registered normal Nx target and optional check target, compares the entire output-root post-state with the frozen preview, and retains the existing journaled rollback, cancellation, progress, and empty-second-plan guarantees.

No external runtime dependency, compatibility fallback, arbitrary shell command, generator execution during expected-byte prediction, Git mutation, or `compose/**` / `temp/compose/**` access was added.

## Protocol and engine contract

Exports added in `🧹️normalization/🟦️.ts`:

- `TaxonomyGeneratorPreviewNode`
- `TaxonomyGeneratorPreviewManifest`
- `parseGeneratorPreviewManifest(content, expectedContractId, outputRoots, excludedRoots?)`

`TaxonomyRegeneration` now freezes the preview manifest and digest, exact input/pre-output/output node inventories, output roots, stale removals, generation command, and optional verification command. These records participate in canonical plan bytes, regeneration identity, `planDigest`, and expected/actual post-state digests.

The parser rejects:

- anything other than one compact JSON document terminated by one newline;
- noncanonical top-level/node key order, wrong schema version, or wrong contract ID;
- non-NFC, non-relative, excluded, out-of-root, duplicate, overlapping, or byte-unsorted paths;
- symlink/unknown node kinds, missing directory parents, file descendants, omitted output roots, and undeclared stale output;
- non-integer or out-of-range modes and malformed/noncanonical base64;
- expected/stale overlaps, stale ancestor overlap, noisy stdout, or nonempty stderr.

Owned generator loading independently enforces `previewTarget === <target project>:preview-generated`. The Nx manifest must own that target as `nx:run-commands` with the exact owner cwd and `bun ./📜️script.ts preview-generated`. `owned|external` is the complete ownership union; external generators remain stable unresolved findings.

## Planning, apply, and rollback

Planning includes schema-registered ignored roots exactly, never broad ignored traversal. Each affected owned contract:

1. inventories stable input and complete current output trees with hashes and modes;
2. invokes the read-only preview in the registered owner cwd;
3. validates complete pre-output coverage by expected nodes or explicit stale removals;
4. emits a regeneration only when source/output mutation or byte/mode/stale divergence requires it.

Apply revalidates schema ownership, target records, inputs, pre-outputs, preview bytes, regeneration identity, transaction separation, and opaque digests. It backs up every prior non-directory output while retaining the complete prior directory inventory, runs `bun nx run <target>`, requires the exact frozen output tree, runs the optional check target, and journals completion. Rollback removes complete output roots (thereby deleting newly created and undeclared outputs), recreates prior directories with modes, and restores all file/symlink preimages. Successful commit removes staging/backups but retains the canonical journal.

The permanent apply test deliberately fails after generator/check execution, proves byte-for-byte workspace restoration, retries in a fresh transaction directory, commits, then proves a second inventory/plan has no regeneration and no unresolved finding.

## Projection/schema coordination

The engine strictly consumes `sourceMutationDirectoryName` with no equality fallback and retains separate source/canonical uniqueness. It canonicalizes non-test descendants of changed schema mutation roots as projection-owned work and gives extracted scenario leaves `artifact-mutation-test-projection-v1`; schema-root descendant renames receive `artifact-mutation-source-canonicalization-v1`. The mutation golden remains green.

The live schema added CAD/Draw authorities during this lane. The normalization loader now validates their frozen loader-only unions—`commandDirectoryName`, member source segments, catalog/exact-owner tags, rationale literals, fixed/glue descendant nodes, and the three exact Draw `🦀️component.rs` source leaves—without claiming CAD/Draw planning support.

## Permanent tests and authority

The shared repo-lib test has an isolated `GeneratorPreviewProtocol` region with three language-neutral cases:

1. canonical manifest parsing plus third-party `fast-glob` filesystem-inventory parity;
2. malformed/noisy/path/root/mode/base64/order/stale negative cases;
3. exact Nx preview → rollback → retry/commit → empty second plan.

The permanent golden authority is `📦️packages/🟦️typescript/🧫️fixtures/🧪️generator-preview/🔣️.json`; tests do not depend on the active ticket. The ticket copy and probe artifacts are retained as evidence.

## Verification evidence

TDD began red: the first permanent selector failed at module load because `parseGeneratorPreviewManifest` did not exist (`0 pass`, `1 fail`, `1 error`). Final evidence:

```text
bun test '.../🧪️index.test.ts' --test-name-pattern='generator preview protocol|projects every registered golden bundle into artifact profile storage'
4 pass, 222 filtered, 0 fail, 160 expect() calls
preview protocol: 3/3 pass
mutation golden: 1/1 pass
generate and check targets each succeeded twice (rollback attempt and committed retry)
```

```text
bun -e '<loadTaxonomy + validateTaxonomy census>'
{"schemaVersion":7,"problems":0,"generators":18,"projections":3}

bun -e '<normalization inventory scoped to 🧹️normalization>'
{"scope":".../🧹️normalization","entries":7,"excluded":["compose"]}
```

```text
bun build '.../🧹️normalization/🟦️.ts' --target=bun --outfile='.../🧪️generator-preview-engine-build.js'
Bundled 15 modules; exit 0

scoped git diff --check (working tree and index)
exit 0

production/shared-test [DEBUG] scan
0 matches
```

A live registered `print-latex-tokens` preview parsed as canonical protocol bytes with `5760` stdout bytes, one node, and zero stale removals. The tracked output hash remained identical before and after preview: `f9bdeec85d32427af2df4842d9fb31505b25fd9eaf7eb21f771be356a10fd143`, proving the preview itself was read-only.

The repo-lib strict TypeScript command remains exit `2` because of six pre-existing/shared diagnostics outside normalization: two UI-styling `ImportMeta.env/glob` declarations and four cross-root `TS6059` imports. Filtering the same strict output for normalization/protocol paths produced no diagnostics.

## Deterministic digests

| Artifact | SHA-256 |
|---|---|
| `🧹️normalization/🟦️.ts` | `6e6559241adb23cfc92f48c161c9ff261707a666fbd7abc38b19293001dc3fb2` |
| retained Bun bundle | `0240203e6f9e00b955b2494c2892f8bf7bb63dba4b3b3f1245dc956d227c6dea` |
| permanent preview golden | `ebf63d6740e579549d5d50ef6f401cff676418a610493a2d89858c83935a9971` |

## Touched paths

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts`
- isolated `GeneratorPreviewProtocol` region in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧪️generator-preview/🔣️.json`
- this report and retained build/probe artifacts inside the active ticket

## Acceptance status

- Canonical JSON-only preview: pass.
- Exact schema/Nx ownership and owner-local invocation: pass.
- Complete stable input/output/stale plan records: pass.
- Read-only planning: pass.
- Exact post-state/check target: pass.
- Failure rollback, new-output deletion, retry, cancellation/progress integration: pass.
- Apply → empty second plan: pass.
- Third-party inventory parity: pass.
- Strict live schema/discovery and independent normalization loading: pass.
- Compose/temp-Compose opacity and Git-state prohibition: preserved.
