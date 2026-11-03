/** 🗼️ `change-tower-shell-thickness` wire twin: the leaf payload `ChangeTowerShellThickness`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTowerShellThickness {
  id: string;
  newShellThickness: number;
}

export const parseChangeTowerShellThickness: NormWireReader<ChangeTowerShellThickness> = normWireObject<ChangeTowerShellThickness>({ id: normWireRequired(normWireString), newShellThickness: normWireRequired(normWireNumber) });
