/** 🔩️ `change-joint-extension-to-flange` wire twin: the leaf payload `ChangeJointExtensionToFlange`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointExtensionToFlange {
  id: string;
  newExtensionToFlange: number;
}

export const parseChangeJointExtensionToFlange: NormWireReader<ChangeJointExtensionToFlange> = normWireObject<ChangeJointExtensionToFlange>({ id: normWireRequired(normWireString), newExtensionToFlange: normWireRequired(normWireNumber) });
