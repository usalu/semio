import { test, expect } from "bun:test";
const oldMod = await import(process.env.PD_OLD!);
const newMod = await import(process.env.PD_NEW!);
const fn = process.env.PD_FN!;
test(`${fn}: old and new return the identical value`, () => {
  const a = oldMod[fn](), b = newMod[fn]();
  expect(b).toBe(a);
});
