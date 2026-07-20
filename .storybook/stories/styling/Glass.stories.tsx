// #region 🧲Header
// 💻 .storybook/stories/styling/Glass.stories.tsx
// Specs: Docs gallery for `GlassTierProvider`/`useGlassTier`/`getGlassSurfaceClass` (`@semio-tech/ui-react`) across every `GlassTier`.
// Summary: `getGlassSurfaceClass` maps a `GlassTier` ("panel"|"ribbon"|"menu"|"windowOptions") to one of the `ui-glass-*` Tailwind `@utility` classes defined in `ui/styling/js/ui.css` (backdrop-filter blur/saturate + a `color-mix` fill over `--panel`/`--temporary`). Each swatch renders over a colorful backdrop so the blur/alpha differences across tiers are visible without needing WebGL or a running plugin.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

import type { Meta, StoryObj } from "@storybook/react";
import type { ReactElement } from "react";

import { getGlassSurfaceClass, GlassTierProvider, useGlassTier, type GlassTier } from "@semio-tech/ui-react";
import { galleryPageStyle } from "../../styling/index.tsx";

//#region 🔖Tiers
const GLASS_TIERS: readonly GlassTier[] = ["panel", "ribbon", "menu", "windowOptions"];

/** @emoji 🌈 Busy backdrop every glass swatch sits over — a flat background would make blur/saturate differences invisible. */
const BACKDROP_STYLE = {
  backgroundImage: "conic-gradient(from 0deg, #ff344f, #fa9500, #fccf05, #7eb77f, #34d1bf, #ff344f)",
  backgroundSize: "48px 48px",
  padding: 24,
  borderRadius: 10,
} as const;
//#endregion 🔖Tiers

//#region 🔖GlassTierSwatch
/** @emoji 🪟 Reads the tier from context via `useGlassTier()` (falling back to "menu" per its own contract) rather than taking it as a prop — this is what a real consumer nested under `GlassTierProvider` does. */
function GlassTierReader(): ReactElement {
  const tier = useGlassTier();
  const glassClass = getGlassSurfaceClass(tier);
  return (
    <div className={glassClass} style={{ borderRadius: 8, border: "1px solid rgba(128, 128, 128, 0.4)", padding: "20px 16px", minWidth: 160, textAlign: "center" }}>
      <div style={{ fontSize: 13, fontWeight: 600 }}>{tier}</div>
      <div style={{ fontSize: 11, opacity: 0.7, fontFamily: "monospace" }}>.{glassClass}</div>
    </div>
  );
}

/** @emoji 🪟 One `GlassTierProvider` boundary + a reader nested inside — demonstrates the provider/hook contract, not just the class lookup table. */
function GlassTierSwatch({ tier }: { readonly tier: GlassTier }): ReactElement {
  return (
    <GlassTierProvider tier={tier}>
      <GlassTierReader />
    </GlassTierProvider>
  );
}
//#endregion 🔖GlassTierSwatch

//#region 🔖Gallery
function GlassGallery(): ReactElement {
  return (
    <div style={galleryPageStyle}>
      <div style={BACKDROP_STYLE}>
        <div style={{ display: "flex", flexWrap: "wrap", gap: 16 }}>
          {GLASS_TIERS.map((tier) => (
            <GlassTierSwatch key={tier} tier={tier} />
          ))}
        </div>
      </div>
      <p style={{ fontSize: 12, opacity: 0.6, maxWidth: 640 }}>
        Each card is a `GlassTierProvider` boundary around a `useGlassTier()` reader — `getGlassSurfaceClass(tier)` resolves to `ui-glass-panel` / `ui-glass-ribbon` / `ui-glass-menu` /
        `ui-glass-window-options`, whose blur radius and fill alpha come from the active theme's <code>chrome.glassBlurPx</code>/<code>chrome.glassPanelBlurPx</code>/
        <code>chrome.glassWindowOptionsBlurPx</code> metrics and <code>opacities.glassPanelAlpha</code>/<code>glassMenuAlpha</code>/<code>glassWindowOptionsAlpha</code> (see the `theme` toolbar).
      </p>
    </div>
  );
}
//#endregion 🔖Gallery

const meta = {
  title: "🎨styling/Glass",
  component: GlassGallery,
  parameters: {
    layout: "fullscreen",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof GlassGallery>;

export default meta;

type Story = StoryObj<typeof meta>;

//#region 🔖Stories
export const AllTiers: Story = {};

export const PanelTier: Story = {
  render: () => (
    <div style={galleryPageStyle}>
      <div style={BACKDROP_STYLE}>
        <GlassTierSwatch tier="panel" />
      </div>
    </div>
  ),
};

export const WindowOptionsTier: Story = {
  render: () => (
    <div style={galleryPageStyle}>
      <div style={BACKDROP_STYLE}>
        <GlassTierSwatch tier="windowOptions" />
      </div>
    </div>
  ),
};

/** @emoji 🪝 Outside any `GlassTierProvider`, `useGlassTier()` falls back to `"menu"` per its own docstring contract. */
export const DefaultTierFallback: Story = {
  render: () => (
    <div style={galleryPageStyle}>
      <div style={BACKDROP_STYLE}>
        <GlassTierReader />
      </div>
    </div>
  ),
};
//#endregion 🔖Stories
