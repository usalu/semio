/** 📝️ Owned Generation2d native text representation. */
export type Generation2dSnapshotText=string;
/** 🔤️ Admit the actual text primitive without coercion. */
export function parseGeneration2dSnapshotText(v:unknown):Generation2dSnapshotText{if(typeof v!=="string")throw Error("Generation2d snapshot text differs");return v;}
