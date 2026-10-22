"""Which files belong to which catalog and under which key scope."""
from lib import L

VERIFY = L + "/🎛️commands/✅️verify"
COMMIT = L + "/🎛️commands/📝️commit"
BOOT = L + "/⚡️caching/🚀️bootstrap"

CATALOGS = {
    "verify": {
        "texts": VERIFY + "/📖️texts",
        "say": "sayVerifyV1", "refuse": "refuseVerifyV1", "handle": "VERIFY_TEXTS_V1", "doc": "`repo verify`: every progress line, verdict and refusal of the lanes",
        "files": {
            VERIFY + "/🎨️storybook-freshness/🟦️.ts": "storybook",
            VERIFY + "/🎯️tool-job-lane/🟦️.ts": "tool-jobs",
            VERIFY + "/🏁️gate-lanes/🟦️.ts": "gates",
            VERIFY + "/🔬️interactivity-lanes/🟦️.ts": "interactivity",
            VERIFY + "/🔷️typescript-project/🟦️.ts": "typescript",
            VERIFY + "/🚦️gates/🟦️.ts": "census",
            VERIFY + "/🟦️.ts": "run",
            VERIFY + "/🦀️rust-warnings/🟦️.ts": "rust-warnings",
            VERIFY + "/🧾️taxonomy-lanes/🟦️.ts": "lanes",
        },
    },
    "commit": {
        "texts": COMMIT + "/📖️texts",
        "say": "sayCommitV1", "refuse": "refuseCommitV1", "handle": "COMMIT_TEXTS_V1", "doc": "`repo commit` and `repo micro-commit`: the guidance, verdicts and refusals of the bundle workflow",
        "files": {
            COMMIT + "/📄️prepared-message/🟦️.ts": "message",
            COMMIT + "/📈️attribution/🟦️.ts": "attribution",
            COMMIT + "/📊️metrics/🧮️summary/🟦️.ts": "metrics",
            COMMIT + "/📜️body/🟦️.ts": "body",
            COMMIT + "/📤️emit/🟦️.ts": "emit",
            COMMIT + "/🔀️run-commit/🟦️.ts": "run",
            COMMIT + "/🔢️counter/🟦️.ts": "counter",
            COMMIT + "/🔸️bullets/🟦️.ts": "bullets",
            COMMIT + "/🪙️micro-commit/🟦️.ts": "micro",
        },
    },
    "bootstrap": {
        "texts": BOOT + "/📖️texts",
        "say": "sayBootstrapV1", "refuse": "refuseBootstrapV1", "handle": "BOOTSTRAP_TEXTS_V1", "doc": "the Nx bootstrap: the progress lines of the owner admission, the watcher, the graph and the generation receipt",
        "files": {BOOT + "/📜️script.ts": "nx"},
        "kinds": ("console",),
    },
}
