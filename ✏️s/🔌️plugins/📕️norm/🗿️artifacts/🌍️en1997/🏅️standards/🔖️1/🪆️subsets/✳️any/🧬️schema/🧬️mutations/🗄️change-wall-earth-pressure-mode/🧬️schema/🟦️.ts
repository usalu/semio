/** 🗄️ `change-wall-earth-pressure-mode` wire twin: the leaf payload `ChangeWallEarthPressureMode`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type EarthPressureMode, parseEarthPressureMode } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeWallEarthPressureMode {
  mutation: "changeWallEarthPressureMode";
  id: string;
  newEarthPressureMode: EarthPressureMode;
}

export const parseChangeWallEarthPressureMode: NormWireReader<ChangeWallEarthPressureMode> = normWireObject<ChangeWallEarthPressureMode>({ mutation: normWireRequired(normWireLiteral("changeWallEarthPressureMode")), id: normWireRequired(normWireString), newEarthPressureMode: normWireRequired(parseEarthPressureMode) });
