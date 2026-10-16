from common import *
RC = '⚛️reactor/🦀️.rs'
p = PLUG + '/' + RC
t = open(p, encoding='utf-8').read()
a = t.index('fn preflight_cold_pair_close(')
b_start = t.index('fn release_cold_pair_close(')
b = t.index('}\n', t.index('\n}', b_start)) + 2 if False else None
# end of release_cold_pair_close: first "\n}\n" after b_start
b = t.index('\n}\n', b_start) + 3
old = t[a:b]
assert old.count('fn ') == 5 and 'COLD_PAIR_INGRESS' in old, old[:200]
new = '''/// 🔗️ The close participants the reactor core owns above the lifecycle, the patch tracker and the cold document pair ingress: the
/// lifecycle owner drives them through its [`instance_lifetime::ClosePeers`] port, which the turn supplies on every call.
pub(crate) struct ReactorClosePeers;

impl instance_lifetime::ClosePeers for ReactorClosePeers {
    fn preflight_patches(&self, key: instance_lifetime::NativeCloseKey) -> Result<(), semio_framework::Fault> {
        PATCHES.with(|patches| patches.preflight_close_instance(key)).map_err(reactor_close_fault)
    }

    fn reserve_patches(&self, key: instance_lifetime::NativeCloseKey) -> Result<(), semio_framework::Fault> {
        PATCHES.with(|patches| patches.reserve_close_instance(key)).map_err(reactor_close_fault)
    }

    fn activate_patches(&self, key: instance_lifetime::NativeCloseKey) -> Result<(), semio_framework::Fault> {
        PATCHES.with(|patches| patches.activate_close_instance(key)).map_err(reactor_close_fault)
    }

    fn patches_complete(&self, key: instance_lifetime::NativeCloseKey) -> Result<bool, semio_framework::Fault> {
        PATCHES.with(|patches| patches.close_instance_complete(key)).map_err(reactor_close_fault)
    }

    fn release_patches(&self, key: instance_lifetime::NativeCloseKey) -> Result<(), semio_framework::Fault> {
        PATCHES.with(|patches| patches.release_close_instance(key)).map_err(reactor_close_fault)
    }

    fn preflight_cold_pair(&self, key: instance_lifetime::NativeCloseKey) -> Result<(), semio_framework::Fault> {
        COLD_PAIR_INGRESS.with(|ingress| ingress.try_borrow().map_err(|_| reactor_close_fault("cold pair close preflight busy"))?.preflight_close_instance(key).map_err(reactor_close_fault))
    }

    fn reserve_cold_pair(&self, key: instance_lifetime::NativeCloseKey) -> Result<(), semio_framework::Fault> {
        COLD_PAIR_INGRESS.with(|ingress| ingress.try_borrow_mut().map_err(|_| reactor_close_fault("cold pair close reservation busy"))?.reserve_close_instance(key).map_err(reactor_close_fault))
    }

    fn activate_cold_pair(&self, key: instance_lifetime::NativeCloseKey) -> Result<(), semio_framework::Fault> {
        COLD_PAIR_INGRESS.with(|ingress| ingress.try_borrow_mut().map_err(|_| reactor_close_fault("cold pair close activation busy"))?.activate_close_instance(key).map_err(reactor_close_fault))
    }

    fn cold_pair_complete(&self, key: instance_lifetime::NativeCloseKey) -> Result<bool, semio_framework::Fault> {
        COLD_PAIR_INGRESS.with(|ingress| ingress.try_borrow().map_err(|_| "cold pair close receipt busy")?.close_instance_complete(key)).map_err(reactor_close_fault)
    }

    fn release_cold_pair(&self, key: instance_lifetime::NativeCloseKey) -> Result<(), semio_framework::Fault> {
        COLD_PAIR_INGRESS.with(|ingress| ingress.try_borrow_mut().map_err(|_| "cold pair close release busy")?.release_close_instance(key)).map_err(reactor_close_fault)
    }
}
'''
t = t[:a] + new + t[b:]
open(p, 'w', encoding='utf-8').write(t)
print('root edited; replaced', old.count('\n'), 'lines with', new.count('\n'))
