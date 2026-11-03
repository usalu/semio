/** 🚆 `change-derailment-design-force-x` wire twin: the leaf payload `ChangeDerailmentDesignForceX`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeDerailmentDesignForceX {
  id: string;
  newDesignForceX: number;
}

export const parseChangeDerailmentDesignForceX: NormWireReader<ChangeDerailmentDesignForceX> = normWireObject<ChangeDerailmentDesignForceX>({ id: normWireRequired(normWireString), newDesignForceX: normWireRequired(normWireNumber) });
