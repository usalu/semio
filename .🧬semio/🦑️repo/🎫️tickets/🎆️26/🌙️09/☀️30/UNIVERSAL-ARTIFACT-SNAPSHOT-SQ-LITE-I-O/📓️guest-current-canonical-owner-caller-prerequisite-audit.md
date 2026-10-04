# Current Guest Canonical Owner Caller Prerequisites

Read-only source audit; no Cargo or production edits. The first named-record replay measured 299 diagnostics (118 missing canonical crate links, 176 private-symbol references, five trait-bound diagnostics). Those counts describe that historical compile attempt, not current blockers. The newest parent-run unchanged replay measured 34 diagnostics and reached zero assertions: 32 private Value references in 12 test files and two retired `__rt::field_error` calls.

## Current authoritative pairing

`ToValue`, `FromValue`, `DslValue`, and `ValueError` belong to `semio_framework_value`; replace only their private `protocol`, `store`, or product `dsl` qualifications. Preserve the public protocol mutation, codec, descriptor and artifact traits. `Shape`, `FieldValue`, `RecordSpec`, `RecordSpecProducer`, `RecordValue`, `RecordLayout`, and `DslField` belong to `semio_framework_dsl_record`. Derive entrypoints belong to `semio_framework_dsl_record_derive`. No product compatibility reexport is warranted.

Plugin Cargo manifest lines 38–39 now directly link Record and Record-derive. Both are concurrent source repairs; the prior 118 missing-crate diagnostics no longer justify a manifest edit. Prior five missing DslField diagnostics are cascading compile evidence, not independently verified runtime failures.

## Remaining measured Value caller pairs

| Current file | Symbol | Current lines | Canonical replacement |
|---|---|---|---|
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🛰️declaration-channels-unit/🦀️.rs | FromValue | 2 | `semio_framework_value::FromValue` |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🛰️declaration-channels-unit/🦀️.rs | ToValue | 2 | `semio_framework_value::ToValue` |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🧪️tests/🔬️shell-fault-frame/🦀️.rs | ToValue | 11 | `semio_framework_value::ToValue` |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/🧬️mutations/🔁️set-state/🧪️tests/🧪️set-state/🦀️.rs | ToValue | 12 | `semio_framework_value::ToValue` |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📡️contributed-mutation-wire-unit/🦀️.rs | ToValue | 18 | `semio_framework_value::ToValue` |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️time-travel/🦀️.rs | ToValue | 72 | `semio_framework_value::ToValue` |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️bounded-reload/🦀️.rs | ToValue | 20, 22 | `semio_framework_value::ToValue` |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️bounded-reload/🦀️.rs | DslValue | 21, 27 | `semio_framework_value::DslValue` |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️bounded-reload/🦀️.rs | FromValue | 26, 28 | `semio_framework_value::FromValue` |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️bounded-reload/🦀️.rs | ValueError | 27 | `semio_framework_value::ValueError` |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-owned-media-error/🦀️.rs | ToValue | 16 | `semio_framework_value::ToValue` |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-owned-media-error/🦀️.rs | FromValue | 16 | `semio_framework_value::FromValue` |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-owned-media-error/🦀️.rs | ValueError | 20 | `semio_framework_value::ValueError` |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-declarations-fixture/🦀️.rs | ToValue | 363, 364 | `semio_framework_value::ToValue` |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-declarations-fixture/🦀️.rs | FromValue | 363, 364 | `semio_framework_value::FromValue` |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🧪️tests/🔗️dependency-contribution-unit/🦀️.rs | ToValue | 14 | `semio_framework_value::ToValue` |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️extension-retirement/🦀️.rs | DslValue | 31, 38, 171, 183 | `semio_framework_value::DslValue` |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧩️composition/🦀️.rs | FromValue | 143, 161, 825, 836 | `semio_framework_value::FromValue` |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📄️natural-file-lifecycle/🦀️.rs | ToValue | 83 | `semio_framework_value::ToValue` |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📄️natural-file-lifecycle/🦀️.rs | DslValue | 93 | `semio_framework_value::DslValue` |

At declaration-channels-unit line 2, split the grouped import:

```rust
use semio_framework_value::{FromValue, ToValue};
use protocol::{Mutation, MutationDiff, MutationLeaf, OpBinary, OpText, SemanticMutation};
```

For qualified trait calls/bounds and the natural-file octet pattern, use the named Value owner directly (or an explicit local import). Keep all existing assertions, descriptor serialization, typed cancellation downcast, reserved import fixture bounds and payload pattern unchanged.

## All 54 historical private-symbol/file pairs

| File | Symbol | Measured count | Current source qualification |
|---|---|---|
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🛰️declaration-channels-unit/🦀️.rs | FromValue | 1 | still private at 2 |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🛰️declaration-channels-unit/🦀️.rs | ToValue | 1 | still private at 2 |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🧪️tests/🔬️shell-fault-frame/🦀️.rs | ToValue | 2 | still private at 11 |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📝️draft/🚫️none/♻️retirement/🦀️.rs | ToValue | 2 | old private qualification absent; concurrent repair |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📝️draft/🚫️none/♻️retirement/🦀️.rs | FromValue | 2 | old private qualification absent; concurrent repair |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🫧️transient/🦀️.rs | ToValue | 1 | old private qualification absent; concurrent repair |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🫧️transient/🦀️.rs | FromValue | 1 | old private qualification absent; concurrent repair |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/📥️retained/🦀️.rs | Shape | 28 | old private qualification absent; concurrent repair |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/📥️retained/🦀️.rs | FieldValue | 23 | old private qualification absent; concurrent repair |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/📥️retained/🦀️.rs | DslValue | 10 | old private qualification absent; concurrent repair |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/📥️retained/🦀️.rs | RecordSpec | 4 | old private qualification absent; concurrent repair |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/📥️retained/🦀️.rs | RecordSpecProducer | 3 | old private qualification absent; concurrent repair |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/📥️retained/🦀️.rs | RecordValue | 2 | old private qualification absent; concurrent repair |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/📥️retained/🦀️.rs | DslField | 1 | old private qualification absent; concurrent repair |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🦀️.rs | ToValue | 2 | old private qualification absent; concurrent repair |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🦀️.rs | FromValue | 1 | old private qualification absent; concurrent repair |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🦀️.rs | DslField | 1 | old private qualification absent; concurrent repair |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🦀️.rs | DslValue | 3 | old private qualification absent; concurrent repair |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/🧬️mutations/🔁️set-state/🦀️.rs | ToValue | 2 | old private qualification absent; concurrent repair |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/🧬️mutations/🔁️set-state/🦀️.rs | DslValue | 2 | old private qualification absent; concurrent repair |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/🧬️mutations/🔁️set-state/🦀️.rs | FromValue | 2 | old private qualification absent; concurrent repair |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/🧬️mutations/🔁️set-state/🦀️.rs | ValueError | 1 | old private qualification absent; concurrent repair |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/🧬️mutations/🔁️set-state/🧪️tests/🧪️set-state/🦀️.rs | ToValue | 1 | still private at 12 |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📡️contributed-mutation-wire-unit/🦀️.rs | ToValue | 1 | still private at 18 |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/🦀️.rs | ToValue | 1 | old private qualification absent; concurrent repair |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🛠️tool-machine/🦀️.rs | ToValue | 1 | old private qualification absent; concurrent repair |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🦀️.rs | ToValue | 16 | old private qualification absent; concurrent repair |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🦀️.rs | FromValue | 12 | old private qualification absent; concurrent repair |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️time-travel/🦀️.rs | ToValue | 1 | still private at 72 |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️bounded-reload/🦀️.rs | ToValue | 2 | still private at 20,22 |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️bounded-reload/🦀️.rs | DslValue | 2 | still private at 21,27 |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️bounded-reload/🦀️.rs | FromValue | 2 | still private at 26,28 |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️bounded-reload/🦀️.rs | ValueError | 1 | still private at 27 |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️history-edit-acceptance/🦀️.rs | FromValue | 2 | old private qualification absent; concurrent repair |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️history-edit-acceptance/🦀️.rs | ToValue | 1 | old private qualification absent; concurrent repair |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️node-drag-history/🦀️.rs | ToValue | 2 | old private qualification absent; concurrent repair |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️node-drag-history/🦀️.rs | FromValue | 2 | old private qualification absent; concurrent repair |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs | DslField | 1 | old private qualification absent; concurrent repair |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs | Shape | 2 | old private qualification absent; concurrent repair |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs | RecordSpec | 4 | old private qualification absent; concurrent repair |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs | RecordLayout | 3 | old private qualification absent; concurrent repair |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs | RecordSpecProducer | 1 | old private qualification absent; concurrent repair |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs | FieldValue | 4 | old private qualification absent; concurrent repair |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs | RecordValue | 1 | old private qualification absent; concurrent repair |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-owned-media-error/🦀️.rs | ToValue | 1 | still private at 16 |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-owned-media-error/🦀️.rs | FromValue | 1 | still private at 16 |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-owned-media-error/🦀️.rs | ValueError | 1 | still private at 20 |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-declarations-fixture/🦀️.rs | ToValue | 2 | still private at 363,364 |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-declarations-fixture/🦀️.rs | FromValue | 2 | still private at 363,364 |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🧪️tests/🔗️dependency-contribution-unit/🦀️.rs | ToValue | 1 | still private at 14 |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️extension-retirement/🦀️.rs | DslValue | 4 | still private at 31,38,171,183 |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧩️composition/🦀️.rs | FromValue | 4 | still private at 143,161,825,836 |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📄️natural-file-lifecycle/🦀️.rs | ToValue | 1 | still private at 83 |
| /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📄️natural-file-lifecycle/🦀️.rs | DslValue | 1 | still private at 93 |

## Latest measured diagnostics

| Code | Message | Actual primary location |
|---|---|---|
| E0425 | cannot find function `field_error` in module `semio_framework_dsl_record::__rt` | /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:13539 |
| E0425 | cannot find function `field_error` in module `semio_framework_dsl_record::__rt` | /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:13390 |
| E0603 | trait `FromValue` is private | /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🛰️declaration-channels-unit/🦀️.rs:2 |
| E0603 | trait `ToValue` is private | /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🛰️declaration-channels-unit/🦀️.rs:2 |
| E0603 | trait `ToValue` is private | /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🧪️tests/🔬️shell-fault-frame/🦀️.rs:11 |
| E0603 | trait `ToValue` is private | /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🧪️tests/🔬️shell-fault-frame/🦀️.rs:11 |
| E0603 | trait `ToValue` is private | /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/🧬️mutations/🔁️set-state/🧪️tests/🧪️set-state/🦀️.rs:12 |
| E0603 | trait `ToValue` is private | /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📡️contributed-mutation-wire-unit/🦀️.rs:18 |
| E0603 | trait `ToValue` is private | /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️time-travel/🦀️.rs:72 |
| E0603 | trait `ToValue` is private | /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️bounded-reload/🦀️.rs:20 |
| E0603 | enum `DslValue` is private | /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️bounded-reload/🦀️.rs:21 |
| E0603 | trait `ToValue` is private | /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️bounded-reload/🦀️.rs:22 |
| E0603 | trait `FromValue` is private | /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️bounded-reload/🦀️.rs:26 |
| E0603 | enum `DslValue` is private | /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️bounded-reload/🦀️.rs:27 |
| E0603 | struct `ValueError` is private | /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️bounded-reload/🦀️.rs:27 |
| E0603 | trait `FromValue` is private | /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️bounded-reload/🦀️.rs:28 |
| E0603 | trait `ToValue` is private | /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-owned-media-error/🦀️.rs:16 |
| E0603 | trait `FromValue` is private | /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-owned-media-error/🦀️.rs:16 |
| E0603 | struct `ValueError` is private | /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-owned-media-error/🦀️.rs:20 |
| E0603 | trait `ToValue` is private | /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-declarations-fixture/🦀️.rs:363 |
| E0603 | trait `FromValue` is private | /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-declarations-fixture/🦀️.rs:363 |
| E0603 | trait `ToValue` is private | /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-declarations-fixture/🦀️.rs:364 |
| E0603 | trait `FromValue` is private | /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-declarations-fixture/🦀️.rs:364 |
| E0603 | trait `ToValue` is private | /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🧪️tests/🔗️dependency-contribution-unit/🦀️.rs:14 |
| E0603 | enum `DslValue` is private | /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️extension-retirement/🦀️.rs:31 |
| E0603 | enum `DslValue` is private | /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️extension-retirement/🦀️.rs:38 |
| E0603 | enum `DslValue` is private | /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️extension-retirement/🦀️.rs:171 |
| E0603 | enum `DslValue` is private | /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️extension-retirement/🦀️.rs:183 |
| E0603 | trait `FromValue` is private | /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧩️composition/🦀️.rs:143 |
| E0603 | trait `FromValue` is private | /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧩️composition/🦀️.rs:161 |
| E0603 | trait `FromValue` is private | /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧩️composition/🦀️.rs:825 |
| E0603 | trait `FromValue` is private | /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧩️composition/🦀️.rs:836 |
| E0603 | trait `ToValue` is private | /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📄️natural-file-lifecycle/🦀️.rs:83 |
| E0603 | enum `DslValue` is private | /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📄️natural-file-lifecycle/🦀️.rs:93 |

The two retired field-error sites require the canonical typed diagnostic constructor preserving the actual field/span/error rather than restoring `__rt`, a forwarding alias, or a String fallback. Parent owns that production repair.

## Qualification

This report compares actual compiler diagnostics and source snapshots while concurrent edits continue. The machine roster records exact current snippets for the 54 measured pairs. No native law ran successfully in either failed prerequisite replay; no eight-kind guest refusal runtime RED/GREEN or whole-module proof follows from these source repairs. Rerun the unchanged existing guest selector after authoritative callers are paired.
