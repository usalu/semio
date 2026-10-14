#!/usr/bin/env python3
"""🏁️ Wave 0: the checkerboard import fixtures (`checker_image`, `checker_data_url`, `checker_data_url_jpeg`,
`checker_video_data_url`) lived twice as `#[cfg(test)]` code inside two production command files, together with the
editor-harness helper `verify_import_checker_stream`. The fixtures move once into the editor kit (they are shared by the
reconstruction session's and the editor's tests across crates), the harness helper into the editor test harness (lane P2)."""
import os, re
REPO = os.path.abspath(os.path.join(os.path.dirname(os.path.abspath(__file__)), *[".."] * 8))
ED = os.path.join(REPO, "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor")
video = os.path.join(ED, "🎮️commands/📼️import-video-frame-payload/🦀️.rs")
still = os.path.join(ED, "🎮️commands/🖼️import-frame-payload/🦀️.rs")
kit = os.path.join(ED, "🧰️kit/🦀️.rs")
harness = os.path.join(ED, "🧪️tests/🔬️unit/🦀️.rs")
def cut_region(path):
    text = open(path, encoding="utf-8").read()
    a = text.index("//#region 🧪️UnitTests\n"); b = text.index("//#endregion 🧪️UnitTests\n") + len("//#endregion 🧪️UnitTests\n")
    block = text[a:b]
    open(path, "w", encoding="utf-8").write(text[:a] + text[b:])
    return block
vblock = cut_region(video)
cut_region(still)
fixtures = vblock[vblock.index("/// 🏁️ High-contrast"):vblock.index("//#endregion 🧪️UnitTests\n")]
fixtures = fixtures.replace("#[cfg(test)]\n", "").replace("pub(crate) async fn", "pub async fn").replace("\nfn checker_image", "\npub fn checker_image")
fixtures = fixtures.replace("crate::editor::remodeling::engine::video::container_providers::mp4::write_mjpeg", "crate::editor::remodeling::engine::video::container_providers::mp4::write_mjpeg")
k = open(kit, encoding="utf-8").read().rstrip("\n")
k += "\n\n//#region 🏁️Fixtures\n" + fixtures.rstrip("\n") + "\n//#endregion 🏁️Fixtures\n"
open(kit, "w", encoding="utf-8").write(k)
helper = vblock[vblock.index("/// 📥️ Imports `n` checker frames"):vblock.index("/// 🏁️ High-contrast")]
helper = helper.replace("#[cfg(test)]\n", "").replace("pub(crate) async fn verify_import_checker_stream(app: &mut crate::editor::remodeling::unit_tests::context::RemodelingApp, n: u32)", "pub async fn verify_import_checker_stream(app: &mut RemodelingApp, n: u32)")
helper = helper.replace("    use crate::editor::remodeling::unit_tests::context::dispatch;\n", "").replace("    use crate::editor::remodeling::RemodelingCommand;\n", "")
helper = helper.replace("import_frame_payload::ImportFramePayload { payload: checker_data_url(", "crate::editor::remodeling::commands::import_frame_payload::ImportFramePayload { payload: crate::editor::remodeling::kit::checker_data_url(")
h = open(harness, encoding="utf-8").read()
anchor = "    /// 🧪️ The ONE app shape every test uses:"
assert anchor in h
h = h.replace(anchor, "\n".join("    " + line if line else "" for line in helper.rstrip("\n").split("\n")) + "\n\n" + anchor, 1)
open(harness, "w", encoding="utf-8").write(h)
print("ok")
