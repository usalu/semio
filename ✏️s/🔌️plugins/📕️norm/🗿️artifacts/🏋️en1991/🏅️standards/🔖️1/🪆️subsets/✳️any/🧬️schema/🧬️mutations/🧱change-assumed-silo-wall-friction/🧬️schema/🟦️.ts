/** 🧱 `change-assumed-silo-wall-friction` wire twin: the leaf payload `ChangeAssumedSiloWallFriction`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeAssumedSiloWallFriction {
  newAssumedSiloWallFriction: number;
}

export const parseChangeAssumedSiloWallFriction: NormWireReader<ChangeAssumedSiloWallFriction> = normWireObject<ChangeAssumedSiloWallFriction>({ newAssumedSiloWallFriction: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
