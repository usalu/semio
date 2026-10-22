import os, shutil, sys
from paths import *
import patches

OVERLAY = sys.argv[1] if len(sys.argv) > 1 else "/private/tmp/cache-gc/r"
HERE = os.path.dirname(os.path.abspath(__file__))
STAGED = J(HERE, "staged")
VALIDATOR = J(FRAMEWORK, MODULES, E(0x1F9EC) + "schema", E(0x2705) + "validator")
PKG = J(E(0x1F4E6) + "packages", E(0x1F7E6) + "typescript", TS)
LEASES_SCHEMA = J(FRAMEWORK, MODULES, PROCESS, E(0x1F512) + "leases", SCHEMA, E(0x1F4DC) + "ticket.json")
NATIVE_SCHEMA = J(FRAMEWORK, MODULES, PROCESS, E(0x1F4E6) + "artifacts", E(0x1F3D7) + "native-build", STORAGE, SCHEMA, JSON_)
BUDGET_POLICY = J(L, CACHING, STORAGE, ENFORCEMENT, BUDGET, POLICYFILE)
NODE = {
    "ajv": "ajv@8.20.0", "fast-deep-equal": "fast-deep-equal@3.1.3", "fast-uri": "fast-uri@3.1.8",
    "json-schema-traverse": "json-schema-traverse@1.0.0", "require-from-string": "require-from-string@2.0.2",
}
COPY = [J(VALIDATOR, TS), J(VALIDATOR, PKG), LEASES_SCHEMA, NATIVE_SCHEMA, BUDGET_POLICY]
shutil.rmtree(OVERLAY, ignore_errors=True)
os.makedirs(OVERLAY)
for name, target in NEW.items():
    destination = J(OVERLAY, target)
    os.makedirs(os.path.dirname(destination), exist_ok=True)
    shutil.copyfile(J(STAGED, name), destination)
for target in COPY:
    destination = J(OVERLAY, target)
    os.makedirs(os.path.dirname(destination), exist_ok=True)
    shutil.copyfile(J(ROOT, target), destination)
for name, folder in NODE.items():
    shutil.copytree(J(ROOT, "node_modules", ".bun", folder, "node_modules", name), J(OVERLAY, "node_modules", name), symlinks=False)
for source, name in (("@types/node", "@types/node"), ("@types/bun", "@types/bun")):
    shutil.copytree(os.path.realpath(J(ROOT, "node_modules", source)), J(OVERLAY, "node_modules", name), symlinks=False)
for folder, name in (("bun-types@1.4.3", "bun-types"), ("undici-types@7.24.6", "undici-types")):
    shutil.copytree(J(ROOT, "node_modules", ".bun", folder, "node_modules", name), J(OVERLAY, "node_modules", name), symlinks=False)
open(J(OVERLAY, "package.json"), "w").write('{"name":"overlay","private":true,"type":"module"}\n')
print("overlay", OVERLAY)
