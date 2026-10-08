import { readFileSync, writeFileSync, mkdirSync, existsSync } from "node:fs";
import { join, resolve, dirname } from "node:path";

const root = resolve(import.meta.dirname, "../../../../../../../..");
const ticket = dirname(import.meta.dirname);
const generated = join(ticket, "🗑️generated");
const hubRoot = join(root, "🌎️hub");
const hub: any = Bun.TOML.parse(readFileSync(join(hubRoot, "Cargo.toml"), "utf8"));
const paneOwners = new Map<string, string>();
for (const member of hub.workspace.members.filter((member: string) => member.startsWith("🧩️compositions/"))) {
  const manifest: any = Bun.TOML.parse(readFileSync(join(hubRoot, member, "Cargo.toml"), "utf8"));
  const owner = resolve(hubRoot, member, "../..", "🔣️.json").slice(root.length + 1);
  for (const pane of manifest.package.metadata?.semio?.playground ?? []) {
    if (paneOwners.has(pane.variant)) throw new Error(`duplicate owner for ${pane.variant}`);
    paneOwners.set(pane.variant, owner);
  }
}
const runtime = JSON.parse(readFileSync(join(root, "🏢️semio-tech/🎡️play/🔨️modules/🧩️runtime/🔣️.json"), "utf8"));
const panes = runtime.groups.flatMap((group: any) => group.panes).map((pane: any) => {
  const owner = paneOwners.get(pane.variant);
  if (!owner) throw new Error(`missing owner for ${pane.variant}`);
  return { variant: pane.variant, owner };
});
const owners = [...new Set(panes.map((pane: any) => pane.owner))] as string[];
const results: any[] = [];
mkdirSync(join(generated, "feature-oracles"), { recursive: true });

for (const owner of owners) {
  const manifestPath = resolve(root, dirname(owner), "📦️packages/🦀️rust/Cargo.toml");
  const manifest: any = Bun.TOML.parse(readFileSync(manifestPath, "utf8"));
  const workspacePath = resolve(dirname(manifestPath), manifest.package.workspace, "Cargo.toml");
  const workspace: any = Bun.TOML.parse(readFileSync(workspacePath, "utf8"));
  const appSource = [resolve(dirname(manifestPath), manifest.lib.path), resolve(root, dirname(owner), "🦀️.rs"), resolve(root, dirname(owner), "🪪️manifest/🎪️demonstrator/🦀️.rs")].filter(existsSync).map((path) => readFileSync(path, "utf8")).join("\n");
  const selected = new Set<string>(["default"]);
  for (const feature of selected) for (const child of manifest.features?.[feature] ?? []) selected.add(child);
  const artifacts = Object.entries(manifest.dependencies ?? {}).filter(([name]) => name.startsWith("semio-s-artifact-")).map(([name, declared]: [string, any]) => {
    const dependency = declared.workspace ? { ...workspace.workspace.dependencies[name], ...declared } : declared;
    const base = declared.workspace ? dirname(workspacePath) : dirname(manifestPath);
    const childPath = resolve(base, dependency.path, "Cargo.toml");
    const child: any = Bun.TOML.parse(readFileSync(childPath, "utf8"));
    const enabled = !declared.optional || selected.has(`dep:${name}`) || [...selected].some((value) => value.startsWith(`${name}/`));
    const features = new Set<string>([...(dependency.features ?? []), ...(dependency["default-features"] === false ? [] : ["default"]), ...[...selected].filter((value) => value.startsWith(`${name}/`)).map((value) => value.slice(name.length + 1))]);
    for (const feature of features) for (const value of child.features?.[feature] ?? []) features.add(value);
    const appConsumed = new RegExp(`${name.replaceAll("-", "_")}::(?:editor|viewer)\\b`).test(appSource);
    return { name, manifest: childPath.slice(root.length + 1), appConsumed, enabled, assemblyRequired: appConsumed && child.features?.["component-app-assembly"] !== undefined, assemblyEnabled: enabled && features.has("component-app-assembly") };
  });
  const oracleDirectory = join(generated, "feature-oracles", manifest.package.name);
  mkdirSync(oracleDirectory, { recursive: true });
  const oracleManifest = join(oracleDirectory, "Cargo.toml");
  writeFileSync(oracleManifest, `[package]\nname = "audit-${manifest.package.name}"\nversion = "0.0.0"\nedition = "2024"\n[lib]\npath = ${JSON.stringify(resolve(dirname(manifestPath), manifest.lib.path))}\n[workspace]\nresolver = ${JSON.stringify(workspace.workspace.resolver)}\n[dependencies]\n${manifest.package.name} = { path = ${JSON.stringify(dirname(manifestPath))} }\n`);
  const args = ["tree", "--offline", "--manifest-path", oracleManifest, "-p", manifest.package.name, "--target", "wasm32-wasip2", "--edges", "normal", "--prefix", "none", "--format", "{p}\t{f}"];
  const child = Bun.spawn(["cargo", ...args], { cwd: root, stdout: "pipe", stderr: "pipe" });
  const [stdout, stderr, status] = await Promise.all([new Response(child.stdout).text(), new Response(child.stderr).text(), child.exited]);
  writeFileSync(join(generated, "feature-oracles", `${manifest.package.name}.txt`), `${stdout}\n${stderr}`);
  const oracle = artifacts.map((artifact) => ({ name: artifact.name, assemblyEnabled: stdout.split(/\r?\n/).some((line) => line.startsWith(`${artifact.name} `) && line.split(/\t|\\t/)[1]?.replace(/ \(\*\)$/, "").split(",").includes("component-app-assembly")) }));
  const row = { owner, package: manifest.package.name, manifest: manifestPath.slice(root.length + 1), paneCount: panes.filter((pane: any) => pane.owner === owner).length, selectedFeatures: [...selected], artifacts, cargoStatus: status, oracle };
  results.push(row);
  writeFileSync(join(generated, "browser-feature-closure.json"), JSON.stringify(results, null, 2));
  console.log(`[DEBUG] ${row.package}: ${row.paneCount} panes; cargo=${status}; assembly=${artifacts.filter((artifact) => artifact.assemblyEnabled).length}/${artifacts.filter((artifact) => artifact.assemblyRequired && artifact.enabled).length}`);
}

const omissions = results.flatMap((row) => row.artifacts.filter((artifact: any) => artifact.enabled && artifact.assemblyRequired && !artifact.assemblyEnabled).map((artifact: any) => `${row.package}:${artifact.name}`));
const mismatches = results.flatMap((row) => row.artifacts.filter((artifact: any) => artifact.enabled && artifact.assemblyRequired && artifact.assemblyEnabled !== row.oracle.find((value: any) => value.name === artifact.name)?.assemblyEnabled).map((artifact: any) => `${row.package}:${artifact.name}`));
console.log(`[DEBUG] ${results.length} owners, ${panes.length} panes; omissions=${JSON.stringify(omissions)}; Cargo mismatches=${JSON.stringify(mismatches)}`);
if (omissions.length || mismatches.length || results.some((row) => row.cargoStatus !== 0)) process.exitCode = 1;
