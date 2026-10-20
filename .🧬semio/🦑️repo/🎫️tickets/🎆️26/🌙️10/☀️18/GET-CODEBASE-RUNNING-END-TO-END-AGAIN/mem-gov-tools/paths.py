import os


def E(code):
    return chr(code) + "️"


def find_root():
    path = os.path.dirname(os.path.abspath(__file__))
    for _ in range(40):
        if os.path.isfile(os.path.join(path, "bunfig.toml")) and os.path.isfile(os.path.join(path, "bun.lock")):
            return path
        path = os.path.dirname(path)
    return "/Users/ueli/Documents/semio"


ROOT = find_root()
J = os.path.join
FRAMEWORK, MODULES, PROCESS, BUILD = E(0x1F9F0) + "framework", E(0x1F528) + "modules", E(0x1F3C3) + "process", E(0x1F3D7) + "build"
HOST = E(0x1F321) + "host-capacity"
SCHEMA, JSON_, TS, TESTS, FIXTURES = E(0x1F9EC) + "schema", E(0x1F523) + ".json", E(0x1F7E6) + ".ts", E(0x1F9EA) + "tests", E(0x1F9EB) + "fixtures"
DECISION, SAMPLING, GATE = E(0x1F4CF) + "decision", E(0x1F4E1) + "sampling", E(0x1F3AB) + "gate"
ORACLE, GATETEST = E(0x1F52E) + "sqlite", E(0x1F6AA) + "gate"
PACKAGES, TYPESCRIPT, JSONDIR = E(0x1F4E6) + "packages", E(0x1F7E6) + "typescript", E(0x1F523) + "json"
POLICY, LIBRARY = E(0x1F523) + "policy.json", None

M = J(FRAMEWORK, MODULES, PROCESS, BUILD, HOST)
PKG = J(FRAMEWORK, MODULES, PROCESS, PACKAGES, TYPESCRIPT, BUILD, HOST)

NEW_MODULE = {
    "module-schema.json": J(M, SCHEMA, JSON_),
    "module-schema-entry.ts": J(PKG, SCHEMA, TS),
    "module-schema-json-entry.ts": J(PKG, SCHEMA, JSONDIR, TS),
    "module-index.ts": J(M, TS),
    "module-decision.ts": J(M, DECISION, TS),
    "module-sampling.ts": J(M, SAMPLING, TS),
    "module-gate.ts": J(M, GATE, TS),
    "package-entry.ts": J(PKG, TS),
    "test-decision.ts": J(M, TESTS, TS),
    "test-fixtures.json": J(M, TESTS, FIXTURES, JSON_),
    "test-oracle.ts": J(M, TESTS, ORACLE, TS),
    "test-sampling.ts": J(M, TESTS, SAMPLING, TS),
    "test-sampling-fixtures.json": J(M, TESTS, SAMPLING, FIXTURES, JSON_),
    "test-gate.ts": J(M, TESTS, GATETEST, TS),
}

PRODUCTS, REPO, LIBRARY = E(0x1F6CD) + "products", E(0x1F991) + "repo", E(0x1F4DA) + "library"
REPOMODULES = E(0x1F528) + "modules"
L = J(FRAMEWORK, PRODUCTS, REPO, REPOMODULES, LIBRARY)
B = J(L, BUILD)
WORKERS, OPERATIONS, CATALOG, SCRIPT = E(0x1F477) + "workers", E(0x2699) + "operations", E(0x1F523) + "catalog.json", E(0x1F4DC) + "script.ts"
TOOLS, BAZEL, COMMAND = E(0x1F527) + "tools", E(0x1F3D7) + "bazel", E(0x2328) + "command"
CACHING, BOOTSTRAP, TOOLS2 = E(0x26A1) + "caching", E(0x1F680) + "bootstrap", E(0x1F6E0) + "tools"
POLICYFILE = E(0x1F523) + "policy.json"
HELPDIR = E(0x1F4D6) + "help"
PACKAGEJSON = "package.json"

NEW_GLUE = {
    "glue-capacity.ts": J(B, HOST, TS),
    "glue-command.ts": J(B, HOST, COMMAND, TS),
    "glue-test.ts": J(B, HOST, TESTS, TS),
    "glue-test-fixtures.json": J(B, HOST, TESTS, FIXTURES, JSON_),
    "glue-bazel.ts": J(B, TOOLS, BAZEL, HOST, TS),
    "glue-bazel-test.ts": J(B, TOOLS, BAZEL, HOST, TESTS, TS),
    "glue-bazel-fixtures.json": J(B, TOOLS, BAZEL, HOST, TESTS, FIXTURES, JSON_),
    "glue-nx.ts": J(L, CACHING, BOOTSTRAP, TOOLS2, HOST, TS),
    "glue-nx-test.ts": J(L, CACHING, BOOTSTRAP, TOOLS2, HOST, TESTS, TS),
}

TARGET = {
    "policy": J(B, POLICYFILE),
    "build-script": J(B, SCRIPT),
    "bazel-script": J(B, TOOLS, BAZEL, SCRIPT),
    "nx-script": J(L, CACHING, BOOTSTRAP, SCRIPT),
    "catalog": J(B, OPERATIONS, CATALOG),
    "operations-schema": J(B, OPERATIONS, SCHEMA, JSON_),
    "help-snapshots": J(B, OPERATIONS, TESTS, FIXTURES, HELPDIR, JSON_),
    "process-package": J(FRAMEWORK, MODULES, PROCESS, PACKAGES, TYPESCRIPT, PACKAGEJSON),
    "telemetry": J(FRAMEWORK, MODULES, PROCESS, E(0x1F4E6) + "artifacts", E(0x1F3D7) + "native-build", E(0x1F4C8) + "telemetry", TS),
}
