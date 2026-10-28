export interface ExportMilestone{stage:number;rawBytes:number;encodedBytes:number;publishedChunks:number}
/** 📍️ Only an emitted prefix, phase change or accepted output chunk commits a replay milestone. */
export function committedMilestone(before:ExportMilestone,after:ExportMilestone):boolean{return before.stage!==after.stage||before.rawBytes!==after.rawBytes||before.encodedBytes!==after.encodedBytes||before.publishedChunks!==after.publishedChunks;}
/** 🔁️ Work turns do not identify replay position; the matching committed prefix does. */
export function replayPosition(current:number,target:number,samePrefix:boolean,finished=false):boolean{if(current<target&&!finished)return false;if(current!==target||!samePrefix)throw Error('layout-export-checkpoint-replay-mismatch');return true;}
