import type{DrawingSourceAddress}from'../../../../🧬️schema/🟦️.ts';
export interface LayoutExportDrawingReadState{closing:boolean;planning:boolean;active:boolean;source:DrawingSourceAddress|undefined}
/** 🧭️ Export observes only its active fixed numeric source address before an outer owner obtains the real read. */
export function requiredExportDrawingSource(state:LayoutExportDrawingReadState):DrawingSourceAddress|undefined{return state.closing||!state.planning||!state.active?undefined:state.source}
/** 📖️ A Drawing consumer cannot substitute a cold codec when the actual original read is absent. */
export function requireOriginalExportDrawingRead<T>(source:DrawingSourceAddress|undefined,read:T|undefined):T|undefined{if(source&&!read)throw Error('Layout export requires the original qualified Drawing child read');return read}
