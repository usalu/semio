/** 🎛️ `change-product-configuration` wire twin: the leaf payload `ChangeProductConfiguration`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type Configuration, parseConfiguration } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeProductConfiguration {
  id: string;
  newConfiguration: Configuration;
}

export const parseChangeProductConfiguration: NormWireReader<ChangeProductConfiguration> = normWireObject<ChangeProductConfiguration>({ id: normWireRequired(normWireString), newConfiguration: normWireRequired(parseConfiguration) });
