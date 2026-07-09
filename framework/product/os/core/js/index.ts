// #region Header
/** @emoji 🖥️ `@semio-tech/framework-os-core` — minimal JS surface for OS program registration until full port lands. */
// #endregion Header

export type OsProgramResourceMap = Readonly<Record<string, { readonly kind: string; readonly id: string; readonly label: string }>>;

const programDefinitions = new Map<string, unknown>();
const vcsHandlers = new Set<() => void>();

export function osBaselineResource(kind: string, id: string, label: string) {
  return { kind, id, label };
}

export function mergeOsProgramDefinition(programId: string, definition: unknown, resources?: OsProgramResourceMap): void {
  programDefinitions.set(programId, { definition, resources });
}

export function registerAppVcsHandler(handler: () => void): void {
  vcsHandlers.add(handler);
}

export function osOutPort(resourceKind: string, id = "out", label = "Out") {
  return { id, label, resourceKind };
}

export function osInPort(resourceKind: string, id: string, label: string, required = false) {
  return { id, label, resourceKind, required };
}
