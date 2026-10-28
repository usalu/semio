"""✏️ innerclock edits on the staged copies (`python3 -I edit.py`, idempotent on a fresh copy of base)."""
import os, shutil
HERE = os.path.dirname(os.path.abspath(__file__))
def sub(name, pairs):
    p = os.path.join(HERE, "files", name); t = open(p, encoding="utf8").read()
    for old, new in pairs:
        assert t.count(old) == 1, (name, old[:70], t.count(old)); t = t.replace(old, new)
    open(p, "w", encoding="utf8").write(t)
for n in ("f01.ts", "f02.ts", "f03.ts"): shutil.copyfile(os.path.join(HERE, "base", n), os.path.join(HERE, "files", n))
sub("f01.ts", [('export type { TestLevel } from "../../../../🧪️testing/🎚️budget/🟦️.ts";', 'export type { TestLevel } from "../../../../🧪️testing/🎚️budget/🟦️.ts";\nexport { budgetStretchV1, sharedHostContentionV1 } from "../../../../🧪️testing/🎚️budget/⏲️contention/🟦️.ts";')])
sub("f02.ts", [
 ('import { activeTestLevel, TEST_LEVELS, type TestLevel, testLevelBudgetMs, testLevelBudgetSeconds } from', 'import { activeTestLevel, budgetStretchV1, sharedHostContentionV1, TEST_LEVELS, type TestLevel, testLevelBudgetMs } from'),
 ('function levelsAbove(level: TestLevel): readonly TestLevel[] {', '''/** 🐌️ The contention of this host at launch (the shared load-aware budget clock source): inner tool timeouts count at its stretch so a busy host does not kill a run that fits its budget on an idle one.
 * @see ../../../../../../🔨️modules/🏃️process/🧪️testing/🎚️budget/⏲️contention/🟦️.ts */
export function launchContentionRatioV1(): number {
  return sharedHostContentionV1()().ratio;
}

/** ⏱️ The level budget in milliseconds stretched by the contention `ratio`, clamped to the budget clock's policy bounds (an idle host keeps the plain budget). */
export function stretchedLevelBudgetMsV1(level: TestLevel, ratio: number): number {
  return Math.ceil(testLevelBudgetMs(level) * budgetStretchV1(ratio));
}

function levelsAbove(level: TestLevel): readonly TestLevel[] {'''),
 ('''/** 🐹️Cumulative `go test` args for the active level: per-level `-timeout`, keeps `-short` through `quick`, adds `-skip` for `Test<Level>`-prefixed tests above it. */
export function goLevelTestArgs(level: TestLevel = activeTestLevel()): string[] {
  const args: string[] = ["-timeout", `${testLevelBudgetSeconds(level)}s`];''', '''/** 🐹️Cumulative `go test` args for the active level: per-level `-timeout` (a whole-binary wall clock, so stretched by the host's contention at launch), keeps `-short` through `quick`, adds `-skip` for `Test<Level>`-prefixed tests above it. */
export function goLevelTestArgs(level: TestLevel = activeTestLevel(), ratio: number = launchContentionRatioV1()): string[] {
  const args: string[] = ["-timeout", `${Math.ceil(stretchedLevelBudgetMsV1(level, ratio) / 1000)}s`];'''),
 ('''export function vitestLevelArgs(level: TestLevel = activeTestLevel()): string[] {
  const ms = String(testLevelBudgetMs(level));''', '''export function vitestLevelArgs(level: TestLevel = activeTestLevel(), ratio: number = launchContentionRatioV1()): string[] {
  const ms = String(stretchedLevelBudgetMsV1(level, ratio));'''),
 ('''export function bunTestLevelArgs(level: TestLevel = activeTestLevel()): string[] {
  return ["--timeout", String(testLevelBudgetMs(level))];''', '''export function bunTestLevelArgs(level: TestLevel = activeTestLevel(), ratio: number = launchContentionRatioV1()): string[] {
  return ["--timeout", String(stretchedLevelBudgetMsV1(level, ratio))];'''),
 ('''export function pytestLevelArgs(level: TestLevel = activeTestLevel()): string[] {''', '''export function pytestLevelArgs(level: TestLevel = activeTestLevel(), ratio: number = launchContentionRatioV1()): string[] {'''),
 ('''  args.push(`--timeout=${testLevelBudgetSeconds(level)}`, "--timeout-method=thread");''', '''  args.push(`--timeout=${Math.ceil(stretchedLevelBudgetMsV1(level, ratio) / 1000)}`, "--timeout-method=thread");'''),
 ('''export function dotnetLevelArgs(level: TestLevel = activeTestLevel()): string[] {''', '''export function dotnetLevelArgs(level: TestLevel = activeTestLevel(), ratio: number = launchContentionRatioV1()): string[] {'''),
 ('''  args.push("--blame-hang", "--blame-hang-timeout", String(testLevelBudgetMs(level)));''', '''  args.push("--blame-hang", "--blame-hang-timeout", String(stretchedLevelBudgetMsV1(level, ratio)));'''),
 ('''export function playwrightTestTimeoutMs(level: TestLevel = activeTestLevel()): number {
  return testLevelBudgetMs(level);''', '''export function playwrightTestTimeoutMs(level: TestLevel = activeTestLevel(), ratio: number = launchContentionRatioV1()): number {
  return stretchedLevelBudgetMsV1(level, ratio);'''),
])
sub("f03.ts", [
 ("expect(goLevelTestArgs(level)).toEqual(row.go);expect(vitestLevelArgs(level)).toEqual(row.vitest);expect(bunTestLevelArgs(level)).toEqual(row.bun);expect(pytestLevelArgs(level)).toEqual(row.pytest);expect(dotnetLevelArgs(level)).toEqual(row.dotnet);", "expect(goLevelTestArgs(level,1)).toEqual(row.go);expect(vitestLevelArgs(level,1)).toEqual(row.vitest);expect(bunTestLevelArgs(level,1)).toEqual(row.bun);expect(pytestLevelArgs(level,1)).toEqual(row.pytest);expect(dotnetLevelArgs(level,1)).toEqual(row.dotnet);"),
 ("playwrightTestTimeoutMs(level)", "playwrightTestTimeoutMs(level,1)"),
 ('goLevelTestArgs("fundamental")', 'goLevelTestArgs("fundamental",1)'),
 ('pytestLevelArgs("fundamental")', 'pytestLevelArgs("fundamental",1)'),
 ('bunTestLevelArgs("fundamental")', 'bunTestLevelArgs("fundamental",1)'),
 ('playwrightTestTimeoutMs("fundamental")', 'playwrightTestTimeoutMs("fundamental",1)'),
])
p = os.path.join(HERE, "files", "f03.ts"); t = open(p, encoding="utf8").read()
assert t.count("bunTestLevelArgs()") == 3; open(p, "w", encoding="utf8").write(t.replace("bunTestLevelArgs()", "bunTestLevelArgs(undefined,1)"))
