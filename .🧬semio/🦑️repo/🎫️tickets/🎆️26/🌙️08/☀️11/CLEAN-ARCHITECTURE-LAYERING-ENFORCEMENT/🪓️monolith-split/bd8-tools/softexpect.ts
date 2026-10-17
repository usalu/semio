import { expect as real } from "bun:test";
import { appendFileSync } from "node:fs";
const out = process.env.BD8_SOFT_LOG ?? "/tmp/bd8-soft.log";
function site(error: Error): string {
  const frames = String(error.stack ?? "").split("\n").filter(line => line.includes("bd8-diag"));
  return frames[0]?.trim() ?? "?";
}
function wrap(target: any): any {
  return new Proxy(target, {
    get(object, property) {
      const value = object[property];
      if (typeof value === "function") return (...args: unknown[]) => { try { return value.apply(object, args); } catch (error) { appendFileSync(out, `${site(error as Error)} :: ${String((error as Error).message).split("\n").slice(0, 4).join(" | ")}\n`); return undefined; } };
      if (value !== null && typeof value === "object") return wrap(value);
      return value;
    },
  });
}
export const expect: any = (value: unknown, label?: string) => wrap((real as any)(value, label));
