#!/usr/bin/env bun
/**
 * One-time proof (ticket input, not codebase code) that the removed launch rows still resolve:
 *  1. `r2-v1-launch-selections.json`: the 1073 selections M-1a derived from the retained rows of the removed
 *     `.vscode/launch.json` (HEAD), its four compounds and `.claude/launch.json` (HEAD), with the cmd/args/env/ready
 *     each row stood for; every one is run with `semio run … --dry-run` and compared with the shared comparator.
 *  2. `r2-m2-mapping.json`: the `semio run …` commands M-2 gave the 79 `.claude` entries, the 4 compounds and the
 *     deviations; every command in it must resolve in a dry run.
 * Usage: bun r2-v1-launch-coverage.ts [selections|mapping|all]   (SEMIO_TEST_CLI selects the binary)
 */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { pool, repository, semio } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🧭️journeys/🧰️support/🟦️.ts";
import { compare, runArguments, type Json, type Launch, type Resolved, type Selection } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🗺️coverage/🔮️oracle/🟦️.ts";

const ticket = import.meta.dir;
const out: string[] = [];
const say = (line: string): void => { out.push(line); console.log(line); };
const mode = process.argv[2] ?? "all";

async function dry(selection: Selection): Promise<{ code: number; stderr: string; launch?: Launch }> {
  const run = await semio(["run", ...runArguments(selection, ["--dry-run"])], { cwd: repository, timeoutMs: 300_000 });
  return { code: run.code, stderr: run.stderr, launch: run.code === 0 ? JSON.parse(run.stdout) : undefined };
}

async function selections(): Promise<void> {
  const frozen = JSON.parse(readFileSync(join(ticket, "r2-v1-launch-selections.json"), "utf8")) as { selections: { origin: string; id: string; parameters: Record<string, string | boolean>; extraArgs: string[]; expected: Json; match: string | null }[] };
  const failures: string[] = [];
  const equivalences = new Map<string, number>();
  await pool(frozen.selections, 6, async (selection) => {
    const { code, stderr, launch } = await dry({ id: selection.id, parameters: selection.parameters, extraArgs: selection.extraArgs });
    const label = `${selection.origin} ${selection.id} ${JSON.stringify(selection.parameters)} ${JSON.stringify(selection.extraArgs)} [${selection.match}]`;
    if (!launch) { failures.push(`${label}: exit ${code}: ${stderr.trim().slice(0, 240)}`); return; }
    const expected = selection.expected;
    if (!expected) { equivalences.set("resolve-only", (equivalences.get("resolve-only") ?? 0) + 1); return; }
    const resolved: Resolved = expected.members
      ? { cmd: "", args: [], env: {}, cwd: "", requires: [], stop: expected.stop, longRunning: true, members: expected.members.map((member: Json) => ({ selection: { id: member.run, parameters: member.parameters ?? {}, extraArgs: [] }, resolved: { cmd: member.cmd, args: member.args, env: member.env ?? {}, cwd: member.cwd ?? "", ready: member.ready ?? undefined, requires: [], longRunning: true } })) }
      : { cmd: expected.cmd, args: expected.args, env: expected.env ?? {}, cwd: expected.cwd ?? "", ready: expected.ready ?? undefined, requires: [], longRunning: false };
    const notes: string[] = [];
    const faults = compare(selection.id, resolved, launch, notes).filter((fault) => !fault.includes(" requires "));
    for (const note of notes) equivalences.set(note.split(":")[0]!, (equivalences.get(note.split(":")[0]!) ?? 0) + 1);
    if (faults.length) failures.push(`${label}:\n    ${faults.join("\n    ")}`);
  });
  say(`## selections: ${frozen.selections.length - failures.length} of ${frozen.selections.length} resolve as the launch rows did; accepted equivalences ${JSON.stringify(Object.fromEntries(equivalences))}`);
  for (const failure of failures) say(failure);
}

function commandsOf(text: string): string[] {
  const found = [...text.matchAll(/semio run ((?:[^\s`'"]|\[[^\]]*\])+(?: (?:--param|--env) \S+)*(?: -- \S+)?)/g)].map((match) => `semio run ${match[1]}`);
  return found.map((command) => command.replace(/\s*\[[^\]]*\]/g, "").replace(/ --detach| --wait-ready/g, ""));
}

async function mapping(): Promise<void> {
  const map = JSON.parse(readFileSync(join(ticket, "r2-m2-mapping.json"), "utf8")) as { claude: { original: string; command: string; status: string }[]; compounds: { original: string; command: string; status: string }[]; deviations: { items: { original: string; command: string; status: string }[] }[] };
  const rows = [...map.claude, ...map.compounds, ...map.deviations.flatMap((family) => family.items)];
  const runnable = rows.flatMap((row) => commandsOf(row.command).map((command) => ({ row, command })));
  const expand = (command: string): string[] => {
    const alternatives = /([a-z-]+)=([^\s|]+(?:\|[^\s|]+)+)/.exec(command);
    return alternatives ? alternatives[2]!.split("|").flatMap((value) => expand(command.replace(alternatives[0], `${alternatives[1]}=${value}`))) : [command];
  };
  const commands = [...new Set(runnable.flatMap(({ command }) => expand(command)))];
  const failures: string[] = [];
  await pool(commands, 6, async (command) => {
    const words = command.split(" ").slice(2);
    const split = words.indexOf("--");
    const head = split < 0 ? words : words.slice(0, split);
    const extra = split < 0 ? [] : words.slice(split + 1);
    const run = await semio(["run", ...head, "--dry-run", ...(extra.length ? ["--", ...extra] : [])], { cwd: repository, timeoutMs: 300_000 });
    if (run.code !== 0) failures.push(`${command}: exit ${run.code}: ${run.stderr.trim().slice(0, 240)}`);
  });
  say(`## mapping: ${rows.length} entries (${map.claude.length} .claude, ${map.compounds.length} compounds, ${rows.length - map.claude.length - map.compounds.length} deviation items); ${commands.length} distinct commands, ${commands.length - failures.length} resolve in a dry run`);
  for (const failure of failures) say(failure);
  const dropped = rows.filter((row) => row.status === "intentionally dropped").length;
  say(`## mapping: ${dropped} entries intentionally dropped, ${rows.filter((row) => row.status === "declared now").length} declared now`);
}

if (mode === "selections" || mode === "all") await selections();
if (mode === "mapping" || mode === "all") await mapping();
writeFileSync(join(ticket, "🗑️generated/v1-launch-coverage.txt"), out.join("\n"));
