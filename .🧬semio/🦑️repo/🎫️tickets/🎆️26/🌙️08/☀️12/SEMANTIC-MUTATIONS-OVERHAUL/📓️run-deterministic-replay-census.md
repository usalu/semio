# Run Deterministic Replay Census

## Scope

Read-only census for the frozen five-leaf Run packet. No source, schema, fixture, codec, or launcher was changed. This records the minimum Start/Seal timestamp propagation required before a deterministic-replay implementation can be approved.

## Current application-time clock reads

| Domain result | Exact consumer | Current source | Replay consequence |
| --- | --- | --- | --- |
| `RunArtifact.started_at` | `apply_run_operation` Start arm | `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🦀️component.rs:1948-1957` | Reapplying identical `startRun` bytes changes the artifact. |
| `RunArtifact.finished_at` | `apply_run_operation` Seal arm | `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🦀️component.rs:1972-1975` | Reapplying identical `sealRun` bytes changes the artifact. |
| Node-start audit log `at` | `apply_run_operation` StartRunNode arm | `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🦀️component.rs:1959-1960` | Separate remaining nondeterminism; it is not solved by a Start/Seal-only packet. |

`store::now_iso()` is defined at `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:9647-9652` and currently returns a decimal millisecond string. Its documented ownership is Store edit/checkpoint stamping. It must remain available for that Store responsibility, but a pure Run domain application must not call it.

`RunArtifact` already has the target state shape at `workflow/🦀️component.rs:1867-1868`: `started_at: String` and `finished_at: Option<String>`. The empty artifact deliberately has an empty start timestamp and no finish timestamp (`1885-1886`).

## Required authored propagation

The five operation identities, aggregate variant names, text opcodes, and binary tags remain unchanged. Only two payload bodies need added, required authored values:

| Leaf | New required payload member | State assignment | Required propagation path |
| --- | --- | --- | --- |
| `start-run` / `StartRun` | `startedAt` | `RunArtifact.started_at` | `StartRun` → `StartRun::diff` → `RunDiff::Start` → `RunDiff::apply` reconstruction → `apply_run_operation` |
| `seal-run` / `SealRun` | `finishedAt` | `Some(RunArtifact.finished_at)` | `SealRun` → `SealRun::diff` → `RunDiff::Seal` → `RunDiff::apply` reconstruction → `apply_run_operation` |

The source sites are:

- Start leaf and `MutationKind::diff`: `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🏃️run/🧬️schema/🧬️mutations/🚀️start-run/🦀️.rs:9-25`.
- Seal leaf and `MutationKind::diff`: `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🏃️run/🧬️schema/🧬️mutations/🔏️seal-run/🦀️.rs:9-15`.
- Diff definitions and operation reconstruction: `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🦀️component.rs:1981-2041`.

The field type must be fixed as a single Run timestamp lexical contract before implementation. The current state and Store helper only prove a `String`; they do not establish RFC 3339, HLC, or another grammar for this domain field. Therefore the safe bounded design is a required, nonempty authored textual timestamp with one explicitly frozen grammar shared by Rust validation and JSON Schema. It must not use `Option`, `#[serde(default)]`, a fallback value, or any clock read during `diff` or `apply`.

`finished_at` remains optional only in the artifact because an unsealed run has no finish event. The `sealRun.finishedAt` command payload itself must be required and non-null.

## JSON, text, and binary surfaces

1. Add `startedAt` to the strict payload schema at `.../🚀️start-run/🧬️schema/🔣️.json`, and `finishedAt` to `.../🔏️seal-run/🧬️schema/🔣️.json`. Both appear in `required`; their exact shared lexical schema remains a pre-implementation decision.
2. The aggregate schema at `.../🧬️mutations/🔣️.json` already references leaf `$defs.payload` bodies, so it inherits the fields without changing its five-variant roster or `operation` envelope.
3. Leaf serde derives are strict camel-case records and the aggregate is a strict internally tagged enum (`.../🧬️mutations/🦀️.rs:24-32`). Required Rust fields therefore reject missing and snake-case JSON once added.
4. `StartRun` and `SealRun` derive `dsl::DslRecord`; aggregate text and binary codecs are generic `DslOps`/`Mutations` paths. The new scalar members must be included in the existing exact OpText and OpBinary round trips, rather than adding a separate codec or opcode.

## Producer and consumer census

`RunSink::record` at `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️component.rs:527-530` passes a complete operation through `apply_run_operation_checked`; it must not create timestamps. `SpaceRunner::run` likewise owns node-level records only, and its module documentation explicitly leaves Start/Seal to callers (`run/🦀️component.rs:506-510`). Its `Instant::now` at `1295-1297` measures elapsed node work and is unrelated to durable Run time.

The sole production Start/Seal author is the OS Run executable:

- Start: `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/📦️bin.rs:320-329`.
- Successful seal: `.../📦️bin.rs:346`.
- Failure seal: `.../📦️bin.rs:353-354`.

That executable already authors an `AppendRunLog.at` timestamp at the failure ingress (`:353`). It is the appropriate current boundary to obtain and place Start/Seal timestamps in their commands. The various Workflow and Run component constructors are test fixtures and must supply fixed values after the payload becomes required; the direct source call sites are at `workflow/🦀️component.rs:2439,2471,2480,2492,2506,2526,2537,2560` and `run/🦀️component.rs:2342,2360,2409,2435,2465,2499`.

Store mutation metadata is not a replacement for these domain values. Store replay obtains `mutation.timestamp()` for metadata only and otherwise ticks its local clock (`🏪️store/🦀️component.rs:15548-15561`); `apply_run_operation` receives only `RunMutation`, so metadata cannot populate the Run artifact's two body fields.

## Language-neutral replay vectors for the future packet

1. **Start exact replay:** the same complete `startRun` JSON, including fixed `startedAt`, decodes and applies to two fresh empty artifacts; canonical artifact JSON is byte-equal and retains that exact field.
2. **Seal exact replay:** the same complete `sealRun` JSON, including fixed `finishedAt`, applies to equivalent unsealed snapshots; canonical artifact JSON is byte-equal and has `sealed: true`, its requested status, and that exact finish value.
3. **Sequence preservation:** authored Start then authored Seal produces exactly the two supplied state timestamps; replay never substitutes a later wall-clock value.
4. **Wire parity:** each positive leaf payload survives aggregate JSON, OpText, and OpBinary round trips without changing `start-run`/`seal-run` or tags `0`/`4`.
5. **Strict rejection:** for both leaves, reject missing timestamp, `null`, non-string, malformed according to the frozen shared grammar, snake-case aliases, and unknown fields. No rejected operation alters the base artifact.
6. **No hidden clock:** a trapped clock/application harness proves Start and Seal application consume only the supplied payload value. This belongs beside the Rust and language-neutral vectors once the exact grammar is frozen.

## Decision boundary

This census does not choose a timestamp grammar or change code. Approval is required for that lexical/type contract and for the producer authority at command ingress. A Start/Seal implementation alone will not make every Run replay deterministic because `StartRunNode` still creates an ambient-time log entry; that leaf requires its own authored-time packet.
