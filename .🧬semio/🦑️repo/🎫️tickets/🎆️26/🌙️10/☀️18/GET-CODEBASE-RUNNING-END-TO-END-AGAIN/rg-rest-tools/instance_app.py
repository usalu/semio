import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/rg-rest-tools")
from delegate import *
APPLY = "--apply" in sys.argv
I = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance"
SRC = I + "/🔌️plugin-app-impl/🦀️.rs"
imap = import_map([rd(SRC), rd(I + "/🦀️.rs")])
MARK = "PluginApp for VcsArtifactApp<A, M> {"
jobs = I + "/👷️jobs/📥️staging/🧱️components/🦀️.rs"
docs = I + "/📄️docs/📥️staging/🧱️components/🦀️.rs"
kids = I + "/🪆️children/📥️staging/🧱️components/🦀️.rs"
plan = [
 ("take_media_export_chunk", "media", docs, r'impl<A: ArtifactApp> MediaTransfers<A> \{'),
 ("take_segmented_download_chunk", "media", docs, r'impl<A: ArtifactApp> MediaTransfers<A> \{'),
 ("acknowledge_document_archive_load", "ingest", docs, r'impl<A: ArtifactApp, M: SpaceMember \+ MemberFactory[^{]*\{'),
 ("cancel_document_archive_load", "ingest", docs, r'impl<A: ArtifactApp, M: SpaceMember \+ MemberFactory[^{]*\{'),
 ("child_packs", "child_composition", kids, r'impl<M: SpaceMember \+ MemberFactory \+ \'static> ChildComposition<M> \{'),
 ("child_head_packs", "child_composition", kids, r'impl<M: SpaceMember \+ MemberFactory \+ \'static> ChildComposition<M> \{'),
 ("original_render_tree_source", "pending", jobs, r'impl<A: ArtifactApp> PendingReserved<A> \{'),
 ("take_typed_operation_preparation_receipt", "tools", jobs, r'impl<A: ArtifactApp> ToolOperations<A> \{'),
]
allow = {"document_load_fault"}
for name, field, tgt, rx in plan:
    if not APPLY:
        d, c, a, b = move_method(SRC, MARK, name, field, tgt, rx, imap, allow, dry=True)
        print("====", name, a, "->", b)
        if name in ("take_media_export_chunk", "child_packs", "acknowledge_document_archive_load", "original_render_tree_source"):
            print(d); print(c)
    else:
        print(name, move_method(SRC, MARK, name, field, tgt, rx, imap, allow))
