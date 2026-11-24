"""🏗️ Repair ops of `tower-crane` of the third round (after the op of SAM), authored against the catalog file as it stood before this round (`SAM_SOURCE`). Volume 1:200 and 1:500: the three pendants (round bar of 50 mm, 18 to 41 m long) and the three runs of the hoist rope (14 mm) are capsules between the same end points instead of open tubes of revolution. The silhouette guard takes the centre line of a slender solid from its mesh; for a tube 800 times as long as thick that line came out 0.16 degrees askew, so the guard compared the 50 mm pendants as bands of 104 and 168 mm (`strokes.ts`), which closed the acute wedges against the jib chord and at the tower head (side elevation 1:200: 135.0 mm, now 60.0 mm; 1:500: 169.8 mm, now 112.5 mm). A capsule gets a straight line and its true width. The finer scales keep the tubes: there the same change measured no gain (1:100) or a loss (1:50 front 108.2 to 118.2 mm)."""
from oplib import load, write

ID = "tower-crane"
a = load(ID)
ops = []
for scale in ("200", "500"):
    for s in a["volumes"][scale]["body"]:
        if s["id"].startswith(("pendant", "hoist-rope")) and s["kind"] == "revolve":
            (l0, r), (l1, _) = s["profile"]
            rod = {k: v for k, v in s.items() if k not in ("profile", "axis", "origin")}
            rod.update(kind="capsule", p0=[round(o + d * l0, 5) for o, d in zip(s["origin"], s["axis"])], p1=[round(o + d * l1, 5) for o, d in zip(s["origin"], s["axis"])], r0=r, r1=r)
            ops.append({"op": "solid", "scale": scale, "id": s["id"], "solid": rod})
write(ID, ops)
