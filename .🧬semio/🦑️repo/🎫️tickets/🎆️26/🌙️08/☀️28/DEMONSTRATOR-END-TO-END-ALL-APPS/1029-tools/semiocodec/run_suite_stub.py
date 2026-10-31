"""Stub of the repo test host (semio_repo_test) for local runs of the mutate-semio-* adapters."""
import hashlib
import importlib.util
import json
import pathlib
import sys
import types
import traceback


class Outcome:
    def __init__(self, projection, raw=None, diagnostics=None, artifacts=None, production_dispatch=None):
        self.projection = projection
        self.raw = raw


class Adapter:
    def __init__(self, implementation="python"):
        self._handlers = {}

    def oracle(self, scenario, handler):
        self._handlers[scenario + "::oracle"] = handler
        return self

    def subject(self, scenario, handler):
        self._handlers[scenario + "::subject"] = handler
        return self


class Context:
    def __init__(self, subset: pathlib.Path, name: str, steps=()):
        self.subset = subset
        self.scenario = {"id": name, "steps": [{"text": line} for line in steps]}

    def step_input_uris(self):
        import re
        return [m.group(0) for step in self.scenario["steps"] for m in re.finditer(r"\b(shared|local|asset|schema)://([^\s\"'`,;)\]]+)", step["text"])]

    def input_bytes(self, uri: str) -> bytes:
        scheme, _, rest = uri.partition("://")
        if scheme == "asset":
            path = self.subset / "🖼️assets" / rest
        elif scheme == "shared":
            path = self.subset / "🧪️tests" / "🧫️fixtures" / rest
        else:
            raise KeyError(uri)
        return path.read_bytes()


def digest(payload):
    return hashlib.sha256(payload if payload is not None else b"").hexdigest()[:32]


module = types.ModuleType("semio_repo_test")
module.Adapter, module.Context, module.Outcome, module.digest = Adapter, Context, Outcome, digest
sys.modules["semio_repo_test"] = module

