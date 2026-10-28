/** 🪢️ One rule of a read plan: a whole directory tree or a single file, relative to the workspace root with `/` separators (`""` is the root itself), that grants or refuses reading. The rules apply in order and the last matching rule decides, so a refusal inside a granted tree and a grant inside a refused tree are both one rule. */
export type ReadRuleV1 = {readonly effect: "allow" | "deny"; readonly kind: "subpath" | "literal"; readonly path: string};

/** 📜️ What a command may read: the declared files, the trees every command may read (installed dependencies, caches) and the file names readable anywhere (package manifests). */
export type ReadPolicyV1 = {readonly allowed: readonly string[]; readonly ambient: readonly string[]; readonly names: readonly string[]};

/** 🗺️ The compiled policy every backend renders: the cheapest ordered rule list that reads exactly the declared files (below the root, which starts refused) plus the ambient trees and names, and the rule lists of other cost trade-offs (fewer grants for more refusals) that a backend with a small profile limit can split into several runs. */
export type ReadPlanV1 = {readonly root: string; readonly rules: readonly ReadRuleV1[]; readonly alternatives: readonly (readonly ReadRuleV1[])[]; readonly ambient: readonly string[]; readonly names: readonly string[]};

/** 📂️ Lists the entry names of a workspace directory (`""` is the root); `undefined` when it does not exist. */
export type DirectoryListerV1 = (directory: string) => readonly string[] | undefined;

/** 🏃️ One run of the command under a policy: exit status, the tail of its output, the undeclared reads the backend named (workspace relative) and notes for the report. */
export type ReadRoundV1 = {readonly exit: number; readonly tail: string; readonly seconds: number; readonly denied: readonly string[]; readonly notes: readonly string[]};

/** ⏱️ The command of one round: argument vector, workspace relative working directory, environment and deadline. */
export type ReadRunV1 = {readonly command: readonly string[]; readonly cwd: string; readonly env: Readonly<Record<string, string | undefined>>; readonly timeoutMs: number; readonly scratch: string; readonly round: number; readonly signal?: AbortSignal};

/** 🌳️ The trees that hold no declared input: installed dependencies, version control, build output and the caches of the build system. */
export const ambientTreesV1: readonly string[] = ["node_modules", ".cache", ".git", "target", ".🧬semio/🦑️repo/⚡️cache"];

/** 🎚️ How the cover trades grants against refusals: `refusals: false` never refuses (a backend that can only allow, like Landlock), `refusalWeight` makes a refusal cheaper than a grant (below 1) so the cover grants big trees and refuses their exceptions, and `ignore` names the entries that are readable anyway (ambient trees, named files). */
export type CoverOptionsV1 = {readonly refusals?: boolean; readonly refusalWeight?: number; readonly ignore?: (path: string) => boolean};

type Node = Map<string, Node>;
type Choice = "allow" | "deny" | undefined;

/** 📚️ The listing of the declared directory tree, built once and shared by every cover weight: per directory the sorted names that are not ignored, whether each is declared (`1`), undeclared (`0`) or a nested declared directory, and the summed weights of the declared and the undeclared entries. */
export type CoverLayoutV1 = {readonly path: string; readonly names: readonly string[]; readonly kinds: readonly (CoverLayoutV1 | 0 | 1)[]; readonly declaredWeight: number; readonly undeclaredWeight: number};
type Solved = {readonly refused: number; readonly granted: number; readonly flip: {readonly deny: boolean; readonly allow: boolean}; readonly directories: readonly Solved[]};

const compare = (left: string, right: string): number => (left < right ? -1 : left > right ? 1 : 0);
const WEIGHT = 2 ** 24;
const weightOf = (path: string): number => WEIGHT + Buffer.byteLength(path);

/** 🗂️ Lists the directories that hold declared files once; `ignore` names the entries that are readable anyway. */
export function layoutV1(allowed: readonly string[], list: DirectoryListerV1, ignore: (path: string) => boolean = () => false): CoverLayoutV1 | undefined {
  const tree: Node = new Map();
  for (const path of new Set(allowed)) {
    if (ignore(path)) continue;
    let node = tree;
    for (const part of path.split("/")) {
      let child = node.get(part);
      if (!child) node.set(part, (child = new Map()));
      node = child;
    }
  }
  const build = (path: string, node: Node): CoverLayoutV1 | undefined => {
    const listed = list(path);
    if (listed === undefined) return undefined;
    const names: string[] = [], kinds: (CoverLayoutV1 | 0 | 1)[] = [];
    let declaredWeight = 0, undeclaredWeight = 0;
    for (const name of [...listed].sort(compare)) {
      const at = path ? `${path}/${name}` : name, child = node.get(name);
      if (ignore(at)) continue;
      if (child === undefined) { names.push(name); kinds.push(0); undeclaredWeight += weightOf(at); }
      else if (!child.size) { names.push(name); kinds.push(1); declaredWeight += weightOf(at); }
      else {
        const inner = build(at, child);
        if (!inner) continue;
        names.push(name);
        kinds.push(inner);
      }
    }
    return {path, names, kinds, declaredWeight, undeclaredWeight};
  };
  return build("", tree);
}

/** 🧮️ The cheapest ordered rule list for a layout: the cost of a rule is one unit (plus its path length as tie break), and per directory and inherited state (refused or granted) it is the cheaper of keeping the state for every entry or flipping it with one subpath rule and paying for the exceptions. */
export function coverLayoutV1(layout: CoverLayoutV1 | undefined, options: CoverOptionsV1 = {}): ReadRuleV1[] {
  const refusal = options.refusals === false ? Number.POSITIVE_INFINITY : options.refusalWeight ?? 1;
  const solve = (directory: CoverLayoutV1): Solved => {
    const directories: Solved[] = [];
    let refused = directory.declaredWeight, granted = directory.undeclaredWeight ? refusal * directory.undeclaredWeight : 0;
    for (const kind of directory.kinds) {
      if (typeof kind === "number") continue;
      const inner = solve(kind);
      directories.push(inner);
      refused += inner.refused;
      granted += inner.granted;
    }
    const grant = weightOf(directory.path) + granted, refuse = refusal * weightOf(directory.path) + refused;
    return {refused: Math.min(refused, grant), granted: Math.min(granted, refuse), flip: {allow: grant < refused, deny: refuse < granted}, directories};
  };
  const rules: ReadRuleV1[] = [];
  const emit = (directory: CoverLayoutV1, solved: Solved, inherited: "deny" | "allow"): void => {
    let state = inherited;
    if (inherited === "deny" ? solved.flip.allow : solved.flip.deny) {
      state = inherited === "deny" ? "allow" : "deny";
      rules.push({effect: state, kind: "subpath", path: directory.path});
    }
    let nested = 0;
    for (const [index, kind] of directory.kinds.entries()) {
      const path = directory.path ? `${directory.path}/${directory.names[index]}` : directory.names[index]!;
      if (typeof kind !== "number") emit(kind, solved.directories[nested++]!, state);
      else if (kind === 1 && state === "deny") rules.push({effect: "allow", kind: "literal", path});
      else if (kind === 0 && state === "allow") rules.push({effect: "deny", kind: "subpath", path});
    }
  };
  if (layout) emit(layout, solve(layout), "deny");
  return rules;
}

/** 🎯️ The cheapest ordered rule list that reads exactly the declared files that exist (see `coverLayoutV1`). */
export function coverV1(allowed: readonly string[], list: DirectoryListerV1, options: CoverOptionsV1 = {}): ReadRuleV1[] {
  return coverLayoutV1(layoutV1(allowed, list, options.ignore), options);
}

/** 🔍️ Whether a rule covers a workspace file. */
export function ruleCoversV1(rule: ReadRuleV1, path: string): boolean {
  return rule.kind === "literal" ? path === rule.path : rule.path === "" || path === rule.path || path.startsWith(`${rule.path}/`);
}

/** ⚖️ Whether an ordered rule list grants a workspace file (the root starts refused, the last matching rule decides). */
export function rulesGrantV1(rules: readonly ReadRuleV1[], path: string): boolean {
  let choice: Choice;
  for (const rule of rules) if (ruleCoversV1(rule, path)) choice = rule.effect;
  return choice === "allow";
}

const isAmbient = (ambient: readonly string[], path: string): boolean => ambient.some((tree) => path === tree || path.startsWith(`${tree}/`));

/** 🪜️ The refusal weights of the alternative covers: each trades grants for refusals, so the grants every split run must carry shrink while the refusals, which a split distributes, grow. */
export const alternativeWeightsV1: readonly number[] = [0.6, 0.35, 0.2, 0.1, 0.05];

/** 🧭️ Compiles a policy against the real directory layout. `refusals: false` plans for a backend that can only allow (Landlock); `alternatives` also computes the covers of other weights for a backend with a profile size limit. */
export function planV1(root: string, policy: ReadPolicyV1, list: DirectoryListerV1, options: {readonly refusals?: boolean; readonly alternatives?: boolean} = {}): ReadPlanV1 {
  const ambient = [...new Set([...ambientTreesV1, ...policy.ambient])], names = new Set(policy.names);
  const ignore = (path: string) => isAmbient(ambient, path) || names.has(path.slice(path.lastIndexOf("/") + 1));
  const layout = layoutV1(policy.allowed, list, ignore), rules = coverLayoutV1(layout, {refusals: options.refusals});
  return {root, rules, alternatives: options.refusals !== false && options.alternatives ? alternativeWeightsV1.map((refusalWeight) => coverLayoutV1(layout, {refusalWeight})) : [], ambient, names: [...policy.names]};
}

/** ✅️ Whether the plan grants a workspace file: the semantics every backend must implement, used by the independent evaluator. */
export function planGrantsV1(plan: ReadPlanV1, path: string): boolean {
  if (isAmbient(plan.ambient, path)) return true;
  if (plan.names.includes(path.slice(path.lastIndexOf("/") + 1))) return true;
  return rulesGrantV1(plan.rules, path);
}

/** ✂️ Splits a plan into runs whose profiles each hold at most `capacity` rules, or `undefined` when no split fits. Every run carries all grants and a share of the refusals (in order), so a declared file is granted in every run and an undeclared file is refused in every run that holds the refusal that decides it: a file whose deciding refusal is in one run is refused there, and the union of the runs refuses everything the whole plan refuses. The alternative with the fewest runs wins. */
export function splitPlanV1(plan: ReadPlanV1, capacity: number): ReadPlanV1[] | undefined {
  let best: {readonly rules: readonly ReadRuleV1[]; readonly runs: number} | undefined;
  for (const rules of [plan.rules, ...plan.alternatives]) {
    const grants = rules.filter((rule) => rule.effect === "allow").length, refusals = rules.length - grants, room = capacity - grants;
    if (rules.length <= capacity) { best = {rules, runs: 1}; break; }
    if (room < 1) continue;
    const runs = Math.ceil(refusals / room);
    if (!best || runs < best.runs) best = {rules, runs};
  }
  if (!best) return undefined;
  if (best.runs === 1) return [{...plan, rules: best.rules}];
  const refusals = best.rules.filter((rule) => rule.effect === "deny"), size = Math.ceil(refusals.length / best.runs);
  return Array.from({length: Math.ceil(refusals.length / size)}, (_, index) => {
    const share = new Set(refusals.slice(index * size, (index + 1) * size));
    return {...plan, rules: best!.rules.filter((rule) => rule.effect === "allow" || share.has(rule))};
  });
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

/** 🚫️ Reads of the denied list that are undeclared: workspace relative, outside the ambient trees, not yet allowed. */
export function undeclaredV1(paths: readonly string[], plan: ReadPlanV1): string[] {
  return [...new Set(paths)].filter((path) => !planGrantsV1(plan, path)).sort();
}
