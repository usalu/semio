import {validateJsonSchemaSubset} from "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type: "json"};

/** 🧩️ Closed registry of unit languages (schema `definitions.Language`). */
export type UnitLanguageV1 = "typescript" | "rust" | "go" | "python" | "dotnet" | "cpp" | "wit" | "tex";
export type UnitKindV1 = "library" | "binary" | "test" | "generated" | "document";
/** 🔗️ What a dependent consumes of a dependency: `interface` (compilers, checkers), `implementation` (linkers, runners), `both` (compiles against the interface and links the implementation) or `declared` (ordering and selection only, never keyed). */
export type UnitUseV1 = "interface" | "implementation" | "both" | "declared";
export type UnitDependencyV1 = {readonly unit: string; readonly use: UnitUseV1};
/** 📦️ One compilation unit: sources, declared dependencies, an interface artifact for early cutoff, outputs and a toolchain identity. */
export type UnitV1 = {
  readonly id: string;
  readonly language: UnitLanguageV1;
  readonly kind: UnitKindV1;
  readonly label: string;
  readonly sources: readonly string[];
  readonly inputs: readonly string[];
  readonly dependencies: readonly UnitDependencyV1[];
  readonly externals: readonly string[];
  readonly interface: {readonly kind: string} | null;
  readonly outputs: readonly string[];
  readonly toolchain: {readonly id: string; readonly parts: readonly string[]};
  readonly action: {readonly argv: readonly string[]; readonly env: Readonly<Record<string, string>>};
};
export type UnitGraphV1 = {readonly version: 1; readonly units: readonly UnitV1[]};
export type UnitInterfaceKindV1 = {readonly language: UnitLanguageV1; readonly artifact: string; readonly digest: string; readonly closure: "transitive" | "direct"; readonly bazel: "artifact" | "sources"};
/** 🧾️ The policy document: interface kinds per language, key layout, language rules and EN/DE texts. */
export type UnitsPolicyV1 = {
  readonly version: 1;
  readonly languages: readonly UnitLanguageV1[];
  readonly interfaceKinds: Readonly<Record<string, UnitInterfaceKindV1>>;
  readonly key: {readonly version: 1; readonly order: "code-point"; readonly text: string};
  readonly go: {readonly goos: readonly string[]; readonly goarch: readonly string[]; readonly unix: readonly string[]; readonly auxiliaryExtensions: readonly string[]; readonly skipDirectories: readonly string[]; readonly cgoDefault: boolean};
  readonly python: {readonly stdlib: readonly string[]; readonly skipDirectories: readonly string[]; readonly unannotatedReturns: "ignore" | "body"; readonly api: string};
  readonly rust: {readonly skipDirectories: readonly string[]; readonly sourceExtensions: readonly string[]};
  readonly messages: {readonly languages: readonly string[]; readonly texts: Readonly<Record<"planning" | "running" | "hit" | "ran" | "cutoff" | "failed" | "blocked" | "done" | "cancelled", Readonly<Record<string, string>>>>};
};

/** 🔤️ Orders strings by Unicode code point (equals UTF-8 byte order; plain `<` on UTF-16 differs for astral characters such as the emoji in repository paths). */
export function compareCodePointsV1(left: string, right: string): number {
  const a = left[Symbol.iterator](), b = right[Symbol.iterator]();
  for (;;) {
    const x = a.next(), y = b.next();
    if (x.done || y.done) return x.done === y.done ? 0 : x.done ? -1 : 1;
    const delta = x.value.codePointAt(0)! - y.value.codePointAt(0)!;
    if (delta) return delta;
  }
}
export const sortedCodePointsV1 = (values: Iterable<string>): string[] => [...values].sort(compareCodePointsV1);

const validateAs = (name: string, value: unknown): string[] => validateJsonSchemaSubset({definitions: schema.definitions, $ref: `#/definitions/${name}`}, value);
/** 🔎️ Validates a value against a named definition of the unit schema and returns the failures. */
export function unitsSchemaFailuresV1(name: "Graph" | "Unit" | "Receipt" | "Policy" | "InterfaceFile" | "OutFile", value: unknown): readonly string[] {return validateAs(name, value);}

/** 🔎️ Reads the policy document against its schema. */
export function readUnitsPolicyV1(value: unknown): UnitsPolicyV1 {
  const failures = validateAs("Policy", value);
  if (failures.length) throw Error("Invalid units policy: " + failures.join("; "));
  return value as UnitsPolicyV1;
}

/** 🔁️ Tarjan strongly connected components of a graph given as adjacency sets, iterative so deep chains cannot overflow the stack. Members of a component are sorted with `compare` (default: code point order). */
export function stronglyConnectedV1(nodes: readonly string[], adjacent: ReadonlyMap<string, ReadonlySet<string>>, compare: (left: string, right: string) => number = compareCodePointsV1): string[][] {
  const index = new Map<string, number>(), low = new Map<string, number>(), onStack = new Set<string>(), stack: string[] = [], out: string[][] = [];
  let counter = 0;
  for (const start of nodes) {
    if (index.has(start)) continue;
    const work: {node: string; next: Iterator<string>}[] = [];
    const enter = (node: string): void => {index.set(node, counter); low.set(node, counter++); stack.push(node); onStack.add(node); work.push({node, next: (adjacent.get(node) ?? new Set<string>())[Symbol.iterator]()});};
    enter(start);
    while (work.length) {
      const frame = work[work.length - 1]!;
      const step = frame.next.next();
      if (!step.done) {
        const target = step.value;
        if (!index.has(target)) enter(target);
        else if (onStack.has(target)) low.set(frame.node, Math.min(low.get(frame.node)!, index.get(target)!));
        continue;
      }
      work.pop();
      if (work.length) {const parent = work[work.length - 1]!.node; low.set(parent, Math.min(low.get(parent)!, low.get(frame.node)!));}
      if (low.get(frame.node) === index.get(frame.node)) {
        const component: string[] = [];
        let member: string;
        do {member = stack.pop()!; onStack.delete(member); component.push(member);} while (member !== frame.node);
        out.push(component.sort(compare));
      }
    }
  }
  return out;
}

/** 🗜️ Condenses a possibly cyclic graph: every strongly connected component is one node named after its smallest member. */
export function condensationV1(nodes: readonly string[], adjacent: ReadonlyMap<string, ReadonlySet<string>>): {readonly components: readonly (readonly string[])[]; readonly componentOf: ReadonlyMap<string, string>} {
  const components = stronglyConnectedV1(sortedCodePointsV1(nodes), adjacent).sort((a, b) => compareCodePointsV1(a[0]!, b[0]!)), componentOf = new Map<string, string>();
  for (const component of components) for (const member of component) componentOf.set(member, component[0]!);
  return {components, componentOf};
}

/** ⚖️ Every law of a unit graph; an empty list means the graph is valid. */
export function unitGraphFailuresV1(graph: UnitGraphV1, policy?: UnitsPolicyV1): readonly string[] {
  const failures: string[] = [...validateAs("Graph", graph)];
  if (failures.length) return failures;
  const byId = new Map<string, UnitV1>(), owner = new Map<string, string>();
  for (const unit of graph.units) {
    if (byId.has(unit.id)) failures.push(`duplicate unit ${unit.id}`);
    byId.set(unit.id, unit);
    if (!unit.id.startsWith(unit.language + ":")) failures.push(`unit ${unit.id} does not start with its language ${unit.language}:`);
    if (policy && unit.interface) {
      const kind = policy.interfaceKinds[unit.interface.kind];
      if (!kind) failures.push(`unit ${unit.id} has unknown interface kind ${unit.interface.kind}`);
      else if (kind.language !== unit.language) failures.push(`unit ${unit.id} (${unit.language}) uses interface kind ${unit.interface.kind} of ${kind.language}`);
    }
    for (const path of unit.sources) {
      const other = owner.get(path);
      if (other !== undefined && other !== unit.id) failures.push(`file ${path} is a source of both ${other} and ${unit.id}`);
      owner.set(path, unit.id);
    }
    if (new Set(unit.sources).size !== unit.sources.length) failures.push(`unit ${unit.id} lists a source twice`);
    if (new Set(unit.inputs).size !== unit.inputs.length) failures.push(`unit ${unit.id} lists an input twice`);
  }
  for (const unit of graph.units) {
    const seen = new Set<string>();
    for (const dependency of unit.dependencies) {
      const target = byId.get(dependency.unit);
      if (!target) {failures.push(`unit ${unit.id} depends on unknown unit ${dependency.unit}`); continue;}
      if (dependency.unit === unit.id) failures.push(`unit ${unit.id} depends on itself`);
      if (seen.has(dependency.unit)) failures.push(`unit ${unit.id} lists dependency ${dependency.unit} twice`);
      seen.add(dependency.unit);
      if ((dependency.use === "interface" || dependency.use === "both") && !target.interface) failures.push(`unit ${unit.id} consumes the interface of ${dependency.unit}, which has none`);
    }
  }
  if (failures.length) return failures;
  const adjacent = new Map<string, Set<string>>(graph.units.map(unit => [unit.id, new Set(unit.dependencies.map(dependency => dependency.unit))]));
  for (const component of stronglyConnectedV1(graph.units.map(unit => unit.id), adjacent)) if (component.length > 1) failures.push(`dependency cycle: ${component.join(" -> ")}`);
  return failures;
}

/** 🔎️ Reads a unit graph and refuses it when any law fails. */
export function readUnitGraphV1(value: unknown, policy?: UnitsPolicyV1): UnitGraphV1 {
  const failures = unitGraphFailuresV1(value as UnitGraphV1, policy);
  if (failures.length) throw Error("Invalid unit graph: " + failures.join("; "));
  return value as UnitGraphV1;
}

/** 🗺️ The derived shape of a valid graph: dependency order, layer (longest path from a unit without dependencies) and reverse edges. */
export type UnitPlanV1 = {
  readonly graph: UnitGraphV1;
  readonly byId: ReadonlyMap<string, UnitV1>;
  readonly order: readonly string[];
  readonly layer: Readonly<Record<string, number>>;
  readonly dependents: Readonly<Record<string, readonly string[]>>;
};
/** 🗺️ Plans a valid graph: a stable topological order (by layer, then code point), layers and dependents. */
export function planUnitsV1(graph: UnitGraphV1): UnitPlanV1 {
  const byId = new Map(graph.units.map(unit => [unit.id, unit] as const)), layer: Record<string, number> = {}, dependents: Record<string, string[]> = {};
  for (const unit of graph.units) dependents[unit.id] = [];
  for (const unit of graph.units) for (const dependency of unit.dependencies) dependents[dependency.unit]!.push(unit.id);
  const visit = (id: string, trail: Set<string>): number => {
    const known = layer[id];
    if (known !== undefined) return known;
    if (trail.has(id)) throw Error("dependency cycle at " + id);
    trail.add(id);
    let height = 0;
    for (const dependency of byId.get(id)!.dependencies) height = Math.max(height, visit(dependency.unit, trail) + 1);
    trail.delete(id);
    return layer[id] = height;
  };
  for (const unit of graph.units) visit(unit.id, new Set());
  for (const id of Object.keys(dependents)) dependents[id] = sortedCodePointsV1(dependents[id]!);
  const order = sortedCodePointsV1(byId.keys()).sort((a, b) => layer[a]! - layer[b]!);
  return {graph, byId, order, layer, dependents};
}

/** 🎯️ The units a set of changed files can affect: owners of the files and every unit that depends on them, directly or not (edges of every use select). */
export function affectedUnitsV1(plan: UnitPlanV1, changed: readonly string[]): readonly string[] {
  const touched = new Set(changed), hit = new Set<string>(), queue: string[] = [];
  for (const unit of plan.graph.units) if (unit.sources.some(path => touched.has(path)) || unit.inputs.some(path => touched.has(path))) {hit.add(unit.id); queue.push(unit.id);}
  while (queue.length) for (const next of plan.dependents[queue.pop()!]!) if (!hit.has(next)) {hit.add(next); queue.push(next);}
  return plan.order.filter(id => hit.has(id));
}

/** ⏱️ The critical path of a plan for given unit weights (milliseconds): the heaviest dependency chain and its length. */
export function criticalPathV1(plan: UnitPlanV1, weight: (id: string) => number): {readonly milliseconds: number; readonly path: readonly string[]} {
  const best = new Map<string, {milliseconds: number; previous: string | undefined}>();
  for (const id of plan.order) {
    let previous: string | undefined, base = -1;
    for (const dependency of sortedCodePointsV1(plan.byId.get(id)!.dependencies.map(row => row.unit))) {const milliseconds = best.get(dependency)!.milliseconds; if (milliseconds > base) {base = milliseconds; previous = dependency;}}
    best.set(id, {milliseconds: (previous === undefined ? 0 : base) + weight(id), previous});
  }
  let tail: string | undefined, longest = -1;
  for (const id of plan.order) {const row = best.get(id)!; if (row.milliseconds > longest) {longest = row.milliseconds; tail = id;}}
  const path: string[] = [];
  for (let at = tail; at !== undefined; at = best.get(at)!.previous) path.unshift(at);
  return {milliseconds: Math.max(longest, 0), path};
}
