/** 🌬 `change-wind-face-c-pi` wire twin: the leaf payload `ChangeWindFaceCPi`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeWindFaceCPi {
  id: string;
  newCPi: number;
}

export const parseChangeWindFaceCPi: NormWireReader<ChangeWindFaceCPi> = normWireObject<ChangeWindFaceCPi>({ id: normWireRequired(normWireString), newCPi: normWireRequired(normWireNumber) });
