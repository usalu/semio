import {spawnSync} from "node:child_process";
import {readFileSync} from "node:fs";
import {join} from "node:path";

export const ROOT = "/Users/ueli/Documents/semio";
export const PROJECT = "🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript";

export const inventory = (root = ROOT): string[] => {
  const out = spawnSync("git", ["ls-files", "-z", "--cached", "--others", "--exclude-standard"], {cwd: root, maxBuffer: 1 << 30});
  return [...new Set(out.stdout.toString("utf8").split("\0").filter(Boolean))].sort();
};

export const expandInputs = (manifest: any, root: string, targetInputs: readonly unknown[], projectRoot = PROJECT) => {
  const nx = JSON.parse(readFileSync(join(root, "nx.json"), "utf8")).namedInputs ?? {};
  const named: Record<string, unknown[]> = {default: ["{projectRoot}/**/*"], ...nx, ...(manifest.namedInputs ?? {})};
  const patterns: string[] = [], objects: unknown[] = [];
  const walk = (list: readonly unknown[], depth = 0) => {
    if (depth > 20) throw Error("named input cycle");
    for (const input of list) {
      if (typeof input === "string") {
        if (input.startsWith("^")) continue;
        if (Object.hasOwn(named, input)) { walk(named[input]!, depth + 1); continue; }
        patterns.push(input.replaceAll("{projectRoot}", projectRoot).replaceAll("{workspaceRoot}/", "").replace(/^\/+/u, ""));
      } else objects.push(input);
    }
  };
  walk(targetInputs);
  return {patterns, objects};
};

export const select = (patterns: readonly string[], files: readonly string[]): Set<string> => {
  const include = patterns.filter(p => !p.startsWith("!")), exclude = patterns.filter(p => p.startsWith("!")).map(p => new Bun.Glob(p.slice(1)));
  const out = new Set<string>();
  for (const pattern of include) {
    const glob = new Bun.Glob(pattern);
    if (!/[?*{[]/u.test(pattern)) { if (files.includes(pattern)) out.add(pattern); continue; }
    for (const file of files) if (glob.match(file)) out.add(file);
  }
  for (const file of [...out]) if (exclude.some(glob => glob.match(file))) out.delete(file);
  return out;
};
