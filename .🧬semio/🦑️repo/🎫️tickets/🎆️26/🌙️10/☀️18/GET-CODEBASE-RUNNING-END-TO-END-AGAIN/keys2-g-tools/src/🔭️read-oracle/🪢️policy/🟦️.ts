/** 🪢️ One read grant in a backend neutral form: a whole directory tree or a single file, relative to the workspace root with `/` separators. */
export type ReadRuleV1 = {readonly kind: "subpath" | "literal"; readonly path: string};

/** 🔐️ What a command may read: the declared files, the trees every command may read (installed dependencies, caches) and the file names readable anywhere (package manifests). */
export type ReadPolicyV1 = {readonly allowed: readonly string[]; readonly ambient: readonly string[]; readonly names: readonly string[]};

/** 📦️ The compiled policy every backend renders: the smallest rule set plus the ambient trees and names. */
export type ReadPlanV1 = {readonly root: string; readonly rules: readonly ReadRuleV1[]; readonly ambient: readonly string[]; readonly names: readonly string[]};

/** 📂️ Lists the entry names of a workspace directory (`""` is the root); `undefined` when it does not exist. */
export type DirectoryListerV1 = (directory: string) => readonly string[] | undefined;

/** 📈️ One run of the command under a policy: exit status, the tail of its output, the undeclared reads the backend named (workspace relative) and notes for the report. */
export type ReadRoundV1 = {readonly exit: number; readonly tail: string; readonly seconds: number; readonly denied: readonly string[]; readonly notes: readonly string[]};

/** ⚙️ The command of one round: argument vector, workspace relative working directory, environment and deadline. */
export type ReadRunV1 = {readonly command: readonly string[]; readonly cwd: string; readonly env: Readonly<Record<string, string | undefined>>; readonly timeoutMs: number; readonly scratch: string; readonly round: number; readonly signal?: AbortSignal};

/** 🌲️ The trees that hold no declared input: installed dependencies, version control, build output and the caches of the build system. */
export const ambientTreesV1: readonly string[] = ["node_modules", ".cache", ".git", "target", ".🧬semio/🦑️repo/⚡️cache"];

type Node = Map<string, Node>;

/** ✂️ The smallest rule set that grants exactly the declared files: a directory whose every entry is declared becomes one subtree rule, every other declared file stays a literal (kernel profiles have size limits). */
export function coverV1(allowed: readonly string[], list: DirectoryListerV1): ReadRuleV1[] {
  const tree: Node = new Map();
  for (const path of new Set(allowed)) {
    let node = tree;
    for (const part of path.split("/")) {
      let child = node.get(part);
      if (!child) node.set(part, (child = new Map()));
      node = child;
    }
  }
  const full = new Map<string, boolean>();
  const complete = (path: string, node: Node): boolean => {
    if (!node.size) return true;
    const names = list(path);
    let all = names !== undefined && names.length === node.size && names.every((name) => node.has(name));
    for (const [name, child] of node) all = complete(path ? `${path}/${name}` : name, child) && all;
    full.set(path, all);
    return all;
  };
  complete("", tree);
  const rules: ReadRuleV1[] = [];
  const walk = (path: string, node: Node): void => {
    if (path && node.size && full.get(path)) return void rules.push({kind: "subpath", path});
    for (const [name, child] of [...node].sort(([left], [right]) => (left < right ? -1 : left > right ? 1 : 0))) {
      const at = path ? `${path}/${name}` : name;
      if (child.size) walk(at, child);
      else rules.push({kind: "literal", path: at});
    }
  };
  walk("", tree);
  return rules;
}

/** 🧮️ Compiles a policy against the real directory layout. */
export function planV1(root: string, policy: ReadPolicyV1, list: DirectoryListerV1): ReadPlanV1 {
  return {root, rules: coverV1(policy.allowed, list), ambient: [...new Set([...ambientTreesV1, ...policy.ambient])], names: [...policy.names]};
}

/** ✅️ Whether the plan grants a workspace file: the semantics every backend must implement, used by the independent evaluator. */
export function planGrantsV1(plan: ReadPlanV1, path: string): boolean {
  if (plan.ambient.some((tree) => path === tree || path.startsWith(`${tree}/`))) return true;
  if (plan.names.includes(path.slice(path.lastIndexOf("/") + 1))) return true;
  return plan.rules.some((rule) => (rule.kind === "literal" ? path === rule.path : path === rule.path || path.startsWith(`${rule.path}/`)));
}

/** 🏷️ The files named like one of `names` anywhere below the root, found by walking the directories that are not ambient trees or hidden. */
export function namedFilesV1(names: readonly string[], ambient: readonly string[], list: DirectoryListerV1, isDirectory: (path: string) => boolean): string[] {
  const found: string[] = [], wanted = new Set(names), pending = [""];
  while (pending.length) {
    const directory = pending.pop()!;
    for (const name of list(directory) ?? []) {
      const path = directory ? `${directory}/${name}` : name;
      if (ambient.some((tree) => path === tree) || name.startsWith(".")) continue;
      if (isDirectory(path)) pending.push(path);
      else if (wanted.has(name)) found.push(path);
    }
  }
  return found.sort();
}

/** ❓️ Reads of the denied list that are undeclared: workspace relative, outside the ambient trees, not yet allowed. */
export function undeclaredV1(paths: readonly string[], plan: ReadPlanV1): string[] {
  return [...new Set(paths)].filter((path) => !planGrantsV1(plan, path)).sort();
}
