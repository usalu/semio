#!/usr/bin/env python3
"""🧰️ Wave 0: moves the editor plumbing every command, mode and panel shares (app-addressed actions, fixed UI values,
labels, the reset-document effect, data-URL and still-image payload decoding) out of the editor root into the
registered editor member `✏️editor/🧰️kit/🦀️.rs`, mounted at `editor::remodeling::kit` (lane P2)."""
import os, re
REPO = os.path.abspath(os.path.join(os.path.dirname(os.path.abspath(__file__)), *[".."] * 8))
ART = os.path.join(REPO, "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling")
ED = os.path.join(ART, "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor")
path = os.path.join(ED, "🦀️.rs")
text = open(path, encoding="utf-8").read()
def cut(start, end):
    global text
    a = text.index(start); b = text.index(end, a) + len(end)
    block = text[a:b]; text = text[:a] + text[b:]
    return block
constants = cut("pub const REMODELING_PLAY_APP_ID: &str = \"remodeling-play\";\n", "/// 🌳️ Admits fallibly assembled UI nodes into fixed child storage.\n")
constants = constants.replace("/// 🌳️ Admits fallibly assembled UI nodes into fixed child storage.\n", "")
workflow = re.search(r"/// 🔌️ Well-known stream id every `photos:in` import lands on.*?const REMODELING_WORKFLOW_PHOTOS_STREAM_ID: &str = \"workflow-photos\";\n\n", constants, re.S).group(0)
constants = constants.replace(workflow, "")
text = text.replace("//#region 🔖️Constants\n", "//#region 🔖️Constants\n" + workflow.rstrip("\n") + "\n", 1)
reset = cut("//#region 🔖️ResetDocument\n", "//#endregion 🔖️ResetDocument\n")
payloads = cut("//#region 🔖️Payloads\n", "//#endregion 🔖️Payloads\n")
kit = ("//! 🧰️ Remodeling editor kit — the plumbing every command, mode, window and panel of the remodeling editor shares:\n"
       "//! app-addressed actions, fixed UI values and labels, the reset-document effect and payload decoding.\n\n"
       "use crate::editor::remodeling::engine::images as remodeling_image;\n"
       "use crate::standards::v1::subsets::any::schema::snapshot::{RemodelingSnapshot, REMODELING_DOCUMENT_SCHEMA};\n"
       "use pack::snapshot::ArtifactPack;\n\n"
       "//#region 🔖️Actions\n/// 🪪️ The remodeling editor's app id — the controller every action of this editor addresses.\n" + constants.rstrip("\n") + "\n//#endregion 🔖️Actions\n\n" + reset + "\n" + payloads)
os.makedirs(os.path.join(ED, "🧰️kit"), exist_ok=True)
open(os.path.join(ED, "🧰️kit", "🦀️.rs"), "w", encoding="utf-8").write(kit)
open(path, "w", encoding="utf-8").write(text)
root = os.path.join(ART, "🦀️.rs")
r = open(root, encoding="utf-8").read()
anchor = '        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🦀️.rs"]\n        pub mod transient;\n'
assert anchor in r
r = r.replace(anchor, anchor + '        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧰️kit/🦀️.rs"]\n        pub mod kit;\n', 1)
open(root, "w", encoding="utf-8").write(r)
print("kit lines", kit.count("\n"))
