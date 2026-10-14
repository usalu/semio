# Store Constructor Alias Current Read-Only Audit

Read-only source observation at 2026-10-14T18:49:20.185103+00:00. No compiler, tests, frontend, initializer, native capture, formatter, modifying Git, source edit or deletion ran. Sources are being edited concurrently; hashes identify observed bytes only.

## Findings

No remaining injected `ArtifactSnapshotPublication` on a verified Wasmtime defining type was found in the inspected current constructor sources. The current standard-publication scan reaches 98 files in Framework and S. These references include actual ArtifactStore, named aliases and test fixture implementations; lexical matches alone are not a type-validation result.

Plugin Host imports `wasmtime::{..., Store}` at line 66 and its line 2030 call is now `Store::new(&self.engine, host_state)` with no publication generic. Interpreter oracle calls are likewise restored fully qualified `wasmtime::Store::new`. The previous report's Plugin Host standard-publication row is historical, not a remaining ArtifactStore call. Host tests inheriting `super::*` inherit that Wasmtime Store; their restored bare calls must remain without the producer generic.

General `DraftStore<P, Mutation>` (Store root 885) and `ConfigStore<C, ConfigMutation>` (1366) are direct ArtifactStore aliases. Their injected publication selects the correct receiving constructor. InteractionStore and TestStore aliases also explicitly name ArtifactStore. The remaining named alias definitions inspected below name ArtifactStore; no evidence of a renamed Wasmtime alias appears in those defining clauses. Tests with one-argument `ArtifactStore::new` are frequently their own fixture wrappers: the Store unit fixture's `new` at 1473 delegates through `new_with_actor` to `super::ArtifactStore::new::<ArtifactSnapshotPublication>` with two arguments at 1483. They are not automatically missing-producer errors. No blanket claim that every unqualified one-argument caller is safe is made.

Store `new<C:ArtifactStorePublication<P>>` at 2997 forwards `Self::construct::<C>` at 3000 to the identically bounded construct at 3007. C is retained, not inferred or dropped at this binding point. Standard publication is a blanket impl for P:ArtifactStoreSnapshot; named standard Store callers therefore still need that capability. The alias check establishes the method owner, not every concrete P implementation or full compilation.

`create_document_envelope<P,Mutation>` has two named generic parameters and an argument-position `impl ArtifactGenesisProducer<P>` fifth argument. Existing two-parameter turbofish clauses remain correct: the producer is not a third named generic parameter. Its P:Clone bound suffices for its inspected birth/from-genesis sequence. `AdmittedArtifactGenesis::born<G:ArtifactGenesisProducer<P>>` and `from_stored_pack<G>` take the selected producer by reference.

The five original parse/replay helpers (`parse_document_spr`, `parse_decoded_document_spr`, `replay_ops`, `parse_document_text`, `parse_document_pack`) retain exactly two named generic parameters and accept a third borrowed `(impl ArtifactGenesisProducer<P> + Sync)` argument. Internal forwards preserve that argument. SPR decode calls the selected decoder; replay birth calls `born(initial_snapshot, producer)`; text parsing retains its required P:ArtifactDsl bound. There is no necessary third turbofish argument. OSHost's generic decode_backbone_payload has P:ArtifactPack, sufficient for its explicit ArtifactPackGenesisProducer; concrete OS space pack import/write calls select that producer with SpaceSnapshot. These exact clauses establish no static missing producer argument or wrong method-generic count at the inspected binding points.

No new provable compiler/type error was established by this bounded inspection. All 413 genesis and 99 parse calls have not been individually type-resolved; macro expansion, cfg-specific compilation, external dependency closures and native runtime behavior remain unexecuted. The historical 229-file frontier and corrected constructor report remain source rosters, not deletion-safety or compiler acceptance.

## Defining Named Alias Clauses

- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🔨️modules/🏠️host/🧰️owned/🦀️.rs:6`: `pub type Puzzle2dStore = ArtifactStore<Puzzle2dSnapshot, Puzzle2dMutation>;`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs:86`: `pub type RasterStore = store::ArtifactStore<RasterSnapshot, RasterMutation>;`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🔨️modules/🏠️host/🧰️owned/🦀️.rs:6`: `pub type Puzzle5dStore = ArtifactStore<Puzzle5dSnapshot, Puzzle5dMutation>;`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🔨️modules/🏠️host/🧰️owned/🦀️.rs:6`: `pub type Puzzle3dStore = ArtifactStore<Puzzle3dSnapshot, Puzzle3dMutation>;`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚙️operations/🦀️.rs:9`: `pub type RewriteRuleStore = ArtifactStore<RewritingSnapshot, RewriteRuleMutation>;`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs:43`: `pub type GisMapStore = ArtifactStore<GisMapSnapshot, GisMapMutation>;`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚙️operations/🦀️.rs:10`: `pub type TrinityGraphStore = ArtifactStore<JackSnapshot, TrinityGraphMutation>;`
- `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs:11`: `pub type DagStore = store::ArtifactStore<DagSnapshot, DagMutation>;`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🔨️modules/🏠️host/🧰️owned/🦀️.rs:4`: `pub type LayoutStore = store::ArtifactStore<crate::LayoutSnapshot, LayoutMutation>;`
- `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🔨️modules/🏠️host/🧰️owned/🦀️.rs:3`: `pub type DagStore = store::ArtifactStore<crate::DagSnapshot, crate::schema::mutations::DagMutation>;`
- `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚙️operations/🦀️.rs:13`: `pub type VcsStore = store::ArtifactStore<VcsSnapshot, VcsDemoMutation>;`
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs:14`: `pub type Block2dStore = store::ArtifactStore<Block2dSnapshot, Block2dMutation>;`
- `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🔨️modules/🏠️host/🧰️owned/🦀️.rs:3`: `pub type EquationStore = store::ArtifactStore<crate::EquationSnapshot, crate::schema::mutations::EquationMutation>;`
- `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️.rs:11`: `pub type ShootingStore = store::ArtifactStore<ShootingSnapshot, ShootingMutation>;`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🧪️tests/🔬️store-fixture/🦀️.rs:15`: `pub type Generation2dFixtureStore = store::ArtifactStore<Generation2dSnapshot, Generation2dMutation>;`
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs:14`: `pub type Block5dStore = store::ArtifactStore<Block5dSnapshot, Block5dMutation>;`
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs:14`: `pub type Block3dStore = store::ArtifactStore<Block3dSnapshot, Block3dMutation>;`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🧪️tests/🧰️support/🔬️store-fixture/🦀️.rs:14`: `pub type Generation3dFixtureStore = store::ArtifactStore<Generation3dSnapshot, Generation3dMutation>;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🌿️vcs/🦀️.rs:584`: `pub type DagStore = ArtifactStore<DagSnapshot, DagMutation>;`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🔨️modules/🏠️host/🧰️owned/🦀️.rs:8`: `pub type PresentationStore = ArtifactStore<PresentationSnapshot, PresentationMutation>;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🏗️initialization/🔁️replay/🧪️tests/🔬️unit/🦀️.rs:15`: `type Store = ArtifactStore<DemoSnapshot, DemoMutation>;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:885`: `pub type DraftStore<P, Mutation> = ArtifactStore<P, Mutation>;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:1366`: `pub type ConfigStore<C, ConfigMutation> = ArtifactStore<C, ConfigMutation>;`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🧬️mutations/🦀️.rs:100`: `pub type Fem2dStore = ArtifactStore<Fem2dSnapshot, Fem2dMutation>;`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🧬️mutations/🦀️.rs:333`: `pub type Fem3dStore = ArtifactStore<Fem3dSnapshot, Fem3dMutation>;`
- `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🔨️modules/🏠️host/🧰️owned/🦀️.rs:14`: `pub type WriterDocumentStore = store::ArtifactStore<WriterSnapshot, WriterMutation>;`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🔨️modules/🏠️host/🧰️owned/🦀️.rs:9`: `pub type CadStore = ArtifactStore<CadSnapshot, CadMutation>;`
- `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🔨️modules/🏠️host/🧰️owned/🦀️.rs:4`: `pub type FormsStore = store::ArtifactStore<FormsSnapshot, crate::op::FormMutation>;`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚙️operations/🦀️.rs:8`: `pub type SequenceStore = store::ArtifactStore<SequenceSnapshot, SequenceMutation>;`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🔨️modules/🏠️host/🧰️owned/🦀️.rs:3`: `pub type NoteStore = store::ArtifactStore<crate::NoteSnapshot, crate::schema::mutations::NoteMutation>;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/📖️capture/🧪️tests/📖️capture/🦀️.rs:6`: `type InteractionStore = ArtifactStore<InteractionState, InteractionConfigMutation>;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/📡️live/🧪️tests/📡️live/🦀️.rs:7`: `type TestStore = ArtifactStore<protocol::InteractionState, InteractionConfigMutation>;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/📃️query/🧪️tests/📃️query/🦀️.rs:10`: `type InteractionStore = ArtifactStore<InteractionState, InteractionConfigMutation>;`
- `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️.rs:16`: `pub type Process3dStore = ArtifactStore<Process3dSnapshot, Process3dMutation>;`

## Exact SHA-256 Observations

- `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🔨️modules/🏠️host/🧰️owned/🦀️.rs`: `28ab579f1a2a1e1bc82648b3b77b97742a235687af940c0acb79cd5fe4e003bf`
- `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🔨️modules/🏠️host/🧰️owned/🦀️.rs`: `a433c93275942bb6b41f98082100c4d5fd28852e773500c1b69e08f88bc47203`
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/✋️drag/🧪️tests/🔬️unit/🦀️.rs`: `dd8b785503c9472c4089363e6bf927908ea9eb27fc85dcb53550fcfa8357257b`
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧪️brush-tool/🦀️.rs`: `cb3270af28c120b5377aa970c5c8fe3db8ad80099a858724846d1bb770638e4b`
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/✋️drag/🧪️tests/🔬️unit/🦀️.rs`: `fc0984966a683631f348f48de5f1ef84187a0199cc8249f88804ea4bf6720f16`
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🗄️store-fixture/🦀️.rs`: `ef417e8fd939835a5acdeb47d3596143ede24022864bd97e393ff5e6118ca1bd`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🧪️tests/🔬️store-fixture/🦀️.rs`: `186decd912e6245f8f5e8e1aa6504be1fc8370198900cf8dd8f60d21479428d5`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🧪️tests/🧰️support/🔬️store-fixture/🦀️.rs`: `c25b6b3161cf9f12d61b020e15ca87158f1e3a94f4bb6f4704f73e920f52034d`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`: `81e8973091b2a065a0b396cf4d3c1ad565b864d632d9e83f4cb1f5d7dc4a4345`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`: `62888fb767addaf965601a191b552c5459edd64378f953a27a105804b89bcf98`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`: `96f66e8d9081369ae5568c6fc90df6289e196ffdb672729b137942340058e6f5`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🔨️modules/🏠️host/🧰️owned/🧪️tests/🔬️unit/🦀️.rs`: `bb44f30523dee9566832651f06a48ebe9e55c4a894e21e1fb4dc9bf933694ca6`
- `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚙️operations/🦀️.rs`: `136fc6a285025fc1a32e1ccebf2944010e74a5ecea79d089ce72477c1e795b05`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🔨️modules/🏠️host/🧰️owned/🦀️.rs`: `c154f438113f60707e7aa532fe95db75f89dcbdea309dcb125a5af82751d840f`
- `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🧪️tests/🔬️unit/🦀️.rs`: `4eb1c74aac6ee849656d3c5c6af078a4e17ff705b0d02434116650f0038b9b54`
- `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧭️gumball/🧪️tests/🔬️unit/🦀️.rs`: `76fef029ce0b7df03991426898484e35b70d12bcb5c4fa50ef532b18c9f9734f`
- `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`: `f88da266ebd3c38eadfa6e3f2f710262f487c8bc72895b69590a1ddb3e48f05e`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚙️operations/🦀️.rs`: `eee44af05ea3a6af40444e7c6f1c83f2f321c2e3bb51975c2ecccc361cd0a0a9`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🌉️wasm/🧪️tests/🔬️unit/🦀️.rs`: `2eac9d5707918ea4590209e50caea67fa4f52da1ca00f2ea6349c4745fa88119`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🕹️interaction/🧭️gumball/🧪️tests/🔬️unit/🦀️.rs`: `4cb9d0f5226bb8657a4c1dab27405ae506cfb381d9a83cd09c32ed1a6229a563`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`: `75d1d9dd08f32ec545ebabcd3deedcff7381a3eb5f864894ccb90a544229582e`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🕹️interaction/🧭️gumball/🧪️tests/🔬️unit/🦀️.rs`: `b935eae8fd730a7a2b0b3908751b5df59932211fc87bbd36e90bd7d9932a9842`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`: `66776c0337badef7ddce1a941d7074afca8d078345648c140e4af0ada7aea368`
- `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🧪️tests/🔬️unit/🦀️.rs`: `2d2eec716580ff7bd8180d20febd37cb46f579100b9cd62cc37c9af0b7962c52`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧲️transform/🧪️tests/🔬️unit/🦀️.rs`: `6ba210292ef0b985ac8781bc6519b6e18c29e31b2ff4e32322d4ab2b9b81ef29`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`: `7e235ee90d1736b05c72043a23295f6d183a75d5e44bf1667a76ac0b431685ab`
- `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🔨️modules/🏠️host/🧰️owned/🦀️.rs`: `05ff7b396a088e0854f3f7c26f157acec7d89a1394096f041dffbcf06614b12d`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🔨️modules/🏠️host/🧰️owned/🦀️.rs`: `e9a41f3df97a79b364cd2e54352d369459c5886dce6c75f431a09522134e5dd3`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/📦️member-authority/🦀️.rs`: `b3b7a7e810daf41e79154247ad1aafdcedbd5071b6a7c37debe8d310254f886b`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`: `862100cd91c458eac9ff3b9de11bde4d352165ed18979b7bcc3a2424a4d8050f`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🔨️modules/🏠️host/🧰️owned/🦀️.rs`: `c91c1db87b1ce3f45184ce674bc6a998ac92f4cc70345dc3bdb0c40298432807`
- `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs`: `18dfc1d19139c8a7773dfd2773c66a6a41fc99b7673f92ac55fdc64cff0109eb`
- `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🧪️tests/🔬️unit/🦀️.rs`: `a30a0ca2dbf1567af56ccbd8b4e58016caacc7923052db92cb0832f36a63f4af`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`: `1fe06afb53d87d9e1ebdc03a864e0e31ebfa5127cef32deec83d8ac7b04a1369`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`: `07d7f65a7c19150bba9744e34289b773cc8bab931aa3a6eba63dd1b19dfcd2b6`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🪆️child-frame/🦀️.rs`: `135d588be2d77612929693829ef34e265e006a42e2060d2be5571a997a034d5f`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚙️operations/🦀️.rs`: `60da4d98bf4fe9cf8ad8fa9b71ef6aa38df07b1716cdad715396dfca6c5344c9`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs`: `5329150776db57388864e968a9aa3fcd4c4d5b9c4dc417d88980bf302fb56d69`
- `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🔨️modules/🏠️host/🧰️owned/🦀️.rs`: `8159a30b609b8d0d4876eca9e692f1bfc7e730831932204fd7afec4f6f07543c`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📤️export-document/🧵️export/🧪️tests/🔬️unit/🦀️.rs`: `bc359af94e58e5f2ceaf573abfcbfa99209769b20ddba62cc9f7b1c4e768d185`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️geometry/🧪️tests/🔬️unit/🦀️.rs`: `6bc3fb754921a4cf63b29c33f637880b2032c40dfb1549f49ec73d8b776398b2`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs`: `6f33ca12d746e0b280f89c91e09135ae1bc21211472a4ae9ec3ba72a37ed4711`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`: `2beb3117dd174cff19335685381519570220a8e958ed79cc4ac8cddf72348d24`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/📦️owned/🧪️tests/🦀️.rs`: `6266e59839cde59b5068de788ee03025605d28b500a7edda165602f4473f0425`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎬️scene/📋️prepare/🧪️tests/🔬️unit/🦀️.rs`: `16d66d29ccdb5d5681693fca7edc20a7423b0c3bb47983f3ac2856a49100a3c5`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`: `e40de1a9cfdcdb4907cb9f49ef0b8eaf9b3d68cf93e0159b0ee201ec19085f22`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs`: `72e88d2563992a40d9a11afcbcbc945c5bbcb2a5e7cd22adead7484b26cec1e5`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`: `a0edf1da3aa123b0e6b869623f393f8b65712ca643ce3759770a668c44db817d`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌈️apply-filter/🧪️tests/🦀️.rs`: `8c9f5deea06069c11b687f89f46e11fa6ddd53640de74bd102a80539c7c11e41`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️transform-image/🧪️tests/🦀️.rs`: `3dc2087bf0db5401c90dbc4ecf7ac4e1bc09f4c083f380fb7512a5b0bd9ad81f`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖌️paint-stroke/🧪️tests/🦀️.rs`: `b47077b00323bb7062aeeff99ef82df16b0249524f8fe67c7a2329a3ad7790db`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`: `bf3a7ab0052e8b5e0d78a4b6365e65bf331ec42378c5a482e7cdfd8af3f4398d`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪣️fill-region/🧪️tests/🦀️.rs`: `283dea48810ebe86ab866bb15cb198886e8422bdd0af9acf2230b5856eadbae8`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🫗️fill-selection/🧪️tests/🦀️.rs`: `dc0c889c877e3a0ac4253fea463dddd2ae4ed1f3fd55888974f2c0cf87ad123f`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/✏️editor/🧪️tests/🔬️unit/🦀️.rs`: `49ffbb8b896aab0819eea52c273346c8cc9b66da9ccd8d41ff75935c38b9b524`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`: `1a0ee9bff65c938e73204043ddda418df58b176ab4c4c1dc932bfd0a840c757c`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🔨️modules/🏠️host/🧰️owned/🦀️.rs`: `75b11dc33c80120653aba13302d1269f6ccd441d7ad3c96e0f6b77c2b875ad44`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🔨️modules/🏠️host/🧰️owned/🦀️.rs`: `edbb36ead2ba668a570ddf79a9db298360bf06bd4010a312a0cab6ec03a11b6f`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🔨️modules/🏠️host/🧰️owned/🦀️.rs`: `45ab703e2e0867b1d18c859e7477050ca24e84826d8d2efdef42774465cbb0e8`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🔨️modules/🏠️host/🧰️owned/🦀️.rs`: `31f5ef7679d18d0fa5afdecbfbf1a0e7f315c46cb99527c50a13cd80c4d6d87c`
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`: `192030eee581ca2c654b34282a2ddb020474bbfc2af32a50b7af747de945517c`
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`: `f799bacd97f47d8057b7565e8a81138bcfe9f5204f3073d598c6c72b3bb9f962`
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`: `b5c890a7c47ad3322083daae23709817e1042970cd681218499642df38776bb4`
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`: `e50d8cb7e3f84b43833b8e6ad965da8fa82682afb9c52217cff34ca435d8ccda`
- `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`: `40ea480b665d39105fd7a7e4dd97c70e83bfb276d7c91c047b68e462f44578ef`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🌿️vcs/🦀️.rs`: `13e9928a637726fecabf669ebd55167feb81edfda8626c0cb28519bdba0f9837`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🧪️tests/🔬️flow-vcs/🦀️.rs`: `a08559b254ee4a7f2343b9d5bc04bc98e37791dc15ac8e72597595e89020b0d8`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🚪️io/💾️binary/🌱️genesis/🦀️.rs`: `0ec7b0a4b25a65345c3bc310be4aebf8f7d4eab57ecf5a50726cab60d3bcab1d`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🏗️bootstrap/🦀️.rs`: `78f3ca9fb63b7eaab3c16cfc40bc0ef401f0ae770cf114d049466373504009b8`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🏗️initialization/🔁️replay/🧪️tests/🔬️unit/🦀️.rs`: `a4fb68f878ddc4858839f4b11f0934533956d8195d5a458b59999bc1f62a353e`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs`: `c82f1a8765d31fbf34b73c4a55874d1df67ea9120f23e3cd2e3d4c381eb11f3a`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️backbone/✂️detach/🧪️tests/✂️detach/🦀️.rs`: `2a7278997da81b7a9d6adfdeb4f0d8cdd02b18a1ddda4edcd712e55e1601f42a`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🚪️io/📝️text/🦀️.rs`: `0eb59c9f10cb137ff08651b52838b9ffd0cda5ff8821c84071b4e96d71d0a8d4`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs`: `811100ba3f34a095a88391b6043d91c8db2762d5fdbabdd332d8567cd034d24a`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🧪️tests/🔬️unit/🦀️.rs`: `3fb8942fbb92956558b24e357a0eb2c4a69e98a605c74b870478e1d654fba391`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs`: `fd7a9d8787eb8bc95796cd4e03948f24de2fbdf95153aea663f75c6c347bc3bd`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️supersede-replay/🦀️.rs`: `ea80562e6c8ec92203d18e3131ff17ce0b75b1ee1e3745bdb3ac45a256cdff22`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧾️document/🦀️.rs`: `52f85ada484e9c07f079a75cce1f42219287834ddb72b328352e8526b80f22bc`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚖️authoring-law/🦀️.rs`: `e4b30102e3295520688ad8de3d5e44996ac7d31c2fd38a9e3ff6062b8a92d514`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/📃️query/🧪️tests/📃️query/🦀️.rs`: `5b7a0051f0e37bb0d309800a237cd052add82721845b171d1ea49f05977f984a`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/📖️capture/🧪️tests/📖️capture/🦀️.rs`: `ba936e843394261b4a600dc721574fc7d5f54b36e831cb088f4caef0b3a58f0d`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/📡️live/📨️dispatch/🧪️tests/📨️dispatch/🦀️.rs`: `6e12f8cd5f1bcf5eacc911c7fee8c64f3b5f2ca5fb3d12d6e22712839736e695`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/📡️live/🧪️tests/📡️live/🦀️.rs`: `4d94131ef92da7a52e847fc01f62e164e78486fdebf3ffa6925dc3ffb5d3f69b`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs`: `31cc3fd297eaa97ebf395c9cbb7701dd0b1e4d922ea9615f82808ebede9077ea`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs`: `b7e20d1fa14b5c63c2e1ae56633a2e44287f80f859efb62a209a266f9d6c916c`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧩️composition/🦀️.rs`: `bd82a237ce6a21567556d5222475d6a41ed59fcde4a31585d2a1cc67afbe096d`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️bounded-reload/🦀️.rs`: `4b90e6773d576f16ede923dadce31e4f1597011c5aa1f502453be4082ec6f76f`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️node-drag-history/🦀️.rs`: `9651cf75bc3531f970ed2cd3db9f3be2a2976fb8564ec15cb62e8e3e639c9638`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️tool-run-member/🦀️.rs`: `00a714a2e8ae1633b590685a1cc94c8ea048bedf1999f91309053df7b2daf870`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧾️document-archive-load-legs/🦀️.rs`: `867ee5160dcf17178ca53108a10565787cde0829e11ce5d3671dfa1c18c9faaf`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🦀️.rs`: `b9b80dd8aca1d8f63399e965819ebe9ce5b990138c1886e034bb9cc59bebbe19`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🦀️.rs`: `bae37e39c3a228a4063f89324f0409faf85fbb50b261d013611fc9710731e1c9`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🧪️tests/🪟️retained-window-config/🦀️.rs`: `72343d29d94d951faae7bd3a087ef41b01536467c6a956290e9d58e15c714464`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs`: `07ba1fa56a6948a4cb721eaa097aaf39bc93b947c27ce4b0338effd4d80eb399`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🗂️collection/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs`: `d96bac30822d5b57fb89fe94f5b1fbd2c28a79e3dd05f71f31d8dbd3426460f6`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🗂️collection/🧪️tests/⚖️retirement-law/🦀️.rs`: `8b40cc4df8819fe6f0ed8942a107601b26c412dd1eca6917f2e060dc7d79bb22`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🪐️space/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs`: `decca6235b52b468dd64315ed8a85d3d5501dd715d8413c3a79f4e10ad9ea738`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🪐️space/🧪️tests/⚖️retirement-law/🦀️.rs`: `797e41001f3cd95ba79380e1eab0acc60ca07652e98ce28f86bd745b3179c63a`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🦀️.rs`: `8961fa8468624b1523e531a95cd6cb8c12fba6c1ba7129194d57c64b11a71fb2`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🧪️tests/🔬️unit/🦀️.rs`: `b9b8b1aeda11d8d09f65cbb98710baccccafee89aecd38c00b0f16ff67a8c72a`
- `🧰️framework/🛍️products/💻️os/🖥️host/🦀️.rs`: `a218ccadfa638b32250e6ce3fb472f21b055387b3262dda151468d0c39616a7e`
- `🧰️framework/🛍️products/💻️os/🖥️host/🧪️tests/🔬️host-unit/🦀️.rs`: `b19d14493133108ba0564b090bddecd5ea0675336c423157d078a784f4898f03`
