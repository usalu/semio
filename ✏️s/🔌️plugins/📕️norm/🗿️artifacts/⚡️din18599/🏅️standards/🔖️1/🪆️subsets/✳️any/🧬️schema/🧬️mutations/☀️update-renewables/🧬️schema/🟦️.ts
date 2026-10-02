/** ☀️ `update-renewables` wire twin: the leaf payload `UpdateRenewables`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { parseRenewables, type Renewables } from "../../../📸️snapshot/🟦️.ts";

export interface UpdateRenewables {
  mutation: "updateRenewables";
  newRenewables: Renewables;
}

export const parseUpdateRenewables: NormWireReader<UpdateRenewables> = normWireObject<UpdateRenewables>({ mutation: normWireRequired(normWireLiteral("updateRenewables")), newRenewables: normWireRequired(parseRenewables) });
