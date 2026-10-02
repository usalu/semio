/** 📏️ `change-elevation-regular` wire twin: the leaf payload `ChangeElevationRegular`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireBoolean, normWireInteger, normWireLiteral, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeElevationRegular {
  mutation: "changeElevationRegular";
  buildingIndex: number;
  newElevationRegular: boolean;
}

export const parseChangeElevationRegular: NormWireReader<ChangeElevationRegular> = normWireObject<ChangeElevationRegular>({ mutation: normWireRequired(normWireLiteral("changeElevationRegular")), buildingIndex: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newElevationRegular: normWireRequired(normWireBoolean) });
