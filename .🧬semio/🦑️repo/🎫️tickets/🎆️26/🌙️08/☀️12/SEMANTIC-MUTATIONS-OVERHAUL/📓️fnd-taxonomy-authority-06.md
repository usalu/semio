# FND-TAXONOMY-AUTHORITY-06

## Scope

The sole runtime taxonomy authority is now root [`📋️project.json`](../../../../../../../../📋️project.json) at `metadata.semio.taxonomy`, with the frozen value `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`.

Changed production surface:

- root [`📋️project.json`](../../../../../../../../📋️project.json): one `metadata.semio.taxonomy` field;
- [`🔍️discovery/🟦️component.ts`](../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts): `WorkspaceTaxonomyAuthority` and the sole no-follow manifest resolver used by the taxonomy loader.

The loader has no source-relative taxonomy-file fallback. It validates the exact root `nx.json` + `📋️project.json` marker pair. It validates `metadata.semio.taxonomy` before target access: string-only, nonempty repository-relative slash path; no NUL, backslash, drive prefix, absolute path, empty/dot/parent segments, or ASCII-case-folded `compose` segment. Root/start-directory APIs reject raw NUL, drive-relative, dot/parent, and ASCII-case-folded `compose` components before normalization, then `lstat` every root ancestry component, the marker pair, each taxonomy target component, and the final taxonomy file before either manifest/taxonomy file is read. An encountered `nx.json` is an anchor: a missing, symlinked, or non-file sibling `📋️project.json` fails at that anchor instead of borrowing an outer workspace authority. Project-only files do not anchor traversal.

## Neutral Evidence

[`workspace-taxonomy/🛂️schema.json`](../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️workspace-taxonomy/🛂️schema.json) validates the language-neutral vectors in [`workspace-taxonomy/🧫️fixtures/🔣️.json`](../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️workspace-taxonomy/🧫️fixtures/🔣️.json) through Ajv. The registered test materializes each vector in a virtual workspace and uses the existing Node filesystem/path primitives plus `lstat` to independently establish regular-file versus file-, directory-, and manifest-symlink states. It retains each virtual tree beneath `SEMIO_TEST_ARTIFACT_DIR` when the gate provides that artifact directory; otherwise it uses the platform temporary root after `realpath` and cleans up. The virtual `compose` vector is lexical only; no real `compose/**` path was inspected.

Covered vectors: missing/type, absolute and Windows drive forms, backslash, empty/dot/parent, NUL, virtual lower/title/upper-case compose, root and child-directory positives, terminal/component/manifest symlinks, exact marker-pair absence, symlink ancestry, raw `compose/../` plus `symlink/../` root/start inputs, root NUL/drive-relative inputs, and a valid outer authority shadowed by malformed nested `nx.json` anchors. The child-directory case invokes the exported start-directory resolver.

## Commands and Results

| Command | Result | Evidence |
| --- | --- | --- |
| `SEMIO_TEST_BUDGET_MS=180000 bun nx run @semio-tech/repo-lib:test-quick --skip-nx-cache -- --timeout 30000 -t 'workspace taxonomy authority'` before implementation | expected compile/import red: missing `resolveWorkspaceTaxonomyAuthority` export | [`🧪️fnd-taxonomy-authority-06-red.log`](../🧪️fnd-taxonomy-authority-06-red.log) |
| nested-anchor regression before enforcement | expected behavioral red: case-folded compose locator and malformed nested anchor could bypass their intended boundary | [`🧪️fnd-taxonomy-authority-06-anchor-red.log`](../🧪️fnd-taxonomy-authority-06-anchor-red.log) |
| `SEMIO_TEST_ARTIFACT_DIR=.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️fnd-taxonomy-authority-06-fixtures SEMIO_TEST_BUDGET_MS=180000 bun nx run @semio-tech/repo-lib:test-quick --skip-nx-cache -- --timeout 30000 -t 'workspace taxonomy authority'` | 3 pass, 35 assertions, 0 fail; retained virtual fixtures | [`🧪️fnd-taxonomy-authority-06-anchor-green.log`](../🧪️fnd-taxonomy-authority-06-anchor-green.log) |
| `bun -e` direct module probe calling `resolveWorkspaceTaxonomyAuthority(process.cwd())` and `loadCatalogTaxonomy()` | `[DEBUG]` canonical manifest locator and 72 file kinds | [`🧪️fnd-taxonomy-authority-06-loader.log`](../🧪️fnd-taxonomy-authority-06-loader.log) |
| `... -t 'taxonomy normalization'` | terminated after independent live-workspace catalog failures/timeouts (missing registry script input and changed golden count); no authority assertion failed before termination | [`🧪️fnd-taxonomy-authority-06-normalization.log`](../🧪️fnd-taxonomy-authority-06-normalization.log) |
| scoped `git diff --check` | clean | command executed after focused green |

No Cargo command ran.

## Limit

The no-follow contract is an `lstat` preflight. Node's cross-platform synchronous `readFileSync` cannot retain an `O_NOFOLLOW` descriptor across every component, so a malicious concurrent replacement after the checks and before a read remains a filesystem TOCTOU limit. This packet prevents normal symlink traversal and validates all components before reading. Root/start path inputs intentionally have stricter raw lexical rules than the existing compiler source helper: they reject dot/parent inputs rather than changing that helper's valid `consumer/../leaf` behavior. One early relative artifact-directory invocation was moved intact to [`🧪️fnd-taxonomy-authority-06-mislocated-artifacts`](../🧪️fnd-taxonomy-authority-06-mislocated-artifacts); subsequent runs resolve a relative `SEMIO_TEST_ARTIFACT_DIR` from the workspace root. Descriptor/proc-macro authority and broader taxonomy normalization remain out of scope.
