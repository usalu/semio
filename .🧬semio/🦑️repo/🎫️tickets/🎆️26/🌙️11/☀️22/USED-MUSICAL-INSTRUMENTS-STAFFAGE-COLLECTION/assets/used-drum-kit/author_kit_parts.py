"""🧩️ Every part of the used drum kit per scale: solid kind, bone, geometry (in the bone's local frame), material, thermal region, tone, line role, how the 2D draws it and whether it is a structural member.

Part geometry: `cyl` (a, b, r), `tube` (a, b, ro, ri), `box` (c, size along u v w), `rev` (o, axis, profile [axial, radial]); points in metres in the local (u, v, w) frame of the bone.
Draw modes: `member` (a 2D member bar, exact in every pose), `hull` (convex hull polygon of the projected solid), `profile` (the exact revolved profile, vertical axis), `stroke` (centre-line stroke at 1:500), `none`.
"""
import math
import author_kit_layout as L
from author_kit_math import frames, to_local, to_world, add, sub, mul, unit, comb, norm

FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}


class Kit:
    """🥁️ The rig, its rest and played frames and the part lists of every scale."""

    def __init__(self):
        bones = L.rig()
        self.plays = L.add_drum_chains(bones)
        self.bones = L.ordered(bones)
        self.by = {b["id"]: b for b in self.bones}
        self.rest = frames(self.bones)
        self.played = frames(self.bones, L.played_offsets(self.plays))

    def lw(self, bone, p, space="rest"):
        """📍️ A world point of `space` (rest or played) in the local frame of a bone."""
        return to_local((self.rest if space == "rest" else self.played)[bone], p)

    def parts(self, scale):
        """🧱️ The parts of one scale."""
        self.scale = scale
        self.out = []
        self.rug()
        self.cases()
        self.trolley()
        self.throne()
        self.pedal()
        for key in ("hihat", "snare-stand", "crash", "ride"):
            self.stand(key)
        self.hihat_top()
        self.boom("crash")
        self.boom("ride")
        self.drums()
        return self.out

    def add(self, pid, bone, kind, geom, mat, region, tone, draw, struct=False, role="body", flags=None, d500=None):
        """➕️ One part."""
        p = {"id": pid, "bone": bone, "kind": kind, "g": geom, "mat": mat, "region": region, "tone": tone, "draw": draw, "struct": struct, "role": role, "flags": dict(flags or {})}
        self.out.append(p)
        return p

    def wcyl(self, pid, bone, a, b, r, mat, region, tone, draw, struct=False, role="body", flags=None, space="rest"):
        """🧴️ A cylinder between two world points."""
        return self.add(pid, bone, "cyl", {"a": self.lw(bone, a, space), "b": self.lw(bone, b, space), "r": r}, mat, region, tone, draw, struct, role, flags)

    def thin(self, r, length):
        """📏️ The radius a thin tube keeps at this scale and whether it is slender: true section when 20 times as long as thick, else the smallest feature at 1:500."""
        if length >= 20 * 2 * r:
            return r, True
        if self.scale == 500:
            return L_FEAT500 / 2, False
        return r, False

    def tube_draw(self, struct=True):
        """✏️ The draw mode of a thin tube at this scale."""
        if self.scale in (50, 100):
            return "member" if struct else "hull"
        if self.scale == 200:
            return "hull"
        return "stroke"

    def rug(self):
        """🟫️ The worn needle-felt drum rug under kit and trolley; feet and castors press into its 6 mm pile."""
        (x0, x1), (z0, z1), t = L.RUG
        self.add("rug", "rug", "box", {"c": self.lw("rug", [(x0 + x1) / 2, t / 2, (z0 + z1) / 2]), "s": [x1 - x0, t, z1 - z0]}, "carpet", "rug", 0.45, "hull", True, flags={"sheet": True})

    def cases(self):
        """🧳️ The five round hard cases: rotomoulded polyethylene wall, floor disc, telescoping lid with its skirt."""
        for cid, c in L.CASES.items():
            R, H = c["R"], c["H"]
            band = {}
            if self.scale <= 100:
                self.add(f"{cid}-wall", cid, "tube", {"a": [0, 0, 0], "b": [H - L.FLOOR, 0, 0], "ro": R, "ri": R - L.WALL}, "pe-case", "cases", 0.2, "member", True, flags=band)
                self.add(f"{cid}-floor", cid, "cyl", {"a": [0, 0, 0], "b": [L.FLOOR, 0, 0], "r": R - L.WALL}, "pe-case", "cases", 0.2, "hull", flags={"sheet": True})
                self.add(f"{cid}-lid", cid, "cyl", {"a": [H - L.FLOOR, 0, 0], "b": [H, 0, 0], "r": R + L.SKIRT_T}, "pe-case", "cases", 0.16, "hull", flags={"sheet": True})
                self.add(f"{cid}-skirt", cid, "tube", {"a": [H - L.SKIRT_H, 0, 0], "b": [H - L.FLOOR, 0, 0], "ro": R + L.SKIRT_T, "ri": R}, "pe-case", "cases", 0.16, "hull", flags={"slender": True})
            elif self.scale == 500:
                R = {"case-snare": L.CASES["case-floor"]["R"], "case-tom10": L.CASES["case-tom12"]["R"]}.get(cid, R)
                self.add(f"{cid}-wall", cid, "cyl", {"a": [0, 0, 0], "b": [H, 0, 0], "r": R}, "pe-case", "cases", 0.2, "hull", True)
                continue
            else:
                self.add(f"{cid}-wall", cid, "tube", {"a": [0, 0, 0], "b": [H, 0, 0], "ro": R, "ri": R - L.WALL}, "pe-case", "cases", 0.2, "hull", True, flags=band)
                self.add(f"{cid}-floor", cid, "cyl", {"a": [0, 0, 0], "b": [L.FLOOR, 0, 0], "r": R - L.WALL}, "pe-case", "cases", 0.2, "none", flags={"sheet": True})
                self.add(f"{cid}-lid", cid, "cyl", {"a": [H - L.FLOOR, 0, 0], "b": [H, 0, 0], "r": R - L.WALL}, "pe-case", "cases", 0.16, "none", flags={"sheet": True})

    def trolley(self):
        """🛒️ Platform trolley: plywood deck on a steel frame, four castors (two swivel, two fixed) and the push handle."""
        x0, y0, z0 = L.TROLLEY
        s = self.scale
        self.add("deck", "trolley", "box", {"c": self.lw("trolley", [x0, 0.186, z0]), "s": [1.20, 0.018, 0.70]}, "plywood", "trolley-deck", 0.62, "member" if s <= 100 else "hull", True, flags={"sheet": True} if s >= 200 else None)
        if s <= 200:
            for side, zs in (("l", 1), ("r", -1)):
                self.add(f"rail-{side}", "trolley", "box", {"c": self.lw("trolley", [x0, 0.162, z0 + 0.335 * zs]), "s": [1.20, 0.03, 0.03]}, "steel-powder", "trolley-frame", 0.38, "member" if s <= 100 else "hull", True)
            for end, xs in (("front", 1), ("rear", -1)):
                self.add(f"rail-{end}", "trolley", "box", {"c": self.lw("trolley", [x0 + 0.585 * xs, 0.162, z0]), "s": [0.03, 0.03, 0.64]}, "steel-powder", "trolley-frame", 0.38, "hull", True)
        for key, xl in (("front", 0.48), ("rear", -0.48)):
            wb = f"trolley-wheels-{key}"
            for side, zs in (("l", 1), ("r", -1)):
                zc = z0 + 0.31 * zs
                xc = x0 + xl
                if s <= 200:
                    self.add(f"castor-{key}-{side}", "trolley", "box", {"c": self.lw("trolley", [xc, 0.111 if s <= 100 else 0.111, zc]), "s": [0.06, 0.072 if s <= 100 else 0.072, 0.05]}, "steel-powder", "trolley-frame", 0.3, "hull", True)
                    self.add(f"castor-plate-{key}-{side}", "trolley", "box", {"c": self.lw("trolley", [xc, 0.143, zc]), "s": [0.09, 0.008, 0.08]}, "steel-powder", "trolley-frame", 0.3, "hull") if s == 50 else None
                    self.add(f"wheel-{key}-{side}", wb, "cyl", {"a": self.lw(wb, [xc, 0.0625, zc - 0.0175]), "b": self.lw(wb, [xc, 0.0625, zc + 0.0175]), "r": 0.0625}, "tpr-wheel", "trolley-wheels", 0.14, "hull")
                else:
                    self.add(f"castor-{key}-{side}", "trolley", "box", {"c": self.lw("trolley", [xc, 0.0885, zc]), "s": [0.10, 0.177, 0.075]}, "tpr-wheel", "trolley-wheels", 0.14, "hull", True)
        hx = x0 - 0.615
        for side, zs in (("l", 1), ("r", -1)):
            r, sl = self.thin(0.0125, 0.785)
            self.wcyl(f"handle-{side}", "trolley", [hx, 0.15, z0 + 0.30 * zs], [hx, 0.935, z0 + 0.30 * zs], r, "steel-powder", "trolley-frame", 0.38, self.tube_draw(), True, flags={"slender": sl})
            if s <= 200:
                self.add(f"handle-bracket-{side}", "trolley", "box", {"c": self.lw("trolley", [hx + 0.01, 0.165, z0 + 0.30 * zs]), "s": [0.03, 0.04, 0.04]}, "steel-powder", "trolley-frame", 0.38, "hull")
        r, sl = self.thin(0.0125, 0.625)
        self.wcyl("handle-bar", "trolley", [hx, 0.935, z0 - 0.3125], [hx, 0.935, z0 + 0.3125], r, "steel-powder", "trolley-frame", 0.38, self.tube_draw(), True, flags={"slender": sl})

    def tripod(self, bone, cx, cz, hub, reach, legR, azimuths, tone=0.72, braces=True):
        """🦵️ A tripod base: legs from the hub to rubber feet, the hub collar and (at 1:50) the braces to the lower collar."""
        s = self.scale
        for k, az in enumerate(azimuths):
            ca, sa = math.cos(math.radians(az)), math.sin(math.radians(az))
            top = [cx + 0.022 * ca, hub, cz + 0.022 * sa]
            tip = [cx + reach * ca, L.FOOT_H if s < 500 else 0.0, cz + reach * sa]
            ln = norm(sub(tip, top))
            r, sl = self.thin(legR, ln)
            if s == 500:
                dy = (top[1] - tip[1]) / ln
                tip[1] = r * math.sqrt(1 - dy * dy)
            self.wcyl(f"{bone}-leg-{k + 1}", bone, top, tip, r, "chrome-steel", "stand-chrome", tone, "member" if s <= 100 else "hull", True, flags={"slender": sl})
            if s < 500:
                self.wcyl(f"{bone}-foot-{k + 1}", bone, [tip[0], 0.0, tip[2]], [tip[0], L.FOOT_H, tip[2]], L.FOOT_R, "rubber", "stand-chrome", 0.1, "hull")
            if braces and s == 50:
                mid = comb(top, 0.55, tip, 0.45)
                low = [cx + 0.02 * ca, 0.085, cz + 0.02 * sa]
                self.wcyl(f"{bone}-brace-{k + 1}", bone, low, mid, 0.0075, "chrome-steel", "stand-chrome", tone, "hull", role="detail")
        if s < 500:
            self.wcyl(f"{bone}-hub", bone, [cx, hub - 0.025, cz], [cx, hub + 0.025, cz], 0.024, "chrome-steel", "stand-chrome", 0.66, "hull")
        if braces and s == 50:
            self.wcyl(f"{bone}-collar", bone, [cx, 0.07, cz], [cx, 0.10, cz], 0.022, "chrome-steel", "stand-chrome", 0.66, "hull", role="detail")

    def throne(self):
        """🪑️ The throne: tripod, outer tube, spindle post, seat plate and round vinyl cushion."""
        t = L.THRONE
        s = self.scale
        self.tripod("throne", 0.0, 0.0, t["hub"], t["reach"], t["legR"], t["legs"], braces=True)
        r, sl = self.thin(t["tubeR"], t["tubeTop"] - 0.07)
        self.wcyl("throne-tube", "throne", [0, 0.07, 0], [0, t["tubeTop"], 0], r, "chrome-steel", "stand-chrome", 0.7, self.tube_draw(), True, flags={"slender": sl})
        r, sl = self.thin(t["postR"], t["seatRest"] - 0.18)
        self.wcyl("throne-post", "throne-seat", [0, 0.18, 0], [0, t["seatRest"], 0], r, "chrome-steel", "stand-chrome", 0.74, self.tube_draw(), True, flags={"slender": sl})
        R = t["cushR"]
        prof = [[0.0, 0.15], [0.012, 0.15], [0.014, R - 0.01], [0.022, R], [0.078, R], [0.092, R - 0.012], [0.10, R - 0.03], [0.10, 0.0]] if s <= 100 else [[0.0, 0.15], [0.012, 0.15], [0.016, R], [0.084, R], [0.10, R - 0.025], [0.10, 0.0]]
        self.add("throne-cushion", "throne-seat", "rev", {"o": self.lw("throne-seat", [0, t["seatRest"], 0]), "axis": [1, 0, 0], "profile": prof}, "vinyl-seat", "throne-seat", 0.12, "profile")

    def pedal(self):
        """🦶️ The bass drum pedal: base plate, posts, axle, hoop clamp, footboard on its heel hinge and the felt beater."""
        s = self.scale
        if s < 500:
            self.add("pedal-base", "kick-pedal", "box", {"c": self.lw("kick-pedal", [0.435, 0.004, 0.0]), "s": [0.29, 0.008, 0.10]}, "alu-cast", "stand-chrome", 0.55, "member" if s == 50 else "hull", True)
            for side, zs in (("l", 1), ("r", -1)):
                self.wcyl(f"pedal-post-{side}", "kick-pedal", [0.555, 0.008, 0.055 * zs], [0.555, 0.21, 0.055 * zs], 0.010, "chrome-steel", "stand-chrome", 0.7, "member" if s <= 100 else "hull", True)
            self.wcyl("pedal-axle", "kick-pedal", [0.555, 0.20, -0.068], [0.555, 0.20, 0.068], 0.012, "chrome-steel", "stand-chrome", 0.66, "hull")
            self.add("pedal-clamp", "kick-pedal", "box", {"c": self.lw("kick-pedal", [0.59, 0.0205, 0.0]), "s": [0.03, 0.025, 0.07]}, "chrome-steel", "stand-chrome", 0.6, "hull")
            self.add("footboard", "kick-footboard", "box", {"c": [0.13, 0.009, 0.0], "s": [0.26, 0.018, 0.085]}, "alu-cast", "stand-chrome", 0.5, "member" if s <= 100 else "hull", True)
            self.add("footboard-hinge", "kick-footboard", "cyl", {"a": [0.0, 0.004, -0.045], "b": [0.0, 0.004, 0.045], "r": 0.012}, "alu-cast", "stand-chrome", 0.5, "hull")
            if s == 50:
                self.add("pedal-strap", "kick-footboard", "cyl", {"a": [0.25, 0.018, 0.0], "b": self.lw("kick-footboard", [0.540, 0.188, 0.0]), "r": 0.007}, "steel-powder", "stand-chrome", 0.3, "hull", role="detail")
        else:
            self.add("pedal-frame", "kick-pedal", "box", {"c": self.lw("kick-pedal", [0.5575, 0.105, 0.0]), "s": [0.075, 0.21, 0.13]}, "alu-cast", "stand-chrome", 0.55, "hull", True)
            self.add("footboard", "kick-footboard", "box", {"c": [0.13, 0.0255, 0.0], "s": [0.26, 0.075, 0.085]}, "alu-cast", "stand-chrome", 0.5, "hull", True)
        rr = 0.004 if s < 500 else 0.004
        self.add("beater-shaft", "kick-beater", "cyl", {"a": [0.008, 0.0, 0.0], "b": [0.17, 0.0, 0.0], "r": rr}, "chrome-steel", "stand-chrome", 0.7, "hull" if s < 500 else "stroke", True, flags={"slender": True})
        if s < 500:
            self.add("beater-hub", "kick-beater", "cyl", {"a": [0.0, 0.0, -0.02], "b": [0.0, 0.0, 0.02], "r": 0.016}, "chrome-steel", "stand-chrome", 0.66, "hull")
        hr = 0.03 if s < 500 else 0.0375
        self.add("beater-head", "kick-beater", "cyl", {"a": [0.185, 0.0, -0.0225 if s < 500 else -0.0375], "b": [0.185, 0.0, 0.0225 if s < 500 else 0.0375], "r": hr}, "felt", "stand-chrome", 0.85, "hull")

    def stand(self, key):
        """🎚️ A stand base (tripod, lower tube, memory clamp) and its telescoping upper tube."""
        st = L.STANDS[key]
        bone = "snare-stand" if key == "snare-stand" else f"{key}-stand"
        upper = "snare-basket" if key == "snare-stand" else f"{key}-upper"
        cx, cz = st["at"]
        s = self.scale
        self.tripod(bone, cx, cz, st["hub"], st["reach"], st["legR"], st["legs"])
        bottom = 0.03 if key in ("crash", "ride", "hihat") else 0.06
        r, sl = self.thin(st["tubeR"], st["tubeTop"] - bottom)
        self.wcyl(f"{bone}-tube", bone, [cx, bottom, cz], [cx, st["tubeTop"], cz], r, "chrome-steel", "stand-chrome", 0.72, self.tube_draw(), True, flags={"slender": sl})
        if s < 500:
            self.wcyl(f"{bone}-clamp", bone, [cx, st["tubeTop"] - 0.03, cz], [cx, st["tubeTop"] + 0.01, cz], 0.02, "chrome-steel", "stand-chrome", 0.6, "hull")
        top = st["tubeTop"] + (st["upperRest"] if key != "snare-stand" else 0.03)
        ln = st["upperLen"]
        r, sl = self.thin(st["upperR"], ln)
        if key == "snare-stand" and s == 100:
            r = max(r, 0.0075)
        self.wcyl(f"{upper}-tube", upper, [cx, top - ln, cz], [cx, top, cz], r, "chrome-steel", "stand-chrome", 0.76, self.tube_draw(), True, flags={"slender": sl})
        if key == "hihat":
            self.hihat_bottom(cx, cz, top)
            self.hihat_footboard(cx, cz)
        if key == "snare-stand":
            self.basket(cx, cz, top)

    def basket(self, cx, cz, top):
        """🧺️ The snare basket: hub and three arms with rubber grips."""
        s = self.scale
        if s == 500:
            return
        self.wcyl("basket-hub", "snare-basket", [cx, top - 0.01, cz], [cx, top + 0.02, cz], 0.025, "chrome-steel", "stand-chrome", 0.62, "hull")
        ar = 0.006 if s == 50 else (0.0075 if s == 100 else 0.006)
        for k, az in enumerate((0, 120, 240)):
            ca, sa = math.cos(math.radians(az)), math.sin(math.radians(az))
            a = [cx + 0.02 * ca, top + 0.005, cz + 0.02 * sa]
            b = [cx + 0.19 * ca, top + 0.005, cz + 0.19 * sa]
            self.wcyl(f"basket-arm-{k + 1}", "snare-basket", a, b, ar, "chrome-steel", "stand-chrome", 0.7, "hull")
            if s <= 100:
                self.wcyl(f"basket-grip-{k + 1}", "snare-basket", [cx + 0.198 * ca, top - 0.003, cz + 0.198 * sa], [cx + 0.198 * ca, top + 0.03, cz + 0.198 * sa], 0.0085, "rubber", "stand-chrome", 0.1, "hull")

    def cymbal(self, pid, bone, base, axis, R, rb, Hb, bow, t):
        """🔔️ A cymbal as its real thin bowed section with the bell (1:50 to 1:200) or as a flat sheet disc (1:500); `base` is the local point of the edge bottom on the axis."""
        if self.scale == 500:
            tt = 0.012
            mid = bow * 0.5
            prof = [[mid, R], [mid + tt, R], [mid + tt, 0.0]]
            prof = [[mid, 0.0], [mid, R], [mid + tt, R], [mid + tt, 0.0]]
            return self.add(pid, bone, "rev", {"o": base, "axis": axis, "profile": prof}, "bronze-b20", "cymbals", 0.62, "profile500", flags={"sheet": True})
        if self.scale == 200:
            prof = [[bow + Hb, 0.0], [bow + Hb, 0.4 * rb], [bow, rb], [0.0, R], [t + 0.001, R], [bow + t + 0.001, rb], [bow + Hb + t + 0.001, 0.4 * rb], [bow + Hb + t + 0.001, 0.0]]
        else:
            prof = [[bow + Hb, 0.0], [bow + Hb, 0.45 * rb], [bow + 0.7 * Hb, 0.85 * rb], [bow, rb], [0.55 * bow, 0.5 * (rb + R)], [0.0, R], [t, R], [0.55 * bow + t, 0.5 * (rb + R)], [bow + t, rb], [bow + 0.7 * Hb + t, 0.85 * rb], [bow + Hb + t, 0.45 * rb], [bow + Hb + t, 0.0]]
        return self.add(pid, bone, "rev", {"o": base, "axis": axis, "profile": prof}, "bronze-b20", "cymbals", 0.62, "profile")

    def hihat_bottom(self, cx, cz, top):
        """🎩️ The bottom hi-hat cymbal on its seat cup."""
        s = self.scale
        cup = 0.012 if s == 50 else 0.0
        if s < 500:
            self.wcyl("hihat-cup", "hihat-upper", [cx, top - 0.004, cz], [cx, top + max(cup, 0.0), cz], 0.02, "felt", "cymbals", 0.8, "hull") if s == 50 else None
        bow, Hb = 0.014, 0.016
        seat = top + cup
        base = self.lw("hihat-upper", [cx, seat - (bow + Hb), cz])
        self.cymbal("hihat-bottom", "hihat-upper", base, [1, 0, 0], 0.178, 0.06, Hb, bow, 0.0015)

    def hihat_top(self):
        """🎩️ The pull rod, the clutch and the top hi-hat cymbal, open 12 mm when stored."""
        st = L.STANDS["hihat"]
        cx, cz = st["at"]
        s = self.scale
        top = st["tubeTop"] + st["upperRest"]
        cup = 0.012 if s == 50 else 0.0
        bow, Hb, t = 0.014, 0.016, 0.0015
        edge_bottom = top + cup - (bow + Hb) + t + L.HIHAT_OPEN
        if s == 500:
            edge_bottom = top + cup - (bow + Hb) + 0.5 * bow + 0.012 + L.HIHAT_OPEN
            base = self.lw("hihat-rod", [cx, edge_bottom - 0.5 * bow, cz])
        else:
            base = self.lw("hihat-rod", [cx, edge_bottom, cz])
        self.cymbal("hihat-top", "hihat-rod", base, [1, 0, 0], 0.178, 0.06, Hb, bow, t)
        bell_top = edge_bottom + bow + Hb + t if s < 500 else edge_bottom + 0.012
        if s < 500:
            self.wcyl("hihat-clutch", "hihat-rod", [cx, bell_top, cz], [cx, bell_top + 0.035, cz], 0.017, "chrome-steel", "stand-chrome", 0.66, "hull")
        r, sl = self.thin(0.0035, 0.66)
        self.wcyl("hihat-rod", "hihat-rod", [cx, bell_top + 0.045 - 0.66, cz], [cx, bell_top + 0.045, cz], 0.0035, "chrome-steel", "stand-chrome", 0.74, "hull" if s < 500 else "stroke", True, flags={"slender": True})

    def hihat_footboard(self, cx, cz):
        """🦶️ The hi-hat footboard and its heel plate."""
        s = self.scale
        hb = "hihat-footboard"
        th = 0.018 if s < 500 else 0.075
        self.add("hihat-board", hb, "box", {"c": [0.12, th / 2 - (0.0 if s < 500 else 0.012), 0.0], "s": [0.24, th, 0.08]}, "alu-cast", "stand-chrome", 0.5, "member" if s <= 100 else "hull", True)
        if s < 500:
            self.add("hihat-heel", hb, "cyl", {"a": [0.0, 0.0, -0.042], "b": [0.0, 0.0, 0.042], "r": 0.012}, "alu-cast", "stand-chrome", 0.5, "hull")
            self.add("hihat-heel-plate", "hihat-stand", "box", {"c": self.lw("hihat-stand", [0.15, 0.004, cz]), "s": [0.008, 0.06, 0.09]}, "alu-cast", "stand-chrome", 0.5, "hull")
            self.add("hihat-pedal-frame", "hihat-stand", "box", {"c": self.lw("hihat-stand", [cx - 0.01, 0.03, cz]), "s": [0.06, 0.09, 0.06]}, "alu-cast", "stand-chrome", 0.55, "hull")

    def boom(self, key):
        """🦒️ A boom arm with its tilter, the felts and the cymbal."""
        st = L.STANDS[key]
        s = self.scale
        bb, cb = f"{key}-boom", f"{key}-cymbal"
        r, sl = self.thin(0.0095 if key == "ride" else 0.0095, st["boom"])
        self.add(f"{key}-boom-tube", bb, "cyl", {"a": [0.0, 0.0, 0.0], "b": [st["boom"], 0.0, 0.0], "r": r}, "chrome-steel", "stand-chrome", 0.74, self.tube_draw(), True, flags={"slender": sl})
        if s < 500:
            self.add(f"{key}-boom-joint", bb, "cyl", {"a": [0.0, 0.0, -0.026], "b": [0.0, 0.0, 0.026], "r": 0.02}, "chrome-steel", "stand-chrome", 0.6, "hull")
        R, rb, Hb, bow, t = (0.203, 0.065, 0.018, 0.016, 0.0011) if key == "crash" else (0.254, 0.075, 0.020, 0.022, 0.0014)
        felt = 0.008
        if s < 500:
            self.add(f"{key}-mount", cb, "cyl", {"a": [0.0, -0.03, 0.0], "b": [0.0, felt if s > 50 else 0.0, 0.0], "r": 0.016 if s == 50 else 0.02}, "chrome-steel", "stand-chrome", 0.6, "hull")
            if s == 50:
                self.add(f"{key}-felt", cb, "cyl", {"a": [0.0, 0.0, 0.0], "b": [0.0, felt, 0.0], "r": 0.02}, "felt", "cymbals", 0.82, "hull")
                self.add(f"{key}-wingnut", cb, "cyl", {"a": [0.0, felt + t, 0.0], "b": [0.0, felt + t + 0.012, 0.0], "r": 0.017}, "felt", "cymbals", 0.82, "hull")
            self.cymbal(f"{key}-cymbal", cb, [0.0, felt - (bow + Hb), 0.0], [0, 1, 0], R, rb, Hb, bow, t)
        else:
            self.cymbal(f"{key}-cymbal", cb, [0.0, -0.5 * bow, 0.0], [0, 1, 0], R, rb, Hb, bow, t)

    def drum(self, did, bone, axis, e1, e2, spec, region_hoop, kick=False):
        """🥁️ A drum: shell, two hoops, two heads and the tube lugs, in its local frame with the axis along `axis`."""
        s = self.scale
        R, D = spec["R"], spec["D"]
        P = lambda a, r=0.0, phi=0.0: add(mul(axis, a), comb(e1, r * math.cos(phi), e2, r * math.sin(phi)))
        shell_mat = "chrome-steel" if did == "snare" else "birch-ply"
        shell_region = "drum-hardware" if did == "snare" else "drum-shells"
        if kick:
            spans = [(-spec["hoopOut"], spec["hoopIn"]), (D - spec["hoopIn"], D + spec["hoopOut"])]
            hoop_mat, hoop_tone = "birch-ply", 0.4
        else:
            spans = [(-L.HOOP_DOWN, L.HOOP_UP), (D - L.HOOP_UP, D + L.HOOP_DOWN)]
            hoop_mat, hoop_tone = "chrome-steel", 0.68
        if s == 500:
            self.add(f"{did}-body", bone, "cyl", {"a": P(spans[0][0]), "b": P(spans[1][1]), "r": spec["hoopO"]}, shell_mat, shell_region, 0.62 if did == "snare" else 0.42, "hull", True, flags={"thermalOnly": False, "noThermal": True})
            self.add(f"{did}-shell", bone, "tube", {"a": P(0), "b": P(D), "ro": R, "ri": R - spec["ply"]}, shell_mat, shell_region, 0.42, "none", flags={"thermalOnly": True})
        else:
            self.add(f"{did}-shell", bone, "tube", {"a": P(0), "b": P(D), "ro": R, "ri": R - spec["ply"]}, shell_mat, shell_region, 0.62 if did == "snare" else 0.42, "member" if s <= 100 else "hull", True)
        hoop_i = spec["hoopI"] if s != 200 else R
        for k, (a0, a1) in enumerate(spans):
            self.add(f"{did}-hoop-{('batter', 'reso')[k] if kick else ('bottom', 'top')[k]}", bone, "tube", {"a": P(a0), "b": P(a1), "ro": spec["hoopO"], "ri": hoop_i}, hoop_mat, region_hoop, hoop_tone, "hull" if s < 500 else "none", flags={"slender": True, "thermalOnly": s == 500})
        heads = [(-L.HEAD_T, 0.0), (D, D + L.HEAD_T)]
        for k, (a0, a1) in enumerate(heads):
            if kick and k == 1 and s <= 200:
                rr = spec["hoopI"] - 0.001
                self.add("kick-head-reso", bone, "ext", {"u0": a0, "u1": a1, "r": rr, "hole": (-0.13, 0.06, 0.05)}, "pet-head", "drum-heads", 0.86, "hull", flags={"sheet": True})
                continue
            self.add(f"{did}-head-{('batter', 'reso')[k] if kick else ('bottom', 'top')[k]}", bone, "cyl", {"a": P(a0), "b": P(a1), "r": (spec["hoopI"] - 0.001) if s != 200 else R}, "pet-head", "drum-heads", 0.9 if k == (0 if kick else 1) else 0.86, "hull" if s < 500 else "none", flags={"sheet": True, "thermalOnly": s == 500})
        lugR = spec["lugR"]
        keep = s == 50 or (s == 100 and 2 * lugR >= 0.02)
        if keep:
            n = spec["lugs"]
            for k in range(n):
                phi = math.pi * (2 * k + 1) / n
                self.add(f"{did}-lug-{k + 1}", bone, "cyl", {"a": P(0.025 if not kick else 0.03, R + lugR - 0.003, phi), "b": P(D - (0.025 if not kick else 0.03), R + lugR - 0.003, phi), "r": lugR}, "chrome-steel", "drum-hardware", 0.7, "hull", role="detail")

    def radial(self, pid, bone, axis, e1, e2, at, phi, r0, r1, rr, mat, region, tone, struct=False):
        """📍️ A radial cylinder on a drum shell (bracket, boss)."""
        P = lambda a, r: add(mul(axis, a), comb(e1, r * math.cos(phi), e2, r * math.sin(phi)))
        return self.add(pid, bone, "cyl", {"a": P(at, r0), "b": P(at, r1), "r": rr}, mat, region, tone, "hull", struct)

    def drums(self):
        """🥁️ All five drums, the tom holder, the spurs and the floor tom legs."""
        s = self.scale
        U, V, W = [1, 0, 0], [0, 1, 0], [0, 0, 1]
        k = L.KICK
        self.drum("kick", "kick", U, V, W, k, "drum-shells", kick=True)
        if s < 500:
            self.radial("kick-mount", "kick", U, V, W, 0.20, 0.0, k["R"] - 0.006, 0.300 if s < 200 else 0.305, 0.028 if s < 200 else 0.03, "chrome-steel", "drum-hardware", 0.64)
            for side, zs in (("l", 1), ("r", -1)):
                self.radial(f"kick-spur-bracket-{side}", "kick", U, V, W, 0.33, math.radians(135 if zs > 0 else -135), k["R"] - 0.006, 0.312, 0.016, "chrome-steel", "drum-hardware", 0.64)
                self.spur(side)
        self.holder()
        for tid, spec in (("tom10", L.TOM10), ("tom12", L.TOM12)):
            self.drum(tid, tid, V, U, W, spec, "drum-hardware")
            if s < 500:
                self.radial(f"{tid}-bracket", tid, V, U, W, 0.10 if tid == "tom10" else 0.11, 0.0, spec["R"] - 0.004, spec["R"] + 0.045, 0.018, "chrome-steel", "drum-hardware", 0.64)
        f = L.FLOORTOM
        self.drum("floor-tom", "floor-tom", U, V, W, f, "drum-hardware")
        for j, az in enumerate((90, 210, 330)):
            if s < 500:
                self.radial(f"floor-bracket-{j + 1}", "floor-tom", U, V, W, 0.15, math.radians(az), f["R"] - 0.004, f["R"] + 0.04, 0.016, "chrome-steel", "drum-hardware", 0.64)
            P = lambda a: add(mul(U, a), comb(V, 0.232 * math.cos(math.radians(az)), W, 0.232 * math.sin(math.radians(az))))
            self.add(f"floor-leg-{j + 1}", "floor-tom-legs", "cyl", {"a": P(-0.006 + 0.025 if s < 500 else -0.006), "b": P(0.40), "r": 0.00525}, "chrome-steel", "drum-hardware", 0.72, "hull" if s < 500 else "stroke", True, flags={"slender": True})
            if s < 500:
                self.add(f"floor-foot-{j + 1}", "floor-tom-legs", "cyl", {"a": P(-0.006), "b": P(-0.006 + 0.025 if s < 200 else 0.024), "r": 0.013 if s < 200 else 0.015}, "rubber", "drum-hardware", 0.1, "hull")
        self.drum("snare", "snare", U, V, W, L.SNARE, "drum-hardware")
        if s < 200:
            self.radial("snare-strainer", "snare", U, V, W, 0.07, 0.0, L.SNARE["R"] - 0.002, L.SNARE["R"] + 0.035, 0.016, "chrome-steel", "drum-hardware", 0.64)
            self.radial("snare-butt", "snare", U, V, W, 0.07, math.pi, L.SNARE["R"] - 0.002, L.SNARE["R"] + 0.022, 0.014, "chrome-steel", "drum-hardware", 0.64)

    def spur(self, side):
        """🦯️ A folding bass drum spur with its rubber tip."""
        b = f"kick-spur-{side}"
        s = self.scale
        r = 0.008 if s < 200 else 0.015
        self.add(f"{b}-rod", b, "cyl", {"a": [0.0, 0.0, 0.0], "b": [L.SPUR_L - 0.028, 0.0, 0.0], "r": r}, "chrome-steel", "drum-hardware", 0.7, "member" if s <= 100 else "hull", True)
        self.add(f"{b}-tip", b, "cyl", {"a": [L.SPUR_L - 0.03, 0.0, 0.0], "b": [L.SPUR_L, 0.0, 0.0], "r": L.SPUR_TIP_R if s < 200 else 0.015}, "rubber", "drum-hardware", 0.1, "hull")

    def holder(self):
        """🦾️ The double tom holder: post in the bass drum mount, ball-clamp block and the two arms into the tom brackets."""
        s = self.scale
        hb = "kick-holder"
        if s == 500:
            return
        self.add("holder-post", hb, "cyl", {"a": [0.0, 0.0, 0.0], "b": [0.20, 0.0, 0.0], "r": 0.011 if s < 200 else 0.015}, "chrome-steel", "drum-hardware", 0.7, "member" if s <= 100 else "hull", True)
        self.add("holder-clamp", hb, "box", {"c": [0.215, 0.0, 0.0], "s": [0.05, 0.05, 0.10]}, "chrome-steel", "drum-hardware", 0.62, "hull")
        ar = {50: 0.0065, 100: 0.0075, 200: 0.015}[s]
        for tid in ("tom10", "tom12"):
            spec = L.TOM10 if tid == "tom10" else L.TOM12
            at = 0.10 if tid == "tom10" else 0.11
            end_world = to_world(self.played[tid], [spec["R"] + 0.025, at, 0.0])
            end = to_local(self.played[hb], end_world)
            zs = 1 if end[2] > 0 else -1
            self.add(f"holder-arm-{tid}", hb, "cyl", {"a": [0.215, 0.0, 0.05 * zs], "b": end, "r": ar}, "chrome-steel", "drum-hardware", 0.7, "hull", True)


L_FEAT500 = 0.075
