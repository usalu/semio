import { AssertionError } from "node:assert";
import { inspect, isDeepStrictEqual } from "node:util";
import type { RuntimeMatchersV1 } from "../../📋️contract/🟦️.ts";

function looseEqual(left: unknown, right: unknown): boolean {
  if (Object.is(left, right)) return true;
  if (typeof left !== "object" || typeof right !== "object" || left === null || right === null) return false;
  if (left instanceof Date || right instanceof Date) return left instanceof Date && right instanceof Date && left.getTime() === right.getTime();
  if (left instanceof RegExp || right instanceof RegExp) return String(left) === String(right);
  if (left instanceof Map || right instanceof Map) return left instanceof Map && right instanceof Map && left.size === right.size && [...left].every(([key, value]) => right.has(key) && looseEqual(value, right.get(key)));
  if (left instanceof Set || right instanceof Set) return left instanceof Set && right instanceof Set && left.size === right.size && [...left].every(value => right.has(value) || [...right].some(other => looseEqual(value, other)));
  if (ArrayBuffer.isView(left) || ArrayBuffer.isView(right)) return ArrayBuffer.isView(left) && ArrayBuffer.isView(right) && isDeepStrictEqual(Buffer.from(left.buffer, left.byteOffset, left.byteLength), Buffer.from(right.buffer, right.byteOffset, right.byteLength));
  if (Array.isArray(left) !== Array.isArray(right)) return false;
  if (Array.isArray(left)) return left.length === (right as unknown[]).length && left.every((value, index) => looseEqual(value, (right as unknown[])[index]));
  const keys = (value: object): string[] => Object.keys(value).filter(key => (value as Record<string, unknown>)[key] !== undefined);
  const leftKeys = keys(left), rightKeys = keys(right);
  return leftKeys.length === rightKeys.length && leftKeys.every(key => Object.hasOwn(right, key) && looseEqual((left as Record<string, unknown>)[key], (right as Record<string, unknown>)[key]));
}

function subsetOf(actual: unknown, expected: unknown): boolean {
  if (expected === null || typeof expected !== "object") return looseEqual(actual, expected);
  if (actual === null || typeof actual !== "object") return false;
  if (Array.isArray(expected)) return Array.isArray(actual) && actual.length === expected.length && expected.every((value, index) => subsetOf(actual[index], value));
  return Object.entries(expected).every(([key, value]) => subsetOf((actual as Record<string, unknown>)[key], value));
}

function fail(message: string | undefined, text: string, actual: unknown, expected: unknown): never {
  throw new AssertionError({ message: `${message === undefined ? "" : `${message}: `}${text}`, actual, expected });
}

function errorMatches(error: unknown, expected: RegExp | string | Function | undefined): boolean {
  if (expected === undefined) return true;
  const text = error instanceof Error ? error.message : String(error);
  if (typeof expected === "string") return text.includes(expected);
  if (expected instanceof RegExp) return expected.test(text);
  return error instanceof (expected as new (...arguments_: unknown[]) => unknown);
}

function matchersFor(actual: unknown, message: string | undefined, negate: boolean, mode: "value" | "resolves" | "rejects"): RuntimeMatchersV1 {
  const check = (passed: boolean, text: string, expected?: unknown): void => { if (passed === negate) fail(message, `expected ${inspect(actual, { depth: 3 })}${negate ? " not" : ""} ${text}`, actual, expected); };
  const settle = (body: (value: unknown) => void): unknown => {
    if (mode === "value") return body(actual);
    return Promise.resolve(typeof actual === "function" && mode === "rejects" ? (actual as () => unknown)() : actual).then(
      value => { if (mode === "rejects") fail(message, "expected the promise to reject", value, undefined); body(value); },
      error => { if (mode === "resolves") throw error; body(error); },
    );
  };
  const make = (_name: string, body: (value: unknown, ...rest: any[]) => void) => (...rest: unknown[]): unknown => settle(value => body(value, ...rest));
  const api: Record<string, unknown> = {
    toBe: make("toBe", (value, expected) => check(Object.is(value, expected), `to be ${inspect(expected)}`, expected)),
    toEqual: make("toEqual", (value, expected) => check(looseEqual(value, expected), `to equal ${inspect(expected, { depth: 3 })}`, expected)),
    toStrictEqual: make("toStrictEqual", (value, expected) => check(isDeepStrictEqual(value, expected), `to strictly equal ${inspect(expected, { depth: 3 })}`, expected)),
    toMatchObject: make("toMatchObject", (value, expected) => check(subsetOf(value, expected), `to match ${inspect(expected, { depth: 3 })}`, expected)),
    toBeTruthy: make("toBeTruthy", value => check(Boolean(value), "to be truthy")),
    toBeFalsy: make("toBeFalsy", value => check(!value, "to be falsy")),
    toBeNull: make("toBeNull", value => check(value === null, "to be null")),
    toBeUndefined: make("toBeUndefined", value => check(value === undefined, "to be undefined")),
    toBeDefined: make("toBeDefined", value => check(value !== undefined, "to be defined")),
    toBeNaN: make("toBeNaN", value => check(Number.isNaN(value), "to be NaN")),
    toBeGreaterThan: make("toBeGreaterThan", (value, expected) => check((value as number) > expected, `to be greater than ${String(expected)}`, expected)),
    toBeGreaterThanOrEqual: make("toBeGreaterThanOrEqual", (value, expected) => check((value as number) >= expected, `to be at least ${String(expected)}`, expected)),
    toBeLessThan: make("toBeLessThan", (value, expected) => check((value as number) < expected, `to be less than ${String(expected)}`, expected)),
    toBeLessThanOrEqual: make("toBeLessThanOrEqual", (value, expected) => check((value as number) <= expected, `to be at most ${String(expected)}`, expected)),
    toBeCloseTo: make("toBeCloseTo", (value, expected, digits = 2) => check(Math.abs((value as number) - expected) < 10 ** -digits / 2, `to be close to ${String(expected)}`, expected)),
    toContain: make("toContain", (value, expected) => check(typeof value === "string" ? value.includes(expected) : [...(value as Iterable<unknown>)].includes(expected), `to contain ${inspect(expected)}`, expected)),
    toContainEqual: make("toContainEqual", (value, expected) => check([...(value as Iterable<unknown>)].some(item => looseEqual(item, expected)), `to contain an item equal to ${inspect(expected)}`, expected)),
    toHaveLength: make("toHaveLength", (value, expected) => check((value as { length: number }).length === expected, `to have length ${String(expected)}`, expected)),
    toHaveProperty: make("toHaveProperty", (value, path, expected) => { let current: unknown = value, found = true; for (const part of String(path).split(".")) { if (current !== null && current !== undefined && part in Object(current)) current = (current as Record<string, unknown>)[part]; else { found = false; break; } } check(found && (expected === undefined || looseEqual(current, expected)), `to have property ${String(path)}`, expected); }),
    toMatch: make("toMatch", (value, expected) => check(typeof expected === "string" ? String(value).includes(expected) : (expected as RegExp).test(String(value)), `to match ${String(expected)}`, expected)),
    toBeInstanceOf: make("toBeInstanceOf", (value, expected) => check(value instanceof (expected as new () => unknown), `to be an instance of ${(expected as Function).name}`, expected)),
    toThrow: (expected?: RegExp | string | Function): unknown => {
      if (mode !== "value") return settle(error => check(errorMatches(error, expected), `to throw ${String(expected ?? "")}`, expected));
      let thrown: unknown, threw = false;
      try { (actual as () => unknown)(); } catch (error) { threw = true; thrown = error; }
      check(threw && errorMatches(thrown, expected), `to throw ${String(expected ?? "")}`, expected);
      return undefined;
    },
  };
  Object.defineProperty(api, "not", { get: () => matchersFor(actual, message, !negate, mode) });
  Object.defineProperty(api, "resolves", { get: () => matchersFor(actual, message, negate, "resolves") });
  Object.defineProperty(api, "rejects", { get: () => matchersFor(actual, message, negate, "rejects") });
  return api as unknown as RuntimeMatchersV1;
}

/** ✅️ Builds the thin `expect` of the framework's test API on `node:assert` and `node:util`; the closed matcher subset the framework's own tests use. */
export function createExpectV1(): (actual: unknown, message?: string) => RuntimeMatchersV1 {
  return (actual, message) => matchersFor(actual, message, false, "value");
}
