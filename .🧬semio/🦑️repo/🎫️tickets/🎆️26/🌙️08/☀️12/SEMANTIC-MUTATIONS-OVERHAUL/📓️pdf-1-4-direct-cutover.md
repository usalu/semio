# PDF 1.4 Any, A, and X Direct Mutation Cutover

## Outcome and Acceptance Boundary

The three PDF 1.4 roots now own nine concrete direct mutations: Any 5, A 2, X 2. The coordinator's existence-checked 17-rule policy reports zero violations for each exact root. This is structural acceptance, not compiler or runtime acceptance.

No Cargo build or test was started in this lane. The coordinator is running the shared registered STDIO test gate; the earlier library-only check did not close test-library compilation. The newly authored Rust semantic, serde, text, binary, and lopdf tests have **not** executed in this packet. No codec-runtime or PDF-file-runtime pass is claimed.

Rust/schema/fixture sources were frozen at the coordinator's checkpoint. A subsequent narrow authorization corrected five missing TypeScript closing braces only. Final work after that was read-only validation and ticket evidence.

## Scope

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️a/🧬️schema/🧬️mutations`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️x/🧬️schema/🧬️mutations`

Only genuine local mounts, consumers, catalogs, grammar/protocol assets, and tests were changed. The parent additionally authorized removing the stale PDF1.7 `NoMutation` block from the physical PDF1.4 bachelor-thesis test file; its adjacent real diff-identity law and still-used apply import remain. No other PDF1.7 source was changed. No textual/glTF audit targets, root scripts, taxonomy, AGENTS, or `compose/**` were changed.

Applicable root, S, and STDIO AGENTS were read. A final exact-ancestor check found no additional instructions for these source and glue paths. No modifying Git command or worktree was used.

## Exact Leaf and Wire Table

| Subset | Direct Folder | Aggregate Variant | Text Opcode | Binary Tag |
| --- | --- | --- | --- | --- |
| any | `📥️insert-page` | `InsertPage` | `insert-page` | 0 |
| any | `🗑️remove-page` | `RemovePage` | `remove-page` | 1 |
| any | `🔀️move-page` | `MovePage` | `move-page` | 2 |
| any | `📐️resize-page` | `ResizePage` | `resize-page` | 3 |
| any | `📝️replace-page-text` | `ReplacePageText` | `replace-page-text` | 4 |
| a | `📝️set-page-text` | `SetPageText` | null | null |
| a | `🧹️clear-page-text` | `ClearPageText` | null | null |
| x | `📐️set-page-size` | `SetPageSize` | null | null |
| x | `📉️collapse-page-size` | `CollapsePageSize` | null | null |

Any retains its true `PageDoc { width, height, text }` page-list domain, not PDF1.7's COS graph. Its five operations address an explicit page index; move destinations are final indices. A's two operations only change the first page's text. X's two operations only change the first page's geometry; collapse sets width to zero and preserves height.

Every descriptor is complete: explicit-mutation invertibility, apply-only diff participation, atomic composition, and applied/error outcomes. No detector is claimed. All nine own direct Rust and descriptors. Any additionally owns five payload JSON Schemas, TypeScript, GraphQL, protobuf, and direct Rust text/binary facets. A/X remain Rust-only with null wire identities, matching their actual declared surfaces.

Any's JSON/serde/TypeScript union uses the same kebab-case `mutation` discriminator and bare `payload`. There is no Apply/Restore phase wrapper, arbitrary Diff/Snapshot payload carrier, sentinel, or generic snapshot operation. All emitted JSON payload schemas are named `🔣️payload.schema.json`; none use the JSON Lines emoji.

## Behavior and Inverse Closure

| Operation | Concrete Inverse Data |
| --- | --- |
| Insert page | Remove the inserted index |
| Remove page | Insert the removed `PageDoc` at its original index |
| Move page | Swap source and final destination indices |
| Resize page | Restore that page's original width and height |
| Replace page text | Restore that page's original text |
| A set/clear first-page text | Set the original first-page text, including the empty string |
| X set/collapse first-page size | Set the original width and height, including zero |

Each leaf owns its payload, validation, sparse page diff, inverse, label/target, and tests. The roots contain direct mounts/reexports, wrapped aggregates, generic delegation, and structural catalog assertions. Hand-maintained root `KINDS` constants were removed after the independent gate caught them; catalog assertions now read the derive-owned `SemanticMutation::kinds()`.

The public codec registries execute direct leaf callbacks. They do not serialize the whole aggregate and do not retain unused decorative registries. Any's binary frame is format byte 1, tag 0–4, then native little-endian u64 indices, finite f64 geometry, and u64-length-prefixed UTF-8 text. The text frame is the semantic opcode plus `payload=` and hexadecimal UTF-8 JSON of that leaf's bare payload.

Every inverse fixture is language-neutral and asserts full expected snapshot and concrete inverse operations. The Rust leaf tests additionally assert serialized union shape and round-trip forward/inverse values through canonical text/binary codecs. Insert-page's nested page parser enforces exactly the three declared required page fields. The production inverse planners, not manually transcribed undo implementations or oracle inverses, are now called by all three subject adapters.

### Domain Guard Decision

The codec's documented page-list domain requires at least one page. New operations reject missing targets, invalid indices, non-finite new geometry, and removal of the final page. A/X no longer create a page when an invalid empty snapshot is supplied; they reject it atomically and return no inverse. This explicit guard was reported to the coordinator because the old first-page helper silently created a page and could not be inverted by the two-operation subset vocabularies. Finite zero and negative extents are not silently normalized; X's collapse remains representable and invertible.

## TDD and Independent Evidence

Nine committed fixture files preceded implementation. The owner-presence check first failed 9/9 missing, then passed 9/9 present. The old 12-file snapshot tree was backed up to `🔣️pdf-1-4-legacy-baseline.json`, removed, and its 12 proven-empty directories removed. Those source contents remain recoverable from the ticket baseline.

Executed validation:

| Check | Result |
| --- | --- |
| Independent existence-checked policy | Any 0/17, A 0/17, X 0/17 |
| Ajv descriptors / payloads | 9 / 5 accepted |
| Ajv wire positive/negative checks | 35 checks, 0 errors |
| Internal validator vs Ajv | 55 checks, 0 disagreements |
| Lodash 4.18.1 independent page-vector output | 18 forward/inverse checks, 0 errors |
| Direct required surfaces / vectors | 39 surfaces, 9 vectors, 0 missing |
| Executable registry references / KSY tag parity | 20 callback references, 5 tags, 0 errors |
| Canonical root mounts | 3 roots, 0 missing targets |
| Bun and TypeScript 5.9.3 parsers | 8/8 each |
| Pinned nightly Rust syntax parse | 30/30 |
| Scoped rustfmt check | 30 files, exit 0 |
| Scoped git diff check | Exit 0 |
| Scoped debug / forbidden ownership markers | 0 / 0 |

The lodash oracle independently constructs the same nine page-vector outputs using third-party array/object operations; it is not a PDF-byte oracle. The test-only lopdf producers/readers and committed 65-page thesis scenarios were updated, but are pending runtime. There are 18 mutation/inverse scenario rows plus three explicit codec identity scenarios. Existing sentinel identity specs were replaced by explicit reference codec round-trip functions.

GraphQL/protobuf native compiler packages and Kaitai compiler were unavailable locally. GraphQL/protobuf identity and direct schema reachability were checked structurally; YAML parsed the Kaitai document and its exact tag map. No native parser/compiler pass is claimed for those formats. Rust syntax parsing does not prove trait resolution, derive expansion, linking, test compilation, or runtime.

No temporary runtime `[DEBUG]` source was left behind. A Rust debug probe was not injected during the shared source freeze; full runtime confirmation remains the coordinator's gate.

## Files and Evidence

- Current mutation-root files: Any 64, A 7, X 7 = 78.
- Exact currently owned paths including local glue/consumers: 96 (34 Rust).
- Removed legacy paths: 12.
- Exact path and SHA-256 list: `🔣️pdf-1-4-direct-files.json`.
- Complete executed commands and outputs: `📓️pdf-1-4-direct-verification-commands.md`.
- Compact output transcript: `🧪️pdf-1-4-direct-validation.log`.
- Independent policy red/final: `🧪️pdf-1-4-independent-policy.log` and `🧪️pdf-1-4-independent-policy-final.log`.
- Removed source recovery: `🔣️pdf-1-4-legacy-baseline.json`.
- Prior bounded textual fallback audit: `📓️textual-inverse-carrier-audit.md`; no textual source was changed.

The shared glue file's digest is point-in-time evidence only; this lane changed only the PDF1.4 mutation mount hunk, removing duplicate codec mounts and all nested legacy mounts. Concurrent unrelated changes were preserved.
