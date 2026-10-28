import { activeTestLevel, budgetStretchV1, sharedHostContentionV1, TEST_LEVELS, type TestLevel, testLevelBudgetMs } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";

/** 🐌️ The contention of this host at launch (the shared load-aware budget clock source): inner tool timeouts count at its stretch so a busy host does not kill a run that fits its budget on an idle one.
 * @see ../../../../../../🔨️modules/🏃️process/🧪️testing/🎚️budget/⏲️contention/🟦️.ts */
export function launchContentionRatioV1(): number {
  return sharedHostContentionV1()().ratio;
}

/** ⏱️ The level budget in milliseconds stretched by the contention `ratio`, clamped to the budget clock's policy bounds (an idle host keeps the plain budget). */
export function stretchedLevelBudgetMsV1(level: TestLevel, ratio: number): number {
  return Math.ceil(testLevelBudgetMs(level) * budgetStretchV1(ratio));
}

function levelsAbove(level: TestLevel): readonly TestLevel[] {
  return TEST_LEVELS.slice(TEST_LEVELS.indexOf(level) + 1);
}

/** 🐹️Cumulative `go test` args for the active level: per-level `-timeout` (a whole-binary wall clock, so stretched by the host's contention at launch), keeps `-short` through `quick`, adds `-skip` for `Test<Level>`-prefixed tests above it. */
export function goLevelTestArgs(level: TestLevel = activeTestLevel(), ratio: number = launchContentionRatioV1()): string[] {
  const args: string[] = ["-timeout", `${Math.ceil(stretchedLevelBudgetMsV1(level, ratio) / 1000)}s`];
  if (TEST_LEVELS.indexOf(level) <= TEST_LEVELS.indexOf("quick")) args.push("-short");
  const skipped = levelsAbove(level).map((l) => l[0]!.toUpperCase() + l.slice(1));
  if (skipped.length) args.push("-skip", `^Test(${skipped.join("|")})`);
  return args;
}

/** 🪝️Vitest per-test/hook/teardown timeouts (ms) for the active level budget. */
export function vitestLevelArgs(level: TestLevel = activeTestLevel(), ratio: number = launchContentionRatioV1()): string[] {
  const ms = String(stretchedLevelBudgetMsV1(level, ratio));
  return ["--testTimeout", ms, "--hookTimeout", ms, "--teardownTimeout", ms];
}

/** ⏱️`bun test` `--timeout` (ms) for the active level budget. */
export function bunTestLevelArgs(level: TestLevel = activeTestLevel(), ratio: number = launchContentionRatioV1()): string[] {
  return ["--timeout", String(stretchedLevelBudgetMsV1(level, ratio))];
}

/** 🐍️Cumulative pytest args for the active level: `-m` marker filter, per-test `--timeout`, and `--timeout-method=thread`. */
export function pytestLevelArgs(level: TestLevel = activeTestLevel(), ratio: number = launchContentionRatioV1()): string[] {
  const excluded = levelsAbove(level);
  const args = excluded.length ? ["-m", excluded.map((l) => `not ${l}`).join(" and ")] : [];
  args.push(`--timeout=${Math.ceil(stretchedLevelBudgetMsV1(level, ratio) / 1000)}`, "--timeout-method=thread");
  return args;
}

/** 🔷️Cumulative dotnet xunit args for the active level: `--filter` for levels above it and `--blame-hang` with a per-test timeout (ms). */
export function dotnetLevelArgs(level: TestLevel = activeTestLevel(), ratio: number = launchContentionRatioV1()): string[] {
  const excluded = levelsAbove(level);
  const args = excluded.length ? ["--filter", excluded.map((l) => `Category!=${l}`).join("&")] : [];
  args.push("--blame-hang", "--blame-hang-timeout", String(stretchedLevelBudgetMsV1(level, ratio)));
  return args;
}

/** 🎭️Playwright per-spec timeout (ms) — active level budget via [[testLevelBudgetMs]]. */
export function playwrightTestTimeoutMs(level: TestLevel = activeTestLevel(), ratio: number = launchContentionRatioV1()): number {
  return stretchedLevelBudgetMsV1(level, ratio);
}
