# Next Schema Production Cut: Current Read-Only Preparation

Observed on 2026-10-02T09:01:06.986Z. This is scope preparation after the exact goal-framework cleanup. No source, fixture, schema, Cargo manifest, generated Rust, native compiler or test route was modified or executed. Native's catalog/Draw epoch remains stable. References below are current literal source candidates, including comments; they are not claims of semantic alias/provider completeness.

## Current authority and dependency closure

Schema Cargo34 still directly depends on OS. Actual package root mounts component7; production component8 imports StateClass, component123 imports six composition types/traits through OS, component249–252 imports three Kernel descriptor families/FacetLeaves plus six registration/visit callbacks, and359/514/679 invoke OS registration lookups. Changing StateClass alone leaves the direct product edge.

OS package265–266 is the only current compile mount of schema/🧩️composition/🦀️.rs. The source is pure std-only. Schema derive already emits canonical schema-qualified traits at expansion223/235/238/249/258/270–278. Move the one mount to a dedicated lower schema composition crate and bind both schema and OS/store directly to that single trait/type identity; remove the OS module, not a forwarding alias.

StateClass is authored replication wire294–323, not OS. Its four-value neutral Value codec must move with its enum if extracting a narrow state owner. Protocol's actual Cargo library name is protocol. Replication has Value/Value-derive/hash/base64/optionaldeflate plus serde/browser dependencies; Schema→Pack→Protocol already exists. Protocol→mainSchema would create a cycle. The clean proposed closure is mainSchema→dedicated schema-state(Value), schema-composition(std), existing schema-registry(std), validator/Pack/async/derive; Protocol→schema-state+schema-registry; OS/store→same composition/state/catalog providers. Do not point Protocol at mainSchema or duplicate StateClass/descriptor sources. New lower packages/semantic registrations are not authored yet.

The std-only descriptor block is wire328–474. Existing schema-registry owns FacetLeaves with the same five static fields; main schema defines artifact/inference/app descriptors and kernel conversion functions269–293/415–424/620–629. One canonical neutral descriptor identity should live in registry, with all three independent registries and deterministic sorted visits; physically remove Kernel-prefixed types/functions and duplicate conversions. Schema-version methods currently depend on Pack ContentHash/JSON canonicalization and must remain higher as explicit schema functions/traits, without dragging Pack→Protocol into registry or leaving old descriptor aliases. Exact version-method caller rebinding is part of the implementation scope.

Current public batch registration preflights conflicting existing and within-batch entries before writes, but single-register callbacks replace directly and mirror artifact facets while ignoring the export-registry Result. Preflight and commit acquire separate locks. Therefore closed tests must distinguish current single-replacement behavior, sequential batch no-partial-update, mirrored-facet conflict and concurrent transaction semantics; no stronger atomicity or rollback guarantee is inferred from the current docstring. Root must coordinate any deliberate contract correction rather than silently changing these semantics during ownership extraction.

## Original law retention

Current component-unit retains30 named laws (29 async and the synchronous fragment-validation law). Only artifact_composition_projection_real_child_alias_has_fixed_admission_bounds is higher: it imports real OS ArtifactChild/ChildRestoreProjection and must move unchanged to an actual OS child-composition test mount, with direct neutral IO ArtifactRef/Dialect bindings. Preserve64 accepted/65 ReferenceLimit,257 absent-element TraversalLimit,128 ä characters accepted and one appended byte InvalidReference. Stand-in lower fixtures are not a replacement for this law.

Lower slot-table/default-leaf/aliased-nested-Option-and-Vec traversal laws remain. Original traversal records two children after7 charged steps; cancellation after4 steps leaves only the first child. Keep all state/derive/GraphQL/export/fragment-validation/validator/format/entity/version laws, schema-module-compile and schema-export-entries targets. Existing permanent package script is schema/📦️packages/🦀️rust/📜️script.ts (not schema root); default test runs Cargo then original Bun fragment-validation oracle. Registry default permanent test includes schema-registry-catalog. Use those owned commands plus scoped new subcommands, registered Nx/package/seed routes, preserving levels and deadlines.

## Schema-first closed TDD matrix to author after release

|Facet|Exact portable witness IDs and expected evidence|Independent existing oracle / native integration|
|---|---|---|
|State vocabulary|artifact/config/presence/transient with exact Rust bare strings Artifact/Config/Presence/Transient, kebab values, GraphQL uppercase; all seven original retired spellings rejected; unknown/non-string refusal|serde_json four-variant test enum and AJV closed enum; actual owned Value ordinary/controlled codecs and derive field tables|
|Composition|leaf-empty; one-child; many-child; alias-nested-option; none-option; mixed-vector; seven-step-two-row; cancel-at-four-one-row; empty-vector-one-step; link-role-table|independent Node traversal over closed JSON recursive tagged shapes; unchanged native visitor/derive tests using the single lower trait identity|
|Artifact catalog|new; exact-repeat; single-replace-current; conflicting-established-batch; conflicting-within-batch; unrelated-earlier-entry-rollback; sorted-snapshot; mirror-export-conflict|Node Map/JSON tuple equality and AJV exact descriptor shapes; genuine registry factory/read callbacks retaining original schema assertions|
|Inference/app catalogs|new/exact-repeat/conflict-existing/conflict-batch/independent-family for each; same textID across three families remains independent|same independent Map oracle, genuine native three-family callbacks, unchanged inference/app schema-validation laws|
|Provider identity|one composition compile mount; StateClass/descriptor declaration origin; no Kernel aliases/OS export facade; direct actual crate identities; no schema↔Protocol cycle|captured no-follow source/manifest graph and independent @iarna/TOML; actual products/S/Hub-absent lower Cargo/native route retaining full laws|
|Higher child restore|original64/65/257/UTF8 boundary body retained unchanged|real OS child owner runtime, separate from absent-products lower proof|

Corpus IDs, cardinality, exact row shapes and per-ID expected outputs must be schema bound, including hostile substitutions. Test oracles must consume handcrafted values, not synthesize goldens from implementation. Lower native proof must capture every admitted mounted source/feature/manifest and remove all products/S/Hub physically; merely deleting one Cargo edge or checking source substrings is insufficient. No Cargo/deletion proof was run in this preparation epoch.

## Current source hashes

|Source|SHA256|
|---|---|
|`🧰️framework/🔨️modules/🧬️schema/🧪️tests/🔬️component-unit/🦀️.rs`|`228cb2e2543a4dabd211c5ec0e758829da69ff720a2788ac201fa89afcd53aaa`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|`6399dc57d8825d80dac2453dc33da6b9c53897aa992d4a089f74696d8eab5b5b`|
|`🧰️framework/🔨️modules/🧬️schema/🧩️composition/🦀️.rs`|`b2ad8e5e3f68d4a500ff80b5e1ecb24936ab50d34108120d92c6df5f4c99f143`|
|`🧰️framework/🔨️modules/🧬️schema/✨️derive/⚙️expansion/🦀️.rs`|`e1fc4ef7f0fd1c0e7073106a6359ea001119d80d87dbe0f66922052786e30679`|
|`🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️.rs`|`be3ff0edab5dbc80637a32642e0a6f6e9e3641928ad00bfb8036b750c9d5bc7c`|
|`🧰️framework/🔨️modules/🧬️schema/📇️registry/🦀️.rs`|`82fbd7a6c0acf8d627229ccaae2b2ec7a4e46f91010f84727e1582aeaa434524`|

## Literal caller inventory

### composition

150 literal rows in 34 distinct authored Rust files. Byte/line receipt only, not semantic proof.

|Source|Line|Observed source|
|---|---:|---|
|`✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🧪️tests/🔬️unit/🦀️.rs`|140|`    assert_eq!(projection.len(), <crate::WiresSnapshot as store::os_schema_composition::ArtifactCompositionFields>::child_slots().len());`|
|`✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🧪️tests/🔬️unit/🦀️.rs`|124|`    assert_eq!(projection.len(), <crate::EquationSnapshot as store::os_schema_composition::ArtifactCompositionFields>::child_slots().len());`|
|`🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🦀️.rs`|266|`pub mod os_schema_composition;`|
|`✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🧪️tests/🔬️unit/🦀️.rs`|64|`    assert_eq!(projection.len(), <crate::FormsSnapshot as store::os_schema_composition::ArtifactCompositionFields>::child_slots().len());`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow/🦀️.rs`|1325|`impl semio_framework_schema::ArtifactCompositionFields for WorkflowSnapshot {`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow/🦀️.rs`|1326|`    fn visit_child_refs<'a, V: semio_framework_schema::ChildRefVisitor<'a>>(&'a self, _visitor: &mut V) -> Result<(), V::Error> {`|
|`✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🧪️tests/🔬️unit/🦀️.rs`|192|`    assert_eq!(projection.len(), <crate::NoteSnapshot as store::os_schema_composition::ArtifactCompositionFields>::child_slots().len());`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|123|`pub use semio_framework_os_kernel::os_schema_composition::{ArtifactCompositionFields, ChildFieldRefs, ChildRefFields, ChildRefVisitor, ChildSlotSpec, LinkSlotSpec};`|
|`🧰️framework/🔨️modules/🧬️schema/🧪️tests/🔬️component-unit/🦀️.rs`|63|`impl<T> ChildFieldRefs for ArtifactChild<T> {`|
|`🧰️framework/🔨️modules/🧬️schema/🧪️tests/🔬️component-unit/🦀️.rs`|65|`    fn visit_child_field<'a, V: ChildRefVisitor<'a>>(&'a self, slot: &'static str, visitor: &mut V) -> Result<(), V::Error> {`|
|`🧰️framework/🔨️modules/🧬️schema/🧪️tests/🔬️component-unit/🦀️.rs`|67|`        visitor.child(slot, ChildRefFields { child_id: "child", artifact_id: "child", artifact_kind: "s.stdio.mesh", standard: "v1", subset: "*" })`|
|`🧰️framework/🔨️modules/🧬️schema/🧪️tests/🔬️component-unit/🦀️.rs`|106|`    assert_eq!(children[0], ChildSlotSpec { name: "primaryMesh", kind: "s.stdio.mesh", many: false });`|
|`🧰️framework/🔨️modules/🧬️schema/🧪️tests/🔬️component-unit/🦀️.rs`|107|`    assert_eq!(children[1], ChildSlotSpec { name: "textures", kind: "s.stdio.image", many: true });`|
|`🧰️framework/🔨️modules/🧬️schema/🧪️tests/🔬️component-unit/🦀️.rs`|111|`    assert_eq!(links[0], LinkSlotSpec { name: "baseMaterial", roles: &["base", "material"], many: false });`|
|`🧰️framework/🔨️modules/🧬️schema/🧪️tests/🔬️component-unit/🦀️.rs`|128|`    impl<'a> ChildRefVisitor<'a> for Visitor {`|
|`🧰️framework/🔨️modules/🧬️schema/🧪️tests/🔬️component-unit/🦀️.rs`|137|`        fn child(&mut self, slot: &'static str, fields: ChildRefFields<'a>) -> Result<(), ()> {`|
|`🧰️framework/🔨️modules/🧬️schema/🧪️tests/🔬️component-unit/🦀️.rs`|144|`    assert_eq!(slots, &[ChildSlotSpec { name: "optionalChild", kind: "s.stdio.mesh", many: false }, ChildSlotSpec { name: "children", kind: "s.stdio.mesh", many: true }]);`|
|`🧰️framework/🔨️modules/🧬️schema/✨️derive/⚙️expansion/🦀️.rs`|235|`                ::semio_framework_schema::ChildSlotSpec { name: #name_lit, kind: #kind_lit, many: <#ty as ::semio_framework_schema::ChildFieldRefs>::MANY }`|
|`🧰️framework/🔨️modules/🧬️schema/✨️derive/⚙️expansion/🦀️.rs`|238|`                ::semio_framework_schema::ChildFieldRefs::visit_child_field(&self.#name, #name_lit, visitor)?;`|
|`🧰️framework/🔨️modules/🧬️schema/✨️derive/⚙️expansion/🦀️.rs`|249|`                    ::semio_framework_schema::LinkSlotSpec { name: #name_lit, roles: &[#(#role_lits),*], many: #many }`|
|`🧰️framework/🔨️modules/🧬️schema/✨️derive/⚙️expansion/🦀️.rs`|270|`        impl ::semio_framework_schema::ArtifactCompositionFields for #ident {`|
|`🧰️framework/🔨️modules/🧬️schema/✨️derive/⚙️expansion/🦀️.rs`|271|`            fn visit_child_refs<'a, V: ::semio_framework_schema::ChildRefVisitor<'a>>(&'a self, visitor: &mut V) -> Result<(), V::Error> {`|
|`🧰️framework/🔨️modules/🧬️schema/✨️derive/⚙️expansion/🦀️.rs`|275|`            fn child_slots() -> &'static [::semio_framework_schema::ChildSlotSpec] {`|
|`🧰️framework/🔨️modules/🧬️schema/✨️derive/⚙️expansion/🦀️.rs`|278|`            fn link_slots() -> &'static [::semio_framework_schema::LinkSlotSpec] {`|
|`🧰️framework/🔨️modules/🧬️schema/✨️derive/🦀️.rs`|10|`/// sibling [&#96;ArtifactCompositionFields&#96;] impl from &#96;ArtifactChild<T>&#96; / &#96;ArtifactLink&#96; field types`|
|`🧰️framework/🔨️modules/🧬️schema/🧩️composition/🦀️.rs`|11|`pub struct ChildSlotSpec {`|
|`🧰️framework/🔨️modules/🧬️schema/🧩️composition/🦀️.rs`|19|`pub struct ChildRefFields<'a> {`|
|`🧰️framework/🔨️modules/🧬️schema/🧩️composition/🦀️.rs`|28|`pub trait ChildRefVisitor<'a> {`|
|`🧰️framework/🔨️modules/🧬️schema/🧩️composition/🦀️.rs`|31|`    fn child(&mut self, slot: &'static str, fields: ChildRefFields<'a>) -> Result<(), Self::Error>;`|
|`🧰️framework/🔨️modules/🧬️schema/🧩️composition/🦀️.rs`|35|`pub trait ChildFieldRefs {`|
|`🧰️framework/🔨️modules/🧬️schema/🧩️composition/🦀️.rs`|37|`    fn visit_child_field<'a, V: ChildRefVisitor<'a>>(&'a self, slot: &'static str, visitor: &mut V) -> Result<(), V::Error>;`|
|`🧰️framework/🔨️modules/🧬️schema/🧩️composition/🦀️.rs`|40|`impl<T: ChildFieldRefs> ChildFieldRefs for Option<T> {`|
|`🧰️framework/🔨️modules/🧬️schema/🧩️composition/🦀️.rs`|42|`    fn visit_child_field<'a, V: ChildRefVisitor<'a>>(&'a self, slot: &'static str, visitor: &mut V) -> Result<(), V::Error> {`|
|`🧰️framework/🔨️modules/🧬️schema/🧩️composition/🦀️.rs`|51|`impl<T: ChildFieldRefs> ChildFieldRefs for Vec<T> {`|
|`🧰️framework/🔨️modules/🧬️schema/🧩️composition/🦀️.rs`|53|`    fn visit_child_field<'a, V: ChildRefVisitor<'a>>(&'a self, slot: &'static str, visitor: &mut V) -> Result<(), V::Error> {`|
|`🧰️framework/🔨️modules/🧬️schema/🧩️composition/🦀️.rs`|65|`pub struct LinkSlotSpec {`|
|`🧰️framework/🔨️modules/🧬️schema/🧩️composition/🦀️.rs`|72|`pub trait ArtifactCompositionFields {`|
|`🧰️framework/🔨️modules/🧬️schema/🧩️composition/🦀️.rs`|73|`    fn visit_child_refs<'a, V: ChildRefVisitor<'a>>(&'a self, visitor: &mut V) -> Result<(), V::Error>;`|
|`🧰️framework/🔨️modules/🧬️schema/🧩️composition/🦀️.rs`|74|`    fn child_slots() -> &'static [ChildSlotSpec] {`|
|`🧰️framework/🔨️modules/🧬️schema/🧩️composition/🦀️.rs`|77|`    fn link_slots() -> &'static [LinkSlotSpec] {`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs`|213|`impl store::os_schema_composition::ArtifactCompositionFields for ProbeSnapshot {`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs`|214|`    fn visit_child_refs<'a, V: store::os_schema_composition::ChildRefVisitor<'a>>(&'a self, _visitor: &mut V) -> Result<(), V::Error> {`|
|`✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🧪️tests/🔬️unit/🦀️.rs`|56|`    assert_eq!(projection.len(), <crate::DagSnapshot as store::os_schema_composition::ArtifactCompositionFields>::child_slots().len());`|
|`✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🧪️tests/🔬️unit/🦀️.rs`|59|`    assert_eq!(projection.len(), <crate::ProcedureSnapshot as store::os_schema_composition::ArtifactCompositionFields>::child_slots().len());`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs`|3326|`impl<S> crate::os_schema_composition::ChildFieldRefs for ArtifactChild<S> {`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs`|3328|`    fn visit_child_field<'a, V: crate::os_schema_composition::ChildRefVisitor<'a>>(&'a self, slot: &'static str, visitor: &mut V) -> Result<(), V::Error> {`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs`|3332|`            crate::os_schema_composition::ChildRefFields {`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs`|3432|`    slots: &'static [crate::os_schema_composition::ChildSlotSpec],`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs`|3433|`    rows: [Option<(&'static str, crate::os_schema_composition::ChildRefFields<'a>)>; CHILD_RESTORE_MAXIMUM_REFERENCES],`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs`|3440|`    pub fn from_snapshot<S: crate::os_schema_composition::ArtifactCompositionFields>(snapshot: &'a S) -> Result<Self, ChildRestoreProjectionError> {`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs`|3471|`    pub fn get(&self, index: usize) -> Option<(&'static str, crate::os_schema_composition::ChildRefFields<'a>)> {`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs`|3487|`    pub fn admit_complete<'b>(&self, incoming: impl IntoIterator<Item = (&'b str, crate::os_schema_composition::ChildRefFields<'b>)>) -> Result<(), ChildRestoreProjectionError> {`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs`|3508|`impl<'a> crate::os_schema_composition::ChildRefVisitor<'a> for ChildRestoreProjection<'a> {`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs`|3517|`    fn child(&mut self, slot: &'static str, fields: crate::os_schema_composition::ChildRefFields<'a>) -> Result<(), Self::Error> {`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs`|3557|`/// &#96;ChildSlotSpec&#96;/&#96;LinkSlotSpec&#96; schema facets).`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs`|24076|`    P: Clone + ToValue + FromValue + ArtifactPack + crate::os_schema_composition::ArtifactCompositionFields + Send + Sync + 'static,`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs`|24445|`        P: Clone + ToValue + FromValue + ArtifactPack + crate::os_schema_composition::ArtifactCompositionFields + Send + Sync + 'static,`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs`|24454|`        P: Clone + ToValue + FromValue + ArtifactPack + crate::os_schema_composition::ArtifactCompositionFields + Send + Sync + 'static,`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs`|1496|`    P: Clone + ToValue + FromValue + ArtifactPack + crate::os_schema_composition::ArtifactCompositionFields + Send + Sync + 'static,`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs`|1689|`impl crate::os_schema_composition::ArtifactCompositionFields for DemoSnapshot {`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs`|1690|`    fn visit_child_refs<'a, V: crate::os_schema_composition::ChildRefVisitor<'a>>(&'a self, _visitor: &mut V) -> Result<(), V::Error> {`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs`|3308|`impl crate::os_schema_composition::ArtifactCompositionFields for RetainedTextSnapshot {`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs`|3309|`    fn visit_child_refs<'a, V: crate::os_schema_composition::ChildRefVisitor<'a>>(&'a self, _visitor: &mut V) -> Result<(), V::Error> {`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs`|8448|`    use crate::os_schema_composition::{ArtifactCompositionFields, ChildRefFields, ChildRefVisitor, ChildSlotSpec};`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs`|8450|`    fn fields(row: &serde_json::Value) -> ChildRefFields<'_> {`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs`|8451|`        ChildRefFields {`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs`|8459|`    impl ArtifactCompositionFields for Parent<'_> {`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs`|8460|`        fn child_slots() -> &'static [ChildSlotSpec] {`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs`|8461|`            &[ChildSlotSpec { name: "single", kind: "s.test.member", many: false }, ChildSlotSpec { name: "many", kind: "s.test.member", many: true }, ChildSlotSpec { name: "other", kind: "s.test.member", many: false }]`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs`|8463|`        fn visit_child_refs<'a, V: ChildRefVisitor<'a>>(&'a self, visitor: &mut V) -> Result<(), V::Error> {`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs`|282|`    P: Clone + ToValue + FromValue + ArtifactPack + MemberStoreOwner<M> + crate::os_schema_composition::ArtifactCompositionFields,`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs`|314|`    P: Clone + ToValue + FromValue + ArtifactPack + MemberStoreOwner<M> + crate::os_schema_composition::ArtifactCompositionFields,`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs`|338|`    P: Clone + ToValue + FromValue + ArtifactPack + MemberStoreOwner<M> + crate::os_schema_composition::ArtifactCompositionFields + Send + Sync + 'static,`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs`|712|`    P: Clone + ToValue + FromValue + ArtifactPack + MemberStoreOwner<M> + crate::os_schema_composition::ArtifactCompositionFields + Send + Sync + 'static,`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs`|819|`    P: Clone + ToValue + FromValue + ArtifactPack + MemberStoreOwner<M> + crate::os_schema_composition::ArtifactCompositionFields + Send + Sync + 'static,`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs`|840|`    P: Clone + ToValue + FromValue + ArtifactPack + MemberStoreOwner<M> + crate::os_schema_composition::ArtifactCompositionFields,`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🌳️closure/🧪️tests/🔬️unit/🦀️.rs`|3|`use crate::os_schema_composition::{ArtifactCompositionFields, ChildRefFields, ChildRefVisitor, ChildSlotSpec};`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🌳️closure/🧪️tests/🔬️unit/🦀️.rs`|11|`impl ArtifactCompositionFields for FixtureNode {`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🌳️closure/🧪️tests/🔬️unit/🦀️.rs`|12|`    fn child_slots() -> &'static [ChildSlotSpec] {`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🌳️closure/🧪️tests/🔬️unit/🦀️.rs`|14|`            ChildSlotSpec { name: "objects", kind: "s.stdio.semio", many: true },`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🌳️closure/🧪️tests/🔬️unit/🦀️.rs`|15|`            ChildSlotSpec { name: "mesh", kind: "s.stdio.semio", many: true },`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🌳️closure/🧪️tests/🔬️unit/🦀️.rs`|16|`            ChildSlotSpec { name: "value", kind: "s.stdio.semio", many: true },`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🌳️closure/🧪️tests/🔬️unit/🦀️.rs`|17|`            ChildSlotSpec { name: "children", kind: "s.stdio.semio", many: true },`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🌳️closure/🧪️tests/🔬️unit/🦀️.rs`|18|`            ChildSlotSpec { name: "other", kind: "s.stdio.semio", many: true },`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🌳️closure/🧪️tests/🔬️unit/🦀️.rs`|21|`    fn visit_child_refs<'a, V: ChildRefVisitor<'a>>(&'a self, visitor: &mut V) -> Result<(), V::Error> {`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🌳️closure/🧪️tests/🔬️unit/🦀️.rs`|25|`            visitor.child(slot, ChildRefFields {`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs`|1528|`impl store::os_schema_composition::ArtifactCompositionFields for HashProjection {`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs`|1529|`    fn visit_child_refs<'a, V: store::os_schema_composition::ChildRefVisitor<'a>>(&'a self, _visitor: &mut V) -> Result<(), V::Error> {`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`|1190|`    /// (&#96;semio_framework_schema::ArtifactCompositionFields&#96;), rather than only foreign-dialect IO`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`|1199|`        fn slots() -> &'static [::semio_framework_schema::ChildSlotSpec];`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`|1202|`        /// &#96;ChildSlotSpec.kind&#96;) — the composition-side counterpart to &#96;decompose_to_children&#96;.`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`|1220|`        fn slots() -> &'static [::semio_framework_schema::ChildSlotSpec] {`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`|1366|`        /// — a child slot names only a &#96;kind&#96;, per &#96;ChildSlotSpec.kind: &'static str&#96;, never a`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`|3007|`        /// 🧒️🔗️ Pulled from &#96;<Snapshot as ArtifactCompositionFields>::{child_slots,link_slots}&#96; via`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`|3011|`        /// runtime reads &#96;ArtifactCompositionFields&#96; straight off the snapshot type on demand); they`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`|3015|`        child_slots: &'static [::semio_framework_schema::ChildSlotSpec],`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`|3017|`        link_slots: &'static [::semio_framework_schema::LinkSlotSpec],`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`|3066|`        child_slots: &'static [::semio_framework_schema::ChildSlotSpec],`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`|3067|`        link_slots: &'static [::semio_framework_schema::LinkSlotSpec],`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`|3366|`        /// 🧒️🔗️ Pulls &#96;child_slots&#96;/&#96;link_slots&#96; from &#96;<Snapshot as ArtifactCompositionFields>&#96; — the`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`|3370|`        pub fn composition<Snapshot: ::semio_framework_schema::ArtifactCompositionFields>(mut self) -> Self {`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`|7382|`            S: semio_framework_schema::ArtifactCompositionFields,`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`|10142|`        P: semio_framework_schema::ArtifactCompositionFields,`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`|11535|`    impl semio_framework_schema::ArtifactCompositionFields for NoConfig {`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`|11536|`        fn visit_child_refs<'a, V: semio_framework_schema::ChildRefVisitor<'a>>(&'a self, _visitor: &mut V) -> Result<(), V::Error> {`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`|13728|`        type Snapshot: Clone + PartialEq + protocol::ToValue + protocol::FromValue + Send + Sync + store::ArtifactDsl + ArtifactPack + semio_framework_schema::ArtifactCompositionFields + 'static;`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`|22476|`        P: Clone + protocol::ToValue + protocol::FromValue + ArtifactPack + semio_framework_schema::ArtifactCompositionFields + Send + Sync + 'static,`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`|22516|`        P: Clone + protocol::ToValue + protocol::FromValue + ArtifactPack + semio_framework_schema::ArtifactCompositionFields + Send + Sync + 'static,`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`|23114|`        P: Clone + protocol::ToValue + protocol::FromValue + ArtifactPack + semio_framework_schema::ArtifactCompositionFields + Send + Sync + 'static,`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`|36921|`        type Snapshot: Clone + PartialEq + protocol::ToValue + protocol::FromValue + Send + Sync + store::ArtifactDsl + ArtifactPack + semio_framework_schema::ArtifactCompositionFields + 'static;`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`|37643|`        type Snapshot: Clone + PartialEq + protocol::ToValue + protocol::FromValue + Send + Sync + store::ArtifactDsl + ArtifactPack + semio_framework_schema::ArtifactCompositionFields + 'static;`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`|38969|`        /// (composition still reads &#96;<Snapshot as ArtifactCompositionFields>&#96; directly; capabilities`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🖥️test-app-mutations-document/🦀️.rs`|106|`impl semio_framework_schema::ArtifactCompositionFields for TestSnapshot {`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🖥️test-app-mutations-document/🦀️.rs`|107|`    fn visit_child_refs<'a, V: semio_framework_schema::ChildRefVisitor<'a>>(&'a self, visitor: &mut V) -> Result<(), V::Error> {`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🖥️test-app-mutations-document/🦀️.rs`|108|`        semio_framework_schema::ChildFieldRefs::visit_child_field(&self.slot, "slot", visitor)`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🖥️test-app-mutations-document/🦀️.rs`|111|`    fn child_slots() -> &'static [semio_framework_schema::ChildSlotSpec] {`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🖥️test-app-mutations-document/🦀️.rs`|112|`        &[semio_framework_schema::ChildSlotSpec { name: "slot", kind: "s.test.child", many: true }]`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️derived-artifact-children/🦀️.rs`|84|`const CHILD_SLOTS: &[::semio_framework_schema::ChildSlotSpec] = &[::semio_framework_schema::ChildSlotSpec { name: "primaryMesh", kind: "s.stdio.mesh", many: false }];`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️derived-artifact-children/🦀️.rs`|90|`    fn slots() -> &'static [::semio_framework_schema::ChildSlotSpec] {`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-declarations-fixture/🦀️.rs`|38|`            impl semio_framework_schema::ArtifactCompositionFields for $snapshot {`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-declarations-fixture/🦀️.rs`|39|`                fn visit_child_refs<'a, V: semio_framework_schema::ChildRefVisitor<'a>>(&'a self, _visitor: &mut V) -> Result<(), V::Error> {`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures-surface/🦀️.rs`|52|`impl semio_framework_schema::ArtifactCompositionFields for SurfaceSnapshot {`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures-surface/🦀️.rs`|53|`    fn visit_child_refs<'a, V: semio_framework_schema::ChildRefVisitor<'a>>(&'a self, _visitor: &mut V) -> Result<(), V::Error> {`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures-dummy/🦀️.rs`|50|`impl semio_framework_schema::ArtifactCompositionFields for DummySnapshot {`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures-dummy/🦀️.rs`|51|`    fn visit_child_refs<'a, V: semio_framework_schema::ChildRefVisitor<'a>>(&'a self, _visitor: &mut V) -> Result<(), V::Error> {`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures-transaction/🦀️.rs`|52|`impl semio_framework_schema::ArtifactCompositionFields for TxnSnapshot {`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures-transaction/🦀️.rs`|53|`    fn visit_child_refs<'a, V: semio_framework_schema::ChildRefVisitor<'a>>(&'a self, _visitor: &mut V) -> Result<(), V::Error> {`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧫️fixtures/🧩️component/🧬️schema/📸️snapshot/🦀️.rs`|8|`impl semio_framework_schema::ArtifactCompositionFields for Snapshot {`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧫️fixtures/🧩️component/🧬️schema/📸️snapshot/🦀️.rs`|9|`    fn visit_child_refs<'a,V:semio_framework_schema::ChildRefVisitor<'a>>(&'a self,_visitor:&mut V)->Result<(),V::Error>{Ok(())}`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🦀️.rs`|855|`        P: Clone + store::ToValue + store::FromValue + ArtifactPack + semio_framework_schema::ArtifactCompositionFields + Send + Sync + 'static,`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🦀️.rs`|878|`        P: Clone + store::ToValue + store::FromValue + ArtifactPack + semio_framework_schema::ArtifactCompositionFields + Send + Sync + 'static,`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🦀️.rs`|896|`        P: Clone + store::ToValue + store::FromValue + ArtifactPack + semio_framework_schema::ArtifactCompositionFields + Send + Sync + 'static,`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🦀️.rs`|1394|`        P: Clone + store::ToValue + store::FromValue + ArtifactPack + semio_framework_schema::ArtifactCompositionFields + Send + Sync + 'static,`|
|`✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🦀️.rs`|178|`impl store::os_schema_composition::ArtifactCompositionFields for ModuleRenderPayload {`|
|`✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🦀️.rs`|179|`    fn visit_child_refs<'a, V: store::os_schema_composition::ChildRefVisitor<'a>>(&'a self, _visitor: &mut V) -> Result<(), V::Error> {`|
|`✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🧪️tests/🔬️unit/🦀️.rs`|303|`    assert_eq!(projection.len(), <crate::JackSnapshot as store::os_schema_composition::ArtifactCompositionFields>::child_slots().len());`|
|`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`|722|`impl semio_framework_schema::ArtifactCompositionFields for Puzzle2dPlaySnapshot {`|
|`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`|723|`    fn visit_child_refs<'a, V: semio_framework_schema::ChildRefVisitor<'a>>(&'a self, _visitor: &mut V) -> Result<(), V::Error> {`|
|`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`|693|`impl semio_framework_schema::ArtifactCompositionFields for Puzzle5dPlaySnapshot {`|
|`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`|694|`    fn visit_child_refs<'a, V: semio_framework_schema::ChildRefVisitor<'a>>(&'a self, _visitor: &mut V) -> Result<(), V::Error> {`|
|`✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🦀️.rs`|59|`impl semio_framework_schema::ChildFieldRefs for LayoutDrawingChild {`|
|`✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🦀️.rs`|61|`    fn visit_child_field<'a, V: semio_framework_schema::ChildRefVisitor<'a>>(&'a self, slot: &'static str, visitor: &mut V) -> Result<(), V::Error> {`|
|`✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🦀️.rs`|63|`        semio_framework_schema::ChildFieldRefs::visit_child_field(&self.handle, slot, visitor)`|
|`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`|1018|`impl semio_framework_schema::ArtifactCompositionFields for Puzzle3dPlaySnapshot {`|
|`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`|1019|`    fn visit_child_refs<'a, V: semio_framework_schema::ChildRefVisitor<'a>>(&'a self, visitor: &mut V) -> Result<(), V::Error> {`|
|`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`|1020|`        semio_framework_schema::ArtifactCompositionFields::visit_child_refs(self.typed.as_ref(), visitor)`|
|`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`|1022|`    fn child_slots() -> &'static [semio_framework_schema::ChildSlotSpec] {`|
|`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`|1023|`        <Puzzle3dSnapshot as semio_framework_schema::ArtifactCompositionFields>::child_slots()`|
|`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`|1025|`    fn link_slots() -> &'static [semio_framework_schema::LinkSlotSpec] {`|
|`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`|1026|`        <Puzzle3dSnapshot as semio_framework_schema::ArtifactCompositionFields>::link_slots()`|

### catalog

230 literal rows in 60 distinct authored Rust files. Byte/line receipt only, not semantic proof.

|Source|Line|Observed source|
|---|---:|---|
|`🌎️hub/🧩️compositions/🗄️stdio/🧪️tests/🚢️shipped-fleet/🦀️.rs`|181|`            assert!(semio_framework_os_kernel::kernel_artifact_schema_descriptor_registered(kind), "{} ({}) edits {kind}, whose document schema its package's assembly must publish", app.id, package.id);`|
|`🌎️hub/🧩️compositions/🗄️stdio/🧪️tests/🚢️shipped-fleet/🦀️.rs`|260|`            None => semio_framework_os_kernel::kernel_artifact_schema_descriptor_registered(&value("schema")),`|
|`🌎️hub/🧩️compositions/🗄️stdio/🧪️tests/🚢️shipped-fleet/🦀️.rs`|262|`        "inference" => semio_framework_os_kernel::kernel_artifact_inference_descriptor_registered(&value("schema")),`|
|`🌎️hub/🧩️compositions/🗄️stdio/🧪️tests/🚢️shipped-fleet/🦀️.rs`|301|`    let contracts = semio_framework_os_kernel::with_kernel_artifact_schema_catalog(&#124;entries&#124; entries.iter().map(&#124;entry&#124; entry.id).collect::<Vec<_>>());`|
|`✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🦀️.rs`|859|`/// &#96;register_app_schema_descriptor&#96; is not in the W1 census's artifact-scoped function set.`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/🚪️io/🦀️.rs`|48|`        ::framework_schema::register_artifact_schema_descriptor(crate::standards::iana::subsets::any::schema::tsv_artifact_schema_descriptor());`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/🚪️io/🦀️.rs`|59|`        ::framework_schema::register_artifact_inference_descriptor(crate::standards::iana::subsets::any::schema::inferences::tsv_artifact_inference_descriptor());`|
|`✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🦀️.rs`|360|`/// (see that struct's own doc) — &#96;register_app_schema_descriptor&#96; is not in §6's artifact-scoped`|
|`✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🦀️.rs`|276|`/// (see that struct's own doc) — &#96;register_app_schema_descriptor&#96; is not in §6's artifact-scoped`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🚪️io/🦀️.rs`|163|`    ::framework_schema::register_artifact_schema_descriptor(crate::standards::v_raw::subsets::any::schema::binary_artifact_schema_descriptor());`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🚪️io/🦀️.rs`|171|`    ::framework_schema::register_artifact_inference_descriptor(crate::standards::v_raw::subsets::any::schema::inferences::binary_artifact_inference_descriptor());`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/🦀️.rs`|146|`    ::framework_schema::register_artifact_schema_descriptor(crate::schema::txt_artifact_schema_descriptor());`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/🦀️.rs`|154|`    ::framework_schema::register_artifact_inference_descriptor(crate::standards::v_utf_8::subsets::any::schema::inferences::txt_artifact_inference_descriptor());`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧱️baseline/✏️editor/🧪️tests/🔬️unit/🦀️.rs`|6|`    framework_schema::register_artifact_schema_descriptors(vec![crate::standards::v6_0::subsets::document::schema::tiff_artifact_schema_descriptor()]).expect("the tiff document schema registers");`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/✏️editor/🧪️tests/🔬️unit/🦀️.rs`|6|`    framework_schema::register_artifact_schema_descriptors(vec![crate::standards::v6_0::subsets::document::schema::tiff_artifact_schema_descriptor()]).expect("the tiff document schema registers");`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/7️⃣87a/🪆️subsets/✳️any/🚪️io/🦀️.rs`|678|`    ::framework_schema::register_artifact_schema_descriptor(crate::standards::v87a::subsets::any::schema::gif_artifact_schema_descriptor());`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/7️⃣87a/🪆️subsets/✳️any/🚪️io/🦀️.rs`|686|`/// sibling to &#96;register_artifact_schema_descriptor&#96; above (separate registry, ticket`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/7️⃣87a/🪆️subsets/✳️any/🚪️io/🦀️.rs`|690|`    ::framework_schema::register_artifact_inference_descriptor(crate::standards::v87a::subsets::any::schema::inferences::gif_artifact_inference_descriptor());`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🚪️io/🦀️.rs`|401|`/// &#96;s.stdio.gif&#96;/&#96;stdio.gif&#96;. &#96;store::register_document_codec&#96;/&#96;::framework_schema::register_artifact_schema_descriptor&#96;`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🚪️io/🦀️.rs`|412|`    ::framework_schema::register_artifact_schema_descriptor(crate::standards::v89a::subsets::any::schema::gif_artifact_schema_descriptor());`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🚪️io/🦀️.rs`|420|`/// sibling to &#96;register_artifact_schema_descriptor&#96; above (separate registry, ticket`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🚪️io/🦀️.rs`|424|`    ::framework_schema::register_artifact_inference_descriptor(crate::standards::v89a::subsets::any::schema::inferences::gif89a_artifact_inference_descriptor());`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧱️baseline/✏️editor/🧪️tests/🔬️unit/🦀️.rs`|6|`    framework_schema::register_artifact_schema_descriptors(vec![crate::standards::v_jfif_1_01::subsets::document::schema::jpg_artifact_schema_descriptor()]).expect("the jpg document schema registers");`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`|2987|`    /// &#96;📓️w0-d-sdk-surface.md&#96; §6); the app-scoped &#96;register_app_schema_descriptor&#96; has no field`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`|4312|`            ::semio_framework_schema::register_artifact_schema_descriptors(self.schemas.clone()).map_err(&#124;error&#124; PluginAssemblyError::new("plugin-assembly.declaration-schema", error.to_string()))?;`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`|4313|`            ::semio_framework_schema::register_artifact_inference_descriptors(self.inferences.clone()).map_err(&#124;error&#124; PluginAssemblyError::new("plugin-assembly.declaration-inference", error.to_string()))`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`|39189|`            ::semio_framework_schema::register_artifact_schema_descriptors(schemas).map_err(&#124;error&#124; PluginAssemblyError::new("plugin-assembly.declaration-schema", error.to_string()))?;`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`|39190|`            ::semio_framework_schema::register_artifact_inference_descriptors(inferences).map_err(&#124;error&#124; PluginAssemblyError::new("plugin-assembly.declaration-inference", error.to_string()))?;`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/✏️editor/🧪️tests/🔬️unit/🦀️.rs`|6|`    framework_schema::register_artifact_schema_descriptors(vec![crate::standards::v_jfif_1_01::subsets::document::schema::jpg_artifact_schema_descriptor()]).expect("the jpg document schema registers");`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`|4|`    semio_framework_schema::register_artifact_schema_descriptor(crate::standards::isobmff::subsets::any::schema::mp4_artifact_schema_descriptor());`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🚪️io/🦀️.rs`|47|`        ::framework_schema::register_artifact_schema_descriptor(crate::standards::isobmff::subsets::any::schema::mp4_artifact_schema_descriptor());`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🚪️io/🦀️.rs`|54|`    /// catalog — sibling to &#96;register_artifact_schema_descriptor&#96; above (separate registry,`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🚪️io/🦀️.rs`|57|`        ::framework_schema::register_artifact_inference_descriptor(crate::standards::isobmff::subsets::any::schema::inferences::mp4_artifact_inference_descriptor());`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🚪️io/🦀️.rs`|48|`        ::framework_schema::register_artifact_schema_descriptor(crate::standards::mpeg1_layer3::subsets::any::schema::mp3_artifact_schema_descriptor());`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🚪️io/🦀️.rs`|55|`    /// catalog — sibling to &#96;register_artifact_schema_descriptor&#96; above (separate registry,`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🚪️io/🦀️.rs`|59|`        ::framework_schema::register_artifact_inference_descriptor(crate::standards::mpeg1_layer3::subsets::any::schema::inferences::mp3_artifact_inference_descriptor());`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`|74|`    semio_framework_schema::register_artifact_schema_descriptors(vec![crate::schema::deflate_artifact_schema_descriptor()]).expect("register deflate schema");`|
|`✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🪟️details/🧪️tests/🔬️unit/🦀️.rs`|754|`        semio_framework_schema::register_artifact_schema_descriptor(semio_framework_schema::ArtifactSchemaDescriptor {`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`|7|`    framework_schema::register_artifact_schema_descriptors(vec![crate::standards::riff_pcm::subsets::any::schema::wav_artifact_schema_descriptor()]).expect("the wav document schema registers");`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🚪️io/🦀️.rs`|618|`    ::framework_schema::register_artifact_schema_descriptor(crate::schema::bmp_artifact_schema_descriptor());`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🚪️io/🦀️.rs`|626|`    ::framework_schema::register_artifact_inference_descriptor(crate::standards::v_v3::subsets::any::schema::inferences::bmp_artifact_inference_descriptor());`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🚪️io/🦀️.rs`|48|`        ::framework_schema::register_artifact_schema_descriptor(crate::standards::riff_pcm::subsets::any::schema::wav_artifact_schema_descriptor());`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🚪️io/🦀️.rs`|55|`    /// catalog — sibling to &#96;register_artifact_schema_descriptor&#96; above (separate registry,`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🚪️io/🦀️.rs`|59|`        ::framework_schema::register_artifact_inference_descriptor(crate::standards::riff_pcm::subsets::any::schema::inferences::wav_artifact_inference_descriptor());`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🚪️io/🦀️.rs`|48|`        ::framework_schema::register_artifact_schema_descriptor(crate::standards::v5::subsets::any::schema::html_artifact_schema_descriptor());`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🚪️io/🦀️.rs`|59|`        ::framework_schema::register_artifact_inference_descriptor(crate::standards::v5::subsets::any::schema::inferences::html_artifact_inference_descriptor());`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🚪️io/🦀️.rs`|51|`        ::framework_schema::register_artifact_schema_descriptor(crate::standards::energyplus::subsets::any::schema::epw_artifact_schema_descriptor());`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🚪️io/🦀️.rs`|62|`        ::framework_schema::register_artifact_inference_descriptor(crate::standards::energyplus::subsets::any::schema::inferences::epw_artifact_inference_descriptor());`|
|`✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🦀️.rs`|237|`/// (see that struct's own doc) — &#96;register_app_schema_descriptor&#96; is not in §6's artifact-scoped`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔢️value/🚪️io/🦀️.rs`|119|`        ::framework_schema::register_artifact_schema_descriptor(crate::standards::v1::subsets::value::schema::semio_value_artifact_schema_descriptor());`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔢️value/🚪️io/🦀️.rs`|148|`    /// catalog — sibling to &#96;register_artifact_schema_descriptor&#96; above (separate registry,`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔢️value/🚪️io/🦀️.rs`|152|`        ::framework_schema::register_artifact_inference_descriptor(crate::standards::v1::subsets::value::schema::inferences::semio_value_artifact_inference_descriptor());`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🧬️schema/🦀️.rs`|4|`//! flat &#96;::framework_schema::register_artifact_schema_descriptor&#96; registry.`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🧬️schema/🦀️.rs`|304|`    ::framework_schema::register_artifact_schema_descriptor(ifc2x3_artifact_schema_descriptor());`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🧬️schema/🦀️.rs`|322|`    ::framework_schema::register_artifact_inference_descriptor(crate::standards::v2x3::subsets::base::schema::inferences::ifc2x3_artifact_inference_descriptor());`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🏛️model/🚪️io/🦀️.rs`|121|`        ::framework_schema::register_artifact_schema_descriptor(crate::standards::v1::subsets::model::schema::semio_model_artifact_schema_descriptor());`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🏛️model/🚪️io/🦀️.rs`|150|`    /// catalog — sibling to &#96;register_artifact_schema_descriptor&#96; above (separate registry,`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🏛️model/🚪️io/🦀️.rs`|154|`        ::framework_schema::register_artifact_inference_descriptor(crate::standards::v1::subsets::model::schema::inferences::semio_model_artifact_inference_descriptor());`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🦀️.rs`|344|`    ::framework_schema::register_artifact_schema_descriptor(ifc_artifact_schema_descriptor());`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🦀️.rs`|352|`    ::framework_schema::register_artifact_inference_descriptor(crate::standards::v4::subsets::any::schema::inferences::ifc_artifact_inference_descriptor());`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🚪️io/🦀️.rs`|186|`        ::framework_schema::register_artifact_schema_descriptor(crate::standards::v1::subsets::document::schema::semio_document_artifact_schema_descriptor());`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🚪️io/🦀️.rs`|215|`    /// catalog — sibling to &#96;register_artifact_schema_descriptor&#96; above (separate registry,`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🚪️io/🦀️.rs`|219|`        ::framework_schema::register_artifact_inference_descriptor(crate::standards::v1::subsets::document::schema::inferences::semio_document_artifact_inference_descriptor());`|
|`🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️.rs`|331|`pub struct KernelFacetLeaves {`|
|`🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️.rs`|341|`pub struct KernelArtifactSchemaDescriptor {`|
|`🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️.rs`|343|`    pub artifact: KernelFacetLeaves,`|
|`🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️.rs`|344|`    pub snapshot: KernelFacetLeaves,`|
|`🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️.rs`|345|`    pub diff: KernelFacetLeaves,`|
|`🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️.rs`|346|`    pub mutations: KernelFacetLeaves,`|
|`🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️.rs`|350|`    by_id: HashMap<&'static str, KernelArtifactSchemaDescriptor>,`|
|`🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️.rs`|355|`fn kernel_artifact_schema_catalog() -> &'static Mutex<KernelArtifactSchemaCatalog> {`|
|`🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️.rs`|360|`pub fn register_kernel_artifact_schema_descriptor(descriptor: KernelArtifactSchemaDescriptor) {`|
|`🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️.rs`|361|`    kernel_artifact_schema_catalog().lock().expect("kernel artifact schema catalog lock").by_id.insert(descriptor.id, descriptor);`|
|`🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️.rs`|365|`pub fn kernel_artifact_schema_descriptor_registered(id: &str) -> bool {`|
|`🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️.rs`|366|`    kernel_artifact_schema_catalog().lock().expect("kernel artifact schema catalog lock").by_id.contains_key(id)`|
|`🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️.rs`|370|`pub fn kernel_artifact_schema_catalog_len() -> usize {`|
|`🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️.rs`|371|`    kernel_artifact_schema_catalog().lock().expect("kernel artifact schema catalog lock").by_id.len()`|
|`🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️.rs`|375|`pub fn with_kernel_artifact_schema_catalog<R>(visit: impl FnOnce(&[KernelArtifactSchemaDescriptor]) -> R) -> R {`|
|`🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️.rs`|376|`    let guard = kernel_artifact_schema_catalog().lock().expect("kernel artifact schema catalog lock");`|
|`🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️.rs`|377|`    let mut entries: Vec<KernelArtifactSchemaDescriptor> = guard.by_id.values().cloned().collect();`|
|`🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️.rs`|385|`/// [&#96;KernelArtifactSchemaDescriptor&#96;], not a field on it: the four-facet descriptor already has ~107`|
|`🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️.rs`|392|`pub struct KernelArtifactInferenceDescriptor {`|
|`🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️.rs`|394|`    pub inference: KernelFacetLeaves,`|
|`🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️.rs`|398|`    by_id: HashMap<&'static str, KernelArtifactInferenceDescriptor>,`|
|`🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️.rs`|403|`fn kernel_artifact_inference_catalog() -> &'static Mutex<KernelArtifactInferenceCatalog> {`|
|`🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️.rs`|408|`pub fn register_kernel_artifact_inference_descriptor(descriptor: KernelArtifactInferenceDescriptor) {`|
|`🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️.rs`|409|`    kernel_artifact_inference_catalog().lock().expect("kernel artifact inference catalog lock").by_id.insert(descriptor.id, descriptor);`|
|`🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️.rs`|413|`pub fn kernel_artifact_inference_descriptor_registered(id: &str) -> bool {`|
|`🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️.rs`|414|`    kernel_artifact_inference_catalog().lock().expect("kernel artifact inference catalog lock").by_id.contains_key(id)`|
|`🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️.rs`|418|`pub fn kernel_artifact_inference_catalog_len() -> usize {`|
|`🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️.rs`|419|`    kernel_artifact_inference_catalog().lock().expect("kernel artifact inference catalog lock").by_id.len()`|
|`🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️.rs`|423|`pub fn with_kernel_artifact_inference_catalog<R>(visit: impl FnOnce(&[KernelArtifactInferenceDescriptor]) -> R) -> R {`|
|`🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️.rs`|424|`    let guard = kernel_artifact_inference_catalog().lock().expect("kernel artifact inference catalog lock");`|
|`🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️.rs`|425|`    let mut entries: Vec<KernelArtifactInferenceDescriptor> = guard.by_id.values().cloned().collect();`|
|`🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️.rs`|434|`pub struct KernelAppSchemaDescriptor {`|
|`🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️.rs`|436|`    pub config: KernelFacetLeaves,`|
|`🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️.rs`|437|`    pub presence: KernelFacetLeaves,`|
|`🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️.rs`|441|`    by_id: HashMap<&'static str, KernelAppSchemaDescriptor>,`|
|`🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️.rs`|446|`fn kernel_app_schema_catalog() -> &'static Mutex<KernelAppSchemaCatalog> {`|
|`🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️.rs`|451|`pub fn register_kernel_app_schema_descriptor(descriptor: KernelAppSchemaDescriptor) {`|
|`🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️.rs`|452|`    kernel_app_schema_catalog().lock().expect("kernel app schema catalog lock").by_id.insert(descriptor.id, descriptor);`|
|`🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️.rs`|456|`pub fn kernel_app_schema_descriptor_registered(id: &str) -> bool {`|
|`🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️.rs`|457|`    kernel_app_schema_catalog().lock().expect("kernel app schema catalog lock").by_id.contains_key(id)`|
|`🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️.rs`|461|`pub fn kernel_app_schema_catalog_len() -> usize {`|
|`🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️.rs`|462|`    kernel_app_schema_catalog().lock().expect("kernel app schema catalog lock").by_id.len()`|
|`🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️.rs`|466|`pub fn with_kernel_app_schema_catalog<R>(visit: impl FnOnce(&[KernelAppSchemaDescriptor]) -> R) -> R {`|
|`🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️.rs`|467|`    let guard = kernel_app_schema_catalog().lock().expect("kernel app schema catalog lock");`|
|`🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️.rs`|468|`    let mut entries: Vec<KernelAppSchemaDescriptor> = guard.by_id.values().cloned().collect();`|
|`🧰️framework/🔨️modules/📡️replication/🧾️wire/🧪️tests/🔬️unit/🦀️.rs`|426|`fn empty_kernel_facet_leaves() -> KernelFacetLeaves {`|
|`🧰️framework/🔨️modules/📡️replication/🧾️wire/🧪️tests/🔬️unit/🦀️.rs`|427|`    KernelFacetLeaves { rust: "", typescript: "", graphql: "", json_schema: "", proto: "" }`|
|`🧰️framework/🔨️modules/📡️replication/🧾️wire/🧪️tests/🔬️unit/🦀️.rs`|431|`fn kernel_artifact_inference_catalog_registers_independently_of_the_four_facet_descriptor() {`|
|`🧰️framework/🔨️modules/📡️replication/🧾️wire/🧪️tests/🔬️unit/🦀️.rs`|432|`    let before = kernel_artifact_inference_catalog_len();`|
|`🧰️framework/🔨️modules/📡️replication/🧾️wire/🧪️tests/🔬️unit/🦀️.rs`|433|`    register_kernel_artifact_inference_descriptor(KernelArtifactInferenceDescriptor { id: "s.wave3.synthetic.inference", inference: empty_kernel_facet_leaves() });`|
|`🧰️framework/🔨️modules/📡️replication/🧾️wire/🧪️tests/🔬️unit/🦀️.rs`|434|`    assert!(kernel_artifact_inference_descriptor_registered("s.wave3.synthetic.inference"));`|
|`🧰️framework/🔨️modules/📡️replication/🧾️wire/🧪️tests/🔬️unit/🦀️.rs`|435|`    assert_eq!(kernel_artifact_inference_catalog_len(), before.max(1));`|
|`🧰️framework/🔨️modules/📡️replication/🧾️wire/🧪️tests/🔬️unit/🦀️.rs`|437|`    with_kernel_artifact_inference_catalog(&#124;entries&#124; {`|
|`🧰️framework/🔨️modules/📡️replication/🧾️wire/🧪️tests/🔬️unit/🦀️.rs`|440|`    assert!(found, "registered inference descriptor must be visible via with_kernel_artifact_inference_catalog");`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔊️audio/🚪️io/🦀️.rs`|139|`        ::framework_schema::register_artifact_schema_descriptor(crate::standards::v1::subsets::audio::schema::semio_audio_artifact_schema_descriptor());`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔊️audio/🚪️io/🦀️.rs`|168|`    /// catalog — sibling to &#96;register_artifact_schema_descriptor&#96; above (separate registry,`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔊️audio/🚪️io/🦀️.rs`|172|`        ::framework_schema::register_artifact_inference_descriptor(crate::standards::v1::subsets::audio::schema::inferences::semio_audio_artifact_inference_descriptor());`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🛜️i-json/✏️editor/🧪️tests/🔬️unit/🦀️.rs`|7|`    semio_framework_schema::register_artifact_schema_descriptors(vec![crate::schema::json_artifact_schema_descriptor()]).expect("the json document schema registers");`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/✏️editor/🧪️tests/🔬️unit/🦀️.rs`|7|`    semio_framework_schema::register_artifact_schema_descriptors(vec![crate::schema::json_artifact_schema_descriptor()]).expect("the json document schema registers");`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🚪️io/🦀️.rs`|48|`        ::framework_schema::register_artifact_schema_descriptor(crate::standards::v1_0::subsets::any::schema::avi_artifact_schema_descriptor());`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🚪️io/🦀️.rs`|55|`    /// catalog — sibling to &#96;register_artifact_schema_descriptor&#96; above (separate registry,`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🚪️io/🦀️.rs`|59|`        ::framework_schema::register_artifact_inference_descriptor(crate::standards::v1_0::subsets::any::schema::inferences::avi_artifact_inference_descriptor());`|
|`🧰️framework/🔨️modules/🧬️schema/📇️registry/🦀️.rs`|304|`/// &#96;register_artifact_schema_descriptor&#96; so a descriptor's facets resolve through the same`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|209|`    // fixed-signature &#96;with_kernel_artifact_schema_catalog&#96;, and the &#96;semio-framework-plugin&#96;`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|250|`    register_kernel_app_schema_descriptor, register_kernel_artifact_inference_descriptor, register_kernel_artifact_schema_descriptor, with_kernel_app_schema_catalog, with_kernel_artifact_inference_catalog, with_kernel_artifact_schema_catalog,`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|251|`    KernelAppSchemaDescriptor, KernelArtifactInferenceDescriptor, KernelArtifactSchemaDescriptor, KernelFacetLeaves,`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|269|`fn facet_leaves_to_kernel(leaves: FacetLeaves) -> KernelFacetLeaves {`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|270|`    KernelFacetLeaves { rust: leaves.rust, typescript: leaves.typescript, graphql: leaves.graphql, json_schema: leaves.json_schema, proto: leaves.proto }`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|274|`// forced sync by the wire crate's fixed-signature &#96;with_kernel_artifact_schema_catalog&#96; closure.`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|275|`fn facet_leaves_from_kernel(leaves: &KernelFacetLeaves) -> FacetLeaves {`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|279|`fn descriptor_to_kernel(descriptor: &ArtifactSchemaDescriptor) -> KernelArtifactSchemaDescriptor {`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|280|`    KernelArtifactSchemaDescriptor {`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|291|`// &#96;with_kernel_artifact_schema_catalog&#96; (fixed signature outside this packet's scope).`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|292|`fn descriptor_from_kernel(kernel: &KernelArtifactSchemaDescriptor) -> ArtifactSchemaDescriptor {`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|319|`pub fn register_artifact_schema_descriptor(descriptor: ArtifactSchemaDescriptor) {`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|320|`    register_kernel_artifact_schema_descriptor(descriptor_to_kernel(&descriptor));`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|347|`pub fn register_artifact_schema_descriptors(descriptors: Vec<ArtifactSchemaDescriptor>) -> Result<(), SchemaDescriptorRegistryError> {`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|351|`            register_artifact_schema_descriptor(descriptor);`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|359|`    semio_framework_os_kernel::kernel_artifact_schema_descriptor_registered(id)`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|365|`    with_kernel_artifact_schema_catalog(&#124;entries&#124; {`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|376|`    with_kernel_artifact_schema_catalog(&#124;entries&#124; {`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|386|`    with_kernel_artifact_schema_catalog(&#124;entries&#124; {`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|407|`/// [&#96;ArtifactSchemaDescriptor&#96;], not a field on it (see [&#96;KernelArtifactInferenceDescriptor&#96;]'s own`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|415|`fn inference_descriptor_to_kernel(descriptor: &ArtifactInferenceDescriptor) -> KernelArtifactInferenceDescriptor {`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|416|`    KernelArtifactInferenceDescriptor { id: descriptor.id, inference: facet_leaves_to_kernel(descriptor.inference) }`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|420|`// &#96;with_artifact_inference_registry&#96; hands to &#96;with_kernel_artifact_inference_catalog&#96; (fixed`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|422|`fn inference_descriptor_from_kernel(kernel: &KernelArtifactInferenceDescriptor) -> ArtifactInferenceDescriptor {`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|473|`pub fn register_artifact_inference_descriptor(descriptor: ArtifactInferenceDescriptor) {`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|474|`    register_kernel_artifact_inference_descriptor(inference_descriptor_to_kernel(&descriptor));`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|502|`pub fn register_artifact_inference_descriptors(descriptors: Vec<ArtifactInferenceDescriptor>) -> Result<(), SchemaDescriptorRegistryError> {`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|506|`            register_artifact_inference_descriptor(descriptor);`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|514|`    semio_framework_os_kernel::kernel_artifact_inference_descriptor_registered(id)`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|520|`    with_kernel_artifact_inference_catalog(&#124;entries&#124; {`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|532|`    with_kernel_artifact_inference_catalog(&#124;entries&#124; {`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|543|`    with_kernel_artifact_inference_catalog(&#124;entries&#124; entries.iter().find(&#124;entry&#124; entry.id == key).map(&#124;entry&#124; graphql_leaf_with_preamble(entry.inference.graphql)))`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|620|`async fn app_descriptor_to_kernel(descriptor: &AppSchemaDescriptor) -> KernelAppSchemaDescriptor {`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|621|`    KernelAppSchemaDescriptor { id: descriptor.id, config: facet_leaves_to_kernel(descriptor.config), presence: facet_leaves_to_kernel(descriptor.presence) }`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|625|`// &#96;with_app_schema_registry&#96; hands to &#96;with_kernel_app_schema_catalog&#96; (fixed signature outside`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|627|`fn app_descriptor_from_kernel(kernel: &KernelAppSchemaDescriptor) -> AppSchemaDescriptor {`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|631|`/// 🔌 Open app-schema registry API for plugin crates — call these from your own &#96;🔧️setup&#96;/init code to register your app's config + presence schema facets. Every app owner self-registers via [&#96;register_app_schema_descriptor&#96;]; there is no closed framework-side catalog.`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|633|`/// - 📎 [&#96;register_app_schema_descriptor&#96;] registers one app owner's handcrafted descriptor into the OS-wide catalog.`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|639|`pub async fn register_app_schema_descriptor(descriptor: AppSchemaDescriptor) {`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|640|`    register_kernel_app_schema_descriptor(app_descriptor_to_kernel(&descriptor).await);`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|667|`pub async fn register_app_schema_descriptors(descriptors: Vec<AppSchemaDescriptor>) -> Result<(), SchemaDescriptorRegistryError> {`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|671|`            register_app_schema_descriptor(descriptor).await;`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|679|`    semio_framework_os_kernel::kernel_app_schema_descriptor_registered(id)`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|685|`    with_kernel_app_schema_catalog(&#124;entries&#124; {`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|696|`    with_kernel_app_schema_catalog(&#124;entries&#124; {`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|707|`    with_kernel_app_schema_catalog(&#124;entries&#124; {`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|721|`/// ✅ Validates a descriptor's JSON Schema leaves: each non-empty facet must be an object schema whose properties all carry a valid &#96;x-semio-state&#96; matching the facet's expected [&#96;StateClass&#96;] (&#96;config&#96; for config, &#96;presence&#96; for presence). Panics with a descriptor-id-prefixed message on the first violation — call this from a plugin's own tests before [&#96;register_app_schema_descriptor&#96;].`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|863|`/// registers its exports beside [&#96;register_artifact_schema_descriptor&#96;].`|
|`🧰️framework/🔨️modules/🧬️schema/🧪️tests/🔬️component-unit/🦀️.rs`|384|`    register_artifact_inference_descriptor(ArtifactInferenceDescriptor {`|
|`🧰️framework/🔨️modules/🧬️schema/🧪️tests/🔬️component-unit/🦀️.rs`|447|`    register_artifact_schema_descriptor(descriptor);`|
|`🧰️framework/🔨️modules/🧬️schema/🧪️tests/🔬️component-unit/🦀️.rs`|448|`    register_artifact_schema_descriptor(descriptor);`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🚪️io/🦀️.rs`|134|`        ::framework_schema::register_artifact_schema_descriptor(crate::standards::v1::subsets::base::schema::semio_artifact_schema_descriptor());`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🚪️io/🦀️.rs`|152|`    /// sibling to &#96;register_artifact_schema_descriptor&#96; above (separate registry, ticket`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🚪️io/🦀️.rs`|156|`        ::framework_schema::register_artifact_inference_descriptor(crate::standards::v1::subsets::base::schema::inferences::semio_artifact_inference_descriptor());`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`|6|`    framework_schema::register_artifact_schema_descriptors(vec![crate::standards::v1_2::subsets::any::schema::png_artifact_schema_descriptor()]).expect("the png document schema registers");`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖼️image/🚪️io/🦀️.rs`|127|`        ::framework_schema::register_artifact_schema_descriptor(crate::standards::v1::subsets::image::schema::semio_image_artifact_schema_descriptor());`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖼️image/🚪️io/🦀️.rs`|156|`    /// catalog — sibling to &#96;register_artifact_schema_descriptor&#96; above (separate registry,`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖼️image/🚪️io/🦀️.rs`|160|`        ::framework_schema::register_artifact_inference_descriptor(crate::standards::v1::subsets::image::schema::inferences::semio_image_artifact_inference_descriptor());`|
|`✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🦀️.rs`|49|`/// &#96;register_artifact_schema_descriptor&#96;/&#96;register_artifact_inference_descriptor&#96;/`|
|`✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🦀️.rs`|52|`/// &#96;.document_codec()&#96; call and no app-scope &#96;register_app_schema_descriptor&#96; escape hatch to keep —`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🚪️io/🦀️.rs`|126|`        ::framework_schema::register_artifact_schema_descriptor(crate::standards::v1::subsets::presentation::schema::semio_presentation_artifact_schema_descriptor());`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🚪️io/🦀️.rs`|155|`    /// inference catalog — sibling to &#96;register_artifact_schema_descriptor&#96; above (separate`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🚪️io/🦀️.rs`|159|`        ::framework_schema::register_artifact_inference_descriptor(crate::standards::v1::subsets::presentation::schema::inferences::semio_presentation_artifact_inference_descriptor());`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔤️text/🚪️io/🦀️.rs`|89|`        ::framework_schema::register_artifact_schema_descriptor(crate::standards::v1::subsets::text::schema::semio_text_artifact_schema_descriptor());`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔤️text/🚪️io/🦀️.rs`|111|`    /// catalog — sibling to &#96;register_artifact_schema_descriptor&#96; above (separate registry,`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔤️text/🚪️io/🦀️.rs`|115|`        ::framework_schema::register_artifact_inference_descriptor(crate::standards::v1::subsets::text::schema::inferences::semio_text_artifact_inference_descriptor());`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🚪️io/🦀️.rs`|146|`        ::framework_schema::register_artifact_schema_descriptor(crate::standards::v1::subsets::cad::schema::semio_cad_artifact_schema_descriptor());`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🚪️io/🦀️.rs`|173|`    /// catalog — sibling to &#96;register_artifact_schema_descriptor&#96; above (separate registry,`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🚪️io/🦀️.rs`|177|`        ::framework_schema::register_artifact_inference_descriptor(crate::standards::v1::subsets::cad::schema::inferences::semio_cad_artifact_inference_descriptor());`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🦀️.rs`|198|`    ::framework_schema::register_artifact_schema_descriptor(standards::v1_2::subsets::any::schema::png_artifact_schema_descriptor());`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🦀️.rs`|199|`    ::framework_schema::register_artifact_inference_descriptor(standards::v1_2::subsets::any::schema::inferences::png_artifact_inference_descriptor());`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📦️object/🚪️io/🦀️.rs`|112|`        ::framework_schema::register_artifact_schema_descriptor(crate::standards::v1::subsets::object::schema::semio_object_artifact_schema_descriptor());`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📦️object/🚪️io/🦀️.rs`|136|`    /// catalog — sibling to &#96;register_artifact_schema_descriptor&#96; above (separate registry,`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📦️object/🚪️io/🦀️.rs`|140|`        ::framework_schema::register_artifact_inference_descriptor(crate::standards::v1::subsets::object::schema::inferences::semio_object_artifact_inference_descriptor());`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🚪️io/🦀️.rs`|112|`        ::framework_schema::register_artifact_schema_descriptor(crate::standards::v1::subsets::kit::schema::semio_kit_artifact_schema_descriptor());`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🚪️io/🦀️.rs`|134|`    /// catalog — sibling to &#96;register_artifact_schema_descriptor&#96; above (separate registry,`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🚪️io/🦀️.rs`|138|`        ::framework_schema::register_artifact_inference_descriptor(crate::standards::v1::subsets::kit::schema::inferences::semio_kit_artifact_inference_descriptor());`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🚪️io/🦀️.rs`|88|`        ::framework_schema::register_artifact_schema_descriptor(crate::standards::v1::subsets::graph::schema::semio_graph_artifact_schema_descriptor());`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🚪️io/🦀️.rs`|112|`    /// catalog — sibling to &#96;register_artifact_schema_descriptor&#96; above (separate registry,`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🚪️io/🦀️.rs`|116|`        ::framework_schema::register_artifact_inference_descriptor(crate::standards::v1::subsets::graph::schema::inferences::semio_graph_artifact_inference_descriptor());`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🌊️flow/🚪️io/🦀️.rs`|121|`        ::framework_schema::register_artifact_schema_descriptor(crate::standards::v1::subsets::flow::schema::semio_flow_artifact_schema_descriptor());`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🌊️flow/🚪️io/🦀️.rs`|148|`    /// catalog — sibling to &#96;register_artifact_schema_descriptor&#96; above (separate registry,`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🌊️flow/🚪️io/🦀️.rs`|152|`        ::framework_schema::register_artifact_inference_descriptor(crate::standards::v1::subsets::flow::schema::inferences::semio_flow_artifact_inference_descriptor());`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎬️video/🚪️io/🦀️.rs`|135|`        ::framework_schema::register_artifact_schema_descriptor(crate::standards::v1::subsets::video::schema::semio_video_artifact_schema_descriptor());`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎬️video/🚪️io/🦀️.rs`|164|`    /// catalog — sibling to &#96;register_artifact_schema_descriptor&#96; above (separate registry,`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎬️video/🚪️io/🦀️.rs`|168|`        ::framework_schema::register_artifact_inference_descriptor(crate::standards::v1::subsets::video::schema::inferences::semio_video_artifact_inference_descriptor());`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🚪️io/🦀️.rs`|149|`        ::framework_schema::register_artifact_schema_descriptor(crate::standards::v1::subsets::mesh::schema::semio_mesh_artifact_schema_descriptor());`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🚪️io/🦀️.rs`|176|`    /// catalog — sibling to &#96;register_artifact_schema_descriptor&#96; above (separate registry,`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🚪️io/🦀️.rs`|180|`        ::framework_schema::register_artifact_inference_descriptor(crate::standards::v1::subsets::mesh::schema::inferences::semio_mesh_artifact_inference_descriptor());`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎞️animation/🚪️io/🦀️.rs`|127|`        ::framework_schema::register_artifact_schema_descriptor(crate::standards::v1::subsets::animation::schema::semio_animation_artifact_schema_descriptor());`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎞️animation/🚪️io/🦀️.rs`|156|`    /// catalog — sibling to &#96;register_artifact_schema_descriptor&#96; above (separate registry,`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎞️animation/🚪️io/🦀️.rs`|160|`        ::framework_schema::register_artifact_inference_descriptor(crate::standards::v1::subsets::animation::schema::inferences::semio_animation_artifact_inference_descriptor());`|
|`✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🦀️.rs`|520|`/// &#96;register_app_schema_descriptor&#96; is not in the §6 artifact-scoped set.`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🚪️io/🦀️.rs`|141|`        ::framework_schema::register_artifact_schema_descriptor(crate::standards::v1::subsets::brep::schema::semio_brep_artifact_schema_descriptor());`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🚪️io/🦀️.rs`|168|`    /// catalog — sibling to &#96;register_artifact_schema_descriptor&#96; above (separate registry,`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🚪️io/🦀️.rs`|172|`        ::framework_schema::register_artifact_inference_descriptor(crate::standards::v1::subsets::brep::schema::inferences::semio_brep_artifact_inference_descriptor());`|
|`✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🦀️.rs`|125|`/// (see that struct's own doc) — &#96;register_app_schema_descriptor&#96; is not in §6's artifact-scoped`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📊️table/🚪️io/🦀️.rs`|90|`        ::framework_schema::register_artifact_schema_descriptor(crate::standards::v1::subsets::table::schema::semio_table_artifact_schema_descriptor());`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📊️table/🚪️io/🦀️.rs`|114|`    /// catalog — sibling to &#96;register_artifact_schema_descriptor&#96; above (separate registry,`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📊️table/🚪️io/🦀️.rs`|118|`        ::framework_schema::register_artifact_inference_descriptor(crate::standards::v1::subsets::table::schema::inferences::semio_table_artifact_inference_descriptor());`|
|`✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🦀️.rs`|293|`/// (&#96;register_language&#96;/&#96;register_artifact_schema_descriptor&#96;/… all ARE §6 and now live in the builder`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🚪️io/🦀️.rs`|156|`        ::framework_schema::register_artifact_schema_descriptor(crate::standards::v1::subsets::drawing::schema::semio_drawing_artifact_schema_descriptor());`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🚪️io/🦀️.rs`|185|`    /// catalog — sibling to &#96;register_artifact_schema_descriptor&#96; above (separate registry,`|
|`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🚪️io/🦀️.rs`|189|`        ::framework_schema::register_artifact_inference_descriptor(crate::standards::v1::subsets::drawing::schema::inferences::semio_drawing_artifact_inference_descriptor());`|

### state

91 literal rows in 33 distinct authored Rust files. Byte/line receipt only, not semantic proof.

|Source|Line|Observed source|
|---|---:|---|
|`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`|303|`    register_fem2d_mutation_descriptors(::semio_framework_os_kernel::StateClass::Artifact).expect("mutation descriptor registration");`|
|`✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚙️operations/🧪️tests/🔬️unit/🦀️.rs`|90|`    register_vcs_demo_mutation_descriptors(::semio_framework_os_kernel::StateClass::Artifact).expect("mutation descriptor registration");`|
|`✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`|54|`    register_note_mutation_descriptors(::semio_framework_os_kernel::StateClass::Artifact).expect("mutation descriptor registration");`|
|`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`|245|`    register_generation2d_mutation_descriptors(::semio_framework_os_kernel::StateClass::Artifact).expect("mutation descriptor registration");`|
|`✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚙️operations/🧪️tests/🔬️unit/🦀️.rs`|82|`    register_procedure_mutation_descriptors(::semio_framework_os_kernel::StateClass::Artifact).expect("mutation descriptor registration");`|
|`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`|328|`    register_fem3d_mutation_descriptors(::semio_framework_os_kernel::StateClass::Artifact).expect("mutation descriptor registration");`|
|`✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`|132|`    register_playbook_mutation_descriptors(::semio_framework_os_kernel::StateClass::Artifact).expect("mutation descriptor registration");`|
|`✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`|130|`    register_drawing_mutation_descriptors(::semio_framework_os_kernel::StateClass::Artifact).expect("mutation descriptor registration");`|
|`🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🦀️.rs`|119|`    register_opening_config_mutation_descriptors(semio_framework_os_kernel::StateClass::Config)?;`|
|`🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🦀️.rs`|120|`    register_ui_preferences_config_mutation_descriptors(semio_framework_os_kernel::StateClass::Config)?;`|
|`🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🦀️.rs`|121|`    register_merge_policy_config_mutation_descriptors(semio_framework_os_kernel::StateClass::Config)?;`|
|`🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🦀️.rs`|122|`    register_identity_config_mutation_descriptors(semio_framework_os_kernel::StateClass::Config)?;`|
|`🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🦀️.rs`|123|`    register_local_catalog_config_mutation_descriptors(semio_framework_os_kernel::StateClass::Config)?;`|
|`🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🦀️.rs`|124|`    register_local_folders_config_mutation_descriptors(semio_framework_os_kernel::StateClass::Config)`|
|`✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚙️operations/🧪️tests/🔬️unit/🦀️.rs`|37|`    register_s_space_mutation_descriptors(::semio_framework_os_kernel::StateClass::Artifact).expect("mutation descriptor registration");`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️.rs`|441|`    state_class: crate::os_spr::StateClass,`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️.rs`|449|`    pub fn new(id: crate::os_spr::ids::SchemaId, schema_version: crate::os_spr::ids::SchemaVersion, state_class: crate::os_spr::StateClass, leaf: MutationLeafDescriptor, semantics: SemanticDescriptor) -> Result<Self, MutationDescriptorError> {`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️.rs`|476|`    pub fn state_class(&self) -> crate::os_spr::StateClass {`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️.rs`|490|`fn descriptor_fingerprint(id: &crate::os_spr::ids::SchemaId, schema_version: crate::os_spr::ids::SchemaVersion, state_class: crate::os_spr::StateClass, leaf: &MutationLeafDescriptor, semantics: &SemanticDescriptor) -> [u8; 32] {`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️.rs`|496|`        state_class: crate::os_spr::StateClass,`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️.rs`|602|`    pub state_class: crate::os_spr::StateClass,`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🔬️unit/🦀️.rs`|81|`    assert_eq!(op.state_class(), crate::os_spr::StateClass::Artifact);`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🔬️unit/🦀️.rs`|348|`    let construct = &#124;semantics&#124; MutationDescriptor::new(crate::os_spr::SchemaId("mini.doc#rename-mini".into()), crate::os_spr::SchemaVersion(1), crate::os_spr::StateClass::Artifact, RenameMini::DESCRIPTOR, semantics).unwrap();`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🔬️unit/🦀️.rs`|377|`    register_mini_mutation_descriptors(crate::os_spr::StateClass::Artifact).unwrap();`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🔬️unit/🦀️.rs`|378|`    register_mini_mutation_descriptors(crate::os_spr::StateClass::Artifact).unwrap();`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🔬️unit/🦀️.rs`|384|`    assert!(register_mini_mutation_descriptors(crate::os_spr::StateClass::Config).is_err());`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🔬️unit/🦀️.rs`|394|`    let first = build("mini.first", crate::os_spr::StateClass::Artifact);`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🔬️unit/🦀️.rs`|395|`    let conflict = build("mini.first", crate::os_spr::StateClass::Config);`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🔬️unit/🦀️.rs`|396|`    let second = build("mini.second", crate::os_spr::StateClass::Artifact);`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🔬️unit/🦀️.rs`|842|`        state_class: crate::os_spr::StateClass::Transient,`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🦀️.rs`|56|`    read_f64, read_str, read_varint_u64, write_f64, write_str, write_varint_u64, ActorId, ArtifactId, ArtifactVersion, HybridLogicalTimestamp, MergePolicy, MutationId, PayloadHash, SchemaId, SchemaVersion, StateClass, UndoPolicy,`|
|`✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚙️operations/🧪️tests/🔬️unit/🦀️.rs`|12|`    register_s_home_mutation_descriptors(::semio_framework_os_kernel::StateClass::Artifact).expect("mutation descriptor registration");`|
|`🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️.rs`|2196|`            state_class: ::semio_framework_os_kernel::StateClass,`|
|`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`|137|`    register_puzzle2d_mutation_descriptors(::semio_framework_os_kernel::StateClass::Artifact).expect("mutation descriptor registration");`|
|`✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚙️operations/🧪️tests/🔬️unit/🦀️.rs`|88|`    register_sequence_mutation_descriptors(::semio_framework_os_kernel::StateClass::Artifact).expect("mutation descriptor registration");`|
|`✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`|220|`    register_wires_mutation_descriptors(::semio_framework_os_kernel::StateClass::Artifact).expect("mutation descriptor registration");`|
|`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`|136|`    register_puzzle5d_mutation_descriptors(::semio_framework_os_kernel::StateClass::Artifact).expect("mutation descriptor registration");`|
|`🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️.rs`|291|`/// draft artifact's fields are still [&#96;StateClass::Artifact&#96;]. Derivation travels on its own axis`|
|`🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️.rs`|294|`pub enum StateClass {`|
|`🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️.rs`|304|`impl crate::value::ToValue for StateClass {`|
|`🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️.rs`|306|`        crate::value::DslValue::String(match self { StateClass::Artifact => "Artifact", StateClass::Config => "Config", StateClass::Presence => "Presence", StateClass::Transient => "Transient" }.to_string())`|
|`🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️.rs`|309|`impl crate::value::FromValue for StateClass {`|
|`🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️.rs`|313|`                "Artifact" => Ok(StateClass::Artifact),`|
|`🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️.rs`|314|`                "Config" => Ok(StateClass::Config),`|
|`🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️.rs`|315|`                "Presence" => Ok(StateClass::Presence),`|
|`🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️.rs`|316|`                "Transient" => Ok(StateClass::Transient),`|
|`🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️.rs`|317|`                other => Err(crate::value::ValueError::new(format!("unknown StateClass variant &#96;{other}&#96;"))),`|
|`🧰️framework/🔨️modules/📡️replication/🧾️wire/🧪️tests/🔬️unit/🦀️.rs`|416|`    let lanes = [StateClass::Artifact, StateClass::Config, StateClass::Presence, StateClass::Transient];`|
|`🧰️framework/🔨️modules/📡️replication/🧾️wire/🧪️tests/🔬️unit/🦀️.rs`|419|`        assert_eq!(<StateClass as crate::value::FromValue>::from_value(value).unwrap(), class);`|
|`🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs`|218|`    fn state_class(&self) -> crate::StateClass {`|
|`🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs`|219|`        crate::StateClass::Artifact`|
|`🧰️framework/🔨️modules/🧬️schema/✨️derive/⚙️expansion/🦀️.rs`|223|`                    (#camel, ::semio_framework_schema::StateClass::#variant_ident)`|
|`🧰️framework/🔨️modules/🧬️schema/✨️derive/⚙️expansion/🦀️.rs`|262|`            async fn field_states() -> &'static [(&'static str, ::semio_framework_schema::StateClass)] {`|
|`🧰️framework/🔨️modules/🧬️schema/🧪️tests/🔬️component-unit/🦀️.rs`|207|`        let mut derived: Vec<(String, StateClass)> = SyntheticSnapshot::field_states().await.iter().map(&#124;(name, class)&#124; ((*name).to_string(), *class)).collect();`|
|`🧰️framework/🔨️modules/🧬️schema/🧪️tests/🔬️component-unit/🦀️.rs`|219|`    assert!(GRAPHQL_STATE_PREAMBLE.contains("enum StateClass { ARTIFACT CONFIG PRESENCE TRANSIENT }"));`|
|`🧰️framework/🔨️modules/🧬️schema/🧪️tests/🔬️component-unit/🦀️.rs`|220|`    assert!(GRAPHQL_STATE_PREAMBLE.contains("directive @state(class: StateClass!) on FIELD_DEFINITION"));`|
|`🧰️framework/🔨️modules/🧬️schema/🧪️tests/🔬️component-unit/🦀️.rs`|226|`    for class in [StateClass::Artifact, StateClass::Config, StateClass::Presence, StateClass::Transient] {`|
|`🧰️framework/🔨️modules/🧬️schema/🧪️tests/🔬️component-unit/🦀️.rs`|230|`    assert_eq!(state_class_kebab(StateClass::Artifact).await, "artifact");`|
|`🧰️framework/🔨️modules/🧬️schema/🧪️tests/🔬️component-unit/🦀️.rs`|231|`    assert_eq!(state_class_kebab(StateClass::Config).await, "config");`|
|`🧰️framework/🔨️modules/🧬️schema/🧪️tests/🔬️component-unit/🦀️.rs`|232|`    assert_eq!(state_class_kebab(StateClass::Presence).await, "presence");`|
|`🧰️framework/🔨️modules/🧬️schema/🧪️tests/🔬️component-unit/🦀️.rs`|233|`    assert_eq!(state_class_kebab(StateClass::Transient).await, "transient");`|
|`🧰️framework/🔨️modules/🧬️schema/🧪️tests/🔬️component-unit/🦀️.rs`|244|`/// 💡️ Derivation travels on its own axis: &#96;#[derived]&#96; fields carry no [&#96;StateClass&#96;] and are`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|8|`pub use semio_framework_os_kernel::StateClass;`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|95|`enum StateClass { ARTIFACT CONFIG PRESENCE TRANSIENT }\n\`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|97|`directive @state(class: StateClass!) on FIELD_DEFINITION\n\`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|103|`/// ✨️ Per-artifact field → [&#96;StateClass&#96;] table emitted by [&#96;ArtifactSchema&#96;].`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|106|`/// snapshot rather than stored in any lane, so they carry no [&#96;StateClass&#96;] at all and are reported`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|111|`    fn field_states() -> impl std::future::Future<Output = &'static [(&'static str, StateClass)]> + Send;`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|721|`/// ✅ Validates a descriptor's JSON Schema leaves: each non-empty facet must be an object schema whose properties all carry a valid &#96;x-semio-state&#96; matching the facet's expected [&#96;StateClass&#96;] (&#96;config&#96; for config, &#96;presence&#96; for presence). Panics with a descriptor-id-prefixed message on the first violation — call this from a plugin's own tests before [&#96;register_app_schema_descriptor&#96;].`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|733|`            let expected = if facet == "config" { StateClass::Config } else { StateClass::Presence };`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|741|`/// 🏷️ Parse the canonical kebab &#96;x-semio-state&#96; string into [&#96;StateClass&#96;].`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|744|`/// without inventing a parallel source of truth. The kernel already owns [&#96;StateClass&#96;].`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|748|`pub fn parse_state_class_kebab(value: &str) -> Option<StateClass> {`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|750|`        "artifact" => Some(StateClass::Artifact),`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|751|`        "config" => Some(StateClass::Config),`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|752|`        "presence" => Some(StateClass::Presence),`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|753|`        "transient" => Some(StateClass::Transient),`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|758|`/// 🏷️ Canonical kebab spelling of a [&#96;StateClass&#96;] for JSON Schema &#96;x-semio-state&#96;.`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|759|`pub async fn state_class_kebab(class: StateClass) -> &'static str {`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|761|`        StateClass::Artifact => "artifact",`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|762|`        StateClass::Config => "config",`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|763|`        StateClass::Presence => "presence",`|
|`🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs`|764|`        StateClass::Transient => "transient",`|
|`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`|193|`    register_puzzle3d_mutation_descriptors(::semio_framework_os_kernel::StateClass::Artifact).expect("mutation descriptor registration");`|
|`✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚙️operations/🧪️tests/🔬️unit/🦀️.rs`|106|`    register_rewrite_rule_mutation_descriptors(::semio_framework_os_kernel::StateClass::Artifact).expect("mutation descriptor registration");`|
|`✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`|228|`    register_remodeling_mutation_descriptors(::semio_framework_os_kernel::StateClass::Artifact).expect("mutation descriptor registration");`|
|`✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚙️operations/🧪️tests/🔬️unit/🦀️.rs`|210|`    register_trinity_graph_mutation_descriptors(::semio_framework_os_kernel::StateClass::Artifact).expect("mutation descriptor registration");`|
|`✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`|160|`    register_dag_mutation_descriptors(::semio_framework_os_kernel::StateClass::Artifact).expect("mutation descriptor registration");`|
|`✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`|170|`    register_block2d_mutation_descriptors(::semio_framework_os_kernel::StateClass::Artifact).expect("mutation descriptor registration");`|
|`✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`|219|`    register_block5d_mutation_descriptors(::semio_framework_os_kernel::StateClass::Artifact).expect("mutation descriptor registration");`|
|`✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`|206|`    register_block3d_mutation_descriptors(::semio_framework_os_kernel::StateClass::Artifact).expect("mutation descriptor registration");`|

## Exact implementation registration scope, not yet edited

Existing owned router/project paths:

- `🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust/📜️script.ts`
- `🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust/📋️project.json`
- `🧰️framework/🔨️modules/🧬️schema/📇️registry/📦️packages/🦀️rust/📜️script.ts`
- `🧰️framework/🔨️modules/🧬️schema/📇️registry/📦️packages/🦀️rust/📋️project.json`

Future native lower owners require canonical Cargo/library/mount metadata, root workspace members, taxonomy facet/package/test contribution registration, exact script test subcommands and fresh Nx inputs. Main schema/router retains its existing default Cargo+Bun behavior. New portable stages use closed corpus and independent test-only AJV/Node/Serde oracles. Permanent input contracts belong under schema/🧩️composition, schema/📇️registry and the chosen narrow state owner, not ticket generated storage. Caller-owned generated outputs will be recreated solely under ticket 🗑️generated after Root releases actual execution. No launch slot is reserved here; fresh authored seed must be checked with Root before registration.

Current schema-version methods to preserve/rebind explicitly are artifact_schema_version, snapshot_schema_version, diff_schema_version and mutations_schema_version. Their higher Pack-backed implementation cannot move into dependency-free registry as an inherent impl on a foreign descriptor (Rust orphan/inherent rule); use owned explicit free functions or a schema-owned extension trait with direct client import, not a duplicate descriptor or forwarding legacy adapter. Facet lookup/order API can stay on canonical registry descriptor because it needs only std.

Read-only candidate rosters total150 composition rows/34files,230 catalog rows/60files and91 StateClass rows/33files across authored Framework/S/Hub. These overlap and include comments; do not add them as a unique-edit count. Every actual affected crate/multiple mount must be reconciled with captured source/manifest graph before claiming complete native ownership or absent-products closure. All five original source hashes exactly match Low's pre-cut roster, preserving the original higher law source provenance.
