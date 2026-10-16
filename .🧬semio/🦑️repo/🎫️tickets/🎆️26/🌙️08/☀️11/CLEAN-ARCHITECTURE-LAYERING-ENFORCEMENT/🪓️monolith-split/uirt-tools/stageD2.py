"""⏭️ Stage D2: decomposes the SurfaceReconciler god object into the SurfaceShadow component (cursor crate) and a thin owner (authority crate)."""
import re,sys,os
sys.path.insert(0,".")
from prim_names import prim_items,REPO,RT
R=f"{REPO}/{RT}"
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/uirt/snapA"
snap=open(f"{S}/core.rs",encoding="utf-8").read().split("\n")
def L(a,b): return "\n".join(snap[a-1:b])
CFG='#[cfg(any(test, feature = "testkit"))]'
def sub1(t,old,new):
    assert t.count(old)==1,(t.count(old),old[:90])
    return t.replace(old,new)
# ---------------------------------------------------------------- shadow
sh=L(30,344)
assert sh.startswith("//#region 🔖️Reconciler") and sh.endswith("//#endregion 🔖️Reconciler")
i0=sh.index("/// ♻️ Keyed differ for one render surface.")
i1=sh.index("impl SurfaceReconciler {")
new_struct='''/// ♻️ The retained shadow of one render surface — the component a keyed reconciliation diffs against. Owns a shadow
/// copy of what the receiver has (`document`, `root`) plus the `(parent, key) → id` index (`key_index`) that carries
/// every node's identity across frames, and the monotonic `allocator` that mints an id for a node the first time it
/// is ever seen. A completed reconcile is the only place any of these change together. The authority crate composes
/// it with the handback reservation it owes the process-wide registry.
#[derive(Debug)]
pub struct SurfaceShadow {
    surface: ui_contract::SurfaceId,
    revision: ui_contract::UiRevision,
    allocator: ui_contract::UiNodeIdAllocator,
    document: Option<ui_contract::UiDocumentLease>,
    native_resources: Option<UiNativeResources>,
    native_resource_loans: UiNativeResourceLoans,
    assembly: ui_contract::UiDocumentAssembly,
    ordinals: SurfaceLinearMap<ui_contract::UiNodeId, usize, SURFACE_RECONCILE_FIXED_NODES>,
    key_index: SurfaceLinearMap<NodeIdentity, ui_contract::UiNodeId, SURFACE_RECONCILE_FIXED_NODES>,
    root: Option<ui_contract::UiNodeId>,
    retire_scalar: u8,
    seal_phase: u8,
}

'''
sh=sh[:i0]+new_struct+sh[i1:]
sh=sub1(sh,"impl SurfaceReconciler {\n    /// 🌱️ A reconciler","impl SurfaceShadow {\n    /// 🌱️ A reconciler")
sh=sub1(sh,"    pub fn new(surface: ui_contract::SurfaceId) -> Self {\n        Self::from_surface_id(surface)\n    }\n\n","")
sh=sub1(sh,"    fn from_surface_id(surface: ui_contract::SurfaceId) -> Self {","    pub fn new(surface: ui_contract::SurfaceId) -> Self {")
sh=sh.replace("Self::from_surface_id(","Self::new(")
sh=sub1(sh,"            seal_phase: 0,\n            handback: None,\n            retirement_armed: true,\n","            seal_phase: 0,\n")
sh=sub1(sh,"    pub(crate) fn transaction_reader(&self) -> Self {","    pub fn transaction_reader(&self) -> Self {")
sh=sub1(sh,'''    #[cfg(any(test, feature = "testkit"))]
    pub(crate) fn close_transaction_oracle(&mut self) {
        while !self.retire_one() {}
    }

''',"")
sh=sub1(sh,"    fn read_record(&self,","    pub fn read_record(&self,")
sh=sub1(sh,"    fn seal_step(&mut self, usage: SurfaceReconcileUsage,","    pub fn seal_step(&mut self, owner_bytes: usize, usage: SurfaceReconcileUsage,")
sh=sub1(sh,"let fixed = size_of::<Self>() + size_of_val","let fixed = owner_bytes + size_of_val")
# retire_one
a=sh.index("    fn retire_one(&mut self) -> bool {")
b=sh.index("impl Drop for SurfaceReconciler {")
ret='''    /// 🧹️ One retirement unit of this shadow, answering whether the shadow is fully retired. The handback
    /// reservation the owner holds is released at the same step the monolithic owner released it, through the
    /// explicit `release` parameter, so the shadow never learns the registry that issued it.
    pub fn retire_one<H>(&mut self, handback: &mut Option<H>, release: impl FnOnce(H)) -> bool {
        if self.native_resources.is_some() || !self.native_resource_loans.terminal_is_empty() { return false; }
        if let Some(document) = self.document.as_mut() {
            if document.close_read_step_with_grant(1, SURFACE_COMPONENT_COPY_WORK_BYTES).expect("canonical read retirement preserves exact fault authority").complete { self.document = None; }
            return false;
        }
        if !self.assembly.terminal_is_empty() {
            self.assembly.close_step(1, SURFACE_COMPONENT_COPY_WORK_BYTES).expect("canonical assembly retirement preserves exact fault authority");
            return false;
        }
        if self.ordinals.take_first().is_some() {
            return false;
        }
        let indexed_identity = self.key_index.keys().next().cloned();
        if let Some(identity) = indexed_identity {
            self.key_index.remove(&identity);
            return false;
        }
        match self.retire_scalar {
            0 => self.root = None,
            1 => self.surface.0 = ui_contract::UiText::default(),
            2 => self.revision = ui_contract::UiRevision::default(),
            3 => self.allocator = ui_contract::UiNodeIdAllocator::default(),
            4 => self.seal_phase = 0,
            5 => {
                if let Some(handback) = handback.take() {
                    release(handback);
                }
            }
            _ => return true,
        }
        self.retire_scalar += 1;
        self.retire_scalar >= 6
    }

    /// 📦️ Moves every retained owner into a fresh shadow, leaving this one empty.
    pub fn take_all(&mut self) -> Self {
        Self {
            surface: take(&mut self.surface),
            revision: take(&mut self.revision),
            allocator: take(&mut self.allocator),
            document: self.document.take(),
            native_resources: self.native_resources.take(),
            native_resource_loans: take(&mut self.native_resource_loans),
            assembly: take(&mut self.assembly),
            ordinals: self.ordinals.take_all(),
            key_index: self.key_index.take_all(),
            root: self.root.take(),
            retire_scalar: self.retire_scalar,
            seal_phase: self.seal_phase,
        }
    }

    /// 🧾️ `true` when no retained owner is left to retire.
    pub fn terminal_is_empty(&self) -> bool {
        self.document.is_none() && self.native_resources.is_none() && self.native_resource_loans.terminal_is_empty() && self.assembly.terminal_is_empty() && self.ordinals.is_empty() && self.key_index.is_empty()
    }

    /// 📖️ `true` when the canonical root has been sealed.
    pub fn has_document(&self) -> bool {
        self.document.is_some()
    }

    /// 📖️ `true` when `lease` aliases this shadow's canonical root.
    pub fn document_root_matches(&self, lease: Option<&ui_contract::UiDocumentLease>) -> bool {
        self.document.as_ref().zip(lease).is_some_and(|(current, captured)| current.same_root(captured))
    }

    /// 🪪️ The byte length of the surface identifier this shadow was opened for.
    pub fn surface_identifier_bytes(&self) -> usize {
        self.surface.0.len()
    }
}

'''
# drop the Drop impl for the old owner
c=sh.index("struct SurfaceRecordRead<'a> {")
sh=sh[:a]+ret.rstrip("\n")[:ret.rindex("}\n\n")].rstrip("\n")+"\n}\n\n"+sh[c:] if False else sh[:a]+ret+sh[c:]
sh=sub1(sh,"impl SurfaceReconciler {\n    fn install_oracle","impl SurfaceShadow {\n    fn install_oracle")
sh=sub1(sh,"    fn install_fixture_record(","    pub fn install_fixture_record(")
sh=sub1(sh,"let mut credit = reserve_surface_reconcile(SurfaceReconcileLimits { max_bytes: 1024 * 1024, ..Default::default() });","let mut credit = reserve_surface_reconcile_floor(SurfaceReconcileLimits { max_bytes: 1024 * 1024, ..Default::default() });")
sh=sh.replace("SurfaceReconciler","SurfaceShadow")
# move the constructor doc onto `new`
doc_block=sh[sh.index("    /// 🌱️ A reconciler for"):sh.index("    #[cfg(any(test, feature = \"testkit\"))]\n    pub fn from_text")]
sh=sh.replace(doc_block,"")
sh=sub1(sh,"    pub fn new(surface: ui_contract::SurfaceId) -> Self {",doc_block+"    pub fn new(surface: ui_contract::SurfaceId) -> Self {")
# new() must precede from_text for readability: swap
ft_start=sh.index("    #[cfg(any(test, feature = \"testkit\"))]\n    pub fn from_text")
new_start=sh.index("    /// 🌱️ A reconciler for")
ft=sh[ft_start:new_start]
new_end=sh.index("    /// ♻️ Diffs `tree` against")
newfn=sh[new_start:new_end]
sh=sh[:ft_start]+newfn+ft+sh[new_end:]
sh=sub1(sh,"struct SurfaceRecordRead<'a> {","pub struct SurfaceRecordRead<'a> {")
DRAFT="/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/uirt"
open(f"{DRAFT}/shadow-draft.rs","w",encoding="utf-8").write(sh)
print(len(sh.split("\n")),"shadow lines")

# ---------------------------------------------------------------- cursor
cu=L(346,1382)
assert cu.startswith("//#region ⏭️ResumableReconcile") 
cu=cu.replace("&SurfaceReconciler","&SurfaceShadow")
cu=sub1(cu,"pub(crate) enum SurfaceReconcileStage","pub enum SurfaceReconcileStage")
cu=sub1(cu,"pub(crate) enum SurfaceReconcileStep","pub enum SurfaceReconcileStep")
cu=sub1(cu,"pub(crate) struct SurfaceReconcileCursor","pub struct SurfaceReconcileCursor")
cu=sub1(cu,"    Complete { reconciler: SurfaceReconciler, patch","    Complete { shadow: SurfaceShadow, patch")
cu=sub1(cu,"reason = \"Completion transfers the admitted reconciler and patch","reason = \"Completion transfers the admitted shadow and patch")
cu=sub1(cu,"    pub(crate) fn new(tree: ComponentTree,","    pub fn new(tree: ComponentTree,")
cu=sub1(cu,"    pub(crate) fn new_with_limits(","    pub fn new_with_limits(")
cu=sub1(cu,"        let credit = reserve_surface_reconcile(limits);","        let credit = surface_reconcile_limits_admissible(limits).then(|| reserve_surface_reconcile_floor(limits)).flatten();")
cu=sub1(cu,"    fn new_admitted(","    pub fn new_admitted(")
cu=sub1(cu,"    pub(crate) fn step(&mut self,","    pub fn step(&mut self,")
cu=sub1(cu,"    fn retire_one(&mut self) -> bool {","    pub fn retire_one(&mut self) -> bool {")
cu=sub1(cu,"    fn clear_fault(&mut self) {","    pub fn clear_fault(&mut self) {")
# completion
a=cu.index("                let reconciler = SurfaceReconciler {")
b=cu.index("                SurfaceReconcileStep::Complete { reconciler, patch }")
blk=cu[a:b]
blk=blk.replace("let reconciler = SurfaceReconciler {","let shadow = SurfaceShadow {")
blk=sub1(blk,"                    seal_phase: 0,\n                    handback: None,\n                    retirement_armed: true,\n","                    seal_phase: 0,\n")
cu=cu[:a]+blk+cu[b:].replace("Complete { reconciler, patch }","Complete { shadow, patch }",1)
assert "SurfaceReconciler" not in cu.replace("SurfaceReconciler","X",0) or True
# usage accessor after new_admitted block end: add before `pub fn step`
cu=sub1(cu,"    pub fn step(&mut self,","    pub fn usage(&self) -> SurfaceReconcileUsage {\n        self.usage\n    }\n\n    pub fn step(&mut self,")
cu=cu.replace("The retained reconciler is read-only until `Complete`","The retained shadow is read-only until `Complete`")
cu=cu.replace("SurfaceReconciler","SurfaceShadow")
cu=cu+"\n\n//#endregion ⏭️ResumableReconcile"
# ---------------------------------------------------------------- oracle
orc=L(2827,3015)
assert orc.startswith("//#region 🔖️Diff") and orc.endswith("//#endregion 🔖️Diff")
orc=orc.replace("&SurfaceReconciler","&SurfaceShadow").replace("SurfaceReconciler","SurfaceShadow")
open(f"{DRAFT}/cursor-slice.rs","w",encoding="utf-8").write(cu)
open(f"{DRAFT}/oracle-slice.rs","w",encoding="utf-8").write(orc)
print(len(cu.split("\n")),"cursor lines;",len(orc.split("\n")),"oracle lines")

# ---------------------------------------------------------------- authority (upper)
up=L(1384,2825)
assert up.startswith("//#region 🎟️RetainedAuthority") and up.endswith("//#endregion ⏭️ResumableReconcile")
# remove moved credit helpers (floor consts/asserts, release fns) and recompose reserve
a=up.index("/// 🎟️ What one reconcile reservation costs BEFORE")
b=up.index("fn reserve_surface_reconcile(limits: SurfaceReconcileLimits)")
up=up[:a]+up[b:]
a=up.index("fn reserve_surface_reconcile(limits: SurfaceReconcileLimits)")
b=up.index("fn register_surface_reconcile_backing(")
up=up[:a]+'''fn reserve_surface_reconcile(limits: SurfaceReconcileLimits) -> Option<ui_contract::UiResidentPermit> {
    if !surface_reconcile_limits_admissible(limits) { return None; }
    if !register_surface_reconcile_backing(SURFACE_RECONCILE_PAGE_BYTES).ok()? { return None; }
    reserve_surface_reconcile_floor(limits)
}

'''+up[b:]
a=up.index("fn release_surface_reconcile(mut credit")
b=up.index("/// 🎫️ Pre-materialization aggregate reservation")
up=up[:a]+up[b:]
up=sub1(up,"        Ok(Self { generation, revision: current.revision, lease, ready: false, fault: None })","        Ok(Self { generation, revision: current.revision(), lease, ready: false, fault: None })")
up=sub1(up,"if !current.document.as_ref().zip(self.lease.as_ref()).is_some_and(|(current, captured)| current.same_root(captured)) {","if !current.shadow.document_root_matches(self.lease.as_ref()) {")
up=sub1(up,"StaleRevision { expected: self.revision, actual: current.revision });","StaleRevision { expected: self.revision, actual: current.revision() });")
assert up.count("let surface_bytes = current.surface.0.len();")==2
up=up.replace("let surface_bytes = current.surface.0.len();","let surface_bytes = current.shadow.surface_identifier_bytes();")
assert up.count("SurfaceReconcileCursor::new_admitted(tree, &current, ")==2
up=up.replace("SurfaceReconcileCursor::new_admitted(tree, &current, ","SurfaceReconcileCursor::new_admitted(tree, &current.shadow, ")
up=sub1(up,"map_or(state.usage, |cursor| cursor.usage)","map_or(state.usage, SurfaceReconcileCursor::usage)")
up=sub1(up,"match cursor.step(current) {","match cursor.step(&current.shadow) {")
up=sub1(up,"SurfaceReconcileStep::Complete { reconciler, patch } => {","SurfaceReconcileStep::Complete { shadow, patch } => {")
up=sub1(up,"state.usage = cursor.usage;","state.usage = cursor.usage();")
up=sub1(up,"state.candidate = Some(reconciler);","state.candidate = Some(SurfaceReconciler::from_shadow(shadow));")
up=sub1(up,"if candidate.document.is_none() || candidate.handback.is_some()","if !candidate.shadow.has_document() || candidate.handback.is_some()")
up=sub1(up,"//#region 🎟️RetainedAuthority","//#region ⏭️ResumableReconcile\n\n//#region 🎟️RetainedAuthority")
up=up[:up.rindex("//#endregion ⏭️ResumableReconcile")].rstrip("\n")+"\n\n//#endregion ⏭️ResumableReconcile"
open(f"{DRAFT}/upper-slice.rs","w",encoding="utf-8").write(up)
print(len(up.split("\n")),"upper lines")

# ---------------------------------------------------------------- thin owner
owner='''//#region 🔖️Reconciler

/// ♻️ Keyed differ for one render surface: the retained [`SurfaceShadow`] it diffs against, composed with the
/// handback reservation it owes the process-wide registry. The shadow owns every retained owner and its
/// retirement; this owner only adds the registry hand-off that outlives it.
#[derive(Debug)]
pub struct SurfaceReconciler {
    shadow: SurfaceShadow,
    handback: Option<SurfaceReconcileHandbackReservation>,
    retirement_armed: bool,
}

impl SurfaceReconciler {
    /// 🌱️ A reconciler for `surface` with no retained state yet — the next reconcile necessarily emits a full
    /// `SetRoot` plus one `Upsert` per node.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn new(surface: ui_contract::SurfaceId) -> Self {
        Self::from_shadow(SurfaceShadow::new(surface))
    }

    fn from_shadow(shadow: SurfaceShadow) -> Self {
        Self { shadow, handback: None, retirement_armed: true }
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn from_text(surface: impl AsRef<str>) -> Self {
        Self::from_shadow(SurfaceShadow::from_text(surface))
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn reconcile(&mut self, tree: &ComponentTree) -> Option<ui_contract::UiPatch> {
        self.shadow.reconcile(tree)
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn snapshot(&self) -> ui_contract::UiSnapshot {
        self.shadow.snapshot()
    }

    /// 🧬️ Returns the retained scalar revision without cloning the retained document.
    pub fn revision(&self) -> ui_contract::UiRevision {
        self.shadow.revision()
    }

    /// 🪪️ Returns the reconciler's surface identity without allocating a second owner.
    pub fn surface(&self) -> &ui_contract::SurfaceId {
        self.shadow.surface()
    }

    /// 📖️ Captures the canonical root without a second payload tree or reservation.
    pub fn capture_document(&self, target: &mut Option<ui_contract::UiDocumentLease>, admitted_bytes: usize) -> Result<bool, ui_contract::UiDocumentLeaseError> {
        self.shadow.capture_document(target, admitted_bytes)
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn transaction_reader(&self) -> Self {
        Self::from_shadow(self.shadow.transaction_reader())
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn close_transaction_oracle(&mut self) {
        while !self.retire_one() {}
    }

    fn seal_step(&mut self, usage: SurfaceReconcileUsage, output: &mut Option<ui_contract::UiResidentPermit>, has_output: bool) -> Result<bool, ui_contract::UiDocumentAssemblyError> {
        self.shadow.seal_step(size_of::<Self>(), usage, output, has_output)
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn mark_rejected(&mut self) {
        self.shadow.mark_rejected();
    }

    pub fn native_resources(&self) -> Option<&UiNativeResources> { self.shadow.native_resources() }

    pub fn native_resource_loans_terminal_is_empty(&self)->bool{self.shadow.native_resource_loans_terminal_is_empty()}

    pub fn with_native_resource_loans_on_original<R>(&mut self,cx:&mut semio_framework_job::StepContext<'_>,receive:impl FnOnce(&mut UiNativeResourceLoans,&mut semio_framework_job::StepContext<'_>)->Result<R,semio_framework_job::OriginalTurnFailure>)->Result<Option<R>,semio_framework_job::OriginalTurnFailure>{
        self.shadow.with_native_resource_loans_on_original(cx,receive)
    }

    pub fn close_native_resources_on_original(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<bool>, semio_framework_job::OriginalTurnFailure> {
        self.shadow.close_native_resources_on_original(cx)
    }

    fn retire_one(&mut self) -> bool {
        self.shadow.retire_one(&mut self.handback, release_surface_reconcile_handback)
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn install_fixture_record(&mut self, record: ui_contract::UiNodeRecord) {
        self.shadow.install_fixture_record(record);
    }
}

impl Drop for SurfaceReconciler {
    fn drop(&mut self) {
        if !self.retirement_armed {
            return;
        }
        self.retirement_armed = false;
        if self.shadow.terminal_is_empty() {
            if let Some(handback) = self.handback.take() {
                release_surface_reconcile_handback(handback);
            }
            return;
        }
        let generation = self.handback.as_ref().map_or(0, |reservation| reservation.key.generation);
        let owner = SurfaceReconciler { shadow: self.shadow.take_all(), handback: None, retirement_armed: false };
        let state = Box::new(SurfaceReconcileRetained {
            output_handback: None,
            generation,
            phase: SurfaceReconcileJobPhase::Closing,
            current: Some(owner),
            source: None,
            cursor: None,
            candidate: None,
            patch: ui_contract::UiPendingPatch::default(),
            retire_tree: SurfaceTreeRetireCursor::default(),
            fault: None,
            usage: SurfaceReconcileUsage::default(),
            credit: None,
            handback: self.handback.take(),
        });
        handback_surface_reconcile(state);
    }
}

//#endregion 🔖️Reconciler'''
open(f"{DRAFT}/owner-slice.rs","w",encoding="utf-8").write(owner)
