import { spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, relative, resolve } from "node:path";
import * as toml from "@iarna/toml";

const root = process.cwd();
const oldOwner = "✏️s/🔌️plugins/🗄️stdio/🔮️oracles";
const owner = "🌎️hub/🧩️compositions/🗄️stdio/🔮️oracles";
const oldPackage = "semio-s-plugin-stdio-test-oracle";
const packageName = "semio-hub-stdio-test-oracle";
const oldLibrary = "semio_s_plugin_stdio_test_oracle";
const library = "semio_hub_stdio_test_oracle";
const output = resolve(import.meta.dir, "../🗑️generated/stdio-oracle-composition-inventory.json");
const read = (path: string) => readFileSync(resolve(root, path), "utf8");
const digest = (value: string) => createHash("sha256").update(value).digest("hex");
const canonical = (value: string) => value.replaceAll(oldLibrary, "$oracleLibrary").replaceAll(library, "$oracleLibrary").replaceAll(oldPackage, "$oraclePackage").replaceAll(packageName, "$oraclePackage").replaceAll(oldOwner, "$oracleOwner").replaceAll(owner, "$oracleOwner").replaceAll("pub(crate) mod part21", "pub mod part21");
const listed = (args: string[]) => {
  const result = spawnSync("rg", args, { cwd: root, encoding: "utf8", maxBuffer: 64 * 1024 * 1024 });
  if (result.status !== 0) throw new Error(result.stderr || `rg exited ${result.status}`);
  return result.stdout.trim().split("\n").filter(Boolean).sort();
};
const codeRoots = ["✏️s", "🌎️hub", "🧰️framework"];
const oldSources = listed(["--files", oldOwner]).filter((path) => path.endsWith(".rs"));
const lines = read(`${oldOwner}/🦀️.rs`).split("\n");
const namespaces: string[] = [];
const mounts: { namespace: string; source: string; path: string; sha256: string }[] = [];
for (let index = 0; index < lines.length; index++) {
  const line = lines[index];
  const declaration = line.match(/^(\s*)pub mod ([a-z0-9_]+) \{/u);
  if (declaration) {
    const depth = declaration[1].length / 4;
    namespaces.length = depth;
    namespaces.push(declaration[2]);
  }
  const mount = line.match(/#\[path = "([^"]+\.rs)"\]/u);
  if (!mount || !mount[1].includes("🗿️artifacts")) continue;
  const source = relative(root, resolve(root, oldOwner, mount[1])).replaceAll("\\", "/");
  mounts.push({ namespace: namespaces.join("::"), source, path: relative(resolve(root, owner), resolve(root, source)).replaceAll("\\", "/"), sha256: digest(canonical(read(source))) });
}
const callers = listed(["-l", oldLibrary, ...codeRoots, "-g", "*.rs"]);
const textualReferences = listed(["-l", `${oldLibrary}|${oldPackage}|${oldOwner}`, ...codeRoots, "-g", "*.rs", "-g", "*.ts", "-g", "*.mjs", "-g", "*.json", "-g", "Cargo.toml"]);
const contributionPaths = listed(["--files", "✏️s/🔌️plugins", "🌎️hub/🧩️compositions", "-g", "🔣️.json"]).filter((path) => path.endsWith("/🔮️oracles/🔣️.json"));
const contributions = contributionPaths.map((path) => ({ path, value: JSON.parse(read(path)) })).filter(({ path, value }) => path.startsWith("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/") || JSON.stringify(value).includes(oldOwner) || JSON.stringify(value).includes(oldPackage));
const cargo = toml.parse(read(`${oldOwner}/📦️packages/🦀️rust/Cargo.toml`));
const contract = {
  schemaVersion: 1,
  owner,
  package: { implementation: "rust", package: packageName, path: `${owner}/📦️packages/🦀️rust`, library, features: ["oracles"] },
  retired: { owner: oldOwner, package: oldPackage, library: oldLibrary },
  sources: oldSources.map((source) => ({ source, destination: source.replace(oldOwner, owner), sha256: digest(canonical(read(source))) })),
  mounts,
  callers: callers.map((source) => ({ source, sha256: digest(canonical(read(source))) })),
  dependencies: cargo.dependencies,
  features: cargo.features,
  contributions: contributions.map(({ path, value }) => ({ path, value })),
  textualReferences,
};
mkdirSync(dirname(output), { recursive: true });
writeFileSync(output, `${JSON.stringify(contract, null, 2)}\n`);
console.log(JSON.stringify({ output, mounts: mounts.length, sources: oldSources.length, callers: callers.length, contributions: contributions.length, textualReferences: textualReferences.length, nonStdioMounts: mounts.filter(({ source }) => !source.startsWith("✏️s/🔌️plugins/🗄️stdio/")) }, null, 2));
const artifactOwners = [...new Set(callers.map((path) => path.split("/").slice(0, 5).join("/")))].sort();
const fixture = {
  schemaVersion: 1,
  owner,
  package: contract.package,
  retired: contract.retired,
  sources: contract.sources,
  mounts,
  callers: contract.callers,
  dependencies: contract.dependencies,
  features: contract.features,
  artifactBindings: artifactOwners.map((owner) => ({
    owner,
    features: [...new Set(contributions.filter(({ path }) => owner === path.slice(0, -"/🔮️oracles/🔣️.json".length) || owner.startsWith(`${path.slice(0, -"/🔮️oracles/🔣️.json".length)}/`)).flatMap(({ value }) => value.oracleHostPackages ?? []).filter((entry) => entry.package === oldPackage).flatMap((entry) => entry.features ?? []))].sort(),
  })),
  contributions: contributions.map(({ path, value }) => ({
    path,
    fields: Object.fromEntries(Object.entries(value).filter(([key]) => key !== "oracleHostPackages").map(([key, field]) => [key, digest(canonical(JSON.stringify(field)))])),
    hosts: value.oracleHostPackages ?? [],
  })),
  internalModules: mounts.filter(({ source }) => /\/(?:📐️step\/🏅️standards\/🔖️ap214|🏗️ifc\/🏅️standards\/🔖️2x3)\/🔮️oracles\/🦀️\.rs$/u.test(source)).map(({ source }) => ({ source, module: "part21" })),
};
const fixturePath = resolve(root, owner, "🧫️fixtures/🧩️composition/🔣️.json");
mkdirSync(dirname(fixturePath), { recursive: true });
writeFileSync(fixturePath, `${JSON.stringify(fixture, null, 2)}\n`);
console.log(JSON.stringify({ fixturePath, artifactBindings: artifactOwners.length, internalModules: fixture.internalModules }));
const report = ["# Exact Stdio Oracle Source Inventory", "", "This authored inventory preserves the current source cohort before its coordinated cut. Counts reflect physical inputs rather than executed native cases.", "", `- Shared oracle Rust files: ${oldSources.length}`, `- Mounted artifact/standard/subset oracle files: ${mounts.length}`, `- Stdio Rust callers: ${callers.filter((path) => path.startsWith("✏️s/🔌️plugins/🗄️stdio/")).length}`, `- Complete canonical-library Rust callers: ${callers.length}`, `- Artifact case roots requiring explicit package contributions: ${artifactOwners.length}`, `- Involved contribution manifests: ${contributions.length}`, "", "## Exact Mounted Sources", "", ...mounts.map(({ namespace, source, path }) => `- \`${namespace}\`: \`${source}\` → higher assembly \`${path}\``), "", "## Exact Shared Source Destinations", "", ...contract.sources.map(({ source, destination }) => `- \`${source}\` → \`${destination}\``), "", "## Exact Canonical-Library Callers", "", ...callers.map((source) => `- \`${source}\``), "", "## Exact Contribution Inputs", "", ...contributions.map(({ path }) => `- \`${path}\``), "", "## All Authored Textual References", "", ...textualReferences.map((path) => `- \`${path}\``), ""];
writeFileSync(resolve(import.meta.dir, "../📓️2026-10-02-canonical-stdio-oracle-source-inventory.md"), report.join("\n"));
