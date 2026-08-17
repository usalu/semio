# protocol — Wave 3 DSL/OpText notes

## Types (protocol/rs/lib.rs)
- `ProtocolSpec { schema, id, version, title: Option<String>, steps: Vec<ProtocolStep> }`
- `ProtocolStep { id, title, description: Option<String>, blocks: Vec<ProtocolBlock> }`
- `ProtocolBlock` — 19 fields: id, label, kind (required) + description/required/placeholder/default(Value)/
  min/max/step/unit/text/options(Vec<ProtocolBlockOption>)/fields(Vec<ProtocolVectorField>)/schema/src/accept/
  fixture_slug/params(Value)/condition(ProtocolExpr) all Option.
- `ProtocolExpr` recursive: Const{value:Value}, Var{name}, Eq{left,right: Box}, And{items}, Or{items}, Truthy{expr:Box}
- `ProtocolOperation` 9 variants: AddStep, RemoveStep, MoveStep, AddBlock, RemoveBlock, MoveBlock, UpdateBlock,
  UpdateStep, UpdateProtocol.

## Fixture/example investigation
- `protocol/plugin/rs/lib.rs` `ProtocolPlayApp::initial_projection` calls `empty_protocol_projection()` directly
  (hand-built Rust, not JSON). Grepped repo-wide for `.example(` — protocol/plugin has NO `.example(...)` call at
  all. So there is nothing to retarget to `print_dsl()` here — already programmatic, no JSON fixture exists.
- `protocol/module/procedural/rs/lib.rs` defines its OWN `ModuleRenderPayload`/`ModulePayloadOperation` for a
  DIFFERENT document (`protocol.module.procedural.payload`, a transient FlowFixture render/params payload) —
  entirely separate from `ProtocolSpec`/`ProtocolOperation`. It does NOT define extra `ProtocolOperation`
  variants; it only contributes a block KIND (`buildingComponent`) via `Contribution::ProtocolBlockKind`
  (label/icon/default_value_json/params_body_key/preview_body_key) — no op-text handling needed there for this
  ticket. `HEX_COLUMN_FIXTURE_JSON` in that file is a `FlowFixture` JSON (flow_core), unrelated to ProtocolSpec.
  Left untouched (out of scope: different technology's fixture format).

## DSL design (`.protocol` extension)
Hand-rolled recursive-descent, `key=value` header fields + `{ }` bodies, modeled closely on `note/plugin`'s
`note_text` module (same Tok/Lexed/Parser/quote/wrap_body shape).

```
protocol id="recipe" schema="protocol.program" version="1" title="Recipe"
steps {
  step id="s1" title="Basics" description="First step" {
    block kind=text id="b1" label="Label b1"
    block kind=slider id="b-full" label="Team Size" description="How many people?" required=true
      placeholder="Enter a number" default="5" min=1 max=50 step=1 unit="people" text="Some note text\nwith a newline"
      schema="solid.step" src="https://example.com/img.png" accept="image/*" fixtureSlug="hexagonal-mushroom-column"
      params="{\"height\":6.0,...}" {
      options { opt value="red" label="Red" opt value="blue" label="Blue" }
      fields { field key="x" label="X" value=1.5 field key="y" }
      condition and { truthy { var name="show-team-size" } eq { var name="mode" const value="\"advanced\"" } or { var name="a" var name="b" } }
    }
  }
  step id="s2" title="Review" { }
}
```
- `default`/`params`/`Const.value` (arbitrary `serde_json::Value`) are embedded as a quoted field whose string
  content is compact JSON text (serde_json used only to encode/decode this one leaf — no new crate dependency,
  the surrounding grammar is entirely hand-rolled, same pattern the mechanism ticket's own escaping uses).
- Optional `Vec` fields (`options`/`fields`) use PRESENCE of the `options {}`/`fields {}` keyword in the block
  body to mean `Some(vec)` (even `Some(vec![])` via an empty `{ }`); total absence means `None`. Same idea for
  `condition`.
- Op-text keywords: `add-step`/`remove-step`/`move-step`/`add-block`/`remove-block`/`move-block`/`update-block`/
  `update-step`/`update-protocol` — the four that carry a whole `step`/`block` reprint the same
  `print_step`/`print_block` grammar compactly (pretty=false) after their own `key=value` op fields.

## Verification
- `cargo test -p protocol --lib`: 21/21 passed (0 failed) — all pre-existing tests plus the 12 new
  DSL/OpText tests (empty + sample_spec DSL round trip, one op round trip per of the 9 `ProtocolOperation`
  variants, document-text round trip on a `DocumentVcsStore` with 3 applied operations).
- `cargo test -p protocol-plugin --lib`: pending (running).
- `cargo check -p protocol-plugin --target wasm32-unknown-unknown`: pending (running).
(Repo has heavy concurrent build activity from other Wave 3 sessions per CLAUDE.md/memory notes — first
protocol run briefly blocked on the shared target/ dir lock, then completed in 7m41s once free.)
