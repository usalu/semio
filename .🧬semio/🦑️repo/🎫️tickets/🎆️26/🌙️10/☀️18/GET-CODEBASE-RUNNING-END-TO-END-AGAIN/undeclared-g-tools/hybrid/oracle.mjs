// undeclared-g hybrid oracle: the Seatbelt read oracle for declared sets too large for one literal per file. Same contract as the repository's read-oracle module (kernel-enforced EPERM for every workspace read outside the declared set, denials named by the unified log,
// learning rounds add the denied files), but the profile comes from the hybrid cover (grant a directory, deny its undeclared entries). The log parser and the capture are the module's own (imported, not copied).
import {existsSync, lstatSync, mkdtempSync, readdirSync, rmSync, writeFileSync} from "node:fs";
import {spawnSync} from "node:child_process";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {setTimeout as sleep} from "node:timers/promises";
import {hybridCoverV1, hybridGrantsV1} from "./cover.mjs";

const LIBRARY = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔭️read-oracle";
const {readSeatbeltDenialsV1} = await import(`${LIBRARY}/🍎️seatbelt/🟦️.ts`);
const {captureV1} = await import(`${LIBRARY}/🏃️run/🟦️.ts`);

const quoted = (text) => `"${text.replaceAll("\\", "\\\\").replaceAll('"', '\\"')}"`;
export const AMBIENT = ["node_modules", ".cache", ".git", "target", ".🧬semio/🦑️repo/⚡️cache"];

/** 🍎️ Renders the Seatbelt profile of a hybrid cover: deny the root, grant the rules, deny the exceptions, re-grant the ambient trees; directories stay listable (names are not contents). */
export function hybridProfileV1(root, cover, ambient) {
  const lines = ["(version 1)", "(allow default)", `(deny file-read-data (subpath ${quoted(root)}))`];
  for (const rule of cover.rules) lines.push(`(allow file-read-data (${rule.kind} ${quoted(rule.path === "." ? root : join(root, rule.path))}))`);
  for (const denial of cover.denials) lines.push(`(deny file-read-data (subpath ${quoted(join(root, denial))}))`);
  for (const tree of ambient) lines.push(`(allow file-read-data (subpath ${quoted(join(root, tree))}))`);
  lines.push("(allow file-read-data (vnode-type DIRECTORY))");
  return `${lines.join("\n")}\n`;
}

/** 📏️ Makes a cover fit the kernel's profile limit: while the kernel refuses the compiled profile, the denials of single FILES are relaxed (5 % per step, files before directories) and returned, so the report names exactly which undeclared entries were NOT enforced. */
export function fitCoverV1(root, cover, ambient, scratch) {
  const relaxed = [], isFile = (path) => { try { return lstatSync(join(root, path)).isFile(); } catch { return false; } };
  for (let step = 0; step < 120; step++) {
    const probe = join(scratch, "fit.sb");
    writeFileSync(probe, hybridProfileV1(root, cover, ambient));
    const run = spawnSync("sandbox-exec", ["-f", probe, "/usr/bin/true"], {encoding: "utf8"});
    if (run.status === 0 || !/exceeds maximum/u.test(run.stderr)) return {cover, relaxed};
    if (step < 6 && cover.denials.length) {
      const quota = Math.max(10, Math.ceil(cover.denials.length * 0.05)), drop = new Set(), order = [...cover.denials].sort((a, b) => Number(isFile(b)) - Number(isFile(a)));
      for (const path of order) { if (drop.size >= quota) break; drop.add(path); }
      relaxed.push(...drop);
      cover = {rules: cover.rules, denials: cover.denials.filter((path) => !drop.has(path))};
      continue;
    }
    const groups = new Map();
    for (const rule of cover.rules) if (rule.kind === "literal") { const directory = rule.path.slice(0, Math.max(0, rule.path.lastIndexOf("/"))); groups.set(directory, (groups.get(directory) ?? 0) + 1); }
    const largest = [...groups].filter(([directory, count]) => directory !== "" && count >= 3).sort((a, b) => b[1] - a[1]).slice(0, 15).map(([directory]) => directory);
    if (!largest.length) throw new Error("profile does not fit and nothing is left to collapse");
    const set = new Set(largest);
    for (const directory of largest) for (const name of lister(root)(directory) ?? []) { const path = `${directory}/${name}`; if (!cover.rules.some((rule) => rule.path === path)) relaxed.push(path); }
    cover = {rules: [...cover.rules.filter((rule) => !(rule.kind === "literal" && set.has(rule.path.slice(0, Math.max(0, rule.path.lastIndexOf("/")))))), ...largest.map((directory) => ({kind: "subpath", path: directory}))], denials: cover.denials.filter((denial) => !largest.some((directory) => denial.startsWith(`${directory}/`)))};
  }
  throw new Error("profile does not fit after 120 relaxations");
}

/** 🧭️ Workspace paths an error text names (the kernel enforced the denial, but the rate limited unified log did not report it): files as they are, a directory as its `Cargo.toml`. */
export function namedByErrorV1(output, root) {
  const found = new Set(), pattern = new RegExp(`${root.replace(/[.*+?^${}()|[\]\\]/gu, "\\$&")}/[^\\s\`'"]+`, "gu");
  for (const match of output.matchAll(pattern)) {
    const absolute = match[0].replace(/[:,)\].]+$/u, "");
    try { const stat = lstatSync(absolute); const file = stat.isDirectory() ? join(absolute, "Cargo.toml") : absolute; if (existsSync(file) && lstatSync(file).isFile()) found.add(file.slice(root.length + 1)); } catch { /* not a path */ }
  }
  return [...found].sort();
}

const lister = (root) => (directory) => { try { return readdirSync(directory ? join(root, directory) : root); } catch { return undefined; } };

/**
 * 🔭️ Runs a command under the hybrid profile and learns the undeclared reads round by round.
 * @param {{root: string, cwd: string, allowed: readonly string[], ambient?: readonly string[], command: readonly string[], env: Record<string, string | undefined>, rounds: number, timeoutMs: number, progress?: (text: string) => void, signal?: AbortSignal}} request
 */
export async function runHybridOracleV1(request) {
  const root = request.root, ambient = [...AMBIENT, ...(request.ambient ?? [])], learned = [], rounds = [], scratch = mkdtempSync(join(tmpdir(), "ho-"));
  try {
    for (let round = 0; round < request.rounds; round++) {
      if (request.signal?.aborted) throw new Error("cancelled");
      if (process.memoryUsage().rss > 500 * 1048576) throw new Error("RSS above 500 MB");
      const raw = hybridCoverV1([...request.allowed, ...learned], lister(root)), {cover, relaxed} = fitCoverV1(root, raw, ambient, scratch), profile = join(scratch, `p-${round}.sb`), since = new Date(Date.now() - 2000);
      writeFileSync(profile, hybridProfileV1(root, cover, ambient));
      const done = await captureV1(["sandbox-exec", "-f", profile, ...request.command], {cwd: join(root, request.cwd), env: request.env, timeoutMs: request.timeoutMs, signal: request.signal});
      if (/^sandbox-exec: [^\n]*(?:exceeds maximum|invalid|syntax error|parse)/iu.test(done.output.trimStart())) throw new Error(`profile refused: ${done.output.trim().slice(0, 300)} (${cover.rules.length} grants, ${cover.denials.length} denials)`);
      await sleep(2500);
      const denied = readSeatbeltDenialsV1(since, root).filter((path) => !ambient.some((tree) => path === tree || path.startsWith(`${tree}/`)) && !hybridGrantsV1(cover, path));
      rounds.push({round, exit: done.exit, seconds: done.seconds, denied, grants: cover.rules.length, denials: cover.denials.length, relaxed: relaxed.length, relaxedPaths: round === 0 ? relaxed : undefined, tail: done.output.slice(-1500)});
      request.progress?.(`round ${round}: exit ${done.exit}, ${denied.length} undeclared reads, ${done.seconds} s, ${cover.rules.length} grants + ${cover.denials.length} denials, ${relaxed.length} relaxed`);
      let named = denied;
      if (!denied.length && done.exit !== 0) named = namedByErrorV1(done.output, root).filter((path) => !hybridGrantsV1(cover, path) && !ambient.some((tree) => path === tree || path.startsWith(`${tree}/`)));
      const fresh = named.filter((path) => !learned.includes(path));
      if (request.partial) writeFileSync(request.partial, JSON.stringify({version: 1, backend: "seatbelt-hybrid", strength: "kernel-enforced", incomplete: true, allowed: request.allowed.length, learned: [...learned, ...fresh], rounds, passed: false, clean: false}));
      if (named !== denied && fresh.length) request.progress?.(`  ${fresh.length} read(s) named by the error text (the unified log dropped them): ${fresh.slice(0, 2).join(", ")}`);
      if (!fresh.length) break;
      learned.push(...fresh);
    }
  } finally { rmSync(scratch, {recursive: true, force: true}); }
  const last = rounds.at(-1);
  return {version: 1, backend: "seatbelt-hybrid", strength: "kernel-enforced", allowed: request.allowed.length, learned, rounds, passed: last?.exit === 0, clean: last !== undefined && last.denied.length === 0};
}
