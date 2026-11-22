"""🏢 Rework 19, curtain wall bay: the fixings the summary names become bodies instead of structure-only marks. 1:50 and 1:100: steel slab cleats beside the mullion heads with the hanging bolt through tube and cleat, and the aluminium spigot sleeves in the mullion feet; 1:50 also the EPDM setting blocks under the glass unit and the spandrel. The structure list references cleats, bolts and spigots; the setting blocks stay authored in both lists because the coarser structure lists keep theirs."""
import copy
import sys

sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE")
import rework_19_lib as L

SLUG = "curtain-wall-bay"
CLEAT_T = {"50": 0.008, "100": 0.015}
BOLT_R = {"50": 0.008, "100": 0.0125}
NOTE = {"en": "The fixings are bodies: a steel cleat from the slab bracket lies against the inner side of each mullion head and the hanging bolt passes through tube and cleat; in each mullion foot sits the aluminium spigot sleeve that guides it on the mullion below; the slab itself is outside the bay.", "de": "Die Befestigungen sind Körper: an der Innenseite jedes Pfostenkopfs liegt eine Stahllasche der Deckenkonsole, der Hängebolzen geht durch Rohr und Lasche; in jedem Pfostenfuß sitzt die Aluminium-Stoßhülse, die ihn auf dem unteren Pfosten führt; die Decke selbst liegt außerhalb des Feldes."}
BLOCKS = {"en": "Under the glass unit and the spandrel lie two EPDM setting blocks each, 100 mm long at the quarter points, which carry the dead load into the transom.", "de": "Unter Glaseinheit und Brüstung liegen je zwei EPDM-Klötze, 100 mm lang in den Viertelspunkten, die das Eigengewicht in den Riegel leiten."}


def body_of(v, structure_id, body_id, role="detail", change=None, keep=False):
    k = next(n for n, x in enumerate(v["structure"]) if x["id"] == structure_id)
    s = v["structure"][k]
    if "of" in s:
        return
    b = {key: copy.deepcopy(val) for key, val in s.items() if key not in ("member", "part", "anchor")}
    b["id"], b["role"] = body_id, role
    if change:
        change(b)
    v["body"].append(b)
    if keep:
        return
    ref = {"id": structure_id, "of": body_id, "member": s["member"], "part": s["part"]}
    if "anchor" in s:
        ref["anchor"] = s["anchor"]
    v["structure"][k] = ref


doc = L.load_source(SLUG)
for scale in ("50", "100"):
    v = doc["volumes"][scale]
    if any(x["id"] == "slab-cleat" for x in v["body"]):
        continue
    t, r = CLEAT_T[scale], BOLT_R[scale]
    v["body"].append({"id": "slab-cleat", "kind": "box", "bone": "frame", "material": "steel", "role": "detail", "tone": 0.3, "size": [t, 0.11, 0.08], "transform": {"translation": [0.05 + t / 2, 3.245, 0.06]}, "repeat": {"count": 2, "translation": [1.3 - t, 0.0, 0.0]}})
    v["structure"].append({"id": "m-slab-cleat", "of": "slab-cleat", "member": "frame", "part": "member"})

    def left(b):
        b["profile"], b["origin"] = [[0.0, r], [0.05 + t + 0.004, r]], [0.0, 3.275, 0.06]

    def right(b):
        b["profile"], b["origin"] = [[0.0, r], [0.05 + t + 0.004, r]], [1.35 - t - 0.004, 3.275, 0.06]

    body_of(v, "support-head-left", "hanging-bolt-left", change=left)
    body_of(v, "support-head-right", "hanging-bolt-right", change=right)
    body_of(v, "support-base-left", "spigot-left")
    body_of(v, "support-base-right", "spigot-right")
    v["decisions"].append(NOTE)
    if scale == "50":
        body_of(v, "joint-igu-setting-blocks", "igu-setting-blocks", keep=True)
        body_of(v, "joint-spandrel-setting-blocks", "spandrel-setting-blocks", keep=True)
        v["decisions"].append(BLOCKS)
L.save_source(SLUG, doc)
