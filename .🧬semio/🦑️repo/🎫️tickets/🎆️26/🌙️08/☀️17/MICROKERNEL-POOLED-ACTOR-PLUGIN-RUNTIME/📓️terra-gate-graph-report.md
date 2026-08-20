# 📓️ terra-gate-graph — `semio-framework-graph` green

## Verified exit codes (all pasted from real runs, `CARGO_TARGET_DIR` = ticket scratch target)

```
semio-framework-graph      --lib          EXIT 0   (terra-gategraph-REVERIFY-lib.txt)
semio-framework-graph      --all-targets  EXIT 0   (terra-gategraph-REVERIFY-alltargets.txt)
semio-framework-graph      cargo test     EXIT 0, 174 passed / 0 failed / 0 ignored (terra-gategraph-test-FINAL.txt)
semio-framework-os-kernel  --lib          EXIT 0   (terra-gategraph-REVERIFY-oskernel.txt)
semio-framework            --lib          EXIT 0   (terra-gategraph-REVERIFY-framework.txt)
```

Named test set (`cargo test -p semio-framework-graph`, one binary `semio_framework_graph-*`): 174/0/0 across
`⚙️engine::tests` (+ nested `engine::tests::quick`), `🗣️dsl::wire::tests`, `🗣️dsl::tests` (both blocks),
`🧮️algorithms::tests`, `🖊️drawing::force::tests`, `🖊️drawing::routing::tests`, `🛂️manifest::tests`. Doc-tests: 0/0.

## Start → end

576 baseline errors (`--lib`) → 0. `--all-targets` (not run by any prior packet on this crate) started at
**723** once `--lib` went green and surfaced the test modules; also 0 now.

## Mechanism per fix family (R9 both-halves shown where used)

1. **Bulk `.await` insertion** — `insert-await.py --scope 🧰️framework/🔨️modules/🕸️graph`, run to fixpoint
   after every hand-fix (7 total passes across `--lib` and `--all-targets`, ~600 mechanical edits). Always
   run *after* confirming signatures were already `async fn` (this crate's codemod pass was already done;
   no `asyncify-universal.py` needed).
2. **Struct-literal shorthand `.await`** (R10 shape #7) — `🗣️dsl/component.rs`: `QueryableEdge { ..., properties.await }`
   from `.map(json_to_property_bag).unwrap_or_default()` (an async fn used as a `.map()` value, shape #4).
   Fixed at the constructor: `match obj.get(...) { Some(v) => json_to_property_bag(v).await, None => default() }`.
3. **Self-recursive async fns → `Box::pin`** (E0733, shape #3) — `UnionFind::find`, `strongconnect`,
   `MappedHeap`... no — `algorithms::dfs` (cycle-finder), `dfs_blocking_flow` (max-flow), `collect_expr_vars`,
   `eval_expr` (And/Or), `property_value_to_dsl_value`/`dsl_value_to_property_value` (Array/Object recursion,
   duplicated in both `manifest` and `dsl`).
4. **`.await` inside a sync closure** (shape #1) — rewritten as `for` loops with the async call awaited
   per-iteration. Recurring in every `GraphView` wrapper (`SubgraphView`/`EdgeSubgraphView`/`FilteredView`/
   `UndirectedView`/`ReversedView`'s `out_degree`/`in_degree`/`edges()`/`out_neighbors()`), `Storage::edges()`,
   `tokenize`, `complete`, `return_items_want_graph`, `IdIndex::edges_to_indices`, `circular_layout`/`grid_layout`.
5. **Awaiting one future repeatedly** (shape #2, E0382) — `drawing::add_pairwise_repulsion`'s `f.await` used
   twice; `Interner::from_labels`'s `interner.await.intern(...)` in a loop; `connected_components`'s
   `uf.await.union/.find` reused. Fixed by hoisting to a single `.await` at the binding.
6. **Constructor left un-awaited, then every use `x.await.method()`** (shape #6) — this was the dominant
   pattern in `⚙️engine`'s test module: `let mut g = NU::new(); g.await.add_node();` repeated ~150 times
   across ~40 tests (`Storage`, `Csr`, `MappedHeap`, `Interner`, `FlowNetwork`). Fixed by hand, function by
   function: await the constructor once, await each async method call at its own call site, drop the
   now-unneeded `.await` on the already-resolved bindings.
7. **R9 (E1) de-asyncified with tag**: `PropertyValue::as_str`/`as_f64` (fn-pointer into `Option::and_then`
   at every call site, no consumer ever awaits them) · `deserialize_value_type`/`serialize_value_type`/
   `parse_value_type_value` (serde `deserialize_with`/`serialize_with` hooks, called synchronously from
   derive-generated `Deserialize`/`Serialize`, both externally-declared and sync) · `Storage::default`/
   `MappedHeap::default` (`impl Default`, external trait, mirrors `new()`'s literal I/O-free body instead
   of calling it) · `PropertyValue::shape()` (`DslField::shape` is itself E4-tagged sync in the trait
   definition, `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/component.rs`) · `build.rs`'s `note_newest`/
   `watch_manifest_sources` (E3 — `fn main` in a build script has no executor, transitively forces its
   helpers sync). Both halves shown in each tag's inline comment.
8. **E4 fn-pointer slot** — `dsl::idiom_hooks`/`idiom_canonicalize`/`idiom_classify`/`idiom_complete`
   (`dsl_core::IdiomHooks` fields are plain `fn` pointers). Since their bodies call genuinely-async
   `format`/`tokenize`/`complete`, added the crate's one **E5 executor bridge** `resolve_ready()` (single
   poll, panics if `Pending` — sound because this crate's tokenizer/parser does zero I/O).
9. **`#[test] async fn` — R4 clause 5, the dominant `--all-targets` class (174 of 517 residual errors)** —
   `#[test]` cannot run an async fn directly and this crate has no `#[async_test]` macro dependency
   (registrar-only `Cargo.toml`, out of scope to add). Added one `block_on_test()` helper per `mod tests`
   block (7 insertions across the 5 files) — same single-poll bridge as `resolve_ready`, tagged E5/R4-cl.5
   — and wrapped every async test body: `fn NAME() { block_on_test(async { BODY }) }`. Tests with zero
   `.await` in the body just dropped `async`. Built and ran a structural (attribute+signature-anchored,
   never name-keyed) repair script for this — `terra-fix-async-test-fns.py`, saved in the ticket folder —
   verified every rewrite by full recompile + `cargo test`, and hand-fixed the one file (`🗣️dsl`) where a
   `    }` inside a raw-string JSON fixture fooled the indentation-based brace matcher into ending two
   function bodies early (structure stayed valid — only the `has_await` scan was affected — repaired by
   hand-wrapping the true bodies of `run_dag_fixture_query`/`run_port_filtered_query`).

## 🚨️ Dropped-future silent no-ops found (the "highest-value" class per this ticket's own header)

All confirmed via the compiler's own `unused_must_use` / "futures do nothing" warning (23 real sites,
zero false positives — this is the diagnostic-driven signal, not a name-keyed guess) plus 2 more found by
inspection once the shape was known:

- **`Storage::add_edge_with`**: `self.link_adjacency(...)` was called without `.await`. Every edge ever
  inserted through this path was silently **absent from `successors`/`predecessors`** — neighbor queries,
  degree counts, and traversal would all have missed it, while `self.edges` itself looked populated.
- **`Storage::remove_edge`**: `self.unlink_adjacency(...)` likewise dropped — a "removed" edge's adjacency
  entries were **never actually cleared**.
- **`Storage::unlink_adjacency`** itself: all **four** of its own internal `unlink_one(...)` branches were
  *also* dropped — so even after fixing the outer call site, the inner mutator still did nothing until this
  was fixed too. Two-layer instance of the same bug.
- **`Storage::remove_node`**: `self.remove_edge(edge_id)` over incident edges dropped — removing a node
  never actually removed its incident edges.
- **`MappedHeap`**: `sift_up`/`sift_down`/`swap` were dropped at **every** call site, including sift_up/
  sift_down calling their own `swap` internally. The whole binary-heap invariant was never maintained —
  every `push_or_decrease`/`decrease_key`/`pop_min` silently left the backing `Vec` in insertion order.
  This type backs Dijkstra-shaped priority-queue algorithms in this crate.
- **`drawing::run_force_layout`**: `add_pairwise_repulsion(...)` dropped — the repulsion half of the
  force-directed layout never ran (only the spring/gravity terms did).
- **`FlowNetwork::max_flow`**: `self.bfs_levels(source)` dropped inside the Dinic's-algorithm outer loop —
  the level graph was never built, so `dfs_blocking_flow` — *also* called without `.await` — never
  actually pushed flow. Both fixed together; verified against the CLRS textbook network test (max flow 23,
  min-cut duality) which now passes for real rather than vacuously.
- **`build.rs`**: `note_newest`/`watch_manifest_sources` were `async fn`, called without `.await` from
  `fn main`. Their whole job — tracking the newest manifest-source mtime to decide whether the generated
  registry is stale — silently never ran; `stale` could only ever become `true` via the *missing-file*
  branch, never the *stale-but-present* branch. Fixed at the root (R9/E3: de-asyncified both, since `main`
  is a hard-sync entry point with no executor in a build script).
- **`dsl::render_wire_line`**: `Writer::new()` unawaited (so `writer` was a future, not a `Writer`) *and*
  `print_shape(...)` — a `()`-returning call whose entire job is to mutate `writer` — dropped outright.
  Every wire-literal render silently produced nothing until fixed.

## `lease-request`

None. `QueryableGraph`'s cross-crate open-set nature (5 impls across `graph`/`cad`/`procedural`) never
required a fleet-side edit for this packet's scope — the trait itself was already correctly AFIT
(`async fn` in trait, R7-compliant) and needed no `dyn`/enum changes; only call-site `.await` and one E4
fn-pointer bridge, both inside `🕸️graph`.

## Exit codes (paste, for the record)

```
$ CARGO_TARGET_DIR=<scratch>/target-gate-graph cargo check -p semio-framework-graph --lib
Finished ... EXIT 0

$ CARGO_TARGET_DIR=<scratch>/target-gate-graph cargo check -p semio-framework-graph --all-targets
Finished ... EXIT 0

$ CARGO_TARGET_DIR=<scratch>/target-gate-graph cargo test -p semio-framework-graph
test result: ok. 174 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
   Doc-tests: test result: ok. 0 passed; 0 failed
EXIT 0

$ CARGO_TARGET_DIR=<scratch>/target-gate-graph cargo check -p semio-framework-os-kernel --lib
Finished ... EXIT 0

$ CARGO_TARGET_DIR=<scratch>/target-gate-graph cargo check -p semio-framework --lib
Finished ... EXIT 0
```

## Files touched (all under owned path `🧰️framework/🔨️modules/🕸️graph/**`)

- `⚙️engine/🦀️component.rs`
- `🗣️dsl/🦀️component.rs`
- `🧮️algorithms/🦀️component.rs`
- `🖊️drawing/🦀️component.rs`
- `🛂️manifest/🦀️component.rs`
- `📦️packages/🦀️rust/build.rs`

No `🤖️generated/**` files touched. No files outside `🕸️graph/**` touched (`git diff --stat HEAD` confirms).

## Tools used / added to the ticket folder

- `insert-await.py` (existing, run 7×) — `terra-gategraph-insertaway-*-report*.json` are its reports.
- `terra-fix-async-test-fns.py` (new, saved this packet) — structural `#[test] async fn` → sync
  `#[test] fn { block_on_test(async { .. }) }` rewriter, attribute+signature-anchored, never name-keyed;
  verified every output by full recompile. Documents the one known limitation (raw-string brace collision)
  in its own docstring-equivalent comment at the top.
- `find-dropped-void-futures.py` (scratchpad only, not copied into the ticket folder — pure `()`-return
  name-collision scanner used once for triage, superseded by the compiler's own `unused_must_use` warning
  which is exact).

## Not done / explicitly out of scope

- Did not touch `🏪️store`, `🗣️dsl/🧬️schema`, `💡️inference` (peer-active) — only *read* their type
  definitions (`Writer`, `IdiomHooks`, `DslField`) to know correct call-site signatures.
- Did not touch `neural_engine` (`🧠️neural/⚙️engine`, a different `💻️os` module) — only read `ValueType::id`/
  `matches` to confirm they're sync (no edit needed there).
