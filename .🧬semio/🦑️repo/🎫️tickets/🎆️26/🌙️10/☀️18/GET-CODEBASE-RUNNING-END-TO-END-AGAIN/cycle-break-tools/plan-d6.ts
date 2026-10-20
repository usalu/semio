/** ✂️ D6: the shard client no longer imports the kernel return content (it only forwarded it into a test bag), so actor stays below kernel. */
import { Plan } from "./plan.ts";
import { finish } from "./direct.ts";
const W = "d6-actor-below-kernel";
const plan = new Plan();
const S = "🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts";
plan.edit({ file: S, from: `import { OwnedKernelReturnContent } from "@semio-tech/framework-kernel/return/content/input";\n`, to: "", mode: "text", wave: W, why: "actor must not import kernel" });
plan.edit({ file: S, from: "OwnedActorTurnOutputs, OwnedKernelReturnContent, OwnedNativeUiPatchAuthority", to: "OwnedActorTurnOutputs, OwnedNativeUiPatchAuthority", mode: "text", wave: W, why: "the test bag no longer forwards a kernel class" });
const T = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️shardclient-reserved-response-settlement";
const imp = `import { OwnedKernelReturnContent } from "@semio-tech/framework-kernel/return/content/input";\n`;
const a = `${T}/🔓️captured-return-admission/🟦️.ts`, b = `${T}/🔗️kernel-return-input-binding/🟦️.ts`;
plan.edit({ file: a, from: "const { BUDGET, NO_RESIDENT_FAULT, OwnedKernelReturnContent, OwnedResidentLedger,", to: "const { BUDGET, NO_RESIDENT_FAULT, OwnedResidentLedger,", mode: "text", wave: W, why: "the class comes from the kernel directly" });
plan.edit({ file: a, from: `import { readModuleTreeSource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";\n`, to: `import { readModuleTreeSource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";\n${imp}`, mode: "text", wave: W, why: "the class comes from the kernel directly" });
plan.edit({ file: b, from: "const { OwnedKernelReturnContent, OwnedShardReturnPage, OwnedUiInstance,", to: "const { OwnedShardReturnPage, OwnedUiInstance,", mode: "text", wave: W, why: "the class comes from the kernel directly" });
plan.edit({ file: b, from: `import type { ShardClientSettlementTestKit } from "../🧰️settlement-test-kit/🟦️.ts";\n`, to: `${imp}import type { ShardClientSettlementTestKit } from "../🧰️settlement-test-kit/🟦️.ts";\n`, mode: "text", wave: W, why: "the class comes from the kernel directly" });
finish(plan, "d6", process.argv);
