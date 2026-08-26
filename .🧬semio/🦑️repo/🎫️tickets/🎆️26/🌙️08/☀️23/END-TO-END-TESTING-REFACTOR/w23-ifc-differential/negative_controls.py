#!/usr/bin/env python3
"""🧪️ Negative controls for the two IfcOpenShell differential oracles.

A green oracle phase proves nothing on its own — the audit's §0 finding. These probes force each
in-role assertion to FIRE, so the reader knows the laws are load-bearing rather than decorative.
Run under the runner's own cache-local interpreter.
"""
import importlib.util
import json
import os
import sys
import types

REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", "..", "..", "..", "..", "..", ".."))


def load(case: str):
    """📦️ Loads one case adapter with the same `semio_repo_test` facade the real host installs."""
    facade = types.ModuleType("semio_repo_test")

    class Outcome:
        def __init__(self, projection, raw=None, diagnostics=None):
            self.projection, self.raw, self.diagnostics = projection, raw, diagnostics or []

    class Adapter:
        def __init__(self, implementation="python"):
            self.handlers = {}

        def oracle(self, scenario, handler):
            self.handlers[scenario] = handler
            return self

    facade.Outcome = Outcome
    facade.Adapter = Adapter
    facade.Context = object
    facade.digest = lambda payload: ""
    sys.modules["semio_repo_test"] = facade
    path = os.path.join(REPO, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🧪️tests", case, "🐍️component.py")
    spec = importlib.util.spec_from_file_location("adapter_" + case.replace("-", "_"), path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


FIXTURES = os.path.join(REPO, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🧫️fixtures")
IFC4 = os.path.join(FIXTURES, "🏗️nakagin-capsule-tower.ifc")
IFC2X3 = os.path.join(FIXTURES, "🏗️wellness-center-sama-street-level.ifc")


def expect_raises(label, fn):
    try:
        fn()
    except AssertionError as error:
        print("PASS  %-58s raised: %s" % (label, str(error)[:110]))
        return True
    except Exception as error:  # noqa: BLE001
        print("PASS  %-58s raised %s: %s" % (label, type(error).__name__, str(error)[:100]))
        return True
    print("FAIL  %-58s DID NOT RAISE — the assertion is not load-bearing" % label)
    return False


def main() -> int:
    ok = True
    m4 = load("differential-ifc-4")

    # 1️⃣ A row whose parameters make the mutation a no-op must be refused in role.
    baseline = m4.project(m4.apply_mutation(IFC4, {"kind": "no-mutation", "params": {}}).decode("utf-8"))
    noop = m4.project(m4.apply_mutation(IFC4, {"kind": "set-entity-arg", "params": {"id": 16976, "index": 2, "value": {"t": "string", "v": "b"}}}).decode("utf-8"))
    ok &= expect_raises("ifc4 observability law on a no-op set-entity-arg", lambda: m4.observable("set-entity-arg", baseline, noop))

    # 2️⃣ The real row must be accepted — the law is not simply always-raise.
    real = m4.project(m4.apply_mutation(IFC4, {"kind": "set-entity-arg", "params": {"id": 16976, "index": 2, "value": {"t": "string", "v": "origin-marker"}}}).decode("utf-8"))
    m4.observable("set-entity-arg", baseline, real)
    print("PASS  %-58s accepted the real row" % "ifc4 observability law")

    # 3️⃣ One changed argument, 16 976 entities deep in a 24 792-entity graph, must be found.
    found = m4.first_divergence("$", baseline, real)
    print(("PASS  " if found else "FAIL  ") + "%-58s %s" % ("ifc4 divergence finder on one deep argument", found))
    ok &= bool(found)

    # 4️⃣ A wrong inverse must be refused rather than silently accepted.
    mutated = m4.apply_mutation(IFC4, {"kind": "set-entity-arg", "params": {"id": 16976, "index": 2, "value": {"t": "string", "v": "origin-marker"}}})
    work = os.path.join(os.path.dirname(__file__), "negative-control-work")
    os.makedirs(work, exist_ok=True)
    intermediate = os.path.join(work, "mutated.ifc")
    with open(intermediate, "wb") as handle:
        handle.write(mutated)
    wrong = m4.project(m4.apply_mutation(intermediate, {"kind": "set-entity-arg", "params": {"id": 16976, "index": 2, "value": {"t": "string", "v": "NOT-THE-ORIGINAL"}}}).decode("utf-8"))
    ok &= expect_raises("ifc4 inverse law on a wrong inverse", lambda: (_ for _ in ()).throw(AssertionError(m4.first_divergence("$", baseline, wrong))) if m4.first_divergence("$", baseline, wrong) else None)

    # 5️⃣ The cascade guard must refuse a removal IfcOpenShell would repair.
    ok &= expect_raises("ifc4 remove-entity cascade guard on referenced #16976", lambda: m4.apply_mutation(IFC4, {"kind": "remove-entity", "params": {"id": 16976}}))

    # 6️⃣ A kind IfcOpenShell cannot produce must be an error, never a silent no-op.
    ok &= expect_raises("ifc4 unproducible kind set-entity-name", lambda: m4.apply_mutation(IFC4, {"kind": "set-entity-name", "params": {"id": 16976, "name": "RENAMED_PROXY"}}))

    # 7️⃣ A malformed §6.4.2 control directive must be an error, never a passed-through lexeme.
    ok &= expect_raises("ifc4 string decoder on \\Q", lambda: m4.decode_string_literal("\\Q"))
    ok &= expect_raises("ifc4 string decoder on \\X\\ZZ", lambda: m4.decode_string_literal("\\X\\ZZ"))
    ok &= expect_raises("ifc4 string decoder on unterminated \\X2\\", lambda: m4.decode_string_literal("\\X2\\4E2D"))
    ok &= expect_raises("ifc4 string decoder on ISO 8859-2 \\S\\", lambda: m4.decode_string_literal("\\PB\\\\S\\A"))
    for lexeme, value in [("\\\\", "\\"), ("\\X\\41", "A"), ("\\X2\\4E2D\\X0\\", "中"), ("\\S\\A", "Á")]:
        got = m4.decode_string_literal(lexeme)
        state = "PASS" if got == value else "FAIL"
        ok &= got == value
        print("%s  %-58s %r -> %r" % (state, "ifc4 string decoder", lexeme, got))

    m2 = load("differential-ifc-2x3")
    base2 = m2.project(m2.apply_mutation(IFC2X3, {"kind": "no-mutation", "params": {}}).decode("utf-8"))
    noop2 = m2.project(m2.apply_mutation(IFC2X3, {"kind": "set-header", "params": m2.ORIGINAL_HEADER}).decode("utf-8"))
    ok &= expect_raises("ifc2x3 observability law on a no-op set-header", lambda: m2.observable("set-header", base2, noop2))
    ok &= expect_raises("ifc2x3 remove-instance cascade guard on referenced #270549", lambda: m2.apply_mutation(IFC2X3, {"kind": "remove-instance", "params": {"id": 270549}}))

    print("\nALL NEGATIVE CONTROLS PASSED" if ok else "\nSOME NEGATIVE CONTROLS FAILED")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
