/** 🔗 `change-connections` wire twin: the leaf payload `ChangeConnections`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireLiteral, normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type AluminiumConnection, parseAluminiumConnection } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeConnections {
  mutation: "changeConnections";
  connections: AluminiumConnection[];
}

export const parseChangeConnections: NormWireReader<ChangeConnections> = normWireObject<ChangeConnections>({ mutation: normWireRequired(normWireLiteral("changeConnections")), connections: normWireRequired(normWireArray(parseAluminiumConnection)) });
