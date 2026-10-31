import glob
import json
import zipfile

zpath = glob.glob("/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/TEACHING-ARCHITECTURE-DEPLOY-READY/🗑️generated/nx/e2e/20261030T204911-1220686/report/dev-shared/*/trace.zip")[0]
with zipfile.ZipFile(zpath) as archive:
    lines = archive.read("1-trace.trace").splitlines()

for index, raw in enumerate(lines):
    if b'"type": "input"' not in raw:
        continue
    event = json.loads(raw)
    if event.get("type") == "input" and (3600 <= index <= 3720 or 4940 <= index <= 5040):
        print("INPUT", index, event.get("callId"), event.get("point"), event.get("box"))

def mode_at(index: int) -> None:
    event = json.loads(lines[index])
    html = event["snapshot"]["html"]

    def walk(node):
        if isinstance(node, list) and node and isinstance(node[0], str):
            attrs = node[1] if len(node) > 1 and isinstance(node[1], dict) else {}
            if isinstance(attrs, dict) and attrs.get("data-layered-overview") == "":
                print(index, {k: attrs.get(k) for k in ("data-mode", "data-interactive", "data-layered-resting", "data-pan")})
                return True
            kids = node[2:] if isinstance(attrs, dict) else node[1:]
            return any(walk(kid) for kid in kids)
        if isinstance(node, list):
            return any(walk(kid) for kid in node)
        return False

    walk(html)

for index in range(3660, 3680):
    event = json.loads(lines[index])
    if event.get("type") == "frame-snapshot" and event.get("snapshot", {}).get("phase") == "action":
        mode_at(index)
mode_at(5012)

def logs(start: int, end: int) -> None:
    print(f"\n==== {start}-{end} ====")
    for index in range(start, end):
        event = json.loads(lines[index])
        if event.get("type") == "log":
            print(index, event.get("message", "")[:220])
        elif event.get("type") == "frame-snapshot":
            url = event["snapshot"].get("frameUrl", "")
            if "#learner" in url or index in (start, end - 1):
                print(index, "SNAP", url, event["snapshot"].get("phase"))
        elif event.get("method") in ("click", "reload"):
            print(index, event.get("method"), str(event.get("params", ""))[:160])

logs(3610, 3720)
logs(4948, 5035)
