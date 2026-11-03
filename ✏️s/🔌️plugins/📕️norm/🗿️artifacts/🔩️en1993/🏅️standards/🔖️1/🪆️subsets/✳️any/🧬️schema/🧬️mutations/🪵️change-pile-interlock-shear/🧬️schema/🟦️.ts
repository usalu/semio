/** 🪵️ `change-pile-interlock-shear` wire twin: the leaf payload `ChangePileInterlockShear`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePileInterlockShear {
  id: string;
  newInterlockShear: number;
}

export const parseChangePileInterlockShear: NormWireReader<ChangePileInterlockShear> = normWireObject<ChangePileInterlockShear>({ id: normWireRequired(normWireString), newInterlockShear: normWireRequired(normWireNumber) });
