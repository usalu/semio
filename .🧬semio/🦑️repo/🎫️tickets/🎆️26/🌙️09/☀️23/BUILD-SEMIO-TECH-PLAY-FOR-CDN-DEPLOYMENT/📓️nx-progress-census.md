# Nx Progress Census

Observation: 2026-10-07T02:43:29.048151+00:00.

The current inner invocation explicitly reports `build` plus 320 prerequisite tasks: 321 tasks including the final Play build. Its retained stdout records 30 unique prerequisite start headers at this observation. The current producer is `@semio-tech/cad-extension-spatial-shape-rust:describe`, corroborated by the current invocation row with parent PID 86657.

Two task failures are confirmed in this original invocation: Cad `component-dev` and AEC-building `component-dev`. Their subsequent separate retries both passed, but separate retry success does not rewrite the original invocation's historical task statuses. The current full invocation remains active and needs the final consistent rerun after temporary instrumentation is removed.

At the initial history inspection, the private Nx SQLite history contained 10 completed standalone recovery/test invocation entries and had not persisted the main invocation's completed-task history. The shared private `nx/run.json` described a standalone AEC unit run, rather than the active full build. Therefore those persisted artifacts did not produce an exact main-graph successful-task count. Later standalone runs may update these shared files; a start header alone is not treated as successful completion.

Precisely observed counts are 30 prerequisite starts, two confirmed failures and one current producer. Of the 320 prerequisite tasks, 290 have no start header in the current captured log. That is an unstarted-log count, not a scheduler assertion that all 290 remain runnable: downstream nodes may be blocked by the original failures. An exact successful/blocked/remaining status census requires the main runner's terminal task results; no estimated successes are reported as facts.

## Updated Observable Main Graph Census

At 2026-10-07T03:12:18.131499+00:00, the still-active main graph log has 34 distinct prerequisite start headers out of320. Four explicitly failed main invocations are Cad component-dev, AEC component-dev, Demonstrator component-dev and Flow component-dev. Cad and AEC standalone retries succeeded; the historical main task failures remain until a consistent full graph rerun. Main most recently entered Flow BIM describe, following its component-dev transition. A completion total is still unavailable while main Nx task history has not committed; start-header absence is not a success/failure estimate.


Current stream census records 31 distinct started target headers including outer build-fresh. Latest producer: `@semio-tech/flow-extension-brep-rust:component-dev`. Started headers do not prove successful completion. Full graph still active; native compiler sample records framework-plugin and StdIO binary/TXT/DWG under private cold-generation output.


Latest read-only sample: 31 distinct started target headers (includes outer build-fresh; these are starts, not successes). Latest producer is @semio-tech/flow-extension-brep-rust:component-dev. Own cold-generation compiler observations: 72350 68180 01:06 94.1 semio_s_artifact_stdio_dwg, 72598 68180 00:42 12.8 semio_framework_os_flow. Full inner graph remains alive, no final task-history census or pages readiness available.


Subsequent concrete compiler sample: Semio rustc PID 74469 (parent 68180) at 2m55s/36% and PID 75330 (parent 74448) at 1m36s/36.6%, both under this task private cold-generation stores. Descriptor and Curation retry logs still contain their owner preparation banners; no new compiler diagnostic or assertion receipt has been emitted for these retries. No release completion or page readiness is inferred.


Flow Brep metadata component completed its actual wasm-dev compilation in 10m42s and staged one deliverable, then the main graph started its describe target. This confirms prerequisite completion, not optimized release publication. The descriptor regression retry meanwhile progressed through its outer preparation queue from more than 20 owners to admission and now its canonical test body/source preparation. Assertions remain pending.


Main stream now records 33 distinct started target headers, including the outer fresh entry. Latest starts: @semio-tech/flow-extension-bim-rust:describe, @semio-tech/flow-extension-brep-rust:component-dev, @semio-tech/flow-extension-brep-rust:describe, @semio-tech/flow-extension-dictionary-rust:component-dev. This remains a start count, not a whole-task success census. Main is active in Flow Dictionary preparation; no complete release/page acceptance is claimed.


Latest actual producer milestone: flow-extension-dictionary-rust completed wasm-dev compilation in2m02s. The original full graph is still live; this proves only that prerequisite compilation, not complete task success or optimized shipping. Flow typed-owner red is compiling current Semio dependencies, and Curation long gate remains preparation-queued.


Updated streamed census:40 distinct task headers have actually started (including outer launcher); newest sequence BIMdescribe→Brepcomponent-dev/describe→Dictionarycomponent-dev/describe→Drawcomponent-dev. This supersedes earlier33-start observation; none is relabeled a completed task without a whole-task receipt. Dictionarywasm-dev2m02s is actualcompiler completion. Original full main remains alive with earlier failures; no final321 consistency/release/pages receipt yet.


Draw first diagnostic/process audit (root tool session35601 expired; process/log remains authoritative):

```text
@semio-tech/semio-tech-play: > nx run @semio-tech/flow-extension-draw-rust:component-dev
@semio-tech/semio-tech-play: @semio-tech/flow-extension-draw-rust: Warning: The 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.
@semio-tech/semio-tech-play: @semio-tech/flow-extension-draw-rust:       at warnOnDeactivatedColors (internal:tty:33:24)
@semio-tech/semio-tech-play: @semio-tech/flow-extension-draw-rust:       at getColorDepth (internal:tty:42:39)
@semio-tech/semio-tech-play: @semio-tech/flow-extension-draw-rust:       at shouldColorize (internal:util/colors:14:109)
@semio-tech/semio-tech-play: @semio-tech/flow-extension-draw-rust:       at refresh (internal:util/colors:18:31)
@semio-tech/semio-tech-play: @semio-tech/flow-extension-draw-rust:       at internal:util/colors (internal:util/colors:24:16)
@semio-tech/semio-tech-play: @semio-tech/flow-extension-draw-rust:       at internal:assert/assertion_error (internal:assert/assertion_error:2:187)
@semio-tech/semio-tech-play: @semio-tech/flow-extension-draw-rust:       at loadAssertionError (node:assert:28:96)
@semio-tech/semio-tech-play: [process:owner-command] running elapsedMs=11103067
@semio-tech/semio-tech-play: [process:owner-command] running elapsedMs=11113067
@semio-tech/semio-tech-play: [process:owner-command] running elapsedMs=11123072
@semio-tech/semio-tech-play: [process:owner-command] running elapsedMs=11133072
@semio-tech/semio-tech-play: [process:owner-command] running elapsedMs=11143072
@semio-tech/semio-tech-play: [process:owner-command] running elapsedMs=11153074
@semio-tech/semio-tech-play: [process:owner-command] running elapsedMs=11163076
@semio-tech/semio-tech-play: [process:owner-command] running elapsedMs=11173078
@semio-tech/semio-tech-play: [process:owner-command] running elapsedMs=11183080
@semio-tech/semio-tech-play: [process:owner-command] running elapsedMs=11193081
@semio-tech/semio-tech-play: [process:owner-command] running elapsedMs=11203083
@semio-tech/semio-tech-play: [process:owner-command] running elapsedMs=11213084
@semio-tech/semio-tech-play: [process:owner-command] running elapsedMs=11223085
@semio-tech/semio-tech-play: [process:owner-command] running elapsedMs=11233091
@semio-tech/semio-tech-play: [process:owner-command] running elapsedMs=11243093
@semio-tech/semio-tech-play: [process:owner-command] running elapsedMs=11253095
@semio-tech/semio-tech-play: [process:owner-command] running elapsedMs=11263099
@semio-tech/semio-tech-play: [process:owner-command] running elapsedMs=11273102
@semio-tech/semio-tech-play: [process:owner-command] running elapsedMs=11283104
@semio-tech/semio-tech-play: [process:owner-command] running elapsedMs=11293106
@semio-tech/semio-tech-play: [process:owner-command] running elapsedMs=11303108
@semio-tech/semio-tech-play: [process:owner-command] running elapsedMs=11313110
@semio-tech/semio-tech-play: [process:owner-command] running elapsedMs=11323113
@semio-tech/semio-tech-play: [process:owner-command] running elapsedMs=11333114
@semio-tech/semio-tech-play: [process:owner-command] running elapsedMs=11343114
@semio-tech/semio-tech-play: [process:owner-command] running elapsedMs=11353114
```


Draw diagnostic is the standard NO_COLOR/FORCE_COLOR warning, not an assertion failure. Main86484→86656→86657 remain alive after3h29m; current Draw describe2818/prepare2837 follows component-dev. Current streamed headers:41 distinct starts, not completion count. Current own Flow Cargo-phase waits explicitly report artifact-directory lock; active shared native compiler is root child-history job (rustc2992 semio_framework,99%CPU), preserved.

```text
@semio-tech/semio-tech-play: @semio-tech/flow-extension-draw-rust: 46031 | ...   let descriptor_value: DslValue = decode_wire_serialized_or(&descriptor, semio_framework_value::DslValue::Null).await;
@semio-tech/semio-tech-play: @semio-tech/flow-extension-draw-rust: 46031 -                     let descriptor_value: DslValue = decode_wire_serialized_or(&descriptor, semio_framework_value::DslValue::Null).await;
@semio-tech/semio-tech-play: @semio-tech/flow-extension-draw-rust: 46031 +                     let descriptor_value: DslValue = decode_wire_serialized_or(&descriptor, DslValue::Null).await;
@semio-tech/semio-tech-play: @semio-tech/flow-extension-draw-rust: 46044 | ...   let descriptor_value: DslValue = serde_json::from_str(&descriptor_json).unwrap_or(semio_framework_value::DslValue::Null);
@semio-tech/semio-tech-play: @semio-tech/flow-extension-draw-rust: 46044 -                         let descriptor_value: DslValue = serde_json::from_str(&descriptor_json).unwrap_or(semio_framework_value::DslValue::Null);
@semio-tech/semio-tech-play: @semio-tech/flow-extension-draw-rust: 46044 +                         let descriptor_value: DslValue = serde_json::from_str(&descriptor_json).unwrap_or(DslValue::Null);
@semio-tech/semio-tech-play: @semio-tech/flow-extension-draw-rust:     Finished `wasm-dev` profile [unoptimized] target(s) in 9m 54s
@semio-tech/semio-tech-play: @semio-tech/flow-extension-draw-rust: [nx-native] staged 1 deliverables in ✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/📦️packages/🦀️rust/dist/component-dev
```


Latest bounded main log census:42distinct target-start headers including the outer launcher. Draw described and the Flow list component-dev now compiles inside the original cold graph; last main elapsed13,403,668ms. These are starts, not successful task counts; original Nx footer remains absent. Wrapper86656/innerNx86657 remain alive despite the original tool session expiration. No whole321 success, fresh final catalog or pages readiness claimed. Final consistent rerun uses the existing cold generation with all task/remote caches bypassed after current source owners freeze.


Main continued naturally:43distinct target starts including outer. Flow list component-dev completed actual wasm-dev5m23s with one staged plugin, then :describe entered owner13606→prepare13607. InnerNx86657 remains alive3h49m. All task counts still start-header counts, no final success/footer.


Original graph now46distinct task-start headers including outer launcher, latest Flow math component-dev compiling framework UI at outer15,084,326ms. Healthy flow extension sequence continued after targeted Demonstrator milestone; original historical task failures still require final all321 consistency rerun after Value release-axis schema/consumers and domain native proofs stabilize. No final success/footer observed.


Latest read-only census: 48 distinct target start headers including outer Play, with Flow logic describe, math component/describe and core component-dev most recent. Main log elapsed 15,564,488ms; no whole-graph footer. This is an exact start count, not a completed-target count. Standalone declared-endpoint native log is compiling vello_svg after framework dependencies, no E-code/runtime receipt. Preserve current private generation and healthy owners.


Read-only private Nx SQLite census: task_history contains 59 failure and 30 success rows, but these include all concurrent same-generation diagnostic invocations and historical retries. running_tasks is empty, although the actual original build descendant process is alive and its log advances. task_invocations records the outer root build only rather than all inner prerequisites. Therefore this database cannot establish original-graph completed/failed/remaining counts; reporting those aggregates as the 321 graph result would be incorrect. Exact log target start count and concrete active producer remain the reliable bounded observations until a complete Nx footer.


Latest original graph log: 51 distinct target start headers; Flow core component and description completed far enough to admit text extension component-dev, which is in actual artifact-rust compilation at elapsed100065ms. Main outer elapsed15794556ms. No complete graph footer. Standalone host first gate completed compiler-red (two actor fixture calls); canonical retry49460 active. Editor-owned Value full gate has actual native compile and 164-run/161-pass/3-fail/0-skip runtime receipt, requiring owner correction before final acceptance.


Latest bounded log observation: 52 distinct target start headers, GIS plugin component-dev is latest after Flow text description. Main elapsed15994603ms; no graph footer. Standalone endpoint retry49460 has entered Cargo build elapsed60015ms; no E-code/result in its current tail. All healthy owners preserved, no cache deletion or lease bypass.


Main bounded log now55 distinct started targets, procedural plugin component-dev follows playbook/procedural extension. Current composition preparation reports44conversion targets, outerelapsed17665007ms. No complete graph footer, no final optimized pages accepted. Geometry diagnostic52794 is in actual Cargo build elapsed630123ms; six-row boundary native policy baseline3451 is admitted to native owner preparation. Source strict catalog5cases and independent boundary Manifold19assertions green remain separate source milestones.
