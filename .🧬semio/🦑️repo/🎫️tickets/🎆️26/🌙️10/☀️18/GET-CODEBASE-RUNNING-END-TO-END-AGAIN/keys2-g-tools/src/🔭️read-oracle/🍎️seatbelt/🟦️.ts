import {spawnSync} from "node:child_process";
import {writeFileSync} from "node:fs";
import {join} from "node:path";
import {setTimeout as sleep} from "node:timers/promises";
import type {ReadPlanV1, ReadRoundV1, ReadRunV1} from "../🪢️policy/🟦️.ts";
import {captureV1} from "../🏃️run/🟦️.ts";

const quoted = (text: string): string => `"${text.replaceAll("\\", "\\\\").replaceAll('"', '\\"')}"`;
const escaped = (text: string): string => text.replace(/[.^$*+?()[\]{}|\\]/gu, "\\$&");

/** 🍎️ The Seatbelt profile that denies every workspace file read except the plan: directories stay listable (names are not contents), ambient trees and the named files are readable, every rule of the plan is one grant. */
export function seatbeltProfileV1(plan: ReadPlanV1): string {
  const lines = ["(version 1)", "(allow default)", `(deny file-read-data (subpath ${quoted(plan.root)}))`, "(allow file-read-data (vnode-type DIRECTORY))"];
  for (const tree of plan.ambient) lines.push(`(allow file-read-data (subpath ${quoted(join(plan.root, tree))}))`);
  for (const name of plan.names) lines.push(`(allow file-read-data (regex #"^${escaped(plan.root)}/(.*/)?${escaped(name)}$"))`);
  for (const rule of plan.rules) lines.push(`(allow file-read-data (${rule.kind} ${quoted(join(plan.root, rule.path))}))`);
  return `${lines.join("\n")}\n`;
}

/** 📜️ The workspace relative paths of the `deny(1) file-read-data` lines of the unified log that lie inside the root. */
export function seatbeltDenialsV1(log: string, root: string): string[] {
  const seen = new Set<string>(), marker = "deny(1) file-read-data ";
  for (const line of log.split("\n")) {
    const at = line.indexOf(marker);
    if (at < 0) continue;
    const path = line.slice(at + marker.length).trim();
    if (path.startsWith(`${root}/`)) seen.add(path.slice(root.length + 1));
  }
  return [...seen].sort();
}

/** 🔍️ Reads the denials of the last seconds from the unified log (the kernel logs every enforced denial; report mode would be rate limited). */
export function readSeatbeltDenialsV1(since: Date, root: string): string[] {
  const stamp = `${since.getFullYear()}-${String(since.getMonth() + 1).padStart(2, "0")}-${String(since.getDate()).padStart(2, "0")} ${String(since.getHours()).padStart(2, "0")}:${String(since.getMinutes()).padStart(2, "0")}:${String(since.getSeconds()).padStart(2, "0")}`;
  const done = spawnSync("/usr/bin/log", ["show", "--start", stamp, "--style", "compact", "--predicate", `eventMessage CONTAINS "deny(1) file-read-data" AND eventMessage CONTAINS ${quoted(root)}`], {encoding: "utf8", maxBuffer: 256 * 1024 * 1024});
  return seatbeltDenialsV1(done.stdout ?? "", root);
}

/** 🩺️ Whether the Seatbelt launcher works for this user: the tool exists and accepts a trivial profile. */
export function seatbeltAvailableV1(platform: NodeJS.Platform = process.platform): {available: boolean; reason?: string} {
  if (platform !== "darwin") return {available: false, reason: `Seatbelt needs macOS, this is ${platform}`};
  const probe = spawnSync("sandbox-exec", ["-p", "(version 1)(allow default)", "/usr/bin/true"], {encoding: "utf8"});
  return probe.status === 0 ? {available: true} : {available: false, reason: `sandbox-exec failed: ${(probe.stderr || String(probe.error ?? "")).trim().slice(0, 200)}`};
}

/** 🚧️ The kernel refused to compile the profile (too many rules): the caller falls back to a backend without a profile limit. */
export class SeatbeltProfileRefusedV1 extends Error {
  constructor(reason: string) {
    super(reason);
    this.name = "SeatbeltProfileRefusedV1";
  }
}

/** 🧱️ Whether a run ended because the kernel refused to compile the profile (`sandbox-exec: data object length 75339 exceeds maximum (65535)`), not because the command failed. */
export function seatbeltRefusalV1(exit: number, output: string): boolean {
  return exit !== 0 && /^sandbox-exec: [^\n]*(?:exceeds maximum|invalid|syntax error|parse)/iu.test(output.trimStart());
}

/** ▶️ Runs the command under the profile of the plan and reads the enforced denials back from the unified log (which needs a moment to flush). */
export async function runSeatbeltV1(plan: ReadPlanV1, run: ReadRunV1): Promise<ReadRoundV1> {
  const profile = join(run.scratch, `profile-${run.round}.sb`), since = new Date(Date.now() - 2000);
  writeFileSync(profile, seatbeltProfileV1(plan));
  const done = await captureV1(["sandbox-exec", "-f", profile, ...run.command], {cwd: join(plan.root, run.cwd), env: run.env, timeoutMs: run.timeoutMs, signal: run.signal});
  if (seatbeltRefusalV1(done.exit, done.output)) throw new SeatbeltProfileRefusedV1(done.output.trim().slice(0, 300));
  await sleep(2500);
  return {exit: done.exit, tail: done.output.slice(-1500), seconds: done.seconds, denied: readSeatbeltDenialsV1(since, plan.root), notes: []};
}
