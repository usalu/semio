# trinity — Wave 4 DSL + OpText notes

## Scope
- Jack graph app: `trinity/ram/rs/lib.rs` — `GraphFixture` (DocumentDsl, ext `trinity`), `TrinityGraphOperation` (OpText).
- Rewrite app: `trinity/rewrite/engine/rs/lib.rs` — `RewriteRuleState` (DocumentDsl, ext `rewrite`), `RewriteRuleOperation` (OpText).

## Grammar summary
- `.trinity` doc: header lines `manifest <id|->`, `name "<text>"`, `camera x y zoom`, optional `root <id>`,
  then `node <id>:<kind> "<name>" x y w h {props} [port:kind:dir{props}, ...]` and
  `edge <id>:<kind> <from>:<kind>@<port>-><to>:<kind>@<port> {props}` lines (edge connector syntax reuses
  `mathematical_graph_dsl::wire`'s `id:Kind@port->id2:Kind2@port2` style verbatim; node line is new since
  WireNode has no geometry/name/multi-port).
- Property values: `'str'`, bare number/bool/null, `{k: v}`, `[v, v]` — same as `wire::property_value_literal`,
  but `parse_property_value` ALSO parses nested `{...}`/`[...]` back (wire's own `parse_value` only reads
  scalars — real gap found when nakagin's `position: {x,y,z}` needed round-tripping).
- Op-text: one keyword per `TrinityGraphOperation` variant, e.g. `createNode id:kind x y w h [ports] "name"`,
  `createEdge id:kind src->tgt {props}`, `setDataProperty node:id key value`, `setFixture "<escaped whole dsl doc>"`.
- `.rewrite` doc: `before "<escaped json>"` / `lhs "<...>"` / `rhs "<...>"` (RewriteRuleState's own `_json`
  fields keep their JSON contract — only line framing is new), then `binding <key> <value>` /
  `layout <key> <x> <y>` lines. Reuses `trinity_ram::print_property_value`/`parse_property_value_line`
  (made `pub`) instead of a second value-literal parser.

## Real bug found + fixed
Trinity ids are UUIDs that can start with a digit (`7dc5b737-...`). First lexer draft dispatched purely on
first-byte class (digit → number-only scan), which broke on such ids. Fixed by scanning the maximal
ident-or-number run first (shared charset) then classifying via `looks_like_number` (all-digits-and-≤1-dot).
Caught by `dsl_round_trip_nakagin_fixture` test failing before the fix.

## Fixture conversion
- `trinity/example/nakagin-capsule-tower.trinity.json` → `.trinity` (hand-transcribed from JSON, not
  generated via print_dsl since cold cargo compiles here are extremely slow — repo has ~15+ other Wave-4
  agents compiling concurrently, matches "Concurrent Cargo Workspace Churn" pattern).
- `trinity/example/branch-chain.trinity.json` → `.trinity` (hand-transcribed).
- Old `.trinity.json` files deleted.
- Updated all call sites: `trinity/rewrite/engine/rs/lib.rs` (~1220 wasm ctor, ~1425 `nakagin_graph()`,
  ~1500 `rewrite_labeled_fixture_reloads`), `trinity/plugin/rs/lib.rs` (app_jack consts/`default_fixture`/
  `fixture_dsl_for_preset`/`setActiveExample`/`.example("nakagin",...)`; app_rewrite `nakagin_fixture_json()`
  helper (parses DSL once, `.to_json()` for the `_json` fields), `default_rule_state`, lhs/rhs fixture
  fallbacks, `render_rule_graph` fallback, `.example("label-core",...)` → `print_dsl()`).
- Bonus: `trinity/jack/shell/rs/bin.rs` (a CLI shell, not in the original call-site list but hardcodes the
  same default path) updated too + added `vcs` dependency to its Cargo.toml (needed for `DocumentDsl`/`TextError`).

## Test status (CARGO_TARGET_DIR now `/private/tmp/claude-501-trinity-wave4-cargo-target`, renamed from a
scratchpad path partway through per a sibling-agent infra tip about generic target-dir collisions)
- trinity_ram: 24/24 passed.
- trinity_rewrite: first attempt hit a wall of `signal: 9, SIGKILL` compile failures (libm, core-foundation,
  citationberg, url, rand_chacha, wasmparser, syntect, enum-ordinalize, ciborium, toml) — host-wide OOM from
  many concurrent sibling cargo builds, not a code error. Retrying with `--jobs 3` to cut peak memory, target
  dir renamed+moved (cache preserved) to `/private/tmp/claude-501-trinity-wave4-cargo-target`.
- trinity-plugin (+ wasm32 check): pending.

## Blockers so far
Host-wide concurrent-build memory pressure (SIGKILL'd rustc jobs), being retried with reduced parallelism.
No code-level blockers.
