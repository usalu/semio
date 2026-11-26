/** 🧽️ A recording stand-in for `describe`, `it` and `expect` of vitest: a case runs to its end, every assertion that would fail is listed with its label, the value and the wanted value (a missing value still ends the view it is asked in). Used by `soft.ts` to measure every assertion of a suite, not only the first red one; it never replaces the suite. */
type Failure = { readonly label: string; readonly matcher: string; readonly got: unknown; readonly want: unknown };
type Case = { readonly name: string; readonly run: () => unknown };
export const cases: Case[] = [];
export const failures: Failure[] = [];
const path: string[] = [];
const show = (value: unknown): unknown => (typeof value === "number" ? Number(value.toPrecision(6)) : value);

export function describe(name: string, body: () => void): void {
  path.push(name);
  body();
  path.pop();
}

const format = (name: string, value: unknown): string => name.replace(/%[sid]/, String(Array.isArray(value) ? value[0] : value));
export const it = Object.assign((name: string, run: () => unknown): void => void cases.push({ name: [...path, name].join(" > "), run }), {
  each: (list: readonly unknown[]) => (name: string, run: (...args: any[]) => unknown): void => {
    for (const value of list) cases.push({ name: [...path, format(name, value)].join(" > "), run: () => (Array.isArray(value) ? run(...value) : run(value)) });
  },
});

export function expect(got: any, label = ""): Record<string, (...args: any[]) => void> {
  const check = (matcher: string, ok: boolean, want: unknown, fatal = false): void => {
    if (ok) return;
    failures.push({ label, matcher, got: show(Array.isArray(got) && got.length > 8 ? `[${got.length} items]` : got), want: show(want) });
    if (fatal) throw new Error(`missing: ${label}`);
  };
  return {
    toBeDefined: () => check("defined", got !== undefined, "defined", true),
    toBe: (want: unknown) => check("toBe", Object.is(got, want), want),
    toEqual: (want: unknown) => check("toEqual", JSON.stringify(got) === JSON.stringify(want), want),
    toHaveLength: (want: number) => check("length", got?.length === want, want),
    toBeCloseTo: (want: number, digits = 2) => check(`closeTo(${digits})`, Math.abs(got - want) < 10 ** -digits / 2, want),
    toBeGreaterThan: (want: number) => check(">", got > want, want),
    toBeGreaterThanOrEqual: (want: number) => check(">=", got >= want, want),
    toBeLessThan: (want: number) => check("<", got < want, want),
    toBeLessThanOrEqual: (want: number) => check("<=", got <= want, want),
  };
}
