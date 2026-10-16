# 🔗️ F7: the lifecycle close owner reaches the patch tracker and the cold pair ingress through a port supplied per call (no lifecycle -> higher crate edge).
from common import *
import sys
def edit(path, pairs):
    p = PLUG + '/' + path
    t = open(p, encoding='utf-8').read()
    for old, new in pairs:
        assert t.count(old) == 1, (path, t.count(old), old[:90])
        t = t.replace(old, new)
    open(p, 'w', encoding='utf-8').write(t)
LT = '⚛️reactor/🚪️lifetime/🦀️.rs'
edit(LT, [
("""pub(crate) trait GuestLifetimeOwner: terminal_owner::Sealed {
    fn terminal_is_empty(&self) -> Result<bool, semio_framework::Fault>;
    fn release_terminal(owner: &mut Option<Self>, maximum_items: usize, maximum_bytes: usize) -> Result<GuestTerminalRelease, semio_framework::Fault>
    where
        Self: Sized;
}
""","""pub(crate) trait GuestLifetimeOwner: terminal_owner::Sealed {
    type Peers: ?Sized;
    fn terminal_is_empty(&self, peers: &Self::Peers) -> Result<bool, semio_framework::Fault>;
    fn release_terminal(owner: &mut Option<Self>, maximum_items: usize, maximum_bytes: usize, peers: &Self::Peers) -> Result<GuestTerminalRelease, semio_framework::Fault>
    where
        Self: Sized;
}

/// 🔗️ The close participants that sit above the lifecycle owner, the patch tracker and the cold pair ingress: the owner drives them
/// through this port, which the reactor supplies on every call, so the lifecycle names no crate above itself. Every method keeps the
/// reserve, activate, complete, release order of the owner's own participants and answers with the participant's own `Fault`.
pub(crate) trait ClosePeers {
    fn preflight_patches(&self, key: NativeCloseKey) -> Result<(), semio_framework::Fault>;
    fn reserve_patches(&self, key: NativeCloseKey) -> Result<(), semio_framework::Fault>;
    fn activate_patches(&self, key: NativeCloseKey) -> Result<(), semio_framework::Fault>;
    fn patches_complete(&self, key: NativeCloseKey) -> Result<bool, semio_framework::Fault>;
    fn release_patches(&self, key: NativeCloseKey) -> Result<(), semio_framework::Fault>;
    fn preflight_cold_pair(&self, key: NativeCloseKey) -> Result<(), semio_framework::Fault>;
    fn reserve_cold_pair(&self, key: NativeCloseKey) -> Result<(), semio_framework::Fault>;
    fn activate_cold_pair(&self, key: NativeCloseKey) -> Result<(), semio_framework::Fault>;
    fn cold_pair_complete(&self, key: NativeCloseKey) -> Result<bool, semio_framework::Fault>;
    fn release_cold_pair(&self, key: NativeCloseKey) -> Result<(), semio_framework::Fault>;
}
"""),
("""    fn prepare_retired(&mut self) -> Result<bool, semio_framework::Fault> {""","""    fn prepare_retired(&mut self, peers: &O::Peers) -> Result<bool, semio_framework::Fault> {"""),
("""        if !self.owner.as_ref().ok_or_else(|| super::reactor_close_fault("guest lifetime lost its native owner"))?.terminal_is_empty()? {""","""        if !self.owner.as_ref().ok_or_else(|| super::reactor_close_fault("guest lifetime lost its native owner"))?.terminal_is_empty(peers)? {"""),
("""    fn release_owner_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<(), semio_framework::Fault> {""","""    fn release_owner_step(&mut self, maximum_items: usize, maximum_bytes: usize, peers: &O::Peers) -> Result<(), semio_framework::Fault> {"""),
("""        if !self.owner.as_ref().ok_or_else(|| super::reactor_close_fault("terminal receipt lost its owner"))?.terminal_is_empty()? {""","""        if !self.owner.as_ref().ok_or_else(|| super::reactor_close_fault("terminal receipt lost its owner"))?.terminal_is_empty(peers)? {"""),
("""        match O::release_terminal(&mut self.owner, maximum_items, maximum_bytes)? {""","""        match O::release_terminal(&mut self.owner, maximum_items, maximum_bytes, peers)? {"""),
("""    pub(crate) fn request_close(&mut self, request: ActorInstanceCloseRequest, runtime: &crate::plugin_runtime::PluginRuntime<PA>) -> Result<(), semio_framework::Fault> {""","""    pub(crate) fn request_close(&mut self, request: ActorInstanceCloseRequest, runtime: &crate::plugin_runtime::PluginRuntime<PA>, peers: &dyn ClosePeers) -> Result<(), semio_framework::Fault> {"""),
("""        super::PATCHES.with(|patches| patches.preflight_close_instance(self.key)).map_err(super::reactor_close_fault)?;
        super::pending::with_state(|pending| pending.try_borrow().map_err(|_| "pending close preflight busy")?.preflight_close_instance(self.key)).map_err(super::reactor_close_fault)?;
        super::preflight_cold_pair_close(self.key)?;""","""        peers.preflight_patches(self.key)?;
        super::pending::with_state(|pending| pending.try_borrow().map_err(|_| "pending close preflight busy")?.preflight_close_instance(self.key)).map_err(super::reactor_close_fault)?;
        peers.preflight_cold_pair(self.key)?;"""),
("""    pub(crate) fn advance_admission(&mut self, runtime: &crate::plugin_runtime::PluginRuntime<PA>) -> Result<Option<(ActorInstanceCloseRequest, u64)>, semio_framework::Fault> {""","""    pub(crate) fn advance_admission(&mut self, runtime: &crate::plugin_runtime::PluginRuntime<PA>, peers: &dyn ClosePeers) -> Result<Option<(ActorInstanceCloseRequest, u64)>, semio_framework::Fault> {"""),
("""            super::PATCHES.with(|patches| patches.reserve_close_instance(self.key)).map_err(super::reactor_close_fault)?;
            self.reserved[1] = true;""","""            peers.reserve_patches(self.key)?;
            self.reserved[1] = true;"""),
("""            super::reserve_cold_pair_close(self.key)?;
            self.reserved[3] = true;""","""            peers.reserve_cold_pair(self.key)?;
            self.reserved[3] = true;"""),
("""            super::activate_cold_pair_close(self.key)?;
            self.active[3] = true;""","""            peers.activate_cold_pair(self.key)?;
            self.active[3] = true;"""),
("""            super::PATCHES.with(|patches| patches.activate_close_instance(self.key)).map_err(super::reactor_close_fault)?;
            self.active[1] = true;""","""            peers.activate_patches(self.key)?;
            self.active[1] = true;"""),
("""impl<PA: crate::plugin_app::PluginApp> GuestLifetimeOwner for NativeLifetimeOwner<PA> {
    fn terminal_is_empty(&self) -> Result<bool, semio_framework::Fault> {""","""impl<PA: crate::plugin_app::PluginApp> GuestLifetimeOwner for NativeLifetimeOwner<PA> {
    type Peers = dyn ClosePeers;
    fn terminal_is_empty(&self, peers: &dyn ClosePeers) -> Result<bool, semio_framework::Fault> {"""),
("""        if !self.released[1] && !super::PATCHES.with(|patches| patches.close_instance_complete(self.key)).map_err(super::reactor_close_fault)? {""","""        if !self.released[1] && !peers.patches_complete(self.key)? {"""),
("""        if !self.released[3] && !super::cold_pair_close_complete(self.key).map_err(super::reactor_close_fault)? {""","""        if !self.released[3] && !peers.cold_pair_complete(self.key)? {"""),
("""    fn release_terminal(owner: &mut Option<Self>, maximum_items: usize, maximum_bytes: usize) -> Result<GuestTerminalRelease, semio_framework::Fault> {
        if maximum_items == 0 || maximum_bytes < size_of::<Self>() {""","""    fn release_terminal(owner: &mut Option<Self>, maximum_items: usize, maximum_bytes: usize, peers: &dyn ClosePeers) -> Result<GuestTerminalRelease, semio_framework::Fault> {
        if maximum_items == 0 || maximum_bytes < size_of::<Self>() {"""),
("""        if !retained.terminal_is_empty()? {
            return Ok(GuestTerminalRelease::Pending);""","""        if !retained.terminal_is_empty(peers)? {
            return Ok(GuestTerminalRelease::Pending);"""),
("""            super::PATCHES.with(|patches| patches.release_close_instance(retained.key)).map_err(super::reactor_close_fault)?;
            retained.released[1] = true;""","""            peers.release_patches(retained.key)?;
            retained.released[1] = true;"""),
("""            super::release_cold_pair_close(retained.key).map_err(super::reactor_close_fault)?;
            retained.released[3] = true;""","""            peers.release_cold_pair(retained.key)?;
            retained.released[3] = true;"""),
("""    pub(crate) fn prepare_turn(&mut self, instance: u32) -> Result<Option<ActorInstanceLifecycleReceipt>, semio_framework::Fault> {
        let slot = self.get_mut(instance).ok_or_else(|| super::reactor_close_fault("turn lifecycle missing"))?;
        slot.cell.prepare_retired()?;
        slot.cell.release_owner_step(1, 4096)?;""","""    pub(crate) fn prepare_turn(&mut self, instance: u32, peers: &dyn ClosePeers) -> Result<Option<ActorInstanceLifecycleReceipt>, semio_framework::Fault> {
        let slot = self.get_mut(instance).ok_or_else(|| super::reactor_close_fault("turn lifecycle missing"))?;
        slot.cell.prepare_retired(peers)?;
        slot.cell.release_owner_step(1, 4096, peers)?;"""),
])
TU = '⚛️reactor/🔄️turn/🦀️.rs'
edit(TU, [
(""".request_close(*request, runtime)?;""",""".request_close(*request, runtime, &ReactorClosePeers)?;"""),
("""owner.advance_admission(runtime)).transpose()?.flatten()""","""owner.advance_admission(runtime, &ReactorClosePeers)).transpose()?.flatten()"""),
("""borrow_mut().prepare_turn(instance)).transpose()?.flatten();""","""borrow_mut().prepare_turn(instance, &ReactorClosePeers)).transpose()?.flatten();"""),
])
print('lifetime + turn edited')
