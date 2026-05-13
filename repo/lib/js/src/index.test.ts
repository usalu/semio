import { describe, expect, test } from "bun:test";
import { defineLint } from "./script.ts";
import type { FileLinter } from "./linter.ts";

describe("defineLint", () => {
  test("returns same function", () => {
    const f = defineLint("x", (_l: FileLinter) => []);
    expect(typeof f).toBe("function");
  });
});
