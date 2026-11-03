#!/usr/bin/env python3
"""Apply from the repo root in a patch window. The durable-group companion test crate (`os-store-durable-group-tests`) asserts
the binding ladder through its retained state (`binding_step`, `retirement_progress`, `decision`, the captured issuers of each
prepared member, the commit coordinator's `value`/`decision_pack`). Those fields were narrowed to `pub(super)`/private;
the tests cannot compile against them (E0616 x ~30). Restore exactly the fields the laws read as `pub`. Fail-closed: one anchor each."""
R="🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/"
def rep(p,pairs):
    s=open(R+p).read()
    for a,b in pairs:
        assert s.count(a)==1,(p,a)
        s=s.replace(a,b)
    open(R+p,"w").write(s)
rep("🧩️composition/🗄️durable-group/📒️group-decision-journal-record-and-snapshot-capture/🦀️.rs",[
 ("    pub(super) decision: Option<Box<DurableOwnedThreeMemberDecisionV1>>,\n    pub(super) decision_pack: Option<Vec<u8>>,\n    pub(super) binding_step: u8,","    pub decision: Option<Box<DurableOwnedThreeMemberDecisionV1>>,\n    pub(super) decision_pack: Option<Vec<u8>>,\n    pub binding_step: u8,"),
 ("    pub(super) retirement_progress:semio_framework_value::RetainedCloneProgress,\n    #[cfg(feature = \"deflate\")]\n    pub(super) decision_encoding","    pub retirement_progress:semio_framework_value::RetainedCloneProgress,\n    #[cfg(feature = \"deflate\")]\n    pub(super) decision_encoding")])
rep("⚙️engine/🗄️stage/📥️lifted/🗄️durable-group/🦀️.rs",[
 ("    mutation_retirement: Option<Arc<dyn semio_framework_value::ArtifactOwnedValueRetirementFactory<Mutation>>>,\n    snapshot_retirement: Option<","    pub mutation_retirement: Option<Arc<dyn semio_framework_value::ArtifactOwnedValueRetirementFactory<Mutation>>>,\n    pub snapshot_retirement: Option<")])
rep("🧩️composition/🗄️durable-group/🛣️map-commit-recovery-hosts-and-assembly-owners/🦀️.rs",[
 ("    pub(super) value: Option<DurableStoreBoundOutcomeV1<ValueP, ValueMutation>>,\n    pub(super) decision: Option<Box<DurableOwnedThreeMemberDecisionV1>>,","    pub value: Option<DurableStoreBoundOutcomeV1<ValueP, ValueMutation>>,\n    pub(super) decision: Option<Box<DurableOwnedThreeMemberDecisionV1>>,"),
 ("    pub(super) decision_pack: Option<Vec<u8>>,\n    pub(super) decision_sha256: String,","    pub decision_pack: Option<Vec<u8>>,\n    pub(super) decision_sha256: String,")])
