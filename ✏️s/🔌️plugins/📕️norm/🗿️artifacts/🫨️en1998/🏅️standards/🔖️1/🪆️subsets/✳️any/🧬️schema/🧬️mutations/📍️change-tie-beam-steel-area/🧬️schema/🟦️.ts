/** 📍️ `change-tie-beam-steel-area` wire twin: the leaf payload `ChangeTieBeamSteelArea`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTieBeamSteelArea {
  mutation: "changeTieBeamSteelArea";
  id: string;
  newSteelArea: number;
}

export const parseChangeTieBeamSteelArea: NormWireReader<ChangeTieBeamSteelArea> = normWireObject<ChangeTieBeamSteelArea>({ mutation: normWireRequired(normWireLiteral("changeTieBeamSteelArea")), id: normWireRequired(normWireString), newSteelArea: normWireRequired(normWireNumber) });
