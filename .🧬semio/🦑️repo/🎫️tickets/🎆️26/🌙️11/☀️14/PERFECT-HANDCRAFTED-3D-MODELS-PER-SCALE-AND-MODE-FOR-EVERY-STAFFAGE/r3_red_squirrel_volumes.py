"""🧊 Round 3 red squirrel: the volumes of every scale assembled from the handcrafted lists."""
from r3_red_squirrel_geo import clean
import r3_red_squirrel_body50 as b50
import r3_red_squirrel_body100 as b100
import r3_red_squirrel_lists as lists
import r3_red_squirrel_text as text
import r4_red_squirrel_coarse as coarse


def scale(denominator, body, thermal, structure):
    return {"status": "authored", "denominator": denominator, "body": [clean(s) for s in body], "thermal": [clean(s) for s in thermal], "structure": [clean(s) for s in structure], "decisions": {**text.VOLUME_DECISIONS, **coarse.DECISIONS}[str(denominator)]}


def volumes():
    body50, body100, body200, body500 = b50.body(), b100.body(), coarse.body200(), coarse.body500()
    return {
        "50": scale(50, body50, lists.thermal50(body50), lists.skeleton50()),
        "100": scale(100, body100, lists.thermal100(body100), lists.skeleton100()),
        "200": scale(200, body200, coarse.thermal(body200, 200), coarse.skeleton(200)),
        "500": scale(500, body500, coarse.thermal(body500, 500), coarse.skeleton(500)),
        "_solids": {"50": body50, "100": body100},
    }
