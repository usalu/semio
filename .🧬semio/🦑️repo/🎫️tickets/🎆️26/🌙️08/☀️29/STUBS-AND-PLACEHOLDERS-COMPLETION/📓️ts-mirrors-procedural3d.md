# TS Mirrors — procedural3d `🧬️mutations` (43 leaves)

Filled every `🟦️component.ts` leaf under `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/` listed in `leaves-procedural3d.txt` (43 files). All 14 procedural3d mutation verbs were entirely stub (mutation+diff+inverse), plus the union root — unlike jack, no verb in this facet had a filled reference to copy directly, so every field was mirrored straight from Rust.

## Pattern used

Followed the jack reference precedent (`🔌️jack/…/🧬️mutations/🌱️create-node/🟦️component.ts` + `🔺️diff` + `↩️inverse`):
- `🦠️mutation/🟦️component.ts` — payload interface, camelCase fields, self-contained (dependent types imported from the verb that "owns" the entity).
- `🔺️diff/🟦️component.ts` — a pure `diff(payload)` function returning the *sparse collection delta* shape the Rust `diff.rs` builds (`{removed, set}` pairs mirroring the Rust `WidgetsDiff`/`SynapsesDiff`/`LayoutDiff` helper structs, or `{ops: [...]}` for the generation collection), not the full `Procedural3dDiff`.
- `↩️inverse/🟦️component.ts` — a pure `inverse(payload, baseXxx)` function returning plain (untagged) sibling-mutation payload objects, taking whatever BASE-derived value the Rust function needed as an extra parameter (since the facade cannot read BASE itself) — same convention jack uses (`baseNode`, `severedEdgeIds`, etc).

Canonical type ownership (avoids duplicating shared value types across leaves, mirrors jack's `create-node` owning `JackNode`):
- `Widget` (opaque `string`, JSON text) + `widgetId()` helper → `🌱create-widget/🦠️mutation`
- `SynapseSpec` → `🔗connect-synapse/🦠️mutation`
- `WidgetLayout` → `📍move-widget/🦠️mutation`
- `CameraJson` → `📷update-camera/🦠️mutation`
- `FormGeneration` → `➕create-generation/🦠️mutation`

`Widget = string` (and `widgetId()` parsing it as JSON) matches the **already-committed, repo-wide** convention for this Rust enum — confirmed identical in `🌊️flow/…/🔺️diff/🟦️component.ts:66` and in procedural3d's own already-filled `🧬️schema/🔺️diff/🟦️component.ts:44` (both say "Widget payload as JSON text (opaque enum)"). I deliberately did **not** import `CameraJson`/`WidgetLayout`/`SynapseSpec`/`FormGeneration` from that same top-level `🔺️diff/🟦️component.ts` file, because its `FormGeneration` mirror (`{id, name, valuesJson: string}`) is wrong against the real Rust struct (`flow::playbook::FormGeneration` has `values: Map<String, Value>`, a JSON object, not a stringified field) — see `🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🦀️component.rs:345`. My local `FormGeneration` uses `values: Record<string, unknown>` to stay faithful to Rust.

## Rust sources mirrored (file:line)

Top-level union: `🧬️mutations/🦀️component.rs:154-169` (`Procedural3dMutation` enum, 14 variants in `KINDS` order at line 176).

| Verb | mutation.rs | diff.rs | inverse.rs |
|---|---|---|---|
| create-widget | `🌱create-widget/🦠️mutation/🦀️component.rs:17-20` | same dir `🔺️diff/🦀️component.rs:53-59` | `↩️inverse/🦀️component.rs:70-72` |
| update-widget | `🩹update-widget/🦠️mutation/🦀️component.rs:93-95` | `🔺️diff:130-144` | `↩️inverse:154-160` |
| delete-widget | `❌delete-widget/🦠️mutation/🦀️component.rs:179-181` | `🔺️diff:213-218` | `↩️inverse:229-234` |
| connect-synapse | `🔗connect-synapse/🦠️mutation/🦀️component.rs:252-255` | `🔺️diff:287-299` | `↩️inverse:309-311` |
| update-synapse | `🔄update-synapse/🦠️mutation/🦀️component.rs:330-332` | `🔺️diff:365-374` | `↩️inverse:384-389` |
| disconnect-synapse | `✂️disconnect-synapse/🦠️mutation/🦀️component.rs:407-409` | `🔺️diff:441-446` | `↩️inverse:457-462` |
| move-widget | `📍move-widget/🦠️mutation/🦀️component.rs:481-484` | `🔺️diff:516-524` | `↩️inverse:535-540` |
| delete-widget-position | `🧹delete-widget-position/🦠️mutation/🦀️component.rs:558-560` | `🔺️diff:592-600` | `↩️inverse:611-616` |
| update-camera | `📷update-camera/🦠️mutation/🦀️component.rs:20-22` | `🔺️diff:50-58` | `↩️inverse:67-69` |
| change-schema | `🔤change-schema/🦠️mutation/🦀️component.rs:87-89` | `🔺️diff:117-125` | `↩️inverse:133-135` |
| create-generation | `➕create-generation/🦠️mutation/🦀️component.rs:152-154` | `🔺️diff:187-193` | `↩️inverse:202-204` |
| delete-generation | `🗑delete-generation/🦠️mutation/🦀️component.rs:217-219` | `🔺️diff:250-255` | `↩️inverse:265-267` |
| rename-generation | `🏷rename-generation/🦠️mutation/🦀️component.rs:280-283` | `🔺️diff:314-322` | `↩️inverse:331-333` |
| change-generation-value | `🔧change-generation-value/🦠️mutation/🦀️component.rs:349-353` | `🔺️diff:383-391` | `↩️inverse:402-409` |

Shared helper structs (`WidgetsDiff`/`SynapsesDiff`/`LayoutDiff`, `{removed, set: Vec<(index_or_id, T)>}`) mirrored from `🧬️schema/🔺️diff/📝️text/🦀️component.rs:17-35`. `Widget` enum (9 variants, `#[serde(tag="kind")]`) read from `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📄️artifact/🦀️component.rs:178-238` to confirm the opaque-string mirror is the right call rather than an omission. `GenerationMutation` (`#[serde(tag="kind", rename_all="camelCase")]`, `Add/Remove/Rename/UpdateValues`) read from `🧰️framework/…/📖️playbook/🦀️component.rs:469-475` to get the `ops` ("kind") literal shapes right in the generation diffs.

## Verification

Typecheck command actually run (repo has no dedicated `project.json`/nx target for this facet's TS leaves, and the root `tsconfig.json` includes the whole monorepo, so I typechecked the exact 43-file list directly against the same compiler options declared in the root `tsconfig.json`):

```
bunx tsc --noEmit --strict --esModuleInterop --isolatedModules --moduleResolution bundler \
  --module ESNext --target ESNext --resolveJsonModule --skipLibCheck --allowImportingTsExtensions \
  <43 file paths from leaves-procedural3d.txt>
```

Output: **empty, zero errors** (`bunx tsc --version` → `Version 5.9.3`).

Stub grep re-run over the same 43-file list (`grep -q "stub" "$p"` for each): **zero matches** — no `export {};`-only stub leaves remain in this list.

`git status --porcelain` scoped to `🧬️mutations/` under procedural3d: exactly the 43 files I touched, all `M` (modified from stub), no extra/unexpected files — no collision observed with the concurrent `PROCEDURAL-3D-END-TO-END` session during this run.

## Unfinished / caveats

- The already-filled `🧬️schema/🔺️diff/🟦️component.ts` (sibling, out of my file list) has an inaccurate `FormGeneration` mirror (`valuesJson: string` instead of `values: Record<string,unknown>`); I did not touch it since it wasn't in my assigned list, but it's now inconsistent with my `➕create-generation/🦠️mutation` mirror. Worth a follow-up fix by whichever session owns that file.
- `diff.ts`/`inverse.ts` leaves are pure structural facades (matching the jack precedent's level of fidelity) — they do not reimplement Rust's validation/Fatal-outcome logic (duplicate-id checks, finite-number checks, no-op detection); they only mirror the shape of the delta/inverse a valid call would produce, exactly like the jack reference examples do.
