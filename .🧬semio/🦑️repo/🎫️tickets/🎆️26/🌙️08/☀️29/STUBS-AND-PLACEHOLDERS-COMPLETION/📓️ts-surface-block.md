# 🧱️ block plugin — TypeScript surface repair

Scope: `✏️s/🔌️plugins/🧱️block` only. Started at 336 `tsc --strict` errors (208×TS2307, 128×TS2374, 0×TS2304). Finished at **0 errors**.

Repro command (unchanged):
```
bunx tsc --noEmit --strict --target ESNext --module ESNext \
  --moduleResolution bundler --esModuleInterop --skipLibCheck --allowImportingTsExtensions \
  "✏️s/🔌️plugins/🧱️block/📦️packages/🟦️typescript/📦️index.ts"
```

## TS2374 (duplicate index signature) — 128 → 0

Six files — `◻2d`/`🧊️3d`/`🖐️5d` × {`🧬️schema/🟦️component.ts`, `🧬️schema/📸️snapshot/🟦️component.ts`} — each contained the *whole* interface block duplicated verbatim, and every nested type was a `{ [key: string]: unknown }` stub instead of a real mirror. Rewrote all six from the Rust source of truth:

- `✏️s/🔌️plugins/🧱️block/🦀️component.rs` (plugin-root, shared across all 3 dimensions) → new `✏️s/🔌️plugins/🧱️block/🟦️component.ts` carrying `BlockKindIdentity`, `BlockAttribute`, `BlockAuthor`, `BlockCompatibilityRule`, `BlockRepresentation`, `BlockCamera2d`, `BlockCamera3d`, `BlockMeta` (all `#[serde(rename_all = "camelCase")]`, verified field-for-field).
- Each artifact's own `🗿️artifacts/<dim>/🦀️component.rs` → new `🗿️artifacts/<dim>/🟦️component.ts` carrying the dimension-specific nested types (`Block2dPresentation`/`Block2dHandleKind`/`Block2dHandleTemplate`; `Block3dVortexKind`/`Block3dVortexKindExtra`/`Block3dVortexTemplate`/`Block3dWindowView`/`Block3dBrushPreview`; `Block5dPart2d`/`Block5dPart3d`/`Block5dGripKind`/`Block5dGripTemplate`).
- The 6 schema/snapshot files now `import type` from those two new files instead of duplicating.

Two bugs the stub had baked in, caught by reading the Rust and cross-checking fixtures:
- `🧊️3d` schema/snapshot were missing the `catalog: store::ArtifactChild<SemioKitSnapshot>` field entirely, and had the wrong field name/type (`vortexKinds: Block3dVortexKind[]` instead of the real `vortexKindExtra: Block3dVortexKindExtra[]`) — `🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs:15-27`. Fixed using the `ArtifactChildHandle { childId; target }` convention already established in `🏭️process/🗿️artifacts/🧊️process3d` and `🪵️sourcing/🗂️curate`'s schema files.
- `🖐️5d` snapshot's `part_2d`/`part_3d` fields carry `#[serde(rename = "2d"/"3d")]` overrides (only on the *persisted* `Block5dSnapshot`, not the live `Block5dArtifact`, which stays `part2d`/`part3d`) — `🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs:17-24`. Verified against the real fixture `🗿️artifacts/🖐️5d/…/🧬️mutations/🙅remove-author/🧪️tests/uncredits-ada/📸️snapshot/⬅️before/🔣️component.json`, which does serialize `"2d"`/`"3d"` — used quoted keys in the snapshot interface only.

## TS2307 (missing module) — 208 → 0

Each of the three `…/🧬️schema/🧬️mutations/🟦️component.ts` dispatch-union facades imported every leaf mutation from `./<name>/🦠️mutation/🟦️component` — a path segment (`🦠️mutation/`) that doesn't exist anywhere on disk. Checked the established convention (`✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🟦️component.ts`): siblings are imported directly as `./<name>/🟦️component.ts`, no extra segment. Stripped `/🦠️mutation` from all 208 references (52 in `◻2d`, 78 in `🧊️3d`, 78 in `🖐️5d`).

That import then needed a real target: **104 mutation leaf directories had no `🟦️component.ts` at all** (26 in `◻2d`, 38 in `🧊️3d`, 40 in `🖐️5d`) — only `🦀️.rs` + fixture/diff/inverse machinery existed. Generated all 104 from their `🦀️.rs` structs (script in scratchpad, not committed): every payload struct is `#[serde(rename_all = "camelCase")]` (verified — none of the 104 deviate), fields are plain `String`/`Option<String>`/`f64`/`Option<f64>`/`[f64; N]`/`Option<[f64; N]>`, or a `#[dsl(block)]`-annotated reference to one of the shared/dimension types above (`rule: BlockCompatibilityRule`, `attribute: BlockAttribute`, `author: BlockAuthor`, `handle_kind: Block2dHandleKind`, `vortex_kind: Block3dVortexKind`, `grip_kind: Block5dGripKind`, `representation: BlockRepresentation`, etc.) — those import from the two new root files above. Spot-checked several outputs by hand (`RenameNodeKind`, `AddCompatibilityRule`, `MoveVortex`, `UpdatePart2d`) against their `🦀️.rs` siblings; all field names/optionality/tuple arities match.

## TS2304 — none present in block's own error set (repo-wide count is from other plugins).

## Verification

- `bunx tsc` (exact command above) → **0 errors**, run three times across the session (after each fix wave) to confirm no regression.
- `git status --porcelain` scoped to files actually touched this session: 4 new root/artifact mirror files, 9 edited schema/mutations-facade files, 104 new mutation leaf `🟦️component.ts` files = 117 files. (`git status` on the whole `🧱️block` tree shows ~199 modified paths — the remainder, e.g. `📦️index.ts`, `💾️binary/*`, `📝️text/*`, `*.proto`/`*.graphql`/`*.json`/`*.g4` siblings, are other concurrent sessions' work, not touched here.)

## Nothing left unfinished for this scope

All 336 original errors in `🧱️block` are resolved; no error required a type owned by another plugin.

## Wire-format correction (post-review)

A reviewer caught a bug `tsc` cannot see: the three dispatch-union facades' `mutation` discriminant string literals were **kebab-case** (`"rename-node-kind"`), but the Rust enums are tagged `#[serde(tag = "mutation", rename_all = "camelCase")]`, so the real wire form is camelCase (`"renameNodeKind"`). String literal types type-check regardless of casing, so this was invisible to the typecheck and needed fixture cross-referencing, not just `tsc`.

**Checked each enum's own attribute — none deviate:**
- `◻2d/…/🧬️mutations/🦀️component.rs:29` — `#[serde(tag = "mutation", rename_all = "camelCase")]` on `Block2dMutation`.
- `🧊️3d/…/🧬️mutations/🦀️component.rs:29` — same, on `Block3dMutation`.
- `🖐️5d/…/🧬️mutations/🦀️component.rs:30` — same, on `Block5dMutation`.

All three are camelCase; none use kebab-case or omit `rename_all`. Converted every discriminant in all three facades by lowercasing the first character of the already-correct Rust/TS variant name (`RenameNodeKind` → `renameNodeKind`) — every one of the 103 discriminants across the three files was wrong (all kebab-case) and is now fixed:
- `◻2d`: 26 discriminants corrected.
- `🧊️3d`: 37 discriminants corrected.
- `🖐️5d`: 40 discriminants corrected.

**Fixture cross-check:** every one of the 104 mutation leaf directories has a committed `…/🧪️tests/*/🦠️mutation/🔣️component.json` fixture (104/104 — full coverage, not partial). Wrote a script comparing each fixture's `"mutation"` tag and top-level field keys against the corresponding facade discriminant and generated leaf interface. **104/104 discriminants confirmed against a fixture, 0 derived from the attribute alone, 0 mismatches.** (`resizeVortex`/`newRadius`, `changeGripKindDefaultRopeKind`/`newDefaultRopeKind`, `addAttribute`/`attribute.key`+`value`, etc. — all verified.)

**Leaf and nested payload casing** — re-verified against fixtures rather than trusting the `camelCase` attribute alone, as asked:
- Leaf scalar fields: `resizeVortex{id,newRadius}`, `changeGripKindDefaultRopeKind{id,newDefaultRopeKind}` — confirmed camelCase.
- Nested `#[dsl(block)]` payload objects (not covered by the leaf-only automated check, so checked by hand against their own fixtures): `Block2dHandleKind`, `Block2dHandleTemplate`, `Block3dVortexKind`, `Block3dVortexTemplate`, `Block5dGripKind`, `Block5dGripTemplate`, `BlockRepresentation` — all match field-for-field via `🌱️create-handle-kind`, `🌿️create-handle`, `🌱create-vortex-kind`, `🌀create-vortex`, `🌱create-grip-kind`, `🌿create-grip`, `🧱create-representation` fixtures.
- `BlockCompatibilityRule`, `BlockAttribute`, `BlockAuthor`, `BlockCamera2d`, `BlockCamera3d`, `BlockMeta`, `BlockKindIdentity`, `Block2dPresentation`, `Block5dPart2d`/`Block5dPart3d` — all confirmed against the full `🖐️5d` snapshot fixture (`…/🙅remove-author/🧪️tests/uncredits-ada/📸️snapshot/⬅️before/🔣️component.json`) and a `◻2d` snapshot fixture.
- `Block3dWindowView`, `Block3dBrushPreview` — **no committed fixture touches these** (config/UI-only fields, never exercised by a mutation test). Left as derived from the Rust struct's own `#[serde(rename_all = "camelCase")]` attribute (`🧊️3d/🦀️component.rs`) — flagging this per the ask, since it's attribute-only, not fixture-confirmed.

**Another inferred-not-verified spot found and fixed while re-checking:** my own `ArtifactChildHandle` (used for the `🧊️3d` schema/snapshot `catalog` field) copied the shape already present in `🏭️process/🧊️process3d` and `🪵️sourcing/🗂️curate`'s existing (unrelated, not-mine) schema files verbatim — `{ childId: string; target: string; }`. Checking it against the real fixture (`🧊️3d/…/🙅remove-author/🧪️tests/uncredits-ada/📸️snapshot/⬅️before/🔣️component.json`'s `catalog` field, and cross-checked against the same shape in `curate`'s and `process3d`'s own snapshot fixtures) showed `target` is actually a nested object, not a string:
```json
"catalog": { "childId": "...", "target": { "artifactId": "...", "dialect": { "artifactKind": "...", "standard": "...", "subset": "..." } } }
```
Fixed in the two `🧊️3d` files I own (`🧬️schema/🟦️component.ts`, `🧬️schema/📸️snapshot/🟦️component.ts`) to `ArtifactChildHandle { childId: string; target: ArtifactChildTarget }` with `ArtifactChildTarget { artifactId: string; dialect: ArtifactDialect }` / `ArtifactDialect { artifactKind: string; standard: string; subset: string }` (naming follows the existing `💠️lowpoly` plugin's `ArtifactDialect`/`ArtifactRef` convention). **Not** fixed in `curate`/`process3d` — those files are outside this ticket's `🧱️block` scope and weren't touched by me; flagging here since it's the same bug pattern and someone should check those too.

## Re-verification

Ran the exact `tsc --noEmit --strict …` command from the brief again after all wire-format fixes:
```
$ bunx tsc --noEmit --strict --target ESNext --module ESNext --moduleResolution bundler --esModuleInterop --skipLibCheck --allowImportingTsExtensions "✏️s/🔌️plugins/🧱️block/📦️packages/🟦️typescript/📦️index.ts"
(no output, exit 0)
```
0 errors, confirmed by `wc -l` on the captured output (`0`). Ran three separate times across this correction pass (after the discriminant fix, after the `ArtifactChildHandle` fix) with identical 0-error results each time.
