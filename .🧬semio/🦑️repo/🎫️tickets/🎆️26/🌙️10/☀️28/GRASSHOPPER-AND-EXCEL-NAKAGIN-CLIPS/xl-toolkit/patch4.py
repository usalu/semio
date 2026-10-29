import sys
path = sys.argv[1]
src = open(path, encoding="utf-8").read()
a = src.index("    def page_trace")
b = src.index("    def page_formulas")
new = '''    def page_trace(self, view, sheet):
        window = self.window(view, sheet, selected="B5")
        arrows = [self.xl_trace(window, "B2", "B5"), self.xl_trace(window, "B3", "B5"), self.xl_trace(window, "B5", "B6", kind="dependent"), self.xl_trace(window, "B4", "B6"),
                  self.xl_trace(window, f"'{view.sheets[0]}'!C2", "B2"), self.xl_trace(window, "B9:B10", "B11")]
        self.add(window, *arrows, self.xl_fill_handle(window, "B5", to="B7"))

    def page_error(self, view, sheet):
        window = self.window(view, sheet, selected="B11")
        self.add(window, self.xl_error(window, "B11", "#DIV/0!"))

'''
src = src[:a] + new + src[b:]
src = src.replace('selected="B8:B9", show="formulas"', 'selected="B7:B8", show="formulas"')
open(path, "w", encoding="utf-8").write(src)
