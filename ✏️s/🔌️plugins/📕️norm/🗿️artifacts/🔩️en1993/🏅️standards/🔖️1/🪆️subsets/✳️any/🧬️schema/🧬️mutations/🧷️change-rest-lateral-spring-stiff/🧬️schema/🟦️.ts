/** 🧷️ `change-rest-lateral-spring-stiff` wire twin: the leaf payload `ChangeRestLateralSpringStiff`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRestLateralSpringStiff {
  id: string;
  newLateralSpringStiffness: number;
}

export const parseChangeRestLateralSpringStiff: NormWireReader<ChangeRestLateralSpringStiff> = normWireObject<ChangeRestLateralSpringStiff>({ id: normWireRequired(normWireString), newLateralSpringStiffness: normWireRequired(normWireNumber) });
