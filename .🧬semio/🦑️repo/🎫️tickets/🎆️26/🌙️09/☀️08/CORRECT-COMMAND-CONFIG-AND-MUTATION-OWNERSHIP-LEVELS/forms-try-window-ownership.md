# Forms Try Window Ownership

## Current result

Forms Try navigation now owns `currentStepIndex` in the caller's exact Try window config, and unfinished `tryValues` live in that same window's transient owner. The Forms app config retains only `contributionsJson`, which remains the host contribution projection.

Single-value and bulk continuation state is keyed by app, document, semantic operation, operation id and generation, canonical base revision, input id, exact window id, exact window kind, window generation, and document generation. Retained dispatch validates the concrete view and lease before it reads or publishes window state. Exact-window reset cancels and clears only that lease. Direct entry points that require retained transient context fail closed.

The mounted empty Forms presence owner was removed. Forms now uses the framework `NoPresence`, `NoPresenceMutation`, `NoTransient`, and their no-state lifecycle factories directly.

## Validation boundary

- **Runtime verified:** `bun nx run workspace:forms-try-window-ownership-oracle` passed on 2026-09-12. Ajv 2020 independently admits both exact-window config/transient schemas, and fast-json-patch independently applies the two-window fixture. The oracle proves left advance/stage/reset, right isolation, document byte isolation, JSON config reload, exact continuation identities, and cleared transient.
- **Native runtime verified:** isolated ticket target `abstraction-ownership-validation:forms-try-window-ownership-native`, session 2760, passed all three laws on 2026-09-12. The registry law proved two same-kind Try windows isolate navigation and staged values, exact config reload restores the caller window, reset clears only that transient and lease, the other continuation resumes, document/app bytes remain unchanged, and both apps close terminal-empty. The neutral mutation inverse/text/binary and pure two-window laws also passed. Exact log: `🗑️generated/forms-try-window-ownership-native-5.log`.
- **Earlier native runtime finding:** session 20319 compiled the Forms crate; two laws passed, while the registry setup exposed a one-row declaration for a forward-plus-inverse durable edit and absent document/config owners and disposers. The successful run includes the canonical two-row point-invertible footprint and bounded owner/disposer bindings.
- **Native rerun infrastructure block:** session 9024 exited before Cargo because Nx rejected concurrent WGPU taxonomy changes whose package-generation paths and input patterns were not unique and byte ordered. This attempt produced no Forms compiler or runtime evidence. Its exact log is `🗑️generated/forms-try-window-ownership-native-4.log`.
- **Earlier local compile findings:** session 85227 found a duplicate exact-config `MutationDiff` implementation, incorrect text-error conversion, a bulk decode iterator lifetime, and mutable access in the transient test helper. All five were fixed before session 20319 compiled successfully.
- **Earlier native attempt:** session 86299 exited before the Forms crate compiled because a concurrently edited shared fixture factory had unqualified `SpaceMember` and `MemberFactory` bounds. The recursive SDK owner subsequently store-qualified those shared declarations.
- **Native law source ready:** the explicit 8 MiB test thread creates two same-kind Try windows, advances and stages the left, reloads exact window config, resets the left, then resumes the right continuation. It asserts the other window, document pack, and app contribution pack remain unchanged and closes both registry-backed apps through the bounded lifecycle helper.
- **Static audit:** app config schemas contain neither `currentStepIndex` nor `tryValues`; the retired `FormsPresence` type is absent; Try production codecs use the internal DSL JSON codec; JSON and Rust enforce 64 values, 512-byte keys, and 4,096-byte chunks.

## Exact file ledger

### Exact Try owners, schemas, oracle, and native law

- `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🎚️config/🦀️.rs`
- `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🎚️config/🧬️schema/{🔣️.json,🟦️.ts,🦀️.rs,🔗️.graphql,🛰️.proto}`
- `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🫧️transient/🦀️.rs`
- `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🫧️transient/🧬️schema/{🔣️.json,🟦️.ts,🦀️.rs,🔗️.graphql,🛰️.proto}`
- `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🎚️config/🧫️fixtures/🔬️window-ownership/🔣️.json`
- `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🎚️config/🧪️tests/🔬️window-ownership/{🟦️.ts,🦀️.rs}`

### Forms app config and empty-owner retirement

- `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs`
- `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/{🔣️.json,🟦️.ts,🦀️.rs,🔗️.graphql,🛰️.proto}`
- `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/🦀️.rs`
- `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/📸️replace-config/🧬️schema/🔣️.json`
- Deleted the three-file mutation leaves `{🔣️.json,🦀️.rs,🧬️schema/🔣️.json}` under app-config mutations `✅️commit-try-values-batch`, `🎯️commit-try-value`, `👣️set-step-index`, `📦️stage-try-value-chunk`, `🔎️verify-try-value-chunk`, `🗃️stage-try-values-entry`, `🗑️discard-try-value-staging`, `🧹️discard-try-values-batch`, and `🧽️clear-try-values`.
- `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧫️fixtures/🔁️mutation-contracts.json`
- `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️.rs`
- Deleted `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/{🔣️.json,🟦️.ts,🦀️.rs,🔗️.graphql,🛰️.proto}`.

### Commands and rendering

- `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️set-try-value/🦀️.rs`
- `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗃️set-try-values/🦀️.rs`
- `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/{↩️reset-try,◀️previous-step,▶️next-step,✅️submit,📍️set-try-value-step}/🦀️.rs`
- `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🦀️.rs`
- `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🧪️tests/🔬️{unit,control-vectors}/🦀️.rs`
- Removed app-config reset publication from document commands `{🔣️set-spec-json,🩹️patch-questions,📥️set-active-example,✏️patch-step,↔️move-step,➖️remove-step,❓️add-question,🗑️remove-question,🚚️move-question,📃️add-step,🫳️drop-question-kind}/🦀️.rs`.
- Deleted obsolete `🎯️set-try-value/🧪️tests/🔬️unit/🦀️.rs`, `🗃️set-try-values/🧪️tests/🔬️unit/🦀️.rs`, and `🗃️set-try-values/🧫️fixtures/🔑️continuations.json`.
- Updated Forms unit-test literals under editor unit tests, command unit tests, and panel unit tests to the contributions-only app config.

The Forms editor owner also declares the point-invertible durable preparation footprint through `ArtifactStoreOneItemFootprint::for_one_invertible_item` and installs the bounded document/config store owners and disposers required by registry-backed shutdown.

### Nx and launch surfaces

- `📜️script.ts`
- `📋️project.json`
- `.vscode/launch.json`
- `.vscode/🧩️launch.seed.jsonc`

These four shared files also contain concurrent agents' changes. The Forms entries are only the `forms-try-window-ownership-{oracle,native}` routes/targets and launch orders 311.196/311.197.
