/** 📏️ `change-elevation-regular` wire twin: the leaf payload `ChangeElevationRegular`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireBoolean, normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeElevationRegular {
  mutation: "changeElevationRegular";
  id: string;
  newElevationRegular: boolean;
}

export const parseChangeElevationRegular: NormWireReader<ChangeElevationRegular> = normWireObject<ChangeElevationRegular>({ mutation: normWireRequired(normWireLiteral("changeElevationRegular")), id: normWireRequired(normWireString), newElevationRegular: normWireRequired(normWireBoolean) });
