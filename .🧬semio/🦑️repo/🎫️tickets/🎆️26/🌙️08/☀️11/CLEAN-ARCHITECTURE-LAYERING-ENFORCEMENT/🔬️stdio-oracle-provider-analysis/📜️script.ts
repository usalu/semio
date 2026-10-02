import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { inspectRustModuleGraphFacts, inspectRustMutationMetadataFacts, rustTokens } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";

const ticket = resolve(import.meta.dir, "..");
const inventory = JSON.parse(readFileSync(resolve(ticket, "🗑️generated/stdio-oracle-composition-inventory.json"), "utf8"));
const read = (path: string) => readFileSync(resolve(process.cwd(), path), "utf8");
const library: string = inventory.retired.library;
const callers = inventory.callers.map(({ source }: { source: string }) => {
  const text = read(source), facts = inspectRustModuleGraphFacts(text), aliases = inspectRustMutationMetadataFacts(text).crateAliases.filter(({ source }) => source === library || source.startsWith(`${library}::`));
  const tokens = rustTokens(text);
  const rootAliases = new Set([library, ...aliases.filter(({ source }) => source === library).map(({ alias }) => alias)]);
  const moduleReferences = [...new Set(tokens.flatMap((token, index) => rootAliases.has(token.text) && tokens[index + 1]?.text === "::" ? [tokens[index + 2]?.text] : []).filter(Boolean))].sort();
  return { source, artifactOwner: source.split("/").slice(0, 5).join("/"), imports: facts.uses.filter(({ specifier }) => specifier === library || specifier.startsWith(`${library}::`)), aliases, moduleReferences, lawOnly: moduleReferences.length === 1 && moduleReferences[0] === "law" };
});
const mounted = inventory.mounts.map((mount: { namespace: string; source: string; path: string; sha256: string }) => {
  const text = read(mount.source), tokens = rustTokens(text), facts = inspectRustModuleGraphFacts(text);
  const crateReferences = [...new Set(tokens.flatMap((token, index) => token.text === "crate" && tokens[index + 1]?.text === "::" ? [tokens.slice(index, index + 14).map(({ text }) => text).join(" ")] : []))];
  return { ...mount, artifactOwner: mount.source.split("/").slice(0, 5).join("/"), imports: facts.uses, crateReferences };
});
const groups = [...new Set(mounted.map(({ artifactOwner }) => artifactOwner))].sort().map((owner) => ({ owner, mounts: mounted.filter(({ artifactOwner }) => artifactOwner === owner), callers: callers.filter(({ artifactOwner }) => artifactOwner === owner) }));
const data = { schemaVersion: 1, callers, groups, lawOnly: callers.filter(({ lawOnly }) => lawOnly), docOnly: callers.filter(({ moduleReferences }) => moduleReferences.length === 0) };
writeFileSync(resolve(ticket, "🗑️generated/stdio-oracle-provider-analysis.json"), `${JSON.stringify(data, null, 2)}\n`);
const report = ["# Stdio Oracle Provider And Neutral Law Source Evidence", "", "Actual Rust tokens discard comments and strings; module/use/alias facts retain lexical scope and conditional participation. This is a source partition witness, separate from native compile and original runtime receipts.", "", `- Complete original textual caller inputs: ${callers.length}`, `- Actual law-only Rust callers: ${data.lawOnly.length}`, `- Comment/doc-only library references: ${data.docOnly.length}`, `- Mounted artifact provider owners: ${groups.length}`, "", "## Exact Provider Cohorts", "", ...groups.flatMap(({ owner, mounts }) => [`### ${owner}`, "", ...mounts.map(({ namespace, source }) => `- \`${namespace}\` from \`${source}\``), ""]), "## Actual Non-Law Caller Imports", "", ...callers.filter(({ lawOnly, moduleReferences }) => !lawOnly && moduleReferences.length > 0).flatMap(({ source, imports, moduleReferences }) => [`- \`${source}\`: root references \`${moduleReferences.join(", ")}\``, ...imports.map(({ specifier, modulePath, conditional }) => `  - \`${specifier}\` at \`${modulePath.join("::") || "root"}\`${conditional ? " (conditional)" : ""}`)]), "", "## All Actual Law-Only Callers", "", ...data.lawOnly.map(({ source }) => `- \`${source}\``), "", "## Comment/Documentation Only Library References", "", ...data.docOnly.map(({ source }) => `- \`${source}\``), ""];
writeFileSync(resolve(ticket, "📓️2026-10-02-stdio-oracle-provider-source-evidence.md"), report.join("\n"));
console.log(JSON.stringify({ callers: callers.length, lawOnly: data.lawOnly.length, docOnly: data.docOnly.length, providerOwners: groups.length }));
