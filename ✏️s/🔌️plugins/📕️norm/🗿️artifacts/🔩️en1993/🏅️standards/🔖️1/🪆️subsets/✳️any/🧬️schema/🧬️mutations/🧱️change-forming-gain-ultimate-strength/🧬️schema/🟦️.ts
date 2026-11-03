/** 🧱️ `change-forming-gain-ultimate-strength` wire twin: the leaf payload `ChangeFormingGainUltimateStrength`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFormingGainUltimateStrength {
  id: string;
  newUltimateStrength: number;
}

export const parseChangeFormingGainUltimateStrength: NormWireReader<ChangeFormingGainUltimateStrength> = normWireObject<ChangeFormingGainUltimateStrength>({ id: normWireRequired(normWireString), newUltimateStrength: normWireRequired(normWireNumber) });
