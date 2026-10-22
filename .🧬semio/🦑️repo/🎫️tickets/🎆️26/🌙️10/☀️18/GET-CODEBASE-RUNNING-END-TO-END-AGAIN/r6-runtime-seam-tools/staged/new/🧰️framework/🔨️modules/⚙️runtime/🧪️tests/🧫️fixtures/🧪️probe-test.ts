import { afterAll, describe, expect, test } from "../../🧪️test/🟦️.ts";

describe("probe", () => {
  test("synchronous matchers", () => {
    expect(1 + 1).toBe(2);
    expect({ a: 1, b: undefined }).toEqual({ a: 1 });
    expect([1, 2, 3]).toContain(2);
    expect(() => { throw new Error("boom"); }).toThrow("boom");
    expect("abc").not.toBe("abd");
    if (process.env.SEMIO_PROBE_FAIL === "1") expect(1).toBe(2);
  });
  test("asynchronous matchers", async () => {
    await expect(Promise.resolve(3)).resolves.toBe(3);
    await expect(Promise.reject(new Error("x"))).rejects.toThrow("x");
  });
});

describe("probe variants", () => {
  afterAll(() => undefined, 5000);
  test("a completion callback ends the test", (done: (error?: unknown) => void) => { setTimeout(() => { done(); }, 1); }, 5000);
  test("a body may return a value that is not void", async () => { expect(1).toBe(1); return 7; });
  test.skipIf(true)("a skipped test never runs", () => { throw new Error("must not run"); });
  test.skipIf(false)("a test that is not skipped runs", () => { expect([1]).toHaveLength(1); });
});

