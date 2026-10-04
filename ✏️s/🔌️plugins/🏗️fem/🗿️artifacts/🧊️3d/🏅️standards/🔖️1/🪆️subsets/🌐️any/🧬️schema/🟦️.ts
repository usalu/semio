/** 🏗️ Artifact and Snapshot share the complete persisted FEM state. */
import {parseFem3dSnapshot,type Fem3dSnapshot} from "./📸️snapshot/🟦️.ts";
export type {FemNode,FemDof,FemAxis,FemElement,FemMaterial,FemSection,FemSupport,FemLoad,FemLoadCase,FemSolid,FemCombination,FemAnalysisSettings} from "./📸️snapshot/🟦️.ts";
export {parseFemAnalysisSettings,parseFemNode,parseFemElement,parseFemSolid,parseFemMaterial,parseFemSection,parseFemSupport,parseFemLoad,parseFemLoadCase,parseFemCombination} from "./📸️snapshot/🟦️.ts";
/** 🧬️ The artifact contains exactly its persisted typed snapshot fields. */
export interface Fem3dArtifact extends Fem3dSnapshot {}
/** 🛂️ Validate the literal owned artifact without a numeric wire projection. */
export function parseFem3dArtifact(value:unknown,at="$"):Fem3dArtifact{return parseFem3dSnapshot(value,at)}
