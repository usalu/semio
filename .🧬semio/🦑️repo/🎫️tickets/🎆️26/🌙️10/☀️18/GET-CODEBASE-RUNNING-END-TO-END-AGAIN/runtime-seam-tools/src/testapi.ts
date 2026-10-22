import { requireRuntimeMemberV1, runtimeMessageV1 } from "../🧭️selection/🟦️.ts";

const api = (() => { try { return requireRuntimeMemberV1("testApi")(); } catch (error) { throw new Error(runtimeMessageV1("testRunnerOnly", {}), { cause: error }); } })();

/** 🧪️ The thin test API of the hosting runtime's test runner: `bun test` and `node --test` run the same test file. */
export const { test, describe, beforeAll, afterAll, beforeEach, afterEach, expect } = api;
