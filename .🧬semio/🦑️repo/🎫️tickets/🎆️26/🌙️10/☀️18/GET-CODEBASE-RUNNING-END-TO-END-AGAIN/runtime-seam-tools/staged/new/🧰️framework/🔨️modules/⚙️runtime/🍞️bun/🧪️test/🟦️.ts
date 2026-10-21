import type { RuntimeTestApiV1, RuntimeTestCommandOptionsV1 } from "../../📋️contract/🟦️.ts";

/** 🧪️ Exposes the Bun test runner's own functions as the thin test API; valid inside a `bun test` process. */
export function bunTestApiV1(): RuntimeTestApiV1 {
  const loaded = process.getBuiltinModule("bun:test") as unknown as RuntimeTestApiV1 | undefined;
  if (loaded === undefined) throw new Error("The Bun test runner is only available inside `bun test`");
  const { test, describe, beforeAll, afterAll, beforeEach, afterEach, expect } = loaded;
  return { test, describe, beforeAll, afterAll, beforeEach, afterEach, expect };
}

/** 🧪️ Builds the arguments that make `bun` run test files. */
export function bunTestCommandV1(files: readonly string[], options: RuntimeTestCommandOptionsV1 = {}): string[] {
  return ["test", ...(options.timeoutMilliseconds === undefined ? [] : ["--timeout", String(options.timeoutMilliseconds)]), ...(options.filter === undefined ? [] : ["-t", options.filter]), ...files];
}
