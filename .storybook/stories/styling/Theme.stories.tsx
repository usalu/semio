// #region 🧲Header
// 💻 .storybook/stories/styling/Theme.stories.tsx
// Specs: Docs gallery for every builtin `UiTheme` × light/dark appearance × `ThemePaletteGroup`.
// Summary: Iterates `builtinUiThemes()` (re-exported by `@semio-tech/ui-react` from `@semio-tech/ui-styling`) and resolves each theme's `board`/`map`/`canvas`/`chrome` paints via `resolveThemeAppearancePalettes` into swatch grids — a read-only gallery, not a component under test.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

import type { Meta, StoryObj } from "@storybook/react";
import type { ReactElement } from "react";

import { builtinUiThemes } from "@semio-tech/ui-react";
import { resolveThemeAppearancePalettes, type ThemeAppearanceName, type ThemePaletteGroup, type UiTheme } from "@semio-tech/ui-styling";
import { galleryCardStyle, galleryPageStyle, SwatchGrid } from "../../styling/index.tsx";

//#region 🔖Groups
const PALETTE_GROUPS: readonly ThemePaletteGroup[] = ["board", "map", "canvas", "chrome"];
const APPEARANCES: readonly ThemeAppearanceName[] = ["light", "dark"];
//#endregion 🔖Groups

//#region 🔖ThemeAppearanceCard
/** @emoji 🌓 One theme resolved for one appearance — every palette group's swatches stacked in a bordered card. */
function ThemeAppearanceCard({ theme, appearance }: { readonly theme: UiTheme; readonly appearance: ThemeAppearanceName }): ReactElement {
  const palettes = resolveThemeAppearancePalettes(theme, appearance);
  return (
    <div style={galleryCardStyle}>
      <h3 style={{ margin: 0, fontSize: 14, textTransform: "capitalize" }}>{appearance}</h3>
      {PALETTE_GROUPS.map((group) => (
        <SwatchGrid key={group} title={group} entries={Object.entries(palettes[group])} />
      ))}
    </div>
  );
}
//#endregion 🔖ThemeAppearanceCard

//#region 🔖ThemeSection
function ThemeSection({ theme }: { readonly theme: UiTheme }): ReactElement {
  return (
    <section style={{ display: "flex", flexDirection: "column", gap: 16 }}>
      <h2 style={{ margin: 0 }}>
        {theme.label} <code style={{ fontSize: 12, opacity: 0.6, fontWeight: 400 }}>{theme.id}</code>
      </h2>
      <div style={{ display: "grid", gridTemplateColumns: "repeat(auto-fit, minmax(380px, 1fr))", gap: 16 }}>
        {APPEARANCES.map((appearance) => (
          <ThemeAppearanceCard key={appearance} theme={theme} appearance={appearance} />
        ))}
      </div>
    </section>
  );
}
//#endregion 🔖ThemeSection

//#region 🔖Gallery
/** @emoji 🖼️ Every builtin theme (semio + any `framework/ui/styling/theme/*.theme.json` premade) rendered light+dark. `component` is this gallery itself — the story exercises `builtinUiThemes()`/`resolveThemeAppearancePalettes` data, not a UI widget. */
function ThemeGallery(): ReactElement {
  const themes = builtinUiThemes();
  return (
    <div style={galleryPageStyle}>
      {themes.map((theme) => (
        <ThemeSection key={theme.id} theme={theme} />
      ))}
    </div>
  );
}
//#endregion 🔖Gallery

const meta = {
  title: "🎨styling/Theme",
  component: ThemeGallery,
  parameters: {
    layout: "fullscreen",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof ThemeGallery>;

export default meta;

type Story = StoryObj<typeof meta>;

//#region 🔖Stories
export const AllThemes: Story = {};

export const SemioOnly: Story = {
  render: () => {
    const theme = builtinUiThemes().find((candidate) => candidate.id === "semio") ?? builtinUiThemes()[0]!;
    return (
      <div style={galleryPageStyle}>
        <ThemeSection theme={theme} />
      </div>
    );
  },
};

export const BoardGroupAcrossThemes: Story = {
  render: () => (
    <div style={galleryPageStyle}>
      {builtinUiThemes().flatMap((theme) =>
        APPEARANCES.map((appearance) => {
          const palettes = resolveThemeAppearancePalettes(theme, appearance);
          return <SwatchGrid key={`${theme.id}-${appearance}`} title={`${theme.label} · ${appearance} · board`} entries={Object.entries(palettes.board)} />;
        }),
      )}
    </div>
  ),
};
//#endregion 🔖Stories
