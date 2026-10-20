/** 🩹 Missed consumer of the d6 change: the settlement test kit still destructured the kernel return content class from the shard client dependencies. */
import { Plan } from "./plan.ts";
import { finish } from "./direct.ts";
const FILE = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️shardclient-reserved-response-settlement/🧰️settlement-test-kit/🟦️.ts";
const plan = new Plan();
const edit = (from: string, to: string): void => plan.edit({ file: FILE, mode: "text", from, to, wave: "fix-d6", why: "the shard client no longer re-exports the kernel return content; the kit imports it from its owner" });
edit('import { fixtureOriginalWorkStock } from "@semio-tech/framework-actor/retained-turn/bootstrap/testing/fixtures";', 'import { fixtureOriginalWorkStock } from "@semio-tech/framework-actor/retained-turn/bootstrap/testing/fixtures";\nimport { OwnedKernelReturnContent as OwnedKernelReturnContentClass } from "@semio-tech/framework-kernel/return/content/input";');
edit("OwnedActorTurnOutput, OwnedActorTurnOutputs, OwnedKernelReturnContent, OwnedNativeUiPatchAuthority", "OwnedActorTurnOutput, OwnedActorTurnOutputs, OwnedNativeUiPatchAuthority");
edit("NO_RESIDENT_FAULT, OwnedActorTurnOutputs, OwnedKernelReturnContent, OwnedNativeUiPatchAuthority", "NO_RESIDENT_FAULT, OwnedActorTurnOutputs, OwnedKernelReturnContent: OwnedKernelReturnContentClass, OwnedNativeUiPatchAuthority");
finish(plan, "fix6", process.argv);
