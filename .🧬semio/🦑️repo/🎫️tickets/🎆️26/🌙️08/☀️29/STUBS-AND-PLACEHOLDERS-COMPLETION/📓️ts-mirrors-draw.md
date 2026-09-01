# 🖍️draw plugin — TS mirror leaves filled

Scope: the 18-file `leaves-draw.txt` list (🖍️draw plugin, `🧬️mutations` region), each a
`export {};` facade stub. All 18 are now real TypeScript mirrors of their Rust counterpart,
following the `interface` (payload) / `function diff()` / `function inverse()` convention
established by the jack reference triads (`✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/…`).

Six verbs, three leaves each (`🦠️mutation`, `🔺️diff`, `↩️inverse`):

## set-layer-visible
Rust source: `…/🧬️mutations/👁️set-layer-visible/{🦠️mutation,🔺️diff,↩️inverse}/🦀️component.rs`
- `🦠️mutation`: `SetLayerVisible { layerId: string; visible: boolean }` — mirrors the `SetLayerVisible` struct (mutation.rs:12-15).
- `🔺️diff`: builds `{ layers: { patched: [{ id, patch: { visible } }] } }` — mirrors `diff_set_layer_visible` via `DrawLayerPatch.visible` (diff/component.rs schema struct + the leaf's `diff()`).
- `↩️inverse`: takes `baseVisible: boolean | undefined` (BASE-state lookup done by caller) and returns `[]` or `[{ layerId, visible: baseVisible }]` — mirrors `inverse()` (inverse/component.rs:8-13).

## set-layer-locked
Rust source: `…/🧬️mutations/🔒️set-layer-locked/{🦠️mutation,🔺️diff,↩️inverse}/🦀️component.rs`
Same shape as set-layer-visible with `locked: boolean` in place of `visible`.

## set-layer-opacity
Rust source: `…/🧬️mutations/🌫️set-layer-opacity/{🦠️mutation,🔺️diff,↩️inverse}/🦀️component.rs`
Same shape with `opacity: number`. (Rust `diff` also rejects non-finite opacity and no-ops on an
unchanged value — both are outcome-policy branches the leaf's own `diff()` owns; the TS mirror is
the sparse-patch *builder* only, matching how the jack reference mirrors never re-derive the
target-missing/no-op branching either.)

## set-layer-blend-mode
Rust source: `…/🧬️mutations/🖌️set-layer-blend-mode/{🦠️mutation,🔺️diff,↩️inverse}/🦀️component.rs`
Same shape with `blendMode: string` (camelCase of Rust `blend_mode`).

## reorder-layer
Rust source: `…/🧬️mutations/🔃reorder-layer/{🦠️mutation,🔺️diff,↩️inverse}/🦀️component.rs`
- `🦠️mutation`: `ReorderLayer { layerId: string; parentId?: string; index: number }` — mirrors the `ReorderLayer` struct (mutation.rs:12-17), FINAL-state address per its own docstring.
- `🔺️diff`: `diff(payload, layer: DrawLayerNode)` builds `{ layers: { removed: [layerId], added: [{ parentId?, index, layer }] } }` — mirrors `diff_reorder_layer` (draw's `🔺️diff/🦀️component.rs` builder) and the leaf's own remove+insert (diff/component.rs:8-19). `layer` (the source subtree) is a caller-resolved parameter, same convention as jack's `delete-node` inverse taking `baseNode`.
- `↩️inverse`: `inverse(payload, baseLocation)` returns the OLD `(parentId, index)` as a `ReorderLayer[]` — mirrors inverse/component.rs:8-13; `baseLocation` is the caller-resolved BASE address.
- Imports `DrawLayerNode` from the schema root (`../../../🟦️component.ts`, i.e. `✳️any/🧬️schema/🟦️component.ts`), which was already filled (`{ kind: string; [key: string]: unknown }`).

## duplicate-layer
Rust source: `…/🧬️mutations/🧬️duplicate-layer/{🦠️mutation,🔺️diff,↩️inverse}/🦀️component.rs`
- `🦠️mutation`: `DuplicateLayer { layerId: string }` — mirrors the struct (mutation.rs:14-16); the duplicate's id is content-addressed (Rust `clone_draw_layer_node` hash), so it is never carried in the payload.
- `🔺️diff`: `diff(payload, duplicate, sourceLocation, rootLayerCount)` reproduces the Rust branch — `sourceLocation` found ⇒ insert at `(parentId, index + 1)`; not found ⇒ append at `(undefined, rootLayerCount)` — mirroring diff/component.rs:8-18's `match find_draw_layer_location(...)`. The content-addressed hash itself is **not** mirrored — `duplicate` (the already-cloned `DrawLayerNode`) is a caller-resolved parameter, same as `sourceLocation`.
- `↩️inverse`: `inverse(payload, duplicateLayerId)` returns `[]` or `[{ layerId: duplicateLayerId }]` (the `delete-layer` payload shape) — mirrors inverse/component.rs:8-14. `duplicateLayerId` is caller-resolved (same hash-avoidance reasoning as the diff leaf).

## Why some values are caller-resolved parameters
Three Rust diff/inverse functions rely on `crate::artifacts::draw::schema::{find_draw_layer,
find_draw_layer_location, clone_draw_layer_node}` — tree lookups and a content-addressed hash.
Mirroring those in TypeScript here would require porting the recursive layer tree and the hash
function too, well beyond a single leaf's structural-transform scope. Following the established
jack convention (`🗑️delete-node/↩️inverse` takes `baseNode: JackNode | undefined` as a
caller-supplied parameter rather than looking it up itself), the draw mirrors accept the
resolved lookup/hash result as a parameter and mirror only the pure structural transform —
field-for-field the same branching and shape as the Rust `diff`/`inverse` bodies.

## Verification
No `project.json` target or workspace entry covers the draw TS package's mutation leaves
directly (`✏️s/🔌️plugins/🖍️draw/📦️packages/🟦️typescript` is not in the root `package.json`
workspaces list, and its `📋️project.json` only has a `test` target). Typechecked directly with a
scratch tsconfig extending the repo root `tsconfig.json` compiler options, scoped to the whole
`✳️any/🧬️schema/**/*.ts` tree (so every existing sibling file is checked too, not just the 18 new
ones):

```
bunx tsc --noEmit -p /private/tmp/claude-501/-Users-ueli-Documents-semio/c17a0f0b-94f9-4f2f-bbd0-8ff82df33749/scratchpad/tsconfig.draw-mutations.json
```

Output: **no errors** (empty output, exit clean).

Confirmed with `--listFiles` that tsc actually picked up all 18 target files plus the
already-filled `🧬️mutations/🟦️component.ts` union root (19 files matched under `🧬️mutations/`).

Confirmed zero `export {};`-only stub leaves remain in the 18-file list (grepped every path in
`leaves-draw.txt` for `"facade stub"` — no matches).

## Files touched (18, all newly written; nothing else in the repo was modified)
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👁️set-layer-visible/🦠️mutation/🟦️component.ts`
- `…/👁️set-layer-visible/🔺️diff/🟦️component.ts`
- `…/👁️set-layer-visible/↩️inverse/🟦️component.ts`
- `…/🔒️set-layer-locked/🦠️mutation/🟦️component.ts`
- `…/🔒️set-layer-locked/🔺️diff/🟦️component.ts`
- `…/🔒️set-layer-locked/↩️inverse/🟦️component.ts`
- `…/🌫️set-layer-opacity/🦠️mutation/🟦️component.ts`
- `…/🌫️set-layer-opacity/🔺️diff/🟦️component.ts`
- `…/🌫️set-layer-opacity/↩️inverse/🟦️component.ts`
- `…/🖌️set-layer-blend-mode/🦠️mutation/🟦️component.ts`
- `…/🖌️set-layer-blend-mode/🔺️diff/🟦️component.ts`
- `…/🖌️set-layer-blend-mode/↩️inverse/🟦️component.ts`
- `…/🔃reorder-layer/🦠️mutation/🟦️component.ts`
- `…/🔃reorder-layer/🔺️diff/🟦️component.ts`
- `…/🔃reorder-layer/↩️inverse/🟦️component.ts`
- `…/🧬️duplicate-layer/🦠️mutation/🟦️component.ts`
- `…/🧬️duplicate-layer/🔺️diff/🟦️component.ts`
- `…/🧬️duplicate-layer/↩️inverse/🟦️component.ts`

## Unfinished / out of scope
Nothing unfinished within the assigned 18-file list. Note for awareness (not touched, not in
scope): the sibling verbs `create-layer`, `delete-layer`, `rename-layer`,
`replace-layer-fill`/`-stroke`, `set-layer-boolean-operation`, `update-layer-transform`,
`update-layer-trace-params` have **no** TS component.ts files at all yet at their triad leaves
(not stubs — simply absent), so the `🧬️mutations/🟦️component.ts` union root's mismatch with the
jack-style per-verb discriminated union (it currently only declares `noMutation`/`setSnapshot`)
was left alone — it was already filled before this task and is not one of the 18 target leaves.

---

# Follow-up: union root completed, leaf bulk-creation ruled out

The coordinator asked for two follow-ups after the initial 18-file pass. Final rulings below.

## Item 1 — `🖍️draw` mutation union root: DONE

`…/✳️any/🧬️schema/🧬️mutations/🟦️component.ts` previously declared only a `JsonMutation` type with
two arms (`noMutation`, `setSnapshot`) that don't correspond to any Rust `DrawMutation` variant —
not referenced anywhere else in the repo (`grep -rn "JsonMutation" --include="*.ts"` under
`🖍️draw` hits only that file's own declaration and two unrelated `JsonMutationsText`/
`JsonMutationsBinary` binary/text wire types), so it was safe to replace outright.

Rewrote it as a real 14-arm discriminated union, one arm per `DrawMutation` variant, same
declaration order and camelCase discriminant as the Rust dispatch enum
(`…/🧬️schema/🧬️mutations/🦀️component.rs:14-29`, `#[serde(tag = "mutation", rename_all =
"camelCase")]`), in the jack reference's `({ mutation: "..." } & Payload)` shape
(`✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️component.ts`).

Six arms import their payload from the leaf files filled in the first pass
(`SetLayerVisible`, `SetLayerLocked`, `SetLayerOpacity`, `SetLayerBlendMode`, `DuplicateLayer`,
`ReorderLayer`). The other eight verbs have no TS `🦠️mutation` leaf on disk (see item 2 ruling
below), so their payload interfaces are inlined directly in the union-root file, each annotated
with its Rust source:

- `RenameLayer` ← `✏️rename-layer/🦠️mutation/🦀️component.rs:12-15` — `{ layerId, newName }`.
- `UpdateLayerTransform` ← `🔄️update-layer-transform/🦠️mutation/🦀️component.rs:13-18` — `{ layerId, transform: DrawTransform }`; `DrawTransform` itself mirrors `🗿️artifacts/🖍️draw/🦀️component.rs:33-42` (`x, y, scaleX, scaleY, rotation`).
- `ReplaceLayerFill` ← `🔁replace-layer-fill/🦠️mutation/🦀️component.rs:13-18` — `{ layerId, fill?: FillStyle }` (Rust's `#[serde(skip_serializing_if = "Option::is_none")]` ⇒ TS optional, not `| null`); `FillStyle` mirrors the tagged union at `🗿️artifacts/🖍️draw/🦀️component.rs:61-78` (`solid` / `linearGradient` / `radialGradient`, tag `kind`), reusing a `GradientStop` mirror of lines 52-58.
- `ReplaceLayerStroke` ← `♻️replace-layer-stroke/🦠️mutation/🦀️component.rs:13-18` — `{ layerId, stroke?: StrokeStyle }`; `StrokeStyle` mirrors lines 82-90 (`color, width, cap, join, dash?`).
- `SetLayerBooleanOperation` ← `🔀set-layer-boolean-operation/🦠️mutation/🦀️component.rs:12-15` — `{ layerId, booleanOperation }`.
- `UpdateLayerTraceParams` ← `🔧update-layer-trace-params/🦠️mutation/🦀️component.rs:13-17` — `{ layerId, params: DrawTraceParams }`; `DrawTraceParams` mirrors lines 108-112 (`threshold, simplifyEpsilon`).
- `CreateLayer` ← `🌱create-layer/🦠️mutation/🦀️component.rs:12-18` — `{ parentId?, index?, layer: DrawLayerNode }` (both `parentId`/`index` carry `skip_serializing_if` ⇒ optional); `DrawLayerNode` reuses the existing generic mirror already at `✳️any/🧬️schema/🟦️component.ts:34-37` (`{ kind: string; [key: string]: unknown }`), imported as `../🟦️component.ts`.
- `DeleteLayer` ← `🗑️delete-layer/🦠️mutation/🦀️component.rs:12-14` — `{ layerId }`.

## Item 2 — bulk-creating the 8 missing verbs' leaves: RULED OUT, nothing created

Read the two named gates in `📜️script.ts` directly:

- **`policySubsetTsParityBreaches`** (line 27043) only checks one file per subset —
  `<subset>/🧬️schema/🟦️component.ts` (the schema ROOT, 49 lines for `🖍️draw`, already real) — flagging
  it only if ≤7 lines. It does not look at `🧬️mutations/` leaves at all, so it's irrelevant here.
- **`policyMutationTsMirrorBreaches`** (line 28337) is the relevant one. It emits two breach
  families, both `priority: "low"`:
  - `mutation-ts-mirror-stub-*` — a `.ts` leaf under `🧬️mutations/` whose stripped content is `""`
    or `export {};` (line 28347).
  - `mutation-ts-mirror-absent-*` — a Rust triad leaf with **no** `.ts` sibling at all (line 28360).

  Its own doc comment (28330-28336) calls the stub shape "near-universal today", keeps both
  families advisory "rather than seeding ~1000+ file paths for a finding that blocks nothing," and
  names the real fix as landing DSL TS codegen for the triad, not hand-authoring. Confirmed the
  function itself is **not called anywhere** in `📜️script.ts` (`grep -n
  "policyMutationTsMirrorBreaches("` returns only its own definition line) — it isn't wired into any
  aggregator or the `verify`/`runGate` pipeline at all right now.
  - Separately confirmed how "low"-priority breaches are treated where similar aggregators ARE
    wired in: `VerifyScript.runGate`'s dissolve-core block (`📜️script.ts:11042-11054`) does
    `.filter((b) => b.priority === "high")` before throwing, and the inference-family region's own
    comment (line ~29151) states the same pattern generalizes — only `"high"`-priority breaches ever
    block a gate.

**Conclusion: no taxonomy position in `🖍️draw` is REQUIRED to carry a TS mirror by any currently
enforced gate.** An absent leaf is at most a low-priority, non-blocking, currently-unwired tracking
signal whose intended remedy is future codegen. Per the coordinator's ruling, item 2 is skipped
entirely — zero new leaf files were created for `create-layer`, `delete-layer`, `rename-layer`,
`replace-layer-fill`, `replace-layer-stroke`, `set-layer-boolean-operation`,
`update-layer-transform`, `update-layer-trace-params`. Their payload shapes are documented inline
in the union root instead (item 1, above).

## Re-verification

Same scratch tsconfig, same tree, re-run after the union-root rewrite:

```
bunx tsc --noEmit -p /private/tmp/claude-501/-Users-ueli-Documents-semio/c17a0f0b-94f9-4f2f-bbd0-8ff82df33749/scratchpad/tsconfig.draw-mutations.json
```

Output: **no errors** (empty output, exit clean) — same as the first pass, now also covering the
120-line rewritten union root.

## Final file list (19 files touched across both passes; nothing else in the repo was modified by this session)

18 leaves from the first pass (see file list above), plus:
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️component.ts` (rewritten: `JsonMutation` → real 14-arm `DrawMutation` union)

Note: `git status` also shows `…/🧬️mutations/🦀️component.rs` (the Rust dispatch enum, sibling of
the union root) as modified. This session did not touch any `.rs` file — that change is unrelated
concurrent work from another session (CLAUDE.md: ignore unrelated concurrent changes, never treat
them as this session's own).
