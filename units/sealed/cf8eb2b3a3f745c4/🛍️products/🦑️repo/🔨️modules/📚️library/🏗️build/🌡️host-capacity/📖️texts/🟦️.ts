import { bindCatalogV1, type TextValuesV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/📖️catalog/🟦️.ts";
import { repoErrorV1, type RepoErrorCodeV1, type RepoErrorV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/🚨️error/🟦️.ts";
import catalog from "./🔣️.json" with { type: "json" };

/** 📖️ The texts of the host capacity wait: the refusals of a cancelled wait in English and German. */
export const CAPACITY_TEXTS_V1 = bindCatalogV1("capacity", catalog);

/** 🔑️ Every message key of this catalog. */
export type CapacityTextKeyV1 = keyof typeof catalog.messages;

/** 🚨️ Builds a refusal of this catalog as data; the CLI boundary renders it in the resolved language. */
export const refuseCapacityV1 = (code: RepoErrorCodeV1, key: CapacityTextKeyV1, values: TextValuesV1 = {}, options?: ErrorOptions): RepoErrorV1 => repoErrorV1(CAPACITY_TEXTS_V1, code, key, values, options);
