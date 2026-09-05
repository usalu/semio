#!/usr/bin/env bun
/** 🔎️ Audits repository path emojis against fixed-name contracts and an independent emoji oracle. */
import emojiRegex from "emoji-regex";
import { existsSync, lstatSync, readFileSync } from "node:fs";
import { basename, dirname, join } from "node:path";
import { createFixedContractResolver, leadingEmojiIdentity, loadCatalogTaxonomy, pathEmojiStatuteFindings, semanticDirectoryKindId } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";

type Entry = { path: string; name: string; parent: string; directory: boolean };
type Finding = { kind: "missing" | "generic" | "presentation" | "spacing" | "duplicate" | "multiple" | "reserved-emoji" | "oracle"; path: string; sibling?: string; emoji?: string };

if ((Bun.argv[2] ?? "audit") !== "audit") throw new Error("Disabled: automatic emoji selection and bulk basename replacement corrupted the workspace. Only read-only audit is permitted; names must be handpicked individually.");

const root = process.cwd();
const requestedPrefix = (Bun.argv[3] ?? "").replace(/^\.\//u, "").replace(/\/$/u, "").normalize("NFC");
const taxonomy = JSON.parse(readFileSync(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"), "utf8")) as ReturnType<typeof loadCatalogTaxonomy>;
const gitPathspecs = requestedPrefix
  ? [requestedPrefix]
  : [".", ...taxonomy.pathEmojiPolicy.reservedSubtreeDirectoryNames.map((name) => `:(exclude)${name}`)];
const raw = Bun.spawnSync(["git", "ls-files", "-co", "--exclude-standard", "-z", "--", ...gitPathspecs], { cwd: root }).stdout.toString();
const observedInventory = raw.split("\0").filter(Boolean).map((path) => path.replace(/\/$/u, "").normalize("NFC")).filter((path) => requestedPrefix === "" || path === requestedPrefix || path.startsWith(`${requestedPrefix}/`)).flatMap((path) => {
  try { return [{ path, directory: lstatSync(join(root, path)).isDirectory() }]; } catch { return []; }
});
const inventory = observedInventory.map((entry) => entry.path);
const files = observedInventory.filter((entry) => !entry.directory).map((entry) => entry.path);
const directories = new Set<string>(observedInventory.filter((entry) => entry.directory).map((entry) => entry.path));
for (const path of inventory) {
  let parent = dirname(path).replaceAll("\\", "/");
  while (parent !== "." && parent !== "") {
    directories.add(parent);
    parent = dirname(parent).replaceAll("\\", "/");
  }
}

const excluded = (path: string): boolean => {
  const normalized = `${path}/`;
  return Object.values(taxonomy.pathExclusions).some((rule) => normalized.startsWith(rule.path));
};

const oracleLeading = (value: string): string => {
  const match = emojiRegex().exec(value);
  return match?.index === 0 ? match[0] : "";
};
const fold = (value: string): string => value.replaceAll("\uFE0F", "").replaceAll("\uFE0E", "");
const reservedSubtrees = new Set(taxonomy.pathEmojiPolicy.reservedSubtreeDirectoryNames.map(fold));
const isReservedSubtreeName = (name: string): boolean => reservedSubtrees.has(fold(name)) || reservedSubtrees.has(fold(leadingEmojiIdentity(name).rest));
const resolver = createFixedContractResolver(taxonomy);
const packageContext = (path: string): { packageRoot: boolean; ecosystemId?: string; parentDirectoryKindId?: string; siblingFixedFilenameContractIds?: readonly string[] } => {
  const parent = dirname(path).replaceAll("\\", "/");
  const segments = parent.split("/");
  let packages = -1;
  for (let index = segments.length - 1; index >= 0; index--) {
    if (semanticDirectoryKindId(segments[index]!, taxonomy) === "packages") {
      packages = index;
      break;
    }
  }
  const adjacentEcosystem = ([
    ["Cargo.toml", "🦀️rust"],
    ["go.mod", "🐹️go"],
    ["package.json", "🟦️typescript"],
  ] as const).find(([manifest]) => basename(path) === manifest || existsSync(join(root, parent, manifest)))?.[1];
  const ecosystemId = packages >= 0 ? segments[packages + 1] : adjacentEcosystem;
  const siblingFixedFilenameContractIds = existsSync(join(root, parent, "📋️project.json")) ? ["nx-project-manifest"] : undefined;
  return { packageRoot: packages === segments.length - 2 || adjacentEcosystem !== undefined, ecosystemId, parentDirectoryKindId: semanticDirectoryKindId(basename(parent), taxonomy) ?? undefined, siblingFixedFilenameContractIds };
};
const fixedDirectoryIds = new Map([...directories].map((path) => [path, resolver.directoryIdsForPath(path, packageContext(path))] as const));
const fixedReservedSubtreeRoots = [...fixedDirectoryIds].filter(([, ids]) => ids.some((id) => taxonomy.fixedDirectoryContracts[id]?.descendants === "reserved")).map(([path]) => path);
const underReservedSubtree = (path: string): boolean => path.split("/").slice(0, -1).some(isReservedSubtreeName) || fixedReservedSubtreeRoots.some((reservedRoot) => path.startsWith(`${reservedRoot}/`));
const entries: Entry[] = [
  ...[...directories].map((path) => ({ path, name: path.split("/").at(-1)!, parent: dirname(path).replaceAll("\\", "/"), directory: true })),
  ...files.map((path) => ({ path, name: path.split("/").at(-1)!, parent: dirname(path).replaceAll("\\", "/"), directory: false })),
].filter((entry) => !excluded(entry.path));

const statuteEntries = entries.map((entry) => {
  const context = packageContext(entry.path);
  const fixed = entry.directory
    ? (fixedDirectoryIds.get(entry.path) ?? resolver.directoryIdsForPath(entry.path, context)).length > 0
    : resolver.filenameIdsForPath(entry.path, context).length > 0;
  return { path: entry.path, nodeKind: entry.directory ? "directory" as const : "file" as const, reserved: underReservedSubtree(entry.path) || entry.directory && isReservedSubtreeName(entry.name) || fixed };
});
const statuteByPath = new Map(statuteEntries.map((entry) => [entry.path, entry]));

const findings: Finding[] = [...pathEmojiStatuteFindings(statuteEntries, taxonomy.pathEmojiPolicy.genericEmojiIdentities)];
for (const entry of entries.filter((candidate) => !statuteByPath.get(candidate.path)?.reserved)) {
  const emoji = leadingEmojiIdentity(entry.name).first;
  const oracle = oracleLeading(entry.name);
  if (oracle && fold(emoji) !== fold(oracle)) findings.push({ kind: "oracle", path: entry.path, emoji: `${emoji} != ${oracle}` });
}

const counts = Object.fromEntries(["missing", "generic", "presentation", "spacing", "duplicate", "multiple", "reserved-emoji", "oracle"].map((kind) => [kind, findings.filter((finding) => finding.kind === kind).length]));
const audit = { files: files.length, directories: directories.size, governed: statuteEntries.filter((entry) => !entry.reserved).length, counts, findings };
process.stdout.write(`${JSON.stringify(audit, null, 2)}\n`);
