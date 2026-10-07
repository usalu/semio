import{artifactSqliteCheckpoint,type ArtifactSqliteOptions}from"../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import {proceduralValidation} from "../../🧬️schema/📸️snapshot/🛡️admission/🟦️.ts";
/** ⏱️ Validate every borrowed owned field before relational projection under caller cancellation. */
export async function validateProceduralSnapshotForProjection(value:unknown,options:ArtifactSqliteOptions):Promise<void>{for(const units of proceduralValidation("snapshot",value,options.maxRows??1000000))await artifactSqliteCheckpoint(options,"projectSnapshot",units,0);}
