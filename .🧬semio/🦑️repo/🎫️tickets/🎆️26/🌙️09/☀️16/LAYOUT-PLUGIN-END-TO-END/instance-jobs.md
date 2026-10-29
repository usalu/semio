# Instance Jobs Current Interface Alignment

2026-10-28. Native mutations-native-13.log exposed obsolete dispatch/new calls and a host helper named from the wrong owned crate.

## Actual Consumer Proof

Fresh source inspection across tracked and untracked framework and s source trees, pruning only dependency/cache/output directories, searched Rust/TS/JS/JSON/WIT/GraphQL/proto surfaces. Inspected 109697 files. Remaining name references after removing the unused three-method chain:

[
  {
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🛬️lifted/🏛️reserved/🦀️.rs",
    "name": "run_framework_reserved_job",
    "lines": [
      23
    ]
  },
  {
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-job-coverage/🧾️proof-catalog-and-admission/🟦️.ts",
    "name": "run_framework_reserved_job",
    "lines": [
      151
    ]
  },
  {
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract-part-5/🥾️operation-action-stack-and-peer-presence/🦀️.rs",
    "name": "run_framework_reserved_job",
    "lines": [
      75
    ]
  },
  {
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-dff-public-action-admission/🦀️.rs",
    "name": "run_framework_reserved_job",
    "lines": [
      145
    ]
  },
  {
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👷️job/🏛️reserved/⚙️worker/🧪️tests/🟦️.ts",
    "name": "run_framework_reserved_job",
    "lines": [
      10
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🛣️routes/🟦️.ts",
    "name": "run_framework_reserved_job",
    "lines": [
      70,
      134
    ]
  },
  {
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🏭️factory/🟦️.ts",
    "name": "run_framework_reserved_job",
    "lines": [
      385,
      398,
      430,
      433
    ]
  },
  {
    "path": "✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🔬️tool-job-puzzle-reserved-routes/🟦️.ts",
    "name": "run_framework_reserved_job",
    "lines": [
      16
    ]
  }
]

These remaining references are source-law textual recognizers, positive/negative fixture strings, tests asserting absence or explanatory comments; they are not executable runtime call sites, schemas, WIT exports or reflection hooks. The recognizers describe old and negative fixture source shapes and do not dynamically dispatch named Rust methods. A separate repository-wide tracked git grep corroborates this.

## Changes

Removed only the unused run_framework_reserved_job/run_framework_reserved_job_inner/drive_framework_reserved_worker chain from instance/jobs/staging/ingress. Its sole obsolete ActionBus.dispatch and ToolOperationSpec::new references had no current consumers, and adapting that dead by-value J route would create dishonest custody and identity behavior. The actual live admit_original_reserved_producer and admit_retained_tool_producer routes remain untouched, retaining the original StepContext/body/caller/source scopes and bounded ownership. No compatibility alias, replacement constructor or fake authority was introduced.

The tool-run host helper already exists and is mounted at reactor/tasks/extension-continuation/host_for_instance. The job request now references that actual reactor-tasks owner directly, and the jobs manifest declares that dependency. Reactor tasks depends on actor-host/decl-emission/ABI/framework/value/kernel and has no dependency back to instance-jobs, so this introduces no dependency cycle. No host implementation was fabricated.

Exact changed files under 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin: 🧵️instance/👷️jobs/📥️staging/🚪️ingress/🦀️.rs; 🧵️instance/👷️jobs/📥️staging/⏯️tool-run/🦀️.rs; 🧵️instance/👷️jobs/📦️packages/🦀️rust/Cargo.toml.

No behavior was added, so no redundant new regression fixture was created. The existing source law explicitly asserts the old reserved method name is absent from live exchange dispatch. Compiler validation remains sequenced by Rust; this packet launched none.

Canonical normal source inventory after the packet:18inputs/171580bytes/1981active lines. Unchanged32/262144/4000 bounds. Input retained as instance-jobs-inventory.ts.

## Source-Law Asset Follow-Up

No runtime consumer or external export hook exists, but the broad search found positive historical source-law recognizers in repo laws/tool-job-coverage/factory (lines385/398/430/433), routes (70/134), and a Puzzle positive host fixture (16). They recognize an older textual architecture and need a precise separate migration to the actual original producer/worker route. Blind renaming would falsely assert preserved authority semantics. Root and Rust received the exact paths; this packet does not claim those source-law fixtures migrated. Other references are intentional rejection fixtures, absence assertions or explanatory comments.

## Source Law Follow-up

The positive factory and Puzzle host recognizers now inspect the actual original producer/caller/worker custody route. Intentional rejection fixtures remain. The managed source-law suite passed18/0, including16neutral host cases and37nested Puzzle checks; see source-laws.md for exact files, command, independent Ajv/JSONPatch witness and review.
