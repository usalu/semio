/** 🧪 Self-test of the workflow law: red on the file GitHub rejected (pushed 701-703), green on the live files, red on seeded defects (negative controls), every local action referenced exists. */
import { expect, test } from "bun:test"
import { existsSync, readFileSync } from "node:fs"
import { dirname, join } from "node:path"
import { checkRoot, checkWorkflow, duplicateKeys, parseExpression } from "./check.ts"

const root = (() => { for (let p = import.meta.dir; ; p = dirname(p)) { if (existsSync(join(p, ".github", "workflows", "build-system.yml"))) return p; if (dirname(p) === p) throw Error("repository root required") } })()
const live = readFileSync(join(root, ".github", "workflows", "build-system.yml"), "utf8")
const red = readFileSync(join(import.meta.dir, "..", "fixtures", "pushed-build-system.red.yml"), "utf8")
const messages = (text: string) => checkWorkflow("x", text, root).map((f) => `${f.where}: ${f.message}`)

test("the file GitHub rejected is red: the runner context in a job-level env", () => {
  const found = messages(red)
  expect(found.length).toBe(3)
  for (const m of found) expect(m).toMatch(/^jobs\.bench\.env\.BENCH_\w+: Context 'runner' is not available here/)
})

test("the live repository is green and every referenced local action exists", () => {
  expect(checkRoot(root)).toEqual([])
  const uses = [...live.matchAll(/uses: (\.\/\S+)/g)].map((m) => m[1]!)
  expect(uses.length).toBeGreaterThan(5)
  for (const u of uses) expect(existsSync(join(root, u, "action.yml")), u).toBe(true)
})

test("negative controls bite", () => {
  const seed = (from: string, to: string) => { expect(live.includes(from), from).toBe(true); return messages(live.replace(from, to)) }
  expect(seed("needs: [smoke]", "needs: [smoke, nowhere]").join()).toContain("needs unknown job 'nowhere'")
  expect(seed("runs-on: ubuntu-latest", "runs-on: ${{ env.NODE_VERSION }}").join()).toContain("Context 'env' is not available")
  expect(seed("uses: ./.github/actions/cache-proof", "uses: ./.github/actions/missing").join()).toContain("has no action.yml")
  expect(seed("timeout-minutes: 5", "timeout-minutes: 5\n    if: ${{ secrets.X == '1' }}").join()).toContain("Context 'secrets' is not available")
  expect(seed("matrix.os }}", "matrix.nope }}").join()).toContain("matrix.nope is not a matrix key")
  expect(seed("if: ${{ !cancelled() }}", "if: ${{ !cancelled( }}").join()).toContain("expression syntax")
  expect(duplicateKeys("a:\n  b: 1\n  b: 2\n")).toEqual(["line 3: duplicate key 'b'"])
  expect(duplicateKeys("- a: 1\n  b: 2\n- a: 3\n")).toEqual([])
  expect(() => parseExpression("a &&")).toThrow()
})
