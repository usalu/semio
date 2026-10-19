/** 🖼️ The display model an interaction state resolves to: display items and the model that carries them. */

/** 🖼️ Resolved display primitive for renderer adapters. */
export interface DisplayItem {
  readonly kind: string;
  readonly id: string;
  readonly role?: string;
  readonly params?: Record<string, unknown>;
}

/** 🖼️ Renderer-neutral snapshot slice consumed by `@semio-tech/cad-js/renderer`. */
export interface DisplayModel {
  readonly prompt?: string;
  readonly items: readonly DisplayItem[];
}
