import {createHash} from "node:crypto";
import {compareCodePointsV1, sortedCodePointsV1, type UnitsPolicyV1, type UnitV1} from "../🕸️graph/🟦️.ts";

/** 🔏️ Lowercase hex SHA-256 of text (UTF-8) or bytes. */
export const sha256V1 = (value: string | Uint8Array): string => createHash("sha256").update(value).digest("hex");

/** 🧱️ Canonical JSON: object keys sorted by code point, no whitespace, strings as `JSON.stringify` writes them. Numbers must be integers; `undefined` is refused. */
export function canonicalJsonV1(value: unknown): string {
  if (value === null || typeof value === "string" || typeof value === "boolean") return JSON.stringify(value);
  if (typeof value === "number") {if (!Number.isInteger(value)) throw Error("canonical JSON takes integers only"); return String(value);}
  if (Array.isArray(value)) return "[" + value.map(canonicalJsonV1).join(",") + "]";
  if (typeof value === "object") return "{" + sortedCodePointsV1(Object.keys(value as object)).map(key => JSON.stringify(key) + ":" + canonicalJsonV1((value as Record<string, unknown>)[key])).join(",") + "}";
  throw Error("canonical JSON cannot encode " + typeof value);
}

/** 🧮️ What a finished dependency contributes to its dependents. */
export type UnitResultFactsV1 = {readonly interfaceClosure: string | null; readonly outputClosure: string};

const rows = (paths: readonly string[], digest: (path: string) => string): (readonly [string, string])[] => sortedCodePointsV1(paths).map(path => [path, digest(path)] as const);

/** 🔑️ The content address of one unit run: everything the unit itself declares plus the interface closure (use `interface`) or output closure (use `implementation`) of each dependency. A dependency whose interface did not change leaves the key of its dependents unchanged. */
export function unitKeyV1(unit: UnitV1, input: {readonly digest: (path: string) => string; readonly dependencies: ReadonlyMap<string, UnitResultFactsV1>}): string {
  const dependencies = [...unit.dependencies].sort((a, b) => compareCodePointsV1(a.unit, b.unit)).map(dependency => {
    const facts = input.dependencies.get(dependency.unit);
    if (dependency.use !== "declared" && !facts) throw Error(`no result for dependency ${dependency.unit} of ${unit.id}`);
    const digest = dependency.use === "declared" ? "" : dependency.use === "interface" ? facts!.interfaceClosure : facts!.outputClosure;
    if (digest === null) throw Error(`dependency ${dependency.unit} of ${unit.id} has no interface`);
    return [dependency.unit, dependency.use, digest];
  });
  return sha256V1(canonicalJsonV1({
    v: 1,
    id: unit.id,
    language: unit.language,
    kind: unit.kind,
    toolchain: {id: unit.toolchain.id, parts: unit.toolchain.parts},
    action: {argv: unit.action.argv, env: unit.action.env},
    interface: unit.interface ? unit.interface.kind : null,
    outputs: sortedCodePointsV1(unit.outputs),
    sources: rows(unit.sources, input.digest),
    inputs: rows(unit.inputs, input.digest),
    externals: sortedCodePointsV1(unit.externals),
    dependencies,
  }));
}

/** 🔭️ The interface closure of a unit: its own interface digest and the closures of the dependencies whose interface it consumes and exposes (`exposes`: the interface dependencies the executor reported its interface mentions; null means all), unless the interface kind is self-contained (`direct`). */
export function interfaceClosureV1(unit: UnitV1, ownInterface: string | null, dependencies: ReadonlyMap<string, UnitResultFactsV1>, policy?: UnitsPolicyV1, exposes: readonly string[] | null = null): string | null {
  if (ownInterface === null) return null;
  const direct = policy && unit.interface ? policy.interfaceKinds[unit.interface.kind]?.closure === "direct" : false;
  const rowsOf = direct ? [] : unit.dependencies.filter(dependency => dependency.use === "interface" && (exposes === null || exposes.includes(dependency.unit))).map(dependency => [dependency.unit, dependencies.get(dependency.unit)!.interfaceClosure] as const).sort((a, b) => compareCodePointsV1(a[0], b[0]));
  return sha256V1(canonicalJsonV1({i: ownInterface, d: rowsOf}));
}

/** 🏁️ The output closure of a unit: its own outputs digest and the output closures of every dependency it consumes. */
export function outputClosureV1(unit: UnitV1, ownOutputs: string, dependencies: ReadonlyMap<string, UnitResultFactsV1>): string {
  const rowsOf = unit.dependencies.filter(dependency => dependency.use !== "declared").map(dependency => [dependency.unit, dependencies.get(dependency.unit)!.outputClosure] as const).sort((a, b) => compareCodePointsV1(a[0], b[0]));
  return sha256V1(canonicalJsonV1({o: ownOutputs, d: rowsOf}));
}

/** 📄️ The interface file a unit publishes for build tools that hash files: dependents list this file and nothing else of the unit. */
export function unitInterfaceFileV1(unit: UnitV1, interfaceDigest: string, closure: string, artifact: string | null = null): {readonly version: 1; readonly unit: string; readonly kind: string; readonly interface: string; readonly closure: string; readonly artifact: string | null} {
  return {version: 1, unit: unit.id, kind: unit.interface!.kind, interface: interfaceDigest, closure, artifact};
}
