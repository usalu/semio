# 🪓️ WP7 pre-edit of the sync root: invert the host/actor dependency. The platform-neutral host core stays in the root, `ArtifactHost` moves into each actor module, and both actor modules become files under their platform node.
# usage: sq1.py dry|write
import os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE); sys.path.insert(0, HERE + '/../okwp0-tools')
from okrs import items
import sq1_blocks as B
R = '/Users/ueli/Documents/semio'
SYNC = R + '/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync'
ROOT = SYNC + '/🦀️.rs'
NATIVE_FILE = SYNC + '/🖥️native/🎭️actor/🦀️.rs'
WASM_FILE = SYNC + '/🌐️wasm/🎭️actor/🦀️.rs'
FIX_OLD = SYNC + '/🧪️tests/🔬️native-actor-retained-turn-fixtures/🦀️.rs'
FIX_NEW = SYNC + '/🖥️native/🧪️tests/🔬️retained-turn-fixtures/🦀️.rs'
WRITE = sys.argv[1] == 'write'

def once(t, a, b=None):
    assert t.count(a) == 1, (t.count(a), a[:100])
    return t if b is None else t.replace(a, b)

def dedent(body):
    out = []
    for l in body.split('\n'):
        out.append(l[4:] if l.startswith('    ') else l)
    return '\n'.join(out)

def split_mod(src, name):
    its = [it for it in items(src) if it['kind'] == 'mod' and it['name'] == name]
    assert len(its) == 1, name
    it = its[0]
    text = it['text']
    head = text.index('mod ' + name + ' {')
    brace = text.index('{', head)
    close = text.rindex('}')
    assert text[close + 1:].strip() == '', name
    body = text[brace + 1:close].strip('\n')
    return it, text, head, body

src = open(ROOT, encoding='utf8').read()
orig_lines = src.count('\n')
a = '/// 🎛️ The channels {@link ArtifactHost::open} hands back to a caller'
e = '//#endregion 🔖️Host'
once(src, a); once(src, e)
i = src.index(a); j = src.index(e)
assert i < j
src = src[:i] + B.CORE + '\n' + src[j:]

src = once(src, "    pub async fn attach(&mut self, channels: ArtifactChannels, events:", "    pub async fn attach<Runner>(&mut self, channels: ArtifactChannelsOf<Runner>, events:")

nat, nat_text, nat_head, nat_body = split_mod(src, 'native_actor')
src = src.replace(nat_text, nat_text[:nat_head] + '#[path = "🖥️native/🎭️actor/🦀️.rs"]\nmod native_actor;', 1)
was, was_text, was_head, was_body = split_mod(src, 'wasm_actor')
src = src.replace(was_text, was_text[:was_head] + '#[path = "🌐️wasm/🎭️actor/🦀️.rs"]\nmod wasm_actor;', 1)

src = once(src, '#[cfg(not(target_arch = "wasm32"))]\nuse native_actor::spawn_actor;\n#[cfg(not(target_arch = "wasm32"))]\npub use native_actor::{ArtifactActorRunnerHandle, ArtifactActorRunnerTicket, ArtifactActorTerminalJob, ArtifactActorTerminalReason, ArtifactActorTerminalState};',
           '#[cfg(not(target_arch = "wasm32"))]\npub use native_actor::{ArtifactActorRunnerHandle, ArtifactActorRunnerTicket, ArtifactActorTerminalJob, ArtifactActorTerminalReason, ArtifactActorTerminalState, ArtifactChannels, ArtifactHost};')
src = once(src, '#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]\nuse wasm_actor::spawn_actor;',
           '#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]\npub use wasm_actor::{ArtifactChannels, ArtifactHost};')

nb = dedent(nat_body)
nb = once(nb, 'host: Option<Arc<std::sync::Mutex<ArtifactHostState>>>,', 'host: Option<Arc<std::sync::Mutex<ArtifactHostState<ArtifactActorRunnerHandle>>>>,')
nb = once(nb, 'host: Arc<std::sync::Mutex<ArtifactHostState>>) -> ArtifactActorRunnerTicket', 'host: Arc<std::sync::Mutex<ArtifactHostState<ArtifactActorRunnerHandle>>>) -> ArtifactActorRunnerTicket')
tail = '#[cfg(test)]\ninclude!("🧪️tests/🔬️native-actor-retained-turn-fixtures/🦀️.rs");'
nb = once(nb, tail)
nb = nb.replace(tail, B.NATIVE_HOST.strip('\n') + '\n\n#[cfg(test)]\ninclude!("../🧪️tests/🔬️retained-turn-fixtures/🦀️.rs");')
native_text = '//! 🎭️ The native document actor: one finite-turn actor per open document on the process WorkerPool, its runner, and the native `ArtifactHost` that owns them.\n\nuse super::*;\n' + nb.replace('use super::*;\n', '', 1).lstrip('\n') + '\n'
wb = dedent(was_body)
wb = wb.replace('use super::*;\n', '', 1).lstrip('\n')
marker = '#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]'
wasm_text = '//! 🎬️ The browser document actor: one actor per open document on the page executor, its hub binding, and the browser `ArtifactHost` that owns them.\n\nuse super::*;\n' + wb.rstrip('\n') + '\n' + B.WASM_HOST + '\n'

print('root lines', orig_lines, '->', src.count('\n'), '| native file lines', native_text.count('\n'), '| wasm file lines', wasm_text.count('\n'))
if WRITE:
    for p, t in ((ROOT, src), (NATIVE_FILE, native_text), (WASM_FILE, wasm_text)):
        os.makedirs(os.path.dirname(p), exist_ok=True)
        open(p, 'w', encoding='utf8').write(t)
    os.makedirs(os.path.dirname(FIX_NEW), exist_ok=True)
    assert not os.path.exists(FIX_NEW)
    os.rename(FIX_OLD, FIX_NEW)
    print('written')
