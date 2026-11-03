import importlib
import sys
import unittest

M = "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧍️staffage"
S = importlib.import_module(M + ".🐍️")
t = importlib.import_module(M + ".🧪️tests.🐍️")


def before_fix(source, target):
    try:
        source.rename(target)
    except OSError:
        if not (target / S._INDEX).is_file():
            raise


S._publish = before_fix
suite = unittest.TestSuite([t.Cache("test_a_transient_lock_on_the_rendered_folder_does_not_lose_the_batch")])
result = unittest.TextTestRunner(stream=sys.stdout, verbosity=0).run(suite)
print("fails without the fix:", not result.wasSuccessful())
