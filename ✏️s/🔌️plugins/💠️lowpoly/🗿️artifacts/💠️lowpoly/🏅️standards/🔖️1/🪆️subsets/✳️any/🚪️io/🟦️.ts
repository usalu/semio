/** Lowpoly IO facet — curated stdio matrix, mirrors `import_stdio_kinds()` / `export_stdio_kinds()` in `🦀️.rs`. */
export const LOWPOLY_IO_FORMATS = ["dwg", "gltf", "json", "las", "obj", "ply", "png", "stl", "txt"] as const;
export type LowpolyIoFormat = (typeof LOWPOLY_IO_FORMATS)[number];

export function lowpolyIoAcceptFilter(formats: readonly LowpolyIoFormat[] = LOWPOLY_IO_FORMATS): string {
  return formats.map((format) => `.${format}`).join(",");
}

export function lowpolyIoExportMenu(formats: readonly LowpolyIoFormat[] = LOWPOLY_IO_FORMATS): ReadonlyArray<{ format: LowpolyIoFormat; label: string }> {
  return formats.map((format) => ({ format, label: format.toUpperCase() }));
}

export type LowpolyIoHostBridge = {
  exportMedia: (format: LowpolyIoFormat) => void | Promise<void>;
  importMedia: (format: LowpolyIoFormat) => void | Promise<void>;
};

let bridge: LowpolyIoHostBridge | null = null;

export function installLowpolyIoHostBridge(next: LowpolyIoHostBridge): void {
  bridge = next;
}

export async function exportLowpolyMedia(format: LowpolyIoFormat): Promise<void> {
  if (!bridge) throw new Error("lowpoly io host bridge missing — installLowpolyIoHostBridge first");
  await bridge.exportMedia(format);
}

export async function importLowpolyMedia(format: LowpolyIoFormat): Promise<void> {
  if (!bridge) throw new Error("lowpoly io host bridge missing — installLowpolyIoHostBridge first");
  await bridge.importMedia(format);
}
