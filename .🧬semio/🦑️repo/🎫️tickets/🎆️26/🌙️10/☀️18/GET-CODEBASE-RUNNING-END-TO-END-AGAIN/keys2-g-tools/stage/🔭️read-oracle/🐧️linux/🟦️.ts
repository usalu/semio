import {spawnSync} from "node:child_process";
import {closeSync, existsSync, openSync, readSync, writeFileSync} from "node:fs";
import {join} from "node:path";
import {namedFilesV1, planGrantsV1, type ReadPlanV1, type ReadRoundV1, type ReadRunV1} from "../🪢️policy/🟦️.ts";
import {captureV1} from "../🏃️run/🟦️.ts";

/** 🐧️ The file system questions the grant computation asks, injected so the computation is testable without a Linux host. */
export type LinuxFileSystemV1 = {readonly list: (absolute: string) => readonly string[] | undefined; readonly isDirectory: (absolute: string) => boolean; readonly real: (absolute: string) => string};

/** 🪜️ Landlock grants are allow-lists of trees, so reading the workspace file by file means granting everything outside it: every sibling of each directory on the way down to the root, then inside the root only the plan (rules, ambient trees, named files). */
export function landlockGrantsV1(plan: ReadPlanV1, fs: LinuxFileSystemV1): string[] {
  const grants = new Set<string>(), parts = plan.root.split("/").filter(Boolean);
  for (let depth = 0; depth < parts.length; depth++) {
    const directory = `/${parts.slice(0, depth).join("/")}`.replace(/\/$/u, "") || "/";
    for (const name of fs.list(directory) ?? []) if (name !== parts[depth]) grants.add(fs.real(join(directory, name)));
  }
  for (const tree of plan.ambient) grants.add(join(plan.root, tree));
  for (const rule of plan.rules) grants.add(join(plan.root, rule.path));
  for (const path of namedFilesV1(plan.names, plan.ambient, (directory) => fs.list(directory ? join(plan.root, directory) : plan.root), (path) => fs.isDirectory(join(plan.root, path)))) grants.add(join(plan.root, path));
  return [...grants].sort();
}

const ESCAPE = /\\(x[0-9a-fA-F]{2}|[0-7]{1,3}|.)/gu;
const NAMED: Record<string, number> = {n: 10, t: 9, r: 13, '"': 34, "\\": 92, a: 7, b: 8, f: 12, v: 11, e: 27};
/** 🔡️ Decodes a strace string literal (octal and hex escapes of the raw UTF-8 bytes, named escapes) to text. */
export function straceStringV1(literal: string): string {
  const bytes: number[] = [];
  let at = 0;
  for (const match of literal.matchAll(ESCAPE)) {
    for (const byte of Buffer.from(literal.slice(at, match.index), "utf8")) bytes.push(byte);
    const body = match[1]!;
    bytes.push(body[0] === "x" ? Number.parseInt(body.slice(1), 16) : /^[0-7]/u.test(body) ? Number.parseInt(body, 8) : NAMED[body] ?? body.charCodeAt(0));
    at = match.index! + match[0].length;
  }
  for (const byte of Buffer.from(literal.slice(at), "utf8")) bytes.push(byte);
  return Buffer.from(bytes).toString("utf8");
}

/** 📋️ What a strace log says about file opens: the workspace files opened for reading and the ones refused with EACCES. */
export type StraceReadsV1 = {readonly opened: readonly string[]; readonly denied: readonly string[]};

const CALL = /^(?:\[pid\s+(\d+)\]\s+|(\d+)\s+)?(openat2|openat|open|execveat|execve)\((.*)$/u;
const RESUMED = /^(?:\[pid\s+(\d+)\]\s+|(\d+)\s+)?<\.\.\.\s+(\w+)\s+resumed>(.*)\)\s+=\s+(-?\d+|\?)(?:\s+(E\w+))?/u;
const RESULT = /\)\s+=\s+(-?\d+|\?)(?:\s+(E\w+))?/u;

/** 🔎️ Parses `strace -f -qq -y` output (open, openat, openat2, execve, execveat; unfinished/resumed pairs joined per process): paths resolve against the directory annotation of `-y` or the command directory, only paths inside the root count, a directory open (`O_DIRECTORY`, `O_PATH`) or a write-only open is not a read. */
export function straceReadsV1(lines: Iterable<string>, root: string, cwd: string): StraceReadsV1 {
  const opened = new Set<string>(), denied = new Set<string>(), unfinished = new Map<string, {path: string; reads: boolean}>();
  const settle = (call: {path: string; reads: boolean}, error: string | undefined, result: string) => {
    if (!call.reads || !call.path.startsWith(`${root}/`)) return;
    const relative = call.path.slice(root.length + 1);
    if (error === "EACCES" || error === "EPERM") denied.add(relative);
    else if (result !== "-1" && result !== "?") opened.add(relative);
  };
  for (const line of lines) {
    const resumed = RESUMED.exec(line);
    if (resumed) {
      const key = `${resumed[1] ?? resumed[2] ?? ""}:${resumed[3]}`, call = unfinished.get(key);
      if (call) { unfinished.delete(key); settle(call, resumed[6], resumed[5]!); }
      continue;
    }
    const head = CALL.exec(line);
    if (!head) continue;
    const name = head[3]!, rest = head[4]!, strings = [...rest.matchAll(/"((?:[^"\\]|\\.)*)"/gu)].map((match) => straceStringV1(match[1]!));
    const path = strings[0];
    if (path === undefined) continue;
    const directory = /^(?:AT_FDCWD|\d+)(?:<([^>]*)>)?/u.exec(rest), base = name.startsWith("openat") || name === "execveat" ? directory?.[1] ?? cwd : cwd;
    const absolute = path.startsWith("/") ? path : join(base, path), flags = rest.slice(rest.indexOf('",') + 2);
    const reads = name.startsWith("exec") || (/O_RDONLY|O_RDWR/u.test(flags) && !/O_PATH|O_DIRECTORY/u.test(flags));
    const call = {path: absolute, reads}, done = RESULT.exec(rest);
    if (/<unfinished \.\.\.>/u.test(rest)) unfinished.set(`${head[1] ?? head[2] ?? ""}:${name}`, call);
    else if (done) settle(call, done[2], done[1]!);
  }
  return {opened: [...opened].sort(), denied: [...denied].sort()};
}

/** 🧵️ The strace command line that records every open of a command tree into a log file. */
export function straceCommandV1(log: string, command: readonly string[]): string[] {
  return ["strace", "-f", "-qq", "-y", "-s", "4096", "-e", "trace=openat,open,openat2,execve,execveat", "-o", log, "--", ...command];
}

/** 🛡️ Whether Landlock works (the launcher probes the kernel) and whether strace can name the refused reads. */
export function linuxAvailableV1(launcher: string, python: string, platform: NodeJS.Platform = process.platform): {available: boolean; reason?: string; abi?: number; trace: boolean} {
  if (platform !== "linux") return {available: false, reason: `Landlock needs Linux, this is ${platform}`, trace: false};
  const trace = spawnSync("strace", ["-V"], {encoding: "utf8"}).status === 0, probe = spawnSync(python, ["-I", launcher, "--probe"], {encoding: "utf8"});
  return probe.status === 0 ? {available: true, abi: Number.parseInt(probe.stdout, 10), trace} : {available: false, reason: (probe.stderr || String(probe.error ?? "")).trim().slice(0, 200), trace};
}

/** 📄️ The lines of a large log file, read in chunks so the oracle never holds a whole trace in memory. */
export function* fileLinesV1(path: string): Generator<string> {
  const descriptor = openSync(path, "r"), chunk = Buffer.alloc(1 << 20);
  let rest = "";
  try {
    for (let read = readSync(descriptor, chunk, 0, chunk.length, null); read > 0; read = readSync(descriptor, chunk, 0, chunk.length, null)) {
      const lines = (rest + chunk.toString("utf8", 0, read)).split("\n");
      rest = lines.pop()!;
      yield* lines;
    }
    if (rest) yield rest;
  } finally {
    closeSync(descriptor);
  }
}

/** 🧰️ The tools of a Linux round: the interpreter and launcher of the Landlock restriction, whether strace can name refusals and the file system answers. */
export type LinuxToolsV1 = {readonly python: string; readonly launcher: string; readonly trace: boolean; readonly fs: LinuxFileSystemV1};

/** ⛔️ The launcher refused to restrict the process: the kernel has no usable Landlock. */
export class LandlockRefusedV1 extends Error {
  constructor(reason: string) {
    super(reason);
    this.name = "LandlockRefusedV1";
  }
}

/** 🐚️ Runs the command under the Landlock restriction of the plan; strace over the same run names the refused reads (Landlock itself reports only EACCES to the process). */
export async function runLinuxV1(plan: ReadPlanV1, run: ReadRunV1, tools: LinuxToolsV1): Promise<ReadRoundV1> {
  const planFile = join(run.scratch, `landlock-${run.round}.json`), log = join(run.scratch, `strace-${run.round}.log`), cwd = join(plan.root, run.cwd);
  writeFileSync(planFile, JSON.stringify({read: landlockGrantsV1(plan, tools.fs)}));
  const inner = [tools.python, "-I", tools.launcher, planFile, "--", ...run.command];
  const done = await captureV1(tools.trace ? straceCommandV1(log, inner) : inner, {cwd, env: run.env, timeoutMs: run.timeoutMs, signal: run.signal});
  if (done.exit === 125 && done.output.includes("landlock-unavailable")) throw new LandlockRefusedV1(done.output.trim().slice(-300));
  const denied = tools.trace && existsSync(log) ? straceReadsV1(fileLinesV1(log), plan.root, cwd).denied.filter((path) => !planGrantsV1(plan, path)) : [];
  return {exit: done.exit, tail: done.output.slice(-1500), seconds: done.seconds, denied, notes: tools.trace ? [] : ["trace-missing"]};
}
