/** 🪜️ `change-built-up-design-moment` wire twin: the leaf payload `ChangeBuiltUpDesignMoment`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBuiltUpDesignMoment {
  id: string;
  newDesignMoment: number;
}

export const parseChangeBuiltUpDesignMoment: NormWireReader<ChangeBuiltUpDesignMoment> = normWireObject<ChangeBuiltUpDesignMoment>({ id: normWireRequired(normWireString), newDesignMoment: normWireRequired(normWireNumber) });
