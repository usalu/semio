/** 🧯️ Preserves an owned graph category and the original diagnostic message. */
export function graphDiagnostic(kind: 'Json' | 'Pack' | 'Dag', message: string): {kind:'Json'|'Pack'|'Dag';message:string} { return {kind,message}; }
