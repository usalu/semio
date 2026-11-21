import { fileURLToPath } from "node:url";
import { issueStoreReceiptV1, settleStoreReceiptV1, storeReceiptOutputV1 } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧾️store-receipt/🟨️.mjs";

const [mode, projectRoot, target, issuedJson, ok, start] = process.argv.slice(2);
const declaration = {
  workspaceRoot: fileURLToPath(new URL("../../../../../../../", import.meta.url)),
  projectRoot,
  target,
  outputs: [storeReceiptOutputV1(target)],
};
if (mode === "issue") {
  process.stdout.write(JSON.stringify(issueStoreReceiptV1(declaration)));
} else if (mode === "settle" && ["0", "1"].includes(ok) && Number.isFinite(Number(start))) {
  await settleStoreReceiptV1(declaration, JSON.parse(issuedJson), { succeeded: ok === "1", startedAtMs: Number(start) });
} else {
  throw Error("Expected issue or settle with task status and start time");
}
