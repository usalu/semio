# framework/product/os — DocumentDsl + OpText scratch notes

## Source of truth read
- `vcs/rs/lib.rs` regions `🔖Text`, `🔖TextFormat`, `🔖TestSupport` (test_support module).
- `framework/product/os/core/rs/lib.rs`: `OsProjection` (host module, ~line 425), `OsOperation` (14 variants,
  ~line 441), `OsDiff`/`OperationDiff<OsProjection>` (~line 610-691), `Operation<OsProjection> for OsOperation`
  (~line 693-756, `reconcile` overridden -> `reconcile_os_media_graph`, unrelated to Dsl/OpText).
- `crate::instance` module: `OsAppInstance`, `OsDocumentRef`, `OsParameterFieldBinding`, `OsParameter`
  (tagged enum Numeric/Categorical/Toggle/Text), `OsParameterType`.
- `crate::media_graph` module: `OsMediaGraph{schema,nodes,edges}`, `OsMediaGraphNode{id,instance_id,x,y,
  width,height,inputs,outputs}`, `OsMediaPort{id,resource_kind,direction}`, `OsMediaGraphEdge{id,
  source_node_id,source_port_id,target_node_id,target_port_id,contract}`, `MediaContract{kind_id,
  media_type,wire,conversion}`, `MediaGraphPosition{x,y}`.
- `framework/core/rs/lib.rs`: `MediaClass`, `MediaForm`, `MediaType{class,form}`, `MediaWireFormat::
  {Binary{format},Document{schema}}`, `OsMediaFormat` (has `as_str()`/`parse()` helpers reused directly,
  not reimplemented).

## Example/initial-projection construction (step 1 finding)
Searched for `include_str!`, `.example(`, `Default for OsProjection` in `framework/product/os/**` — none
found. The only construction is `default_os_projection()` (host module) which is 100% programmatic
(`OsProjection { programs: Vec::new(), ... }` struct literal), not JSON/`include_str!`-backed. No fixture
file or serialization call exists to retarget onto `parse_dsl`/`print_dsl` — step 5 of the assignment is
therefore a no-op here. The `framework/product/os/core/fixtures/*.json` files (`no-dirty.json`,
`diamond.json`, `chain.json`, `dead-end.json`, `single-edge.json`) are unrelated DAG cycle-detection test
fixtures (see `🔬 Shared fixtures replay` doc comment near line ~3994 pre-edit) — not OsProjection examples.

## Grammar design
Hand-rolled lexer: `Tok::Word`/`Tok::Str`/`Tok::Eof` only (no braces/commas as separate tokens — commas
inside a bareword or quoted string are just literal characters, split at the semantic level). Reused the
`sequence_text`-style `key=value` collapsing: an unquoted value collapses into the same Word token as its
key; a quoted value is a separate `Str` token immediately following `key=`. This let all record types
(header/instance/node/port/edge/param/binding) share one `parse_kv_map`.

DSL doc layout (`.os` extension):
```
studio programs="puzzle,draw" mediaGraphSchema=s.media-graph activeProgram=puzzle activeAlternative=alt-1
instance id=app-1 program=puzzle app=puzzle2d label="Puzzle Board" yields=puzzle.2d.fixture documentId=doc-1 documentSchema=puzzle.2d.fixture
node id=node-1 instance=app-1 x=0 y=0 w=220 h=92
  out id=app-1:puzzle.out:out kind=puzzle.2d.fixture dir=out
edge id=edge-1 sourceNode=node-1 sourcePort=app-1:puzzle.out:out targetNode=node-2 targetPort=app-2:draw.in:in kindId=puzzle.2d.fixture class=twoD form=vector wireKind=document wireSchema="puzzle.2d.fixture"
param type=numeric id=p1 name="Zoom" value=10 min=0 max=100 step=1
binding parameter=p1 instance=app-1 field="/zoom"
```
Indentation on `in`/`out` port lines is cosmetic only (mirrors `vcs::print_document_text`'s op-log framing)
— the parser reads them as zero-or-more `in`/`out` keyword records immediately following a `node` header,
regardless of whitespace. Whitespace/newlines are never significant, matching every other technology's DSL.

Op-text (single line, one keyword per `OsOperation` variant, 14 total):
`set-active-program`, `set-active-alternative`, `spawn-app-instance`, `remove-app-instance`,
`connect-media-ports`, `disconnect-media-edge`, `move-media-node`, `patch-app-instance`, `add-parameter`,
`remove-parameter`, `patch-parameter` (uses `target=` for the `parameter_id` field to avoid colliding with
the embedded parameter's own `id=`), `bind-parameter-field`, `unbind-parameter-field`, `sync-parameter-ports`.
`spawn-app-instance`/`connect-media-ports`/`add-parameter`/`patch-parameter` reuse the exact same
`print_instance_fields`/`print_edge_fields`/`print_parameter_fields` helpers as the doc-level records.

Percent-escaping (`%25`/`%2C`, mirroring `vcs`'s private `escape_id_component`) is used for comma-joined
list fields (`programs`, categorical `options`) so an item containing a literal comma round-trips.
Quoted-string fields always print quoted even when empty (`options=""`) since an empty bareword right
after `=` isn't representable as a token — the parser doesn't care whether a field is quoted or bareword,
quoting is purely the printer's choice.

## Files touched (all under framework/product/os/)
- `framework/product/os/core/rs/lib.rs`:
  - `//#region 🔖Dsl` + `//#region 🔖OpText` inserted after `//#region 🔖GraphReconcile`, before
    `materialize_os_projection` (host module).
  - Extended the host module's existing `#[cfg(test)] mod tests` with a new `// #region 🔖DslAndOpText`
    (DSL round trip x2, one op-text round-trip test per `OsOperation` variant x14, one
    `assert_document_text_round_trip` test over a `DocumentVcsStore` with one applied operation).
  - Test module imports extended with `MediaContract`, `OsMediaPort`, `MediaClass`, `MediaForm`,
    `MediaType`, `MediaWireFormat`, `OsMediaFormat`.

No other file under `framework/product/os/` needed changes (no JS/TS mirror references `OsProjection`
JSON directly that needed retargeting — confirmed no `include_str!`/fixture file existed to begin with).
