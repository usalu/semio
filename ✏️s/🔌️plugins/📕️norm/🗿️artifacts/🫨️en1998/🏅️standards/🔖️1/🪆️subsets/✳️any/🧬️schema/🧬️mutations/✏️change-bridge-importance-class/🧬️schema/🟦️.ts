/** ✏️ `change-bridge-importance-class` wire twin: the leaf payload `ChangeBridgeImportanceClass`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBridgeImportanceClass {
  mutation: "changeBridgeImportanceClass";
  id: string;
  newImportanceClass: number;
}

export const parseChangeBridgeImportanceClass: NormWireReader<ChangeBridgeImportanceClass> = normWireObject<ChangeBridgeImportanceClass>({ mutation: normWireRequired(normWireLiteral("changeBridgeImportanceClass")), id: normWireRequired(normWireString), newImportanceClass: normWireRequired(normWireNumber) });
