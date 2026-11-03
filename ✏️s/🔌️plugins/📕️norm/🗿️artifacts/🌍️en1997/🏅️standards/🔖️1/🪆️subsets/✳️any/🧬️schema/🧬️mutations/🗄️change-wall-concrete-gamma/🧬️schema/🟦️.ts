/** 🗄️ `change-wall-concrete-gamma` wire twin: the leaf payload `ChangeWallConcreteGamma`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeWallConcreteGamma {
  mutation: "changeWallConcreteGamma";
  id: string;
  newConcreteGamma: number;
}

export const parseChangeWallConcreteGamma: NormWireReader<ChangeWallConcreteGamma> = normWireObject<ChangeWallConcreteGamma>({ mutation: normWireRequired(normWireLiteral("changeWallConcreteGamma")), id: normWireRequired(normWireString), newConcreteGamma: normWireRequired(normWireNumber) });
