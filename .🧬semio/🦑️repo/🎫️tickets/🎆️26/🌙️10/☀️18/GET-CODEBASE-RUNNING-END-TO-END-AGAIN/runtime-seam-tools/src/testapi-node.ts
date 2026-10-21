import { createExpectV1 } from "../../🪶️portable/✅️expect/🟦️.ts";
import type { RuntimeTestApiV1, RuntimeTestBodyV1, RuntimeTestCommandOptionsV1, RuntimeTestFunctionV1, RuntimeTestOptionsV1 } from "../../📋️contract/🟦️.ts";

type NodeTestV1 = { (name: string, options: { timeout?: number }, body: RuntimeTestBodyV1): unknown; skip: NodeTestV1; only: NodeTestV1; todo: NodeTestV1 };

function timeout(options: RuntimeTestOptionsV1 | undefined): { timeout?: number } {
  const value = typeof options === "number" ? options : options?.timeout;
  return value === undefined ? {} : { timeout: value };
}

/** 🧪️ Projects `node:test` and the portable `expect` onto the thin test API. */
export function nodeTestApiV1(): RuntimeTestApiV1 {
  const node = process.getBuiltinModule("node:test") as unknown as { test: NodeTestV1; describe: (name: string, body: () => void) => void; before: (body: RuntimeTestBodyV1) => void; after: (body: RuntimeTestBodyV1) => void; beforeEach: (body: RuntimeTestBodyV1) => void; afterEach: (body: RuntimeTestBodyV1) => void };
  const declare = (variant: NodeTestV1) => (name: string, body: RuntimeTestBodyV1, options?: RuntimeTestOptionsV1): void => { void variant(name, timeout(options), body); };
  const test = Object.assign(declare(node.test), { skip: declare(node.test.skip), only: declare(node.test.only), todo: (name: string, body?: RuntimeTestBodyV1): void => { void node.test.todo(name, {}, body ?? (() => undefined)); } }) as RuntimeTestFunctionV1;
  return { test, describe: node.describe, beforeAll: node.before, afterAll: node.after, beforeEach: node.beforeEach, afterEach: node.afterEach, expect: createExpectV1() };
}

/** 🧪️ Builds the arguments that make `node` run test files. */
export function nodeTestCommandV1(files: readonly string[], options: RuntimeTestCommandOptionsV1 = {}): string[] {
  return ["--test", ...(options.timeoutMilliseconds === undefined ? [] : [`--test-timeout=${options.timeoutMilliseconds}`]), ...(options.filter === undefined ? [] : [`--test-name-pattern=${options.filter}`]), ...(options.concurrency === undefined ? [] : [`--test-concurrency=${options.concurrency}`]), ...files];
}
