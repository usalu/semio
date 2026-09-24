import pathlib, sys

path = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs")
text = path.read_text()
pairs = [
    ("""            let (request, slot) = match ready {
                Some(ready) => ready,
                None => queue.next().await,
            };
            let outcome = match request {
""", """            let debug_idle = std::time::Instant::now();
            let (request, slot) = match ready {
                Some(ready) => ready,
                None => queue.next().await,
            };
            if debug_idle.elapsed() > std::time::Duration::from_secs(2) {
                eprintln!("[DEBUG] wg8 kernel idle {:?}", debug_idle.elapsed());
            }
            let debug_request = std::time::Instant::now();
            let debug_label = match &request { KernelRequest::AdvanceProductReplay { .. } => "advance-product-replay", KernelRequest::AdvanceRetained { .. } => "advance-retained", KernelRequest::MountProductReplay { .. } => "mount-product-replay", KernelRequest::CreateApp { .. } => "create-app", _ => "other" };
            let outcome = match request {
"""),
    ("""            };
            slot.deliver(outcome);
        }
    }
""", """            };
            if debug_request.elapsed() > std::time::Duration::from_secs(1) {
                eprintln!("[DEBUG] wg8 request {debug_label} elapsed={:?}", debug_request.elapsed());
            }
            slot.deliver(outcome);
        }
    }
"""),
]
if sys.argv[1:] == ["revert"]:
    pairs = [(new, old) for old, new in pairs]
for old, new in pairs:
    assert text.count(old) == 1, old[:80]
    text = text.replace(old, new)
path.write_text(text)
print("ok")
