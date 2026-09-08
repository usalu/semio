# Native SDK Ownership Validation Diagnostics

The surface-context SDK test build failed with 1,061 diagnostics. The primary unrelated infrastructure failures are 16 mutation-leaf and 13 mutation-collection source-authority rejections in existing SDK test fixtures. Those missing derives cascade into hundreds of missing Mutation bounds. Ownership-related ViewModel import and handle-signature diagnostics are separately assigned to the plugin cleanup lane. No native runtime pass is claimed.

## Rejected Fixture Owners

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🛰️declaration-channels-1-standard-1-any-mutations-set-value/🦀️.rs`: MutationLeaf source authority failed: source is neither a flat mutation owner nor one registered domain operation
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🛰️declaration-channels-1-standard-1-any-mutations/🦀️.rs`: Mutations source authority failed: aggregate source is not directly inside the taxonomy mutation collection
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🛰️declaration-channels-1-standard-1-strict-mutations-set-value/🦀️.rs`: MutationLeaf source authority failed: source is neither a flat mutation owner nor one registered domain operation
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🛰️declaration-channels-1-standard-1-strict-mutations/🦀️.rs`: Mutations source authority failed: aggregate source is not directly inside the taxonomy mutation collection
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🛰️declaration-channels-2-standard-2-any-mutations-set-value/🦀️.rs`: MutationLeaf source authority failed: source is neither a flat mutation owner nor one registered domain operation
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🛰️declaration-channels-2-standard-2-any-mutations/🦀️.rs`: Mutations source authority failed: aggregate source is not directly inside the taxonomy mutation collection
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🧬️mutation-plan/🧪️tests/🧬️job-test-mutations-mutations-add-value/🦀️.rs`: MutationLeaf source authority failed: source is neither a flat mutation owner nor one registered domain operation
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🧬️mutation-plan/🧪️tests/🧬️job-test-mutations-mutations/🦀️.rs`: Mutations source authority failed: aggregate source is not directly inside the taxonomy mutation collection
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📢️publication-fixtures-presence-mutations-change-publication-presence/🦀️.rs`: MutationLeaf source authority failed: source is neither a flat mutation owner nor one registered domain operation
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📢️publication-fixtures-presence-mutations/🦀️.rs`: Mutations source authority failed: aggregate source is not directly inside the taxonomy mutation collection
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📢️publication-fixtures-transient-mutations-change-publication-transient/🦀️.rs`: MutationLeaf source authority failed: source is neither a flat mutation owner nor one registered domain operation
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📢️publication-fixtures-transient-mutations/🦀️.rs`: Mutations source authority failed: aggregate source is not directly inside the taxonomy mutation collection
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🖥️test-app-mutations-config-mutations-change-test-config-selection/🦀️.rs`: MutationLeaf source authority failed: source is neither a flat mutation owner nor one registered domain operation
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🖥️test-app-mutations-config-mutations/🦀️.rs`: Mutations source authority failed: aggregate source is not directly inside the taxonomy mutation collection
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🖥️test-app-mutations-document-mutations-set-test-count/🦀️.rs`: MutationLeaf source authority failed: source is neither a flat mutation owner nor one registered domain operation
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🖥️test-app-mutations-document-mutations-set-label/🦀️.rs`: MutationLeaf source authority failed: source is neither a flat mutation owner nor one registered domain operation
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🖥️test-app-mutations-document-mutations/🦀️.rs`: Mutations source authority failed: aggregate source is not directly inside the taxonomy mutation collection
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📡️contributed-mutation-wire-mutations-add-value/🦀️.rs`: MutationLeaf source authority failed: source is neither a flat mutation owner nor one registered domain operation
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📡️contributed-mutation-wire-mutations/🦀️.rs`: Mutations source authority failed: aggregate source is not directly inside the taxonomy mutation collection
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures-dummy-mutations-set-dummy-count/🦀️.rs`: MutationLeaf source authority failed: source is neither a flat mutation owner nor one registered domain operation
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures-dummy-mutations/🦀️.rs`: Mutations source authority failed: aggregate source is not directly inside the taxonomy mutation collection
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures-surface-mutations-set-surface-count/🦀️.rs`: MutationLeaf source authority failed: source is neither a flat mutation owner nor one registered domain operation
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures-surface-mutations/🦀️.rs`: Mutations source authority failed: aggregate source is not directly inside the taxonomy mutation collection
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures-transaction-mutations-set-transaction-count/🦀️.rs`: MutationLeaf source authority failed: source is neither a flat mutation owner nor one registered domain operation
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures-transaction-mutations-set-transaction-count-and-notify/🦀️.rs`: MutationLeaf source authority failed: source is neither a flat mutation owner nor one registered domain operation
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures-transaction-mutations-set-transaction-count-without-preflight/🦀️.rs`: MutationLeaf source authority failed: source is neither a flat mutation owner nor one registered domain operation
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures-transaction-mutations/🦀️.rs`: Mutations source authority failed: aggregate source is not directly inside the taxonomy mutation collection
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🧪️tests/🔗️dependency-contribution-mutations-add-value/🦀️.rs`: MutationLeaf source authority failed: source is neither a flat mutation owner nor one registered domain operation
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🧪️tests/🔗️dependency-contribution-mutations/🦀️.rs`: Mutations source authority failed: aggregate source is not directly inside the taxonomy mutation collection

## Canonical Fixture Source Ownership Fix

Moved the 29rejected Rust source files beside their already-authored canonical mutation descriptors. No new descriptors, relaxed authority checks, or duplicate APIs were introduced. Rust module identities remain unchanged; source path and include paths were rebased to their concrete targets. The existing metadata/law fixtures now point to their actual owners. Compilation will be rerun after the current host check. Files moved or referencing changed paths:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🧬️mutation-plan/🧪️tests/🧬️job-test-mutations-mutations-add-value/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🧬️mutation-plan/🧪️tests/🧬️job-test-mutations/🧬️mutations/➕️add-value/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🧬️mutation-plan/🧪️tests/🧬️job-test-mutations-mutations/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🧬️mutation-plan/🧪️tests/🧬️job-test-mutations/🧬️mutations/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🧬️mutation-plan/🧪️tests/🧬️job-test-mutations/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🧪️tests/🔗️dependency-contribution-mutations-add-value/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🧪️tests/🔗️dependency-contribution/🧬️mutations/➕️add-value/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🧪️tests/🔗️dependency-contribution-mutations/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🧪️tests/🔗️dependency-contribution/🧬️mutations/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🧪️tests/🔗️dependency-contribution/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🛰️declaration-channels-1-standard-1-strict-mutations-set-value/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🛰️declaration-channels/1️⃣standard-1/🔒️strict/🧬️mutations/📝️set-value/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🖥️test-app-mutations-config/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📢️publication-fixtures-presence/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures-surface-mutations-set-surface-count/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures/🪟️surface/🧬️mutations/📝️set-surface-count/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🖥️test-app-mutations-document-mutations-set-test-count/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🖥️test-app-mutations/🧬️document/🧬️mutations/📝️set-test-count/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📡️contributed-mutation-wire-mutations/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📡️contributed-mutation-wire/🧬️mutations/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures-dummy-mutations-set-dummy-count/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures/🎲️dummy/🧬️mutations/📝️set-dummy-count/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🛰️declaration-channels-1-standard-1-any-mutations/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🛰️declaration-channels/1️⃣standard-1/🌐️any/🧬️mutations/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures-transaction-mutations-set-transaction-count-without-preflight/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/⏩️set-transaction-count-without-preflight/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures-transaction/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🛰️declaration-channels/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures-dummy/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🖥️test-app-mutations-document-mutations-set-label/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🖥️test-app-mutations/🧬️document/🧬️mutations/🏷️set-label/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🛰️declaration-channels-1-standard-1-any-mutations-set-value/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🛰️declaration-channels/1️⃣standard-1/🌐️any/🧬️mutations/📝️set-value/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures-transaction-mutations-set-transaction-count-and-notify/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/📣️set-transaction-count-and-notify/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures-transaction-mutations-set-transaction-count/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/📝️set-transaction-count/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📢️publication-fixtures-presence-mutations/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📢️publication-fixtures/👥️presence/🧬️mutations/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures-surface/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🖥️test-app-mutations-config-mutations/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🖥️test-app-mutations/🎚️config/🧬️mutations/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📡️contributed-mutation-wire-mutations-add-value/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📡️contributed-mutation-wire/🧬️mutations/➕️add-value/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📢️publication-fixtures-transient-mutations-change-publication-transient/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📢️publication-fixtures/🫧️transient/🧬️mutations/📝️change-publication-transient/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures-dummy-mutations/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures/🎲️dummy/🧬️mutations/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🖥️test-app-mutations-config-mutations-change-test-config-selection/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🖥️test-app-mutations/🎚️config/🧬️mutations/📝️change-test-config-selection/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📢️publication-fixtures-presence-mutations-change-publication-presence/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📢️publication-fixtures/👥️presence/🧬️mutations/📝️change-publication-presence/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures-transaction-mutations/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📢️publication-fixtures-transient-mutations/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📢️publication-fixtures/🫧️transient/🧬️mutations/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📢️publication-fixtures-transient/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🛰️declaration-channels-2-standard-2-any-mutations/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🛰️declaration-channels/2️⃣standard-2/🌐️any/🧬️mutations/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🛰️declaration-channels-1-standard-1-strict-mutations/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🛰️declaration-channels/1️⃣standard-1/🔒️strict/🧬️mutations/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🖥️test-app-mutations-document-mutations/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🖥️test-app-mutations/🧬️document/🧬️mutations/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🛰️declaration-channels-2-standard-2-any-mutations-set-value/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🛰️declaration-channels/2️⃣standard-2/🌐️any/🧬️mutations/📝️set-value/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🖥️test-app-mutations-document/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📡️contributed-mutation-wire/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures-surface-mutations/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures/🪟️surface/🧬️mutations/🦀️.rs`

## Warm SDK Retry

The second native compilation reduced the 1,061 cascading diagnostics to 10 errors. Nine concern the explicit host-context parameter added to app fixture traits/calls; the execution agent is updating them. The remaining error called a dependency's `cfg(test)` method from another crate's test. The binding test now sends through the production `BackboneChannelPort::send` API, exercising the real channel and keeping test-only dependency helpers private.

Updated: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📡️backbone/🔗️binding/🧪️tests/🔬️unit-standalone/🦀️.rs`. Tests are pending the next SDK retry.

## Native Surface Suite Executes

The fourth native attempt compiled successfully and executed five tests. Four passed: retained host context, capacity recovery, real app render/hidden-surface refusal, and focused-window-to-panel projection. Runtime debug messages confirmed the real app render and panel projection. The presence regression failed during setup because `setLabel` has no exact factory proof in the interaction test registry; unwinding also hit a pre-existing disposer assertion. The regression now seeds a peer-presence root and renders an interaction-bound tree directly, so it tests actual peer-derived presence and surface binding without using an unrelated undeclared command. The full suite is awaiting rerun.

## Native Surface Suite Passes

The fifth Bun+Nx surface run passed all five tests: 5 passed, 0 failed, 566 filtered, runtime 0.06 seconds after compilation. Console evidence confirmed retained locale and independent window utilities, real app render with exact surface context, hidden-surface refusal, panel isolation from focused window state, and peer-derived presence addressed separately to each concrete window and panel. This validates the native SDK runtime boundary; it does not claim a full native GUI launch.

## Native Menu and Window Contracts Pass

Bun+Nx ran both `context-menu-view-test` and `window-view-native-test` successfully. The SDK menu wire test requires explicit preference context, projects the addressed concrete window, clears panel fields, and rejects unknown windows. The current Rust window-instance fixture test passed (one test, 216 filtered), matching the shared TypeScript fixture including panel isolation and independent sibling utility values.
