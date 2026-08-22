# P8t Independent Remaining-Tools Global Audit

## Verdict

**REJECT — P0.** This is a source-first, read-only static audit of the remaining
interactive-tool cohorts. Draw, Flow, Forms, and the WFC work already covered by
their independent reviews were not credited or re-audited; Remodel is active repair
and was excluded. The passing catalog verifier is a registration-shape check, not
evidence that the registered work meets the Phase 8 InteractiveJob contract.

Two read-only verifier invocations completed:

- `bun ./📜️script.ts verify interactivity tool-jobs --format json`: exit 0; 50
  macro hosts, 775 rows (773 unique), and 775 rows labelled bounded.
- `bun ./📜️script.ts verify interactivity`: exit 0; the four UI-root static
  policy scopes were clean.

Neither result changes this rejection, for the source evidence below.

## Audit Basis And Required Path

The required path is:

```text
Tool input
  -> command-specific raw pre-deserialization envelope
  -> Operation
  -> resumable InteractiveJob factory
  -> one bounded turn with cancellation, progress/preview, and checkpoint
  -> generation/revision-validated fresh commit
```

An apparently small first step only qualifies if the exact input and work have a
static bound below 8 ms. This audit found no such proof for the general catalog or
the rejected cohort routes. A generic JSON cap is not a command-specific workload
bound, and a one-call terminal adapter is not a resumable job.

## P0 Evidence

| ID | Finding | Exact source evidence |
| --- | --- | --- |
| P0-01 | The common factory runs arbitrary application handlers synchronously in one terminal step. It has no cursor/checkpoint, bounded work proof, preview/progress result, or commit-time revision/generation check. | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:11211-11229` stores a command and output mutex; `:11231-11257` consumes it with `take()`, calls `resolve_ready(A::ephemeral(...))` and `resolve_ready(A::handle(...))`, then returns a terminal empty-state `CommitCandidate`. `:11239` is only a debug assertion, not validation. |
| P0-02 | UI dispatch silently turns a missing exact job key into the generic `typed-command` route. Thus an alias/registration can evade command-specific classification. | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:13150-13178`: missing lookup is not rejected and `:13177-13178` selects the exact key or falls back to `"typed-command"`. |
| P0-03 | The common runner starts a worker then awaits the whole handler; it has no supplied per-command cancellation handle or intermediate publish/commit protocol. | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:13207-13221` uses root cancellation, static interactive budget, `process_worker_pool`, `run_on_worker_async`, then awaits it before querying the watchdog. |
| P0-04 | Direct framework action routes bypass InteractiveJob admission altogether, including unbounded history and imports/configuration. | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:12957-12967` loops `undo_count` dispatches; `:13049-13145` handles clipboard fragment copy/cut/paste directly; `:13269-13285` awaits `A::import_media` and emits directly; `:13287-13297` dispatches configuration binary commands directly. |
| P0-05 | The catalog classifier self-fulfills “bounded”: it finds only literal macro rows and marks every row bounded unless it exactly carries one of three exception strings. It cannot prove one-step cost, envelopes, factories, checkpoints, aliases, or non-macro registrations. | `📜️script.ts:894-906` scans `app_commands!` invocations only; `:920-925` recognizes only `BatchOnlyPendingRewrite\|ForbiddenFromUi\|Deleted`; `:936` has an empty required-ID list; `:939-940` enforce only host/row lower bounds; `:943-965` use substring existence checks; `:975` derives bounded rows by subtraction. |
| P0-06 | “Migrated” is declarative, not independently established. The normal manifest builders assign it as their default classification. | `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs:889-892,1302-1306` implement `bounded_catalog` by setting `InteractiveJobClassification::Migrated`; `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:4996-5042` uses these builders for app mutations/actions/commands. |
| P0-07 | Raw input protection is global and generic outside the Draw/Flow/Forms list; it is not command-specific and occurs only around JSON-envelope validation. | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:16836-16849` declares generic 256 KiB/string/depth limits and enumerates only DFF-specific limits; `:16943-17010` validates the generic public envelope. None of the remaining cohorts have a command-specific pre-deserialization work/byte bound here. |
| P0-08 | Layout export routes allocate/render/compress whole documents in one handler. | `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/🦀️component.rs:491-521` builds a full display list, allocates an image, performs nested pixel loops, and PNG-encodes it; `:523-552` serializes and writes a whole ZIP. `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🐚️export-png/🦀️component.rs:18-22` invokes it then base64-encodes the complete bytes. |
| P0-09 | Animate video export deserializes a whole scene and executes a data-derived frame loop inside the generic terminal route. | `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🐚️export-video-from-deck/🦀️component.rs:14-35` accepts `output_dir`/scene JSON, fully deserializes, and exports; `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🦀️component.rs:561-565` iterates frames from content-derived duration/FPS. `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:294` awaits that command handler. |
| P0-10 | Note ink application deserializes arbitrary event JSON, clones a whole snapshot, and iterates event-derived collections without an operation cursor. | `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖊️ink-apply-events/🦀️component.rs:77-131` clones/diffs and loops events/assets; `:135-159` accepts `events_json`, deserializes it, and invokes the diff. |
| P0-11 | VCS edit deserializes a full document then diffs unbounded tags. | `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🩹️edit/🦀️component.rs:9-34` iterates current/next tags; `:47-71` fully decodes input JSON and invokes the diff. |
| P0-12 | Sourcing set-artifact JSON takes a full snapshot directly, with no command-specific raw limit or resumable conversion. | `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📄️set-artifact-json/🦀️component.rs:18` calls `serde_json::from_str::<CurateSnapshot>(&payload.json)`. |
| P0-13 | Puzzle command routes are absent from the macro catalog, so the 775-row result has no coverage for the remaining non-P4/P7 puzzle commands. Puzzle also keeps live interactive payload/session authority in process globals. | No puzzle editor appears among the 50 `app_commands!` hosts selected by `📜️script.ts:894-906`. `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:98` holds mesh payloads in `PUZZLE3D_MESH_REGISTRY`; `:1966-1982` holds play sessions in `OnceLock<Mutex<...>>`. `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:1404-1407` holds a thread-local play session. |
| P0-14 | Block, Process, Sourcing, Note, and Layout retain document/tool payload outside the operation/job state. This violates the no process-global payload-authority requirement and makes worker migration/restart behavior non-durable. | Block: `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🦀️component.rs:105-118` (vortex catalog scratch). Process: `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🦀️component.rs:778-780` (stock/step scratch). Sourcing: `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🦀️component.rs:199-214` (catalog scratch). Note: `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🦀️component.rs:350-364` (text scratch). Layout: `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🦀️component.rs:85-86` (drawing scratch). |

## P1 Evidence

| ID | Finding | Exact source evidence |
| --- | --- | --- |
| P1-01 | Generated IDs are filtered only against migrated IDs and can fall back to `typed-command`; there is no exhaustive alias-to-catalog assertion. | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:10880-10900`. |
| P1-02 | CAD preview sequencing is thread-local ephemeral state rather than an operation generation stored with a resumable job. | `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:893-918` declares and increments `CAD_PREVIEW_SEQ`. |
| P1-03 | Process and sourcing contributed catalog payloads have additional global mutex authority. | Process: `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:667-668`. Sourcing: `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs:564-565`. |
| P1-04 | The broad interactivity verifier checks only four UI roots and a sanctioned runtime bridge. It is not a route, type, call-graph, or Wasm execution proof. | `📜️script.ts` verifier output reported scope `4 UI roots`; its tool-job verifier source is the limited macro/sub-string scanner at `:894-975`. |

## Cohort Coverage Matrix

“Registration coverage” is an audit observation, not approval. “Generic” means the
route enters the P0-01 adapter unless it uses one of the direct bypasses above.

| Cohort | Live source/registration coverage examined | Result |
| --- | --- | --- |
| Framework commands, clipboard/history, imports, configuration | Generic factory, generated aliases, direct action dispatch, envelope validator | **REJECT P0** — P0-01 through P0-07 |
| CAD | CAD macro host/action declarations; import/selection/transform routes; preview sequence | **REJECT P0/P1** — generic terminal handler lacks exact cost proof; P1-02 |
| Layout | Layout macro host, 20 command source roots, export PNG/PDF/SVG/package paths | **REJECT P0** — P0-08, P0-14 |
| Block | Block macro host and 39 command source roots; 3D vortex actions | **REJECT P0** — P0-14 |
| Process | Process macro host and 14 command source roots; process3d media/export routes | **REJECT P0/P1** — P0-14, P1-03 |
| Sourcing | Sourcing macro host and 15 command source roots; curate set/stock routes | **REJECT P0/P1** — P0-12, P0-14, P1-03 |
| VCS | VCS macro host and 10 command source roots; edit/diff route | **REJECT P0** — P0-11 |
| Animate | Animate macro host and 18 command source roots; video export | **REJECT P0** — P0-09 |
| Note | Note macro host and 36 command source roots; ink events | **REJECT P0** — P0-10, P0-14 |
| Procedural, excluding independently-audited WFC | Procedural 2D/3D macro hosts and 21/31 command source roots; generic command path | **REJECT P0** — no exact input/work proof beyond P0-01/P0-07. Existing WFC `run_to_completion` instances were not used to re-score WFC. |
| Puzzle 2D/5D/3D outside proven P4/P7 slices | 35/41/55 command source roots; non-macro editor registrations and session stores | **REJECT P0** — P0-13. Existing P4/P7 proof is not extended to the remaining puzzle aliases. |
| Remaining inventory-discovered macro plugins (including search, selection, snapping, booleans, routing, baking, diff, compression/package operations where registered) | All 50 macro hosts through the generic registry and 775-row catalog | **REJECT P0** — catalog shape has coverage, but each receives P0-01/P0-05/P0-06/P0-07 unless separately proven by a concrete resumable factory. No blanket proof was found. |

The layout, animate, note, VCS, sourcing, and framework samples are not
representative-only exemptions: they demonstrate that the purported universal
“bounded catalog” classification permits concretely unbounded production work.

## Classification Cross-Check

1. The verifier counts rows that syntactically occur in `app_commands!`, but it
   does not enumerate all dispatch keys accepted by
   `AppActionRegistry::tool_job_registration`.
2. A row becomes “bounded” simply by not matching one of the three exception
   strings. There is no “unclassified” disposition and no command-specific
   proof object.
3. `bounded_catalog` writes `Migrated` at definition time, so validating that
   a definition declares `Migrated` only validates the builder default.
4. A live missing exact key goes to `typed-command`, which makes classification
   appear complete despite an alias not having an exact catalog key.
5. Puzzle’s non-macro registrations do not enter the 775-row ledger at all.

Accordingly, the measured `775 bounded` result must be interpreted as “775
normal macro rows,” not “775 P8-bounded InteractiveJobs.”

## Explicit Verifier Blind Spots

- No Rust AST/type or call-graph analysis confirms that a handler returns before
  expensive work, rather than calling it inside `AppCommandJob::step`.
- No per-command raw-wire measurement before serde deserialization, nor bound on
  decoded collection cardinality, image dimensions, output bytes, file count, or
  compression work.
- No check that jobs persist a cursor/checkpoint for unbounded input.
- No check for progress/preview publication, command cancellation propagation, or
  fresh generation/revision validation before commit.
- No complete key/alias registry comparison, and no inventory of non-macro
  dispatch routes.
- No detection of payload-bearing `thread_local!`, `OnceLock`, `LazyLock`,
  or global `Mutex` stores.
- No test of worker migration, interrupted job recovery, stale-result rejection,
  parallel input, or action cancellation.

## Gates Not Run

Per the read-only constraint, no Cargo command, build, test suite, cache mutation,
or runtime/Wasm invocation was run. The following remain mandatory before a PASS:

- Native Rust type/borrow/Send validation and release behavior for every affected
  plugin and the framework job runner.
- Actual worker handoff, cancellation latency, intermediate progress/preview,
  persisted checkpoint resume, and stale generation/revision rejection tests.
- Large-input scale tests for exports, event/tag arrays, scene frames, zip/media
  payloads, package operations, and all remaining selection/routing/baking/search
  commands.
- Command-specific pre-deserialization envelope rejection tests, including
  decoded-size/cardinality and output-size limits.
- Wasm target compile and execution gates, including plugin descriptor discovery
  and activation under the intended worker runtime.
- Puzzle, generated-alias, and non-`app_commands!` registration enumeration
  compared exhaustively with the P8 classification artifact.

## Required Disposition

Do not promote any remaining cohort from this audit to PASS. Replace the generic
terminal adapter with operation-specific resumable factories where work is not
statically below 8 ms; persist all work/cursor/input authority with the operation;
make exact registrations and aliases fail closed; add command-specific raw bounds;
then re-run static, native-runtime, and Wasm gates.
