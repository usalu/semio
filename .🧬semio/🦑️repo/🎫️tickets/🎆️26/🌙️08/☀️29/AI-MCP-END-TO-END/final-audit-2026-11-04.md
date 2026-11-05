# Final Integration Audit — 2026-11-04

## Required Public Targeting And Context Gap

Source root: 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp.

🧬️schema/🧭️core-and-mutation-tool-schemas/🦀️.rs:104-124 closes both public action inputs with additionalProperties:false but exposes no target, instance, locale or terminology. 🦀️.rs:193-225 derives the instance slot from catalog capability ownership and calls the adapter using the fixed session. An MCP caller cannot choose among mounted instances of the same app/capability or provide missing genuine caller surface context through action_prepare/action_invoke. The prepared record preserves its selected instance correctly for later invoke, but initial public selection remains absent. The attached route ownership tests alone cannot establish public multi-instance targeting.

## Public Output Inspection

The root prepare/invoke handlers serialize complete reports at lines 210 and 226; neither drops untrusted. Reports schema lines 94-118 publishes nullable capability envelope and closed preview, matching the capability-output constructor. Dispatch lines 260-276 retains genuine pure output. Its empty-operation path at 442-470 returns that retained output without transaction prepare/commit and consumes the prepared handle. Commit lines 486-489 choose actual commit output if present, otherwise genuine retained pure output. Real native workspace Emit conversion preserves owner output at 🏠️workspace/🧩️artifact-app-frames/🦀️.rs:545-548. Native TransactionCommitted has no owner output field and maps None at 569, retaining the real pure fallback. Hub relay wrappers preserve supplied output. Complete InvocationReport idempotency preserves output on replay.

## Evidence Boundary

Reviewed plan, launch-summary, bridge-execution, native-execution and coordinator-evidence. This is a read-only source audit. No native compiler, native readiness, real catalog discovery, artifact roundtrip or native visible UI success is established by this review. Those remain required execution gates.
