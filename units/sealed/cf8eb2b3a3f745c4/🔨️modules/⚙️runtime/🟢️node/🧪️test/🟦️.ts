import { createExpectV1 } from "../../🪶️portable/✅️expect/🟦️.ts";
import type { RuntimeTestApiV1, RuntimeTestBodyV1, RuntimeTestCommandOptionsV1, RuntimeTestFunctionV1, RuntimeTestOptionsV1 } from "../../📋️contract/🟦️.ts";

type NodeTestV1 = { (name: string, options: { timeout?: number }, body: () => unknown): unknown; skip: NodeTestV1; only: NodeTestV1; todo: NodeTestV1 };
type NodeHookV1 = (body: () => unknown, options?: { timeout?: number }) => void;

function timeout(options: RuntimeTestOptionsV1 | undefined): { timeout?: number } {
  const value = typeof options === "number" ? options : options?.timeout;
  return value === undefined ? {} : { timeout: value };
}

function settled(body: RuntimeTestBodyV1): () => unknown {
  return body.length === 0 ? () => body(() => undefined) : () => new Promise<void>((resolve, reject) => { body(error => (error === undefined || error === null ? resolve() : reject(error))); });
}

/** 🧪️ Projects `node:test` and the portable `expect` onto the thin test API. */
export function nodeTestApiV1(): RuntimeTestApiV1 {
  const node = process.getBuiltinModule("node:test") as unknown as { test: NodeTestV1; describe: (name: string, body: () => void) => void; before: NodeHookV1; after: NodeHookV1; beforeEach: NodeHookV1; afterEach: NodeHookV1 };
  const declare = (variant: NodeTestV1) => (name: string, body: RuntimeTestBodyV1, options?: RuntimeTestOptionsV1): void => { void variant(name, timeout(options), settled(body)); };
  const hook = (variant: NodeHookV1) => (body: RuntimeTestBodyV1, options?: RuntimeTestOptionsV1): void => { variant(settled(body), timeout(options)); };
  const variants = (condition: boolean): RuntimeTestFunctionV1 => (condition ? test.skip : test) as unknown as RuntimeTestFunctionV1;
  const test: RuntimeTestFunctionV1 = Object.assign(declare(node.test), { skip: declare(node.test.skip), only: declare(node.test.only), todo: (name: string, body?: RuntimeTestBodyV1): void => { void node.test.todo(name, {}, body === undefined ? () => undefined : settled(body)); }, skipIf: (condition: unknown): RuntimeTestFunctionV1 => variants(Boolean(condition)) }) as unknown as RuntimeTestFunctionV1;
  return { test, describe: node.describe, beforeAll: hook(node.before), afterAll: hook(node.after), beforeEach: hook(node.beforeEach), afterEach: hook(node.afterEach), expect: createExpectV1() };
}

/** 🧪️ Builds the arguments that make `node` run test files. */
export function nodeTestCommandV1(files: readonly string[], options: RuntimeTestCommandOptionsV1 = {}): string[] {
  return ["--test", ...(options.timeoutMilliseconds === undefined ? [] : [`--test-timeout=${options.timeoutMilliseconds}`]), ...(options.filter === undefined ? [] : [`--test-name-pattern=${options.filter}`]), ...(options.concurrency === undefined ? [] : [`--test-concurrency=${options.concurrency}`]), ...files];
}

/** 🧪️ Inserts a preload module before the `--test` flag; other commands and commands that already preload have no answer. */
export function nodeTestPreloadV1(args: readonly string[], preload: string): readonly string[] | undefined {
  return args.includes("--test") && !args.includes("--import") ? ["--import", preload, ...args] : undefined;
}
