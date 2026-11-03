/** 🧷️ `change-restraint-design-axial` wire twin: the leaf payload `ChangeRestraintDesignAxial`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRestraintDesignAxial {
  id: string;
  newDesignAxial: number;
}

export const parseChangeRestraintDesignAxial: NormWireReader<ChangeRestraintDesignAxial> = normWireObject<ChangeRestraintDesignAxial>({ id: normWireRequired(normWireString), newDesignAxial: normWireRequired(normWireNumber) });
