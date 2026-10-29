import sys
p = sys.argv[1]
s = open(p, encoding="utf-8").read()
s = s.replace('\n        self.remove(first)', '', 1)
s = s.replace('self.step("panel", _engine.FadeOut(error),', 'self.step("panel", _engine.FadeOut(error), _engine.FadeOut(first),', 1)
open(p, "w", encoding="utf-8").write(s)
