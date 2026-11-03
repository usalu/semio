import importlib, sys, unittest
from manim import VMobject
M = "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧍️staffage"
S = importlib.import_module(M + ".🐍️")
t = importlib.import_module(M + ".🧪️tests.🐍️")
S._BUNDLED = ()
S._Path.set_fill = VMobject.set_fill
suite = unittest.TestSuite([t.Cache("test_bundled_inputs_outside_the_sources_change_the_key"), t.Motion("test_set_opacity_never_gives_an_unfilled_path_a_fill")])
result = unittest.TextTestRunner(verbosity=0).run(suite)
print("failures", len(result.failures), "errors", len(result.errors))
