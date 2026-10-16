/** 🔬 Runs the real owner-admission observation (the function that throws the owner error) for chosen owners, offline. */
import { readFileSync } from "node:fs";
import { join } from "node:path";
const root = process.cwd();
const inf = await import(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs"));
const registry = JSON.parse(readFileSync(join(root, "📋️build-owners.json"), "utf8"));
for (const name of process.argv.slice(2)) {
  const owner = registry.owners.find((o: any) => o.name === name);
  if (!owner) { console.log(name, "NOT IN REGISTRY"); continue; }
  const selection = { kind: "tasks", tasks: [{ project: name, target: "check", configuration: "prototype" }] };
  try {
    const facts = await inf.discoverBuildOwnerAdmissionFactsV1(owner, ["check"], registry, { workspaceRoot: root, selection });
    console.log(name, "OK files:", facts.files.length, "dependencies:", facts.dependencies.length);
  } catch (e: any) { console.log(name, "FAIL:", String(e.message).slice(0, 300)); }
}
