import { requireRuntimeMemberV1 } from "../🧭️selection/🟦️.ts";

const api = requireRuntimeMemberV1("testApi")();

/** 🧪️ The thin test API of the hosting runtime's test runner: `bun test` and `node --test` run the same test file. */
export const { test, describe, beforeAll, afterAll, beforeEach, afterEach, expect } = api;
