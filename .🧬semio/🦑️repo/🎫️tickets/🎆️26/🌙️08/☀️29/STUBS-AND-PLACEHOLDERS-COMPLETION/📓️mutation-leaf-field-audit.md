# Mutation Leaf Field-Mirror Audit

Per-leaf `🧬️mutations/<verb>/…/🟦️component.ts` mirrors (the `🦠️mutation`/`🔺️diff`/`↩️inverse` triad
and the flat single-file leaves), audited against their sibling Rust payload struct's own
`#[serde(rename_all=…)]` and, wherever one exists, the committed `🧪️tests/**/…/🔣️component.json`
fixture. `🧱️block` and `🏛️architect` excluded per instructions (already fixture-verified
104/104 and 266/266 by other agents).

## Scope found on disk

`find ✏️s/🔌️plugins -path "*/🧬️mutations/*/🟦️component.ts"`, minus block/architect: **884 files**
across **31 plugins**, breaking down by immediate parent directory:

| Kind | Count | Field-mirror relevant? |
|---|---|---|
| flat `<verb>/🟦️component.ts` | 526 | yes |
| `<verb>/🦠️mutation/🟦️component.ts` | 115 | yes |
| `<verb>/🔺️diff/🟦️component.ts` | 94 | yes |
| `<verb>/↩️inverse/🟦️component.ts` | 94 | yes |
| `<verb>/🧪️contract/🟦️component.ts` | 54 | no — law-probe helper functions, no Rust sibling, nothing to mirror |
| `<verb>/🔒️private/🟦️component.ts` | 1 | no — command-local validation helpers, no Rust sibling |

829 files carry an actual payload/fragment mirror. Of those, 204 are still bare stubs (`export {};`
or an `export declare function …: unknown` ambient signature) — genuinely unimplemented, nothing to
mis-mirror. **625 files across every one of the 31 plugins were mechanically compared** field-by-field
against their Rust struct's real serialized name (casing derived from `rename_all`, or none) using a
brace-aware Python parser (handles both single-line and multi-line TS interfaces/Rust structs). Every
flagged discrepancy was individually re-read against the Rust source and, wherever one exists, the
fixture. Every plugin bucket that came back **clean** from the mechanical pass was also spot-checked
by hand (1–3 leaves per plugin, picked from different verbs) to rule out a parser blind spot before
trusting the "0 mismatches" result — see "False positives ruled out" below for the two systematic
blind spots that surfaced this way and were then corrected in the tooling.

## Per-plugin audit table

"Fixed" only counted where the file content changed. "Fixture-confirmed" = at least one committed
`.json` fixture was read and matches the corrected shape.

| Plugin | Leaves mechanically compared | Fixture-confirmed | Mismatches found | Fixed |
|---|---|---|---|---|
| `📕️norm` (iso16757) | 63 | 12 direct + rest via Rust `diff()` body (unambiguous) | 50 | **50** |
| `🔱️trinity` (jack) | 15 | 2 | 2 | **2** |
| `🗄️stdio` (pdf, gltf, + 21 other artifacts) | 261 | 1 (jack pattern, N/A here) — pdf/gltf had no fixture, confirmed by Rust struct alone | 2 | **2** |
| `🌀️procedural` (2d/3d) | 30 | spot-checked (fixture on `change-schema`) | 0 (see note) | 0 |
| `🌍️gis` | 2 | — | 0 | 0 |
| `🌿️vcs` | 6 | — | 0 | 0 |
| `🎪️demonstrator` | 1 | — | 0 | 0 |
| `🎬️sequence` | 8 | — | 0 | 0 |
| `📋️forms` | 0 (all 9 remaining are stubs) | — | 0 | 0 |
| `📜️imperative` | 4 | — | 0 | 0 |
| `🔋️energy` | 1 | — | 0 | 0 |
| `🖍️draw` | 6 | — | 0 | 0 |
| `🧩️puzzle` | 0 (all 35 remaining are stubs) | — | 0 | 0 |
| `🪐️space` | 5 | — | 0 | 0 |
| `🪵️sourcing` | 3 | — | 0 | 0 |
| `✒️writer` | 4 | — | 0 | 0 |
| **Totals** | **409 shown above** (+216 more `stdio` artifacts not itemized, all clean) | | **54** | **54** |

The `stdio` "261 compared" figure spans 22 other artifacts beyond pdf/gltf (svg, xml, json, dwg,
pptx, xlsx, ply, jpg, png, bmp, tiff, dxf, md, csv, bcf, semio subsets, …) — all came back clean on
the mechanical pass; 6 were spot-read by hand (svg's `set-doctype`/`set-view-box`/`insert-element`
etc., all correct) in the course of debugging the parser.

## Defects found and fixed — `📕️norm/📓️iso16757` (50 files)

This artifact has **no `rename_all`** on any of its leaf mutation-payload structs, so every field
must stay snake_case — the exact bug class named in the brief. It had drifted to camelCase almost
everywhere.

**16 `🦠️mutation` payload fixes** (field renamed to match the Rust struct verbatim; 4 of these were
completely empty `{}` interfaces missing their Rust-required `id: string` field entirely):

| Leaf | Bug |
|---|---|
| `🍂replace-part-number-rule` | `newRule` → `new_rule` |
| `🛏️rename-product` | `newName` → `new_name` |
| `🌾create-property-definition` | `propertyDefinition` → `property_definition` |
| `🌼change-selection-series` | `newSeriesId` → `new_series_id` |
| `🍀create-product-group` | `productGroup` → `product_group` |
| `🌷update-script-limits` | `newMaxSteps`/`newMaxRecursion`/`newTimeoutMs` → `new_max_steps`/`new_max_recursion`/`new_timeout_ms` |
| `🚿rename-product-group` | `newName` → `new_name` |
| `🌲rename-catalogue` | `newName` → `new_name` |
| `🌴change-selection-class` | `newClassId` → `new_class_id` |
| `🌱change-part-number-input` | `newValue` → `new_value` |
| `🌳rename-manufacturer` | `newName` → `new_name` |
| `🍃change-exchange-process` | `newExchangeProcess` → `new_exchange_process` (the ticket's original example) |
| `🌸delete-product`, `🌻delete-subject`, `🌹delete-product-group`, `🌺delete-property-definition` | empty `{}` → added missing `id: string;` |

Fixture-confirmed for `🍃change-exchange-process`, `🌲rename-catalogue`, `🌷update-script-limits`,
`🍂replace-part-number-rule`, `🌸delete-product`. The rest share the identical struct shape
(`{ id: String }` or `{ new_<x>: T }`, no `rename_all`), confirmed directly against each leaf's own
`🦀️component.rs` struct definition — unambiguous, not inferred.

**21 `🔺️diff` fragment rewrites.** This was a bigger, previously-unreported bug: every leaf's diff
fragment described a *fabricated flat shape* (`{ name?: string }`, `{ classid?: string }`,
`{ maxsteps?, maxrecursion?, timeoutms? }`, `{ rule?: string }`, …) that doesn't correspond to
anything the Rust `diff()` function actually constructs. Reading each leaf's `🔺️diff/🦀️component.rs`
shows every one of them builds a **top-level field of the parent `Iso16757Diff` struct**
(`rename_all = "camelCase"`, confirmed at `…/🧬️schema/🔺️diff/🦀️component.rs:13`) — e.g.
`rename-catalogue` writes `Iso16757Diff{ catalogue: Some(catalogue), .. }`, not a `name` field
anywhere. Fixed all 21 to the correct top-level field name (`catalogue`, `selection`, `dictionary`,
`scriptLimits`, `partNumberRule`, `partNumberInputs`, `exchangeProcess`), fixture-confirmed for 5 of
them directly (`🌲rename-catalogue`, `🌷update-script-limits`, `🌴change-selection-class`,
`🍁create-product`, `🌵create-subject`, `🛁add-selection-constraint` — fixture JSON literally shows
`"catalogue": {…full object…}` / `"scriptLimits": {…}` / `"selection": {…}` at the top level, not the
old fabricated flat fields), the remaining leaves confirmed by the same unambiguous `diff()` Rust
source read (which field of `Iso16757Diff` it populates is stated directly in the function body, not
inferred).

**13 `↩️inverse` missing-import fixes.** `export type XInverse = X;` with **no `import` statement**
in the file at all — `X` was never brought into scope, an outright undefined-name error. Confirmed by
`grep -c "^import"` returning 0 on 13 of the 21 inverse files; the other 8 (delete-*, connect/remove
pairs) already had their imports. Fixed by adding `import type { X } from "../🦠️mutation/🟦️component.ts";`.
Repo-wide re-scan of all 94 `↩️inverse` leaves (all plugins) for this exact pattern after the fix:
**0 remaining** — isolated to iso16757, not systemic.

Also specifically checked (per the brief's explicit ask): **no** `Delete*Inverse` self-aliasing bug
remains in iso16757 — all four delete leaves' inverses already correctly alias their `Create*`
counterpart (`DeleteProductInverse = CreateProduct`, etc.), consistent with a prior session's claim
in `📓️status.md` that this specific defect class had already been fixed. That part of the prior
claim held up; the field-casing and missing-import bugs above did not (still present on disk at
audit time, confirmed by direct read before touching anything).

## Defects found and fixed — other plugins (4 files)

| File | Bug | Confirmed by |
|---|---|---|
| `🔱️trinity/🔌️jack…/🔧️change-data-property/🟦️component.ts` | `newValue` → `new_value` (no `rename_all` on `ChangeDataProperty`) | fixture: `{"key":"label","new_value":"Capsule A"}` |
| `🔱️trinity/🔌️jack…/✏️rename-node/🟦️component.ts` | `newName` → `new_name` (no `rename_all` on `RenameNode`) | fixture: `{"id":"capsule-a","new_name":"Capsule A"}` |
| `🗄️stdio/🧊️gltf…/🌱️📐️create-accessor/🟦️component.ts` | payload field literally named `type` in TS; Rust struct field is `kind: GltfAccessorType` (`type` is used only by the *unrelated* `GltfAccessor` snapshot type, which the same file's `apply()` function separately constructs) | Rust struct read (`pub kind: GltfAccessorType`) — no fixture exists for this leaf |
| `🗄️stdio/📄️pdf…/✂️set-page-crop-box/🟦️component.ts` | `cropBox: unknown[] \| null` → `cropBox: [number, number, number, number] \| null` (type-precision only, matches the `set-trim-box` sibling's established tuple convention) | Rust struct type (`Option<[f64; 4]>`) + sibling convention; not a wire-format bug, no field renamed |

`create-accessor` fix required also updating the one internal read site
(`payload.type` → `payload.kind`) inside the same file's `applyGltfCreateAccessor`, while correctly
*keeping* the literal `type:` key in the constructed `GltfAccessor` object (that object's own field
really is `type`, per the glTF spec mirror at `…/📸️snapshot/🟦️component.ts:71`). Verified no other
file references `GltfCreateAccessorPayload.type`.

## False positives ruled out (do not "fix" these)

1. **`Option<T>` without `#[serde(skip_serializing_if=…)]` does not make the JSON key optional** —
   serde's default `Option<T>` serialization always emits the key, valued `null` or `T`. My first
   parser pass flagged 7 leaves (6 in `stdio/🧊️gltf`, 1 in `stdio/📄️pdf`) as "TS field should be
   optional" purely because the Rust type was `Option<…>`. All 7 have no `skip_serializing_if`, so
   the existing TS (`field: T | null`, no `?`) is the *more* correct representation — a required key
   that may hold `null` — and marking it `field?:` would be a regression, not a fix. Left untouched
   after re-derivation (one, `cropBox`, briefly got a spurious `?` added and then reverted before
   commit-equivalent state — never left in the tree).
2. **`🌀️procedural2d`/`🧊️procedural3d`'s `🔺️diff` and `↩️inverse` leaves intentionally mirror the
   internal sparse delta** (`WidgetsDiff{removed,set}` etc.) rather than the literal wire-level
   `Procedural2dDiff.fixture: FlowFixture` full-replace the Rust `diff()` ultimately produces via
   `diff_fixture_from_helpers`. This is a **documented, deliberate design decision** by the agent who
   filled these 25+43 files (`📓️ts-mirrors-procedural2d.md`, "Design decisions worth flagging"),
   explicitly modeled on `🔱️trinity/🔌️jack`'s own pre-existing hand-written convention (studied as
   the reference pattern). Confirmed via fixture that the *real* wire diff is
   `{fixture: {schema, camera, widgets: Widget[], synapses: […], layout: {…}}, generation: null, …}`
   — structurally different from what the leaf's `diff()` function returns — but this is the
   intended abstraction level, not an unrecognized mirror bug, so left untouched. Flagging it here
   only so a future reader doesn't rediscover the same "mismatch" and assume it's unaudited.
3. **17 "no matching Rust struct" TS interfaces** (`SpaceArtifactRow`, `FormGeneration`, `PathRef`,
   `Step`, `SynapseSpec`, `WidgetLayout`, `CameraJson`, `JackPort`, `JackNode`, `JackEdge`,
   `LayoutPoint`) are auxiliary/nested type declarations duplicated locally inside the leaf file
   (matching a repo-wide convention where every schema surface self-contains its type aliases rather
   than cross-importing — documented explicitly in `📓️ts-mirrors-procedural2d.md`). The actual
   mutation-payload interface in the same file (`CreateArtifact`, `CreateGeneration`, `EditStepParams`,
   `CreateStep`, `ConnectSynapse`, `MoveWidget`, `UpdateCamera`, `CreateNode`, `CreateEdge`,
   `ChangeRuleLayoutPoint`) matched its Rust struct exactly in every case checked.

## Unfinished / honest boundaries

- **204 stub leaves** (`export {};` or `export declare function …: unknown`) across `🌀️procedural`
  (36), `🧩️puzzle` (35), `🔱️trinity` (30), `🗄️stdio` (66 — confirmed these are set-snapshot leaves
  mid-deletion by a **concurrent session**, see `git status` noise and `📓️status.md`'s own note about
  375 modified stdio `.rs` files; not touched), `🖍️draw` (12), `📋️forms` (9), `🪵️sourcing` (6),
  `🌍️gis` (4), `🔋️energy`/`🎪️demonstrator`/`🪐️space` (2 each) — genuinely unimplemented, nothing to
  mis-mirror. Not filled in (out of this audit's scope — auditing existing mirrors, not authoring new
  ones).
- **`🧪️contract` (54) and `🔒️private` (1) leaves** are pure TypeScript logic (law-probes, local
  validation helpers) with no Rust sibling struct — reviewed, confirmed not applicable to the
  field-casing bug class, not touched.
- I did not open a fixture for every one of the ~570 mechanically-clean leaves individually — that
  would mean reading several thousand JSON files by hand. I trust the mechanical comparison (which
  reads the real Rust struct's own `rename_all` per file, not a repo-wide assumption) plus the
  targeted spot-checks per plugin bucket described above. If a residual risk needs closing further,
  the next pass should fixture-check the `🗄️stdio` non-gltf/pdf artifacts leaf-by-leaf (261 files,
  only 6 spot-read here).

## Verification

`bunx tsc --noEmit --strict --target ESNext --module ESNext --moduleResolution bundler --esModuleInterop --skipLibCheck --allowImportingTsExtensions` against each touched plugin's package index, run **after** all fixes:

```
norm    : exit 0 (0 errors)
trinity : exit 0 (0 errors)
stdio   : exit 0 (0 errors)
```

`git status --porcelain` against the exact 54-file list this session edited (not the plugin
directories, which show ~130+ unrelated modified files from concurrent sessions — e.g. stdio's
in-flight set-snapshot deletion, gltf's grammar/proto/graphql regeneration): **54/54** — exact scope,
nothing extra touched, nothing from the list missing.

## Files touched (54)

- `📕️norm/📓️iso16757…/🧬️mutations/`: 16× `🦠️mutation/🟦️component.ts`, 21× `🔺️diff/🟦️component.ts`,
  13× `↩️inverse/🟦️component.ts` (verb list in the tables above)
- `🔱️trinity/🔌️jack…/🔧️change-data-property/🟦️component.ts`
- `🔱️trinity/🔌️jack…/✏️rename-node/🟦️component.ts`
- `🗄️stdio/📄️pdf…/✂️set-page-crop-box/🟦️component.ts`
- `🗄️stdio/🧊️gltf…/🌱️📐️create-accessor/🟦️component.ts`
