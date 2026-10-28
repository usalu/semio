"""[DEBUG] One-off edit: cheaper heat map (quantized colours, merged runs)."""
from pathlib import Path

room = Path("C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🎨️visuals/🏠️room/🐍️.py")
source = room.read_text(encoding="utf-8")
start = source.index("        spans_x = [(max(x - step_x / 2")
end = source.index("        contours = mn.VGroup()")
new = '''        spans_x = [(max(x - step_x / 2, xs[0]), min(x + step_x / 2, xs[-1])) for x in xs]
        spans_y = [(max(y - step_y / 2, ys[0]), min(y + step_y / 2, ys[-1])) for y in ys]
        floor, ceiling = colour.low, colour.high
        index = [[min(steps - 1, max(0, round((value - floor) / (ceiling - floor) * (steps - 1)))) for value in column_] for column_ in grid]
        palette = [colour(floor + (ceiling - floor) * k / (steps - 1)) for k in range(steps)]

        def block(xa, xb, ya, yb, level):
            low, high = axes.c2p(xa, ya), axes.c2p(xb, yb)
            shape = mn.VMobject(fill_color=palette[level], fill_opacity=opacity, stroke_width=0)
            shape.set_points_as_corners([low, np.array([high[0], low[1], 0.0]), high, np.array([low[0], high[1], 0.0]), low])
            return shape

        def runs(column_):
            found, first = [], 0
            for j in range(1, len(column_) + 1):
                if j == len(column_) or column_[j] != column_[first]:
                    found.append((spans_y[first][0], spans_y[j - 1][1], column_[first]))
                    first = j
            return found

        image = mn.VGroup()
        if columns:
            for i, (xa, xb) in enumerate(spans_x):
                image.add(mn.VGroup(*[block(xa, xb, ya, yb, level) for ya, yb, level in runs(index[i])]))
        else:
            open_runs = {}
            for i, (xa, xb) in enumerate(spans_x + [(None, None)]):
                current = {(ya, yb, level) for ya, yb, level in runs(index[i])} if xa is not None else set()
                for key in [key for key in open_runs if key not in current]:
                    image.add(block(open_runs.pop(key), spans_x[i - 1][1], key[0], key[1], key[2]))
                for key in current:
                    open_runs.setdefault(key, xa)
'''
source = source[:start] + new + source[end:]
source = source.replace("legend=None, legend_width=3.5, x_scale=100.0):\n        \"\"\"🗺️", "legend=None, legend_width=3.5, x_scale=100.0, steps=24, columns=True):\n        \"\"\"🗺️")
source = source.replace("Electricity over WWR × overhang as coloured cells in ``axes`` (x in % when ``x_scale`` is 100; ``image`` holds one group per WWR column)",
                        "Electricity over WWR × overhang in ``axes`` (x in % when ``x_scale`` is 100) as merged runs of ``steps`` quantized colours (``image`` holds one group per WWR column, or with ``columns=False`` one flat group of the fewest rectangles)")
room.write_text(source, encoding="utf-8")
