import { afterEach, vi } from "vitest";
import { cleanup } from "@testing-library/react";
vi.mock("@semio-tech/assets", () => ({
  ICONS: {},
  isIconName: () => true,
  resolveCatalogIconSvgFromTheme: () => "<svg></svg>",
  shortcodeCatalogKey: () => undefined,
  shortcodeEmoji: () => undefined,
}));
vi.mock("@semio-tech/ui-styling", () => {
  const theme = { icons: {} };
  return {
  domSizePx: () => 16,
  uiSpacingPx: () => 4,
  uiSpacingLen: () => "4px",
  sizeVar: (name: string) => `var(--size-${name})`,
  STYLING_COMPACT_ROOT_PX: 16,
  activeUiTheme: () => theme,
  subscribeActiveUiTheme: () => () => undefined,
  };
});
afterEach(cleanup);
if (!globalThis.PointerEvent) globalThis.PointerEvent = MouseEvent as typeof PointerEvent;
if (!globalThis.ResizeObserver) globalThis.ResizeObserver = class { observe() {} unobserve() {} disconnect() {} };
if (!window.matchMedia) window.matchMedia = (query) => ({ matches: false, media: query, onchange: null, addListener() {}, removeListener() {}, addEventListener() {}, removeEventListener() {}, dispatchEvent: () => false });
