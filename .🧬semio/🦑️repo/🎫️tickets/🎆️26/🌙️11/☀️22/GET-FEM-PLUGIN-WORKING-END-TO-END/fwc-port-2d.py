import re, sys
root = r"C:\semio\✏️s\🔌️plugins\🏗️fem\🗿️artifacts\◻️2d\🏅️standards\🔖️1\🪆️subsets\🌐️any\✏️editor\🎭️modes\✏️edit\🪟️windows"
CR = "store::os_store::codec_registry::native_snapshot"

def read(p):
    with open(p, encoding="utf-8", newline="") as f:
        return f.read()

def write(p, s):
    with open(p, "w", encoding="utf-8", newline="") as f:
        f.write(s)

for kind in ("🧱️model", "📊️results"):
    base = root + "\\" + kind + "\\🎚️config\\"
    name = "Fem2dModelWindowConfig" if kind == "🧱️model" else "Fem2dResultsWindowConfig"
    cfg = read(base + "🦀️.rs")
    if "OriginalArtifactPackReceiving" in cfg:
        print("skip", kind)
        continue
    cfg = cfg.replace("semio_framework_value::CanonicalJsonTree)]", "semio_framework_value::CanonicalJsonTree, semio_framework_value::NativeReceive)]")
    receiving = (
        f"impl {CR}::OriginalArtifactPackReceiving for {name} {{\n"
        f"    type Receiving = {CR}::OriginalSnapshotRecordReceiving<Self>;\n"
        "    fn original_pack_birth_demand() -> semio_framework_value::RetirementDemand { semio_framework_value::RetirementDemand { copy_bytes: std::mem::size_of::<Self::Receiving>(), depth: 1, ..Default::default() } }\n"
        "    fn admit_original_pack(source: &[u8], limits: protocol::PackLimits, native: &semio_framework_value::NativeDecodeControl<'_>, grant: semio_framework_value::RetainedCloneGrant) -> Result<Option<(Self::Receiving, semio_framework_value::RetainedCloneProgress)>, semio_framework_value::ValueError> {\n"
        "        Self::Receiving::admit(source, <Self as pack::snapshot::ArtifactDsl>::envelope_id(), limits, native, grant)\n"
        "    }\n"
        "}\n"
    )
    marker = f"impl ::semio_os_kernel::os_store::serialization::ArtifactStoreSnapshot for {name}"
    assert marker in cfg
    cfg = cfg.replace(marker, receiving + marker, 1)
    mut = name + "Mutation"
    targets = (
        f"impl store::ArtifactOperationTargets for {mut} {{\n"
        "    fn len(&self) -> usize { 1 }\n"
        "    fn segment_len(&self, index: usize) -> Result<usize, semio_framework_value::ValueError> {\n"
        "        if index == 0 { Ok(6) } else { Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, \"Window target ordinal is absent\")) }\n"
        "    }\n"
        "    fn read_segment(&self, index: usize, offset: usize, output: &mut [u8]) -> Result<usize, semio_framework_value::ValueError> {\n"
        "        if index != 0 || offset > 6 { return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, \"Window target ordinal is absent\")); }\n"
        "        let source = &b\"update\"[offset..];\n"
        "        let count = source.len().min(output.len());\n"
        "        output[..count].copy_from_slice(&source[..count]);\n"
        "        Ok(count)\n"
        "    }\n"
        "}\n\n"
    )
    marker = f"impl protocol::OpText for {mut}"
    assert marker in cfg
    cfg = cfg.replace(marker, targets + marker, 1)
    owner_methods = (
        "    const MAXIMUM_PREPARATION_DEPTH: usize = 64;\n"
        "    fn native_operation_wire() -> store::ArtifactOperationWire<Self::Mutation> {\n"
        "        store::ArtifactOperationWire::canonical().with_original_binary_receiving().with_original_target(|operation| Some(operation))\n"
        "    }\n"
        "    fn native_pair_demands() -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> { Ok(semio_plugin::window_config_native_pair_demands::<Self::State>()) }\n"
        f"    fn admit_native_pair(pack: &[u8], spr: &[u8], pack_limits: protocol::PackLimits, history_limits: {CR}::OriginalArtifactHistoryReadLimits, native: &mut semio_framework_value::NativeDecodeControl<'_>, grant: semio_framework_value::RetainedCloneGrant, output: &mut Option<Box<dyn semio_plugin::WindowConfigNativePairReceiving<Self::State>>>) -> Result<semio_framework_value::RetainedCloneProgress, {CR}::OriginalArtifactPairFailure> {{ semio_plugin::admit_window_config_native_pair::<Self::State>(pack, spr, pack_limits, history_limits, native, grant, output) }}\n"
    )
    marker = "    const MAXIMUM_PREPARATION_DEPTH: usize = 64;\n"
    assert marker in cfg
    cfg = cfg.replace(marker, owner_methods, 1)
    write(base + "🦀️.rs", cfg)
    sch = read(base + "🧬️schema\\🦀️.rs")
    sch = sch.replace("semio_framework_value::CanonicalJsonTree)]", "semio_framework_value::CanonicalJsonTree, semio_framework_value::NativeReceive)]")
    sch = sch.replace("#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, semio_os_kernel::DslArtifact", "#[derive(semio_framework_dsl_record_derive::DslRecord, semio_framework_dsl_record_derive::DslReceiving, Clone, Debug, PartialEq, semio_os_kernel::DslArtifact", 1)
    write(base + "🧬️schema\\🦀️.rs", sch)
    print("ported", kind)
