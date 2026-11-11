"""🧪 Compare the actual BIM clearance formula with geometric support intervals."""
import ast,json,unittest
from pathlib import Path
from jsonschema import Draft202012Validator
from shapely.geometry import LineString
ROOT=Path.cwd()
HERE=Path(__file__).parent
CASES=json.loads((HERE/"bim114-formula-cases.json").read_text(encoding="utf8"))
class Formula(unittest.TestCase):
    def test_actual_symbolic_formula_matches_clear_interval(self):
        """📏 Both support half-widths are subtracted from the axis spacing."""
        Draft202012Validator(json.loads((HERE/"bim114-formula.schema.json").read_text(encoding="utf8"))).validate(CASES)
        source=ROOT/"🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/📊️information/🐍️.py"
        tree=ast.parse(source.read_text(encoding="utf8"))
        owner=next(n for n in tree.body if isinstance(n,ast.ClassDef) and n.name=="Beat1_DataToInformation")
        method=next(n for n in owner.body if isinstance(n,ast.FunctionDef) and n.name=="_bay")
        assignment=next(n for n in ast.walk(method) if isinstance(n,ast.Assign) and any(isinstance(t,ast.Name) and t.id=="clear_row" for t in n.targets))
        mathcall=next(n for n in ast.walk(assignment.value) if isinstance(n,ast.Call) and isinstance(n.func,ast.Attribute) and n.func.attr=="_math")
        terms=[ast.literal_eval(item.elts[1]) for item in mathcall.args[0].elts]
        expression="".join(value.replace("−","-") for value in terms).split("=")[0]
        parsed=ast.parse(expression,mode="eval")
        self.assertTrue(all(isinstance(n,(ast.Expression,ast.BinOp,ast.Add,ast.Sub,ast.Name,ast.Load)) for n in ast.walk(parsed)))
        for case in CASES["cases"]:
            with self.subTest(case=case):
                actual=eval(compile(parsed,"<clear-width>","eval"),{"__builtins__":{}},{"s":case["spacing"],"r_1":case["left"],"r_2":case["right"]})
                expected=LineString([(case["left"],0),(case["spacing"]-case["right"],0)]).length
                self.assertAlmostEqual(expected,case["clear"])
                self.assertAlmostEqual(actual,expected)
if __name__=="__main__":unittest.main()
