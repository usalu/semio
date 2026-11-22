/** 🛟️ Checks the basis-point admission reserve against the pinned 7 GiB fixture and the policy cap. */
import {resolve} from "node:path";
const root = resolve(import.meta.dir, "../../../../../../..");
const a = await import(resolve(root, "🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🟦️.ts"));
const effective = a.admissionMemoryBytesV1(8321515520, 7516192768);
const rows = {effective, reserve: a.admissionReserveBytesV1(effective), capped: a.admissionReserveBytesV1(64 * 2 ** 30), share: a.admissionMemoryShareV1(17179869184, 1, 4)};
console.log(JSON.stringify(rows));
if (rows.reserve !== 1879048192 || rows.capped !== a.ADMISSION_POLICY_V1.maximumReserveBytes || !Number.isSafeInteger(rows.share)) process.exit(1);
