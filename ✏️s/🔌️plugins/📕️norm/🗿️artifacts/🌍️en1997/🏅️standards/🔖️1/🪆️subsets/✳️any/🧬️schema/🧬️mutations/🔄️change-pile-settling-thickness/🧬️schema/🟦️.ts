/** 🔄️ `change-pile-settling-thickness` wire twin: the leaf payload `ChangePileSettlingThickness`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePileSettlingThickness {
  mutation: "changePileSettlingThickness";
  id: string;
  newSettlingThickness: number;
}

export const parseChangePileSettlingThickness: NormWireReader<ChangePileSettlingThickness> = normWireObject<ChangePileSettlingThickness>({ mutation: normWireRequired(normWireLiteral("changePileSettlingThickness")), id: normWireRequired(normWireString), newSettlingThickness: normWireRequired(normWireNumber) });
