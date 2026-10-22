# Historical Command Provenance Audit

Date: 2026-10-21. Read-only; no source edit, job launch, process signal, external chat access or duplicate process diagnostic. Source9977 and the existing KernelEX route remain unchanged.

## Result

**No parent ownership attribution established.** The retained ticket evidence identifies command strings and PIDs but contains no PPID, parent Original, child birth receipt or launch record linking the three commands to this fleet or to Flow-browser parent PID37466. Project names alone do not justify calling them foreign. Equally, the matching demonstrator name does not establish that the demonstrator job is ours.

Current inspected SourceDemand and producer-test bodies do not launch these commands. This is evidence about current source, not proof of the source revision older PID37466 loaded or of its full historical descendant tree.

## Retained matches

A bounded search covered 199 direct ticket Markdown/JSON/log/text files, approximately34.1MB, including every direct generated log under8MiB. Repeated queue references were counted separately from launch evidence.

| Exact command fragment | Retained evidence | Attribution |
| --- | --- | --- |
| `bazel show project @teaching/proctor` | `🗑️generated/leases-wire-current.json:69,82,95,108,121`; PID81882; actual prefix is `repo --tool=bazel show project @teaching/proctor` | Status only |
| `semio-tech-play:activate-prototype` | Same file134,147,160,173,186; PID87073; `repo --tool=bazel run @semio-tech/semio-tech-play:activate-prototype` | Status only |
| `mit-bestand-demonstrator:activate-prototype` | Same file199,212,225,238,251; PID87765; `repo --tool=bazel run @semio-tech/mit-bestand-demonstrator:activate-prototype` | Status only |

That retained snapshot is dated2026-10-21T11:11:40.888Z. Its first recorded holder timestamps are10:43:50.945Z,10:48:15.694Z and10:48:46.315Z respectively; these are lease timestamps, not process birth times.

The demonstrator fragment also occurs in `closure-audit-2026-10-21.md:9` as a proposed activation command. That is a plan, not an execution receipt. No direct retained execution log contained a launch of any of the three exact Bazel command strings.

## PID37466

The same snapshot records PID37466 with:

`repo run semio-framework-os-flow-browser-package:test-producer-source --skip-cache --output=stream`

Its first listed retained holder timestamp is2026-10-21T02:02:46.127Z. Six retained lease snapshots repeat that command; ticket reports and old quota/unit/source logs also reference PID37466 as a queue predecessor. These observations prove what the diagnostic reported, not this fleet's ownership or a parent/child relationship to the three later PIDs.

Holder rows contain resource/hash/mode/pid/task/sinceMs/heldSeconds/class/rank/source/command. They do not contain PPID, parent invocation identity, a child birth receipt, or an executor session owned by this fleet. Historical labels such as “foreign browser verification” in `execution-2026-10-21.md:162` were target-based correlation and must not be reused as provenance proof.

## Current fixture/source routes

A bounded source scan covered3,487 TS/MJS/JSON files,39,045,028bytes,6,800entries under the repo library and Flow browser-package owner. It excluded dependencies and output trees. It found no exact proctor-show or play-activation command string. Relevant demonstrator activation matches elsewhere in the repo library are incremental scenario DATA and the demonstrator dev watch selection, not this SourceDemand/producer test's executed children.

Flow owner:

`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🎁️browser-package/📦️packages/🦀️rust/📋️project.json:394`

The current `test-producer-source` target dispatches one Bun test file:
`🏗️build/🧪️tests/🟦️.ts`.
That complete37-line test binds the wasm command as DATA, reads source, constructs TypeScript programs/module resolution, validates generated-dependency behavior and queries an in-memory SQLite oracle. It contains no process spawn/run/capture and does not execute the producer script it inspects. Its neutral fixture has three generated-ownership cases; none names the three audited CLI commands. No current Flow target declares a dependency on `test-producer-source`.

Repo library:

- `🏗️build/🧪️tests/🕸️graph/🎯️source-demand/🟦️.ts` and its static support/scope imports contain no real CLI spawn/run/capture route for these commands. Current SourceDemand tests inspect source, run bounded in-process compiler/resolver/oracle fixtures and read declared resources.
- `🏗️build/📥️caller/🧪️tests/📥️birth/🟦️.ts:58` does launch its own bounded local `📜️script.ts` child with a birth.json receipt. That child:14 captures Node `-e` which prints the received process envelope and argv; it does not run Nx or repo despite passing “nx” as printed argv DATA.
- The retained ticket has no direct dispatch record for the Flow producer-source invocation itself. The target's appearance in status cannot establish who launched it.

## Limits and disposition

Older loaded fixture/module bytes were not retained as a process-specific source receipt in this evidence set. Current source cannot conclusively rule out an older implementation spawning commands, and none of these matches proves such spawning. Only the separately owned supported parent/process diagnostic or a genuine child-birth/Original receipt could supply additional lineage.

Classification remains **unattributed**, not “proven foreign” or “proven ours.” No cancellation or ownership claim follows. No source/API/custody change was made.

## Supported Process Snapshot Outcome

2026-10-21T12:45:32.963Z: Root used documented repo capacity run with128MiB/1CPU for a bounded read-only ps of the four known PIDs. It waited normally, admitted with20561MiB available against6553MiB floor, and exited0. Output contained only37466 with parent1181; the other three requested PIDs were absent. This does NOT establish their prior ownership or cause of termination. No process signal/action occurred. Complete output is generated/process-provenance.log; current Source queue progression is recorded separately.


## Supported Current Queue Snapshot

2026-10-21T13:21:41.844Z: repo leases status --json was captured completely to generated/leases-group-admission.json. Actual graph-image KernelEX holder45042 is registry-attributed to a workspace repo exec gofmt invocation, held375seconds at snapshot. SourceDemand13310 PID54787 waitsSHposition5/5, with blockers38635/157,41162/167,45042/158; no cycles reported. Ownership of external chat/process not otherwise inferred, and no signal sent. Capacity status concurrently observed15,154MiB available, normalpressure, floor6,553MiB, zero swap-out; aggregate load47.17 on10cores. This is queue/host evidence, not tests or a memory bypass. Initial bounded toolstdout was truncated, so it was not parsed as complete JSON; complete native file capture supplied this snapshot.
