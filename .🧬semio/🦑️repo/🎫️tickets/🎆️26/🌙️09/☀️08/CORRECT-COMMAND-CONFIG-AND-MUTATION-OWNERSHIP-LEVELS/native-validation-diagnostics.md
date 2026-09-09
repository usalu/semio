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
# Current Integration Follow-up

The first long native WGPU build reached the Puzzle 3D artifact during the coordinator's schema/diff edits and failed on now-removed diff fields. The next host run compiled those changes and reached Puzzle 2D. It found old graph-framework re-exports for functionality now owned by OS Infinite, plus utility reducer/context and `scene_for` call sites that still used the previous signature. The coordinator repointed the engine exports to the existing Infinite owner; the plugin utility lane owns the reducer/context call sites.

The component-enabled Rewriting run found WindowTransient types/helpers missing from the SDK's public root exports even though default-feature crate checks had passed. The window-state lane has fixed those exports and is validating the assembled Jack runtime. No failed run above is counted as a passing native behavioral test.

## Current Warm Integration Checks

The second all-artifact check compiled the full shared dependency graph but failed on four private graph ValueType references in Jack query admission. The query lane has explicitly exported the existing graph-owned type from its owning manifest module; the third all-51-artifact check is now running. Native diff creation/application laws have not run yet.

The third host build exposed a deleted Puzzle5D utility module registration and a Puzzle2D retained reducer signature missing the view context; the utility lane corrected both. The third window-action build then found Puzzle3D's public plugin registration using a private job-kind constant. The precompute owner now exposes that constant. Host/action regressions still require a successful rerun.

The first Rewriting window-config run failed with missing new mutation descriptors and routine imports/context updates. Those are fixed, and a second run now includes a real two-window retained publication/render/reload regression. The independent Rewriting JSON Patch/Ajv oracle passes; native runtime behavior remains unverified until this test completes.

The third all-artifact check failed after 9m38s on seven stale Puzzle2D test imports. Those now import `empty_puzzle2d_snapshot` from the real standard/subset schema owner, and a fourth check is pending. The second Rewriting run reduced the compiler errors to one crate-private body key missing from the re-export; that key now has crate visibility and the third run is pending. Its binary command catalog now explicitly exposes the two registered window-configuration tools.

The fourth host run reached the WGPU renderer and reported 50 errors: 45 arose from reaching the shared Board engine through the Puzzle plugin's deleted editor re-export; one from newly required surface context fields in an old queue fixture; three from a removed job accessor in existing recovery tests; one from the newly introduced `ArtifactEvent::DocumentBackbone` variant absent from the shell event match. The Board references now point directly to the existing OS Infinite dependency. Queue/recovery fixtures were updated to the actual owner types without adding compatibility APIs. The DocumentBackbone consumer belongs to the concurrently changing native transport and still needs its exact delivery route before the host run can pass.

Updated Board-owner sources: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs`, `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🔬️wgpu-board2d-engine/🦀️.rs`, and `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs`. Updated fixture: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️wgpu-renderer-kernel-runtime-semantic-document/🦀️.rs`.

## Channel v15 browser worker and Rewriting retry

The Channel v15 browser worker retry completed successfully (Nx exit 0, one selected Vitest test passed). Its runtime assertions covered live OS preference refresh without artifact/config frontier changes, stale opening rejection, serialized actor turns, and cold document ingress/acknowledgement. The first retry observed a concurrent fixture transition; the second used the current authored fixture without a local workaround.

Rewriting native retry 3 compiled past the earlier Rewriting errors but failed on twelve Jack camera/LOD consumer errors during the active Jack window-config migration. Native Rewriting behavior remains unverified pending its next completed run.

## Broad native law run

The first 52-crate committed-diff law run failed during compilation after 11m53s because Norm's shared app surface invoked `TextBuilder::try_id` without the `HasBase` trait import. Added the precise existing first-party trait import in `✏️s/🔌️plugins/📕️norm/🖥️app-surface/🦀️.rs`. The second run is queued. No committed-diff runtime pass is claimed yet.
# Latest Native Checkpoint

Rewriting window-config native run 4 compiled and ran three tests: the trusted concrete-window command test passed, the neutral mutation trace failed because the Rust enum had no internally tagged `kind` codec annotation, and actual app closure failed because Rewriting supplied no document-store disposer. The annotation now matches the canonical JSON/TypeScript contract. Rewriting now explicitly installs cursor retirement for document roots and all seven semantic mutations, plus exact production NoConfig/NoDraft/NoPresence/NoTransient providers. Generic ordered-map retirement belongs to OS Store; recursive graph-property retirement belongs to Graph Manifest. Their independent two-snapshot/seven-mutation byte oracle passed; native run 5 is queued and not yet a pass.

The broad artifact native law run 2 stopped at compilation after 15m58: FEM2's core model uses `dyn_enum` from a missing framework dispatch-macro dependency, causing thirteen cascading errors. Its actual compile-time dependency is now declared; run 3 is queued. The structural gate still passed for 104 contracts across 52 artifacts.

Wires native run 1 rejected a nonexistent `component-app-assembly` feature; Wires already unconditionally includes its editor. The verification target now uses its actual feature configuration, and native run 2 is compiling.

## Exact Retirement and Cache Shutdown

Shared document retirement native run 2 completed successfully through Nx. Its captured executable receipt selected and ran both exact laws: `owned_retirement_matches_neutral_exact_byte_grants` and `owned_retirement_rejects_false_terminal_and_preserves_shared_roots`; each passed with zero failures. The independent oracle covered nine cases, three budgets, 1,301 exact byte grants, and five hostile cases. The native binary SHA-256 was `876c5b58613f2f853e11bd37b645ed9fa9357c984f4261ff6b1332a7fe063802`.

Rewriting native run 6 compiled and ran four tests. Trusted command routing and exact document retirement passed. Neutral state comparison incorrectly compared integer JSON numbers with equivalent floating-point encodings; it now compares independently decoded typed expected states. Actual app closure blocked because the framework projection cache retained document/config roots until after those stores had to retire. The SDK now releases that cache in bounded steps before store disposal. Both runtime fixture apps now bind real instance IDs. Native run 7 is pending; the complete runtime scenario is not yet verified.

Wires native run 3 failed on a stale crate-root module mount after removal of the unused presence lane. Removed that mount and placed its explicit document/config retirement providers on `ArtifactEditor`, their actual trait owner. The runtime fixture now binds its live instance ID. Native run 4 is pending.

The all-artifact committed-diff run 3 failed during the active Writer configuration migration. Its broad retry waits for Writer's source transition to finish; no broad native-law pass is claimed.
