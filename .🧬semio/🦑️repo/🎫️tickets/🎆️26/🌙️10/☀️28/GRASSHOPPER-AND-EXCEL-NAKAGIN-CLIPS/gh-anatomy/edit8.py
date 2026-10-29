import sys
p = sys.argv[1]
s = open(p, encoding="utf-8").read()
def rep(a, b):
    global s
    assert a in s, a
    s = s.replace(a, b, 1)
rep('c.param("fl", "Number", "Width", [{"nakagin": "capsule.interior.width"}])', 'c.param("fl", "Number", "Width")')
rep('"fl.Number>flp.Stream",\n', '"fl.Number>flp.Stream", "w.Number>fl.Number",\n')
rep('c.view("param", "A floating parameter", "Ein freier Parameter", [("fl", 0, 0), ("flp", 1, 0)])', 'c.view("param", "A floating parameter", "Ein freier Parameter", [("w", 0, 0), ("fl", 1, 0), ("flp", 2, 0)])')
open(p, "w", encoding="utf-8").write(s)
