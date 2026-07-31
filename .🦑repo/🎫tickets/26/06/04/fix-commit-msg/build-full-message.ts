import { readFileSync, writeFileSync } from "node:fs";
import { spawnSync } from "node:child_process";
import { extractBundleDateLineFromCommit, formatBundleDateLine, formatBundleSubject, normalizeBundleScopeLabel, pathsFromNumstatRow, labelPathTokens } from "../../../../../../repo/lib/js/src/commit.ts";
import {
  gitRangeNumstat,
  shouldSkipPathForUloc,
  sumGitLangDeltas,
  accumulateGitDeltasFromNumstat,
  gitDeltaLineTotal,
  buildMicroCommitMetricsForRange,
  formatMicroCommitMetricsLines,
  formatBundleUlocSuffix,
} from "../../../../../../repo/lib/js/src/uloc-metrics.ts";

const root = "/Users/ueli/Documents/compose";
const wip = "8fd25bc8c6a8ee50ebc2951b9dc5ec7f2f5aa354";
const head = "HEAD";
const microHead = "🐙ueli🎆26🌙06☀️04🚩";

type GitDeltaSum = { added: number; removed: number; edited: number };
type DaySection = { dateLine: string; bullets: string[] };
type BundleDef = { label: string; keywords: string[]; pathTokens: string[] };

const BUNDLE_DEFS: BundleDef[] = [
  { label: "🏘️compose🗃️fixtures", keywords: ["metabolism", "fixture", "initialkit", "kit diff", "neo4j", "validation snapshot", "hash cases", "diff corpora", "meshurl", "palette fixture"], pathTokens: ["fixtures", "metabolism"] },
  { label: "🏘️compose✍️sketchpad", keywords: ["sketchpad", "design diagram", "diagram replacement", "boot.tsx", "us-00", "store workflow"], pathTokens: ["sketchpad"] },
  {
    label: "🏘️compose",
    keywords: ["compose rust", "compose js", "compose/react", "graphql", "target schema", "golden-schema", "typology", "subscription", "kit store", "grasshopper", "rhino", "family entity", "lib.rs", "compose/client", "compose/rs", "compose/site"],
    pathTokens: ["compose"],
  },
  { label: "🎬presentation📽️", keywords: ["presentation", "projektetage", "reveal", "disposition", "morph", "slide", "chapter", "fullscreen tile", "embodiment"], pathTokens: ["presentation"] },
  {
    label: "🧩puzzle🎮play",
    keywords: ["puzzle", "board play", "board→puzzle", "marquee", "vortex", "topology board", "5d topology", "peer sync", "brush tool", "nakagin 2d", "nakagin 3d", "puzzle2d", "fill brush", "wires 5d"],
    pathTokens: ["puzzle"],
  },
  {
    label: "📐cad🪟spatial",
    keywords: ["cad", "spatial", "brep", "curve", "extrude", "scene package", "construct query", "model-definition", "elements scene", "elements geometry", "topologic", "spatial play", "spatial doctrine", "cell-complex", "chevrotain"],
    pathTokens: ["cad"],
  },
  { label: "🥅framework", keywords: ["@framework", "framework/", "playground", "platform shell", "playgroundcontroller", "uinode", "product/base"], pathTokens: ["framework"] },
  { label: "🖱️ui⚛️react", keywords: ["@semio-tech/ui-react", "ui/react", "engagement", "ghost", "golden layout", "window fill", "storybook", "scrollbar", "selection/hover", "context-menu", "platform settings"], pathTokens: ["ui", "react"] },
  { label: "🖱️ui🎨assets", keywords: ["svg icon", "ui/assets", "ui/styling", "vite-elements", "globals-ui"], pathTokens: ["ui", "assets"] },
  { label: "🗺️gis🧭map", keywords: ["gis", "maplibre", "mvt", "vector tile", "figure-ground", "mapvector", "ancestor map", "lod tier"], pathTokens: ["gis"] },
  { label: "♾️infinite", keywords: ["infinite", "cavas", "vello", "world r3f"], pathTokens: ["infinite"] },
  { label: "📐mathematical", keywords: ["mathematical", "graph normal", "graph port"], pathTokens: ["mathematical"] },
  { label: "🧠reasoning", keywords: ["reasoning"], pathTokens: ["reasoning"] },
  { label: "🧰repo📚js", keywords: ["repo/lib", "repo/mcp", "commit bundle", "commit check", "micro-commit", "script.ts", "project.json", "devcontainer", "launch.json", "ticket", "agents skill", ".repo/"], pathTokens: ["repo"] },
  { label: "🫡agents", keywords: ["agents.md", "agent instructions", "goals mcp"], pathTokens: ["agents"] },
  { label: "🌐workspace🧩other", keywords: [], pathTokens: [] },
];

function git(args: string[]): string {
  const r = spawnSync("git", args, { cwd: root, encoding: "utf8" });
  return r.status === 0 ? (r.stdout ?? "").trim() : "";
}

function add(a: GitDeltaSum, b: GitDeltaSum): GitDeltaSum {
  return { added: a.added + b.added, removed: a.removed + b.removed, edited: a.edited + b.edited };
}

function formatBundleHeaderLine(label: string, total: GitDeltaSum): string {
  return `${normalizeBundleScopeLabel(label)}${formatBundleUlocSuffix(total)}`;
}

function bundleIndexForLabel(label: string): number {
  const i = BUNDLE_DEFS.findIndex((b) => b.label === label);
  if (i < 0) throw new Error(`unknown bundle ${label}`);
  return i;
}

function assignBulletToBundle(bullet: string): string {
  const lower = bullet.toLowerCase();
  if (/\bcad\b|spatial play|brep|extrude|construct query|model-definition/.test(lower)) return "📐cad🪟spatial";
  if (/presentation|projektetage|reveal\.js|disposition|morph/.test(lower)) return "🎬presentation📽️";
  if (/maplibre|mvt|vector tile|figure-ground|gis layer|ancestor map/.test(lower)) return "🗺️gis🧭map";
  if (/ghost provider|golden layout|engagement input|@ui\/react/.test(lower)) return "🖱️ui⚛️react";
  if (/svg icon|storybook|scrollbar|vite-elements/.test(lower)) return "🖱️ui🎨assets";
  const scored = BUNDLE_DEFS.slice(0, -1).map((def, i) => {
    let score = 0;
    for (const kw of def.keywords) {
      if (kw && lower.includes(kw.toLowerCase())) score += kw.length > 8 ? 3 : 1;
    }
    return { i, score };
  });
  scored.sort((a, b) => b.score - a.score);
  const top = scored[0];
  if (!top || top.score === 0) return BUNDLE_DEFS[BUNDLE_DEFS.length - 1]!.label;
  return BUNDLE_DEFS[top.i]!.label;
}

function assignPathToBundle(path: string): number {
  const pl = path.toLowerCase();
  if (pl.includes("fixtures") || pl.includes("metabolism")) return bundleIndexForLabel("🏘️compose🗃️fixtures");
  if (pl.includes("sketchpad")) return bundleIndexForLabel("🏘️compose✍️sketchpad");
  let best = BUNDLE_DEFS.length - 1;
  let bestScore = 0;
  for (let i = 0; i < BUNDLE_DEFS.length - 1; i++) {
    const tokens = BUNDLE_DEFS[i]!.pathTokens.length ? BUNDLE_DEFS[i]!.pathTokens : labelPathTokens(BUNDLE_DEFS[i]!.label);
    if (tokens.length === 0) continue;
    if (!tokens.every((t) => pl.includes(t))) continue;
    if (tokens.length === 1 && tokens[0] === "compose" && (pl.includes("fixtures") || pl.includes("sketchpad"))) continue;
    const score = tokens.length;
    if (score > bestScore) {
      bestScore = score;
      best = i;
    }
  }
  return best;
}

function parseChronology(text: string): DaySection[] {
  const days: DaySection[] = [];
  let current: DaySection | null = null;
  for (const raw of text.split("\n")) {
    const line = raw.trim();
    if (!line) continue;
    if (/^Signed-off-by:/i.test(line)) break;
    const day = /^🎆(\d{2}🌙\d{2}☀️\d{2})(?:📊uloc.*)?$/u.exec(line);
    if (day) {
      current = { dateLine: day[0]!, bullets: [] };
      days.push(current);
      continue;
    }
    const bullet = /^- (.+)$/.exec(line) ?? /^(?:\p{Extended_Pictographic}|\S).*$/u.exec(line);
    if (!bullet || !current) continue;
    const text = bullet[1] ?? line;
    if (text.startsWith("📊uloc") || text.startsWith("🧾")) continue;
    const normalized = text.startsWith("-") ? text.slice(2).trim() : text.replace(/^- /, "");
    const emojiBullet = normalized.match(/^\p{Extended_Pictographic}/u) ? normalized : `🔧${normalized}`;
    current.bullets.push(emojiBullet.startsWith("🔧") && /^[\p{Extended_Pictographic}]/u.test(normalized) ? normalized : emojiBullet);
  }
  return days;
}

function parseLegacyBundles(text: string): Map<string, Map<string, string[]>> {
  const out = new Map<string, Map<string, string[]>>();
  let bundle: string | null = null;
  let day: string | null = null;
  for (const raw of text.split("\n")) {
    const line = raw.trim();
    if (!line || line.startsWith("🐙")) continue;
    if (/^📊uloc➕/.test(line) && !/^🎆/.test(line)) break;
    if (/^Signed-off-by:/i.test(line)) break;
    const dayM = /^🎆(\d{2}🌙\d{2}☀️\d{2})/u.exec(line.replace(/📊uloc.*$/u, "").trim());
    if (dayM) {
      day = dayM[0]!;
      continue;
    }
    if (line.match(/^\p{Extended_Pictographic}/u) && !line.startsWith("🎆") && !line.startsWith("-")) {
      const scope = normalizeBundleScopeLabel(line.replace(/📊uloc.*$/u, ""));
      if (scope && !scope.startsWith("📊")) {
        bundle = scope;
        if (!out.has(bundle)) out.set(bundle, new Map());
        continue;
      }
    }
    if (!bundle || !day) continue;
    if (line.startsWith("-")) continue;
    const bullets = out.get(bundle)!;
    if (!bullets.has(day)) bullets.set(day, []);
    bullets.get(day)!.push(line);
  }
  return out;
}

function mergeBulletsIntoBundles(chronology: DaySection[], legacy: Map<string, Map<string, string[]>>): Map<string, Map<string, Set<string>>> {
  const merged = new Map<string, Map<string, Set<string>>>();
  const add = (label: string, day: string, bullet: string) => {
    const norm = normalizeBundleScopeLabel(label);
    if (!merged.has(norm)) merged.set(norm, new Map());
    const days = merged.get(norm)!;
    if (!days.has(day)) days.set(day, new Set());
    days.get(day)!.add(bullet);
  };
  for (const [label, days] of legacy) {
    for (const [day, bullets] of days) for (const b of bullets) add(label, day, b);
  }
  for (const section of chronology) {
    for (const b of section.bullets) {
      const bullet = b.replace(/^- /, "").trim();
      add(assignBulletToBundle(bullet), section.dateLine, bullet);
    }
  }
  return merged;
}

function compareDaysDesc(a: string, b: string): number {
  return a < b ? 1 : a > b ? -1 : 0;
}

function reconcileListedDayUloc(listedDays: string[], header: GitDeltaSum, raw: Map<string, GitDeltaSum>): Map<string, GitDeltaSum> {
  if (listedDays.length === 0) return new Map();
  const weights = listedDays.map((d) => Math.max(1, gitDeltaLineTotal(raw.get(d) ?? { added: 0, removed: 0, edited: 0 })));
  const totalW = weights.reduce((a, b) => a + b, 0);
  const out = new Map<string, GitDeltaSum>();
  const acc = { added: 0, removed: 0, edited: 0 };
  for (let i = 0; i < listedDays.length; i++) {
    const w = weights[i]! / totalW;
    const isLast = i === listedDays.length - 1;
    const d: GitDeltaSum = isLast
      ? {
          added: header.added - acc.added,
          removed: header.removed - acc.removed,
          edited: header.edited - acc.edited,
        }
      : {
          added: Math.round(header.added * w),
          removed: Math.round(header.removed * w),
          edited: Math.round(header.edited * w),
        };
    out.set(listedDays[i]!, d);
    acc.added += d.added;
    acc.removed += d.removed;
    acc.edited += d.edited;
  }
  return out;
}

const rawMsg = readFileSync("/tmp/head-msg.txt", "utf8");
const footerIdx = rawMsg.indexOf("📊uloc➕42518");
const legacyPart = footerIdx > 0 ? rawMsg.slice(0, footerIdx) : rawMsg.split("Signed-off-by:")[0]!;
const chronPart = rawMsg.includes("🎆26🌙06☀️02\n- ") ? rawMsg.slice(rawMsg.indexOf("🎆26🌙06☀️02\n- ")) : rawMsg.slice(footerIdx > 0 ? footerIdx : 0);

const chronology = parseChronology(chronPart);
const legacy = parseLegacyBundles(legacyPart);
const merged = mergeBulletsIntoBundles(chronology, legacy);

const bundleLabels = [...merged.keys()].map((k) => {
  const def = BUNDLE_DEFS.find((d) => normalizeBundleScopeLabel(d.label) === k);
  return def?.label ?? k;
});

const n = BUNDLE_DEFS.length;
const headers = BUNDLE_DEFS.map(() => ({ added: 0, removed: 0, edited: 0 }));
const dateDeltas: Map<string, GitDeltaSum>[] = BUNDLE_DEFS.map(() => new Map());

for (const row of gitRangeNumstat(root, wip, head)) {
  const rowPaths = pathsFromNumstatRow(row.path);
  if (!rowPaths.length || rowPaths.every((p) => shouldSkipPathForUloc(root, p))) continue;
  const chunk = sumGitLangDeltas(accumulateGitDeltasFromNumstat(root, [{ path: row.path, added: row.added, removed: row.removed }]));
  if (gitDeltaLineTotal(chunk) === 0) continue;
  const bi = assignPathToBundle(rowPaths[0] ?? row.path);
  headers[bi] = add(headers[bi]!, chunk);
  let sha: string | null = null;
  for (const p of rowPaths) {
    sha = git(["log", "-1", "--format=%H", `${wip}..${microHead}`, "--", p]) || null;
    if (sha) break;
  }
  if (!sha) sha = git(["log", "-1", "--format=%H", `${wip}..${head}`, "--", rowPaths[0] ?? row.path]) || null;
  let dateLine = sha ? extractBundleDateLineFromCommit(git(["log", "-1", "--format=%s", sha]), git(["log", "-1", "--format=%B", sha])) : null;
  const bundleLabel = BUNDLE_DEFS[bi]!.label;
  const bundleDays = merged.get(normalizeBundleScopeLabel(bundleLabel));
  const fallback = bundleDays ? [...bundleDays.keys()].sort(compareDaysDesc)[0] : "🎆26🌙06☀️04";
  if (!dateLine) dateLine = fallback ?? "🎆26🌙06☀️04";
  const m = dateDeltas[bi]!;
  m.set(dateLine, add(m.get(dateLine) ?? { added: 0, removed: 0, edited: 0 }, chunk));
}

const ranked = BUNDLE_DEFS.map((def, i) => ({
  def,
  i,
  total: gitDeltaLineTotal(headers[i]!),
  label: def.label,
}))
  .filter((r) => {
    const norm = normalizeBundleScopeLabel(r.label);
    return gitDeltaLineTotal(headers[r.i]!) > 0 || (merged.get(norm)?.size ?? 0) > 0;
  })
  .sort((a, b) => b.total - a.total);

const lines: string[] = [formatBundleSubject({ alias: "ueli", emoji: "🐙", name: "Ueli Saluz", email: "ueli@semio-tech.com" }, new Date("2026-06-04")), ""];

for (let ri = 0; ri < ranked.length; ri++) {
  const { def, i: bi } = ranked[ri]!;
  const label = normalizeBundleScopeLabel(def.label);
  let sections = merged.get(label);
  if (!sections) {
    sections = new Map([["🎆26🌙06☀️04", new Set([`🔧Work in ${def.label}`])]]);
  }
  lines.push(formatBundleHeaderLine(def.label, headers[bi]!));
  const listedDays = [...sections.keys()].sort(compareDaysDesc);
  const dayUloc = reconcileListedDayUloc(listedDays, headers[bi]!, dateDeltas[bi]!);
  for (const dateLine of listedDays) {
    const bulletSet = sections.get(dateLine)!;
    lines.push(formatBundleDateLine(dateLine, dayUloc.get(dateLine) ?? { added: 0, removed: 0, edited: 0 }));
    for (const b of bulletSet) lines.push(b.startsWith("🔧") || /^\p{Extended_Pictographic}/u.test(b) ? b : `🔧${b}`);
  }
  if (ri < ranked.length - 1) lines.push("");
}

const metrics = formatMicroCommitMetricsLines(buildMicroCommitMetricsForRange(root, wip, head));
if (metrics.length) lines.push(...metrics, "");
lines.push("Signed-off-by: Ueli Saluz <ueli@semio-tech.com>");
const message = `${lines.join("\n")}\n`;

let sum: GitDeltaSum = { added: 0, removed: 0, edited: 0 };
for (const h of headers) sum = add(sum, h);
const footer = buildMicroCommitMetricsForRange(root, wip, head);
let fa = 0,
  fe = 0,
  fr = 0;
for (const r of footer) {
  fa += r.added;
  fe += r.edited;
  fr += r.removed;
}
console.error(`[build] bundles=${ranked.length} range ➕${fa}✏️${fe}➖${fr} sum headers ➕${sum.added}✏️${sum.edited}➖${sum.removed}`);

writeFileSync(`${root}/.git/compose-commit-message`, message);
process.stdout.write(message);
