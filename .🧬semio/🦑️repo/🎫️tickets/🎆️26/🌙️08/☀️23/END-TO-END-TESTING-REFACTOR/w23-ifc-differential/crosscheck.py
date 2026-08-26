#!/usr/bin/env python3
"""🔬️ Does the IfcOpenShell oracle's projection agree with the SUBJECT's projector?

No generated Rust subject host links today (`component::component_persistent_local` is missing while
a peer session refactors `💻️os/🔨️modules/🔌️plugin`), so `parity` cannot be measured. This probe
measures the half that CAN be measured: the subject projects through `project_ifc_4_any` /
`project_ifc_2x3_any`, the ruststep-backed projectors in the stdio oracle crate, which compiles fine
on its own. For every document IfcOpenShell actually produced in the oracle phase, this compares
that Rust projection of the same bytes against the Python oracle's own from-scratch projection,
under the `semantic-ifc-v1` profile's own tolerance and ignore list.

Agreement here means the two projections are interchangeable, so when the subject host links again
the only thing left to compare is the two CODECS — which is exactly what the differential is for.
"""
import json
import os
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *([".."] * 8)))
CROSSCHECK = os.environ.get("CROSSCHECK_BIN", "")
OUT = os.path.join(HERE, "crosscheck-work")

#: ⚖️ `semantic-ifc-v1`, as both subsets declare it.
TOLERANCE = 1e-6
IGNORE_KEYS = {"timestamp", "preprocessorVersion", "originatingSystem", "authorization", "byteLength", "fileSize"}


def first_divergence(path, expected, actual):
    if isinstance(expected, dict) and isinstance(actual, dict):
        for key in expected:
            if key in IGNORE_KEYS:
                continue
            if key not in actual:
                return "%s.%s absent on the ruststep side" % (path, key)
            found = first_divergence("%s.%s" % (path, key), expected[key], actual[key])
            if found:
                return found
        for key in actual:
            if key not in IGNORE_KEYS and key not in expected:
                return "%s.%s absent on the IfcOpenShell side" % (path, key)
        return None
    if isinstance(expected, list) and isinstance(actual, list):
        if len(expected) != len(actual):
            return "%s has %d entries against %d" % (path, len(expected), len(actual))
        for index, (left, right) in enumerate(zip(expected, actual)):
            found = first_divergence("%s[%d]" % (path, index), left, right)
            if found:
                return found
        return None
    if isinstance(expected, (int, float)) and isinstance(actual, (int, float)) and not isinstance(expected, bool) and not isinstance(actual, bool):
        return None if abs(float(expected) - float(actual)) <= TOLERANCE else "%s: %r against %r" % (path, expected, actual)
    return None if expected == actual else "%s: %r against %r" % (path, expected, actual)


def load(case):
    """📦️ One case adapter, with the same `semio_repo_test` facade the real host installs."""
    import importlib.util
    import types

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

    facade.Outcome, facade.Adapter, facade.Context, facade.digest = Outcome, Adapter, object, lambda payload: ""
    sys.modules["semio_repo_test"] = facade
    path = os.path.join(REPO, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🧪️tests", case, "🐍️component.py")
    spec = importlib.util.spec_from_file_location("adapter_" + case.replace("-", "_"), path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


CASES = {
    "differential-ifc-4": {
        "subset": "v4",
        "fixture": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🧫️fixtures/🏗️nakagin-capsule-tower.ifc",
        "specs": [
            ("no-mutation", {}),
            ("set-snapshot", {"fileSchema": ["IFC4X3"]}),
            ("set-file-description", {"values": [{"t": "aggregate", "v": [{"t": "string", "v": "wave-7 mutation"}]}, {"t": "string", "v": "2;1"}]}),
            ("set-file-name", {"values": [{"t": "string", "v": "wave-7-mutated.ifc"}, {"t": "string", "v": "2026-08-23T00:00:00"}, {"t": "aggregate", "v": [{"t": "string", "v": "Ueli"}]}, {"t": "aggregate", "v": [{"t": "string", "v": "semio"}]}, {"t": "string", "v": "semio-ifc"}, {"t": "string", "v": "semio"}, {"t": "string", "v": ""}]}),
            ("set-file-schema", {"values": [{"t": "aggregate", "v": [{"t": "string", "v": "IFC4X3"}]}]}),
            ("insert-entity", {"index": 24792, "entity": {"id": 90001, "name": "IFCCARTESIANPOINT", "args": [{"t": "aggregate", "v": [{"t": "real", "v": 1000.0}, {"t": "real", "v": 2000.0}, {"t": "real", "v": 3000.0}]}]}}),
            ("set-entity-arg", {"id": 16976, "index": 2, "value": {"t": "string", "v": "origin-marker"}}),
        ],
    },
    "differential-ifc-2x3": {
        "subset": "v2x3",
        "fixture": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🧫️fixtures/🏗️wellness-center-sama-street-level.ifc",
        "specs": [
            ("no-mutation", {}),
            ("set-snapshot", {"fileSchema": ["IFC2X3", "IFC2X3-WAVE8-SNAPSHOT-MARKER"]}),
            ("upsert-instance", {"instance": {"id": 619887, "entities": [{"name": "IFCCOLUMN", "args": [{"t": "string", "v": "0PfeWE7Aj7GBHCsLa67379"}, {"t": "reference", "v": 41}, {"t": "string", "v": "WAVE8-RENAMED-COLUMN"}, {"t": "unset"}, {"t": "string", "v": "UC-Universal Columns-Column:UC305x305x97"}, {"t": "reference", "v": 619886}, {"t": "reference", "v": 619879}, {"t": "string", "v": "552739"}]}]}}),
            ("set-header", {"header": {"fileDescription": [{"t": "aggregate", "v": [{"t": "string", "v": "ViewDefinition [CoordinationView_V2.0]"}]}, {"t": "string", "v": "2;1"}], "fileName": [{"t": "string", "v": "wellness-center-sama-street-level-wave8"}, {"t": "string", "v": "2021-11-21T06:45:25"}, {"t": "aggregate", "v": [{"t": "string", "v": ""}]}, {"t": "aggregate", "v": [{"t": "string", "v": ""}]}, {"t": "string", "v": "The EXPRESS Data Manager Version 5.02.0100.07 : 28 Aug 2013"}, {"t": "string", "v": "21.0.0.383 - Exporter 21.0.0.383 - Alternate UI 21.0.0.383"}, {"t": "string", "v": ""}], "fileSchema": [{"t": "aggregate", "v": [{"t": "string", "v": "IFC2X3"}]}]}}),
        ],
    },
}


def main() -> int:
    if not CROSSCHECK or not os.path.exists(CROSSCHECK):
        print("set CROSSCHECK_BIN to the built ruststep-crosscheck binary")
        return 2
    os.makedirs(OUT, exist_ok=True)
    ok = True
    for case, config in CASES.items():
        module = load(case)
        fixture = os.path.join(REPO, config["fixture"])
        print("== %s (%s, %d bytes)" % (case, os.path.basename(config["fixture"]), os.path.getsize(fixture)))
        for kind, params in config["specs"]:
            produced = module.apply_mutation(fixture, {"kind": kind, "params": params})
            written = os.path.join(OUT, "%s-%s.ifc" % (case, kind))
            with open(written, "wb") as handle:
                handle.write(produced)
            rust_json = os.path.join(OUT, "%s-%s.ruststep.json" % (case, kind))
            probe = subprocess.run([CROSSCHECK, config["subset"], written, rust_json], capture_output=True, text=True)
            if probe.returncode != 0:
                print("   %-22s ruststep projection FAILED: %s" % (kind, probe.stderr.strip()[:160]))
                ok = False
                continue
            with open(rust_json, "r", encoding="utf-8") as handle:
                by_ruststep = json.load(handle)
            by_ifcopenshell = module.project(produced.decode("utf-8"))
            found = first_divergence("$", by_ruststep, by_ifcopenshell)
            print("   %-22s bytes=%-9d ruststep entities=%-6s ifcopenshell entities=%-6s %s" % (kind, len(produced), by_ruststep.get("entityCount"), by_ifcopenshell.get("entityCount"), "AGREE" if found is None else "DIVERGE: " + found))
            ok &= found is None
    print("\nPROJECTIONS AGREE ON EVERY DOCUMENT" if ok else "\nAT LEAST ONE PROJECTION DIVERGED")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
