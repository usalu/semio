import { bindCatalogV1, type TextValuesV1 } from "@P/📦️packages/🟦️typescript/🏗️build/🗣️language/📖️catalog/🟦️.ts";
import { repoErrorV1, type RepoErrorCodeV1, type RepoErrorV1 } from "@P/📦️packages/🟦️typescript/🏗️build/🗣️language/🚨️error/🟦️.ts";
import catalog from "./🔣️.json" with { type: "json" };

/** 📖️ The texts of `repo config` in English and German. */
export const CONFIG_TEXTS_V1 = bindCatalogV1("config", catalog);

/** 🔑️ Every message key of this catalog. */
export type ConfigTextKeyV1 = keyof typeof catalog.messages;

/** 🚨️ Builds a refusal of this catalog as data; the CLI boundary renders it in the resolved language. */
export const refuseConfigV1 = (code: RepoErrorCodeV1, key: ConfigTextKeyV1, values: TextValuesV1 = {}, options?: ErrorOptions): RepoErrorV1 => repoErrorV1(CONFIG_TEXTS_V1, code, key, values, options);
