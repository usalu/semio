import { expect } from "bun:test";
declare const a: number[] | undefined;
declare const r: readonly string[];
expect(a).toEqual([1]);
expect(r).toEqual(["x"]);
expect(1 as number).toBe("s");
const s: string[] = [];
expect(s).toContain(5);
