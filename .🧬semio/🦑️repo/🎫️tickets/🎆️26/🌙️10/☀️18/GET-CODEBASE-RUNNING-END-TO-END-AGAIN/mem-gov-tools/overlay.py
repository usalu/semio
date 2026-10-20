import os, shutil, sys
from paths import *
import patches

OVERLAY = sys.argv[1] if len(sys.argv) > 1 else "/private/tmp/mem-gov/r"
HERE = os.path.dirname(os.path.abspath(__file__))
STAGED = J(HERE, "staged")
VALIDATOR = J(FRAMEWORK, MODULES, E(0x1F9EC) + "schema", E(0x2705) + "validator")
CONTROL = J(FRAMEWORK, MODULES, PROCESS, E(0x1F512) + "leases", E(0x1F39B) + "control", TS)
LANGUAGE = J(FRAMEWORK, MODULES, PROCESS, BUILD, E(0x1F5E3) + "language")
EXTRA = [(J(LANGUAGE, TS), J(LANGUAGE, TS)), (J(LANGUAGE, E(0x1F523) + ".json"), J(LANGUAGE, E(0x1F523) + ".json")), (J(VALIDATOR, TS), J(VALIDATOR, TS)), (J(VALIDATOR, PACKAGES, TYPESCRIPT, TS), J(VALIDATOR, PACKAGES, TYPESCRIPT, TS)), (CONTROL, CONTROL)]
NODE = {
    "ajv": "ajv@8.20.0", "fast-deep-equal": "fast-deep-equal@3.1.3", "fast-uri": "fast-uri@3.1.8",
    "json-schema-traverse": "json-schema-traverse@1.0.0", "require-from-string": "require-from-string@2.0.2",
}

LIVE = [J(B, WORKERS, TS), J(B, WORKERS, SCHEMA, JSON_), J(B, TOOLS, BAZEL, WORKERS, TS)]
shutil.rmtree(OVERLAY, ignore_errors=True)
os.makedirs(OVERLAY)
for name, target in {**NEW_MODULE, **NEW_GLUE}.items():
    destination = J(OVERLAY, target)
    os.makedirs(os.path.dirname(destination), exist_ok=True)
    shutil.copyfile(J(STAGED, name), destination)
for source, target in EXTRA:
    destination = J(OVERLAY, target)
    os.makedirs(os.path.dirname(destination), exist_ok=True)
    shutil.copyfile(J(ROOT, source), destination)
for target in LIVE:
    destination = J(OVERLAY, target)
    os.makedirs(os.path.dirname(destination), exist_ok=True)
    shutil.copyfile(J(ROOT, target), destination)
for key in ("policy", "catalog"):
    destination = J(OVERLAY, TARGET[key])
    os.makedirs(os.path.dirname(destination), exist_ok=True)
    with open(J(ROOT, TARGET[key]), encoding="utf-8") as handle:
        text = handle.read()
        try:
            patched = patches.EDITORS[key](text)
        except patches.Drift:
            patched = text
    with open(destination, "w", encoding="utf-8") as handle:
        handle.write(patched)
for name, folder in NODE.items():
    shutil.copytree(J(ROOT, "node_modules", ".bun", folder, "node_modules", name), J(OVERLAY, "node_modules", name), symlinks=False)
for source, name in (("@types/node", "@types/node"), ("@types/bun", "@types/bun")):
    shutil.copytree(os.path.realpath(J(ROOT, "node_modules", source)), J(OVERLAY, "node_modules", name), symlinks=False)
for folder, name in (("bun-types@1.4.3", "bun-types"), ("undici-types@7.24.6", "undici-types")):
    shutil.copytree(J(ROOT, "node_modules", ".bun", folder, "node_modules", name), J(OVERLAY, "node_modules", name), symlinks=False)
open(J(OVERLAY, "package.json"), "w").write('{"name":"overlay","private":true,"type":"module"}\n')
longest = max(len(os.path.join(dp, f)) for dp, _, fs in os.walk(OVERLAY) for f in fs)
print("overlay", OVERLAY, "longest path", longest)
