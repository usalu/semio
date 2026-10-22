"""🎟 Independent Python oracle of the Rust compiler capacity request (standard library only): a profile with the default backend, more than one codegen unit and no link time optimisation is bounded by its codegen units; every other profile reserves the whole host. Requests arrive as JSON on stdin."""
import json, sys


def valid(profile):
    if not {"codegen_backend", "codegen_units", "lto"} <= set(profile):
        return False
    backend, units, lto = profile["codegen_backend"], profile["codegen_units"], profile["lto"]
    if not (backend is None or (isinstance(backend, str) and len(backend) > 0)):
        return False
    if not (units is None or (isinstance(units, int) and not isinstance(units, bool) and 1 <= units <= 2 ** 53 - 1)):
        return False
    return lto in ("false", "off", "thin", "fat")


def decide(profile):
    if not valid(profile):
        return {"refused": True}
    bounded = profile["codegen_backend"] is None and profile["codegen_units"] is not None and profile["codegen_units"] > 1 and profile["lto"] in ("false", "off")
    return {"resourceClass": "jobserver-codegen" if bounded else "whole-host-backend", "request": {"kind": "available", "maximumParallelism": profile["codegen_units"]} if bounded else {"kind": "whole-host"}}


print(json.dumps([decide(p) for p in json.load(sys.stdin)["profiles"]]))
