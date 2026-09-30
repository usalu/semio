"""🧪️ W2-W-norm-3: runs a norm subset's Python oracle handlers in-process against its committed vectors.

`python3 🧪️w2-w-norm-3-oracle-probe.py <subset dir> <case dir>` loads the repository test host as `semio_repo_test`, the shared
norm engine and the case's `🐍️.py` adapter, resolves `shared://`/`asset://` exactly as the platform does (owner `🧫️fixtures` /
`🖼️assets`), and runs every registered `mutate-`/`inverse-`/`identity-round-trip` oracle handler — the quick loop before the
real `oracle` phase.
"""
import importlib.util
import json
import os
import sys
import traceback

REPO = os.getcwd()
HOST = os.path.join(REPO, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🖥️host/🐍️.py")
ENGINE = os.environ.get("NORM_ENGINE_DIR") or os.path.join(REPO, "✏️s/🔌️plugins/📕️norm/🔮️oracles/🏃️execution")


def load(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


def main(subset, case):
    host = load("semio_repo_test", HOST)
    sys.path.insert(0, ENGINE)
    adapter_module = load("semio_test_adapter", os.path.join(case, "🐍️.py"))
    adapter = adapter_module.adapter()
    vectors = adapter_module.VECTORS
    fixtures = []
    for directory, fixture in vectors.values():
        for leaf in ("📸️snapshot/⬅️before", "🦠️mutation", "📸️snapshot/➡️after", "🎯️outcome"):
            uri = f"shared://🧬️mutations/{directory}/{fixture}/{leaf}/🔣️.json"
            fixtures.append({"uri": uri, "path": os.path.relpath(os.path.join(subset, "🧫️fixtures/🧬️mutations", directory, fixture, leaf, "🔣️.json"), REPO)})
    asset = adapter_module.DSL_ASSET
    fixtures.append({"uri": asset, "path": os.path.relpath(os.path.join(subset, "🖼️assets", asset[len("asset://"):]), REPO)})
    work = os.path.join(REPO, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/🗑️generated/w2w-norm3/probe-work")
    plan = {"workDir": work, "outputDir": work, "fixtures": fixtures, "owner": subset, "case": os.path.basename(case)}
    ids = [f"{role}-{kind}" for kind in adapter_module.KINDS for role in ("mutate", "inverse")] + ["identity-round-trip"]
    passed, failed = 0, []
    for scenario_id in ids:
        scenario = {"id": scenario_id, "steps": []}
        handler = adapter.handler(scenario, "oracle")
        try:
            handler(host.Context(plan, scenario, "oracle", REPO))
            passed += 1
        except Exception as error:
            failed.append((scenario_id, "".join(traceback.format_exception_only(type(error), error)).strip()[:600]))
    for scenario_id, error in failed:
        print("FAIL", scenario_id, "::", error)
    print(f"{passed}/{len(ids)} oracle scenarios pass")
    return 0 if not failed else 1


if __name__ == "__main__":
    sys.exit(main(sys.argv[1], sys.argv[2]))
