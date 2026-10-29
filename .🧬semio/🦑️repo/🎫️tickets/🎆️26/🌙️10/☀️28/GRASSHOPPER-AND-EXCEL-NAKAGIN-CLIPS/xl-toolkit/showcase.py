"""🖼️ Render the Excel workbook toolkit as 1280x720 stills (en, de) into 🗑️generated/xl-toolkit and run the strict layout checker on each.

Usage: python showcase.py [edit|long|trace|error|formulas|pop|chart|recalc ...]
The Type A sheet comes from the real Nakagin workbook of the XL engine.
"""
import importlib
import os
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
TICKET = HERE.parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
OUT = TICKET / "🗑️generated/xl-toolkit"
sys.path.insert(0, str(ROOT))
sys.path.insert(0, str(TICKET))
manim = importlib.import_module("manim")
visuals = importlib.import_module("🎓️teaching.🏛️architecture.📊️excel.🎬️clip.🎨️visuals.🐍️")
model = importlib.import_module("🎓️teaching.🏛️architecture.📊️excel.🎬️clip.🧱️model.🐍️")
strict = importlib.import_module("layout_strict")

PAGES = sys.argv[1:] or ["edit", "long", "trace", "error", "formulas", "pop", "chart", "recalc"]
SHEET = "Type A"
OUTPUT = {"page": ""}


class Showcase(visuals.ClipScene):
    """🖼️ One still: Excel window with the feature of the page and a caption."""

    NARRATION = [["still", "Workbook toolkit showcase.", "Arbeitsmappen-Werkzeugkasten."]]

    def construct(self):
        visuals._fonts.apply_scene_style(self)
        page, language = OUTPUT["page"], self.language()
        getattr(self, "page_" + page)(model.workbook())
        caption = visuals._art.caption_bar(self.tr("Cell B9 multiplies the interior width B5 by the interior length B6.", "Zelle B9 multipliziert die Innenbreite B5 mit der Innenlänge B6."))
        self.add(caption)
        self.renderer.static_image = None
        self.renderer.update_frame(self, mobjects=self.mobjects, ignore_skipping=True)
        self.renderer.get_frame()
        self.renderer.camera.get_image().save(OUT / f"{page}-{language}.png")
        findings = strict.strict_conflicts(self)
        conflicts = visuals._art.layout_conflicts(self)
        (OUT / f"layout-{page}-{language}.md").write_text(f"# {page} {language}\n\n- shared: {conflicts or 'none'}\n" + strict.strict_section(findings), encoding="utf-8")
        print(f"[DEBUG] {page} {language}: strict {len(findings)}, shared {len(conflicts)}", flush=True)

    def window(self, book, **kwargs):
        kwargs.setdefault("cols", "A:F")
        kwargs.setdefault("rows", (1, 10))
        return self.xl_window(book, SHEET, at=(0, 0, 0), width=12.4, height=5.1, **kwargs)

    def page_edit(self, book):
        window = self.window(book, selected="B9", editing="=B5*B6", ribbon="Formulas")
        self.add(window)

    def page_long(self, book):
        formula = '=SUMPRODUCT((Program!$D$2:$D$400=B5)*(Program!$E$2:$E$400="A")*Program!$F$2:$F$400)+COUNTIFS(Program!$A$2:$A$400,B6,Program!$B$2:$B$400,"Tower 1")'
        window = self.window(book, selected="B9", editing=formula, ribbon="Formulas")
        self.add(window)

    def page_trace(self, book):
        window = self.window(book, selected="B9")
        arrows = [self.xl_trace(window, "B5", "B9"), self.xl_trace(window, "B6", "B9"), self.xl_trace(window, "B9", "B10", kind="dependent"), self.xl_trace(window, "B7", "B10"),
                  self.xl_trace(window, "'Program'!C2", "B2"), self.xl_trace(window, "D3:D4", "D11")]
        self.add(window, *arrows, self.xl_fill_handle(window, "B9", to="B10"))

    def page_error(self, book):
        book = model.apply(book, [{"type": "set-cell", "sheet": SHEET, "cell": "B6", "value": 0}])
        window = self.window(book, selected="B15", rows=(6, 16))
        self.add(window, self.xl_error(window, "B15", "#DIV/0!"))

    def page_formulas(self, book):
        window = self.window(book, selected="B9:B10", show="formulas", ribbon="Formulas", cols="A:D")
        self.add(window)

    def page_pop(self, book):
        window = self.xl_window(book, SHEET, at=(-3.2, 0, 0), width=7.2, height=5.1, cols="A:B", rows=(1, 10), selected="B5")
        self.add(window, self.xl_cell_pop(window, "B9"))

    def page_chart(self, book):
        values = model.evaluate(book)
        rows = [(name.removeprefix("Type "), values[(name, "B14")]) for name in ("Type A", "Type B", "Type C", "Type D")]
        window = self.xl_window(book, SHEET, at=(-3.3, 0, 0), width=6.8, height=5.1, cols="A:B", rows=(1, 11), selected="B9")
        chart = self.xl_bar_chart(rows, at=(3.6, 0.1, 0), width=5.6, height=4.3, unit="kWh/a", title=self.tr("Annual heating demand", "Jahresheizwärmebedarf"))
        tabs = self.xl_tabs(book.order(), SHEET, at=(3.6, -2.45, 0), width=5.6)
        self.add(window, chart, tabs)

    def page_recalc(self, book):
        window = self.window(book, selected="B2")
        after = model.apply(book, [{"type": "set-cell", "sheet": SHEET, "cell": "B2", "value": 3.0}])
        animation = window.recalculate(after)
        animation.begin()
        animation.interpolate(1.0)
        animation.finish()
        self.add(window, self.xl_fill_handle(window, "B2"))


def render(page: str, language: str):
    os.environ["SEMIO_TEACHING_LANGUAGE"] = language
    os.environ["SEMIO_TEACHING_AUDIENCE"] = "speaker"
    os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(OUT / f"fonts-{page}-{language}")
    OUTPUT["page"] = page
    with manim.tempconfig({"pixel_width": 1280, "pixel_height": 720, "frame_rate": 1, "write_to_movie": False, "save_last_frame": False,
                           "disable_caching": True, "skip_animations": True, "media_dir": str(OUT / "media"), "verbosity": "ERROR"}):
        Showcase().render()


if __name__ == "__main__":
    OUT.mkdir(parents=True, exist_ok=True)
    for page in PAGES:
        for language in ("en", "de"):
            render(page, language)
