"""🧭 Round 3, machinery/evaporative-humidifier: the measures of the real unit and the frames of its bones, shared by the volumes and the 2D drawings.

World frame (rest): x along the air flow (inlet at -x, outlet at +x), y up, z lateral (left is +, the service cover is at -z); origin at the centre of the base on the floor.
A galvanised duct-section casing of 0.5 x 0.58 x 0.4 m stands on two base runners 20 mm high. Its lower 70 mm are the water tank. A 0.2 m deep cross-fluted cellulose
pad of 0.36 x 0.40 m stands in a cassette on two rails over the tank; a polypropylene distribution trough lies on the cassette; a bulkhead frame on the inlet side
closes the bypass around the pad and dips into the water; a submersible pump with a riser hose feeds the trough and a float valve keeps the level.
"""
RIG = {"casing": (-0.25, 0.0), "pad": (-0.1, 0.08), "trough": (-0.12, 0.48), "water": (-0.24, 0.02), "drip": (0.0, 0.5)}
HX = 0.25
HZ = 0.2
TOP = 0.6
BASE = 0.02
SHEET = 0.001
RUNNER_X = 0.19
RUNNER_W = 0.04
FRAME_BAR = 0.03
OPEN_Y = (0.09, 0.57)
OPEN_Z = 0.17
LEVEL = 0.053
PAD_X = 0.1
PAD_Z = 0.18
PAD_Y = (0.08, 0.48)
WINDOW_Y = (0.095, 0.47)
RAIL_X = 0.07
TROUGH_X = 0.12
TROUGH_Z = 0.19
TROUGH_Y = (0.48, 0.53)
TROUGH_WALL = 0.003
BULKHEAD_X = -0.12
PUMP = (0.17, 0.135)
PUMP_R = 0.03
HOSE_R = 0.008
FLOAT = (-0.19, 0.066, 0.09)
FLOAT_R = 0.0225
DROP_X = 0.104
DROP_Z = 0.1
OUTLET_Y = 0.3
OUTLET_R = 0.15
FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03}


def loc(bone, x, y):
    """📍 World rest point (x, y) in the local frame of a bone (every bone has angle 0)."""
    ox, oy = RIG[bone]
    return [round(x - ox, 6), round(y - oy, 6)]


def rect(x0, y0, x1, y1):
    """▭ A counter-clockwise rectangle polygon."""
    return [[x0, y0], [x1, y0], [x1, y1], [x0, y1]]


def flutes(count, depth):
    """〰️ Plan outline (z, x) of the pad with `count` vertical flutes of `depth` on its inlet and outlet faces; crests lie on the faces x = +-0.1, a crest is at z = 0."""
    step = 2 * PAD_Z / (2 * count)
    zs = [round(-PAD_Z + k * step, 6) for k in range(2 * count + 1)]
    inlet = [[z, round(-PAD_X + (0 if k % 2 == 0 else depth), 6)] for k, z in enumerate(zs)]
    outlet = [[z, round(PAD_X - (0 if k % 2 == 0 else depth), 6)] for k, z in enumerate(zs)]
    return inlet + outlet[::-1]


def hose_path(scale):
    """🪢 Centre line (x, y) of the riser hose at z = PUMP[1]: up from the pump nipple, over the trough rim and down into the trough."""
    x = PUMP[0]
    if scale == 50:
        return [[x, 0.105], [x, 0.3], [x, 0.52], [x - 0.004, 0.542], [x - 0.014, 0.556], [x - 0.03, 0.562], [0.11, 0.562], [0.098, 0.557], [0.092, 0.546], [0.09, 0.52]]
    return [[x, 0.105], [x, 0.53], [x - 0.012, 0.555], [x - 0.03, 0.562], [0.105, 0.562], [0.093, 0.55], [0.09, 0.52]]


def pump_outline():
    """⛽ Side outline (x, y) of the submersible pump: strainer foot, motor body, shoulder, outlet nipple."""
    x = PUMP[0]
    half = [[0.034, 0.021], [0.034, 0.033], [0.03, 0.033], [0.03, 0.083], [0.022, 0.091], [0.011, 0.091], [0.011, 0.109]]
    return [[round(x + a, 6), b] for a, b in half] + [[round(x - a, 6), b] for a, b in half[::-1]]
