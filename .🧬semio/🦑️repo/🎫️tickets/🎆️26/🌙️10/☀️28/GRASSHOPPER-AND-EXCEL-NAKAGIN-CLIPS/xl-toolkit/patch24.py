import sys
p = sys.argv[1]
s = open(p, encoding="utf-8").read()
s = s.replace('''    def page_trace(self, book):''', '''    def page_long(self, book):
        formula = '=SUMPRODUCT((Program!$D$2:$D$400=B5)*(Program!$E$2:$E$400="A")*Program!$F$2:$F$400)+COUNTIFS(Program!$A$2:$A$400,B6,Program!$B$2:$B$400,"Tower 1")'
        window = self.window(book, selected="B9", editing=formula, ribbon="Formulas")
        self.add(window)

    def page_trace(self, book):''', 1)
s = s.replace('[edit|trace|error|formulas|pop|chart|recalc ...]', '[edit|long|trace|error|formulas|pop|chart|recalc ...]').replace('["edit", "trace"', '["edit", "long", "trace"')
open(p, "w", encoding="utf-8").write(s)
