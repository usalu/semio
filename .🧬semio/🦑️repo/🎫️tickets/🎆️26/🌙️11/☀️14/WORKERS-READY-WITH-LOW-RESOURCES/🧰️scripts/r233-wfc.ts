import {chmodSync, mkdtempSync, readFileSync, renameSync, rmdirSync, rmSync, statSync, writeFileSync} from "node:fs";
import {join} from "node:path";
import {fileURLToPath, pathToFileURL} from "node:url";

const root = process.cwd();
const target = "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/📦️packages/🦀️rust";
const reconciliation = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🧾️registry-consistency/🟦️.ts";
const {BUILD_OWNER_REGISTRY_FILE, checkBuildOwnerRegistry, reconcileBuildOwnerRegistry} = await import(pathToFileURL(join(root, reconciliation)).href);
const registry = join(root, BUILD_OWNER_REGISTRY_FILE);
const generated = fileURLToPath(new URL("../🗑️generated/", import.meta.url));
const scratch = mkdtempSync(join(generated, "r233-wfc-"));
const pending = join(scratch, "owner.json");
const read = () => readFileSync(registry, "utf8");
const json = (value: unknown) => JSON.stringify(value, null, 2) + "\n";

try {
  let committed = false;
  for (let attempt = 0; attempt < 5 && !committed; attempt++) {
    const before = read();
    const original = JSON.parse(before);
    const repair = reconcileBuildOwnerRegistry(root, {scope: [target], write: false});
    const planned = repair.registry;
    const row = planned.owners.find((owner: {root: string}) => owner.root === target);
    if (!row || row.manifest !== target + "/Cargo.toml") throw Error("Required WFC Cargo owner is absent from scoped reconciliation");
    if (read() !== before) continue;
    if (!repair.changed) {
      if (!original.owners.some((owner: {root: string}) => owner.root === target)) throw Error("Scoped owner plan disagrees with the registry");
      console.log(JSON.stringify({changed: false, owner: row}));
      committed = true;
      break;
    }
    const others = {...planned, owners: planned.owners.filter((owner: {root: string}) => owner.root !== target)};
    if (repair.added.length !== 1 || planned.owners.length !== original.owners.length + 1 || json(others) !== before) throw Error("Scoped reconciliation would change rows outside the WFC owner");
    writeFileSync(pending, json(planned));
    chmodSync(pending, statSync(registry).mode);
    if (read() !== before) continue;
    renameSync(pending, registry);
    const actual = JSON.parse(read());
    if (json(actual.owners.find((owner: {root: string}) => owner.root === target)) !== json(row)) throw Error("Published WFC owner was superseded");
    console.log(JSON.stringify({changed: true, before: original.owners.length, after: actual.owners.length, owner: row}));
    committed = true;
  }
  if (!committed) throw Error("WFC owner registry changed through five scoped attempts");
  const verification = checkBuildOwnerRegistry(root, [target]);
  if (verification.findings.length) throw Error(JSON.stringify(verification.findings));
  console.log(JSON.stringify({scope: target, findings: 0, owners: verification.owners}));
} finally {
  rmSync(pending, {force: true});
  rmdirSync(scratch);
}
