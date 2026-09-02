/** CAD IO facet — curated stdio matrix. */
export const CAD_IO_FORMATS = ["dwg", "gltf", "ifc", "json", "obj", "png", "step", "stl"] as const;
export type CadIoFormat = (typeof CAD_IO_FORMATS)[number];

export function cadIoAcceptFilter(formats: readonly CadIoFormat[] = CAD_IO_FORMATS): string {
  return formats.map((format) => `.${format}`).join(",");
}

export function cadIoExportMenu(formats: readonly CadIoFormat[] = CAD_IO_FORMATS): ReadonlyArray<{ format: CadIoFormat; label: string }> {
  return formats.map((format) => ({ format, label: format.toUpperCase() }));
}

export type CadIoHostBridge = {
  exportMedia: (format: CadIoFormat) => void | Promise<void>;
  importMedia: (format: CadIoFormat) => void | Promise<void>;
};

let bridge: CadIoHostBridge | null = null;

export function installCadIoHostBridge(next: CadIoHostBridge): void {
  bridge = next;
}

export async function exportCadMedia(format: CadIoFormat): Promise<void> {
  if (!bridge) throw new Error("[DEBUG] cad io host bridge missing — installCadIoHostBridge first");
  await bridge.exportMedia(format);
}

export async function importCadMedia(format: CadIoFormat): Promise<void> {
  if (!bridge) throw new Error("[DEBUG] cad io host bridge missing — installCadIoHostBridge first");
  await bridge.importMedia(format);
}
