/** 🌉️ `change-bridge-design-life` wire twin: the leaf payload `ChangeBridgeDesignLife`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBridgeDesignLife {
  id: string;
  newDesignLife: number;
}

export const parseChangeBridgeDesignLife: NormWireReader<ChangeBridgeDesignLife> = normWireObject<ChangeBridgeDesignLife>({ id: normWireRequired(normWireString), newDesignLife: normWireRequired(normWireNumber) });
