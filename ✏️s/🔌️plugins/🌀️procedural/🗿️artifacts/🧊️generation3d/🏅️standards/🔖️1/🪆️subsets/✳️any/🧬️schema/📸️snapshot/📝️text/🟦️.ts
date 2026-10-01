/** 📝️ Owned Generation3d native text representation. */
export type Generation3dSnapshotText=string;
/** 🔤️ Admit the actual text primitive without coercion. */
export function parseGeneration3dSnapshotText(v:unknown):Generation3dSnapshotText{if(typeof v!=="string")throw Error("Generation3d snapshot text differs");return v;}
