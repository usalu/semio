import { CATALOG_LANGUAGES_V1, handleTextV1, qualifiedKeyV1, type CatalogHandleV1, type TextValuesV1 } from "../📖️catalog/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };

/** 🔢️ The closed set of refusal classes; the class, not the text, decides the exit status. */
export const REPO_ERROR_CODES_V1: readonly string[] = Object.freeze(schema.properties.code.enum);

/** 🏷️ One refusal class. */
export type RepoErrorCodeV1 = "usage" | "invalid" | "missing" | "refused" | "failed" | "breach" | "stale" | "unsupported";

/**
 * 📦️ The wire form of a refusal.
 * @see ./🧬️schema/🔣️.json the schema this shape is written first in
 */
export type RepoErrorDataV1 = Readonly<{ version: 1; code: RepoErrorCodeV1; key: string; values: TextValuesV1 }>;

/** 🚨️ A refusal that carries `{ code, key, values }` and renders nowhere but at the CLI boundary; `message` is a machine line, never English prose. */
export class RepoErrorV1 extends Error {
  readonly code: RepoErrorCodeV1;
  readonly key: string;
  readonly values: TextValuesV1;
  constructor(code: RepoErrorCodeV1, key: string, values: TextValuesV1 = {}, options?: ErrorOptions) {
    super(`${code} ${key} ${JSON.stringify(values)}`, options);
    this.name = "RepoError";
    this.code = code;
    this.key = key;
    this.values = Object.freeze({ ...values });
  }
}

/** 🏗️ Builds a refusal of one catalog message and proves at the throw site that both languages render with exactly these values. */
export function repoErrorV1(handle: CatalogHandleV1, code: RepoErrorCodeV1, key: string, values: TextValuesV1 = {}, options?: ErrorOptions): RepoErrorV1 {
  for (const language of CATALOG_LANGUAGES_V1) handleTextV1(handle, key, language, values);
  return new RepoErrorV1(code, `${handle.id}.${key}`, values, options);
}

/** 🔎️ Recognizes a refusal even when it was built by another instance of this module. */
export function isRepoErrorV1(value: unknown): value is RepoErrorV1 {
  const row = value as Partial<RepoErrorV1> | null;
  return typeof value === "object" && row !== null && (value instanceof RepoErrorV1 || (row.name === "RepoError" && typeof row.code === "string" && typeof row.key === "string" && typeof row.values === "object"));
}

/** 📥️ Reads the wire form of a refusal and admits only a schema-valid one. */
export function readRepoErrorDataV1(value: unknown): RepoErrorDataV1 {
  const row = value as Record<string, unknown> | null;
  const pattern = new RegExp(schema.properties.key.pattern, "u");
  if (typeof row !== "object" || row === null || Array.isArray(row)) throw new TypeError("repo-error:shape");
  const unknownName = Object.keys(row).find((name) => !["version", "code", "key", "values"].includes(name));
  if (unknownName !== undefined || row.version !== 1 || !REPO_ERROR_CODES_V1.includes(row.code as string) || typeof row.key !== "string" || !pattern.test(row.key)) throw new TypeError("repo-error:shape");
  const values = row.values as Record<string, unknown> | null;
  if (typeof values !== "object" || values === null || Array.isArray(values) || Object.values(values).some((entry) => typeof entry !== "string" && typeof entry !== "number")) throw new TypeError("repo-error:values");
  return Object.freeze({ version: 1, code: row.code as RepoErrorCodeV1, key: row.key, values: Object.freeze({ ...(values as TextValuesV1) }) });
}

/** 📤️ The wire form of a refusal. */
export function repoErrorDataV1(error: RepoErrorV1): RepoErrorDataV1 {
  return readRepoErrorDataV1({ version: 1, code: error.code, key: error.key, values: error.values });
}

/** 🖨️ Renders a refusal in one language from the catalog its key names; this is the CLI boundary's job and nobody else's. */
export function renderRepoErrorV1(error: Pick<RepoErrorV1, "key" | "values">, language: string): string {
  const { handle, key } = qualifiedKeyV1(error.key);
  return handleTextV1(handle, key, language, error.values);
}

/** 🚪️ The exit status of a refusal class: usage mistakes are 2, everything else 1. */
export function repoErrorExitCodeV1(code: RepoErrorCodeV1): number {
  return code === "usage" ? 2 : 1;
}
