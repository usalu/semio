#!/usr/bin/env bun
/** 🔎️ Audits repository path emojis against fixed-name contracts and an independent emoji oracle. */
import emojiRegex from "emoji-regex";
import { existsSync, lstatSync, readFileSync, renameSync, writeFileSync } from "node:fs";
import { basename, dirname, join, posix } from "node:path";
import { createFixedContractResolver, fileKindIdForSourcePath, leadingEmojiIdentity, loadCatalogTaxonomy, pathEmojiStatuteFindings, semanticDirectoryKindId } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";

type Entry = { path: string; name: string; parent: string; directory: boolean };
type Finding = { kind: "missing" | "generic" | "presentation" | "spacing" | "duplicate" | "oracle"; path: string; sibling?: string; emoji?: string };

const root = process.cwd();
const taxonomy = loadCatalogTaxonomy();
const raw = Bun.spawnSync(["git", "ls-files", "-co", "--exclude-standard", "-z"], { cwd: root }).stdout.toString();
const observedInventory = raw.split("\0").filter(Boolean).map((path) => path.replace(/\/$/u, "").normalize("NFC")).flatMap((path) => {
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
const underReservedSubtree = (path: string): boolean => path.split("/").slice(0, -1).some(isReservedSubtreeName);
const packageContext = (path: string): { packageRoot: boolean; ecosystemId?: string; siblingFixedFilenameContractIds?: readonly string[] } => {
  const parent = dirname(path).replaceAll("\\", "/");
  const segments = parent.split("/");
  let packages = -1;
  for (let index = segments.length - 1; index >= 0; index--) {
    if (semanticDirectoryKindId(segments[index]!, taxonomy) === "packages") {
      packages = index;
      break;
    }
  }
  const ecosystemId = packages >= 0 ? segments[packages + 1] : undefined;
  const packageManifest = basename(path) === "package.json" || existsSync(join(root, parent, "package.json"));
  const siblingFixedFilenameContractIds = existsSync(join(root, parent, "📋️project.json")) ? ["nx-project-manifest"] : undefined;
  return { packageRoot: packages === segments.length - 2 || packageManifest, ecosystemId, siblingFixedFilenameContractIds };
};
const entries: Entry[] = [
  ...[...directories].map((path) => ({ path, name: path.split("/").at(-1)!, parent: dirname(path).replaceAll("\\", "/"), directory: true })),
  ...files.map((path) => ({ path, name: path.split("/").at(-1)!, parent: dirname(path).replaceAll("\\", "/"), directory: false })),
].filter((entry) => !excluded(entry.path));

const statuteEntries = entries.map((entry) => {
  const context = packageContext(entry.path);
  const fixed = entry.directory
    ? resolver.directoryIdsForPath(entry.path, context).length > 0
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

const counts = Object.fromEntries(["missing", "generic", "presentation", "spacing", "duplicate", "oracle"].map((kind) => [kind, findings.filter((finding) => finding.kind === kind).length]));
const audit = { files: files.length, directories: directories.size, governed: statuteEntries.filter((entry) => !entry.reserved).length, counts, findings };

type PlannedMove = { source: string; destination: string; sourceName: string; destinationName: string; nodeKind: "directory" | "file" };

const emoji = (value: string): string => value.includes("\u200D") || value.includes("\uFE0F") ? value : `${value}\uFE0F`;
const discriminators = [
  "🔴", "🟠", "🟡", "🟢", "🔵", "🟣", "🟤", "⚫", "⚪", "🟥", "🟧", "🟨", "🟩", "🟦", "🟪", "🟫",
  "🌱", "🌿", "🍀", "🌵", "🌲", "🌳", "🌴", "🌾", "🌻", "🌹", "🌷", "🪻", "🍎", "🍊", "🍋", "🍐",
  "🐙", "🦀", "🐬", "🦋", "🐝", "🐞", "🦊", "🐺", "🐼", "🐨", "🦁", "🐯", "🐸", "🐧", "🦉", "🦅",
  "💎", "🔮", "🧿", "🪁", "🪄", "🎈", "🎨", "🎯", "🧭", "🛟", "🛰", "🚀", "🛸", "⚓", "⛵", "🏔",
].map(emoji);
const semanticRules: readonly [RegExp, string][] = [
  [/access|barriere|a11y/iu, "♿"], [/admin|authority|berechtig/iu, "🪪"], [/agent|bot/iu, "🤖"], [/artifact|artefakt/iu, "🗿"], [/asset|media|bild|image/iu, "🖼"],
  [/auth|login|credential/iu, "🔐"], [/backup|sicher/iu, "💾"], [/bind|link|verknüpf|association/iu, "🔗"], [/block|bauteil|component|element/iu, "🧩"], [/bootstrap|build|builder|generator/iu, "🏗"],
  [/cache|resident|storage/iu, "💾"], [/cancel|reject|remove|delete|retire/iu, "🚫"], [/catalog|katalog|registry|verzeichnis/iu, "📇"], [/client|frontend/iu, "💻"], [/command|befehl/iu, "🎮"],
  [/config|option|setting/iu, "🎚"], [/contract|vertrag/iu, "🤝"], [/copy|duplicate|paste|kopie/iu, "📋"], [/date|calendar|tag/iu, "📅"], [/diff|delta|vergleich/iu, "🔺"],
  [/document|dokument|page|seite|paper/iu, "📃"], [/download|export|output|ausgabe/iu, "📤"], [/editor|edit|rename|eingabe/iu, "✏"], [/engine|runtime|system/iu, "⚙"], [/entry|main|index/iu, "🚪"],
  [/event|signal|wire|transport/iu, "📡"], [/example|beispiel|demo/iu, "🧺"], [/fixture|probe|sample/iu, "🧫"], [/flow|workflow|prozess/iu, "🌊"], [/font|text|label/iu, "🔤"],
  [/graph|tree|baum|dag/iu, "🌳"], [/health|status|valid|check|prüf/iu, "✅"], [/host|server|backend/iu, "🖥"], [/(?:^|[-_.])id(?:$|[-_.])|identity|session|metadata/iu, "🪪"], [/import|upload|eingang/iu, "📥"],
  [/layout|size|maß|dimension/iu, "📐"], [/license|recht/iu, "⚖"], [/lifecycle|lifetime|recycle/iu, "♻"], [/linux/iu, "🐧"], [/lock|mutex/iu, "🔒"],
  [/logo|brand/iu, "🪧"], [/macos|darwin|apple/iu, "🍎"], [/manifest|package|pack/iu, "📦"], [/map|terrain|world|karte/iu, "🗺"], [/math|number|parameter|count/iu, "🔢"],
  [/mcp|plugin|module|extension/iu, "🔌"], [/merge|transform|mutation|change|update|patch/iu, "🔀"], [/model|typolog/iu, "🏛"], [/network|web|http|api|graphql/iu, "🕸"], [/open|launch/iu, "📬"],
  [/panel|pane|tab/iu, "📌"], [/presence|user|benutzer/iu, "👥"], [/print|druck/iu, "🖨"], [/project|projekt/iu, "🎯"], [/query|search|recherche|find|filter/iu, "🔎"],
  [/read|reader/iu, "📖"], [/report|bericht|summary/iu, "📓"], [/scene|slide|präsent/iu, "🎞"], [/schema|typed|type/iu, "🧬"], [/scope|zoom|focus|overview|überblick/iu, "🔭"],
  [/segment|slice|split/iu, "✂"], [/shell|terminal|cli/iu, "🐚"], [/space|planet/iu, "🪐"], [/style|theme|css/iu, "🎨"], [/sync|replicat/iu, "🔄"],
  [/task|job|thread/iu, "🧵"], [/template|vorlage/iu, "🧾"], [/test|spec|oracle/iu, "🧪"], [/ticket|issue/iu, "🎫"], [/tool|utility|werkzeug/iu, "🛠"], [/appendix|anhang/iu, "📎"],
  [/ui|interface|surface|oberfläche/iu, "🖱"], [/version|release/iu, "🔖"], [/warning|error|failure/iu, "⚠"], [/wasm|bridge/iu, "🌉"], [/window|fenster/iu, "🪟"],
];
const stableHash = (value: string): number => {
  let hash = 2166136261;
  for (const byte of Buffer.from(value.normalize("NFC"))) hash = Math.imul(hash ^ byte, 16777619) >>> 0;
  return hash;
};
const semanticEmoji = (value: string): string => emoji(semanticRules.find(([pattern]) => pattern.test(value))?.[1] ?? discriminators[stableHash(value) % discriminators.length]!);
const fileKindEmoji = (path: string): string => {
  const kindId = fileKindIdForSourcePath(path, taxonomy);
  return kindId ? taxonomy.fileKinds[kindId]!.emoji : emoji("📝");
};
const cleanRest = (value: string): string => value.replace(/^\s+/u, "");

const planMoves = (): PlannedMove[] => {
  const affected = new Set(findings.filter((finding) => finding.kind !== "oracle").map((finding) => finding.path));
  const findingsByPath = new Map<string, Set<string>>();
  for (const finding of findings) {
    if (finding.kind === "oracle") continue;
    const kinds = findingsByPath.get(finding.path) ?? new Set<string>();
    kinds.add(finding.kind);
    findingsByPath.set(finding.path, kinds);
  }
  const children = new Map<string, Entry[]>();
  for (const entry of entries.filter((candidate) => !statuteByPath.get(candidate.path)?.reserved)) {
    const siblings = children.get(entry.parent) ?? [];
    siblings.push(entry);
    children.set(entry.parent, siblings);
  }
  const destinationNameByPath = new Map<string, string>();
  for (const [parent, siblings] of [...children].sort(([left], [right]) => Buffer.from(left).compare(Buffer.from(right)))) {
    const used = new Set(siblings.filter((entry) => !affected.has(entry.path)).map((entry) => fold(leadingEmojiIdentity(entry.name).emoji)));
    for (const entry of siblings.filter((candidate) => affected.has(candidate.path)).sort((left, right) => Buffer.from(left.path).compare(Buffer.from(right.path)))) {
      const kinds = findingsByPath.get(entry.path)!;
      const identity = leadingEmojiIdentity(entry.name);
      const rest = cleanRest(identity.rest || entry.name);
      const semantic = semanticEmoji(rest);
      let prefix: string;
      if (kinds.has("missing")) prefix = entry.directory ? semantic : `${fileKindEmoji(entry.path)}${semantic}`;
      else if (kinds.has("generic")) prefix = entry.directory ? semantic : `${fileKindEmoji(entry.path)}${semantic}`;
      else prefix = `${identity.emoji}${semantic}`;
      let folded = fold(prefix);
      if (used.has(folded)) {
        const hash = stableHash(`${entry.name}\0${entry.directory ? "directory" : "file"}`);
        for (let offset = 0; used.has(folded); offset++) {
          prefix += discriminators[(hash + offset) % discriminators.length]!;
          folded = fold(prefix);
        }
      }
      used.add(folded);
      const destinationName = `${prefix}${rest}`.normalize("NFC");
      if (destinationName === entry.name) throw new Error(`Finding did not produce a rename for ${entry.path}.`);
      if (Buffer.byteLength(destinationName) > 240) throw new Error(`Destination basename exceeds 240 bytes: ${destinationName}`);
      destinationNameByPath.set(entry.path, destinationName);
    }
    void parent;
  }
  const variants = new Map<string, Set<string>>();
  for (const entry of entries) {
    const destinationName = destinationNameByPath.get(entry.path);
    if (!destinationName) continue;
    const names = variants.get(entry.name) ?? new Set<string>();
    names.add(destinationName);
    variants.set(entry.name, names);
  }
  for (const [sourceName, names] of variants) {
    if (names.size < 2) continue;
    const shortest = [...names].sort((left, right) => leadingEmojiIdentity(left).emoji.length - leadingEmojiIdentity(right).emoji.length || Buffer.from(left).compare(Buffer.from(right)))[0]!;
    const parsed = leadingEmojiIdentity(shortest), hash = stableHash(`${sourceName}\0global`);
    const canonical = `${parsed.emoji}${discriminators[hash % discriminators.length]}${discriminators[(hash >>> 6) % discriminators.length]}${parsed.rest}`;
    for (const entry of entries) if (entry.name === sourceName && destinationNameByPath.has(entry.path)) destinationNameByPath.set(entry.path, canonical);
  }
  const finalPath = (source: string): string => {
    const sourceSegments = source.split("/");
    const destinationSegments: string[] = [];
    const originalSegments: string[] = [];
    for (const segment of sourceSegments) {
      originalSegments.push(segment);
      destinationSegments.push(destinationNameByPath.get(originalSegments.join("/")) ?? segment);
    }
    return destinationSegments.join("/");
  };
  const moves = [...destinationNameByPath].map(([source, destinationName]) => {
    const entry = entries.find((candidate) => candidate.path === source)!;
    return { source, destination: finalPath(source), sourceName: entry.name, destinationName, nodeKind: entry.directory ? "directory" : "file" };
  }).sort((left, right) => Buffer.from(left.source).compare(Buffer.from(right.source)));
  const moveBySource = new Map(moves.map((move) => [move.source, move]));
  const projected = statuteEntries.map((entry) => ({ ...entry, path: moveBySource.get(entry.path)?.destination ?? finalPath(entry.path) }));
  const remaining = pathEmojiStatuteFindings(projected, taxonomy.pathEmojiPolicy.genericEmojiIdentities);
  if (remaining.length > 0) throw new Error(`Plan leaves ${remaining.length} statute findings: ${JSON.stringify(remaining.slice(0, 20))}`);
  return moves;
};

const replaceReferences = (moves: readonly PlannedMove[]): number => {
  const full = [...moves].sort((left, right) => right.source.length - left.source.length);
  const basenameDestinations = new Map<string, Set<string>>();
  for (const move of moves) {
    const destinations = basenameDestinations.get(move.sourceName) ?? new Set<string>();
    destinations.add(move.destinationName);
    basenameDestinations.set(move.sourceName, destinations);
  }
  const uniqueNames = [...basenameDestinations].filter(([, destinations]) => destinations.size === 1).map(([sourceName, destinations]) => [sourceName, [...destinations][0]!] as const).sort((left, right) => right[0].length - left[0].length);
  const historical = (path: string): boolean => path === ".🧬semio" || path.startsWith(".🧬semio/") || path === ".🧬️semio" || path.startsWith(".🧬️semio/");
  const bulkyReserved = new Set(["🗿️artifacts", "🖼️asset", "🖼️assets", "🧫️fixtures", "🧪️fixtures", "📚️examples", "🤖️generated", "🌐️public"].map(fold));
  const bulky = (path: string): boolean => path.split("/").some((segment) => bulkyReserved.has(fold(segment)));
  let changed = 0;
  for (const [index, path] of files.entries()) {
    if (historical(path) || bulky(path) || !existsSync(join(root, path))) continue;
    const stat = lstatSync(join(root, path));
    if (!stat.isFile() || stat.size > 8 * 1024 * 1024) continue;
    const bytes = readFileSync(join(root, path));
    if (bytes.subarray(0, Math.min(bytes.length, 8192)).includes(0)) continue;
    let content = bytes.toString("utf8"), updated = content;
    for (const move of full) updated = updated.replaceAll(move.source, move.destination);
    for (const [sourceName, destinationName] of uniqueNames) {
      if (leadingEmojiIdentity(sourceName).emoji) updated = updated.replaceAll(sourceName, destinationName);
      else {
        const escaped = sourceName.replace(/[.*+?^${}()|[\]\\]/gu, "\\$&");
        updated = updated.replace(new RegExp(`(^|[\\/\\\\\"'\\x60])${escaped}(?=([\\/\\\\\"'\\x60]|$))`, "gmu"), `$1${destinationName}`);
      }
    }
    if (updated !== content) { writeFileSync(join(root, path), updated); changed++; }
    if (existsSync(join(root, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️04/☀️08/ENFORCE-UNIQUE-SEMANTIC-EMOJIS-ACROSS-REPOSITORY/⛔️cancel"))) throw new Error("Cancelled by ticket marker.");
    if (index % 500 === 0) process.stderr.write(`references ${index}/${files.length}\r`);
  }
  process.stderr.write(`references ${files.length}/${files.length}\n`);
  return changed;
};

const applyMoves = (moves: readonly PlannedMove[]): void => {
  const ordered = [...moves].sort((left, right) => right.source.split("/").length - left.source.split("/").length || Buffer.from(right.source).compare(Buffer.from(left.source)));
  for (const [index, move] of ordered.entries()) {
    const source = join(root, move.source);
    const destination = join(root, dirname(move.source), move.destinationName);
    if (!existsSync(source)) throw new Error(`Move source disappeared: ${move.source}`);
    if (existsSync(destination)) throw new Error(`Move destination already exists: ${posix.join(dirname(move.source), move.destinationName)}`);
    renameSync(source, destination);
    if (index % 100 === 0) process.stderr.write(`moves ${index}/${ordered.length}\r`);
  }
  process.stderr.write(`moves ${ordered.length}/${ordered.length}\n`);
};

/** 🧰️ Finishes interrupted plan moves without overwriting either identity. */
const reconcileMoves = (moves: readonly PlannedMove[]): number => {
  let moved = 0;
  const ordered = [...moves].sort((left, right) => right.source.split("/").length - left.source.split("/").length || Buffer.from(right.source).compare(Buffer.from(left.source)));
  for (const move of ordered) {
    if (move.nodeKind === "directory" && move.source.startsWith("🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/internal")) continue;
    const source = join(root, move.source);
    const destination = join(root, dirname(move.source), move.destinationName);
    if (!existsSync(source) || existsSync(destination)) continue;
    renameSync(source, destination);
    moved += 1;
  }
  return moved;
};

/** 🔒️ Restores externally fixed leaves after their parents have reached final identity. */
const restoreFixedLeaves = (moves: readonly PlannedMove[]): { restored: number; references: number } => {
  const selected = moves.filter((move) => move.nodeKind === "file" && (move.sourceName === "package.json" || move.sourceName === "tsconfig.json" || [
    "🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/README.md",
    "🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/LICENSE.md",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/LICENSE.md",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/LICENSE.md",
  ].includes(move.source)));
  const replacements: readonly (readonly [string, string])[] = selected.map((move) => [move.destination, posix.join(dirname(move.destination), move.sourceName)] as const);
  let restored = 0;
  for (const [source, destination] of replacements) {
    const sourceAbsolute = join(root, source), destinationAbsolute = join(root, destination);
    if (!existsSync(sourceAbsolute) || existsSync(destinationAbsolute)) continue;
    renameSync(sourceAbsolute, destinationAbsolute);
    restored += 1;
  }
  const currentInventory = Bun.spawnSync(["git", "ls-files", "-co", "--exclude-standard", "-z"], { cwd: root }).stdout.toString().split("\0").filter(Boolean).filter((path) => existsSync(join(root, path)));
  let references = 0;
  for (const path of currentInventory) {
    const stat = lstatSync(join(root, path));
    if (!stat.isFile() || stat.size > 8 * 1024 * 1024) continue;
    const bytes = readFileSync(join(root, path));
    if (bytes.subarray(0, Math.min(bytes.length, 8192)).includes(0)) continue;
    const content = bytes.toString("utf8");
    let updated = content;
    for (const [source, destination] of replacements) updated = updated.replaceAll(source, destination);
    updated = updated.replaceAll("package.json", "package.json").replaceAll("tsconfig.json", "tsconfig.json");
    if (updated !== content) {
      writeFileSync(join(root, path), updated);
      references += 1;
    }
  }
  return { restored, references };
};

/** ⚖️ Resolves the final test-role collision without rewriting historical ticket evidence. */
const resolveFinalCollision = (): { moved: boolean; references: number } => {
  const sourceName = "🧪️schema-parity", destinationName = "🧪️⚖️schema-parity";
  const parent = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host";
  const source = join(root, parent, sourceName), destination = join(root, parent, destinationName);
  let moved = false;
  if (existsSync(source) && !existsSync(destination)) {
    renameSync(source, destination);
    moved = true;
  } else if (!existsSync(destination)) throw new Error(`Final collision source is absent: ${posix.join(parent, sourceName)}`);
  const references = [
    "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️.wit",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust/🦀️.rs",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️🌷️taxonomy.json",
  ].reduce((count, path) => {
    const absolute = join(root, path), content = readFileSync(absolute, "utf8"), updated = content.replaceAll(sourceName, destinationName);
    if (updated === content) return count;
    writeFileSync(absolute, updated);
    return count + 1;
  }, 0);
  return { moved, references };
};

/** 🐹️ Restores compiler-mandated ASCII Go import directories while retaining emoji filenames. */
const restoreGoImportDirectories = (): { moved: number; references: number } => {
  const moves: PlannedMove[] = JSON.parse(readFileSync(join(import.meta.dir, "🗑️generated/🧭️rename-plan.json"), "utf8")).moves;
  const directories = moves.filter((move) => move.nodeKind === "directory" && move.source.startsWith("🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/internal"))
    .sort((left, right) => right.destination.split("/").length - left.destination.split("/").length);
  let moved = 0;
  for (const move of directories) {
    const source = join(root, move.destination), destination = join(root, dirname(move.destination), move.sourceName);
    if (!existsSync(source) || existsSync(destination)) continue;
    renameSync(source, destination);
    moved += 1;
  }
  const replacements = [
    ["internal/command", "internal/command"],
    ["internal/eventstore", "internal/eventstore"],
    ["internal/glob", "internal/glob"],
    ["internal/graphql", "internal/graphql"],
    ["internal/humanize", "internal/humanize"],
    ["internal/id", "internal/id"],
    ["internal/ignore", "internal/ignore"],
    ["internal/mcp", "internal/mcp"],
    ["internal/mcpserver", "internal/mcpserver"],
    ["internal/search", "internal/search"],
    ["internal/templatefunc", "internal/templatefunc"],
    ["internal/yaml", "internal/yaml"],
  ] as const;
  const current = Bun.spawnSync(["git", "ls-files", "-co", "--exclude-standard", "-z"], { cwd: root }).stdout.toString().split("\0").filter(Boolean);
  let references = 0;
  for (const path of current) {
    const absolute = join(root, path);
    let stat;
    try { stat = lstatSync(absolute); } catch { continue; }
    if (!stat.isFile() || stat.size > 8 * 1024 * 1024) continue;
    const bytes = readFileSync(absolute);
    if (bytes.subarray(0, Math.min(bytes.length, 8192)).includes(0)) continue;
    const content = bytes.toString("utf8");
    let updated = content;
    for (const [source, destination] of replacements) updated = updated.replaceAll(source, destination);
    if (updated === content) continue;
    writeFileSync(absolute, updated);
    references += 1;
  }
  return { moved, references };
};

/** 🧷️ Repairs exact active references whose target directories gained collision discriminators. */
const repairActiveReferences = (): { references: number } => {
  const replacements = [
    ["🔌️plugin/📇️registry", "🔌️plugin/📇️📇️registry"],
    ["🖱️ui/🎨️styling", "🖱️ui/🎨️🟠️styling"],
    ["🏗️builder/🧬️schema/🔣️.json", "🏗️builder/🧬️🧬️schema/🔣️.json"],
    ["🧦️release/🧬️schema/🔣️.json", "🧦️release/🧬️🧬️schema/🔣️.json"],
    ["📦️payload/🧬️schema/🔣️.json", "📦️payload/🧬️🧬️schema/🔣️.json"],
  ] as const;
  const current = Bun.spawnSync(["git", "ls-files", "-co", "--exclude-standard", "-z"], { cwd: root }).stdout.toString().split("\0").filter(Boolean);
  let references = 0;
  for (const path of current) {
    if (path === ".🧬semio" || path.startsWith(".🧬semio/")) continue;
    const absolute = join(root, path);
    let stat;
    try { stat = lstatSync(absolute); } catch { continue; }
    if (!stat.isFile() || stat.size > 8 * 1024 * 1024) continue;
    const bytes = readFileSync(absolute);
    if (bytes.subarray(0, Math.min(bytes.length, 8192)).includes(0)) continue;
    const content = bytes.toString("utf8");
    let updated = content;
    for (const [source, destination] of replacements) updated = updated.replaceAll(source, destination);
    if (updated === content) continue;
    writeFileSync(absolute, updated);
    references += 1;
  }
  return { references };
};

const command = Bun.argv[2] ?? "audit";
if (command === "audit") process.stdout.write(`${JSON.stringify(audit, null, 2)}\n`);
else if (command === "resolve-final-collision") process.stdout.write(`${JSON.stringify(resolveFinalCollision(), null, 2)}\n`);
else if (command === "restore-go-import-directories") process.stdout.write(`${JSON.stringify(restoreGoImportDirectories(), null, 2)}\n`);
else if (command === "repair-active-references") process.stdout.write(`${JSON.stringify(repairActiveReferences(), null, 2)}\n`);
else {
  const moves: PlannedMove[] = command === "reconcile"
    ? JSON.parse(readFileSync(join(import.meta.dir, "🗑️generated/🧭️rename-plan.json"), "utf8")).moves
    : planMoves();
  if (command === "plan") process.stdout.write(`${JSON.stringify({ audit, moves }, null, 2)}\n`);
  else if (command === "apply") {
    const changedReferences = replaceReferences(moves);
    applyMoves(moves);
    process.stdout.write(`${JSON.stringify({ moves: moves.length, changedReferences }, null, 2)}\n`);
  } else if (command === "reconcile") {
    const reconciled = reconcileMoves(moves);
    const fixed = restoreFixedLeaves(moves);
    process.stdout.write(`${JSON.stringify({ reconciled, ...fixed }, null, 2)}\n`);
  } else throw new Error(`Unknown command ${JSON.stringify(command)}. Expected audit, plan, apply, reconcile, resolve-final-collision, restore-go-import-directories, or repair-active-references.`);
}
