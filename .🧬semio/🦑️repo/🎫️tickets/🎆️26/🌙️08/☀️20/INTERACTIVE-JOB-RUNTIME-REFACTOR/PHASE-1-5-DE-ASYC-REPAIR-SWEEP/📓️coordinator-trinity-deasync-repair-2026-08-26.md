# Trinity Genuine-Suspension Repair

Date: 2026-08-26  
Scope: `✏️s/🔌️plugins/🔱️trinity/**/*.rs` only.  
Verdict: verification in progress; no Phase 1.5 or Trinity cohort closure is claimed by this checkpoint.

## Trigger

The focused Writer build reached Trinity and reported 1,212 value-vs-future diagnostics before it could compile Writer. The affected Trinity declarations followed one bounded shape: synchronous command, configuration, projection, and trait implementations were declared `async fn` even though their bodies contained no suspension point, while the shared `app_commands!` dispatcher and `InteractiveJob::step` contracts are synchronous.

## Applied classification

The live source began with 1,121 `async fn` declarations. A bounded declaration-only rewrite removed `async` from 801 non-suspending declarations in 160 Rust files. It preserved every function immediately governed by `#[semio_framework_async_macros::async_test]` and preserved the two production constructors whose bodies contain genuine `.await` expressions.

The post-rewrite source census is:

| Class | Count |
| --- | ---: |
| All remaining `async fn` declarations | 320 |
| `async_test` entrypoints | 318 |
| Genuine suspending production constructors | 2 |
| Unexpected remaining declarations | 0 |
| Remaining `.await` expressions | 3 |

The two retained production declarations are:

- Jack Wasm `JackArtifactVcs::new`, which awaits `VcsArtifactApp::new`;
- Trinity Rewrite Wasm `TrinityRewriteArtifactVcs::new`, which awaits `VcsArtifactApp::new`.

The third `.await` is inside the nested `async move` future returned by canvas GPU attachment; its enclosing exported function is synchronous because it returns the Promise rather than suspending itself.

This packet deliberately does not generalize the declaration rewrite beyond Trinity. Phase 1.5's repository-wide compiler/span-keyed discipline remains required for ambiguous shapes and same-named callees.

## Verification ledger

| Gate | Current result |
| --- | --- |
| Exact source classification | GREEN: 320 = 318 async tests + 2 genuine constructors; three `.await` sites inspected |
| Native Trinity library compile | BLOCKED upstream: a completely cold Stdio library target reports exactly 2,299 `E0053`/`E0277`/`E0728` diagnostics before reaching Trinity |
| Native Trinity representative tests | PENDING |
| `wasm32-unknown-unknown` Trinity check | PENDING |
| Writer retained `setLocale` runtime tests | PENDING on Trinity compile |
| Writer descriptor regeneration | PENDING on Writer tests |
| Fresh interactivity/action gates | PENDING on source quiescence |

No passing result will be recorded until the command has completed against the current shared tree.

The initial `cargo test --lib --no-run` was intentionally interrupted after it had emitted current dependency type metadata but spent 44 additional minutes code-generating the unrelated 966 MiB Stdio rlib. The replacement `cargo check --lib` uses the same ticket-local target and is the correct compiler-diagnostic gate for this repair; executable representative tests remain a separate required row above.

## Semantic UI and fresh classifier checkpoint

The Jack editor's main text, graph, and results windows now publish semantic TextEditor, NodeGraph, and Table surfaces through fixed-capacity `BuiltNode` assembly. The Rewrite Jack window publishes a semantic TextEditor surface. Jack and Rewrite Artifact, Catalogue, and Inspection panels use fixed-capacity semantic panel/tree construction. Viewer windows publish semantic scene surfaces, and Artifact editor/viewer implementations convert `BuiltNode` to `ComponentTree` only at their framework boundary.

Fresh production/test-aware classification after formatting:

```text
[DEBUG] immediate classifier: production=0 test=318 suspending-or-excepted=2 files=0
```

The three inspected `.await` expressions remain the two genuine Wasm constructors plus the nested canvas GPU future described above. A fresh legacy scan found no `build_node_graph_scene`, `build_table_scene`, `build_text_editor_scene`, `ui_declarative_sections_to_tree`, or `UiNode` return boundary. `cargo fmt -p semio-s-plugin-trinity -- --check` and `git diff --check -- ✏️s/🔌️plugins/🔱️trinity` complete without output.
