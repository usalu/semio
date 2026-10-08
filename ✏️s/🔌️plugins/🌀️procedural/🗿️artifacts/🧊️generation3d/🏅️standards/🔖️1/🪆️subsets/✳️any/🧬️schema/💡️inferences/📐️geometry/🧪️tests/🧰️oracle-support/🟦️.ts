/** 🧰️ Shared helpers of the third-party oracle tests of the generation3d geometry widget computes: fixture case shapes, tolerant deep comparison and the case runner. */

export type FixtureFault = { code: string; port?: string };
export type FixtureCase = { name: string; kind: string; inputs: Record<string, unknown>; outputs?: Record<string, unknown>; fault?: FixtureFault; tolerance?: number };
export type OracleResult = { outputs: Record<string, unknown> } | { fault: FixtureFault };

/** 🔢️ Absolute below magnitude one, relative above, so the same tolerance serves unit-scale and large values. */
export const closeTo = (actual: unknown, expected: unknown, tolerance: number): boolean => {
  if (typeof expected === "number") return typeof actual === "number" && Number.isFinite(actual) && Math.abs(actual - expected) <= tolerance * Math.max(1, Math.abs(expected));
  if (Array.isArray(expected)) return Array.isArray(actual) && actual.length === expected.length && expected.every((item, index) => closeTo(actual[index], item, tolerance));
  if (expected !== null && typeof expected === "object") {
    const entries = Object.entries(expected as Record<string, unknown>);
    return actual !== null && typeof actual === "object" && Object.keys(actual as object).length === entries.length && entries.every(([key, item]) => closeTo((actual as Record<string, unknown>)[key], item, tolerance));
  }
  return actual === expected;
};

/** 🧪️ Runs every fixture case through `oracle` and returns the names of the cases it disagrees with, each with the reason. */
export const disagreements = (cases: readonly FixtureCase[], oracle: (fixtureCase: FixtureCase) => OracleResult, tolerance: number): string[] => {
  const failures: string[] = [];
  for (const fixtureCase of cases) {
    const result = oracle(fixtureCase);
    const tol = fixtureCase.tolerance ?? tolerance;
    if (fixtureCase.fault) {
      if (!("fault" in result)) failures.push(`${fixtureCase.name}: expected fault ${fixtureCase.fault.code}, oracle produced ${JSON.stringify(result.outputs)}`);
      else if (result.fault.code !== fixtureCase.fault.code || result.fault.port !== fixtureCase.fault.port) failures.push(`${fixtureCase.name}: expected fault ${JSON.stringify(fixtureCase.fault)}, oracle faulted ${JSON.stringify(result.fault)}`);
    } else if ("fault" in result) failures.push(`${fixtureCase.name}: oracle faulted ${JSON.stringify(result.fault)}, fixture expects outputs`);
    else if (!closeTo(result.outputs, fixtureCase.outputs, tol)) failures.push(`${fixtureCase.name}: expected ${JSON.stringify(fixtureCase.outputs)}, oracle produced ${JSON.stringify(result.outputs)}`);
  }
  return failures;
};
