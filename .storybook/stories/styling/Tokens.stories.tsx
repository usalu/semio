// #region 🧲️Header
// 💻️ .storybook/stories/styling/Tokens.stories.tsx
// Specs: Docs gallery rendering the generated `framework/ui/styling/js/tokens.generated.ts` constants as readable tables.
// Summary: `STYLING_TOKENS` (primitive colors), `STYLING_SEMIO_THEME.spacing`, `STYLING_RADII`, `STYLING_STROKES`, and `STYLING_OPACITIES` — the flat numeric/hex maps a token author would want to eyeball after running `bun ./📜️script.ts generate` in `framework/ui/styling`. `STYLING_METRICS` is nested per-subsystem (camera/label/board/dag/…) rather than a flat map, so it's out of scope for a single table here.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

import type { Meta, StoryObj } from "@storybook/react-vite";
import type { ReactElement } from "react";

import { STYLING_OPACITIES, STYLING_RADII, STYLING_SEMIO_THEME, STYLING_STROKES, STYLING_TOKENS } from "@semio-tech/ui-styling";
import { DataTable, galleryCardStyle, galleryPageStyle } from "../../styling/index.tsx";

//#region 🔖️Formatting
function formatNumberOrArray(value: number | readonly number[]): string {
  return Array.isArray(value) ? `[${(value as readonly number[]).join(", ")}]` : String(value);
}
//#endregion 🔖️Formatting

//#region 🔖️ColorsTable
/** @emoji 🎨️ `STYLING_TOKENS` — the primitive hex palette every `ThemePaintRef.token` resolves against. */
function ColorsTable(): ReactElement {
  const entries = Object.entries(STYLING_TOKENS);
  return (
    <div style={galleryCardStyle}>
      <h3 style={{ margin: 0 }}>
        Colors <span style={{ fontSize: 12, opacity: 0.6, fontWeight: 400 }}>STYLING_TOKENS ({entries.length})</span>
      </h3>
      <DataTable
        columns={["token", "swatch", "hex"]}
        rows={entries.map(([name, hex]) => [
          name,
          <span key="swatch" style={{ display: "inline-block", width: 20, height: 14, borderRadius: 3, background: hex, border: "1px solid rgba(128, 128, 128, 0.4)", verticalAlign: "middle" }} />,
          hex,
        ])}
      />
    </div>
  );
}
//#endregion 🔖️ColorsTable

//#region 🔖️SpacingTable
/** @emoji 📏️ `STYLING_SEMIO_THEME.spacing` — the only spacing scale a `UiTheme` carries (no standalone `STYLING_SPACING` export; it lives inline on the theme premade). */
function SpacingTable(): ReactElement {
  const entries = Object.entries(STYLING_SEMIO_THEME.spacing);
  return (
    <div style={galleryCardStyle}>
      <h3 style={{ margin: 0 }}>
        Spacing <span style={{ fontSize: 12, opacity: 0.6, fontWeight: 400 }}>STYLING_SEMIO_THEME.spacing ({entries.length})</span>
      </h3>
      <DataTable columns={["key", "value"]} rows={entries.map(([name, value]) => [name, value])} />
    </div>
  );
}
//#endregion 🔖️SpacingTable

//#region 🔖️NumberMapTable
function NumberMapTable({ title, source, record }: { readonly title: string; readonly source: string; readonly record: Record<string, number | readonly number[]> }): ReactElement {
  const entries = Object.entries(record);
  return (
    <div style={galleryCardStyle}>
      <h3 style={{ margin: 0 }}>
        {title} <span style={{ fontSize: 12, opacity: 0.6, fontWeight: 400 }}>
          {source} ({entries.length})
        </span>
      </h3>
      <DataTable columns={["key", "value"]} rows={entries.map(([name, value]) => [name, formatNumberOrArray(value)])} />
    </div>
  );
}
//#endregion 🔖️NumberMapTable

const meta = {
  title: "🎨️styling/Tokens",
  component: ColorsTable,
  parameters: {
    layout: "fullscreen",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof ColorsTable>;

export default meta;

type Story = StoryObj<typeof meta>;

//#region 🔖️Stories
export const Colors: Story = {};

export const Spacing: Story = {
  render: () => (
    <div style={galleryPageStyle}>
      <SpacingTable />
    </div>
  ),
};

export const Radii: Story = {
  render: () => (
    <div style={galleryPageStyle}>
      <NumberMapTable title="Radii" source="STYLING_RADII" record={STYLING_RADII} />
    </div>
  ),
};

export const Strokes: Story = {
  render: () => (
    <div style={galleryPageStyle}>
      <NumberMapTable title="Strokes" source="STYLING_STROKES" record={STYLING_STROKES} />
    </div>
  ),
};

export const Opacities: Story = {
  render: () => (
    <div style={galleryPageStyle}>
      <NumberMapTable title="Opacities" source="STYLING_OPACITIES" record={STYLING_OPACITIES} />
    </div>
  ),
};

export const AllTokens: Story = {
  render: () => (
    <div style={galleryPageStyle}>
      <ColorsTable />
      <SpacingTable />
      <NumberMapTable title="Radii" source="STYLING_RADII" record={STYLING_RADII} />
      <NumberMapTable title="Strokes" source="STYLING_STROKES" record={STYLING_STROKES} />
      <NumberMapTable title="Opacities" source="STYLING_OPACITIES" record={STYLING_OPACITIES} />
    </div>
  ),
};
//#endregion 🔖️Stories
