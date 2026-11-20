"""📦️ Resolve the release output recorded by this Vite invocation."""
import json
import sys
from pathlib import Path


def captured_site(root, lines):
    releases = (root / ".cache/b/p/m/vite/release").resolve()
    outputs = set()
    for line in lines:
        if not line.startswith("[script] {"):
            continue
        try:
            event = json.loads(line[len("[script] "):])
        except ValueError:
            continue
        resource = event.get("activity", {}).get("resource", "")
        if event.get("command") == "vite:entry" and resource.startswith("artifact-capture:"):
            path = Path(resource.removeprefix("artifact-capture:")).resolve()
            if path.name == "out" and path.parent.parent == releases:
                outputs.add(path)
    if len(outputs) != 1:
        raise ValueError(f"Expected one captured Vite release directory, found {len(outputs)}")
    output = outputs.pop()
    if not (output / "index.html").is_file():
        raise FileNotFoundError(output / "index.html")
    return output


if __name__ == "__main__":
    with Path(sys.argv[2]).open(encoding="utf-8", errors="replace") as log:
        print(captured_site(Path(sys.argv[1]), log))
