/** 🪜️ `change-built-up-design-axial` wire twin: the leaf payload `ChangeBuiltUpDesignAxial`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBuiltUpDesignAxial {
  id: string;
  newDesignAxial: number;
}

export const parseChangeBuiltUpDesignAxial: NormWireReader<ChangeBuiltUpDesignAxial> = normWireObject<ChangeBuiltUpDesignAxial>({ id: normWireRequired(normWireString), newDesignAxial: normWireRequired(normWireNumber) });
