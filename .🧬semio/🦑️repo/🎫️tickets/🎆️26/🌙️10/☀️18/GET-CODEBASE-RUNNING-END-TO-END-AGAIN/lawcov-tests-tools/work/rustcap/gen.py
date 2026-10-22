"""📝 Authoring aid: writes the Rust compiler capacity corpus (corpus.json)."""
import json, os

W = os.path.dirname(os.path.abspath(__file__)) + "/"
BOUNDED, EXCLUSIVE = "jobserver-codegen", "whole-host-backend"
rows = []
for backend in [None, "cranelift", "gcc"]:
    for units in [None, 1, 2, 3, 16, 256]:
        for lto in ["false", "off", "thin", "fat"]:
            bounded = backend is None and units is not None and units > 1 and lto in ("false", "off")
            rows.append({"profile": {"codegen_backend": backend, "codegen_units": units, "lto": lto}, "resourceClass": BOUNDED if bounded else EXCLUSIVE, "request": {"kind": "available", "maximumParallelism": units} if bounded else {"kind": "whole-host"}})
refused = [
    ("zero codegen units", {"codegen_backend": None, "codegen_units": 0, "lto": "off"}),
    ("negative codegen units", {"codegen_backend": None, "codegen_units": -1, "lto": "off"}),
    ("fractional codegen units", {"codegen_backend": None, "codegen_units": 1.5, "lto": "off"}),
    ("textual codegen units", {"codegen_backend": None, "codegen_units": "2", "lto": "off"}),
    ("codegen units above the safe integer range", {"codegen_backend": None, "codegen_units": 9007199254740992, "lto": "off"}),
    ("an unknown lto mode", {"codegen_backend": None, "codegen_units": 2, "lto": "yes"}),
    ("a boolean lto", {"codegen_backend": None, "codegen_units": 2, "lto": False}),
    ("an empty backend name", {"codegen_backend": "", "codegen_units": 2, "lto": "off"}),
    ("a missing backend", {"codegen_units": 2, "lto": "off"}),
    ("a missing lto", {"codegen_backend": None, "codegen_units": 2}),
    ("a numeric backend", {"codegen_backend": 1, "codegen_units": 2, "lto": "off"}),
]
exactly = [{"name": "extra resolved fields are ignored", "profile": {"codegen_backend": None, "codegen_units": 2, "lto": "off", "opt_level": 3, "debug": False}, "resourceClass": BOUNDED, "request": {"kind": "available", "maximumParallelism": 2}}, {"name": "the largest safe integer of codegen units is funded as is", "profile": {"codegen_backend": None, "codegen_units": 9007199254740991, "lto": "off"}, "resourceClass": BOUNDED, "request": {"kind": "available", "maximumParallelism": 9007199254740991}}]
json.dump({"version": 1, "classes": {"bounded": BOUNDED, "exclusive": EXCLUSIVE}, "profiles": rows + [{**e} for e in exactly], "refused": [{"name": n, "profile": p} for n, p in refused]}, open(W + "corpus.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)
print(len(rows) + 1, len(refused))
