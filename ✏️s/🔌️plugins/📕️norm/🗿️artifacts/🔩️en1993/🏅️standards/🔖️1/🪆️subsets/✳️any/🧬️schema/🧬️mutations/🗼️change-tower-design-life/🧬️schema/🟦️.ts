/** 🗼️ `change-tower-design-life` wire twin: the leaf payload `ChangeTowerDesignLife`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTowerDesignLife {
  id: string;
  newDesignLife: number;
}

export const parseChangeTowerDesignLife: NormWireReader<ChangeTowerDesignLife> = normWireObject<ChangeTowerDesignLife>({ id: normWireRequired(normWireString), newDesignLife: normWireRequired(normWireNumber) });
