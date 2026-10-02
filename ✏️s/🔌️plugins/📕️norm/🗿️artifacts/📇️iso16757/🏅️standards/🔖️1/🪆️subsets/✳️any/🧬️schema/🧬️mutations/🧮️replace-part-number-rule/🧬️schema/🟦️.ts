/** 🧮️ `replace-part-number-rule` wire twin: the leaf payload `ReplacePartNumberRule`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { parsePartNumberRule, type PartNumberRule } from "../../../📸️snapshot/🟦️.ts";

export interface ReplacePartNumberRule {
  newRule: PartNumberRule;
}

export const parseReplacePartNumberRule: NormWireReader<ReplacePartNumberRule> = normWireObject<ReplacePartNumberRule>({ newRule: normWireRequired(parsePartNumberRule) });
