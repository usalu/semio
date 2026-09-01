# TS Mirrors — procedural2d 🧬️mutations

Filled all 25 placeholder TypeScript leaves listed in
`/private/tmp/claude-501/-Users-ueli-Documents-semio/c17a0f0b-94f9-4f2f-bbd0-8ff82df33749/scratchpad/leaves-procedural2d.txt`,
under
`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/`.

Every file was previously a "WASM facade stub" (`export {};`). Each now mirrors its Rust
counterpart (`🦀️component.rs` sibling) as real TypeScript types + light mechanical logic, following
the established, already-filled `jack` triad-leaf convention (studied first as the reference
pattern per the task brief:
`✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/`):
- `🦠️mutation` = a plain payload `interface`, field-for-field from the Rust struct.
- `🔺️diff` = a small `diff(payload)` function returning the *mechanical* sparse delta (no BASE
  state needed for any of these 8 verbs — matches jack's convention of pure-payload diffs).
- `↩️inverse` = an `inverse(payload, ...resolvedBaseState)` function, taking only whatever
  minimal already-resolved BASE piece Rust's `inverse()` genuinely needs (mirrors jack's
  `baseNode: JackNode | undefined`-style params), returning the array of the actual inverse
  mutation's payload shape.

Also discovered a second, less rigorous precedent (`iso16757` plugin) that uses bare empty/partial
interfaces with **incorrect** field coverage (e.g. `RenameProduct` TS payload is missing its `id`
field; `DeleteProductInverse` is aliased to `DeleteProduct` even though the real Rust inverse
constructs a `CreateProduct`). I did not follow that pattern — the task brief named `jack`
specifically, and `jack`'s fills are internally consistent with their Rust sources. iso16757's
inconsistencies are noted here only as a heads-up, not otherwise touched.

## Scope note: 8 of 14 union variants

`Procedural2dMutation` (…/🧬️mutations/🦀️component.rs:44-59) has 14 variants, but only 8 have a
`🦠️mutation`/`🔺️diff`/`↩️inverse` triad + a `🟦️component.ts` at all (`CreateWidget`, `DeleteWidget`,
`ConnectSynapse`, `DisconnectSynapse`, `MoveWidget`, `ClearWidgetLayout`, `UpdateCamera`,
`ChangeSchema`) — confirmed by directory listing: the other 6 (`ReplaceWidget`, `ReplaceSynapse`,
`CreateGeneration`, `DeleteGeneration`, `RenameGeneration`, `ChangeGenerationValue`) have Rust-only
triads with no `🟦️component.ts` stub anywhere, so they are out of scope for TS mirroring entirely
(not just for this task). The union-root `🟦️component.ts` mirrors only those 8, matching the file
count in scope (8×3 + 1 = 25).

## Per-file mapping (Rust source : TS leaf)

All Rust sources are under
`…/🧬️schema/🧬️mutations/<verb>/`. Rust line numbers refer to the struct/fn as read.

| Verb | 🦠️mutation Rust | 🔺️diff Rust | ↩️inverse Rust |
|---|---|---|---|
| 🌱create-widget | `🦠️mutation/🦀️component.rs:16-19` `CreateWidget{index,widget}` | `🔺️diff/🦀️component.rs:7-13` | `↩️inverse/🦀️component.rs:7-9` |
| 🗑️delete-widget | `🦠️mutation/🦀️component.rs:14-17` `DeleteWidget{id}` | `🔺️diff/🦀️component.rs:7-18` | `↩️inverse/🦀️component.rs:7-12` |
| 🔗connect-synapse | `🦠️mutation/🦀️component.rs:16-19` `ConnectSynapse{index,synapse}` | `🔺️diff/🦀️component.rs:7-22` | `↩️inverse/🦀️component.rs:7-9` |
| ✂️disconnect-synapse | `🦠️mutation/🦀️component.rs:14-17` `DisconnectSynapse{id}` | `🔺️diff/🦀️component.rs:7-12` | `↩️inverse/🦀️component.rs:7-12` |
| 📍move-widget | `🦠️mutation/🦀️component.rs:16-19` `MoveWidget{id,layout}` | `🔺️diff/🦀️component.rs:7-15` | `↩️inverse/🦀️component.rs:8-13` |
| 🧹clear-widget-layout | `🦠️mutation/🦀️component.rs:15-17` `ClearWidgetLayout{id}` | `🔺️diff/🦀️component.rs:7-12` | `↩️inverse/🦀️component.rs:7-12` |
| 🎛set-camera | `🦠️mutation/🦀️component.rs:15-17` `UpdateCamera{camera}` | `🔺️diff/🦀️component.rs:7-16` | `↩️inverse/🦀️component.rs:7-9` |
| 🔤change-schema | `🦠️mutation/🦀️component.rs:13-15` `ChangeSchema{schema}` | `🔺️diff/🦀️component.rs:7-12` | `↩️inverse/🦀️component.rs:7-9` |

Union root: `🧬️mutations/🟦️component.ts` mirrors `Procedural2dMutation` enum declaration order
(…/🧬️mutations/🦀️component.rs:44-59), filtered to the 8 wired variants.

## Design decisions worth flagging

- **`diff()` shape**: Rust's actual runtime `diff()` funnels through `diff_fixture_from_helpers`
  (…/🔺️diff/📝️text/🦀️component.rs:76-88, 180-183), which folds the change into a *whole* replaced
  `fixture: FlowFixture` on the outer `Procedural2dDiff`. But the internal Rust representation
  before that fold is a real sparse per-collection delta (`WidgetsDiff`/`SynapsesDiff`/`LayoutDiff`,
  same file lines 17-38: `{ removed: Vec<String>, set: Vec<(usize|String, T)> }`). Following jack's
  precedent (which also mirrors the semantically meaningful sparse delta — e.g. `JackNodesDelta`
  `{added,removed,patched}` — rather than jack's own coarse wire-level `content`-handle replace), I
  mirrored these same internal sparse shapes (`{ removed: string[]; set: Array<[key, T]> }`) rather
  than a whole-`FlowFixture` replace. This keeps `diff()` a pure function of `payload` alone (no
  BASE needed), consistent with every jack `diff()` example.
- **`Widget` type**: procedural2d's already-filled outer schema surfaces
  (`🧬️schema/🔺️diff/🟦️component.ts` and `🧬️schema/📸️snapshot/🟦️component.ts`, both pre-existing,
  not touched by me) type `Widget` as an opaque `string` ("Polymorphic flow widget — JSON blob"),
  matching the proto (`message Widget { string json = 1; }`) and graphql (`scalar Widget`)
  surfaces. I kept this convention rather than inventing a full 9-variant discriminated union
  (Rust's real `flow::Widget` enum, confirmed at
  `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📄️artifact/🦀️component.rs:180-248`, has 9 variants —
  Neuron/InputSlider/InputNote/InputImage/Variable/OutputPreview/OutputAction/OutputExport/Cluster
  — all with `id: String` as their first field). Since every schema surface for this artifact
  already treats `Widget` as an opaque blob, redeclaring a full union here would be new, unrequested
  scope and would diverge from sibling files.
  - Consequence: `widget_id()` (Rust helper, extracts `id` from the real struct) can't be mirrored
    structurally in TS. `🌱create-widget/↩️inverse` and `🗑️delete-widget/↩️inverse` instead do
    `(JSON.parse(payload.widget) as { id: string }).id` — legitimate given the documented "JSON
    blob" contract, and every real variant does carry `id` as its first field.
  - `🗑️delete-widget/🔺️diff`+`↩️inverse` and `✂️disconnect-synapse/…` inverse, and
    `📍move-widget`/`🧹clear-widget-layout` inverses, all take an explicit already-resolved
    `baseWidget`/`baseSynapse`/`baseLayout` parameter instead of a full `FlowFixture` — mirrors
    jack's `baseNode: JackNode | undefined` pattern exactly (accept only the minimal BASE-derived
    piece Rust's `inverse()` needs, not the whole snapshot type).
- Types `SynapseSpec`, `WidgetLayout`, `CameraJson` are redeclared locally per verb-triad (in the
  owning `🦠️mutation` leaf, imported by sibling `🔺️diff`/`↩️inverse`) rather than imported from the
  outer `🔺️diff`/`📸️snapshot` schema surfaces — this matches the repo-wide convention observed:
  `🧬️schema/🔺️diff/🟦️component.ts` and `🧬️schema/📸️snapshot/🟦️component.ts` **already fully
  duplicate** these exact same type aliases independently of each other (confirmed by reading both,
  byte-identical `CameraJson`/`WidgetLayout`/`SynapseSpec`/`Widget`/`FlowFixture` declarations in
  both files) — i.e. every schema surface in this codebase is self-contained, not cross-importing.

## Verification

Typecheck command (no dedicated lint/typecheck nx target exists for
`@semio-tech/procedural-js` — its only target is `test`, which just prints a static line; root
`tsconfig.json` covers `**/*.ts` repo-wide with `strict: true`, `moduleResolution: "bundler"`,
target `ESNext`). Ran `tsc` directly, scoped to just the 25 files, using the same compiler flags as
root `tsconfig.json`:

```
node_modules/.bin/tsc --noEmit --strict --target ESNext --module ESNext \
  --moduleResolution bundler --esModuleInterop --skipLibCheck @tsc-files.txt
```

Result: **exit code 0, zero diagnostics** (empty stdout/stderr). Re-ran a second time to confirm
the empty output wasn't a fluke — same result.

Stub-marker re-grep over the 25-file list (`grep -l "facade stub\|WASM facade"`): **zero matches**
— confirmed via `git diff --stat` too: exactly 25 files changed under this
`🧬️mutations/` subtree, matching the assigned list 1:1, no stray files.

One stray file was created and immediately removed during this session: a typo'd path
(`🏅️标准` instead of `🏅️standards`, an emoji/CJK autocomplete glitch while hand-typing a Write-tool
path) created `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️procedural2d/🏅️标准/…` — caught immediately,
`rm -rf`'d before any further edits, confirmed clean via `ls` on the parent dir. No trace remains
(not in `git status`, since it was created and deleted within this same session before any commit).

## Nothing left unfinished

All 25 files filled, typechecked clean, no stub markers remain. I did not touch
`iso16757` or `jack` (read-only references) or any file outside the assigned 25.
