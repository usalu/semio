# W2 — block5d boots non-empty, orphan `demo-session` removed

Subset root (all paths below are relative to it unless stated):
`✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/`

## 1. Boot document

### What changed

| File | Symbol | Change |
|---|---|---|
| `🧬️schema/🦀️.rs` | `default_block5d_snapshot()` | NEW. Parses the bundled example DSL, falls back to `empty_block5d_snapshot()` on a parse error. |
| `🧬️schema/🦀️.rs` | `default_definition_boots_on_the_forest_left_example` | NEW test — non-empty, part-kind id/label, first representation's `mesh_url`, 1 grip kind, 1 grip. |
| `✏️editor/🦀️.rs` | `Block5dPlayApp::initial_snapshot` | `empty_block5d_snapshot()` → `default_block5d_snapshot()` (+ doc comment). |
| `👁️viewer/🦀️.rs` | `Block5dViewer::initial_snapshot` | `empty_block5d_snapshot()` → `default_block5d_snapshot()` (+ doc comment). |
| `✏️editor/🦀️.rs` | `boots_on_the_forest_left_example_document` | NEW app-level test — boots non-empty and both windows render the content (`board` shows the label, `world` shows the `.glb`). |
| `✏️editor/🦀️.rs` | `add_grip_kind_then_add_grip_then_remove_round_trips` | ADAPTED to baseline-relative counts (was absolute `1`/`0` against an empty boot). |
| `✏️editor/🦀️.rs` | `undo_redo_round_trips_through_the_wrapper` | ADAPTED to baseline-relative counts. |

`empty_block5d_snapshot()` and its `empty_definition_matches_default` test are untouched.

The implementation mirrors the in-plugin precedent that already landed for block2d
(`🗿️artifacts/◻️2d/…/🧬️schema/🦀️.rs:default_block2d_snapshot`, W1) — same helper shape, same
`unwrap_or_else(|_| empty_…())` fallback, same `super::snapshot::text::…` path, same test shape.
Procedural's `generation3d` uses the same idea one notch terser
(`🗿️artifacts/🧊️generation3d/…/🧬️schema/🦀️.rs:286 default_snapshot()` → `parse_dsl(HEX_COLUMN).unwrap_or_default()`);
the block2d form was preferred because it names the fallback explicitly.

### DEVIATION — booted on `hexagonal-cut-concrete-forest-left`, not `nakagin-capsule`

The task named `nakagin-capsule`. Both fixtures qualify on the stated criterion — verified by reading
the DSL assets:

- `📚️examples/🏢️nakagin-capsule/🖼️assets/🧪️nakagin-capsule/🗣️.dsl.semio` → `r0 "Full Detail" "/mesh/🧊️capsule_J.glb"`
- `📚️examples/🎬️hexagonal-cut-concrete-forest-left/🖼️assets/🧪️hexagonal-cut-concrete-forest-left/🗣️.dsl.semio` → `r0 default "/mesh/🧊️hexagonal-cut-concrete-forest-left.glb"`

So the mesh_url escape hatch does not apply. A different, harder constraint does:

**`Block5dMutation` has no verb that changes `part_kind.id`.** The 42 entries of `KINDS`
(`🧬️schema/🧬️mutations/🦀️.rs:82-122`) cover `rename-part-kind` (name), `change-part-kind-label`,
`-variant`, `-description`, `-icon`, `-unit` — and nothing for `id`; `✏️rename-part-kind/🔺️diff/🦀️.rs`
explicitly rebuilds the identity as `BlockKindIdentity { name: …, ..base.part_kind.clone() }`. The
`setActiveExample` handler loads a whole document as a *minimal ordered batch of semantic mutations*
(`✏️editor/🎮️commands/🎬️set-active-example/🦀️.rs:replace_document_operations`), so it structurally
cannot carry identity from one example to another.

Consequence of booting on nakagin: `part_kind.id` would stay `"Capsule J"` for the whole session, and
two existing tests that assert the loaded identity would fail —

- `set_active_example_loads_forest_left_fixture` → `assert_eq!(projection.part_kind.id, "Hexagonal Cut Concrete Forest Left")`
- `export_media_catalog_out_wraps_the_puzzle5d_fragment` → `assert_eq!(value["parts"][0]["id"], "Hexagonal Cut Concrete Forest Left")`

Booting on forest-left keeps the boot document's identity consistent with everything the suite
asserts, needs no weakening of an existing assertion, and matches block2d exactly. The `id` gap is
recorded here as a **finding for the coordinator**, not fixed: closing it means a new mutation kind,
which touches the mutation catalog, the JSON schema, the `🧩️mutate-block-5d-1` oracle vectors and the
Python second implementation — well outside this packet.

Side effect worth knowing: with the boot document equal to the example `setActiveExample` loads,
`set_active_example_loads_forest_left_fixture` is now near-vacuous (the replacement batch is empty).
block2d has the identical vacuity. A follow-up that boots on nakagin *after* an id verb exists would
turn that test into a real cross-example replacement test.

## 2. Orphaned `demo-session` module removed

Deleted `✏️editor/📚️examples/` in full (its only child was `🎬️demo-session`, with
`🦀️.rs`, `🟦️.ts`, `🖼️assets/🎮️.cmd.semio`, `🧪️tests/🦀️.rs`, `🧪️tests/🟦️.ts`), and removed its mount
from `✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust/🦀️.rs` (the `#[path = "…/🖐️5d/…/✏️editor/📚️examples/🎬️demo-session/🦀️.rs"]`
+ `pub mod app_5d_demo_session;` pair).

Reference sweep before deleting (repo-wide, `--include` rs/ts/tsx/json/jsonc/md, `target/` and
tickets excluded): the only reference to `app_5d_demo_session` anywhere was its own mount line. The
subset registers its examples from `📚️examples/` instead (`✳️any/🦀️.rs:examples()` →
`art_5d_hexagonal_cut_concrete_forest_left::source()`, `art_5d_nakagin_capsule::source()`), never from
the editor-surface facet. Storybook: `.storybook/**` contains no `demo-session` or `block5d`
reference at all.

### Registry — no regeneration needed, verified rather than assumed

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🎮️playgrounds.ts:29` lists
`examples: ["🎬️demo-session"]` for the `block5d` variant. That list is **per crate, not per
variant** — `discoverExamplesForPlayground` (`📇️registry/📜️script.ts:486`) forwards the crate path to
`registryExampleCatalog` (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts:8773`),
which scans every artifact under the plugin root. block2d still carries
`🗿️artifacts/◻️2d/…/✏️editor/📚️examples/🎬️demo-session/`, so the id survives. Evaluated directly:

```
$ bun -e 'const m = await import("./🧰️framework/…/📦️packages/🟦️typescript/🟦️.ts");
          console.log(m.registryExampleCatalog(process.cwd(), "✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust", m.loadCatalogTaxonomy()))'
block example catalog: ["🎬️demo-session"]
```

Unchanged ⇒ `bun nx run @semio-tech/plugin-registry:generate` would be a no-op for this change and
was NOT run. **Hand-off note:** if whoever owns block2d also deletes its `demo-session`, the block
crate's catalog becomes `[]` and `🎮️playgrounds.ts` DOES have to be regenerated for all three block
rows.

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json:6891` lists `🎬️demo-session` under
`members-of-examples.memberNames`, which is a repo-wide vocabulary of allowed example slugs (49 other
`demo_session` mounts still exist across plugins), not per-plugin membership — untouched.

### Related finding (not acted on)

`registryExampleCatalog` scans `🗿️artifacts/<a>/📚️examples/` and
`…/🪆️subsets/<s>/{✏️editor,👁️viewer}/📚️examples/` only. block's real examples live at
**subset** level (`🪆️subsets/✳️any/📚️examples/`), which the scan never visits — which is why the block
playground rows advertise the placeholder `demo-session` and not
`🎬️hexagonal-cut-concrete-forest-left` / `🏢️nakagin-capsule`. That is a registry-discovery gap
affecting all three block variants.

## 3. Verification

See `🗑️generated/w2-check.txt` and `🗑️generated/w2-test.txt` for the raw tails.

<!-- VERIFICATION -->

## 4. Unverified / caveats

<!-- CAVEATS -->
