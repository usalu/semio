import type { BreachRecord } from "./breach.ts";
import type { BaseLinter } from "./linter.ts";

export type LintFn<T extends BaseLinter> = (linter: T) => BreachRecord[] | Promise<BreachRecord[]>;

/** 📜Tags a lint callback for tooling (runner unwraps default export). */
export function defineLint<T extends BaseLinter>(_tag: string, fn: LintFn<T>): LintFn<T> {
  return fn;
}
