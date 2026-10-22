import { bindCatalogV1, type TextValuesV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/📖️catalog/🟦️.ts";
import { repoErrorV1, type RepoErrorCodeV1, type RepoErrorV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/🚨️error/🟦️.ts";
import { reportRepoErrorV1, sayV1, type ProgressLevelV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/📣️progress/🟦️.ts";
import catalog from "./🔣️.json" with { type: "json" };

/** 📖️ The texts of `repo commit` and `repo micro-commit`: the guidance, verdicts and refusals of the bundle workflow in English and German. */
export const COMMIT_TEXTS_V1 = bindCatalogV1("commit", catalog);

/** 🔑️ Every message key of this catalog. */
export type CommitTextKeyV1 = keyof typeof catalog.messages;

/** 📢️ Says one line of this catalog as data; the CLI boundary renders it in the resolved language. */
export const sayCommitV1 = (key: CommitTextKeyV1, values: TextValuesV1 = {}, level: ProgressLevelV1 = "info"): void => sayV1(COMMIT_TEXTS_V1, key, values, level);

/** 🚨️ Builds a refusal of this catalog as data; the CLI boundary renders it in the resolved language. */
export const refuseCommitV1 = (code: RepoErrorCodeV1, key: CommitTextKeyV1, values: TextValuesV1 = {}, options?: ErrorOptions): RepoErrorV1 => repoErrorV1(COMMIT_TEXTS_V1, code, key, values, options);

/** 🖨️ Prints a refusal caught inside the commit workflow: a `RepoErrorV1` is rendered in the resolved language, any other error by its message. */
export function reportCommitErrorV1(error: unknown): void {
  if (!reportRepoErrorV1(error)) console.error(error instanceof Error ? error.message : String(error));
}
