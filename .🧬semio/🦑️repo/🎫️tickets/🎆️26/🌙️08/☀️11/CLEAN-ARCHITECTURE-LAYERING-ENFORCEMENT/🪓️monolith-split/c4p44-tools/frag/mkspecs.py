"""📐 C4P44 spec generator for the one include-fragment split (wsq): chunk start tables to `specs/1.json` for fragsplit.py (descendant of c4p39-tools/mkspecs.py)."""
import json, os

HERE = os.path.dirname(os.path.abspath(__file__))
F = os.path.dirname(os.path.dirname(HERE))
STAGE = "/Users/ueli/.cache/c4p44/stage-frag"
FILE = "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🧪️tests/🔬️quick/🦀️.rs"
ORIG = os.path.dirname(HERE) + "/orig/wsq.rs"
T = [
    (46, "📡️hub-relay-and-workspace-guards", "📡️ Hub relay and workspace guards: the committed fixture reader, the repository root refusal, agent presence, hub document tails, relay queues, terminal links, hub session edit refusal, hub catalog authority and the probe transport."),
    (218, "🔐️authenticated-hub-workspace", "🔐️ Authenticated hub workspace: the empty catalog, the authenticated hub workspace fixture, its snapshot-only resources, retained discovery and the one live link per app bound."),
    (353, "🤖️headless-agent-and-pending-exchanges", "🤖️ Headless agent and pending exchanges: the headless agent operations, abandoned exchange close, pending response grants and faults, the guest fault wire and the probe codec."),
    (476, "📂️folder-workspace-and-probe-documents", "📂️ Folder workspace and probe documents: folder creation, the fresh folder listing, the probe artifact seed, context, resource reads and unknown-handle refusals."),
    (557, "🧭️plugin-capability-routing", "🧭️ Plugin capability routing: capability resolution, routed channels, multi-app instance slots and the routing channel capability and exchange laws."),
    (739, "📖️artifact-resources-and-probe-edits", "📖️ Artifact resources and probe edits: artifact resource validation and schemas, probe mutation commit, undo and redo, the base64 vector and the plugin entry lookup."),
    (802, "🧵️document-backbone-egress", "🧵️ Document backbone egress: the backbone effect addressed to this channel and the committed message without a document actor."),
    (861, "🗂️guest-document-codec", "🗂️ Guest document codec: the guest-backed codec refusals, the unregistered route and the two operations the WIT does not carry."),
    (911, "🔗️inference-artifact-binding", "🔗️ Inference artifact binding: the bound inference rows and commands and the binding field, encoding and contract laws."),
    (999, "🏁️child-heads-hub-commit-and-probe-pack", "🏁️ Child heads, hub commit and probe pack: composed child heads as inference dependencies, the guest JSON decoder round trip, the isolated compile worker, the hub commit status and the cancelled owners of the original probe pack."),
]


def back_up(ls, s):
    k = s
    while k > 1:
        prev = ls[k - 2]
        if prev.strip() and not prev.startswith("//#endregion") and (prev.startswith("///") or prev.startswith("#[") or prev.startswith("//")):
            k -= 1
        else:
            break
    return k


os.makedirs(HERE + "/specs", exist_ok=True)
ls = open(ORIG, encoding="utf-8").read().split("\n")
if ls[-1] == "":
    ls.pop()
starts = [back_up(ls, item) for item, _, _ in T]
assert starts == sorted(starts) and len(set(starts)) == len(starts), starts
chunks = []
for i, (item, folder, doc) in enumerate(T):
    a = starts[i]
    b = (starts[i + 1] - 1) if i + 1 < len(T) else len(ls)
    while not ls[b - 1].strip():
        b -= 1
    chunks.append(dict(name=folder, folder=folder, doc=doc, ranges=[[a, b]]))
    print(folder, a, b, b - a + 1, "prev:", repr(ls[a - 2][:50]))
spec = dict(file=FILE, original=ORIG, stage=STAGE, chunks=chunks)
json.dump(spec, open(f"{HERE}/specs/1.json", "w", encoding="utf-8"), indent=1, ensure_ascii=False)
