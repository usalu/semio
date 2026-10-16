# 🔧 Follow-up of sq1.py: the host owners expose their core, and the in-tree tests reach the registry through it.
import re, sys
R = '/Users/ueli/Documents/semio'
SYNC = R + '/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync'
def edit(path, fn):
    t = open(path, encoding='utf8').read(); n = fn(t)
    assert n != t, path
    open(path, 'w', encoding='utf8').write(n)
def once(t, a, b):
    assert t.count(a) == 1, (t.count(a), a[:100]); return t.replace(a, b)

CORE_NATIVE = '''    /// 🏛️ The platform-neutral registry this host composes with the native actor.
    pub fn core(&self) -> &ArtifactHostCore<ArtifactActorRunnerHandle> {
        &self.core
    }

    pub fn set_local_hub_credential(&self, credential: Arc<crate::os_directory::client::LocalHubCredential>) {'''
edit(SYNC + '/🖥️native/🎭️actor/🦀️.rs', lambda t: once(t, '    pub fn set_local_hub_credential(&self, credential: Arc<crate::os_directory::client::LocalHubCredential>) {', CORE_NATIVE))
CORE_WASM = CORE_NATIVE.replace('ArtifactHostCore<ArtifactActorRunnerHandle>', 'ArtifactHostCore<()>').replace('native actor', 'browser actor')
edit(SYNC + '/🌐️wasm/🎭️actor/🦀️.rs', lambda t: once(t, '    pub fn set_local_hub_credential(&self, credential: Arc<crate::os_directory::client::LocalHubCredential>) {', CORE_WASM))

def fixtures(t):
    t = once(t, 'let state = host.inner.lock().unwrap_or_else(std::sync::PoisonError::into_inner);', 'let state = host.core.state();')
    t = once(t, 'assert!(!host.inner.lock().unwrap_or_else(std::sync::PoisonError::into_inner).documents.contains_key(&space_a));', 'assert!(!host.core.state().documents.contains_key(&space_a));')
    t = once(t, 'assert!(host.inner.lock().unwrap_or_else(std::sync::PoisonError::into_inner).documents.contains_key(&space_b));', 'assert!(host.core.state().documents.contains_key(&space_b));')
    return t
edit(SYNC + '/🖥️native/🧪️tests/🔬️retained-turn-fixtures/🦀️.rs', fixtures)

def unit(t):
    assert t.count('host.pool.clone()') == 2
    t = t.replace('host.pool.clone()', 'host.core().pool.clone()')
    t = once(t, 'host.inner.lock().unwrap().documents.insert(ArtifactDocumentKey::local("doc"), OpenDocument {', 'host.core().insert_document(ArtifactDocumentKey::local("doc"), OpenDocument {')
    t = once(t, 'host.inner.lock().unwrap().documents.insert(key.clone(), OpenDocument {', 'host.core().insert_document(key.clone(), OpenDocument {')
    return t
edit(SYNC + '/🧪️tests/🔬️unit/🦀️.rs', unit)
print('ok')
