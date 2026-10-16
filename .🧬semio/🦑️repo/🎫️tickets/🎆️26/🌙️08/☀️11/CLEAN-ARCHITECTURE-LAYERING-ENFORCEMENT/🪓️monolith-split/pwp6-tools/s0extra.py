import sys; sys.path.insert(0, '.')
from common6 import *
import touched6
rel = '🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs'
p = PLUG + rel
t = open(p, encoding='utf-8').read()
old = """    /// 🧵️ Source text of the mounted artifact instance: the parent file followed by its impl-band children in file order.
    fn instance_source() -> String {
        [
            include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🧵️instance/🦀️.rs")),
            include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🧵️instance/🚪️ingress/🦀️.rs")),
            include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🧵️instance/📨️dispatch/🦀️.rs")),
            include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🧵️instance/🖼️render/🦀️.rs")),
            include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🧵️instance/🧷️command-routes/🦀️.rs")),
            include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🧵️instance/🧾️typed-operations/🦀️.rs")),
            include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🧵️instance/🔌️plugin-app-impl/🦀️.rs")),
        ]
        .concat()
    }
"""
new = """    /// 🧵️ Source text of the mounted artifact instance: its module tree, the parent file followed by every staged impl slice in declaration order, read at run time from the manifest folder of the hosting crate.
    fn instance_source() -> String {
        semio_framework_plugin_test_support::read_module_tree_source(&std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🧵️instance/🦀️.rs")).expect("the instance module tree reads")
    }
"""
assert t.count(old) == 1, 'already applied or changed'
open(p, 'w', encoding='utf-8').write(t.replace(old, new))
touched6.merge(modified=[rel])
print('s0extra applied')
