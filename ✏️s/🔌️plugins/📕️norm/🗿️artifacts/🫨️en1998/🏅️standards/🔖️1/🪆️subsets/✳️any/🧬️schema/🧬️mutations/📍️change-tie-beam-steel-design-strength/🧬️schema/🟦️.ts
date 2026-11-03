/** 📍️ `change-tie-beam-steel-design-strength` wire twin: the leaf payload `ChangeTieBeamSteelDesignStrength`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTieBeamSteelDesignStrength {
  mutation: "changeTieBeamSteelDesignStrength";
  id: string;
  newSteelDesignStrength: number;
}

export const parseChangeTieBeamSteelDesignStrength: NormWireReader<ChangeTieBeamSteelDesignStrength> = normWireObject<ChangeTieBeamSteelDesignStrength>({ mutation: normWireRequired(normWireLiteral("changeTieBeamSteelDesignStrength")), id: normWireRequired(normWireString), newSteelDesignStrength: normWireRequired(normWireNumber) });
