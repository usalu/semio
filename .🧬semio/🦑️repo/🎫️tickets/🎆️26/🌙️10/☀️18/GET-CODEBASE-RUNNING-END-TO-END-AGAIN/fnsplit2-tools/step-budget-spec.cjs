const { R, g, dir, doc, write } = require("./lib_ks.cjs");
const file = R + "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/⏱️wgpu-worker-step-budget/🟦️.ts";
write({
  file, describeLine: 67,
  sharedRewrite: [['dirname(fileURLToPath(import.meta.url)), "..", "..")', 'dirname(fileURLToPath(import.meta.url)), "..", "..", "..")']],
  shared: { dir: dir("🧰", "step-budget-fixtures"), doc: doc("🧰", "The shared frame Worker step budget fixtures: the scheduler harness helpers the budget tests use.") },
  parts: [
    g("🧮", "scheduler-contract-and-ownership", "registerSchedulerContractAndOwnershipTests", "🎟", "The neutral scheduler contract, component close and frame owner alternation tests.", "Registers the two kind scheduler contract, component close, frame and asset decode owner alternation and yielded owner close tests.", 68, 206),
    g("📦", "native-response-and-decode-retirement", "registerNativeResponseAndDecodeRetirementTests", "🧾", "The native response page, decode retirement, mounted I/O and aborted image tests.", "Registers the native response page, reference decode retirement, contended mounted I/O task and aborted ready image tests.", 238, 340),
    g("🖼", "asset-decode-and-presenter-wakes", "registerAssetDecodeAndPresenterWakeTests", "🔔", "The asset decode path, runtime handback and parked presenter wake tests.", "Registers the off frame path asset decode, sealed asset retry, parked presenter checkout and reentrant wake tests.", 375, 483),
    g("🧵", "worker-task-and-ingress", "registerWorkerTaskAndIngressTests", "📥", "The armed Worker task, ingress acknowledgement and attribution ceiling tests.", "Registers the armed Worker task, ingress before frame callback and Worker ceiling attribution tests.", 532, 619),
    g("⏳", "boot-step-budgets", "registerBootStepBudgetTests", "🚦", "The boot step executing time, wall time, plugin graph chunk and bootstrap phase tests.", "Registers the boot step executing and wall time budgets, the plugin graph chunking and the bootstrap phase attribution tests.", 630, 689),
  ],
}, process.argv[2]);
