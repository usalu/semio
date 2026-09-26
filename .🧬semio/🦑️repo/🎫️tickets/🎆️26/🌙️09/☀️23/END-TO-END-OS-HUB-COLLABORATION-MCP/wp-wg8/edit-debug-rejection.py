import pathlib, sys

path = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs")
text = path.read_text()
pairs = [
("""                    if let Some(rejection) = retained.advance_patch_one() {
                        let reason = rejection.1.to_string();""",
"""                    if let Some(rejection) = retained.advance_patch_one() {
                        let reason = rejection.1.to_string();
                        eprintln!("[DEBUG] wg8 retained rejection surface={} revision={:?} reason={reason} published={} queued={} rejected={}", surface.as_ref(), rejection.0, retained.published.is_some(), retained.queued_patches.len(), retained.rejected_patches.len());"""),
]
if sys.argv[1:] == ["revert"]:
    pairs = [(new, old) for old, new in pairs]
for old, new in pairs:
    assert text.count(old) == 1, old[:80]
    text = text.replace(old, new)
path.write_text(text)
print("ok")
