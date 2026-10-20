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


ROOT = os.environ.get("CACHE_GC_ROOT") or find_root()
J = os.path.join
FRAMEWORK, PRODUCTS, REPO, MODULES, LIBRARY = E(0x1F9F0) + "framework", E(0x1F6CD) + "products", E(0x1F991) + "repo", E(0x1F528) + "modules", E(0x1F4DA) + "library"
CACHING, STORAGE, ENFORCEMENT, BUDGET = E(0x26A1) + "caching", E(0x1F4BE) + "storage", E(0x1F512) + "enforcement", E(0x1F4E6) + "budget"
BUILD, OPERATIONS, PROCESS = E(0x1F3D7) + "build", E(0x2699) + "operations", E(0x1F3C3) + "process"
SCHEMA, JSON_, TS, TESTS, FIXTURES = E(0x1F9EC) + "schema", E(0x1F523) + ".json", E(0x1F7E6) + ".ts", E(0x1F9EA) + "tests", E(0x1F9EB) + "fixtures"
SCRIPT, CATALOG, POLICYFILE = E(0x1F4DC) + "script.ts", E(0x1F523) + "catalog.json", E(0x1F523) + "policy.json"
LEDGER, TEXTS, ORACLE, LAW = E(0x1F9FE) + "ledger", E(0x1F4D6) + "texts", E(0x1F52E) + "sqlite", E(0x2696) + "law"
GC, RUNNER, COMMAND = E(0x1F9F9) + "cache-gc", E(0x1F3C3) + "runner", E(0x2328) + "command"
PROVIDERS, HELPDIR = E(0x1F50C) + "providers", E(0x1F4D6) + "help"

L = J(FRAMEWORK, PRODUCTS, REPO, MODULES, LIBRARY)
B = J(L, BUILD)
LED = J(L, CACHING, STORAGE, LEDGER)
GCD = J(B, GC)
PROCESS_LIBRARY = J(L, PROCESS)

NEW = {
    "ledger-schema.json": J(LED, SCHEMA, JSON_),
    "ledger-policy.json": J(LED, POLICYFILE),
    "ledger-texts.json": J(LED, TEXTS, JSON_),
    "ledger-index.ts": J(LED, TS),
    "ledger-test.ts": J(LED, TESTS, TS),
    "ledger-fixtures.json": J(LED, TESTS, FIXTURES, JSON_),
    "ledger-oracle.ts": J(LED, TESTS, ORACLE, TS),
    "ledger-law.ts": J(LED, TESTS, LAW, TS),
    "gc-runner.ts": J(GCD, RUNNER, TS),
    "gc-command.ts": J(GCD, COMMAND, TS),
    "gc-test.ts": J(GCD, TESTS, TS),
}

TARGET = {
    "caching-index": J(L, CACHING, TS),
    "caching-policy": J(L, CACHING, POLICYFILE),
    "build-script": J(B, SCRIPT),
    "operations": J(B, OPERATIONS, TS),
    "catalog": J(B, OPERATIONS, CATALOG),
    "help-snapshots": J(B, OPERATIONS, TESTS, FIXTURES, HELPDIR, JSON_),
    "providers-test": J(PROCESS_LIBRARY, PROVIDERS, TESTS, TS),
    "nx-json": "nx.json",
}
