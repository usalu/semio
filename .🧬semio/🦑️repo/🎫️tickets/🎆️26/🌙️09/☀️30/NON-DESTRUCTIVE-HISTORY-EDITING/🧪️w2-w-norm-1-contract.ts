/**
 * 🧪️ W2-W norm-1: the contract phase restricted to the EN 1991 / EN 1990 owners — the case contract of both mutate
 * cases plus every registry-level breach (vector registry, fixture law 2, inventory, binary protocol drift) whose scope names either subset.
 *
 *     bun 🧪️w2-w-norm-1-contract.ts
 */
import { join, resolve } from "node:path";
import { binaryProtocolDriftBreaches, discoverTestCases, loadOracleRegistry, mutationFixtureBreaches, mutationInventoryBreaches, mutationVectorRegistryBreaches, validateCaseContract } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟦️.ts";

const root = resolve(import.meta.dir, "../../../../../../..");
const owners = ["🗿️artifacts/🏋️en1991/", "🗿️artifacts/⚖️en1990/"];
const mine = (text: string) => owners.some((owner) => text.includes(owner)) || /s\.norm\.en199[01]@/.test(text);
const registry = loadOracleRegistry(root);
const cases = discoverTestCases(root).filter((entry) => mine(entry.caseDir));
const breaches = [
  ...cases.flatMap((entry) => validateCaseContract(root, entry, registry)),
  ...[...mutationVectorRegistryBreaches(root, registry), ...mutationFixtureBreaches(registry), ...mutationInventoryBreaches(root, registry), ...binaryProtocolDriftBreaches(root, registry)].filter((breach) => mine(breach.scope) || mine(breach.summary)),
];
console.log(JSON.stringify({ cases: cases.map((entry) => entry.caseDir), breaches: breaches.map(({ kind, id, scope, summary }) => ({ kind, id, scope: scope.replace(join(root, "/"), ""), summary })) }, null, 2));
