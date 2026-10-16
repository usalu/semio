# 🔗️ F7 revision: the port is a generic parameter of the owner methods instead of an associated type (an associated `dyn` type defaults to 'static and would pin every caller).
from common import *
import re
def edit(path, pairs):
    p = PLUG + '/' + path
    t = open(p, encoding='utf-8').read()
    for old, new in pairs:
        assert t.count(old) == 1, (path, t.count(old), old[:100])
        t = t.replace(old, new)
    open(p, 'w', encoding='utf-8').write(t)
LT = '⚛️reactor/🚪️lifetime/🦀️.rs'
edit(LT, [
("""pub(crate) trait GuestLifetimeOwner: terminal_owner::Sealed {
    type Peers: ?Sized;
    fn terminal_is_empty(&self, peers: &Self::Peers) -> Result<bool, semio_framework::Fault>;
    fn release_terminal(owner: &mut Option<Self>, maximum_items: usize, maximum_bytes: usize, peers: &Self::Peers) -> Result<GuestTerminalRelease, semio_framework::Fault>
    where
        Self: Sized;
}""","""pub(crate) trait GuestLifetimeOwner: terminal_owner::Sealed {
    fn terminal_is_empty<P: ClosePeers + ?Sized>(&self, peers: &P) -> Result<bool, semio_framework::Fault>;
    fn release_terminal<P: ClosePeers + ?Sized>(owner: &mut Option<Self>, maximum_items: usize, maximum_bytes: usize, peers: &P) -> Result<GuestTerminalRelease, semio_framework::Fault>
    where
        Self: Sized;
}"""),
("""    fn prepare_retired(&mut self, peers: &O::Peers) -> Result<bool, semio_framework::Fault> {""","""    fn prepare_retired<P: ClosePeers + ?Sized>(&mut self, peers: &P) -> Result<bool, semio_framework::Fault> {"""),
("""    fn release_owner_step(&mut self, maximum_items: usize, maximum_bytes: usize, peers: &O::Peers) -> Result<(), semio_framework::Fault> {""","""    fn release_owner_step<P: ClosePeers + ?Sized>(&mut self, maximum_items: usize, maximum_bytes: usize, peers: &P) -> Result<(), semio_framework::Fault> {"""),
("""    pub(crate) fn request_close(&mut self, request: ActorInstanceCloseRequest, runtime: &crate::plugin_runtime::PluginRuntime<PA>, peers: &dyn ClosePeers) -> Result<(), semio_framework::Fault> {""","""    pub(crate) fn request_close<P: ClosePeers + ?Sized>(&mut self, request: ActorInstanceCloseRequest, runtime: &crate::plugin_runtime::PluginRuntime<PA>, peers: &P) -> Result<(), semio_framework::Fault> {"""),
("""    pub(crate) fn advance_admission(&mut self, runtime: &crate::plugin_runtime::PluginRuntime<PA>, peers: &dyn ClosePeers) -> Result<Option<(ActorInstanceCloseRequest, u64)>, semio_framework::Fault> {""","""    pub(crate) fn advance_admission<P: ClosePeers + ?Sized>(&mut self, runtime: &crate::plugin_runtime::PluginRuntime<PA>, peers: &P) -> Result<Option<(ActorInstanceCloseRequest, u64)>, semio_framework::Fault> {"""),
("""impl<PA: crate::plugin_app::PluginApp> GuestLifetimeOwner for NativeLifetimeOwner<PA> {
    type Peers = dyn ClosePeers;
    fn terminal_is_empty(&self, peers: &dyn ClosePeers) -> Result<bool, semio_framework::Fault> {""","""impl<PA: crate::plugin_app::PluginApp> GuestLifetimeOwner for NativeLifetimeOwner<PA> {
    fn terminal_is_empty<P: ClosePeers + ?Sized>(&self, peers: &P) -> Result<bool, semio_framework::Fault> {"""),
("""    fn release_terminal(owner: &mut Option<Self>, maximum_items: usize, maximum_bytes: usize, peers: &dyn ClosePeers) -> Result<GuestTerminalRelease, semio_framework::Fault> {""","""    fn release_terminal<P: ClosePeers + ?Sized>(owner: &mut Option<Self>, maximum_items: usize, maximum_bytes: usize, peers: &P) -> Result<GuestTerminalRelease, semio_framework::Fault> {"""),
("""    pub(crate) fn prepare_turn(&mut self, instance: u32, peers: &dyn ClosePeers) -> Result<Option<ActorInstanceLifecycleReceipt>, semio_framework::Fault> {""","""    pub(crate) fn prepare_turn<P: ClosePeers + ?Sized>(&mut self, instance: u32, peers: &P) -> Result<Option<ActorInstanceLifecycleReceipt>, semio_framework::Fault> {"""),
])
T1 = '⚛️reactor/🚪️lifetime/🧪️tests/🚪️lifetime/🦀️.rs'
p = PLUG + '/' + T1
t = open(p, encoding='utf-8').read()
n = t.count('type Peers = ();')
t = re.sub(r'^\s*type Peers = \(\);\n', '', t, flags=re.M)
t = t.replace('fn terminal_is_empty(&self, _peers: &()) ->', 'fn terminal_is_empty<P: ClosePeers + ?Sized>(&self, _peers: &P) ->')
t = t.replace('_maximum_bytes: usize, _peers: &()) -> Result<GuestTerminalRelease', '_maximum_bytes: usize, _peers: &P) -> Result<GuestTerminalRelease')
t = t.replace('fn release_terminal(owner: &mut Option<Self>', 'fn release_terminal<P: ClosePeers + ?Sized>(owner: &mut Option<Self>')
t = t.replace('.prepare_retired(&())', '.prepare_retired(&NoPeers)')
t = re.sub(r'\.release_owner_step\((\d+), 4096, &\(\)\)', r'.release_owner_step(\1, 4096, &NoPeers)', t)
# the null port of the unit tests, after the first `use super::*;`
double = '''
struct NoPeers;
impl ClosePeers for NoPeers {
    fn preflight_patches(&self, _key: NativeCloseKey) -> Result<(), semio_framework::Fault> { Ok(()) }
    fn reserve_patches(&self, _key: NativeCloseKey) -> Result<(), semio_framework::Fault> { Ok(()) }
    fn activate_patches(&self, _key: NativeCloseKey) -> Result<(), semio_framework::Fault> { Ok(()) }
    fn patches_complete(&self, _key: NativeCloseKey) -> Result<bool, semio_framework::Fault> { Ok(true) }
    fn release_patches(&self, _key: NativeCloseKey) -> Result<(), semio_framework::Fault> { Ok(()) }
    fn preflight_cold_pair(&self, _key: NativeCloseKey) -> Result<(), semio_framework::Fault> { Ok(()) }
    fn reserve_cold_pair(&self, _key: NativeCloseKey) -> Result<(), semio_framework::Fault> { Ok(()) }
    fn activate_cold_pair(&self, _key: NativeCloseKey) -> Result<(), semio_framework::Fault> { Ok(()) }
    fn cold_pair_complete(&self, _key: NativeCloseKey) -> Result<bool, semio_framework::Fault> { Ok(true) }
    fn release_cold_pair(&self, _key: NativeCloseKey) -> Result<(), semio_framework::Fault> { Ok(()) }
}
'''
assert t.count('use super::*;\n') >= 1
i = t.index('use super::*;\n') + len('use super::*;\n')
t = t[:i] + double + t[i:]
open(p, 'w', encoding='utf-8').write(t)
print('tests: owners', n, 'NoPeers added')
