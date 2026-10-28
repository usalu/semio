/** 🧪 Local law of GitHub Actions workflow validity: the checks GitHub makes when it accepts a workflow file (a failure there is a run that fails in 0 s with no jobs and shows the file path as its name). Usage: `bun check.ts <root> [--json]` checks every `.github/workflows/*.yml` and every local composite action under `<root>`. Exit 0 = no finding. See https://docs.github.com/en/actions/reference/workflows-and-actions/contexts#context-availability */
import { existsSync, readdirSync, readFileSync, statSync } from "node:fs"
import { join } from "node:path"

type Json = any
export type Finding = { file: string; where: string; message: string }

const STATUS = new Set(["success", "always", "cancelled", "failure"])
const PLAIN = new Set(["contains", "startsWith", "endsWith", "format", "join", "toJSON", "fromJSON"])
const CONTEXTS = new Set(["github", "needs", "strategy", "matrix", "job", "runner", "env", "vars", "secrets", "steps", "inputs"])
const JOB = ["github", "needs", "strategy", "matrix", "vars", "inputs"]
const STEP = [...JOB, "job", "runner", "env", "steps"]
const RUNNER = new Set(["name", "os", "arch", "temp", "tool_cache", "debug", "environment"])
const STRATEGY = new Set(["fail-fast", "job-index", "job-total", "max-parallel"])

type Spot = { contexts: string[]; status?: boolean; hash?: boolean }
const WORKFLOW_NAME: Spot = { contexts: ["github", "inputs", "vars"] }
const WORKFLOW_ENV: Spot = { contexts: ["github", "secrets", "inputs", "vars"] }
const WORKFLOW_DEFAULTS: Spot = { contexts: ["github", "inputs", "vars", "env"] }
const JOB_PLAIN: Spot = { contexts: JOB }
const JOB_IF: Spot = { contexts: ["github", "needs", "vars", "inputs"], status: true }
const JOB_STRATEGY: Spot = { contexts: ["github", "needs", "vars", "inputs"] }
const JOB_ENV: Spot = { contexts: [...JOB, "secrets"] }
const JOB_DEFAULTS: Spot = { contexts: [...JOB, "env"] }
const JOB_OUTPUT: Spot = { contexts: [...STEP, "secrets"], hash: true }
const STEP_IF: Spot = { contexts: STEP, status: true, hash: true }
const STEP_ANY: Spot = { contexts: [...STEP, "secrets"], hash: true }

type Tok = { k: string; v?: string }

export function tokenize(src: string): Tok[] {
  const out: Tok[] = []
  let i = 0
  while (i < src.length) {
    const c = src[i]
    if (/\s/.test(c)) { i++; continue }
    if (c === "'") {
      let j = i + 1, s = ""
      for (;;) {
        if (j >= src.length) throw new Error("unterminated string literal")
        if (src[j] === "'") { if (src[j + 1] === "'") { s += "'"; j += 2; continue } break }
        s += src[j++]
      }
      out.push({ k: "str", v: s }); i = j + 1; continue
    }
    const num = /^-?(0x[0-9a-fA-F]+|\d+(\.\d+)?([eE][+-]?\d+)?)/.exec(src.slice(i))
    if (num && !(out.length && ["id", "num", "str", ")", "]"].includes(out[out.length - 1].k) && num[0][0] === "-")) { out.push({ k: "num", v: num[0] }); i += num[0].length; continue }
    const id = /^[A-Za-z_][A-Za-z0-9_-]*/.exec(src.slice(i))
    if (id) { out.push({ k: "id", v: id[0] }); i += id[0].length; continue }
    const two = src.slice(i, i + 2)
    if (["<=", ">=", "==", "!=", "&&", "||"].includes(two)) { out.push({ k: two }); i += 2; continue }
    if ("()[].,!<>*".includes(c)) { out.push({ k: c }); i++; continue }
    throw new Error(`unexpected character '${c}' at position ${i + 1}`)
  }
  return out
}

type Node = { t: "lit" } | { t: "ctx"; name: string; path: (string | null)[]; at: number } | { t: "call"; name: string; args: Node[] } | { t: "op"; args: Node[] }

export function parseExpression(src: string): Node {
  const toks = tokenize(src)
  let p = 0
  const peek = () => toks[p]?.k
  const eat = (k: string) => { if (peek() !== k) throw new Error(`expected '${k}' but found '${peek() ?? "end"}'`); return toks[p++] }
  const primary = (): Node => {
    const t = toks[p]
    if (!t) throw new Error("unexpected end of expression")
    if (t.k === "(") { p++; const e = or(); eat(")"); return e }
    if (t.k === "str" || t.k === "num") { p++; return { t: "lit" } }
    if (t.k === "id") {
      p++
      if (["true", "false", "null", "NaN", "Infinity"].includes(t.v!)) return { t: "lit" }
      if (peek() === "(") {
        p++
        const args: Node[] = []
        if (peek() !== ")") { for (;;) { args.push(or()); if (peek() === ",") { p++; continue } break } }
        eat(")")
        return { t: "call", name: t.v!, args }
      }
      const path: (string | null)[] = []
      for (;;) {
        if (peek() === ".") {
          p++
          const n = toks[p]
          if (n?.k === "id") { path.push(n.v!); p++ } else if (n?.k === "*") { path.push("*"); p++ } else throw new Error("expected a property name after '.'")
        } else if (peek() === "[") {
          p++
          const n = toks[p]
          if (n?.k === "str" && toks[p + 1]?.k === "]") { path.push(n.v!); p++ } else { or(); path.push(null) }
          eat("]")
        } else break
      }
      return { t: "ctx", name: t.v!, path, at: p }
    }
    throw new Error(`unexpected '${t.k}'`)
  }
  const unary = (): Node => { if (peek() === "!") { p++; return { t: "op", args: [unary()] } } return primary() }
  const bin = (next: () => Node, ops: string[]) => () => { let l = next(); while (ops.includes(peek()!)) { p++; l = { t: "op", args: [l, next()] } } return l }
  const cmp = bin(unary, ["<", "<=", ">", ">="])
  const eq = bin(cmp, ["==", "!="])
  const and = bin(eq, ["&&"])
  const or = bin(and, ["||"])
  const e = or()
  if (p < toks.length) throw new Error(`unexpected '${toks[p].k}'`)
  return e
}

export function expressionsOf(value: string, isIf: boolean): { exprs: string[]; mixed: boolean } {
  const exprs: string[] = []
  let mixed = false
  const rx = /\$\{\{([\s\S]*?)\}\}/g
  let m: RegExpExecArray | null
  let last = 0
  while ((m = rx.exec(value))) { exprs.push(m[1]); last = rx.lastIndex }
  if (isIf && !exprs.length) return { exprs: [value], mixed: false }
  if (isIf && exprs.length) mixed = value.trim() !== value.slice(value.indexOf("${{"), last).trim()
  return { exprs, mixed }
}

type Ctx = { file: string; add: (where: string, message: string) => void; jobNeeds?: Set<string>; stepIds?: Set<string>; matrixKeys?: Set<string> | null; inputKeys?: Set<string>; outputs?: Record<string, Set<string>>; jobOutputs?: Record<string, Set<string>> }

function checkValue(ctx: Ctx, where: string, value: Json, spot: Spot, isIf = false) {
  if (typeof value !== "string") return
  const { exprs, mixed } = expressionsOf(value, isIf)
  if (mixed) ctx.add(where, `condition mixes \${{ }} with other text, GitHub evaluates the whole string as a truthy string: ${JSON.stringify(value)}`)
  for (const src of exprs) {
    let node: Node
    try { node = parseExpression(src) } catch (e) { ctx.add(where, `expression syntax: ${(e as Error).message} in \${{${src}}}`); continue }
    const walk = (n: Node) => {
      if (n.t === "call") {
        if (STATUS.has(n.name)) { if (!spot.status) ctx.add(where, `status function ${n.name}() is not available here`) }
        else if (n.name === "hashFiles") { if (!spot.hash) ctx.add(where, "hashFiles() is not available here") }
        else if (!PLAIN.has(n.name)) ctx.add(where, `unrecognized function: ${n.name}()`)
        n.args.forEach(walk)
      } else if (n.t === "op") n.args.forEach(walk)
      else if (n.t === "ctx") {
        if (!CONTEXTS.has(n.name)) { ctx.add(where, `Unrecognized named-value: '${n.name}' in \${{${src}}}`); return }
        if (!spot.contexts.includes(n.name)) { ctx.add(where, `Context '${n.name}' is not available here (allowed: ${spot.contexts.join(", ")}) in \${{${src}}}`); return }
        const [a, b, c] = n.path
        if (n.name === "runner" && typeof a === "string" && !RUNNER.has(a)) ctx.add(where, `unknown runner property '${a}'`)
        if (n.name === "strategy" && typeof a === "string" && !STRATEGY.has(a)) ctx.add(where, `unknown strategy property '${a}'`)
        if (n.name === "steps" && typeof a === "string" && a !== "*" && ctx.stepIds && !ctx.stepIds.has(a)) ctx.add(where, `steps.${a} refers to a step id that does not exist in this job`)
        if (n.name === "needs" && typeof a === "string" && ctx.jobNeeds && !ctx.jobNeeds.has(a)) ctx.add(where, `needs.${a} refers to a job that is not in needs`)
        if (n.name === "needs" && typeof a === "string" && b === "outputs" && typeof c === "string" && ctx.jobOutputs?.[a] && !ctx.jobOutputs[a].has(c)) ctx.add(where, `needs.${a}.outputs.${c} is not an output of job ${a}`)
        if (n.name === "matrix" && typeof a === "string" && ctx.matrixKeys && !ctx.matrixKeys.has(a)) ctx.add(where, `matrix.${a} is not a matrix key`)
        if (n.name === "inputs" && typeof a === "string" && ctx.inputKeys && !ctx.inputKeys.has(a)) ctx.add(where, `inputs.${a} is not a declared input`)
      }
    }
    walk(node)
  }
}

export function duplicateKeys(text: string): string[] {
  const found: string[] = []
  const stack: { indent: number; keys: Set<string> }[] = []
  let block: number | null = null
  const lines = text.split("\n")
  lines.forEach((raw, i) => {
    const line = raw.replace(/\r$/, "")
    if (!line.trim() || line.trim().startsWith("#")) return
    const indent = line.length - line.trimStart().length
    if (block !== null) { if (indent > block) return; block = null }
    let rest = line.trimStart(), at = indent
    while (rest.startsWith("- ")) {
      while (stack.length && stack[stack.length - 1].indent >= at) stack.pop()
      at += 2; rest = rest.slice(2).trimStart()
      while (rest.length < line.length && at < line.length - rest.length) at = line.length - rest.length
    }
    const m = /^("[^"]*"|'[^']*'|[^\s#:{}\[\],&*!|>'"%@`][^:#]*?)\s*:(\s|$)/.exec(rest)
    if (!m) return
    while (stack.length && stack[stack.length - 1].indent > at) stack.pop()
    if (!stack.length || stack[stack.length - 1].indent < at) stack.push({ indent: at, keys: new Set() })
    const top = stack[stack.length - 1]
    const key = m[1].replace(/^["']|["']$/g, "")
    if (top.keys.has(key)) found.push(`line ${i + 1}: duplicate key '${key}'`)
    top.keys.add(key)
    const value = rest.slice(m[0].length).trim()
    if (/^[|>][+-]?\d*$/.test(value)) block = indent
  })
  return found
}

function isLocal(use: string) { return use.startsWith("./") }

function checkUses(ctx: Ctx, where: string, use: unknown, root: string, job: boolean) {
  if (typeof use !== "string") { ctx.add(where, "uses must be a string"); return }
  if (use.startsWith("docker://")) return
  if (isLocal(use)) {
    const dir = join(root, use)
    if (job) { if (!existsSync(dir) || !statSync(dir).isFile()) ctx.add(where, `local reusable workflow ${use} does not exist`) }
    else if (!existsSync(join(dir, "action.yml")) && !existsSync(join(dir, "action.yaml"))) ctx.add(where, `local action ${use} has no action.yml`)
    return
  }
  const m = /^([\w.-]+)\/([\w.-]+)(\/[^@]+)?@(\S+)$/.exec(use)
  if (!m) ctx.add(where, `uses '${use}' is neither ./path, docker:// nor owner/repo[/path]@ref`)
}

function checkSteps(ctx: Ctx, where: string, steps: Json, root: string, composite: boolean) {
  if (!Array.isArray(steps) || !steps.length) { ctx.add(where, "steps must be a non-empty list"); return }
  const ids = new Set<string>()
  steps.forEach((s: Json, i: number) => {
    if (s?.id !== undefined) { if (ids.has(s.id)) ctx.add(`${where}[${i}]`, `duplicate step id '${s.id}'`); ids.add(s.id) }
  })
  ctx.stepIds = ids
  steps.forEach((s: Json, i: number) => {
    const w = `${where}[${i}]${s?.name ? ` (${String(s.name).slice(0, 60)})` : ""}`
    if (!s || typeof s !== "object") { ctx.add(w, "step must be a mapping"); return }
    if ((s.uses === undefined) === (s.run === undefined)) ctx.add(w, "a step needs exactly one of uses and run")
    if (s.shell !== undefined && s.run === undefined) ctx.add(w, "shell is only valid with run")
    if (composite && s.run !== undefined && s.shell === undefined) ctx.add(w, "a run step of a composite action needs shell")
    if (s.with !== undefined && s.uses === undefined) ctx.add(w, "with is only valid with uses")
    if (s.uses !== undefined) checkUses(ctx, w, s.uses, root, false)
    checkValue(ctx, `${w}.if`, s.if === undefined ? undefined : String(s.if), STEP_IF, true)
    for (const k of ["name", "run", "working-directory", "shell"]) checkValue(ctx, `${w}.${k}`, s[k], STEP_ANY)
    for (const k of ["env", "with"]) for (const [n, v] of Object.entries(s[k] ?? {})) checkValue(ctx, `${w}.${k}.${n}`, v, STEP_ANY)
    if (typeof s["continue-on-error"] === "string") checkValue(ctx, `${w}.continue-on-error`, s["continue-on-error"], STEP_ANY)
    if (typeof s["timeout-minutes"] === "string") checkValue(ctx, `${w}.timeout-minutes`, s["timeout-minutes"], STEP_ANY)
  })
}

export function checkWorkflow(file: string, text: string, root: string): Finding[] {
  const findings: Finding[] = []
  const add = (where: string, message: string) => findings.push({ file, where, message })
  for (const d of duplicateKeys(text)) add("yaml", d)
  let doc: Json
  try { doc = Bun.YAML.parse(text) } catch (e) { add("yaml", `parse error: ${(e as Error).message}`); return findings }
  if (!doc || typeof doc !== "object") { add("yaml", "document is not a mapping"); return findings }
  const on = doc.on ?? doc[true as unknown as string]
  if (on === undefined) add("on", "missing the on key")
  const dispatch = on && typeof on === "object" && !Array.isArray(on) ? on.workflow_dispatch : undefined
  const call = on && typeof on === "object" && !Array.isArray(on) ? on.workflow_call : undefined
  const inputKeys = new Set<string>([...Object.keys(dispatch?.inputs ?? {}), ...Object.keys(call?.inputs ?? {})])
  const base: Ctx = { file, add, inputKeys }
  checkValue(base, "name", doc.name, WORKFLOW_NAME)
  checkValue(base, "run-name", doc["run-name"], WORKFLOW_NAME)
  const conc = doc.concurrency
  if (typeof conc === "string") checkValue(base, "concurrency", conc, WORKFLOW_NAME)
  else for (const [k, v] of Object.entries(conc ?? {})) checkValue(base, `concurrency.${k}`, v, WORKFLOW_NAME)
  for (const [k, v] of Object.entries(doc.env ?? {})) checkValue(base, `env.${k}`, v, WORKFLOW_ENV)
  for (const [k, v] of Object.entries(doc.defaults?.run ?? {})) checkValue(base, `defaults.run.${k}`, v, WORKFLOW_DEFAULTS)
  const jobs = doc.jobs
  if (!jobs || typeof jobs !== "object" || !Object.keys(jobs).length) { add("jobs", "a workflow needs at least one job"); return findings }
  const jobOutputs: Record<string, Set<string>> = {}
  for (const [id, j] of Object.entries<Json>(jobs)) jobOutputs[id] = new Set(Object.keys(j?.outputs ?? {}))
  const edges: Record<string, string[]> = {}
  for (const [id, j] of Object.entries<Json>(jobs)) {
    const w = `jobs.${id}`
    if (!/^[A-Za-z_][A-Za-z0-9_-]*$/.test(id)) add(w, "job id may only contain letters, digits, - and _ and must not start with a digit")
    if (!j || typeof j !== "object") { add(w, "job must be a mapping"); continue }
    const needs: string[] = j.needs === undefined ? [] : Array.isArray(j.needs) ? j.needs : [j.needs]
    edges[id] = needs
    for (const n of needs) if (!(n in jobs)) add(`${w}.needs`, `needs unknown job '${n}'`)
    if (j.uses !== undefined) checkUses(base, `${w}.uses`, j.uses, root, true)
    else if (j["runs-on"] === undefined) add(w, "job needs runs-on (or uses)")
    let matrixKeys: Set<string> | null = null
    const matrix = j.strategy?.matrix
    if (matrix && typeof matrix === "object") {
      matrixKeys = new Set<string>()
      for (const [k, v] of Object.entries<Json>(matrix)) { if (k !== "include" && k !== "exclude") matrixKeys.add(k); else if (k === "include" && Array.isArray(v)) v.forEach((e: Json) => Object.keys(e ?? {}).forEach((x) => matrixKeys!.add(x))) }
    } else if (typeof matrix === "string") matrixKeys = null
    else if (j.strategy === undefined) matrixKeys = new Set()
    const ctx: Ctx = { file, add, jobNeeds: new Set(needs), matrixKeys, inputKeys, jobOutputs }
    checkValue(ctx, `${w}.name`, j.name, JOB_PLAIN)
    checkValue(ctx, `${w}.runs-on`, typeof j["runs-on"] === "string" ? j["runs-on"] : undefined, JOB_PLAIN)
    for (const k of ["timeout-minutes", "continue-on-error"]) checkValue(ctx, `${w}.${k}`, typeof j[k] === "string" ? j[k] : undefined, JOB_PLAIN)
    checkValue(ctx, `${w}.if`, j.if === undefined ? undefined : String(j.if), JOB_IF, true)
    if (j.strategy) for (const [k, v] of Object.entries(j.strategy)) if (k !== "matrix") checkValue({ ...ctx, matrixKeys: null }, `${w}.strategy.${k}`, v, JOB_STRATEGY)
    if (typeof matrix === "string") checkValue({ ...ctx, matrixKeys: null }, `${w}.strategy.matrix`, matrix, JOB_STRATEGY)
    const conc = j.concurrency
    if (typeof conc === "string") checkValue(ctx, `${w}.concurrency`, conc, JOB_PLAIN)
    else for (const [k, v] of Object.entries(conc ?? {})) checkValue(ctx, `${w}.concurrency.${k}`, v, JOB_PLAIN)
    for (const [k, v] of Object.entries(j.env ?? {})) checkValue(ctx, `${w}.env.${k}`, v, JOB_ENV)
    for (const [k, v] of Object.entries(j.defaults?.run ?? {})) checkValue(ctx, `${w}.defaults.run.${k}`, v, JOB_DEFAULTS)
    for (const [k, v] of Object.entries(j.with ?? {})) checkValue(ctx, `${w}.with.${k}`, v, JOB_PLAIN)
    if (j.uses === undefined) {
      checkSteps(ctx, `${w}.steps`, j.steps, root, false)
      for (const [k, v] of Object.entries(j.outputs ?? {})) checkValue(ctx, `${w}.outputs.${k}`, v, JOB_OUTPUT)
    }
  }
  const state: Record<string, number> = {}
  const visit = (id: string, trail: string[]) => {
    if (state[id] === 2) return
    if (state[id] === 1) { add("jobs", `needs cycle: ${[...trail.slice(trail.indexOf(id)), id].join(" -> ")}`); return }
    state[id] = 1
    for (const n of edges[id] ?? []) if (n in edges) visit(n, [...trail, id])
    state[id] = 2
  }
  for (const id of Object.keys(edges)) visit(id, [])
  return findings
}

export function checkAction(file: string, text: string, root: string): Finding[] {
  const findings: Finding[] = []
  const add = (where: string, message: string) => findings.push({ file, where, message })
  for (const d of duplicateKeys(text)) add("yaml", d)
  let doc: Json
  try { doc = Bun.YAML.parse(text) } catch (e) { add("yaml", `parse error: ${(e as Error).message}`); return findings }
  if (!doc?.name) add("name", "an action needs a name")
  if (!doc?.description) add("description", "an action needs a description")
  if (!doc?.runs?.using) { add("runs.using", "missing"); return findings }
  if (doc.runs.using !== "composite") return findings
  const inputKeys = new Set<string>(Object.keys(doc.inputs ?? {}))
  const ctx: Ctx = { file, add, inputKeys, matrixKeys: null }
  checkSteps(ctx, "runs.steps", doc.runs.steps, root, true)
  for (const [k, v] of Object.entries<Json>(doc.outputs ?? {})) checkValue({ ...ctx, stepIds: new Set((doc.runs.steps ?? []).map((s: Json) => s.id).filter(Boolean)) }, `outputs.${k}.value`, v?.value, JOB_OUTPUT)
  return findings
}

export function checkRoot(root: string): Finding[] {
  const findings: Finding[] = []
  const wf = join(root, ".github", "workflows")
  if (existsSync(wf)) for (const f of readdirSync(wf).sort()) if (/\.ya?ml$/.test(f)) findings.push(...checkWorkflow(`.github/workflows/${f}`, readFileSync(join(wf, f), "utf8"), root))
  const ac = join(root, ".github", "actions")
  if (existsSync(ac)) for (const d of readdirSync(ac).sort()) for (const n of ["action.yml", "action.yaml"]) {
    const p = join(ac, d, n)
    if (existsSync(p)) findings.push(...checkAction(`.github/actions/${d}/${n}`, readFileSync(p, "utf8"), root))
  }
  return findings
}

if (import.meta.main) {
  const [root, flag] = process.argv.slice(2)
  if (!root) { console.error("usage: bun check.ts <root> [--json]"); process.exit(2) }
  const findings = checkRoot(root)
  if (flag === "--json") console.log(JSON.stringify(findings, null, 1))
  else for (const f of findings) console.log(`${f.file}: ${f.where}: ${f.message}`)
  console.log(`${findings.length} finding(s)`)
  process.exit(findings.length ? 1 : 0)
}
