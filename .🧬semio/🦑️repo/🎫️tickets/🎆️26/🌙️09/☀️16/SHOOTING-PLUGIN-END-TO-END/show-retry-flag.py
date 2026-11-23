"""🔎 Prints allows_transport_retry and the mailbox reset step."""
from pathlib import Path
root = Path("/home/deck/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu")
for path in root.rglob("*.rs"):
    text = path.read_text(errors="replace")
    if "fn allows_transport_retry" in text or "fn reset_asset_response_step" in text or "resetAssetResponseStep" in text:
        print("FILE", path)
        lines = text.splitlines()
        for i, line in enumerate(lines, 1):
            if "allows_transport_retry" in line or "reset_asset" in line or "resetAsset" in line or "world" in line and "retry" in line:
                start = max(1, i - 2)
                end = min(len(lines), i + 12)
                print(f"--- {path.name}:{i} ---")
                for n in range(start, end + 1):
                    print(f"{n}:{lines[n-1]}")
