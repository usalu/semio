import "../../🟦️.ts";
import Ajv from "ajv";
import { spawnSync } from "node:child_process";
import { runtimeCaptureSyncV1, runtimeCaptureV1, type RuntimeCaptureProgressV1, type RuntimeCaptureRequestV1 } from "../../🎣️capture/🟦️.ts";
import { describe, expect, test } from "../../🧪️test/🟦️.ts";
import corpus from "./🧫️cases.json" with { type: "json" };

interface Row { id: string; script: string; stdin?: string; maxOutputBytes: number; timeoutMilliseconds: number; cancelAfterMilliseconds?: number; abortAfterMilliseconds?: number; cancelledBeforeStart?: boolean; asynchronousOnly?: boolean; oracle?: boolean; expect: { outcome: string; exitCode?: number | null; stdout?: string; stderr?: string; progress?: string[]; stdoutAtMost?: number } }
const rows = corpus.cases as Row[];
const shape = { type: "object", additionalProperties: false, required: ["version", "cases", "refusals"], properties: { version: { const: 1 }, cases: { type: "array", minItems: 1 }, refusals: { type: "array", minItems: 1 } } };
const requestOf = (row: Row, extra: Partial<RuntimeCaptureRequestV1> = {}): RuntimeCaptureRequestV1 => ({ command: [process.execPath, "-e", row.script], stdin: row.stdin, maxOutputBytes: row.maxOutputBytes, timeoutMilliseconds: row.timeoutMilliseconds, ...extra });

function check(row: Row, result: Awaited<ReturnType<typeof runtimeCaptureV1>>, progress: RuntimeCaptureProgressV1[]): void {
  expect(result.outcome).toBe(row.expect.outcome);
  if (row.expect.exitCode !== undefined) expect(result.exitCode).toBe(row.expect.exitCode);
  if (row.expect.stdout !== undefined) expect(result.stdout).toBe(row.expect.stdout);
  if (row.expect.stderr !== undefined) expect(result.stderr).toBe(row.expect.stderr);
  if (row.expect.stdoutAtMost !== undefined) expect(result.stdout.length <= row.expect.stdoutAtMost).toBe(true);
  if (row.expect.progress !== undefined) expect(progress.map(event => event.line)).toEqual(row.expect.progress);
}

describe("runtime capture port", () => {
  test("the corpus has its closed shape", () => { expect(new Ajv({ strict: false }).validate(shape, corpus)).toBe(true); });
  for (const row of rows) {
    test(`asynchronous capture: ${row.id}`, async () => {
      const progress: RuntimeCaptureProgressV1[] = [], controller = new AbortController(), started = Date.now();
      const request = requestOf(row, { onProgress: event => { progress.push(event); }, ...(row.abortAfterMilliseconds !== undefined ? { signal: controller.signal } : {}), ...(row.cancelAfterMilliseconds !== undefined ? { cancelled: () => Date.now() - started >= row.cancelAfterMilliseconds! } : {}), ...(row.cancelledBeforeStart ? { cancelled: () => true } : {}) });
      if (row.abortAfterMilliseconds !== undefined) setTimeout(() => controller.abort(), row.abortAfterMilliseconds);
      const result = await runtimeCaptureV1(request);
      check(row, result, progress);
      if (row.expect.progress !== undefined) expect(progress.every(event => event.live)).toBe(true);
    });
    if (row.asynchronousOnly) continue;
    test(`synchronous capture: ${row.id}`, () => {
      const progress: RuntimeCaptureProgressV1[] = [];
      const result = runtimeCaptureSyncV1(requestOf(row, { onProgress: event => { progress.push(event); }, ...(row.cancelledBeforeStart ? { cancelled: () => true } : {}) }));
      check(row, result, progress);
      if (row.expect.progress !== undefined) expect(progress.every(event => !event.live)).toBe(true);
    });
    if (!row.oracle) continue;
    test(`oracle node:child_process agrees: ${row.id}`, async () => {
      const reference = spawnSync(process.execPath, ["-e", row.script], { input: row.stdin, encoding: "utf8" });
      const sync = runtimeCaptureSyncV1(requestOf(row)), asynchronous = await runtimeCaptureV1(requestOf(row));
      for (const result of [sync, asynchronous]) { expect(result.exitCode).toBe(reference.status); expect(result.stdout).toBe(reference.stdout); expect(result.stderr).toBe(reference.stderr); }
    });
  }
  for (const row of corpus.refusals) {
    test(`refuses an unbounded request: ${row.id}`, async () => {
      const request: RuntimeCaptureRequestV1 = { command: [process.execPath, "-e", "0"], maxOutputBytes: row.maxOutputBytes, timeoutMilliseconds: row.timeoutMilliseconds };
      expect(() => runtimeCaptureSyncV1(request)).toThrow();
      let refused = false; try { await runtimeCaptureV1(request); } catch { refused = true; }
      expect(refused).toBe(true);
    });
  }
});
