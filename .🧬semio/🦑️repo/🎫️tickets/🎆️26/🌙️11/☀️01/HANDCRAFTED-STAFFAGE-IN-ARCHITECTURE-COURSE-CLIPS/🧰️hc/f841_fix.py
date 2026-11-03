"""🧹 Drop unused locals (F841) in ticket clip files; side-effect calls keep their expression."""
import os
import sys

A = "🎓️teaching/🏛️architecture"
PY = "🐍️.py"

TARGETS = {
    "summer": (["summer-strategy"], []),
    "mech": (["ventilation", "mechanical"], []),
    "reg": (["regulations"], []),
    "pe": (["primary-energy"], []),
    "prac": (["practice"], []),
    "calc": (["calculation"], []),
    "cond": (["conduction"], []),
    "phys": (["physics", "fundamentals"], []),
    "idx": (["optimization", "index"], ["visuals"]),
    "ext": (["optimization", "extrema"], ["visuals"]),
    "exp": (["optimization", "experiments"], ["visuals"]),
    "otest": (["optimization", "tests"], ["visuals"]),
    "info": (["bim", "information"], []),
    "exch": (["bim", "exchange"], ["model"]),
    "cons": (["bim", "consistency"], []),
    "model": (["bim", "model"], []),
    "cast": (["grasshopper", "casting"], []),
}

OPS = [
    ("summer", "sub", "        components = VGroup(\n            wheel, wheel_tag, hx, hx_tag,\n            hum_sup, hum_exh, hum_tag, heater,\n        )\n\n", ""),
    ("mech", "sub", "        chips = VGroup(heat_chip, cool_chip)", "        VGroup(heat_chip, cool_chip)"),
    ("reg", "sub", '    br, tr, t = hs["bottom_right"], hs["top_right"], hs["wall"]', '    br, t = hs["bottom_right"], hs["wall"]'),
    ("reg", "sub", '    bl, tl = hs["bottom_left"], hs["top_left"]', '    bl = hs["bottom_left"]'),
    ("pe", "sub", "        legend = VGroup(legend_unit, *words.values())", "        VGroup(legend_unit, *words.values())"),
    ("prac", "del", "rad_x = [float(r.get_center()[0]) for r in rads]"),
    ("calc", "sub", "        loss_box = VGroup(loss_title, loss_desc, loss_slot)", "        VGroup(loss_title, loss_desc, loss_slot)"),
    ("calc", "del", 'floor_y = h["walls"][6].get_top()[1]'),
    ("cond", "sub", '    br, tr, t = h["bottom_right"], h["top_right"], h["wall"]', '    br, t = h["bottom_right"], h["wall"]'),
    ("cond", "sub", '        bl, br, tl, tr, roof_peak = hs["bottom_left"], hs["bottom_right"], hs["top_left"], hs["top_right"], hs["roof_peak"]', '        bl, br, tr, roof_peak = hs["bottom_left"], hs["bottom_right"], hs["top_right"], hs["roof_peak"]'),
    ("phys", "sub", "        bl, br = pot_c + np.array([-pot_w / 2, -pot_h / 2, 0.0]), pot_c + np.array([pot_w / 2, -pot_h / 2, 0.0])", "        bl = pot_c + np.array([-pot_w / 2, -pot_h / 2, 0.0])"),
    ("phys", "sub", "        shelf = VGroup(*cards)", "        VGroup(*cards)"),
    ("idx", "sub", '        s, german = self.scene, self.scene.language() == "de"', "        s = self.scene\n        self.scene.language()"),
    ("idx", "del", "cost = lambda name, rest:"),
    ("idx", "sub", "        equation = mn.VGroup(chips[0]", "        mn.VGroup(chips[0]"),
    ("idx", "del", 'free = round(loss) - round(found["heating_demand"])'),
    ("idx", "del", "flipped = [i for i in five if"),
    ("ext", "sub", "            item.clear_updaters()\n        d, german = self.fixed, self._german()", "            item.clear_updaters()\n        d = self.fixed\n        self._german()"),
    ("exp", "del", 'effects = FACTORIAL["effects"]'),
    ("exp", "sub", "        self.clear_stage()\n        german = self.language() == \"de\"\n        unit = 2.9 / YMAX", "        self.clear_stage()\n        self.language()\n        unit = 2.9 / YMAX"),
    ("otest", "sub", "        gap, head = .3, 2.7", "        gap = .3"),
    ("info", "sub", "        (xa, ya), (xb, yb) = self._xy(a), self._xy(b)\n        gap_x", "        (xa, ya), (xb, _) = self._xy(a), self._xy(b)\n        gap_x"),
    ("info", "del", 'slab_t = next(s_["thickness"]'),
    ("exch", "sub", "        analysis = f.analysis\n        plan = self.villa_plan(", "        plan = self.villa_plan("),
    ("exch", "del", "slab = f.slab"),
    ("exch", "del", "mine = by_tag[TAG]"),
    ("cons", "sub", "        span = max(self.roof_spans(), key=lambda s: s[1] - s[0])\n        pointer", "        pointer"),
    ("cons", "sub", "        span = max(self.roof_spans(), key=lambda s: s[1] - s[0])\n        q_label", "        q_label"),
    ("cons", "del", "num = lambda v: self.number(v"),
    ("cons", "del", 'entry = self.cell[tag]["zones"]["roof"]'),
    ("model", "sub", '    gx, gy = grid["x"], grid["y"]', '    gy = grid["y"]'),
    ("cast", "del", "none = _mn.VGroup()"),
]


def files_for(slug):
    inc, exc = TARGETS[slug]
    found = []
    for dp, _, names in os.walk(A):
        parts = dp.replace("\\", "/")
        last = parts.rsplit("/", 1)[-1]
        if PY in names and last.endswith(inc[-1]) and all(x in parts for x in inc) and not any(x in parts for x in exc):
            found.append(os.path.join(dp, PY).replace("\\", "/"))
    return found


def apply(path, ops):
    raw = open(path, "rb").read().decode("utf-8")
    crlf = "\r\n" in raw
    text = raw.replace("\r\n", "\n")
    for op in ops:
        if op[0] == "sub":
            old, new = op[1], op[2]
            n = text.count(old)
            if n != 1:
                print("NOMATCH", n, op[1][:60].strip(), path)
                continue
            text = text.replace(old, new)
        else:
            needle = op[1]
            lines = text.split("\n")
            hits = [i for i, ln in enumerate(lines) if ln.strip().startswith(needle)]
            if len(hits) != 1:
                print("NOMATCH", len(hits), needle[:60], path)
                continue
            del lines[hits[0]]
            text = "\n".join(lines)
    if crlf:
        text = text.replace("\n", "\r\n")
    if text == raw:
        return "same"
    tmp = path + ".f841.tmp"
    with open(tmp, "wb") as f:
        f.write(text.encode("utf-8"))
    try:
        os.replace(tmp, path)
    except PermissionError:
        os.unlink(tmp)
        return "LOCKED"
    return "changed"


def main():
    plan = {}
    for op in OPS:
        plan.setdefault(op[0], []).append(op[1:] if op[1] == "del" else op[1:])
    done = 0
    for slug, ops in plan.items():
        files = files_for(slug)
        if len(files) != 1:
            print("FILES", slug, len(files), files)
            continue
        p = files[0]
        ops2 = []
        for o in OPS:
            if o[0] == slug:
                ops2.append(("sub", o[2], o[3]) if o[1] == "sub" else ("del", o[2]))
        status = apply(p, ops2)
        print(status, slug, p)
        done += status == "changed"
    print("changed files", done)


if __name__ == "__main__":
    main()
