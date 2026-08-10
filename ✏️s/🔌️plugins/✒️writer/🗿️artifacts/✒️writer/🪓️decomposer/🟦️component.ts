/** WriterDecomposer */
export interface Decomposition<T> { parts: T; confidence: 'high'|'medium'|'low'; diagnostics: unknown[]; }
