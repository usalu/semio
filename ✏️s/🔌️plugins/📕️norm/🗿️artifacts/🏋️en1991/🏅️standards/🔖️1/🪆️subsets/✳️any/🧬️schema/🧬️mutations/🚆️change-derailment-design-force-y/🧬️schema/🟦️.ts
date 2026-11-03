/** 🚆 `change-derailment-design-force-y` wire twin: the leaf payload `ChangeDerailmentDesignForceY`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeDerailmentDesignForceY {
  id: string;
  newDesignForceY: number;
}

export const parseChangeDerailmentDesignForceY: NormWireReader<ChangeDerailmentDesignForceY> = normWireObject<ChangeDerailmentDesignForceY>({ id: normWireRequired(normWireString), newDesignForceY: normWireRequired(normWireNumber) });
