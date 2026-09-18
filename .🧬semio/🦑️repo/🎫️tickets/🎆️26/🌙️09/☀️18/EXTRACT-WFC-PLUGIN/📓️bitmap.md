# 🖼️ A1 — the `s.wfc.bitmap` artifact, as authored

Crate `semio-s-artifact-wfc-bitmap` at `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap`. Dialect
`s.wfc.bitmap@1/*`, OS kind id `2d.wfcbitmap`, app ids `s.wfc.bitmap@1/*#editor` and
`…#viewer`, module ident `bitmap`, surfaces `BitmapEditor`/`BitmapViewer`, labels "Bitmap"/"Bitmap".

---

## 1. Status

| gate | command | result |
|---|---|---|
| native check | `cargo check -p semio-s-artifact-wfc-bitmap --features component-app-assembly --lib --tests -j 4` | ✅️ green, zero warnings from this crate (`🗑️generated/bitmap/check-audit.txt`) |
| native tests | `RUST_MIN_STACK=33554432 cargo test … --lib -j 4 -- --test-threads=4` | ✅️ **176 passed, 0 failed, 1 ignored** (`test-stroke-2.txt`, 36 s) |
| clippy | `cargo clippy … --lib --tests -j 4` | ✅️ green, **zero warnings from this crate** (`clippy-audit.txt`) |
| wasm | `CARGO_PROFILE_WASM_DEV_DEBUG=false cargo check … --target wasm32-wasip2 -j 4` | ✅️ green (`wasm-audit.txt`) |
| python oracle + ticket gate | `python3 "$T/🐍️bitmap-oracle.py"` | ✅️ `bitmap oracle gate PASSED` — 10 vectors, 10 fixture cases, 25 JSON leaves, 0 problems |
| TypeScript | `NX_DAEMON=false bun nx run @semio-tech/wfc-js:test` | ✅️ 10 files / 148 tests (`ts-audit.txt`) |
| taxonomy enforce | `bun ./📜️script.ts verify taxonomy enforce --scope …/🖼️bitmap` | ❌️ red for a repo-wide reason, NOT this artifact — see §8 |

The one ignored test is `emit_committed_fixtures`, the generator every committed quintet and both
example `🗣️.dsl.semio` assets are printed from (§5).

---

## 2. Document model as authored

`🧬️schema/📸️snapshot/🦀️.rs`, normative JSON Schema beside it, GraphQL/Proto/TS mirroring it.

```
BitmapSnapshot {
  schema: String                                  // = "s.wfc.bitmap"
  seed:   u64                                     // authored only via change-seed, never ambient
  input:  BitmapInput {
            width, height: u32                    // ≤ BITMAP_MAX_EDGE = 512
            palette: Vec<BitmapColor{r,g,b,a:u32}>  // ≤ 256 entries; one byte per pixel is the point
            pixels:  String                       // base64 of ONE PALETTE INDEX PER PIXEL, row-major
          }
  output: BitmapOutputSpec { width, height: u32, periodic: bool }
  model:  BitmapOverlappingModel {
            pattern_size: u32   // 2..=5
            symmetry:     u32   // 1..=8, the classic D4 prefix count
            periodic_input: bool
            ground: Option<u32> // palette index forced on the output's bottom row
          }
  pinned: Vec<BitmapPinnedPixel { x, y, color: u32 }>   // canonical ROW-MAJOR order
}
```

Three decisions worth stating.

**Pixels are base64 palette indices, not an inline array.** A 128 × 128 sample is one bounded string
rather than sixteen thousand DSL values. The codec is hand-written in the snapshot facet
(`encode_base64`/`decode_base64`, RFC 4648 §4) because `semio-framework-io-base64` carries no
`[workspace.dependencies]` alias and this artifact will not widen the root manifest for forty lines
of table lookup. A buffer that does not decode to exactly `width * height` bytes answers `None` —
never a truncated best effort.

**Pins are keyed by coordinate and kept sorted.** They carry no author-visible id, so `pin_key(x, y)`
is the diff machinery's key and `ordered_pin_index` is where an insert lands. That is what makes
`pin-pixel`/`unpin-pixel` point-invertible in POSITION as well as value — the law the taxonomy
report's §3.3 says fourteen remodeling vectors violated.

**`symmetry` is a count, not an enum**, and the ORDER it counts through is load-bearing. `1..=8`
selects the first N elements of `BITMAP_SYMMETRY_ORDER` — the classic reference sequence
`e, m, r, rm, r², r²m, r³, r³m`, rotations and reflections ALTERNATING — not `Transform2d::ALL`'s
rotations-then-reflections declaration order. The difference is not cosmetic: under the declaration
order `symmetry = 2` means "identity plus a quarter turn", which stands a meadow on its side, and the
bundled `flowers-24` example could not tile its own declared output at all (the solve answered a
CONTRADICTION). Under the alternating order `2` means "identity plus a horizontal mirror", which is
what an author reaching for 2 means, and the example tiles. This was found by the new local-similarity
test (§4), not by any test that existed before it. `symmetry` is an integer on the wire, so the
`rename_all = "SCREAMING_SNAKE_CASE"` silent-no-op hazard A2 reported does not touch this artifact —
no enum of this document has an uppercase wire token.

### Diff — `BitmapDiff`

```
schema?, seed?, inputWidth?, inputHeight?, inputPixels?, inputRegions[], palette?, output?, model?,
pinnedRemoved[], pinnedUpserted[(index, pin)]
```

Two lanes are deliberately coarser than an id-keyed collection delta, and the document is the reason.
`palette` is POSITIONAL: `add-palette-color` inserts AT an index and renumbers every pixel and pin at
or above it, so a "entry 3 changed" delta would not describe what happened. `inputPixels` is the
whole re-indexed buffer for exactly those renumbering mutations. Ordinary painting never touches
either: it rides `inputRegions`, a real sparse list of rectangular writes.

---

## 3. Mutations (10)

| folder | payload | diff lanes | inverse |
|---|---|---|---|
| `🎲️change-seed` | `seed` | `seed` | prior seed |
| `📐️resize-input` | `width,height` | `inputWidth,inputHeight,inputPixels` | resize back **+** full-buffer restore |
| `🖌️set-input-pixels` | `x,y,width,height,pixels` | one `inputRegions` entry | the same rectangle with the prior bytes |
| `🎨️add-palette-color` | `index,color` | `palette` (+`inputPixels`,`pinnedUpserted` when the insert renumbers) | `remove-palette-color(index)` |
| `🖍️change-palette-color` | `index,color` | `palette` | prior colour at the same index |
| `🧽️remove-palette-color` | `index` | `palette` (+ renumber lanes) | `add-palette-color(index, colour)` |
| `🖼️resize-output` | `width,height,periodic` | `output` + `pinnedRemoved` | prior spec **+** re-pin every cascaded pin |
| `⚙️change-model` | `patternSize,symmetry,periodicInput,ground` | `model` | prior parameters |
| `📌️pin-pixel` | `x,y,color` | `pinnedUpserted` at the CANONICAL row-major index | re-pin prior colour, or unpin |
| `📍️unpin-pixel` | `x,y` | `pinnedRemoved` | re-pin the base's colour |

Refusals are real, not decorative: `remove-palette-color` is FATAL while the colour is still painted
or pinned (`mutation.colour-in-use`) — silently repainting a document to make a delete succeed is the
quiet data loss this vocabulary exists to prevent; `set-input-pixels` is fatal on a malformed,
mis-sized, out-of-bounds or unknown-palette payload; `unpin-pixel` is fatal on a cell that carries no
pin; `resize-output` announces its pin cascade with an `info`-level `mutation.cascade`.

**Verbs.** `protocol::APPROVED_VERBS` has no `pin`/`unpin`, so per the coordinator's convention
`pin-pixel` is `verb: "fix"` (record `Fixed`) and `unpin-pixel` is `verb: "clear"` (record
`Cleared`) — the plugin-wide spelling the coordinator ruled; the kebab KINDS are unchanged. `set-input-pixels` keeps `verb: "set"` rather than
the suggested `replace`/`edit`: `set` IS in `APPROVED_VERBS`, and keeping it makes `kind` exactly
`verb`-`entity` for every kind in this artifact.

**Payload casing.** Every payload struct carries `#[value(rename_all = "camelCase")]`. Without it
`ChangeModel` serialized `pattern_size`/`periodic_input` while its own JSON Schema declared
`patternSize`/`periodicInput` — found by the Python oracle, not by the Rust tests, which never read
the schema.

---

## 4. Inference `s.wfc.bitmap.solve`

`🧬️schema/💡️inferences/🦀️.rs`. Job kind `semio.infer`, payload schema
`s.wfc.bitmap.inference.request.v1`, metadata owner `wfc`.

Stages: `Sample → Extract → Topology → Fixed → {Restore | Solve} → Decode → EncodeCommit → Complete`.

- **Sample** decodes the input buffer into `Sample2d` of `TileId`s.
- **Extract** runs `extract_2d` with `Extract2dConfig { window: patternSize, periodic_input, symmetry: symmetry_group(n) }`, where `symmetry_group(n)` is `SymmetryGroup2d::Custom(Transform2d::ALL[..n])`. Refuses a pattern universe over `MAX_BITMAP_PATTERNS = 4096`.
- **Topology** builds `Grid2dTopology::new(w, h, &Stencil2d::VonNeumann, stencil_relations(), b, b, None)` with `b = Wrap` when `output.periodic` else `Open`. `stencil_relations()` reconstructs `RelationId(0..4)` explicitly and says WHY in its doc: `extract_2d` declares its relations on a fresh `ModelBuilder` in `offsets()` order, so the ids are exactly `0..4` — the one coupling between the two calls, made visible instead of implicit.
- **Fixed** turns each pin into `(NodeId, PatternId)` through the anchor convention, then adds the ground row. A pin outside the output, or on a colour NO pattern anchors, is a REFUSAL (`bitmap-inference-pin-outside-output`, `bitmap-inference-unreachable-pin`) rather than a silent drop — dropping it would hand back a bitmap that quietly disobeys the document.
- **Solve** drives the resumable `WfcJob<Grid2dTopology>`; **Restore** resumes from a checkpoint.
- **Decode** maps the assignment back to palette indices through `PatternDecoder2d::anchor_tile`.
- **EncodeCommit** streams `{"pixels":…,"contradiction":…,"entropy":[…]}` into the payload writer.

Commit: `BitmapInferenceCommit { pixels: base64, contradiction: bool, entropy: Vec<f64> }`. Three
`InferredField`s — `BitmapSolve`, `BitmapContradiction`, `BitmapEntropy` — read only snapshot fields,
so `DepHash` caching is sound. Admission ceilings: 65 536 input cells, 65 536 output cells, 65 536
pins, 4 MiB output.

### The defining property, asserted through this artifact's own pipeline

`every_output_window_of_a_solved_example_occurs_in_the_symmetry_expanded_input` solves BOTH bundled
examples through `BitmapSnapshot` → `solve_with_job` and asserts that every `N × N` window of the
decoded output occurs in the SYMMETRY-EXPANDED input window set — rebuilding that set with the same
enumeration `extract_2d` performs (wrapped positions when `periodicInput`, fully in-bounds otherwise,
each expanded under the same `symmetry_group(n)` transforms). Output windows are taken with
wrap-around when `output.periodic` (`rooms-16`, 24 × 24 periodic, symmetry 8) and only at fully
in-bounds positions when it is not (`flowers-24`, 32 × 24 open, symmetry 2) — the positions the
four-neighbour stencil actually constrained. `local_similarity_holds_under_a_different_seed_and_a_non_periodic_output`
re-checks the law on a second seed and asserts the two seeds really disagree, so a pass cannot be an
accident of one sampler trajectory.

This is the artifact-level proof the brief's whole claim rests on. The engine crate's own
`extract_2d_output_is_locally_similar_to_its_sample` proves the property generically, but with
`SymmetryGroup2d::None` and bypassing `BitmapSnapshot` entirely — so it exercises neither the
symmetry expansion, nor the base64 carrier, nor the palette, nor the ground row, nor pin anchoring.
Both of this artifact's tests do.

### Four defects found and fixed while wiring it

1. **An unsatisfiable collapse was reported as an inference FAILURE.** `WfcJob` publishes an
   exhausted search as `PublicationKind::Fault` with detail `wfc-unsatisfiable`
   (`⚙️engine/💼️job/🦀️.rs:1771`), not as a commit-less completion. The Solve stage now intercepts
   exactly that detail, retires `fault.detail`, closes the child through `engine::job::close_job`, and
   answers the contradiction verdict. Every other fault stays a fault. (Relayed from A3; independently
   reproduced here by the contradiction test.)
2. **Both payloads of a `CommitCandidate` must be retired.** A candidate carries `state` AND `output`;
   keeping only `state` and letting the candidate drop trips `RetainedJobPayload`'s Drop assertion
   from inside `step`, which aborts the process. Both the Solve and the Restore branch now retire what
   they do not keep. (Relayed from A3.)
3. **`terminal_is_empty` must report OWNERSHIP only.** Gating it on a local `closing` flag (as the
   assembly inference does) makes any `while !terminal_is_empty()` driver spin, because the framework
   calls `begin_close()` itself at close stage 0. The headless close loop carries an explicit guard
   assert instead. (Relayed from A3.)
4. **One payload page per small write exhausts the operation's page budget.** A page is 16 KiB and an
   operation may hold 256 of them across ALL streams; the first encoder took a fresh page per entropy
   value and refused with `bitmap-inference-output-admission-exceeded` after a handful of cells —
   which also made the tests appear to HANG (they were retrying, not computing). The encoder now fills
   one page to capacity before asking for another, via a peeked `next_encode_chunk`; the commit text is
   still generated incrementally and never materialised as one oversized `String`, so a 256 × 256
   output stays inside the guest's contiguous-allocation ceiling. The whole inference suite went from
   ">60 s and failing" to **0.03 s**.

Also measured: the headless `BatchJobSession` must NOT use `fuel_per_step: 1` (assembly's value) —
that turns one small collapse into tens of thousands of session round trips. This artifact uses
`HEADLESS_FUEL_PER_STEP = 4096`, `HEADLESS_STEP_BUDGET_US = 50_000`, `InteractiveStage::BackgroundStep`.

---

## 5. Windows and scenes

`✏️editor/🎭️modes/✏️edit` declares two window kinds in a `row` 50/50 layout
(`create_default_layout(&[input, output], "row", Some(&[50.0, 50.0]), …)`):

- **`wfc-bitmap-input`** — interactive. Actions: `set-input-pixels`, `resize-input`,
  `add-palette-color`, `change-palette-color`, `remove-palette-color`, `set-active-color`,
  `change-model`, `change-seed`. Per-instance config `BitmapInputWindowConfig { active_color, zoom }`.
- **`wfc-bitmap-output`** — read-only render of the INFERRED bitmap. Actions: `solve`,
  `resize-output`, `pin-pixel`, `unpin-pixel`. Per-instance config
  `BitmapOutputWindowConfig { show_pins, zoom }`.

Every action is `InteractiveJobClassification::Migrated` — an unmigrated verb is dispatch-dead in the
shell, and a window unit test asserts it for both windows.

**Deviation from the patterns report, stated on purpose.** The report's §8 suggests `Paint2d` +
`Paint2dScene` for this artifact. Both windows use **`SurfaceKind::Canvas2d` + `Canvas2dScene`**
instead. `Paint2dScene`'s host contract is a raster DOCUMENT — a layer tree plus base64 PNG assets in
`assetsJson` — which would force this crate to carry a PNG encoder purely to display a
palette-indexed buffer it already holds exactly. The canvas host's own layer record renders a filled
path with an explicit `fill.color`, reproducing palette indices byte-for-byte with no encoder at all.

The shared builder `crate::bitmap_layers_json` lives at the ARTIFACT root so the read-only viewer
never imports through the editor. It emits a background rectangle for the extent, one filled path per
horizontal run of equal indices, and one marker per pin. Because `Canvas2dScene` keeps `layersJson`
in the 32 KiB fixed-capacity spine (it has NO lane of its own — `Canvas2dSceneLane` carries only
`toolRunTrace`), the builder is budget-guarded: `BITMAP_LAYERS_JSON_BUDGET_BYTES = 24 KiB`, and a
sample whose run-merged rectangles would not fit is re-emitted at a coarser block size rather than
silently failing the whole surface encode. A unit test drives a dense 96 × 96 eight-colour sample
through it and asserts the budget holds.

**Gesture law — wired, catalogued and tested.** Three verbs, all registered on the input window as
`Migrated` actions (an unclassified verb is dispatch-dead in the shell, so an unregistered command is
unreachable dead code):

- `stroke-begin { x, y }` starts a bounding box in the PANE'S OWN window config,
- `stroke-extend { x, y }` grows it,
- `stroke-commit` turns the settled box into exactly ONE `set-input-pixels` filled with the pane's
  armed colour, and clears the box in the same emit.

Mid-drag ticks emit ZERO document operations, and every begin/extend config write carries the single
coalesce key `wfc-bitmap-stroke`, so a two-hundred-sample drag is ONE config edit rather than two
hundred — the ledger discipline a per-sample amend would break. The accumulator lives in the window
config rather than the app transient for a structural reason: `ArtifactEditor::handle` is handed a
`ConfigView` but no `TransientView`, so the config is the only lane a command can write on
pointer-down and READ again on release.

`BitmapEditor::stroke_mutation` is the pure half, lifted out so the contract is testable without a
mounted host: `a_drag_of_four_samples_commits_exactly_one_mutation_over_the_union_region` drives
begin → extend → extend → extend and asserts exactly one `set-input-pixels` at `(2, 3)` sized
`4 × 5` — the union of every sample — filled with the armed colour, and that applying it really moves
the sample. A box that leaves the sample, or a colour the palette does not hold, is REFUSED rather
than clipped: clipping would commit an edit the author did not draw.

**Where the inferred output lives.** In the app TRANSIENT (`BitmapTransient { output_pixels,
contradiction, output_width, output_height }`), written by the `Solve` command through
`ArtifactEditor::ephemeral`, read by `render_with_request_context`. It is never a snapshot field and
never an edit. A cached solve whose extent no longer matches the document's output spec is DISCARDED
rather than reshaped — a stale buffer stretched over a new extent would look like a real answer.

**The viewer runs no collapse.** Its output window states the declared output extent and every pinned
cell in its own palette colour — the part of the answer the document itself authors. Rendering a
solve there would mean either a full solve per frame or reading another surface's cache.

---

## 6. Examples, fixtures, oracles

Two examples, both COMPUTED from a closed form rather than transcribed, because a hand-typed
256-byte index buffer is unreadable and undiffable:

- **`🚪️rooms-16`** — 16 × 16, three colours (wall/floor/door), walls on the five-grid with a door at
  every wall segment's midpoint. Output 24 × 24 periodic, `N = 3`, symmetry 8.
- **`🌸️flowers-24`** — 24 × 24, four colours (sky/ground/stem/petal), a ground band with stems on a
  six-cell pitch and a petal crown on each. Output 32 × 24 non-periodic, `N = 3`, symmetry 2,
  `ground = 1`, one pin. It exercises what `rooms-16` does not: the ground constraint, a non-periodic
  input, a narrow symmetry group, and the pin lane.

Each committed `🗣️.dsl.semio` asset is a PRINT of its Rust builder, asserted by a per-example test.

Ten fixture quintets, one per mutation, all emitted by the ignored `emit_committed_fixtures`
generator from one table in `🧪️tests/🔬️unit/🦀️.rs` — a hand-edited fixture is a bug by construction.
The JSON is re-indented WITHOUT reordering keys (serde's pretty printer sorts them, and a fixture
whose key order no longer matches its struct's field order is exactly the drift the canonical-form
tests exist to catch).

`🔮️oracles/🔣️.json` registers one `verified-native-second-implementation` oracle,
`wfc-bitmap-python-independent`, covering capability `wfc-bitmap-1-mutate` over all ten vectors. The
reference is `🧪️tests/🧩️mutate-bitmap-1/🐍️.py`: it links nothing of this repository, writes its own
base64 codec from RFC 4648, computes its OWN inverse, and asserts the footprint law. The third-party
survey is recorded and declined with a concrete reason: published overlapping-model WFC libraries
compute a COLLAPSE from a PNG and carry no problem document, no palette-indexed carrier and no
mutation vocabulary. The SOLVE is deliberately not a vector — it is an inference, covered by the
engine crate's own tests and by this subset's deterministic-seed and contradiction tests; pinning a
seeded collapse into a committed vector would freeze a sampler, not a specification.

**A defect the Python oracle found that the Rust half did not.** `resize-input`'s diff originally
declared only the two extent fields and let `apply` re-derive the pad/crop. The reference's footprint
law reported the pixel buffer as moved but undeclared. The diff now carries the re-laid-out buffer
explicitly, so a reader can reconstruct `after` from the diff alone.

---

## 7. Test inventory (176 native)

- per-mutation: 10 cases × 7 assertions (apply, inverse-restores, canonical JSON ×2, declared
  outcome, produced diff, diff-applies-to-after)
- snapshot / text / binary / diff / mutations / mutations-binary facet units
- inference: roster identity, descriptor leaves, symmetry order (including "2 is a mirror"),
  **deterministic seed**, different-seed validity, **contradiction verdict**, unreachable-pin
  refusal, admission ceiling, prior entropy, `InferredField` answers, and the two
  **local-similarity** laws (both examples; periodic and non-periodic; second seed)
- editor: manifest, boot example, binary op round trip, every mutation kind reachable from a window,
  every typed command dispatches to the mutation it names, every dispatched mutation moves the boot
  example or says why not, **the four gesture tests** (union region, restart, refusals, the three
  stroke verbs catalogued `Migrated`)
- viewer: manifest, dialect, inert command
- windows ×4: window kind shape, Migrated classification, non-empty render for BOTH examples,
  staleness guard, pin overlay
- examples: asset-is-the-print, parse-back, pack round trip, closed-form content, roster, well-formed
  problem, real pattern universe, solve terminates with a verdict and honours its pins
- artifact root: identity, base64 round trip and refusals, layer builder (runs, pins, budget)
- mount contract (Rust) + store fixture

---

## 8. Known gaps and what the plugin root / W2 needs

1. **`verify taxonomy enforce` is red for repo-wide reasons, not this artifact.** Two different
   failures on two runs, both outside `🖼️bitmap`: `Current compiler input manifest compiler inputs are
   not path-sorted`, and `frozen-coordinate-evidence-invalid: …/🧫️fixtures/📐️cad-draw-path-projection/🔣️.json:
   document digest does not match registered bytes`. The same gate fails identically when scoped to
   the untouched `🌀️procedural/🧩️assembly`, so it is not caused by this slice.
2. **Taxonomy rows I added myself.** The `new mutation` scaffolder refuses a directory that already
   carries `🧪️tests`, so the nine new mutation directory names (`📐️resize-input`,
   `🖌️set-input-pixels`, `🎨️add-palette-color`, `🖍️change-palette-color`, `🧽️remove-palette-color`,
   `🖼️resize-output`, `⚙️change-model`, `📌️pin-pixel`, `📍️unpin-pixel`) were appended to
   `semanticDirectoryMemberKinds["members-of-schema"].memberNames` by hand, as a surgical text insert
   that reformats nothing else. `loadTaxonomy()` validates green afterwards. The transient's own
   `👁️set-solve` needs no row (generation3d's `👁️set-generation-preview` has none either).
3. **The plugin root must carry a real `Transient`.** `BitmapEditor` uses
   `Transient = BitmapTransient` / `TransientMutation = BitmapTransientMutation` (`Config` stays
   `NoConfig`; the per-pane state is WINDOW config, registered through
   `register_window_config_owners`). Two owners to know about:
   `input::config::BitmapInputWindowConfigOwner` and `output::config::BitmapOutputWindowConfigOwner`.
4. **Exports the plugin root needs**: `artifact::<WfcApps>()`, `definition()`, `artifact_kind()`,
   `editor::bitmap::{BitmapEditor, create_bitmap_editor}`,
   `viewer::bitmap::{BitmapViewer, create_bitmap_viewer}`,
   `inferences::{bitmap_inference_metadata, register_bitmap_inference_factory, bitmap_artifact_inference_descriptor}`,
   `examples::{sources, example_source_slice}`.
5. **Cargo deps were slimmed.** `semio-framework-os-infinite` was dropped: it does not compile right
   now (`🌍️world/🦀️.rs:291` — `cannot borrow 'producer' as mutable`, a peer's in-progress edit) and
   this artifact does not use Board2d. Final dependency set: `semio-framework`,
   `semio-framework-job`, `semio-framework-os-kernel`, `semio-framework-plugin`,
   `semio-framework-schema`, `semio-framework-value-derive`, `semio-s-plugin-wfc-engine`, `pack`, and
   the optional `semio-framework-ui-contract` behind `component-app-assembly`. No app-plugin crate is
   depended on.
6. **Not done here** (outside this slice's scope): `describe`/plugin-registry regeneration, the
   playground boot and browser probes, and `verify dependencies literal-external`. The artifact
   declares two playground-relevant ids for the root: variant `bitmap`, app
   `s.wfc.bitmap@1/*#editor`, ports 6041/6141.
7. **The two audit findings are closed.** `📓️audit-bitmap.md`'s blocking item (no artifact-level
   local-similarity test) and its dead-code item (`stroke-begin`/`stroke-extend` unregistered) are
   both fixed above, and fixing the first surfaced and fixed the `flowers-24` contradiction. What
   remains open from that audit is item 3 only — demonstrating the pointer-down/drag/up coalescing
   END TO END through a live host, which needs the playground boot W2 owns. The Rust half of that
   contract is now catalogued, classified and unit-tested.
8. **Diff has no text/binary Rust facet.** `🔺️diff/{📝️text,💾️binary}` exist as TS declarations only
   (the barrel expects them); a diff travels as its own record, never as an authored document, so no
   grammar or protocol is registered for it. Stated rather than silently omitted.
