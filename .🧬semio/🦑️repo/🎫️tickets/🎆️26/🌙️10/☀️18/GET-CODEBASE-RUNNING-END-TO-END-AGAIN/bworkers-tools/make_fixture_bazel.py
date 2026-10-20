# 🧪 Writes the Bazel and Nx worker mapping fixtures: expected flags and environments were read and reviewed against the Bazel and Nx documentation and options, then pinned.
import json
import lib

FLAGS_DEFAULT = ["--jobs=8", "--strategy=RepoTest=worker,sandboxed,local", "--worker_max_instances=RepoTest=8", "--strategy=RepoCheck=worker,sandboxed,local", "--worker_max_instances=RepoCheck=8", "--strategy=RepoGenerate=worker,sandboxed,local", "--worker_max_instances=RepoGenerate=1", "--strategy=RepoBuild=worker,sandboxed,local", "--worker_max_instances=RepoBuild=1", "--strategy=RepoScript=worker,sandboxed,local", "--worker_max_instances=RepoScript=1", "--noworker_multiplex", "--experimental_worker_cancellation", "--experimental_worker_strict_flagfiles", "--experimental_total_worker_memory_limit_mb=9830", "--experimental_shrink_worker_pool"]
FLAGS_SMALL = ["--jobs=2", "--strategy=RepoTest=worker,sandboxed,local", "--worker_max_instances=RepoTest=1", "--strategy=RepoCheck=worker,sandboxed,local", "--worker_max_instances=RepoCheck=1", "--strategy=RepoGenerate=sandboxed,local", "--strategy=RepoBuild=sandboxed,local", "--strategy=RepoScript=sandboxed,local", "--noworker_multiplex", "--experimental_worker_cancellation", "--experimental_worker_strict_flagfiles", "--experimental_total_worker_memory_limit_mb=1228", "--experimental_shrink_worker_pool"]
FLAGS_OFF = ["--jobs=8", "--strategy=RepoTest=sandboxed,local", "--strategy=RepoCheck=sandboxed,local", "--strategy=RepoGenerate=sandboxed,local", "--strategy=RepoBuild=sandboxed,local", "--strategy=RepoScript=sandboxed,local"]
FLAGS_MULTI = ["--jobs=8", "--strategy=RepoTest=worker,sandboxed,local", "--worker_max_instances=RepoTest=8", "--strategy=RepoCheck=worker,sandboxed,local", "--worker_max_instances=RepoCheck=8", "--strategy=RepoGenerate=worker,sandboxed,local", "--worker_max_instances=RepoGenerate=1", "--worker_max_multiplex_instances=RepoGenerate=8", "--strategy=RepoBuild=worker,sandboxed,local", "--worker_max_instances=RepoBuild=1", "--strategy=RepoScript=worker,sandboxed,local", "--worker_max_instances=RepoScript=1", "--worker_multiplex", "--worker_sandboxing", "--experimental_worker_cancellation", "--experimental_worker_strict_flagfiles", "--experimental_total_worker_memory_limit_mb=9830", "--experimental_shrink_worker_pool"]
HOST = {"cores": 10, "memoryBytes": 32 * 1024 ** 3}
SMALL = {"cores": 2, "memoryBytes": 4 * 1024 ** 3}
MULTI = {"kinds.generate.multiplex": True, "kinds.generate.inProcess": False, "sandboxing": True}

bazel = {
    "version": 1,
    "scenarios": [
        {"name": "default policy on ten cores and 32 GiB", "host": HOST, "overrides": {}, "policy": {}, "flags": FLAGS_DEFAULT},
        {"name": "small host leaves later kinds without workers", "host": SMALL, "overrides": {}, "policy": {}, "flags": FLAGS_SMALL},
        {"name": "workers switched off", "host": HOST, "overrides": {"workers": False}, "policy": {}, "flags": FLAGS_OFF},
        {"name": "multiplexed generate kind with sandboxed workers", "host": HOST, "overrides": {}, "policy": MULTI, "flags": FLAGS_MULTI},
    ],
    "mnemonics": {"test": "RepoTest", "check": "RepoCheck", "generate": "RepoGenerate", "build": "RepoBuild", "script": "RepoScript", "service": "RepoLauncher"},
    "graph": {
        "version": 1,
        "projects": {"p": {"name": "p", "root": "p", "targets": {
            "test-unit": {"execution": {"kind": "command", "command": "bun ./x.ts test"}, "cache": True},
            "typecheck": {"execution": {"kind": "command", "command": "bun ./x.ts typecheck"}, "cache": True},
            "generate-docs": {"execution": {"kind": "command", "command": "bun ./x.ts generate"}, "cache": True},
            "build": {"execution": {"kind": "command", "command": "bun ./x.ts build"}, "cache": True},
            "session": {"execution": {"kind": "command", "command": "bun ./x.ts session"}, "cache": False},
            "serve": {"execution": {"kind": "command", "command": "bun ./x.ts serve"}, "continuous": True, "cache": False},
        }}},
        "dependencies": [],
        "namedInputs": {},
    },
    "actions": {"p:test-unit": {"kind": "test", "mnemonic": "RepoTest", "multiplex": False}, "p:typecheck": {"kind": "check", "mnemonic": "RepoCheck", "multiplex": False}, "p:generate-docs": {"kind": "generate", "mnemonic": "RepoGenerate", "multiplex": False}, "p:build": {"kind": "build", "mnemonic": "RepoBuild", "multiplex": False}, "p:session": {"kind": "script", "mnemonic": "RepoScript", "multiplex": False}},
}
lib.write_staged("bazel-workers-fixture.json", json.dumps(bazel, indent=1, ensure_ascii=False) + "\n")

nx = {
    "version": 1,
    "scenarios": [
        {"name": "default policy", "host": HOST, "overrides": {}, "policy": {}, "batchCapable": False, "environment": {"NX_PARALLEL": "8"}},
        {"name": "small host", "host": SMALL, "overrides": {}, "policy": {}, "batchCapable": False, "environment": {"NX_PARALLEL": "2"}},
        {"name": "explicit job count", "host": HOST, "overrides": {"jobs": 3}, "policy": {}, "batchCapable": False, "environment": {"NX_PARALLEL": "3"}},
        {"name": "multiplexed kind without a batch executor", "host": HOST, "overrides": {}, "policy": MULTI, "batchCapable": False, "environment": {"NX_PARALLEL": "8"}},
        {"name": "multiplexed kind with a batch executor", "host": HOST, "overrides": {}, "policy": MULTI, "batchCapable": True, "environment": {"NX_PARALLEL": "8", "NX_BATCH_MODE": "true"}},
        {"name": "workers off never batches", "host": HOST, "overrides": {"workers": False}, "policy": MULTI, "batchCapable": True, "environment": {"NX_PARALLEL": "8"}},
    ],
    "executors": [
        {"name": "this repository", "batchCapable": False, "value": "repo"},
        {"name": "one batch executor", "batchCapable": True, "value": {"executors": {"a": {"implementation": "./a.js"}, "b": {"implementation": "./b.js", "batchImplementation": "./b-batch.js"}}}},
        {"name": "empty", "batchCapable": False, "value": {}},
        {"name": "batch implementation of the wrong type", "batchCapable": False, "value": {"executors": {"a": {"batchImplementation": True}}}},
    ],
}
lib.write_staged("nx-workers-fixture.json", json.dumps(nx, indent=1, ensure_ascii=False) + "\n")
print("ok")
