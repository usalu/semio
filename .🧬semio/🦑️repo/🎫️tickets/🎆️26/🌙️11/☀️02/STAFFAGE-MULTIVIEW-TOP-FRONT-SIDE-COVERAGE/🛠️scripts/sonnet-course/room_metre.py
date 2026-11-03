import sys
from pathlib import Path
E = Path("C:/git/semio/🎓️teaching/🏛️architecture/⚡️energy")
IMPORT = '_manim_module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")\n'
edits = {
    "📊️demand/🎬️clip/♨️system-losses": [('room["h"] / 2.5', 'room_metre(room["h"])')],
    "📊️demand/🎬️clip/⚖️regulations": [('hs["w_height"] / 2 / 2.5', 'room_metre(hs["w_height"] / 2)')],
    "📊️demand/🎬️clip/⚡️demand-power": [("storey / 2.5", "room_metre(storey)")],
    "📊️demand/🎬️clip/🏭️primary-energy": [('hs["w_height"] / 2 / 2.5', 'room_metre(hs["w_height"] / 2)')],
    "📊️demand/🎬️clip/🏷️certificate": [("storey_h / 2.5", "room_metre(storey_h)"), ('hs["w_height"] / 2 / 2.5', 'room_metre(hs["w_height"] / 2)'), ('h_wp["w_height"] / 2 / 2.5', 'room_metre(h_wp["w_height"] / 2)')],
    "📊️demand/🎬️clip/🔁️recap": [('(H["mid_y"] - H["floor_y"]) / 2.5', 'room_metre(H["mid_y"] - H["floor_y"])'), ("storey scale (2.5 m clear height)", "storey scale")],
    "📊️demand/🎬️clip/🧪️practice": [("room_h / 2.5", "room_metre(room_h)")],
    "🔥️heating/🎬️clip/👥️internal-gains": [('house["h"] / 2 / 2.5', 'room_metre(house["h"] / 2)')],
    "🔥️heating/🎬️clip/🧮️calculation": [("room_h / 2.5", "room_metre(room_h)")],
    "🔥️heating/🎬️clip/🧲️fundamentals": [("(CEIL_Y - FLOOR_Y) / 2.5", "room_metre(CEIL_Y - FLOOR_Y)")],
    "🧲️physics/🎬️clip/🧲️fundamentals": [('fe_house["w_height"] / 2 / 2.5', 'room_metre(fe_house["w_height"] / 2)'), ('room["h"] / 2.5', 'room_metre(room["h"])')],
}
for folder, pairs in edits.items():
    p = E / folder / "🐍️.py"
    s = p.read_bytes().decode("utf-8")
    for old, new in pairs:
        if old not in s:
            print("MISSING", folder, old); continue
        s = s.replace(old, new)
    if "room_metre = _manim_module.room_metre" not in s:
        assert IMPORT in s, folder
        s = s.replace(IMPORT, IMPORT + "room_metre = _manim_module.room_metre\n", 1)
    p.write_bytes(s.encode("utf-8"))
v = E / "🏗️authoring/🎨️visuals/🐍️.py"
s = v.read_bytes().decode("utf-8")
anchor = "def fitted_staffage("
new = '''ROOM_HEIGHT_M = 2.5


def room_metre(room_height: float) -> float:
    """📏 Manim units per real metre of a room section drawn `room_height` units high between floor and ceiling (the clear storey height `ROOM_HEIGHT_M`); a staffage `unit` is this scene metre, so every cast keeps its real size."""
    return room_height / ROOM_HEIGHT_M


'''
assert "def room_metre" not in s
s = s.replace(anchor, new + anchor, 1)
v.write_bytes(s.encode("utf-8"))
