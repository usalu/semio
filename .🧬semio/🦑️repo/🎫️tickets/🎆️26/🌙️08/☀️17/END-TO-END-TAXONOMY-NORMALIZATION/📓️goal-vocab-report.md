# 📓️ Vocabulary-Gap Closure Report

Families found (full detail in `📓️goal-vocab-census.md`): (1) a `fileKinds` emoji/extension-family
mismatch (svg 🎨️ vs. real 🔣️; `model-3d` lumping 4 distinct physical formats under one emoji) that
was the true cause of ~1300 rows disguised as vocabulary gaps; (2) `asset-subject`'s pattern
rejecting underscore/mixed-case content names; (3) ~90 genuine event-sourcing/actor-lifecycle domain
words (`admission`, `activation`, `payload`, `composition`, …), 72 with one consistent on-disk emoji,
11 with conflicting emoji per subsystem (not registered — real ambiguity, not a gap); (4) the
`test-case`/`test-fixture-member` 🧪️ collision, root-caused to `matchDirectoryKind`'s ambiguity
fallback whenever a co-located test file's parent isn't a recognized test-hosting context; (5) a
config-leaf family (`vitest.config.ts` etc.) correctly a `fixedFilenameContracts` case but blocked by
a hardcoded validator whitelist in the peer-owned `🧹️normalization/🟦️.ts`.

**Kinds added (68, `semanticDirectoryKinds`)**: `activation` `assembly` `authority` `base64`
`binding` `bindings` `bootstrap` `budget` `bytes` `cancellation` `causal-add` `clock` `commit`
`compare` `composition` `content` `cooperative` `copied` `copy` `credit` `enqueue` `entries`
`evidence` `fault` `fixed` `framing` `graph` `inbound` `inbox` `index` `json` `lifetime` `list`
`local-interaction` `message` `mutation-leaf-contract` `mutation-leaf-source-contract` `nodes`
`numeric` `operations` `ordered` `ownership` `pack` `page` `pages` `payload` `pending` `poll`
`read-lease` `reader` `release` `resident` `response` `retirement` `return` `set` `slot` `source`
`string` `tail` `transaction` `tutorial` `update` `validation` `value` `whole` `wire-retirement`
`writer` — each reusing its already-established on-disk emoji, none inventing new vocabulary.
Widened `asset-subject`'s `slugPattern`; extended `test-case.parentKindIds` with all 68 + `builder`.

**fileKinds fixed (not added, corrected)**: `svg` emoji 🎨️→🔣️ (690:2 real-usage evidence);
`model-3d` split into `cad-source-model` 📐️ (`.3dm`/`.stp`/`.step`), `mesh-model` 🧊️
(`.glb`/`.gltf`/`.obj`/`.stl`/`.ply`/`.las`), `drawing-2d-model` 🖊️ (`.dxf`/`.dwg`),
`building-model` 🏗️ (`.ifc`) — one partition per real, already-consistent on-disk convention.
`fileKindResolutionRules` remapped accordingly.

**Contracts NOT added**: config-leaf `fixedFilenameContracts` (vitest/tailwind/postcss/eslint) were
drafted, correctly reasoned as the right bucket, but reverted — `packageSourceDispositions.validator`
is independently re-validated in `🧹️normalization/🟦️.ts::parseTaxonomy` with the same hardcoded
3-value whitelist I don't own; any new token fails `clean taxonomy plan` at load time. Flagged for
whoever owns that file to generalize the same way I generalized the discovery-side copy (then
reverted to match).

**Reverted mid-flight, documented**: `input` (collided with an existing `members-of-…-modules`
overlay, caught by an existing UI-host test); `hash`/`fonts` (their real on-disk emoji, `#⃣`/`🗚️`,
fails the taxonomy's own `Extended_Pictographic`+VS16 emoji-validity gate — unregisterable without an
on-disk rename, out of this slice's scope).

**Tests**: extended `🧪️tests/🧪️package-language-kind-handoff/🔣️.json` with 71 new `cases` (one per
registered kind, one end-to-end co-located-test-case regression, one nested-fixture-member regression
guard) plus the AJV/independent-oracle infra already in
`📦️packages/🟦️typescript/🧪️index.test.ts`. Real run: `6 pass, 0 fail, 1008 expect() calls`
(`bun test … -t "package language semantic handoff"`). Full-file run surfaced 172 unrelated failures,
traced to a pre-existing, concurrent-session gap (`generatorContracts["wgpu-frame-worker"]` tracked
outputs missing on disk) — confirmed zero diff lines touch `generatorContracts`/`wgpu` in my change,
and the missing files are independently absent from the filesystem; not mine, not chased.

**Before/after** (`clean taxonomy plan --scope "🧰️framework/🔨️modules" --baseline
bb06c41f73f0122fbed315b7487428b976f99921`, plan at `🗑️temp/🔣️vocab-plan.json`):

| metric | before | after |
|---|---:|---:|
| `moves` | 953 | **2082** |
| `edits` | 3651 | 4160 |
| `unresolved` (total) | 2631 | **1477** |
| `semantic-stem-unresolved` | 1495 | **449** |
| `semantic-stem-ambiguous` | 167 | **84** |
| `directory-kind-unresolved` | 111 | **31** |

`moves` rose as intended — stems resolving into neutral folders, not a regression.

---

## Round 2 (continuation, same session)

**`input` — landed, scoped correctly.** The collision was real but the concept wasn't wrong: added
`input` (📥️) with `parentKindIds: ["content"]` (the `content` kind registered in round 1). It now
resolves under `🎠️kernel/📤️return/📦️content/📥️input` and leaves the UI-host
`members-of-…-modules` overlay at `🖥️host/📥️input` untouched (different parent, `contextAllows`
fails there, overlay still wins). Regression-tested both directions.

**`hash`/`fonts` — the gate is correct; one was an authoring mistake, one is a live 16-consumer
crate.** Checked real usage: `#⃣` and `🗚️` each appear in exactly **one** directory (not a
convention like `svg`'s 690:2). `🗚️fonts` (7 files, 1 referencing source file) was safely renamed
by hand to `🔤️fonts` (reusing the already-registered `base64`/`string` emoji) with its one
`include_bytes!` call site updated; `fonts` kind registered. `#⃣hash` is a shared Rust crate
depended on from **16 Cargo.toml files** repo-wide — a hand rename risks a half-updated dependency
graph outside the transactional apply pipeline, so it was **not** renamed; registered `hash` (🔢️,
reusing `numeric`/`bytes`'s emoji) so it resolves the moment the apply pipeline (or a dedicated
slice) renames the directory. Also found and fixed the same-shape cascade: the `fonts` directory
itself, once resolvable, exposed 3 unregistered font-family child directories — registered
`font-subject` (🔤️, `parentKindIds: ["fonts"]`).

**Config-leaf `fixedFilenameContracts` — landed.** Generalized the *same* hardcoded
`packageSourceDispositions.validator` 3-value whitelist in **both** places that independently
enforce it: `🔍️discovery/🟦️component.ts` (already touched once, re-applied) and, on your
authorization, `🧹️normalization/🟦️.ts::parseTaxonomy` (type union at line ~786 + runtime check at
line ~1575) — the latter is the one that was actually blocking `clean taxonomy plan` at load time.
Both now use the same `TOOL_CONFIG_VALIDATORS: Record<token, ownerContractId>` map, pinning each new
token (`tool-config-vitest`, `tool-config-tailwind`, `tool-config-postcss`, `tool-config-eslint`,
`tool-config-dependency-cruiser`) to exactly one contract id, mirroring the pre-existing
`vitest-configuration`→`vitest-config-entry` precedent. Registered the 5 `fixedFilenameContracts` +
`packageSourceDispositions` entries. No other lines in either file touched.

**New mechanical fix, same class as svg/model-3d:** `.schema.json` had no registered file kind at
all (1820 files repo-wide), so every "`<name>.schema.json`" kept ".schema" glued to its stem. Added
a dedicated `json-schema` file kind (🔣️, exactly one extension chain `.schema.json`, own
`fileKindResolutionRules` entry) — a single shared `json` kind can't own two extension chains
(`canonicalFilenameForKind` requires exactly one), so it's a sibling kind, not an appended chain.

**`contract` (21 rows) widened onto `schema`,** not a new kind: `🧬️contract.json` is structurally
identical to `🧬️schema.json` (same emoji, same role). `schema.slugPattern` →
`^(schema|mutations|contract)$`.

**Long tail confirmed, does not converge:** of 449 `semantic-stem-unresolved`, **395 are distinct
(emoji,stem) pairs** — the vast majority count 1-2, almost entirely one-off Rust/TS module names
inside `📦️packages/🦀️rust` or `.../🟦️typescript` (`surface.rs`, `resources.rs`, `layout.rs`,
`pipelines.rs`, `scene_target.rs`, `frame_buffers.rs`, `dispatch.rs`, …) — genuine per-crate
implementation names, not cross-cutting domain vocabulary, so correctly left unregistered rather
than forced into fake "IMPLEMENTATION-NEUTRAL" kinds. This is the expected shape of the residue:
census closure, not a stall.

**Not landed, documented:** the 11 multi-emoji words and the `asset-subject`/`test-fixture-asset`/
`test-fixture-member` 🖼️/🧪️ overlap (6→9 rows once the `.schema.json` fix exposed the `.json`
siblings' twin) — same ambiguity-fallback mechanism as `test-case`, traced but not resolved this
round; needs the same `parentKindIds`-scoping treatment on a future pass.

**Tests**: 6 new `package-language-kind-handoff` cases (fonts, hash, input-scoped, contract synonym,
font-subject two-step). One authoring mistake caught and fixed mid-flight: a no-leading-emoji
directory name (`anta`) exercises an inference path the test's own independent AJV oracle doesn't
model (it only validates when the input already carries the emoji, matching every existing
precedent case) — rewrote the case to supply the emoji up front, matching how every other case in
this harness is written. Real run: `6 pass, 0 fail, 1045 expect() calls`.

**Before → after → after-round-2** (`--scope "🧰️framework/🔨️modules"`, same baseline
`bb06c41f73f0122fbed315b7487428b976f99921`, final plan at `🗑️temp/🔣️vocab-plan-3.json`):

| metric | orig. before | after r1 | after r2 |
|---|---:|---:|---:|
| `moves` | 953 | 2082 | **2117** |
| `unresolved` (total) | 2631 | 1477 | **1459** |
| `semantic-stem-unresolved` | 1495 | 449 | **411** |
| `semantic-stem-ambiguous` | 167 | 84 | **87** |
| `directory-kind-unresolved` | 111 | 31 | **33** |

`semantic-stem-ambiguous`/`directory-kind-unresolved` ticked up slightly this round, not from
regressions: fixing `input`/`hash`/`fonts` unblocked traversal into previously-masked sibling
subtrees (`🧬️contract/🧵️retained`, the 3 font-family dirs) that were never reached before, plus the
`.schema.json` fix exposed 3 pre-existing `.json`-sibling ambiguities under their correct bare stem
for the first time — diffed both plans path-by-path to confirm every "new" row is one of these two
causes, not a fresh defect. Net unresolved and `moves` both moved the right direction.

Files this round: `🧰️framework/🔨️modules/📚️compiler/🌍️world/🔤️fonts/` (renamed from `🗚️fonts/`),
its `🦀️component.rs` (5 path references updated), `🔣️taxonomy.json`, `🔍️discovery/🟦️component.ts`,
`🧹️normalization/🟦️.ts`, `🧪️package-language-kind-handoff/🔣️.json`.
