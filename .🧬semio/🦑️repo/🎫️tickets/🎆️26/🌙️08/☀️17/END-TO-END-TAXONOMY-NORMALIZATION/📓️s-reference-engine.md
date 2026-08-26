# S-Reference Engine

## Outcome

Extended the normalization engine's structured reference coverage using the required `📓️h-generated-ref.md` audit. The production change is confined to `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts`; no schema, discovery, shared-test, root, manifest, Git, Compose, or generator contract was changed.

Final engine size is 2,441 lines / 143,169 bytes. SHA-256:

```text
69265cac42c5fb742f253f4eeefcac70be2ffe0efe9cf1f901d10a57c275cccd
```

## Structured coverage

- Rust: `#[path]`, `include*`, declaration-form `mod`, Cargo scalar and array paths. A moved module rewrites to an explicit relative `#[path = "…"]` declaration rather than producing an invalid identifier.
- TypeScript/JavaScript: imports, side-effect imports, exports/from, dynamic imports, require, worker/URL forms, path-bearing assignment/property literals, and recognized filesystem loader calls. Exact indexed path matching covers package, Nx, tsconfig and self-host loader constants without scanning arbitrary strings.
- Go: source imports, every whitespace-delimited `//go:embed`/`//go:generate` argument, `go.work use` scalar/blocks and `go.mod replace` targets.
- Python: imports, filesystem/resource calls, `__file__` joins, TOML packaging arrays and `module:function` entry points.
- .NET/native: project content/link/hint attributes, `.sln` project paths, resource APIs, native includes/source literals, and structured CMake command arguments.
- Data/docs/CI: TOML scalar/array strings, JSON/JSONC values and exact path-bearing object keys, YAML list/scalar values, raw HTML attributes in HTML/XML/Markdown, and embedded argv in launch/tasks/package/Nx and CI-shaped files.

Every emitted edit retains adapter, offset-bearing structured location, old/new value and preimage hash. Reference targets use precomputed exact, NFC, extensionless and Python-module indexes. Directory references participate through inventory source-to-normalized destination mappings even though directory changes are materialized by file moves. Unsupported quoted/bare path syntax becomes `reference-syntax-unsupported` only when it resolves exactly to an entry with a planned destination, avoiding blind replacement and silent stale references. Lexical opaque paths never enter the index or filesystem resolution.

## Census corrections

- A scoped historical-ticket kind now applies only when path pattern, source pattern and owned extension chain all match; ordinary `.md`, `.json`, `.txt` and other physical leaves fall through to global rules.
- Fixed and configurable package files are classified as configuration before glue analysis. A focused package inventory proved `README.md`, `package.json`, `tsconfig.json`, `📋️project.json` and `📜️script.ts` configuration-clean while implementation leaves remain structural.
- Exact fixed nested `.git` directories are inventoried as boundary nodes but their descendants are not traversed.
- Old role emojis are preserved for semantic-kind selection while the emoji-stripped slug drives generic-stem dropping. A real `📌️empty.md` now normalizes to `📝️.md` with zero violations.
- Emoji-decorated external `CNAME`, `Caddyfile` and `Dockerfile` sources resolve to their exact fixed names at the same scope. Focused inventories produced `CNAME`, `Caddyfile` and `Dockerfile` with the expected fixed contract IDs and no violations.

## Self-host relocation proof

A read-only synthetic plan moved:

```text
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json
→ 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📇️taxonomy/🔣️.json
```

The plan produced one move and seven attached structured edits across six paths: root `📜️script.ts`, `🔒️layering.json`'s exact JSON key, the taxonomy's own configured path value, both discovery loader/diagnostic literals, the normalization loader constant and the shared test fixture constant. In-memory preimage application reported:

```json
{"moveCount":1,"editCount":7,"editedPaths":6,"unresolved":0,"oldExact":[],"newDestination":"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📇️taxonomy/🔣️.json","compatibilityFallback":false}
```

Thus the old full path and discovery-relative `../🔣️taxonomy.json` literals are absent after the planned edits. The plan introduces no compatibility fallback.

## Verification evidence

Focused final-schema suite:

```text
$ bun test '🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts' -t '^taxonomy normalization'
15 pass
196 filtered out
0 fail
182 expect() calls
exit 0
```

Bun module build:

```text
$ bun build '🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts' --target bun --outfile /dev/null
Bundled 15 modules in 14ms
exit 0
```

Strict entry diagnostic command:

```text
$ bunx tsc --noEmit --pretty false --strict --allowImportingTsExtensions --target ES2022 --module ESNext --moduleResolution Bundler --types node '🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts'
```

It reports no normalization/discovery diagnostic. The command exits 2 solely for the two pre-existing transitive UI styling `ImportMeta.env` and `ImportMeta.glob` type declarations.

Additional runtime evidence:

```text
engine scoped inventory: 7 entries, 0 violations
engine scoped plan: 0 moves, 0 edits, 0 unresolved
package scope: fixed/configurable files configuration-clean; implementation leaves still structural
full non-Compose census: 102,702 entries; completed with indexed resolution
git diff --check (engine): exit 0
TODO/FIXME/placeholder scan (engine): no matches
```

## Acceptance checks

- [x] Concrete audited reference forms have structured adapters and deterministic offsets.
- [x] Exact and directory target resolution remains indexed; no per-token repository scan was introduced.
- [x] Unsupported path-bearing syntax affecting a planned destination fails closed.
- [x] Opaque paths are excluded lexically before target resolution and never read.
- [x] One self-host plan rewrites every exact loader/config reference with no fallback.
- [x] Physical-leaf focused suite passes 15/15.
- [x] Generator contracts remain deliberately out of scope for the following schema lane.

## Touched paths

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/📓️s-reference-engine.md`
