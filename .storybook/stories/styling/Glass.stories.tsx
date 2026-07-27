// #region 🧲Header
// 💻 .storybook/stories/styling/Glass.stories.tsx
// Specs: Docs gallery for the `Level` context (`LevelProvider`/`useLevel`) and its two glass fills (`@semio-tech/ui-react`) across every `Level`.
// Summary: `glassClass`/`glassChromeClass` are generic Tailwind `@utility` classes defined in `ui/styling/js/ui.css` (backdrop-filter blur/saturate + a `color-mix` fill); their alpha/blur are derived per the `[data-level="…"]` ancestor the swatch is stamped with — not per a hand-picked tier. Each swatch renders over a colorful backdrop so the blur/alpha differences across levels are visible without needing WebGL or a running plugin.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

import type { Meta, StoryObj } from "@storybook/react";
import type { ReactElement } from "react";

import { glassChromeClass, glassClass, LEVELS, LevelProvider, useLevel, type Level } from "@semio-tech/ui-react";
import { galleryPageStyle } from "../../styling/index.tsx";

//#region 🔖Variants
type GlassVariant = "glass" | "chrome";

/** @emoji 🌈 Busy backdrop every glass swatch sits over — a flat background would make blur/saturate differences invisible. */
const BACKDROP_STYLE = {
  backgroundImage: "conic-gradient(from 0deg, #ff344f, #fa9500, #fccf05, #7eb77f, #34d1bf, #ff344f)",
  backgroundSize: "48px 48px",
  padding: 24,
  borderRadius: 10,
} as const;
//#endregion 🔖Variants

//#region 🔖LevelGlassSwatch
/** @emoji 🪟 Reads the level from context via `useLevel()` (falling back to `"base"` per its own contract) rather than taking it as a prop — this is what a real consumer nested under `LevelProvider` does. */
function LevelGlassReader({ variant }: { readonly variant: GlassVariant }): ReactElement {
  const level = useLevel();
  const swatchClass = variant === "chrome" ? glassChromeClass : glassClass;
  return (
    <div data-level={level} className={swatchClass} style={{ borderRadius: 8, border: "1px solid rgba(128, 128, 128, 0.4)", padding: "20px 16px", minWidth: 160, textAlign: "center" }}>
      <div style={{ fontSize: 13, fontWeight: 600 }}>{level}</div>
      <div style={{ fontSize: 11, opacity: 0.7, fontFamily: "monospace" }}>.{swatchClass}</div>
    </div>
  );
}

/** @emoji 🪟 One `LevelProvider` boundary + a reader nested inside — demonstrates the provider/hook contract, not just the class lookup table. */
function LevelGlassSwatch({ level, variant }: { readonly level: Level; readonly variant: GlassVariant }): ReactElement {
  return (
    <LevelProvider level={level}>
      <LevelGlassReader variant={variant} />
    </LevelProvider>
  );
}
//#endregion 🔖LevelGlassSwatch

//#region 🔖Gallery
function GlassGallery(): ReactElement {
  return (
    <div style={galleryPageStyle}>
      <div style={BACKDROP_STYLE}>
        <div style={{ display: "flex", flexDirection: "column", gap: 16 }}>
          <div style={{ display: "flex", flexWrap: "wrap", gap: 16 }}>
            {LEVELS.map((level) => (
              <LevelGlassSwatch key={`glass-${level}`} level={level} variant="glass" />
            ))}
          </div>
          <div style={{ display: "flex", flexWrap: "wrap", gap: 16 }}>
            {LEVELS.map((level) => (
              <LevelGlassSwatch key={`chrome-${level}`} level={level} variant="chrome" />
            ))}
          </div>
        </div>
      </div>
      <p style={{ fontSize: 12, opacity: 0.6, maxWidth: 640 }}>
        Each card is a `LevelProvider` boundary around a `useLevel()` reader — `.ui-glass` / `.ui-glass-chrome` resolve their alpha and blur from the `[data-level]` ancestor via the formula
        in the levels contract (<code>base=0..menu=5</code>), never a hand-picked per-tier value (see the `theme` toolbar).
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
export const AllLevels: Story = {};

export const PanelLevel: Story = {
  render: () => (
    <div style={galleryPageStyle}>
      <div style={BACKDROP_STYLE}>
        <LevelGlassSwatch level="panel" variant="glass" />
      </div>
    </div>
  ),
};

export const PaneLevelChrome: Story = {
  render: () => (
    <div style={galleryPageStyle}>
      <div style={BACKDROP_STYLE}>
        <LevelGlassSwatch level="pane" variant="chrome" />
      </div>
    </div>
  ),
};

/** @emoji 🪝 Outside any `LevelProvider`, `useLevel()` falls back to `"base"` per its own docstring contract. */
export const DefaultLevelFallback: Story = {
  render: () => (
    <div style={galleryPageStyle}>
      <div style={BACKDROP_STYLE}>
        <LevelGlassReader variant="glass" />
      </div>
    </div>
  ),
};
//#endregion 🔖Stories
