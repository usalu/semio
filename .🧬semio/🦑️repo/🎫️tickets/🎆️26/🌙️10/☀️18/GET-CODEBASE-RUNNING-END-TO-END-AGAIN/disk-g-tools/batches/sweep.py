"""Batch `sweep`: a declared enforcement cadence for every collectable cache area. Found by aud-disk-g and traced here: nothing starts the generic collector except explicit `repo cache gc` and three free-space triggers (graph, products, tools image), so `compile-cache-rust` (1.0 of 0.75 GiB), the undeclared lane scratch (14-day rule), retired roots (`.cache/bazel`, 182 MiB) and every loose tree area stayed over budget until somebody ran gc by hand. Every heavy command now reads the size ledger and the stamp of the last launch and starts the detached, low-priority, single-flight `repo cache gc --area=…` when the interval (12 h, all collectable areas, retired, undeclared) or a recorded excess (30 min, only the areas over budget) is due. Needs the ledger texts; independent of the other batches."""
import os

HERE = os.path.dirname(os.path.abspath(__file__))
FILES = os.path.join(HERE, "..", "staged", "sweep", "files")
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
G = f"{L}/🏗️build/🧹️cache-gc"
S = f"{L}/⚡️caching/💾️storage/🧾️ledger"

PATCH = open(os.path.join(FILES, "launch-patch.txt"), encoding="utf8").read()
DOC = "/** 📏️ The free bytes of the mounted image of a partition of a workspace, undefined while the image is not mounted. */"

EDITS = [
    (f"{S}/🧬️schema/🔣️.json", [(
        '        "quotaQuietMs": 259200000\n',
        '        "quotaQuietMs": 259200000,\n        "sweepIntervalMs": 43200000,\n        "overBudgetIntervalMs": 1800000\n',
    )]),
    (f"{S}/📖️texts/🔣️.json", [(
        '    "gc.graph.launch.failed": {',
        '    "gc.budget.launch.failed": { "en": "The budget cache sweep could not be started: {message}", "de": "Der Budget-Cache-Sammler konnte nicht gestartet werden: {message}" },\n    "gc.graph.launch.failed": {',
    )]),
    (f"{G}/🚀️launch/🟦️.ts", [
        ('import {mkdir,stat,writeFile} from "node:fs/promises";', 'import {mkdir,readFile,stat,writeFile} from "node:fs/promises";'),
        ('import {cacheLedgerV1,cacheTextV1,type CacheLedgerV1} from "../../../⚡️caching/💾️storage/🧾️ledger/🟦️.ts";',
         'import {CACHE_SIZE_FILE_V1,cacheLedgerV1,cacheRootDirectoryV1,cacheTextV1,readCacheSizeLedgerV1,type CacheLedgerV1,type CacheSizeLedgerV1} from "../../../⚡️caching/💾️storage/🧾️ledger/🟦️.ts";\nimport {budgetSweepDecisionV1,type BudgetSweepDecisionV1} from "../🧮️sweep/🟦️.ts";'),
        (DOC, PATCH + DOC),
    ]),
    (f"{L}/🏗️build/⌨️command/⚙️runtime/🟦️.ts", [(
        '    await profilePhaseV1("products.collect",async()=>{await(await import("../../🧹️cache-gc/🚀️launch/🟦️.ts")).launchProductsCollectorIfPressedV1(root);});',
        '    await profilePhaseV1("products.collect",async()=>{await(await import("../../🧹️cache-gc/🚀️launch/🟦️.ts")).launchProductsCollectorIfPressedV1(root);});\n    await profilePhaseV1("budget.sweep",async()=>{await(await import("../../🧹️cache-gc/🚀️launch/🟦️.ts")).launchBudgetSweepIfDueV1(root);});',
    )]),
]

NEW = [
    (f"{G}/🧮️sweep/🟦️.ts", "sweep.ts"),
    (f"{G}/🧮️sweep/🧪️tests/🟦️.ts", "sweep-test.ts"),
]
