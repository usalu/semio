"""[DEBUG] One-off edit: flat heat map as one multi-path shape per colour."""
from pathlib import Path

room = Path("C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🎨️visuals/🏠️room/🐍️.py")
source = room.read_text(encoding="utf-8")
start = source.index("            open_runs = {}")
end = source.index("        contours = mn.VGroup()")
new = '''            open_runs, shapes = {}, {}

            def close(key, xa, xb):
                shape = shapes.setdefault(key[2], mn.VMobject(fill_color=palette[key[2]], fill_opacity=opacity, stroke_width=0))
                low, high = axes.c2p(xa, key[0]), axes.c2p(xb, key[1])
                corners = [low, np.array([high[0], low[1], 0.0]), high, np.array([low[0], high[1], 0.0]), low]
                shape.start_new_path(corners[0])
                for corner in corners[1:]:
                    shape.add_line_to(corner)

            for i, (xa, xb) in enumerate(spans_x + [(None, None)]):
                current = set(runs(index[i])) if xa is not None else set()
                for key in [key for key in open_runs if key not in current]:
                    close(key, open_runs.pop(key), spans_x[i - 1][1])
                for key in current:
                    open_runs.setdefault(key, xa)
            image.add(*[shapes[level] for level in sorted(shapes)])
'''
source = source[:start] + new + source[end:]
room.write_text(source, encoding="utf-8")
