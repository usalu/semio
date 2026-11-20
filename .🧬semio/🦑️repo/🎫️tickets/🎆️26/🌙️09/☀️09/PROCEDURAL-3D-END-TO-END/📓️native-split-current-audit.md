# Native Split Current Audit

Read-only source and existing-log inspection; no native build was started.

- The final Split commit requires two work items in a single turn: `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/🧠️session-core/🦀️.rs:1214` returns empty progress when `maximum_items < 2`. Its `solid_pair_demand` (1186–1194) quotes capacity and depth, without expressing that minimum. If the caller supplies a persistent one-item grant, the job stalls indefinitely. Verify the actual operator grant contract and add an explicit admission guarantee or make the commit schedulable, with a one-item-budget cancellation/progress test. This is a conditional source finding, not a reproduced runtime failure.
- Paired cancellation closes staged claims before closing unpublished solid transactions in `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/〰️analytic-curves/✂️split/🦀️.rs:41–60`. Both claims are staged before `publish_solid_pair` under the session locks. No independent cancellation defect was established by this read.
- `🗑️generated/brep-linked-native-session-split.log` records an unresolved `semio_framework_3d_brep_spline_trim` import, but current `native-registry/📦️packages/🦀️rust/Cargo.toml:28` declares it. That log predates the current source and is not evidence of a present type error. Other native logs similarly represent earlier revisions; no current complete native build result was available in this audit.
- `native-registry/🧊️solid-publication/🦀️.rs:123–147` plans and publishes two certified boxes. Its supported source is explicitly a certified box cut by an axis plane, so general Split remains a feature limitation, rather than evidence of a type regression.

## One-item grant follow-up

The shared `🧪️tests/🔬️unit/🧰️original-support.rs:80–95` query runner grants exactly one item per step and requires completion; a search of the current unit sources found no Split-specific invocation. At commit the session calls `effect.performed(2)`, then `FoldIndex::insert_vacant_pair_reserved` (`🧰️framework/🔨️modules/📡️replication/🔗️causal/🔀️transition/🔁️fold/🗂️index/🦀️.rs:91–131`) independently requires two items and returns two copied items. It reserves backing and validates distinct vacant keys before two infallible fixed row insertions. An atomic bounded pair could structurally be accounted as one event only if the retained-work contract allows it; both the effect admission and index receipt must agree, and this accounting decision requires verification.


## Atomic one-item commit

Split preadmits two vacant native index slots and stages two inactive authority claims through separate turns. Once both constructed boxes and their output dictionaries are ready, a single publish event owns the session locks, validates both products and distinct vacant rows, transfers both rows without allocating or copying payload bytes, then activates both claims. It records one performed work unit and one structural item. The paired index checks all preconditions before either insertion. Cancellation before that turn still closes each claim and unwinds each box independently.

The registered `original-brep-solids` Split test drives publication with exactly one retained item per turn. It requires both live handles to appear on the same completion turn, measures both named outputs, and checks that cancelling after both claims stage restores the original live index and solid arena counts. The language-neutral fixture covers translated X/Y/Z cuts, reversed orientation, unequal volumes and invalid planes. The Manifold and Three.js suites passed nine tests with 83 assertions.

The repository source-closure reader reported 3,999 active lines for the BRep engine, 4,000 for the native registry, 254,000 active bytes for the spatial session, and 158,704 for Flow BRep. The original solids native run reached compilation and reported a missing `LabelSource` path in the new native Split method. The source now uses the compiler-suggested qualified path. Native rerun remains pending during the active tessellation build.
