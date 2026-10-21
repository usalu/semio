import { describe, expect, test } from "../../🧪️test/🟦️.ts";

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
