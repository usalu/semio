import { createHash } from "node:crypto";
import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import * as toml from "@iarna/toml";
import { rustTokens, rustTokenPairs } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";

const ticket = resolve(import.meta.dir, "..");
const texts = new Map<string, string>();
const read = (path: string) => {
  if (!texts.has(path)) texts.set(path, readFileSync(resolve(process.cwd(), path), "utf8"));
  return texts.get(path)!;
};
const inventory = JSON.parse(readFileSync(resolve(ticket, "🗑️generated/stdio-oracle-composition-inventory.json"), "utf8"));
const analysis = JSON.parse(readFileSync(resolve(ticket, "🗑️generated/stdio-oracle-provider-analysis.json"), "utf8"));
const old = inventory.retired.library;
const hostOwner = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test";
const digest = (text: string) => createHash("sha256").update(text).digest("hex");
const families = inventory.sources.filter(({ source }) => source.endsWith("/🦀️.rs") && source.split("/").length === 6 && !source.includes("/⚖️law/")).map(({ source }) => {
  const owner = source.slice(0, -"/🦀️.rs".length), module = owner.split("/").at(-1).replace(/^[^a-z]+/u, "");
  const name = `semio-s-plugin-stdio-${module}-test-oracle`;
  return { owner, module, source, package: { implementation: "rust", package: name, library: name.replaceAll("-", "_"), path: `${owner}/📦️packages/🦀️rust`, features: ["oracles"] }, sources: inventory.sources.filter((row) => row.source.startsWith(`${owner}/`)).map(({ source }) => source), dependencies: [] as string[] };
});
const providers = analysis.groups.map(({ owner, mounts }) => {
  const cargo = toml.parse(read(`${owner}/📦️packages/🦀️rust/Cargo.toml`)) as { package: { name: string } };
  const name = `${cargo.package.name}-test-oracle`;
  const namespace = mounts[0].namespace.split("::").slice(0, mounts[0].namespace.split("::").indexOf("standards")).join("::");
  return { owner, namespace, source: `${owner}/🔮️oracles/🦀️.rs`, package: { implementation: "rust", package: name, library: name.replaceAll("-", "_"), path: `${owner}/🔮️oracles/📦️packages/🦀️rust`, features: ["oracles"] }, mounts: mounts.map(({ source, namespace, sha256 }) => ({ source, namespace: namespace.slice(namespace.indexOf("standards")), sha256 })), dependencies: [] as string[] };
});
const grammarOwner = `${inventory.retired.owner}/🔤️part21`;
const grammarName = "semio-s-plugin-stdio-part21-test-oracle";
const grammar = { owner: grammarOwner, source: `${grammarOwner}/🦀️.rs`, function: "decode_string_literal", originalSource: providers.find(({ namespace }) => namespace === "artifacts::step").mounts.find(({ namespace }) => namespace.endsWith("reference")).source, package: { implementation: "rust", package: grammarName, library: grammarName.replaceAll("-", "_"), path: `${grammarOwner}/📦️packages/🦀️rust`, features: ["oracles"] }, bodySha256: "" };
const grammarText = read(grammar.originalSource), grammarTokens = rustTokens(grammarText), pairs = rustTokenPairs(grammarTokens);
const functionAt = grammarTokens.findIndex(({ text }, index) => text === "fn" && grammarTokens[index + 1]?.text === grammar.function);
let bodyAt = functionAt;
while (grammarTokens[bodyAt].text !== "{") bodyAt++;
grammar.bodySha256 = digest(grammarTokens.slice(bodyAt, pairs.get(bodyAt) + 1).map(({ text }) => text).join(" "));
const bindings = [
  { previous: `${old}::law`, current: "semio_repo_test_host::law", package: "semio-repo-test-host" },
  ...families.map(({ module, package: pkg }) => ({ previous: `${old}::${module}`, current: pkg.library, package: pkg.package })),
  ...providers.map(({ namespace, package: pkg }) => ({ previous: `${old}::${namespace}`, current: pkg.library, package: pkg.package })),
].sort((a, b) => b.previous.length - a.previous.length);
const rootFacts = new Map<string, string[]>();
const roots = (text: string) => {
  if (rootFacts.has(text)) return rootFacts.get(text)!;
  const tokens = rustTokens(text);
  const names = [...new Set(tokens.flatMap(({ text }, index) => tokens[index + 1]?.text === "::" ? [text] : []))];
  rootFacts.set(text, names);
  return names;
};
const optionalNames = Object.keys(inventory.dependencies).filter((name) => name !== "semio-repo-test-host");
for (const family of families) family.dependencies = ["semio-repo-test-host", ...optionalNames.filter((name) => family.sources.some((source) => roots(read(source)).includes(name.replaceAll("-", "_"))))].sort();
for (const provider of providers) {
  const text = provider.mounts.map(({ source }) => read(source)).join("\n");
  provider.dependencies = ["semio-repo-test-host", ...optionalNames.filter((name) => roots(text).includes(name.replaceAll("-", "_"))), ...families.filter(({ module }) => text.includes(`crate::${module}::`)).map(({ package: pkg }) => pkg.package), ...(["artifacts::step", "artifacts::ifc"].includes(provider.namespace) ? [grammarName] : [])].sort();
}
const callers = analysis.callers.map(({ source, artifactOwner }) => ({ source, owner: artifactOwner, rewrites: bindings.filter(({ previous }) => read(source).includes(`${previous}::`) || read(source).includes(`${previous};`)), sha256: digest(read(source)) }));
const artifactBindings = [...new Set(callers.map(({ owner }) => owner))].sort().map((owner) => ({ owner, packages: [...new Set(callers.filter((row) => row.owner === owner).flatMap(({ rewrites }) => rewrites.map(({ package: name }) => name)).filter((name) => name !== "semio-repo-test-host"))].sort() })).filter(({ packages }) => packages.length);
const sourceRewrites = (source: string) => {
  const provider = providers.find(({ mounts }) => mounts.some((row) => row.source === source));
  return [
    ...bindings.filter(({ previous }) => read(source).includes(previous)),
    ...(source.includes("/⚖️law/") && read(source).includes("use semio_repo_test_host::{") ? [{ previous: "use semio_repo_test_host::{", current: "use crate::{", package: "semio-repo-test-host" }] : []),
    ...families.filter(({ module }) => read(source).includes(`crate::${module}::`)).map(({ module, package: pkg }) => ({ previous: `crate::${module}`, current: pkg.library, package: pkg.package })),
    ...(read(source).includes("crate::law::") ? [{ previous: "crate::law", current: "semio_repo_test_host::law", package: "semio-repo-test-host" }] : []),
    ...(provider ? [{ previous: `crate::${provider.namespace}::standards`, current: "crate::standards", package: provider.package.package }] : []),
    ...(read(source).includes("crate::artifacts::step::standards::v_ap214::reference::part21::decode_string_literal") && provider.namespace !== "artifacts::step" ? [{ previous: "crate::artifacts::step::standards::v_ap214::reference::part21::decode_string_literal", current: `${grammar.package.library}::decode_string_literal`, package: grammarName }] : []),
  ].sort((a, b) => b.previous.length - a.previous.length);
};
const tokenDigest = (source: string) => {
  const text = read(source), tokens = rustTokens(text), pairs = rustTokenPairs(tokens);
  if (source !== grammar.originalSource) return digest(tokens.map(({ text }) => text).join(" "));
  const start = tokens.findIndex(({ text }, index) => text === "fn" && tokens[index + 1]?.text === grammar.function);
  let body = start;
  while (tokens[body].text !== "{") body++;
  return digest([...tokens.slice(0, start - 1), ...tokens.slice(pairs.get(body)! + 1)].map(({ text }) => text).join(" "));
};
const sources = inventory.sources.filter(({ source }) => source !== `${inventory.retired.owner}/🦀️.rs`).map(({ source }) => ({ source, destination: source.includes("/⚖️law/") ? source.replace(`${inventory.retired.owner}/⚖️law`, `${hostOwner}/⚖️law`) : source, rewrites: sourceRewrites(source), sha256: digest(read(source)) }));
const mounts = inventory.mounts.map(({ source }) => ({ source, destination: source, rewrites: sourceRewrites(source), sha256: digest(read(source)), tokenSha256: tokenDigest(source) }));
const fixture = {
  schemaVersion: 2, owner: inventory.owner, package: inventory.package, retired: inventory.retired,
  neutralLaw: { owner: `${hostOwner}/⚖️law`, package: "semio-repo-test-host", library: "semio_repo_test_host", source: `${hostOwner}/🦀️.rs`, module: "law" },
  families, grammar, providers, sources, mounts, callers, artifactBindings,
  dependencies: inventory.dependencies, features: inventory.features,
  contributions: inventory.contributions.map(({ path, value }) => ({ path, fields: Object.fromEntries(Object.entries(value).filter(([key]) => key !== "oracleHostPackages").map(([key, value]) => [key, digest(JSON.stringify(value))])), hosts: value.oracleHostPackages ?? [] })),
  internalModules: [{ source: grammar.originalSource, module: "part21" }, { source: grammar.originalSource, module: "ladder" }, { source: providers.find(({ namespace }) => namespace === "artifacts::ifc").mounts.find(({ namespace }) => namespace.endsWith("reference") && namespace.includes("v2x3")).source, module: "part21" }],
};
writeFileSync(resolve(process.cwd(), fixture.owner, "🧫️fixtures/🧩️composition/🔣️.json"), `${JSON.stringify(fixture, null, 2)}\n`);
writeFileSync(resolve(ticket, "📓️2026-10-02-stdio-oracle-exact-provider-contract.md"), ["# Exact Stdio Oracle Provider Contract", "", "The closed authored fixture requires artifact providers to depend only on lower artifact-free families and the neutral Repo host. Hub consumes every provider; generated original artifact hosts select only actual declared providers/families. Production sources are held until coordinated start.", "", "## Exact Family Packages", "", ...families.map(({ owner, package: pkg, dependencies }) => `- \`${owner}\`: \`${pkg.package}\`; actual dependency roots ${dependencies.map((name) => `\`${name}\``).join(", ")}.`), "", "## Exact Artifact Providers", "", ...providers.map(({ owner, namespace, package: pkg, mounts, dependencies }) => `- \`${owner}\`: \`${pkg.package}\`; namespace \`${namespace}\`, ${mounts.length} original mounts; actual dependency roots ${dependencies.map((name) => `\`${name}\``).join(", ")}.`), "", "## Explicit Consumer Artifact Bindings", "", ...artifactBindings.map(({ owner, packages }) => `- \`${owner}\`: ${packages.map((name) => `\`${name}\``).join(", ")}.`), "", "## Source Preservation", "", `The complete caller cohort remains ${callers.length} actual source inputs; ${mounts.length} original mounts and ${sources.length} shared/law implementation inputs retain source digests and explicit owned-import substitutions. STEP's exact decode function body is retained independently by \`${grammar.bodySha256}\` in the shared lower grammar. These source witnesses do not replace original native scenarios and provider/family unit laws.`, ""].join("\n"));
console.log(JSON.stringify({ families: families.length, providers: providers.length, mounts: mounts.length, callers: callers.length, artifactBindings: artifactBindings.length, grammar }));
