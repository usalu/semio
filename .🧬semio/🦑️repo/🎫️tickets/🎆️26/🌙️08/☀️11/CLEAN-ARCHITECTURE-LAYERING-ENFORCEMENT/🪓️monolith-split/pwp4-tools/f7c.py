from common import *
import re
T1 = '⚛️reactor/🚪️lifetime/🧪️tests/🚪️lifetime/🦀️.rs'
T2 = '⚛️reactor/🚪️lifetime/🧪️tests/🧵️runtime/🦀️.rs'
p = PLUG + '/' + T1
t = open(p, encoding='utf-8').read()
n0 = t.count('fn terminal_is_empty(&self)')
t = re.sub(r'^(\s*)fn terminal_is_empty\(&self\) -> Result<bool, semio_framework::Fault> \{', lambda m: f'{m.group(1)}type Peers = ();\n{m.group(1)}fn terminal_is_empty(&self, _peers: &()) -> Result<bool, semio_framework::Fault> {{', t, flags=re.M)
t = t.replace('maximum_items: usize, _maximum_bytes: usize) -> Result<GuestTerminalRelease', 'maximum_items: usize, _maximum_bytes: usize, _peers: &()) -> Result<GuestTerminalRelease')
t = t.replace('.prepare_retired()', '.prepare_retired(&())')
t = re.sub(r'\.release_owner_step\((\d+), 4096\)', r'.release_owner_step(\1, 4096, &())', t)
open(p, 'w', encoding='utf-8').write(t)
print(T1, n0, 'owners updated')
p = PLUG + '/' + T2
t = open(p, encoding='utf-8').read()
old = 'GuestLifetimeOwner::terminal_is_empty(owner).unwrap()'
assert t.count(old) == 1
t = t.replace(old, 'GuestLifetimeOwner::terminal_is_empty(owner, &crate::reactor::ReactorClosePeers).unwrap()')
open(p, 'w', encoding='utf-8').write(t)
print(T2, 'updated')
