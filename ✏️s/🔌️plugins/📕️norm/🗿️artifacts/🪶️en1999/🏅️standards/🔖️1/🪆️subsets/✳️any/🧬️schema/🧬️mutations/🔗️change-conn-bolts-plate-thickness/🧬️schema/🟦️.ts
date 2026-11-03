/** 🔗️ `change-conn-bolts-plate-thickness` wire twin: the leaf payload `ChangeConnBoltsPlateThickness`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeConnBoltsPlateThickness {
  mutation: "changeConnBoltsPlateThickness";
  connectionId: string;
  newBoltsPlateThickness: number;
}

export const parseChangeConnBoltsPlateThickness: NormWireReader<ChangeConnBoltsPlateThickness> = normWireObject<ChangeConnBoltsPlateThickness>({ mutation: normWireRequired(normWireLiteral("changeConnBoltsPlateThickness")), connectionId: normWireRequired(normWireString), newBoltsPlateThickness: normWireRequired(normWireNumber) });
