"""🔮️ S3-GATES: registers the third-party test oracles this ticket's tests use in their owners' `🔮️oracles/🔣️.json` contributions.

The dependency gate (`verify dependencies literal-external`) classifies a package as `test-oracle` only when an oracle registry entry
claims it; the test platform reads the same contributions (`discoverTestContributions`). Each owner gets the entries for the oracles
its own laws import. Existing contributions keep their formatting (they round-trip through `json.dumps(indent=2)` byte for byte); new
ones carry the minimal registry shape (`schemaVersion` 2, `oracles`, `noOracleDecisions`). Idempotent: an id already present is kept.
"""
import json
import os

ROOT = "/Users/ueli/Documents/semio"
SCHEMA = "🧰️framework/🔨️modules/🧪️test/🧬️schema/🔣️.json"
LIB = {
    "color-string": ("1.9.1", "MIT", "https://github.com/Qix-/color-string"),
    "d3-scale": ("4.0.2", "ISC", "https://d3js.org/d3-scale"),
    "decimal.js": ("10.6.0", "MIT", "https://mikemcl.github.io/decimal.js"),
    "fast-check": ("3.23.2", "MIT", "https://fast-check.dev"),
    "xstate": ("5.32.5", "MIT", "https://stately.ai/docs/xstate"),
    "aria-query": ("5.3.0", "Apache-2.0", "https://github.com/A11yance/aria-query"),
    "dom-accessibility-api": ("0.5.16", "MIT", "https://github.com/eps1lon/dom-accessibility-api"),
    "web-tree-sitter": ("0.20.8", "MIT", "https://github.com/tree-sitter/tree-sitter/tree/master/lib/binding_web"),
    "tree-sitter-wasms": ("0.1.13", "Unlicense", "https://github.com/Gregoor/tree-sitter-wasms"),
}
OWNERS = {
    "🧰️framework/🔨️modules/🖱️ui": [
        ("ui-color-input-color-string", "color-string", "color-string get.rgb()", ["ui.color-input.hex-codec"], "Judges `ui_color_hex`/`parse_ui_color_hex` in `🧬️contract/🧪️tests/🧪️color-input` over `🧫️fixtures/🧫️color-input`: color-string parses every accepted text to the same channels and alpha, refuses every refused one and prints the same hex, so the colour input commits the colour the user typed."),
        ("ui-number-controls-d3-scale", "d3-scale", "d3-scale scaleLog()/scaleLinear()", ["ui.number-controls.axis"], "Judges the number controls in `🧬️contract/🧪️tests/🧪️number-controls` (design §18) over `🧫️fixtures/🧫️number-controls`: d3-scale's `scaleLog`/`scaleLinear` are the independent slider axis (position and inverse) and display-factor oracles."),
        ("ui-number-controls-decimal-js", "decimal.js", "decimal.js Decimal", ["ui.number-controls.rounding"], "The independent rounding and ladder oracle of `🧬️contract/🧪️tests/🧪️number-controls` over `🧫️fixtures/🧫️number-controls`: it rounds the EXACT binary expansion of every value half away from zero, so a twin that rounded the shortest decimal would disagree on `1.005`/`2.675`."),
        ("ui-text-splice-fast-check", "fast-check", "fast-check fc.assert", ["ui.scene.text-splice"], "Generates concurrent insert-only typing workloads for `🎬️scene/🧪️tests/✂️text-splice`: folded in hub order, no workload may lose a typed scalar."),
    ],
    "🧰️framework/🔨️modules/🛂️manifest": [
        ("manifest-number-facets-decimal-js", "decimal.js", "decimal.js Decimal", ["manifest.number-facets"], "The independent decimal oracle of `🧪️tests/🧪️number-facets` (staged-argument number facets) over `🧫️fixtures/🧫️number-facets`: it recomputes every refusal's bound in display units (`value × displayFactor` to twelve significant digits) beside the shown unit."),
    ],
    "🧰️framework/🔨️modules/⏪️time-travel": [
        ("time-travel-xstate", "xstate", "xstate createMachine()", ["time-travel.lifecycle"], "Judges the time-travel session lifecycle in `🧪️tests/🧪️conformance` (design §4): the `🧫️lifecycle-law` matrix built as an xstate machine with an independent context model must accept and refuse exactly what the TypeScript twin does."),
        ("time-travel-fast-check", "fast-check", "fast-check fc.assert", ["time-travel.lifecycle"], "Generates random event sequences for the lifecycle law in `🧪️tests/🧪️conformance`; each sequence is folded by the twin and by the xstate oracle and the two must agree at every step."),
    ],
    "🧰️framework/🔨️modules/🛠️tool-machine": [
        ("tool-machine-xstate", "xstate", "xstate createMachine()", ["tool-machine.transaction"], "Judges the tool machine in `🧪️tests/🧪️conformance` (design §5): every `🧫️transaction-law` chart built as an xstate machine against the TypeScript reducer, id minting and runner."),
        ("tool-machine-fast-check", "fast-check", "fast-check fc.assert", ["tool-machine.transaction"], "Generates random yield sequences for the transaction law in `🧪️tests/🧪️conformance`; reducer and xstate oracle must agree at every step."),
    ],
    "🧰️framework/🔨️modules/📡️replication": [
        ("replication-supersede-fold-fast-check", "fast-check", "fast-check fc.assert", ["replication.supersede-fold"], "Shuffles the events of every `🧫️supersede-fold` vector in `🧪️tests/🧪️supersede-fold` (design §2): the resolved supersessions must not depend on the order the events arrive in."),
    ],
    "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store": [
        ("store-tool-transaction-xstate", "xstate", "xstate createMachine()", ["store.tool-transaction"], "Judges the store's tool-transaction lifecycle in `🧪️tests/🧪️tool-transaction` (design §15): an xstate machine agrees with the store twin's refusals and open transaction on the corpus and on generated runs."),
        ("store-tool-transaction-fast-check", "fast-check", "fast-check fc.assert", ["store.tool-transaction"], "Generates runs for `🧪️tests/🧪️tool-transaction`: an abort always restores the state and persisted value of before the transaction, and a commit adds exactly one edit."),
    ],
    "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev": [
        ("dev-time-travel-aria-query", "aria-query", "aria-query roles", ["dev.time-travel.accessibility"], "The ARIA role model the time-travel probe (`🧪️tests/🧪️time-travel`) checks the history panel, editor and finalize dialog against: unknown or unsupported `aria-*` attributes and roles fail the run."),
        ("dev-time-travel-dom-accessibility-api", "dom-accessibility-api", "dom-accessibility-api computeAccessibleName()", ["dev.time-travel.accessibility"], "Computes the accessible name and description of every interactive history control in the time-travel probe (`🧪️tests/🧪️time-travel`) independently of the renderer's own labelling."),
    ],
    "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine": [
        ("engine-history-dom-accessibility-api", "dom-accessibility-api", "dom-accessibility-api computeAccessibleName()", ["engine.history.accessibility"], "Computes accessible names and descriptions in the React history and staged-argument laws (`🧱️elements/🛠️ShellHelpers/⏪️time-travel/🧪️tests/🧩️component`, `🧱️elements/🛠️ShellHelpers/🧪️tests/🧪️staged-arg-controls`, `🧱️elements/🏛️ShellHost/📎️local-folders/🧪️tests/🧩️component`): rows, disabled actions and their reasons must be named as the contract declares."),
    ],
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test": [
        ("mutation-label-tree-sitter-rust", "web-tree-sitter", "web-tree-sitter Parser", ["repo.test.mutation-label-gate"], "The independent syntax-tree reading of every label site in `🧪️tests/🧪️mutation-history-gates`: the `schema mutation-labels` token gate and the tree-sitter-rust parse must give the same verdict on every corpus case of `🧫️fixtures/🧫️mutation-labels`."),
        ("mutation-label-tree-sitter-rust-grammar", "tree-sitter-wasms", "tree-sitter-wasms out/tree-sitter-rust.wasm", ["repo.test.mutation-label-gate"], "The prebuilt tree-sitter-rust grammar that `mutation-label-tree-sitter-rust` loads (`Bun.resolveSync(\"tree-sitter-wasms/package.json\")` → `out/tree-sitter-rust.wasm`) in `🧪️tests/🧪️mutation-history-gates`; third-party, so the syntax tree the label oracle reads is not this repository's own grammar."),
    ],
    "✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any": [
        ("forms-change-block-field-fast-check", "fast-check", "fast-check fc.assert", ["forms-change-block-field"], "The property oracle of `🧬️schema/🧪️tests/🧪️change-block-field` (design §17.1): over generated payloads the TS twin admits exactly what the leaf's JSON Schema admits."),
    ],
}


def entry(oracle_id: str, package: str, implementation: str, capabilities: list[str], rationale: str) -> dict:
    version, license_id, homepage = LIB[package]
    row = {
        "id": oracle_id,
        "kind": "third-party-library",
        "ecosystem": "javascript",
        "package": package,
        "version": version,
        "engine": {"family": package.replace(".", "-"), "implementation": implementation, "version": version},
        "capabilities": capabilities,
        "comparisonProfiles": ["ordered-json-v1"],
        "license": license_id,
        "testOnly": True,
        "productionReachable": False,
        "networkDuringExecution": False,
        "homepage": homepage,
        "rationale": rationale,
    }
    return row


def main() -> None:
    for owner, rows in OWNERS.items():
        path = os.path.join(ROOT, owner, "🔮️oracles", "🔣️.json")
        if os.path.exists(path):
            document = json.load(open(path, encoding="utf8"))
        else:
            os.makedirs(os.path.dirname(path), exist_ok=True)
            depth = owner.count("/") + 2
            document = {"$schema": "../" * depth + SCHEMA, "schemaVersion": 2, "_comment": "🔮️ This owner's contribution to the repository test platform: the third-party references its own laws judge against.", "oracles": [], "noOracleDecisions": []}
        present = {row["id"] for row in document["oracles"]}
        added = [entry(*row) for row in rows if row[0] not in present]
        document["oracles"].extend(added)
        with open(path, "w", encoding="utf8") as out:
            out.write(json.dumps(document, indent=2, ensure_ascii=False) + "\n")
        print(f"{owner}: +{len(added)} ({', '.join(row['id'] for row in added) or 'none'})")


if __name__ == "__main__":
    main()
