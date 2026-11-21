"""🌳 Writes r5-small-leaved-lime-model.ts from the round-3 script with the round-5 crown: every ring of sun lobes restated as flat billows (radial, vertical, tangential semi-axes stated per ring below), the 1:500 epicormic masses along their shoots, output under r5."""
import re

RINGS_50 = {"g": (0.72, 1.0, 1.25), "h": (0.82, 1.3, 1.45), "i": (0.85, 1.36, 1.5), "m": (0.86, 1.38, 1.52), "n": (0.85, 1.36, 1.5), "o": (0.84, 1.35, 1.48), "p": (0.82, 1.32, 1.45), "q": (0.8, 1.28, 1.4), "r": (0.78, 1.22, 1.36), "s": (0.74, 1.04, 1.3), "t": (0.7, 0.8, 1.2)}
RINGS_200 = {"g": (0.82, 1.15, 1.6), "h": (0.92, 1.48, 1.8), "i": (0.95, 1.52, 1.86), "m": (0.96, 1.55, 1.9), "n": (0.95, 1.52, 1.88), "o": (0.93, 1.52, 1.84), "p": (0.9, 1.48, 1.76), "q": (0.88, 1.42, 1.68), "r": (0.85, 1.34, 1.58), "s": (0.8, 1.12, 1.45), "t": (0.75, 0.86, 1.3)}
RINGS_500 = {"a": (1.2, 1.9, 2.35), "b": (1.2, 2.05, 2.45), "c": (1.2, 2.0, 2.4), "d": (1.15, 1.8, 2.2), "e": (1.05, 1.45, 1.95), "f": (0.9, 1.0, 1.6)}
EPI_500 = '500: [["epicormic-e", "trunk", [0.62, 1.58, 0], [0.7, 0.36, 0.36], 52], ["epicormic-w", "trunk", [-0.5, 2.33, 0], [0.53, 0.31, 0.3], 130]],'

src = open("r3-small-leaved-lime-model.ts", encoding="utf-8").read()
blocks = {"RINGS_50": RINGS_50, "RINGS_200": RINGS_200, "RINGS_500": RINGS_500}
for name, table in blocks.items():
    start = src.index(f"const {name}: Ring[] = [")
    end = src.index("];", start)
    body = src[start:end]
    for rid, (a, b, c) in table.items():
        body, n = re.subn(r'(\{ id: "%s", tier: \d, y: [\d.]+, )a: [\d.]+, b: [\d.]+, c: [\d.]+' % rid, r"\g<1>a: %s, b: %s, c: %s" % (a, b, c), body)
        assert n == 1, (name, rid)
    src = src[:start] + body + src[end:]

BILLOWS = """
/** ☁️ The 28 primary billows of the crown, one per branch group of the limb tiers: [azimuth, height, half span in metres]; a lobe near a billow axis stands out and swells, a lobe between billows sinks, so the crown surface undulates at the scale of the branch groups (2.5 to 3.5 m) and the ring lobes ride on it as the finer leaf masses. */
const BILLOWS: [az: number, y: number, span: number][] = [
  [10, 4.1, 1.7], [180, 5.3, 1.6], [70, 4.3, 1.6], [130, 4.0, 1.8], [190, 4.2, 1.7], [250, 4.4, 1.6], [310, 4.1, 1.7],
  [40, 6.0, 1.9], [95, 6.5, 1.8], [150, 6.2, 1.9], [205, 6.6, 1.8], [262, 6.1, 1.9], [320, 6.4, 1.8], [355, 6.9, 1.6],
  [15, 8.5, 1.8], [75, 8.9, 1.7], [135, 8.4, 1.8], [195, 8.8, 1.7], [255, 8.5, 1.8], [315, 8.9, 1.7],
  [45, 10.7, 1.6], [115, 10.9, 1.5], [185, 10.6, 1.6], [255, 11.0, 1.5], [325, 10.8, 1.6],
  [0, 12.6, 1.4], [120, 12.8, 1.4], [240, 12.7, 1.4],
];
const billow = (az: number, y: number): number => Math.max(...BILLOWS.map(([baz, by, span]) => {
  const arc = (Math.abs(((az - baz + 540) % 360) - 180) * D2R) * Math.max(1.2, ENV(az) * dome(y));
  return Math.exp(-((arc / span) ** 2 + ((y - by) / span) ** 2));
}));
"""
OLD_Y = "      const y = ring.y + dy;\n"
NEW_Y = OLD_Y + "      const g = billow(az, y);\n      const kk = k * (scale === 500 ? 1 : scale === 200 ? 0.925 + 0.09 * g : 0.865 + 0.165 * g);\n      const ss = s * (ring.tier === 5 ? 1 : 0.8 + 0.36 * g);\n"
assert OLD_Y in src
src = src.replace(OLD_Y, NEW_Y)
OLD_L = "ENV(az) * dome(y) * k, y, ring.a * s + swell[0], ring.b * s * tall + swell[1], ring.c * s * wide + swell[2]"
assert OLD_L in src
src = src.replace(OLD_L, "ENV(az) * dome(y) * kk, y, ring.a * ss + swell[0], ring.b * ss * tall + swell[1], ring.c * ss * wide + swell[2]")
anchor = "export type CrownLobe"
src = src.replace(anchor, BILLOWS.strip() + "\n\n" + anchor, 1)

src, n = re.subn(r'  500: \[\["epicormic-e".*\]\],', "  " + EPI_500, src)
assert n == 1
DOME_OLD = "(y >= 6.5 ? Math.max(0, 1 - ((y - 6.5) / 7.75) ** 2.3) ** 0.55"
DOME_NEW = "(y >= 6.5 ? Math.max(0, 1 - ((y - 6.5) / 7.75) ** 2.8) ** 0.5"
assert DOME_OLD in src
src = src.replace(DOME_OLD, DOME_NEW)
src = src.replace('join(GENERATED, "r3-small-leaved-lime")', 'join(GENERATED, "r5-small-leaved-lime")')
src = src.replace("/** 🌳️ Round-3 small-leaved lime", "/** 🌳️ Round-5 small-leaved lime (crown of flat leaf billows instead of ring bubbles)")
open("r5-small-leaved-lime-model.ts", "w", encoding="utf-8").write(src)
