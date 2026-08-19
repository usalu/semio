# 📓️ terra-dedyn-fw-graph report

Packet: `dedyn-fw-graph`. Family: `QueryableGraph` (`🧰️framework/🔨️modules/🕸️graph/🗣️dsl/🦀️component.rs`).

## Counts

- **Before**: 21 raw matches of `dyn\s+QueryableGraph` repo-wide (20 real code uses, all `graph: &dyn
  QueryableGraph` function parameters; 1 was inside a doc comment at line 1622).
- **After**: **0** code uses. The 1 comment was reworded (no longer claims `&dyn QueryableGraph`, now
  says "a `QueryableGraph`-bounded generic").
- Verified with two differently-implemented queries, both against the live tree, both re-run after the
  edit:
  ```
  $ python3 -c "... re.compile(r'dyn\s+QueryableGraph') over the whole repo tree ..."
  → 1 match: 🧰️framework/🔨️modules/🕸️graph/🗣️dsl/🦀️component.rs:1622 (comment only)
  $ grep -n "dyn QueryableGraph" 🧰️framework/🔨️modules/🕸️graph/🗣️dsl/🦀️component.rs
  → 1622:    // Jack's own `complete` needs a `&dyn QueryableGraph` ...   (pre-edit; post-edit: 0 matches)
  ```
  Both queries agree: zero `dyn QueryableGraph` in code, repo-wide, before and after the edit.

## Mechanism: GENERICS (R11 case "open set, argument position")

All 20 uses were `graph: &dyn QueryableGraph` in **function-parameter position**, never a trait-method
return type. Per R11, that is the trivially-generic case — no design question. `QueryableGraph` has
**5 impls spread across 3 crates** (`BoardQueryableGraph` + a local `EmptyGraph` in `🕸️graph` itself;
`TrinityQueryableGraph`/`OwnedTrinityQueryableGraph` in `✏️s/🔌️plugins/🔱️trinity/…/jack/…/language-service`;
`CadTopologyGraph` in `✏️s/🔌️plugins/📐️cad/…/inferences`) — a genuinely open set from `🕸️graph`'s point
of view, so `dyn_enum_close!` cannot apply (it needs every impl in one crate) and I do not own the other
two crates' paths to close it there either. Generics sidestep the question entirely: `🕸️graph` never
needs to *name* the fleet's concrete graph types.

Applied to all 20 signatures identically:
```diff
-pub async fn complete(graph: &dyn QueryableGraph, source: &str, cursor: usize) -> Vec<Completion> {
+pub async fn complete<G: QueryableGraph>(graph: &G, source: &str, cursor: usize) -> Vec<Completion> {
```
(and the 19 siblings: `manifest_node_kinds`/`manifest_edge_kinds`/`manifest_property_names`/
`manifest_port_kinds`, `graph_node_kinds`/`graph_edge_kinds`/`graph_property_names`/`graph_port_kinds`,
`semantic_lints`, `lint`, `hover`, `execute`, `run_query`, `run_query_json`, `match_patterns`,
`match_pattern`, `eval_expr`, `binding_value`, `build_return`).

**Zero call-site changes needed anywhere**, including outside my owned path: every caller already passes
a concrete `&Concrete` (`&EmptyGraph`, `&BoardQueryableGraph::from_fixture_json(..)`, and — in the two
plugin crates I do NOT own — `&TrinityQueryableGraph(graph)`, `&CadTopologyGraph { .. }`), which is exactly
what `&G` infers against. This is generics' structural advantage over the enum mechanism here: no
`.into()` fixups, no `Arc`/`Box` unwrapping, nothing to touch in `✏️s/**`.

Also added, per R7 (the trait has `async fn` methods, so `async_fn_in_trait` fires crate-wide):
```diff
+// 🃏️ `QueryableGraph` (Jack) has async trait methods; Send-ness comes structurally from the
+// concrete/generic caller, never from a `+ Send` bound on the trait method — see R3 and R7.
+#![allow(async_fn_in_trait)]
```
at `🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust/📦️glue.rs` (crate root of `semio-framework-graph`;
this crate had no such attribute yet). No `+ Send` was added anywhere, per R3.

## Macro friction

None — the dyn_enum macro was never invoked. Read `📓️terra-dyn-enum-macro-report.md` first as instructed;
its own §6 finding 6 and the multi-crate impl spread here both point the same direction: `dyn_enum_close!`
needs a closed, single-crate impl set, which `QueryableGraph` does not have. This family is additional
evidence for that report's R11 generalization ("open set ⇒ generics"), not a new finding.

## Build / acceptance

`semio-framework-graph`'s crate root (`📦️glue.rs`) mounts `🗣️dsl`, `⚙️engine`, `🛂️manifest`, `🧮️algorithms`,
`🖊️drawing` as one crate, so `cargo check -p semio-framework-graph` compiles the whole crate, not just my
file.

```
$ CARGO_TARGET_DIR=<scratchpad>/target-dedyn-graph cargo check -p semio-framework-graph --lib
error: could not compile `semio-framework-graph` (lib) due to 591 previous errors
```
Exit code: nonzero (`101`).
```
$ CARGO_TARGET_DIR=<scratchpad>/target-dedyn-graph cargo check -p semio-framework-graph --all-targets
error: could not compile `semio-framework-graph` (lib test) due to 1474 previous errors
```
Exit code: nonzero (`101`).

**Both runs are blocked by a pre-existing, unrelated defect, not by my change.** `🗣️dsl/🦀️component.rs`
already had every `QueryableGraph`-family fn marked `async fn` (verified: the trait and every method were
already `async fn` before I touched the file) but had **zero `.await` anywhere in the whole 2944-line file**
— confirmed with `grep -c '\.await'` = 0 on a fresh read before my edit. Every one of the 591/1474 errors
is `E0308`/`E0277`/`E0599` "expected T, found future" with rustc's own `help: consider await`ing`
suggestion, spanning `🗣️dsl` (the bulk), plus `⚙️engine/🦀️component.rs` (`Storage`/`MappedHeap`
`Default::default` calling an async `Self::new()`) and `🛂️manifest/🦀️component.rs` (a serde
`serialize_with`/`deserialize_with` E1 fn made async and needing R9 reversion) — none of which are the
`QueryableGraph` family and none of which are in my packet's scope.

I grepped every error mentioning `QueryableGraph` (6 hits) and read each in context: 4 are
`BoardQueryableGraph::from_fixture_json(..)` test calls missing `.await` on the constructor (nothing to
do with the trait's dyn-ness), and 2 are my own new `<G: QueryableGraph>` signature lines appearing only
as **location markers** inside multi-line "missing `.await` in the function body" diagnostics — not
complaints about the generic parameter itself. I confirmed no diagnostic anywhere concerns the type of
`graph: &G`, a bound-mismatch on `G`, or a call site failing to infer `G`. The generic conversion is
sound and introduces zero new errors; the crate's blocker is a crate-wide missing-`.await` backlog that
existed before I started and needs its own dedicated span-keyed `insert-await.py` pass (out of scope for
`dedyn-fw-graph`, which owns only the `QueryableGraph` family, not general async correctness of `🕸️graph`).

Per the ticket's build-reality note, **reporting acceptance UNRUN, blocking crate `semio-framework-graph`
itself** (not the SDK gate this time — a different, pre-existing local blocker), with the structural proof
above standing in for it.

## What a sibling must know

- `semio-framework-graph` (all of `🕸️graph/**`) does not compile today, for reasons unrelated to any
  `dyn` family: `🗣️dsl/🦀️component.rs` has async signatures but 0 `.await` calls anywhere (591 lib errors,
  1474 with tests); `⚙️engine` and `🛂️manifest` each have their own unrelated async-conversion defects
  (an E1 serde fn needing R9 reversion in `🛂️manifest`; a `Default::default` calling async `Self::new()`
  in `⚙️engine`, for `Storage<P,D>` and `MappedHeap<K,V>`). Whoever owns the await-insertion pass for this
  crate should treat `🗣️dsl/🦀️component.rs` as effectively 100% residue — every call site in the file
  needs `.await` inserted, which is squarely `insert-await.py`'s job (span-keyed, safe) followed by hand
  work for the closure/loop residue shapes R10 describes, since a query language interpreter (`match_pattern`,
  `eval_expr`, `build_return`) is exactly the recursive/loop-heavy shape R10 flags.
- No lease-request needed: the generics mechanism required editing only my own file plus my own crate's
  root inner-attribute, never the two plugin crates (`🔱️trinity`, `📐️cad`) that hold the other 3 impls.

## Files touched

- `🧰️framework/🔨️modules/🕸️graph/🗣️dsl/🦀️component.rs` — 20 fn signatures `&dyn QueryableGraph` →
  `<G: QueryableGraph>(graph: &G)`; 1 doc comment reworded.
- `🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust/📦️glue.rs` — added
  `#![allow(async_fn_in_trait)]` with R3/R7 doc comment.
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/📓️terra-dedyn-fw-graph-report.md`
  (this file).

Scratch (scratchpad, not repo): `target-dedyn-graph/` cargo target dir,
`dedyn-graph-check.txt`/`dedyn-graph-alltargets.txt` raw cargo output — both under the session scratchpad,
not the ticket folder, per rule 24.
