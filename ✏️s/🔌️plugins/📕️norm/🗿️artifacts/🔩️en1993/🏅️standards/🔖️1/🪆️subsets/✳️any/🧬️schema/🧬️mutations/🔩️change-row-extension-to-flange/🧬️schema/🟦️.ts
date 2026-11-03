/** 🔩️ `change-row-extension-to-flange` wire twin: the leaf payload `ChangeRowExtensionToFlange`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRowExtensionToFlange {
  id: string;
  newExtensionToFlange: number;
}

export const parseChangeRowExtensionToFlange: NormWireReader<ChangeRowExtensionToFlange> = normWireObject<ChangeRowExtensionToFlange>({ id: normWireRequired(normWireString), newExtensionToFlange: normWireRequired(normWireNumber) });
