# P7b Puzzle2d Fifth Independent Source/Static Acceptance Audit

Date: 2026-08-25
Auditor: Codex independent read-only source/static audit
Verdict: **GREEN — accept the P7b source/static packet.**

## Scope

This is a fresh audit after the fourth RED remediation. I read the P7 plan and P7b contract, Sol's current implementation report, the initial, second, third, and fourth independent P7b audits, and the actual staged diff. I then inspected the live Puzzle2d editor branch, mounted fill session/capture/terminal/apply paths, Board ingress and terminal codec, fixture path, and local source predicates. Prior claims were not accepted as evidence.

No Cargo, Nx, Wasm, browser, runtime, shared-script, or production-source command/edit was made. This is source/static acceptance only.

## Decisive Findings

The two fourth-audit P0s are closed in the live paths.

- `FillPlacementApplyCursor`, `FillPlacementHandleOwner`, `FillPlacementNodeOwner`, and `FillPlacementEdgeOwner` retain only `BoardFillText`, scalars/enums, and the fixed 16-slot handle array. The borrowed `FillPlacementPublishView` has only references/scalars. The retained-owner census is `String=0`, `Vec=0`, `BTreeMap=0`, `Puzzle2dNode=0`, and `Puzzle2dHandle=0`; the publish-view census is `String=0`, `Vec=0`, `BTreeMap=0`. The direct retained apply has nine one-byte-copy call sites and no whole placement/handle-array reconstruction.
- The producer is a fixed 10,406-byte, versioned `BoardFillCommitCandidate`, placed in one retained commit-output page. It carries the optional complete placement: node and edge IDs/kinds, source/target IDs, exact source edge kind, geometry, optional icon, handle count, and all sixteen possible fixed handles. Decode requires the magic/version, exact byte and page layout, empty state, bounded indexes/count, UTF-8, valid tags, and finite geometry.
- The mounted consumer invokes the same full-candidate decoder and same pre-credited typed publication helper on the initial complete branch and the retained-outcome retry branch. It reserves the two event slots and the schema handle backing before materializing output, then emits exactly `create_node` and `connect_handles`, including the candidate's source edge kind. `BoardFillJob::take_result` is absent.
- The live fill dispatch stays before the ordinary cloned-document/BoardHost route. Its branch has no whole config clone/snapshot mutation, dynamic map/vector authority, `BoardHost`, `RefCell`, or document clone. Worker capture retains the generation-qualified `SnapshotRead`, has exactly one `capture_one` call per wrapper turn, rechecks authority around it, and consumes one fuel unit.

The decoder necessarily creates its fixed typed value while reading the retained page; it does not introduce dynamic backing, a second retained output page, checkpoint-side authority, or a reassembled 10,406-byte/handle-array owner. The action-side dynamic strings and vectors occur only after the final destination has been pre-credited, while materializing the two outgoing schema mutation values in that same terminal turn; they are not retained session/apply/publish-view authority.

## Independent Gates

| Gate | Result |
| --- | --- |
| Four remediation baselines: full fixed codec, retained fixed owners/view, initial+retry terminal consumption, same-turn output credit | GREEN |
| Ten faithful hostile mutations | GREEN — all rejected |
| Six preserved early-route/capture/runtime/ingress/handback/lifecycle predicates | GREEN |
| Scoped edition-2021 `rustfmt --check --config skip_children=true` over Board and eleven Puzzle2d Rust leaves | GREEN |
| Puzzle2d fill-config schema parsed with Bun `JSON.parse` | GREEN |
| Scoped staged `git diff --check` over the P7b Board/Puzzle2d source leaves | GREEN |
| Raw production census | GREEN |

The ten hostile mutations were independently applied in memory to the actual production slices, not treated as passing test-name strings:

1. retained edge owner becomes `String`;
2. borrowed publish-view edge kind becomes `String`;
3. retained edge-kind byte copy becomes a whole fixed-text assignment;
4. checkpoint apply reconstructs a whole commit placement/handle array;
5. terminal decoder becomes the former summary decoder;
6. terminal placement is discarded;
7. same-turn two-mutation reservation is removed;
8. terminal placement gains `String` backing;
9. terminal layout returns to 13 bytes;
10. terminal text encoder performs a whole-slot copy.

Each mutation falsified an independent structural predicate that inspected the relevant live callee slice. The predicates were not simple searches for mutation labels and were run only over production text, excluding the embedded test region.

## Raw Census

| Slice | Observed result |
| --- | --- |
| Retained placement owners | `BoardFillText=11`; no `String`, `Vec`, `BTreeMap`, `Puzzle2dNode`, or `Puzzle2dHandle` |
| Borrowed publish view | no `String`, `Vec`, or `BTreeMap` |
| Terminal | one full decoder; two consumer publication calls; zero `take_result` |
| Fixed pages | 11 `try_push_owned` uses, all explicit `if let Err(owner)` handbacks; zero direct discarded `try_push_owned(...).is_err()` |
| Early fill branch | zero `config.clone()`, `Puzzle2dConfigMutation::Snapshot`, `BoardHost`, `RefCell`, or document clone |

The wider ticket's staged diff still contains an unrelated P7c report with trailing whitespace. It was excluded from the P7b source-only diff scope; no P7b Board/Puzzle2d path has a diff-check error.

## Acceptance Boundary

This GREEN verdict clears the requested P7b source/static acceptance gates only. Compiler/build, runtime behavior, watchdog, worker-count parity, native/Wasm, and browser evidence remain deliberately unclaimed because this audit was instructed not to run those gates.
