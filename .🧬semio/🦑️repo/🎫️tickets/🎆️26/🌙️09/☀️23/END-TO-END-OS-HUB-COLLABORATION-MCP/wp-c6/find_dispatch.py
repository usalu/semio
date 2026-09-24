from pathlib import Path
osroot = next(Path("/Users/ueli/Documents/semio").glob("*/🛍️products/💻️os"))
wgpu = next((osroot / "🔨️modules" / "📺️renderer").rglob("*/🐚️Shell/🎯️targets/*/🦀️.rs"))
print("wgpu", wgpu)
text = wgpu.read_text().splitlines()
for key in ["foldDirectoryEvents", "eventsJson", "ActionInvocation", "handle_action"]:
    idxs=[i for i,l in enumerate(text) if key in l]
    print(key, idxs[:12], "n=", len(idxs))
for i,l in enumerate(text):
    if "DirectoryStreamMessage::Event" in l:
        for j in range(max(0,i-5), min(i+30,len(text))):
            print(f"{j+1}: {text[j][:170]}")
        break
