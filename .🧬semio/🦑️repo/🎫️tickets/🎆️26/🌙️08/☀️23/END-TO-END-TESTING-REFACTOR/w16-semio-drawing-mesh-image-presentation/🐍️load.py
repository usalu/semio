"""🧪️ Scratch loader: imports a case's 🐍️component.py standalone, with a stub `semio_repo_test`.

Ticket 26/08/23/END-TO-END-TESTING-REFACTOR, wave 16. Lets the derivation and smoke scripts in this
ticket folder exercise the independent implementations without the test host running.
"""

import importlib.util
import hashlib
import sys
import types

REPO = "/Users/ueli/Documents/semio"


def _stub() -> None:
    if "semio_repo_test" in sys.modules:
        return
    module = types.ModuleType("semio_repo_test")

    class Outcome:
        def __init__(self, projection, raw=None, diagnostics=None):
            self.projection = projection
            self.raw = raw
            self.diagnostics = diagnostics or []

    class Context:
        pass

    class Adapter:
        def __init__(self, implementation="python"):
            self.implementation = implementation
            self.handlers = {}

        def oracle(self, scenario, handler):
            self.handlers[scenario + "::oracle"] = handler
            return self

        def subject(self, scenario, handler):
            self.handlers[scenario + "::subject"] = handler
            return self

    def digest(payload):
        return hashlib.sha256(payload or b"").hexdigest()[:32]

    module.Outcome = Outcome
    module.Context = Context
    module.Adapter = Adapter
    module.digest = digest
    sys.modules["semio_repo_test"] = module


def load(case: str):
    """📦️ Loads `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧪️tests/<case>/🐍️component.py`."""
    _stub()
    path = "%s/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧪️tests/%s/🐍️component.py" % (REPO, case)
    spec = importlib.util.spec_from_file_location("semio_case_" + case.replace("-", "_"), path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module
