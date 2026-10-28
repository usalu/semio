import {constants, copyFileSync, existsSync, lstatSync, mkdirSync, mkdtempSync, readdirSync, realpathSync, rmSync, symlinkSync} from "node:fs";
import {dirname, join} from "node:path";
import {namedFilesV1, type ReadPlanV1, type ReadRoundV1, type ReadRunV1} from "../🪢️policy/🟦️.ts";
import {captureV1} from "../🏃️run/🟦️.ts";

/** 🧩️ The file system the mirror is built from, injected so the layout can be tested on a fixture tree. */
export type MirrorFileSystemV1 = {readonly copy: (from: string, to: string) => void; readonly link: (target: string, at: string) => void; readonly exists: (path: string) => boolean; readonly make: (directory: string) => void; readonly list: (absolute: string) => readonly string[] | undefined; readonly isDirectory: (absolute: string) => boolean};

/** 🗂️ The real file system: files are CLONED (copy-on-write where the volume has it, a plain copy elsewhere), never hard linked, so a command that writes into the mirror cannot reach the workspace; ambient trees are symbolic links (junctions on Windows, which need no privilege). */
export const realMirrorFileSystemV1: MirrorFileSystemV1 = {
  copy: (from, to) => copyFileSync(from, to, constants.COPYFILE_FICLONE),
  link: (target, at) => symlinkSync(target, at, process.platform === "win32" ? "junction" : "dir"),
  exists: (path) => existsSync(path),
  make: (directory) => mkdirSync(directory, {recursive: true}),
  list: (absolute) => { try { return readdirSync(absolute); } catch { return undefined; } },
  isDirectory: (absolute) => { try { return lstatSync(absolute).isDirectory(); } catch { return false; } },
};

/** 🪞️ Lays out a mirror root that holds exactly the declared files (and the files named like `names`) at their workspace paths, plus links to the ambient trees. */
export function buildMirrorV1(plan: ReadPlanV1, allowed: readonly string[], mirror: string, fs: MirrorFileSystemV1 = realMirrorFileSystemV1): number {
  const named = namedFilesV1(plan.names, plan.ambient, (directory) => fs.list(directory ? join(plan.root, directory) : plan.root), (path) => fs.isDirectory(join(plan.root, path)));
  let copied = 0;
  for (const path of new Set([...allowed, ...named])) {
    const from = join(plan.root, path), to = join(mirror, path);
    if (!fs.exists(from)) continue;
    fs.make(dirname(to));
    fs.copy(from, to);
    copied++;
  }
  for (const tree of plan.ambient) {
    const from = join(plan.root, tree);
    if (!fs.exists(from)) continue;
    fs.make(dirname(join(mirror, tree)));
    fs.link(from, join(mirror, tree));
  }
  return copied;
}

/** 🔁️ Replaces the workspace root by the mirror root in text (command arguments, environment values, output) so the command works inside the mirror. */
export function rebaseV1(text: string, from: string, to: string): string {
  return text.split(from).join(to);
}

/** 🕳️ The undeclared files a failing command could not find: paths inside the mirror that its output names next to a missing-file message and that exist in the workspace. */
export function missingInMirrorV1(output: string, mirror: string, root: string, exists: (workspacePath: string) => boolean): string[] {
  const found = new Set<string>(), start = `${mirror}/`;
  for (const line of output.split("\n")) {
    if (!/ENOENT|no such file|cannot find|not found|os error 2|ModuleNotFound/iu.test(line)) continue;
    for (let at = line.indexOf(start); at >= 0; at = line.indexOf(start, at + 1)) {
      const path = /^[^'"`\s:)(<>|]+/u.exec(line.slice(at + start.length))?.[0];
      if (path && exists(join(root, path))) found.add(path);
    }
  }
  return [...found].sort();
}

/** 🧪️ The sparse mirror backend. It proves that a command gives the same result when ONLY the declared files exist: the same exit status and, when compared, the same output. A read of an undeclared file that the command tolerates (an optional lookup) is invisible to it, which is why it is output equivalence and not a read trace; it runs on every platform without privileges. */
export async function runMirrorV1(plan: ReadPlanV1, allowed: readonly string[], run: ReadRunV1, baseline: {readonly exit: number; readonly output?: string} | undefined, fs: MirrorFileSystemV1 = realMirrorFileSystemV1): Promise<ReadRoundV1> {
  const mirror = realpathSync(mkdtempSync(join(run.scratch, "m-")));
  try {
    const copied = buildMirrorV1(plan, allowed, mirror, fs);
    const rebase = (text: string) => rebaseV1(text, plan.root, mirror);
    const env = Object.fromEntries(Object.entries(run.env).map(([key, value]) => [key, value === undefined ? undefined : rebase(value)]));
    const done = await captureV1(run.command.map(rebase), {cwd: join(mirror, run.cwd), env, timeoutMs: run.timeoutMs, signal: run.signal});
    const output = rebaseV1(done.output, mirror, plan.root), same = baseline === undefined ? true : baseline.exit === done.exit && (baseline.output === undefined || baseline.output.trim() === output.trim());
    const denied = done.exit !== 0 || !same ? missingInMirrorV1(done.output, mirror, plan.root, (path) => fs.exists(path)).filter((path) => !allowed.includes(path)) : [];
    return {exit: same ? done.exit : done.exit === 0 ? 1 : done.exit, tail: output.slice(-1500), seconds: done.seconds, denied, notes: [`copied:${copied}`, ...(same ? [] : ["output-differs"])]};
  } finally {
    rmSync(mirror, {recursive: true, force: true});
  }
}
