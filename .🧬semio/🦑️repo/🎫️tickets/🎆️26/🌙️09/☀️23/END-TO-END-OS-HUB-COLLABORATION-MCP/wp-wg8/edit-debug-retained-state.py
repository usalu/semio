import pathlib, sys

path = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs")
text = path.read_text()
pairs = [
("""            let active =
                retained.patch.is_some() || !retained.queued_patches.is_empty() || !retained.rejected_patches.is_empty() || retained.build.is_some() || retained.build_pending || retained.closing.is_some() || retained.exchange_closing.is_some();
            if !active {""",
"""            let active =
                retained.patch.is_some() || !retained.queued_patches.is_empty() || !retained.rejected_patches.is_empty() || retained.build.is_some() || retained.build_pending || retained.closing.is_some() || retained.exchange_closing.is_some();
            {
                static WG8_CALLS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
                let calls = WG8_CALLS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if requested.is_some() && calls % 2000 == 0 {
                    eprintln!("[DEBUG] wg8 retained state call={calls} surface={} patch={} queued={} rejected={} build={} build_pending={} closing={} exchange_closing={} published={}", surface.as_ref(), retained.patch.is_some(), retained.queued_patches.len(), retained.rejected_patches.len(), retained.build.is_some(), retained.build_pending, retained.closing.is_some(), retained.exchange_closing.is_some(), retained.published.is_some());
                }
            }
            if !active {"""),
]
if sys.argv[1:] == ["revert"]:
    pairs = [(new, old) for old, new in pairs]
for old, new in pairs:
    assert text.count(old) == 1, old[:80]
    text = text.replace(old, new)
path.write_text(text)
print("ok")
