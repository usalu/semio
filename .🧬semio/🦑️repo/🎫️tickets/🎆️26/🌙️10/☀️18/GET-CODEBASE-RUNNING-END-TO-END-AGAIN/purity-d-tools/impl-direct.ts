import { writeFileSync } from "node:fs";
import { spawnSync } from "node:child_process";
import { taxonomyImplementationFilesystemFindings } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🗂️file-kind-resolution/🟦️.ts";
import { loadCatalogTaxonomy } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
const root = "/Users/ueli/Documents/semio";
const listing = spawnSync("git", ["-c", "core.fsmonitor=false", "ls-files", "-z", "--cached", "--others", "--exclude-standard"], { cwd: root, maxBuffer: 1 << 30, encoding: "utf8" }).stdout;
const files = new Set<string>(), directories = new Set<string>();
for (const path of listing.split("\0")) { if (!path) continue; files.add(path); for (let cut = path.lastIndexOf("/"); cut > 0; cut = path.lastIndexOf("/", cut - 1)) directories.add(path.slice(0, cut)); }
const admitted = (path: string, nodeKind: "directory" | "file"): boolean => { const set = nodeKind === "file" ? files : directories; return set.has(path.normalize("NFC")) || set.has(path); };
const findings = await taxonomyImplementationFilesystemFindings(root, loadCatalogTaxonomy(), { admitted });
const kinds: Record<string, number> = {};
for (const f of findings) kinds[f.breachId] = (kinds[f.breachId] ?? 0) + 1;
console.log(findings.length, JSON.stringify(kinds));
writeFileSync(process.argv[2]!, findings.map((f) => [f.breachId, f.path, f.actualBasename ?? "", f.expectedBasename ?? ""].join("\t")).join("\n"));
