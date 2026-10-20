"""Authored E2E readiness edits: anchored (old -> new) replacements and new files, keyed by repo-relative path."""

L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
SESSION = f"{L}/⚡️caching/🌐️vite/🧾️session"
PLAY = "🏢️semio-tech/🎡️play/🔨️modules"
DEMO = "♻️mit-bestand/🧺️demonstrator/🔨️modules"

BUDGET_SCHEMA = '''    "ReadinessBudget": {
      "type": "object",
      "additionalProperties": false,
      "required": ["schemaVersion", "readyTimeoutMs", "progressIntervalMs", "evidence"],
      "properties": {
        "schemaVersion": { "const": 1 },
        "readyTimeoutMs": { "type": "integer", "minimum": 60000, "maximum": 3600000 },
        "progressIntervalMs": { "type": "integer", "minimum": 1000, "maximum": 60000 },
        "evidence": {
          "type": "object",
          "additionalProperties": false,
          "required": ["kind", "coldBootMs", "warmBootMs", "marginFactor", "source"],
          "properties": {
            "kind": { "enum": ["estimate", "measurement"] },
            "coldBootMs": { "$ref": "#/$defs/BootRange" },
            "warmBootMs": { "$ref": "#/$defs/BootRange" },
            "marginFactor": { "type": "number", "minimum": 2, "maximum": 20 },
            "source": { "type": "string", "minLength": 1, "maxLength": 512 }
          }
        }
      }
    },
    "BootRange": {
      "type": "object",
      "additionalProperties": false,
      "required": ["minimum", "maximum"],
      "properties": {
        "minimum": { "type": "integer", "minimum": 1, "maximum": 3600000 },
        "maximum": { "type": "integer", "minimum": 1, "maximum": 3600000 }
      }
    },
'''

SESSION_TYPES = '''/** ⏱️ A product's declared E2E service readiness: the timeout, the progress cadence and the boot evidence that timeout derives from. */
export type ServiceReadinessBudget = {
  readonly schemaVersion: 1;
  readonly readyTimeoutMs: number;
  readonly progressIntervalMs: number;
  readonly evidence: {
    readonly kind: "estimate" | "measurement";
    readonly coldBootMs: { readonly minimum: number; readonly maximum: number };
    readonly warmBootMs: { readonly minimum: number; readonly maximum: number };
    readonly marginFactor: number;
    readonly source: string;
  };
};
/** 📈️ One progress observation while a consumer waits for a service generation. */
export type ServiceReadyProgress = { readonly elapsedMs: number; readonly timeoutMs: number; readonly phase: "awaiting-ready-record" | "verifying-listener" };
/** 📣️ Progress cadence and sink of one readiness wait. */
export type ServiceReadyWait = { readonly intervalMs: number; readonly report: (progress: ServiceReadyProgress) => void };
const budgetSchema = schema.$defs.ReadinessBudget, bootRangeSchema = schema.$defs.BootRange;
const inRange = (value: unknown, bounds: { readonly minimum: number; readonly maximum: number }): value is number => Number.isSafeInteger(value) && Number(value) >= bounds.minimum && Number(value) <= bounds.maximum;
const bootRange = (value: unknown): value is { minimum: number; maximum: number } => keys(value, "maximum,minimum") && inRange(value.minimum, bootRangeSchema.properties.minimum) && inRange(value.maximum, bootRangeSchema.properties.maximum) && Number(value.minimum) <= Number(value.maximum);

/** 🧮️ Admits a declared readiness budget only when its timeout covers the cold-boot maximum times the declared margin. */
export function parseServiceReadinessBudgetV1(value: unknown): ServiceReadinessBudget {
  const reject = (reason: string): never => { throw new Error(`Invalid service readiness budget: ${reason}`); };
  if (!keys(value, "evidence,progressIntervalMs,readyTimeoutMs,schemaVersion") || value.schemaVersion !== 1) return reject("closed versioned record required");
  const evidence = value.evidence;
  if (!keys(evidence, "coldBootMs,kind,marginFactor,source,warmBootMs") || !budgetSchema.properties.evidence.properties.kind.enum.includes(evidence.kind as never)) return reject("closed evidence record required");
  if (!bootRange(evidence.coldBootMs) || !bootRange(evidence.warmBootMs) || evidence.warmBootMs.maximum > evidence.coldBootMs.maximum) return reject("ordered boot ranges required");
  const margin = budgetSchema.properties.evidence.properties.marginFactor;
  if (typeof evidence.marginFactor !== "number" || !(evidence.marginFactor >= margin.minimum && evidence.marginFactor <= margin.maximum)) return reject("margin factor out of range");
  if (typeof evidence.source !== "string" || !evidence.source.length || evidence.source.length > budgetSchema.properties.evidence.properties.source.maxLength) return reject("evidence source required");
  if (!inRange(value.readyTimeoutMs, budgetSchema.properties.readyTimeoutMs) || !inRange(value.progressIntervalMs, budgetSchema.properties.progressIntervalMs)) return reject("timeout or cadence out of range");
  if (Number(value.readyTimeoutMs) < Math.ceil(evidence.coldBootMs.maximum * evidence.marginFactor)) return reject("timeout below cold-boot maximum times margin");
  if (Number(value.progressIntervalMs) > Number(value.readyTimeoutMs)) return reject("cadence beyond timeout");
  return value as ServiceReadinessBudget;
}

const progressText = {
  en: { "awaiting-ready-record": "waiting for the dev server to publish its ready record", "verifying-listener": "verifying the listener identity", of: "of" },
  de: { "awaiting-ready-record": "warte darauf, dass der Dev-Server seinen Bereitschaftseintrag veröffentlicht", "verifying-listener": "prüfe die Identität des Listeners", of: "von" },
} as const;

/** 🗣️ Chooses the progress language like the CLI help: SEMIO_LANGUAGE, then the locale, then the first catalogue language. */
export function serviceReadyProgressLanguage(environment: Readonly<Record<string, string | undefined>>): "en" | "de" {
  for (const candidate of [environment.SEMIO_LANGUAGE, environment.LC_ALL, environment.LC_MESSAGES, environment.LANG]) {
    const language = candidate?.toLowerCase().slice(0, 2);
    if (language === "en" || language === "de") return language;
  }
  return "en";
}

/** 🔤️ Renders one readiness progress observation as a single log line. */
export function serviceReadyProgressLine(owner: string, progress: ServiceReadyProgress, language: "en" | "de"): string {
  const text = progressText[language], seconds = (milliseconds: number): number => Math.floor(milliseconds / 1000);
  return `[e2e] ${owner}: ${text[progress.phase]} (${seconds(progress.elapsedMs)} s ${text.of} ${seconds(progress.timeoutMs)} s)`;
}

'''

WAIT_OLD = '''/** ⏳️ Requires the current generation's HTTP identity before allowing its consumer to proceed. */
export async function waitForServiceReady(root: string, session: ServiceSession, signal: AbortSignal, timeoutMs = 60000): Promise<string> {
  signal.throwIfAborted();
  const path = join(serviceDirectory(root, session.pid), "ready.json"), deadline = performance.now() + timeoutMs;
  while (performance.now() < deadline) {
    signal.throwIfAborted();
'''

WAIT_NEW = SESSION_TYPES + '''/** ⏳️ Requires the current generation's HTTP identity before allowing its consumer to proceed within the caller's declared timeout, reporting progress on the declared cadence. */
export async function waitForServiceReady(root: string, session: ServiceSession, signal: AbortSignal, timeoutMs: number, wait?: ServiceReadyWait): Promise<string> {
  signal.throwIfAborted();
  if (!Number.isSafeInteger(timeoutMs) || timeoutMs < 1) throw new Error("Service readiness requires a declared positive timeout");
  const path = join(serviceDirectory(root, session.pid), "ready.json"), start = performance.now(), deadline = start + timeoutMs;
  let nextReport = start;
  while (performance.now() < deadline) {
    signal.throwIfAborted();
    if (wait && performance.now() >= nextReport) {
      wait.report({ elapsedMs: Math.round(performance.now() - start), timeoutMs, phase: existsSync(path) ? "verifying-listener" : "awaiting-ready-record" });
      nextReport = performance.now() + wait.intervalMs;
    }
'''

THROW_OLD = "  throw new Error(`Service did not become ready with the expected identity: ${session.owner}`);"
THROW_NEW = "  throw new Error(`Service did not become ready with the expected identity within ${timeoutMs} ms: ${session.owner}`);"

CASES_OLD = '''  "readinessRace": ["request", "replace-session", "respond", "reject-stale-session"]
}'''
CASES_NEW = '''  "readinessRace": ["request", "replace-session", "respond", "reject-stale-session"],
  "readiness": {
    "products": [
      { "budget": "🏢️semio-tech/🎡️play/🔨️modules/🧩️runtime/🧪️e2e/🔣️.json", "minimumTimeoutMs": 600000 },
      { "budget": "♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/🧪️e2e/🔣️.json", "minimumTimeoutMs": 600000 }
    ],
    "invalid": [
      { "reason": "timeout below cold-boot maximum times margin", "path": ["readyTimeoutMs"], "value": 100000, "schema": true },
      { "reason": "timeout below the schema minimum", "path": ["readyTimeoutMs"], "value": 1000, "schema": false },
      { "reason": "margin factor below two", "path": ["evidence", "marginFactor"], "value": 1, "schema": false },
      { "reason": "unknown evidence kind", "path": ["evidence", "kind"], "value": "guess", "schema": false },
      { "reason": "unknown member", "path": ["extra"], "value": 1, "schema": false },
      { "reason": "cold minimum above cold maximum", "path": ["evidence", "coldBootMs", "minimum"], "value": 200000, "schema": true },
      { "reason": "warm maximum above cold maximum", "path": ["evidence", "warmBootMs", "maximum"], "value": 190000, "schema": true },
      { "reason": "missing evidence source", "path": ["evidence", "source"], "remove": true, "schema": false },
      { "reason": "unknown schema version", "path": ["schemaVersion"], "value": 2, "schema": false }
    ],
    "progress": {
      "timeoutMs": 450,
      "intervalMs": 100,
      "minimumEvents": 3,
      "lines": [
        { "owner": "fixture:e2e", "progress": { "elapsedMs": 45000, "timeoutMs": 600000, "phase": "awaiting-ready-record" }, "en": "[e2e] fixture:e2e: waiting for the dev server to publish its ready record (45 s of 600 s)", "de": "[e2e] fixture:e2e: warte darauf, dass der Dev-Server seinen Bereitschaftseintrag veröffentlicht (45 s von 600 s)" },
        { "owner": "fixture:e2e", "progress": { "elapsedMs": 61999, "timeoutMs": 600000, "phase": "verifying-listener" }, "en": "[e2e] fixture:e2e: verifying the listener identity (61 s of 600 s)", "de": "[e2e] fixture:e2e: prüfe die Identität des Listeners (61 s von 600 s)" }
      ],
      "languages": [
        { "environment": { "SEMIO_LANGUAGE": "de", "LANG": "en_US.UTF-8" }, "language": "de" },
        { "environment": { "LANG": "de_DE.UTF-8" }, "language": "de" },
        { "environment": { "SEMIO_LANGUAGE": "fr", "LC_ALL": "de_CH" }, "language": "de" },
        { "environment": { "LANG": "C" }, "language": "en" },
        { "environment": {}, "language": "en" }
      ]
    }
  }
}'''

TEST_OLD = '''    await assert.rejects(() => api.openServiceSession(join(root, "hostile"), fixture.owner, fixture.pids[0]), /unowned/i);
  } finally { rmSync(root, { recursive: true, force: true }); }'''
TEST_NEW = '''    await assert.rejects(() => api.openServiceSession(join(root, "hostile"), fixture.owner, fixture.pids[0]), /unowned/i);
    const budgetSchema = { $ref: schema.$id + "#/$defs/ReadinessBudget" }, budgets: any[] = [];
    for (const row of fixture.readiness.products) {
      const budget = JSON.parse(readFileSync(join(workspace, row.budget), "utf8")); budgets.push(budget);
      assert.ok(ajv.validate(budgetSchema, budget), `${row.budget}: ${ajv.errorsText()}`);
      assert.deepEqual(api.parseServiceReadinessBudgetV1(budget), budget);
      assert.ok(budget.readyTimeoutMs >= budget.evidence.coldBootMs.maximum * budget.evidence.marginFactor, row.budget);
      assert.ok(budget.readyTimeoutMs >= row.minimumTimeoutMs, row.budget);
    }
    for (const row of fixture.readiness.invalid) {
      const value = structuredClone(budgets[0]); let target = value;
      for (const key of row.path.slice(0, -1)) target = target[key];
      if (row.remove) delete target[row.path.at(-1)]; else target[row.path.at(-1)] = row.value;
      assert.equal(ajv.validate(budgetSchema, value), row.schema, row.reason);
      assert.throws(() => api.parseServiceReadinessBudgetV1(value), /readiness budget/i, row.reason);
    }
    const waiting = join(root, "progress"), idle = await api.openServiceSession(waiting, fixture.owner, fixture.pids[0]), progress = fixture.readiness.progress, events: any[] = [];
    await assert.rejects(() => api.waitForServiceReady(waiting, idle, new AbortController().signal, progress.timeoutMs, { intervalMs: progress.intervalMs, report: (event: any) => events.push(event) }), new RegExp(`within ${progress.timeoutMs} ms`));
    assert.ok(events.length >= progress.minimumEvents, `progress events: ${events.length}`);
    assert.ok(events[0].elapsedMs < progress.intervalMs);
    assert.ok(events.every((event, index) => event.timeoutMs === progress.timeoutMs && event.phase === "awaiting-ready-record" && (index === 0 || event.elapsedMs >= events[index - 1].elapsedMs + progress.intervalMs - 5)));
    await assert.rejects(() => api.waitForServiceReady(waiting, idle, new AbortController().signal, 0), /declared positive timeout/i);
    for (const row of progress.lines) for (const language of ["en", "de"]) assert.equal(api.serviceReadyProgressLine(row.owner, row.progress, language), row[language]);
    for (const row of progress.languages) assert.equal(api.serviceReadyProgressLanguage(row.environment), row.language, JSON.stringify(row.environment));
  } finally { rmSync(root, { recursive: true, force: true }); }'''

BARREL_OLD = 'export { closeServiceSession, openServiceSession, publishServiceReady, readServiceSession, waitForServiceReady } from "../../../⚡️caching/🌐️vite/🧾️session/🟦️.ts";'
BARREL_NEW = '''export { closeServiceSession, openServiceSession, parseServiceReadinessBudgetV1, publishServiceReady, readServiceSession, serviceReadyProgressLanguage, serviceReadyProgressLine, waitForServiceReady } from "../../../⚡️caching/🌐️vite/🧾️session/🟦️.ts";
export type { ServiceReadinessBudget, ServiceReadyProgress, ServiceReadyWait } from "../../../⚡️caching/🌐️vite/🧾️session/🟦️.ts";'''


def budget_json(product_note):
    return f'''{{
  "schemaVersion": 1,
  "readyTimeoutMs": 600000,
  "progressIntervalMs": 15000,
  "evidence": {{
    "kind": "estimate",
    "coldBootMs": {{ "minimum": 60000, "maximum": 180000 }},
    "warmBootMs": {{ "minimum": 30000, "maximum": 60000 }},
    "marginFactor": 3,
    "source": "{product_note}"
  }}
}}
'''


PLAY_BUDGET = budget_json("audit-e2e-readiness-1020.md section 4 and R1: play serve boot (Vite, 20+ wasm plugins) cold 1-3 min, warm 30-60 s (estimate, not yet measured); timeout = 3 x the 180 s cold maximum, rounded up to the audit's 600 s floor")
DEMO_BUDGET = budget_json("audit-e2e-readiness-1020.md section 4 and R1: demonstrator serve boot cold 1-3 min, warm 30-60 s (estimate, not yet measured); timeout = 3 x the 180 s cold maximum, rounded up to the audit's 600 s floor")

PLAY_IMPORT_OLD = 'import { readServiceSession, waitForServiceReady } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🎮️playground/🟦️.ts";'
PLAY_IMPORT_NEW = '''import { parseServiceReadinessBudgetV1, readServiceSession, serviceReadyProgressLanguage, serviceReadyProgressLine, waitForServiceReady } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🎮️playground/🟦️.ts";
import readinessBudget from "../🧩️runtime/🧪️e2e/🔣️.json" with { type: "json" };'''
PLAY_WAIT_OLD = '      const baseURL = await waitForServiceReady(sessionRoot, session, controller.signal);'
PLAY_WAIT_NEW = '''      const budget = parseServiceReadinessBudgetV1(readinessBudget), language = serviceReadyProgressLanguage(process.env);
      const baseURL = await waitForServiceReady(sessionRoot, session, controller.signal, budget.readyTimeoutMs, { intervalMs: budget.progressIntervalMs, report: progress => console.log(serviceReadyProgressLine(PLAY_E2E_OWNER, progress, language)) });'''

DEMO_IMPORT_OLD = 'import { readServiceSession, waitForServiceReady } from "@semio-tech/repo-lib/dev";'
DEMO_IMPORT_NEW = '''import { parseServiceReadinessBudgetV1, readServiceSession, serviceReadyProgressLanguage, serviceReadyProgressLine, waitForServiceReady } from "@semio-tech/repo-lib/dev";
import readinessBudget from "../🧩️runtime/🧪️e2e/🔣️.json" with { type: "json" };'''
DEMO_WAIT_OLD = '    const baseURL = await waitForServiceReady(sessionRoot, session, controller.signal);'
DEMO_WAIT_NEW = '''    const budget = parseServiceReadinessBudgetV1(readinessBudget), language = serviceReadyProgressLanguage(process.env);
    const baseURL = await waitForServiceReady(sessionRoot, session, controller.signal, budget.readyTimeoutMs, { intervalMs: budget.progressIntervalMs, report: progress => console.log(serviceReadyProgressLine(DEMONSTRATOR_E2E_OWNER, progress, language)) });'''

SCHEMA_ANCHOR = '    "ReadinessRace": { "const": ["request", "replace-session", "respond", "reject-stale-session"] },\n'

EDITS = {
    f"{SESSION}/🧬️schema/🔣️.json": [(SCHEMA_ANCHOR, SCHEMA_ANCHOR + BUDGET_SCHEMA, 1)],
    f"{SESSION}/🟦️.ts": [(WAIT_OLD, WAIT_NEW, 1), (THROW_OLD, THROW_NEW, 1)],
    f"{SESSION}/🧫️cases.json": [(CASES_OLD, CASES_NEW, 1)],
    f"{L}/⚡️caching/🧪️tests/🌐️service-readiness/🟦️.ts": [(TEST_OLD, TEST_NEW, 1)],
    f"{L}/📦️packages/🟦️typescript/🎮️playground/🟦️.ts": [(BARREL_OLD, BARREL_NEW, 1)],
    f"{PLAY}/🧪️e2e/🟦️.ts": [(PLAY_IMPORT_OLD, PLAY_IMPORT_NEW, 1), (PLAY_WAIT_OLD, PLAY_WAIT_NEW, 1)],
    f"{DEMO}/🧪️e2e/🟦️.ts": [(DEMO_IMPORT_OLD, DEMO_IMPORT_NEW, 1), (DEMO_WAIT_OLD, DEMO_WAIT_NEW, 1)],
}

ROSTER_ANCHOR = '''      "reason": "bun repo tools setup bazel provisions Bazel before dependencies exist"
    },
'''
ROSTER_NEW = ROSTER_ANCHOR + f'''    {{
      "id": "nx-authorizer-runtime",
      "entry": "{L}/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/📦️runtime/🟦️.ts",
      "mode": "full",
      "reason": "bun repo tools setup nx captures this closure by source before any workspace link exists; a bare workspace specifier fails the capture"
    }},
    {{
      "id": "nx-authorizer-compiler",
      "entry": "{L}/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/📜️script.ts",
      "mode": "full",
      "reason": "bun repo tools setup nx captures and compiles this closure before any workspace link exists"
    }},
    {{
      "id": "cargo-compiler",
      "entry": "{L}/⚡️caching/🦀️cargo/📦️units/🏗️compiler/📥️entry/📜️script.ts",
      "mode": "full",
      "reason": "bun repo cargo captures this closure by source on a checkout whose workspace links are not installed yet"
    }},
'''
EDITS[f"{L}/🏗️build/📥️source/🛗️zero-touch/🗂️closures/🔣️.json"] = [(ROSTER_ANCHOR, ROSTER_NEW, 1)]

NEW_FILES = {
    f"{PLAY}/🧩️runtime/🧪️e2e/🔣️.json": PLAY_BUDGET,
    f"{DEMO}/🧩️runtime/🧪️e2e/🔣️.json": DEMO_BUDGET,
}

T0 = f"{L}/⚡️caching/🚀️bootstrap/🛠️tools"
FROZEN = {
    f"{T0}/package.json": [('    "typescript": "5.9.3",\n    "@semio-tech/framework-filesystem": "workspace:*"\n  },', '    "typescript": "5.9.3"\n  },', 1)],
    "bun.lock": [('        "@nxlv/python": "23.0.2",\n        "@semio-tech/framework-filesystem": "workspace:*",\n        "nx": "23.2.0",', '        "@nxlv/python": "23.0.2",\n        "nx": "23.2.0",', 1)],
}
