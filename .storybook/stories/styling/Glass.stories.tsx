// #region 🧲️Header
// 💻️ .storybook/stories/styling/Glass.stories.tsx
// Specs: Docs gallery for the `Level` context (`LevelProvider`/`useLevel`) and its glass fill (`@semio-tech/ui-react`) across every `Level`.
// Summary: `glassClass` is a generic Tailwind `@utility` class defined in `framework/ui/styling/js/ui.css` (backdrop-filter blur/saturate + a `color-mix` fill); its alpha/blur are derived per the `[data-level="…"]` ancestor the swatch is stamped with — not per a hand-picked tier. A level's attached chrome (title caps, ribbons, tab bars, rails) renders this exact same fill as its body, so one level always shows one appearance. Each swatch renders over a colorful backdrop so the blur/alpha differences across levels are visible without needing WebGL or a running program.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

import type { Meta, StoryObj } from "@storybook/react-vite";
import type { ReactElement } from "react";

import { glassClass, LEVELS, LevelProvider, useLevel, type Level } from "@semio-tech/ui-react";
import { galleryPageStyle } from "../../styling/index.tsx";

//#region 🔖️Variants
/** @emoji 🌈️ Busy backdrop every glass swatch sits over — a flat background would make blur/saturate differences invisible. */
const BACKDROP_STYLE = {
  backgroundImage: "conic-gradient(from 0deg, #ff344f, #fa9500, #fccf05, #7eb77f, #34d1bf, #ff344f)",
  backgroundSize: "48px 48px",
  padding: 24,
  borderRadius: 10,
} as const;
//#endregion 🔖️Variants

//#region 🔖️LevelGlassSwatch
/** @emoji 🪟️ Reads the level from context via `useLevel()` (falling back to `"base"` per its own contract) rather than taking it as a prop — this is what a real consumer nested under `LevelProvider` does. */
function LevelGlassReader(): ReactElement {
  const level = useLevel();
  return (
    <div data-level={level} className={glassClass} style={{ borderRadius: 8, border: "1px solid rgba(128, 128, 128, 0.4)", padding: "20px 16px", minWidth: 160, textAlign: "center" }}>
      <div style={{ fontSize: 13, fontWeight: 600 }}>{level}</div>
      <div style={{ fontSize: 11, opacity: 0.7, fontFamily: "monospace" }}>.{glassClass}</div>
    </div>
  );
}

/** @emoji 🪟️ One `LevelProvider` boundary + a reader nested inside — demonstrates the provider/hook contract, not just the class lookup table. */
function LevelGlassSwatch({ level }: { readonly level: Level }): ReactElement {
  return (
    <LevelProvider level={level}>
      <LevelGlassReader />
    </LevelProvider>
  );
}
//#endregion 🔖️LevelGlassSwatch

//#region 🔖️Gallery
function GlassGallery(): ReactElement {
  return (
    <div style={galleryPageStyle}>
      <div style={BACKDROP_STYLE}>
        <div style={{ display: "flex", flexWrap: "wrap", gap: 16 }}>
          {LEVELS.map((level) => (
            <LevelGlassSwatch key={level} level={level} />
          ))}
        </div>
      </div>
      <p style={{ fontSize: 12, opacity: 0.6, maxWidth: 640 }}>
        Each card is a `LevelProvider` boundary around a `useLevel()` reader — `.ui-glass` resolves its alpha and blur from the `[data-level]` ancestor via the formula in the levels
        contract (<code>base=0..menu=5</code>), never a hand-picked per-tier value (see the `theme` toolbar). A level's attached chrome renders this identical fill, never a lighter variant.
      </p>
    </div>
  );
}
//#endregion 🔖️Gallery

const meta = {
  title: "🎨️styling/Glass",
  component: GlassGallery,
  parameters: {
    layout: "fullscreen",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof GlassGallery>;

export default meta;

type Story = StoryObj<typeof meta>;

//#region 🔖️Stories
export const AllLevels: Story = {};

export const PanelLevel: Story = {
  render: () => (
    <div style={galleryPageStyle}>
      <div style={BACKDROP_STYLE}>
        <LevelGlassSwatch level="panel" />
      </div>
    </div>
  ),
};

export const PaneLevel: Story = {
  render: () => (
    <div style={galleryPageStyle}>
      <div style={BACKDROP_STYLE}>
        <LevelGlassSwatch level="pane" />
      </div>
    </div>
  ),
};

/** @emoji 🪝️ Outside any `LevelProvider`, `useLevel()` falls back to `"base"` per its own docstring contract. */
export const DefaultLevelFallback: Story = {
  render: () => (
    <div style={galleryPageStyle}>
      <div style={BACKDROP_STYLE}>
        <LevelGlassReader />
      </div>
    </div>
  ),
};
//#endregion 🔖️Stories
