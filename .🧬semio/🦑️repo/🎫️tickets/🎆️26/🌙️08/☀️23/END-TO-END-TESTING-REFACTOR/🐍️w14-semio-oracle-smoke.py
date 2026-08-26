#!/usr/bin/env python3
"""🧪️ Scratch driver for the wave-14 cross-language semio oracles. Loads one case's
`🐍️component.py` through a stand-in of the platform's Python host and runs every registered
handler against the real committed assets, so the implementation is checked before the runner is.
"""

import json, os, sys, types, hashlib, traceback

ROOT = "/Users/ueli/Documents/semio"
ART = ROOT + "/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio"


class Outcome:
    def __init__(self, projection, raw=None, diagnostics=None):
        self.projection, self.raw, self.diagnostics = projection, raw, diagnostics or []


class Adapter:
    def __init__(self, implementation="python"):
        self.implementation = implementation
        self._h = {}

    def oracle(self, scenario, handler):
        self._h[scenario + "::oracle"] = handler
        return self

    def subject(self, scenario, handler):
        self._h[scenario + "::subject"] = handler
        return self

    def handler(self, scenario, role):
        return self._h.get(scenario + "::" + role)


class Context:
    def __init__(self, case_dir, scenario):
        self.case_dir, self.scenario, self.role = case_dir, scenario, "oracle"

    def _resolve(self, uri):
        if uri.startswith("asset://"):
            return os.path.join(ART, uri[len("asset://"):])
        if uri.startswith("local://"):
            return os.path.join(self.case_dir, "🧫️fixtures", uri[len("local://"):])
        raise KeyError(uri)

    def fixture(self, uri):
        return self._resolve(uri)

    def fixture_bytes(self, uri):
        with open(self._resolve(uri), "rb") as h:
            return h.read()


def digest(payload):
    return hashlib.sha256(payload or b"").hexdigest()[:32]


host = types.ModuleType("semio_repo_test")
host.Adapter, host.Context, host.Outcome, host.digest = Adapter, Context, Outcome, digest
sys.modules["semio_repo_test"] = host


def load(case):
    import importlib.util
    path = os.path.join(ART, "🧪️tests", case, "🐍️component.py")
    spec = importlib.util.spec_from_file_location("semio_test_adapter_" + case.replace("-", "_"), path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def scenario(sid, steps):
    return {"id": sid, "name": sid, "level": "exhaustive", "mode": "differential", "seed": "", "steps": steps}


def step(text, doc=None):
    out = {"keyword": "Given", "text": text}
    if doc is not None:
        out["docString"] = doc
    return out


def run(case, plans):
    module = load(case)
    ad = module.adapter()
    case_dir = os.path.join(ART, "🧪️tests", case)
    ok = 0
    for sid, steps in plans:
        h = ad.handler(sid, "oracle")
        if h is None:
            print("  MISSING  %s" % sid)
            continue
        try:
            out = h(Context(case_dir, scenario(sid, steps)))
            ok += 1
            print("  ok       %s  %s" % (sid, json.dumps(out.projection, ensure_ascii=False)[:110]))
        except Exception:
            print("  FAILED   %s" % sid)
            print("     " + traceback.format_exc().replace("\n", "\n     ")[:2000])
    print("%s: %d/%d" % (case, ok, len(plans)))
    return ok == len(plans)
