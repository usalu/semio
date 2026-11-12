"""One-shot patch of the tower clip to declarative transitions (ticket-local input)."""
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
FILE = ROOT / "🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🗼️tower/🐍️.py"
text = FILE.read_text(encoding="utf-8")


def sub(old, new, count=1):
    global text
    assert text.count(old) == count, (text.count(old), old[:110])
    text = text.replace(old, new)


def cut(start, end, new=""):
    global text
    a = text.index(start)
    b = text.index(end, a)
    text = text[:a] + new + text[b:]


sub('''def _tower_display(drawing, camera, *overlays):''', '''TRANSITIONS = Path(__file__).resolve().parent / "🔀️transitions" / "🔣️.json"


def _tower_display(drawing, camera, *overlays):''')
sub('''                    moves.append(_morph.retext(previous, value, run_time=1.2))''', '''                    moves.append(move("note-retext", previous, value))''')
cut("        def shapes(drawing, pick=None):", "        def adopt(*groups):", '''        def parts(drawing, pick=None):
            """🧩 The capsule groups of a drawing by part name."""
            return {"-".join(str(part) for part in key): group for key, group in sorted(drawing.capsules.items()) if pick is None or pick(key)}

        def named(groups):
            """🏷️ Part mapping of a drawing's groups (cores, capitals) by their index."""
            return {str(key): group for key, group in groups.items()}

        def landed(name):
            """🛬 The exact target a transition leaves in the scene."""
            return self.stage.mobject(f"tower.{name}.target")

''')
sub('''        def stage(key, network=None, drawing=None, note=None, seconds=2, via=None, done=None):''', '''        def move(name, source, target):
            """🔀 The declared transition ``tower.<name>``; the active profile decides how it plays."""
            return self.stage.transition(f"tower.{name}", source=source, target=target)

        def stage(key, network=None, drawing=None, note=None, seconds=2, via=None, done=None):''')
sub('''        self._defn, self._cat, self._cache = _gh.canvas("tower"), _gh.catalogue(), {}''', '''        self.stage = _morph.stage(self, TRANSITIONS)
        self._defn, self._cat, self._cache = _gh.canvas("tower"), _gh.catalogue(), {}''')

sub('''_morph.gather(sils(full), count_token, run_time=1.6)''', '''move("count-gather", parts(full), count_token)''')
sub('''        part_sources = {"core": [_morph.silhouette(part) for part in full.cores.values()], "capsules": sils(full),
                        "capital": [_morph.silhouette(part) for part in full.capitals.values()], "podium": [_morph.silhouette(full.podium)]}
        self.add(*[shape for shapes in part_sources.values() for shape in shapes if shape not in self.mobjects])
        gathers = [_morph.gather(shapes, badges[key], run_time=1.4) for key, shapes in part_sources.items()]''',
    '''        part_sources = {"core": named(full.cores), "capsules": parts(full), "capital": named(full.capitals), "podium": {"podium": full.podium}}
        gathers = [move(f"parts-{key}", group, badges[key]) for key, group in part_sources.items()]''')
sub('''_morph.morph(_morph.token("", kind="plane", color=COLORS["planes"], size=1.4).move_to(full.get_center()), badges["planes"], run_time=1.4)''', '''move("parts-planes", _morph.token("", kind="plane", color=COLORS["planes"], size=1.4).move_to(full.get_center()), badges["planes"])''')
sub('''        core_source = _morph.silhouette(list(full.cores.values())[-1])
        del full
        self.add(core_source)''', '''        core_source = list(full.cores.values())[-1]
        del full''')
sub('''_morph.morph(core_source, core_live, run_time=1.8)''', '''move("core-envelope", core_source, core_live)''')
sub('''_morph.morph(core_live, shaft, run_time=1.8)''', '''move("shaft-assembly", core_live, shaft)''')
sub('''_morph.morph(shaft, levels_core, run_time=1.8)''', '''move("series-core", shaft, levels_core)''')
sub('''_morph.split(level_source, list(frames[1]), run_time=1.4)''', '''move("facades-axes", level_source, list(frames[1]))''')
sub('''_morph.morph(frame_source, axes, run_time=1.4)''', '''move("axes-single", frame_source, axes)''')
sub('''_morph.split(port_source, port_tokens, run_time=1.4)''', '''move("items-ports", port_source, port_tokens)''')
sub('''        first_storey = shapes(ordered, lambda key: key[2] == 0)''', '''        first_storey = parts(ordered, lambda key: key[2] == 0)''')
sub('''_morph.scatter(port_tokens, first_storey, run_time=1.4)]}, done=lambda: settle(*first_storey))''', '''move("orient-ports", port_tokens, first_storey)]}, done=lambda: settle(landed("orient-ports")))''')
sub('''        neutral, coloured = sils(ordered), shapes(typed)''', '''        neutral, coloured = parts(ordered), parts(typed)''')
sub('''_morph.scatter(neutral, coloured, lag=.01, path_arc=0, run_time=1.4)]}, done=lambda: settle(*coloured))''', '''move("types-colour", neutral, coloured)]}, done=lambda: settle(landed("types-colour")))''')
sub('''_morph.gather(sils(typed), growing_token, run_time=1.6)''', '''move("count-total", parts(typed), growing_token)''')
sub('''_morph.morph(half_tag, half_end, path_arc=.5, run_time=1.6)''', '''move("nowire-half", half_tag, half_end)''')
sub('''_morph.scatter(candidate_tokens, phantoms, lag=.2, path_arc=.4, run_time=1.4)''', '''move("mistake-candidates", candidate_tokens, phantoms)''')
sub('''_morph.scatter([_morph.silhouette(phantom) for phantom in phantoms], cull_tokens, lag=.2, path_arc=.4, run_time=1.4)''', '''move("fix-cull", phantoms, cull_tokens)''')
sub('''        cap_source = _morph.silhouette(list(fixed.capitals.values())[-1])''', '''        cap_source = list(fixed.capitals.values())[-1]''')
sub('''_morph.morph(cap_source, cap_live, run_time=1.6), *[_morph.morph(start, end, path_arc=.5, run_time=1.6) for start, end in cap_tags]''',
    '''move("capital-core", cap_source, cap_live), *[move(f"capital-{name}-tag", start, end) for name, (start, end) in zip(("height", "overhang"), cap_tags)]''')
sub('''_morph.morph(cap_live, rising_head, run_time=1.6)''', '''move("ontop-head", cap_live, rising_head)''')
sub('''_morph.morph(storey_tag, storey_end, path_arc=.5, run_time=1.6)''', '''move("ontop-storey", storey_tag, storey_end)''')
sub('''_morph.morph(footprint, podium, run_time=1.6)''', '''move("podium-footprint", footprint, podium)''')
sub('''_morph.morph(podium, whole_base, run_time=1.6)''', '''move("why-base", podium, whole_base)''')
sub('''_morph.gather(sils(tinted, lambda k: k[1] == 0), number_a, run_time=1.6), _morph.gather(sils(tinted, lambda k: k[1] == 1), number_b, run_time=1.6)''',
    '''move("towers-a", parts(tinted, lambda k: k[1] == 0), number_a), move("towers-b", parts(tinted, lambda k: k[1] == 1), number_b)''')
sub('''        lit_a, lit_b = shapes(plain, lambda k: k[1] == 0), shapes(plain, lambda k: k[1] == 1)''', '''        lit_a, lit_b = parts(plain, lambda k: k[1] == 0), parts(plain, lambda k: k[1] == 1)''')
sub('''_morph.split(number_a, lit_a, run_time=1.6), _morph.split(number_b, lit_b, run_time=1.6)''', '''move("total-a", number_a, lit_a), move("total-b", number_b, lit_b)''')
sub('''done=lambda: (settle(*lit_a, *lit_b), _retire(self, towers_group)))''', '''done=lambda: (settle(landed("total-a"), landed("total-b")), _retire(self, towers_group)))''')
sub('''        old_source = _morph.silhouette(plain.capsules[pick(old_key)])''', '''        old_source = plain.capsules[pick(old_key)]''')
sub('''_morph.morph(old_source, programs, run_time=1.8)''', '''move("swap-program", old_source, programs)''')
sub('''        recap_parts = {"planes": [_morph.token("", kind="plane", color=COLORS["planes"], size=1.4).move_to(final.get_center())], "core": [_morph.silhouette(part) for part in final.cores.values()], "capsules": shapes(final),
                       "capital": [_morph.silhouette(part) for part in final.capitals.values()], "podium": [_morph.silhouette(final.podium)]}''',
    '''        recap_parts = {"planes": [_morph.token("", kind="plane", color=COLORS["planes"], size=1.4).move_to(final.get_center())], "core": named(final.cores), "capsules": parts(final),
                       "capital": named(final.capitals), "podium": {"podium": final.podium}}''')
sub('''        splits = [_morph.split(_morph.silhouette(badge), recap_parts[key], run_time=1.6) for key, badge in zip(GROUP_KEYS, cards_end) if key in recap_parts]''',
    '''        splits = [move(f"recap-{key}", badge.copy(), recap_parts[key]) for key, badge in zip(GROUP_KEYS, cards_end) if key in recap_parts]''')
sub('''done=lambda: settle(*[shape for shapes_ in recap_parts.values() for shape in shapes_]))''', '''done=lambda: settle(*[landed(f"recap-{key}") for key in recap_parts]))''')
FILE.write_text(text, encoding="utf-8")
print("patched")
