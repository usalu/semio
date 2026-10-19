import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/rg-rest-tools")
from delegate import *
I = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance"
SRC = I + "/🔌️plugin-app-impl/🦀️.rs"
imap = import_map([rd(SRC), rd(I + "/🦀️.rs")])
MARK = "PluginApp for VcsArtifactApp<A, M> {"
stores = I + "/🏪️stores/📥️staging/🧱️components/🦀️.rs"
for n in ("admit_presence_ingress", "reject_presence_ingress"):
    print(n, move_method(SRC, MARK, n, "roster", stores, r'impl<A: ArtifactApp> PeerRoster<A> \{', imap, set(), selfcrate=("semio_framework_plugin_instance_stores::stores_components::", "crate::stores_components::")))
