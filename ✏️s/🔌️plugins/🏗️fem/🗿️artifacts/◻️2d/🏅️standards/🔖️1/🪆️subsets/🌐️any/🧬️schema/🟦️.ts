/** 🏗️ Artifact and Snapshot share the complete persisted FEM state. */
import {parseFem2dSnapshot,type Fem2dSnapshot} from "./📸️snapshot/🟦️.ts";
export type {FemNode,FemDof,FemElement,FemMaterial,FemSection,FemSupport,FemLoad,FemLoadCase,FemRegion,FemCombinationTerm,FemCombination,FemAnalysisSettings} from "./📸️snapshot/🟦️.ts";
export {parseFemAnalysisSettings,parseFemNode,parseFemElement,parseFemRegion,parseFemMaterial,parseFemSection,parseFemSupport,parseFemLoad,parseFemLoadCase,parseFemCombination} from "./📸️snapshot/🟦️.ts";
/** 🧬️ The artifact contains exactly its persisted typed snapshot fields. */
export interface Fem2dArtifact extends Fem2dSnapshot {}
/** 🛂️ Validate the literal owned artifact without a numeric wire projection. */
export function parseFem2dArtifact(value:unknown,at="$"):Fem2dArtifact{return parseFem2dSnapshot(value,at)}
