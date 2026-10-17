"""🧪️ Runs the IfcOpenShell oracle standalone (`check`) with the EXPRESS rule pass disabled (the system Python lacks `_pytest`, which the rule executor imports).

usage: pyharness.py <oracle 🐍️.py> <fixtures/🏗️ifc dir> [case...]
Everything else (parse, kernel tessellation, every audit, the table comparison) runs unchanged, so the output is comparable before and after a split.
"""
import runpy, sys
import ifcopenshell.validate as v

real = v.validate
v.validate = lambda model, log, express_rules=False, **kw: real(model, log, express_rules=False, **kw)
oracle, fixtures, *cases = sys.argv[1:]
sys.argv = [oracle, "check", fixtures, *cases]
runpy.run_path(oracle, run_name="__main__")
