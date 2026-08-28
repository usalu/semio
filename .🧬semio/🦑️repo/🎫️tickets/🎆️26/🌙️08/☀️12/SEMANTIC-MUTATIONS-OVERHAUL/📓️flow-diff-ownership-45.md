# Flow Diff Ownership 45

## Planned Boundary

The current ordered diff clones a complete retained Flow fixture before validating its fragments. Removing or replacing a widget immediately drops that cloned widget, and a later rejection requires synchronous cold disposal of the candidate. Nonempty ordered dictionary/set owners make the ordinary drop path invalid.

The replacement uses a borrowed projection of the supplied snapshot and diff. Removal and replacement manipulate references; layout assignments retain borrowed keys and values. Every fragment is validated before the final successful fixture is materialized. Rejections therefore own no Flow widget, dictionary, set, layout root, or candidate fixture to retire. Successful materialization copies only the final selected payloads, once. The public fallible MutationDiff contract and ordered structural diff schema do not change.

This is mutation-owned collection/application work only. The separate mutation-retirement struct/factory remains assigned to its existing lane. Neighboring FlowFixture/FlowSnapshot retirement, retained VCS runtime, SharedRegistry, SetContributions, Kernel and Plugin lifecycle are excluded. No shared ownership primitive edit is planned.

## Test-First Acceptance

Language-neutral schema and vectors cover retained payload removal/replacement, cancellation by later fragments, full fixture replacement, layout assignments, and late rejection. An independent jsonc-parser edit reference must reproduce complete successful snapshots. Native tests will exercise the actual diff and prove collection validation does not clone or drop payload owners. Native execution remains held; source/neutral evidence is not a Rust runtime claim.

Partial serde/DSL decode failure cleanup is a separate unresolved boundary. This change does not claim that decoding or the entire Flow owner is ready.

## Implemented and Executed

The schema-adjacent `📑️projection/🦀️.rs` now owns only borrowed payload references. The existing collection helper has no Clone bound and accepts `Vec<&T>` plus a borrowed structural delta. Layout validation was moved into that same borrowed projection. `FlowDiff::apply` validates all fragments, then materializes once; it no longer clones an initial candidate, drops removed/replaced widgets, or invokes cold retirement on rejection/import replacement. Public result codes, target prefixes, fragment ordering and diff wire representation are unchanged.

The eleven neutral cases carry actual nonempty neural dictionaries and ordered sets. Six successful complete snapshots agree with the independent jsonc-parser structural-edit reference; five ordered failures match their explicit error codes. Every case verifies unchanged source and diff inputs.

- Test-first source/schema run `🧪️flow-vcs-direct-41/🧫️run-biT0K8`: 324/331, exit 1. Six failures were the not-yet-implemented projection/mount contracts. One independent protection assertion detected that the retirement lane's newly added module mount preceded its granted interval. That lane moved only its authored mount inside the interval; the immutable adjacent fixture/snapshot prefix hash was restored by exact equality, without repinning or modifying neighboring code.
- Author integration replay `🧫️run-6fTr5L`: 331/331, exit 0.
- Root independent replay `🧫️run-REtKHc`: 331/331, exit 0, all captured input hashes stable.
- Exact edited-path `git diff --check`: exit 0.

Two additional native tests are mounted: `retained_payload_projection_matches_neutral_vectors` and `collection_validation_never_clones_or_drops_payloads`. The latter uses actual clone/drop counters and reference identity on the generic collection implementation. Neither has been compiled or executed. No Cargo, rustc, target/cache cleanup or shared primitive edit occurred.

## Root Replay Hashes

- VCS component: `1b3491d2c10bfc38e900648a5dc9add59666ed8d97b94f5e1c7ebc6557c9eac5`
- Ordered diff: `0de68879af253b56040adc2671ba3bcb92de072e2359beded26115fb32ed9c80`
- Borrowed projection: `0bf747be274ddaf9b21281e810ed00fe26cc9084edfc4e6aa1cc5b7d1b04585b`
- Neutral ownership vectors: `919e73dbe97dd02f7b03fcb0d4fe771193d557ed91b64bcdccbb64be08a52727`
- Vector schema: `b18b87894c6af9881a78a085b5ac3ef796a798b3a5d3b1d0edf0ff9039f5610d`
- Native ownership tests: `f348e896198cf01a7bb8d03f6b360e13b456ad7f64b8d94b344034960d0ca63c`
- Source controller: `08aec6f3939fddfbe6f0f3c7ca4ab86650f6e91c6495ecc51c9f619e20a1940a`

## Remaining Boundaries

This removes the owned intermediate snapshot from diff application; it does not make the synchronous MutationDiff interface resumable. Materialization still uses the existing Clone/collection contracts. Intrinsic serde failure after successful owned-field construction and the fallible DSL conversion chains still require an explicit decode-owner design. Existing unrelated Patchable implementations and retained VCS runtime are not altered. The whole Flow owner, Plugin compilation, publication and the monorepo goal remain incomplete.
