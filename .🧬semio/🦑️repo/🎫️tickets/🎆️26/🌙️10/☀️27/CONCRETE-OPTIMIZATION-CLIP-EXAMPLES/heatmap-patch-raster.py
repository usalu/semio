"""[DEBUG] One-off patch: replace the vector heat map body by a raster one in room/🐍️.py."""
import sys
from pathlib import Path
sys.stdout.reconfigure(encoding="utf-8")
ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
path = ROOT / "🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🎨️visuals/🏠️room/🐍️.py"
source = path.read_text(encoding="utf-8")
start = source.index("    def heat_map(")
stop = source.index("        contours = mn.VGroup()", start)
new = '''    def heat_map(self, axes, surface=None, *, colour=None, opacity=0.6, levels=0, labels=True, star=True, legend=None, legend_width=3.5, x_scale=100.0, columns=True):
        """🗺️ Electricity over WWR × overhang in ``axes`` (x in % when ``x_scale`` is 100) as a bilinearly smoothed raster registered to whole pixels (``image`` is a group of one ``ImageMobject`` strip per WWR column, or with ``columns=False`` a single one), with contours, optimum star and a kWh/a strip."""
        mn, german = _engine, self._german()
        surface = surface or _model.surface(41)
        colour = colour or ramp()
        xs, ys, grid = [x * x_scale for x in surface["wwr"]], surface["overhang"], surface["electricity"]
        floor, ceiling = colour.low, colour.high
        camera = self.camera
        to_x = lambda x: (axes.c2p(x, ys[0])[0] - camera.frame_center[0]) * camera.pixel_width / camera.frame_width + camera.pixel_width / 2
        to_y = lambda y: camera.pixel_height / 2 - (axes.c2p(xs[0], y)[1] - camera.frame_center[1]) * camera.pixel_height / camera.frame_height
        edges = [xs[0], *[(a + b) / 2 for a, b in zip(xs, xs[1:])], xs[-1]]
        cuts = [round(to_x(x)) for x in edges]
        top, bottom = round(to_y(ys[-1])), round(to_y(ys[0]))
        width, height = max(cuts[-1] - cuts[0], 1), max(bottom - top, 1)
        ix = np.interp(xs[0] + (np.arange(width) + .5) / width * (xs[-1] - xs[0]), xs, np.arange(len(xs)))
        iy = np.interp(ys[-1] - (np.arange(height) + .5) / height * (ys[-1] - ys[0]), ys, np.arange(len(ys)))
        i0, j0 = np.minimum(ix.astype(int), len(xs) - 2), np.minimum(iy.astype(int), len(ys) - 2)
        tx, ty = (ix - i0)[None, :], (iy - j0)[:, None]
        table = np.asarray(grid, dtype=float).T
        level = (table[j0][:, i0] * (1 - tx) * (1 - ty) + table[j0][:, i0 + 1] * tx * (1 - ty) + table[j0 + 1][:, i0] * (1 - tx) * ty + table[j0 + 1][:, i0 + 1] * tx * ty)
        lut = np.array([mn.ManimColor(colour(floor + (ceiling - floor) * k / 255)).to_rgb() for k in range(256)]) * 255
        pixels = np.empty((height, width, 4), dtype=np.uint8)
        pixels[..., :3] = np.rint(lut[np.clip(np.rint((level - floor) / (ceiling - floor) * 255), 0, 255).astype(int)])
        pixels[..., 3] = round(opacity * 255)
        slices = list(zip(edges, edges[1:], cuts, cuts[1:])) if columns else [(edges[0], edges[-1], cuts[0], cuts[-1])]
        image = mn.Group()
        for xa, xb, ca, cb in slices:
            first = min(ca - cuts[0], width - 1)
            strip = mn.ImageMobject(np.ascontiguousarray(pixels[:, first:max(cb - cuts[0], first + 1)]), resampling_algorithm=mn.RESAMPLING_ALGORITHMS["linear"])
            low, high = axes.c2p(xa, ys[0]), axes.c2p(xb, ys[-1])
            image.add(strip.stretch_to_fit_width(high[0] - low[0]).stretch_to_fit_height(high[1] - low[1]).move_to((low + high) / 2))
'''
path.write_text(source[:start] + new + source[stop:], encoding="utf-8")
print("[DEBUG] patched", len(new))
