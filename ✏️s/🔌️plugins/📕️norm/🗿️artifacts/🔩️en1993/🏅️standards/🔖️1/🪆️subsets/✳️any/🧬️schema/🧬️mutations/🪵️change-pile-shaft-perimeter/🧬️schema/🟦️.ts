/** 🪵️ `change-pile-shaft-perimeter` wire twin: the leaf payload `ChangePileShaftPerimeter`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePileShaftPerimeter {
  id: string;
  newShaftPerimeter: number;
}

export const parseChangePileShaftPerimeter: NormWireReader<ChangePileShaftPerimeter> = normWireObject<ChangePileShaftPerimeter>({ id: normWireRequired(normWireString), newShaftPerimeter: normWireRequired(normWireNumber) });
