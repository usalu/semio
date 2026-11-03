#!/usr/bin/env python3
"""Second half of apply-puzzle3d-window-config-birth.py: `admit_window_config_native_pair::<State>` needs `State: OriginalArtifactPackReceiving` (energy's window state implements it the same way)."""
p="✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🦀️.rs"
s=open(p).read()
a="\n//#region 🪢️TaxonomyMounts\n"
assert s.count(a)==1 and "OriginalArtifactPackReceiving for Puzzle3dWindowConfig" not in s
b="""
impl store::os_store::codec_registry::native_snapshot::OriginalArtifactPackReceiving for Puzzle3dWindowConfig{
    type Receiving=store::os_store::codec_registry::native_snapshot::OriginalSnapshotRecordReceiving<Self>;
    fn original_pack_birth_demand()->semio_framework_value::RetirementDemand{semio_framework_value::RetirementDemand{copy_bytes:std::mem::size_of::<Self::Receiving>(),depth:1,..Default::default()}}
    fn admit_original_pack(source:&[u8],limits:protocol::PackLimits,native:&semio_framework_value::NativeDecodeControl<'_>,grant:semio_framework_value::RetainedCloneGrant)->Result<Option<(Self::Receiving,semio_framework_value::RetainedCloneProgress)>,semio_framework_value::ValueError>{Self::Receiving::admit(source,<Self as pack::snapshot::ArtifactDsl>::envelope_id(),limits,native,grant)}
}
"""
open(p,"w").write(s.replace(a,b+a,1));print("applied")
