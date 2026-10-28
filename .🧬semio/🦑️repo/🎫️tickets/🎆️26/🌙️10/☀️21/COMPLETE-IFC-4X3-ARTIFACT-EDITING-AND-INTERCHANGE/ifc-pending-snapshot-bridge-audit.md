# Pending Snapshot Preparation Bridge Audit

Fresh source confirms pending_effects in C:/git/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🔌️plugin-app-impl/🖼️render-surface/🦀️.rs calls the old two-argument boolean preparation hook. Current ArtifactApp/ArtifactEditor/ArtifactViewer signatures take operation, snapshot, &WindowConfigOwnerRegistry, Option<&ViewModel>, RetainedCloneGrant and return MountedSnapshotReadPreparation. EditorApp and ViewerApp already forward all five arguments.

Minimal correction: compute MountedSnapshotReadPreparation::grant(self.mounted_policy.preparation,cx); if no grant, return empty effects without calling preparation. Call the five-argument hook using &self.stores.window_config_store and unchanged view. Call preparation.record(grant,cx)? before issuing any store snapshot read. Only ready=true obtains Some(store.snapshot_read()); false continues pending_effects with None, preserving its bookkeeping/poll behavior. Never interpret the struct itself as boolean or ignore progress on ready=false.

The actual field type is WindowConfigOwnerRegistry in 🧵️instance/🏪️stores/📥️staging/🧱️components/🦀️.rs:72. The grant intersects policy preparation with caller retained authority and remaining fuel. record checks every retained axis and copied-items fuel, debits original progress/fuel even on an excess-authority error, then returns readiness. No new context or policy enlargement.

ArtifactPendingView borrows committed snapshot, original envelope id, child root, owner, poll run and provisional generation. source_version contains base revision, generation, full canonical revision; take_snapshot_read transfers exactly once via Mutex<Option<SnapshotRead>>. It does not materialize renderer history or clone document source. A worker must retain admitted lease and retire it through the existing Store read custody/return machinery; readiness is not permission to convert it into a fresh unrestricted snapshot.

Existing exact mounted boundary test: C:/git/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👁️view/📸️preparation/🧪️tests/🚪️public/🦀️.rs. PreparationApp implements current hook; real VcsArtifactApp constructed with fixture_mounted_policy + fixture_identity; settle_store_constructor; PluginApp::pending_effects under supplied StepContext. Neutral fixture covers readiness, each grant/progress axis, fault outcomes; asserts original progress, exact admitted grant, original envelope pointer/source version, pre-admission heap observation, successful take only when ready; zero-fuel callback count zero. close_registered_fixture_app asserts terminal emptiness. These authored tests are feasible regression target but not executed natively in this audit.

No change to normal render lease admission is implied: its ArtifactView still has None. No ordinary UI deadline/fuel value is established here.

Unused lease outcome: 🌱️value/📸️snapshot/👁️read/🦀️.rs SnapshotRead::Drop calls return_snapshot_read, returning its original alias/lease to the Store registry. Thus a ready=true read not taken by pending_effects is returned automatically at view drop, without copying the document. A retained worker should use explicit witnessed return/controlled Store retirement when it needs closure proof. take_snapshot_read twice fails closed.


## Applied Mounted Bridge Correction

The actual PluginApp pending poll now obtains `MountedSnapshotReadPreparation::grant` from its existing preparation policy and original caller StepContext, passes the current snapshot, window registry, view, and grant to the five-argument app hook, and records the returned preparation receipt before acquiring any Store snapshot lease. A successful not-ready receipt still calls the pending hook with no lease; denied grants return without invoking it. No independent work context or replacement grant is minted.

The existing public mounted preparation law fixtures cover this actual pending poll boundary, readiness, original parent identity/source version, all grant axes, zero fuel, receipt failures, and heap observations. Their native execution remains gated by the canonical repo build preparation failure recorded in `ifc4-continuation.md`; they have not been claimed as passing. The modified source passed the installed Rust grammar parser and a fresh read-only peer review. Grammar validation does not establish Rust type correctness.

Changed source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🔌️plugin-app-impl/🖼️render-surface/🦀️.rs`.

## Fresh Patch Peer Review

Fresh patched body matches the five-argument trait, records original preparation receipts before optional lease issuance, forwards exact window registry/view, continues false readiness with None, and returns before callback when no grant. No visible borrow/type mismatch; native tests were not executed by this audit.
