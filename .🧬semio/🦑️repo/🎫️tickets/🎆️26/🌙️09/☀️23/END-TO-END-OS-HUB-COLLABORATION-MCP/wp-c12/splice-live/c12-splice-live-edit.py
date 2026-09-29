"""✂️ C12 (live wave p33, collab STEP 11/13 root cause, `probe-c12-splice.mjs` + `[DEBUG] c12 splice` trace `c12splice-1`):
(1) the React TextEditor delivered the text its outbox had QUEUED at the keystroke, not what the editor showed when the delivery
went out: a scene that folded a collaborator's run into the editor in between made the splice (host base = with the run, queued
text = without it) delete that run and re-insert everything around it — both editors then duplicated and lost runs. The outbox now
reads the editor's state when its turn comes (`createTextEditorOutboxV1`), for every typing mode.
(2) a scene whose applied splice is BELOW the host's acknowledged one comes from a reincarnated window (the document reopened,
silently on a concurrent reorder too): the host's unapplied splices never reached that window — they are dropped instead of being
folded onto its text as if still in flight (which duplicated them once the reopened document carried them); a late refusal of a
dropped run is a no-op. Laws: TS twin fixture row + seeded reincarnation simulation; outbox fixture + laws.
Idempotent; usage: python3 <this> [--apply]"""
import json
import os
import sys

ROOT = "/Users/ueli/Documents/semio/"
TWIN = ROOT + "🧰️framework/🔨️modules/🖱️ui/🎬️scene/✂️text-splice/🟦️.ts"
TWIN_LAWS = ROOT + "🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧪️tests/✂️text-splice/🟦️.test.ts"
TWIN_FIXTURE = ROOT + "🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧫️fixtures/✂️text-splice/🔣️.json"
TWIN_SCHEMA = ROOT + "🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧬️schema/✂️text-splice/🔣️.json"
EDITOR = ROOT + "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/✏️TextEditor/🟦️.tsx"
OUTBOX_LAWS = ROOT + "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/✏️TextEditor/🧪️tests/📮️outbox/🟦️.test.ts"
OUTBOX_FIXTURE = ROOT + "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/✏️TextEditor/🧫️fixtures/📮️outbox/🔣️.json"
OUTBOX_SCHEMA = ROOT + "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/✏️TextEditor/🧬️schema/📮️outbox/🔣️.json"

plan, problems, files = [], [], {}


def read(path):
    if path not in files:
        files[path] = open(path, encoding="utf-8").read() if os.path.exists(path) else None
    return files[path]


def edit(path, old, new, label):
    text = read(path)
    if text is None:
        problems.append((label, "missing"))
    elif (new in text) if old in new else (old not in text):
        plan.append(f"present {label}")
    elif text.count(old) != 1:
        problems.append((label, text.count(old)))
    else:
        files[path] = text.replace(old, new)
        plan.append(f"edit    {label}")


def create(path, content, label):
    text = read(path)
    if text == content:
        plan.append(f"present {label}")
    elif text is not None:
        problems.append((label, "exists with other content"))
    else:
        files[path] = content
        plan.append(f"create  {label}")


exec(compile(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "twin.py"), encoding="utf-8").read(), "twin.py", "exec"))
exec(compile(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "editor.py"), encoding="utf-8").read(), "editor.py", "exec"))

for line in plan:
    print(line)
print(len(problems), "problems", problems, "mode=" + ("apply" if "--apply" in sys.argv else "dry-run"))
if "--apply" in sys.argv and not problems:
    for path, text in files.items():
        if text is not None and (not os.path.exists(path) or open(path, encoding="utf-8").read() != text):
            os.makedirs(os.path.dirname(path), exist_ok=True)
            open(path, "w", encoding="utf-8").write(text)
    print("written")
