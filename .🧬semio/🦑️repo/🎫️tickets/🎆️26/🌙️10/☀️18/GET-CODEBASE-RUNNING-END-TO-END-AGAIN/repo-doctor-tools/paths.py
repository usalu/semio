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


ROOT = os.environ.get("REPO_DOCTOR_ROOT") or find_root()
HERE = os.path.dirname(os.path.abspath(__file__))
STAGED = os.path.join(HERE, "staged")
J = os.path.join

FRAMEWORK, PRODUCTS, REPO, MODULES, LIBRARY = E(0x1F9F0) + "framework", E(0x1F6CD) + "products", E(0x1F991) + "repo", E(0x1F528) + "modules", E(0x1F4DA) + "library"
BUILD, OPERATIONS = E(0x1F3D7) + "build", E(0x2699) + "operations"
SCHEMA, JSON_, TS, TESTS, FIXTURES = E(0x1F9EC) + "schema", E(0x1F523) + ".json", E(0x1F7E6) + ".ts", E(0x1F9EA) + "tests", E(0x1F9EB) + "fixtures"
SCRIPT, CATALOG, POLICYFILE = E(0x1F4DC) + "script.ts", E(0x1F523) + "catalog.json", E(0x1F523) + "policy.json"
HELP = E(0x1F4D6) + "help"

L = J(FRAMEWORK, PRODUCTS, REPO, MODULES, LIBRARY)
B = J(L, BUILD)
D = J(B, E(0x1FA7A) + "doctor")
PROBES = J(D, E(0x1F50E) + "probes")
TESTROOT = J(D, TESTS)

NEW = {
    "schema.json": J(D, SCHEMA, JSON_),
    "policy.json": J(D, POLICYFILE),
    "rules.json": J(D, E(0x1F4D6) + "rules", JSON_),
    "texts.json": J(D, E(0x1F4D6) + "texts", JSON_),
    "index.ts": J(D, TS),
    "analysis.ts": J(D, E(0x1F9EE) + "analysis", TS),
    "ports.ts": J(D, E(0x1F50C) + "ports", TS),
    "report.ts": J(D, E(0x1F5A8) + "report", TS),
    "runner.ts": J(D, E(0x1F3C3) + "runner", TS),
    "command.ts": J(D, E(0x2328) + "command", TS),
    "readme.md": J(D, "README.md"),
    "probes.ts": J(PROBES, TS),
    "probe-tools.ts": J(PROBES, E(0x1F527) + "tools", TS),
    "probe-storage.ts": J(PROBES, E(0x1F4BE) + "storage", TS),
    "probe-leases.ts": J(PROBES, E(0x1F512) + "leases", TS),
    "probe-capacity.ts": J(PROBES, E(0x1F3AB) + "capacity", TS),
    "probe-sessions.ts": J(PROBES, E(0x1FAAA) + "sessions", TS),
    "probe-graph.ts": J(PROBES, E(0x1F578) + "graph", TS),
    "probe-dependencies.ts": J(PROBES, E(0x1F4E6) + "dependencies", TS),
    "probe-caches.ts": J(PROBES, E(0x26A1) + "caches", TS),
    "probe-processes.ts": J(PROBES, E(0x1F3C3) + "processes", TS),
    "test.ts": J(TESTROOT, TS),
    "test-fixtures.json": J(TESTROOT, FIXTURES, JSON_),
    "test-oracle.py": J(TESTROOT, E(0x1F40D) + "oracle.py"),
    "test-probes.ts": J(TESTROOT, E(0x1F50E) + "probes", TS),
    "test-command.ts": J(TESTROOT, E(0x2328) + "command", TS),
}

TARGET = {
    "build-script": J(B, SCRIPT),
    "catalog": J(B, OPERATIONS, CATALOG),
    "operations-schema": J(B, OPERATIONS, SCHEMA, JSON_),
    "help-snapshots": J(B, OPERATIONS, TESTS, FIXTURES, HELP, JSON_),
}
