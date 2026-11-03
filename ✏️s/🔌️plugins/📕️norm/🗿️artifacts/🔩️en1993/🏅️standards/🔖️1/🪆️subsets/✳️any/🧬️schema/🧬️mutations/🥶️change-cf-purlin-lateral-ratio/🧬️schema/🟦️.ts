/** 🥶️ `change-cf-purlin-lateral-ratio` wire twin: the leaf payload `ChangeCfPurlinLateralRatio`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCfPurlinLateralRatio {
  id: string;
  newPurlinLateralRatio: number;
}

export const parseChangeCfPurlinLateralRatio: NormWireReader<ChangeCfPurlinLateralRatio> = normWireObject<ChangeCfPurlinLateralRatio>({ id: normWireRequired(normWireString), newPurlinLateralRatio: normWireRequired(normWireNumber) });
