import {
  policyStdioCatalogBreaches,
  policyIoSerializerMatrixBreaches,
  policyIoTerminalityBreaches,
  policyCodecFidelityBreaches,
} from "../../../../../../📜️script.ts";

const repoRoot = "/Users/ueli/Documents/semio";

const runs: Array<[string, () => ReturnType<typeof policyStdioCatalogBreaches>]> = [
  ["policyStdioCatalogBreaches", () => policyStdioCatalogBreaches(repoRoot)],
  ["policyIoSerializerMatrixBreaches", () => policyIoSerializerMatrixBreaches(repoRoot)],
  ["policyIoTerminalityBreaches", () => policyIoTerminalityBreaches(repoRoot)],
  ["policyCodecFidelityBreaches", () => policyCodecFidelityBreaches(repoRoot)],
];

for (const [name, fn] of runs) {
  try {
    const breaches = fn();
    const missingOwnerTable = breaches.filter((b) => b.id === "stdio-catalog-owner-table-missing");
    console.log(`[${name}] ran OK — ${breaches.length} breach(es) total; owner-table-missing breaches: ${missingOwnerTable.length}`);
    if (missingOwnerTable.length > 0) {
      console.log(JSON.stringify(missingOwnerTable, null, 2));
    }
  } catch (err) {
    console.log(`[${name}] THREW: ${(err as Error).message}`);
  }
}
