/** 🌱 Direct `create-step` payload. */
export interface CreateStep {
  step: { id: string; kind: string; params?: Record<string, unknown>; x?: number; y?: number; slot?: { owner: string; name: string } | null; collapsed?: boolean };
}
