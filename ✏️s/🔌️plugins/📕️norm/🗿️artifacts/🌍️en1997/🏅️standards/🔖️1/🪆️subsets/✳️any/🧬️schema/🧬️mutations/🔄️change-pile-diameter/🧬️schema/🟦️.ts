/** 🔄️ `change-pile-diameter` wire twin: the leaf payload `ChangePileDiameter`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePileDiameter {
  mutation: "changePileDiameter";
  id: string;
  newDiameter: number;
}

export const parseChangePileDiameter: NormWireReader<ChangePileDiameter> = normWireObject<ChangePileDiameter>({ mutation: normWireRequired(normWireLiteral("changePileDiameter")), id: normWireRequired(normWireString), newDiameter: normWireRequired(normWireNumber) });
