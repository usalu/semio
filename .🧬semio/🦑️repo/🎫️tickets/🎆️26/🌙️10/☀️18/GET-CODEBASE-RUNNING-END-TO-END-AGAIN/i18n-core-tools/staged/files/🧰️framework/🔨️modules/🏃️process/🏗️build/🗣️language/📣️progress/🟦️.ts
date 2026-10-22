import { AsyncLocalStorage } from "node:async_hooks";
import { CATALOG_LANGUAGES_V1, handleTextV1, qualifiedKeyV1, type CatalogHandleV1, type TextValuesV1 } from "../📖️catalog/🟦️.ts";
import { isRepoErrorV1, renderRepoErrorV1, repoErrorExitCodeV1 } from "../🚨️error/🟦️.ts";
import { resolveLanguageV1, type LanguageEnvironmentV1, type OsLanguageListV1 } from "../🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };

/** 🏷️ The stream of one progress event: `info` goes to the standard output, `warn` and `error` to the standard error. */
export type ProgressLevelV1 = "info" | "warn" | "error";

/** 🎚️ The closed set of progress levels. */
export const PROGRESS_LEVELS_V1: readonly string[] = Object.freeze(schema.properties.level.enum);

/**
 * 📣️ One line of progress as data.
 * @see ./🧬️schema/🔣️.json the schema this shape is written first in
 */
export type ProgressEventV1 = Readonly<{ version: 1; key: string; values: TextValuesV1; level: ProgressLevelV1 }>;

/** 📮️ Receives progress events and decides what to do with them; only the CLI boundary renders. */
export type ProgressSinkV1 = (event: ProgressEventV1) => void;

/** 🖋️ Where a rendered line is written. */
export type ProgressWriteV1 = (stream: "stdout" | "stderr", line: string) => void;

const scope = new AsyncLocalStorage<ProgressSinkV1>();

/** 🧱️ Builds one progress event of a catalog message and proves at the call site that both languages render with exactly these values. */
export function progressV1(handle: CatalogHandleV1, key: string, values: TextValuesV1 = {}, level: ProgressLevelV1 = "info"): ProgressEventV1 {
  for (const language of CATALOG_LANGUAGES_V1) handleTextV1(handle, key, language, values);
  return Object.freeze({ version: 1, key: `${handle.id}.${key}`, values: Object.freeze({ ...values }), level });
}

/** 🖨️ Renders one event in one language from the catalog its key names. */
export function renderProgressV1(event: ProgressEventV1, language: string): string {
  const { handle, key } = qualifiedKeyV1(event.key);
  return handleTextV1(handle, key, language, event.values);
}

/** 🖍️ Writes to the process streams; the default writer of the CLI boundary. */
export const processWriteV1: ProgressWriteV1 = (stream, line) => {
  if (stream === "stderr") console.error(line);
  else console.log(line);
};

/** 🛰️ The sink of the CLI boundary: it resolves the language when a line is rendered, through the one shared resolver, and refuses bilingually when none is chosen. */
export function createCliSinkV1(options: Readonly<{ environment?: LanguageEnvironmentV1; args?: readonly string[]; osLanguages?: OsLanguageListV1; write?: ProgressWriteV1 }> = {}): ProgressSinkV1 {
  const write = options.write ?? processWriteV1;
  return (event) => {
    const language = resolveLanguageV1(CATALOG_LANGUAGES_V1, options.args ?? [], options.environment ?? process.env, options.osLanguages).language;
    write(event.level === "info" ? "stdout" : "stderr", renderProgressV1(event, language));
  };
}

/** 🧵️ Runs `body` with a sink that receives every event emitted inside it, also across awaits. */
export function withProgressSinkV1<T>(sink: ProgressSinkV1, body: () => T): T {
  return scope.run(sink, body);
}

/** 📡️ Sends an event to the sink of the running scope or, outside any scope, to the sink of the CLI boundary. */
export function emitProgressV1(event: ProgressEventV1): void {
  (scope.getStore() ?? createCliSinkV1())(event);
}

/** 📢️ Builds and emits one progress event. */
export function sayV1(handle: CatalogHandleV1, key: string, values: TextValuesV1 = {}, level: ProgressLevelV1 = "info"): void {
  emitProgressV1(progressV1(handle, key, values, level));
}

/** 🚪️ Reports a refusal at the CLI boundary: a `RepoErrorV1` is rendered in the resolved language to the standard error and sets the exit status of its class; anything else is not handled and answers `false`. */
export function reportRepoErrorV1(error: unknown, options: Readonly<{ environment?: LanguageEnvironmentV1; args?: readonly string[]; osLanguages?: OsLanguageListV1; write?: ProgressWriteV1; setExitCode?: (code: number) => void }> = {}): boolean {
  if (!isRepoErrorV1(error)) return false;
  const language = resolveLanguageV1(CATALOG_LANGUAGES_V1, options.args ?? [], options.environment ?? process.env, options.osLanguages).language;
  (options.write ?? processWriteV1)("stderr", renderRepoErrorV1(error, language));
  (options.setExitCode ?? ((code) => { process.exitCode = code; }))(repoErrorExitCodeV1(error.code));
  return true;
}
