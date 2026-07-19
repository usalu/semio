// #region 🧲Header
// 💻 .storybook/styling/index.tsx
// Specs: Shared docs-gallery primitives for the `styling` Storybook scope's story files.
// Summary: Swatch/table rendering shared by Theme/Tokens/Glass/ThemeRoundtrip stories under `stories/styling/` — none of these stories exercise a component under test, they render `@semio-tech/ui-styling` data as a readable gallery, so the formatting helpers live here rather than duplicated per file.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

import type { CSSProperties, ReactElement, ReactNode } from "react";
import type { Rgba8 } from "@semio-tech/ui-styling";

//#region 🔖Color
/** @emoji 🎨 CSS `rgba()` for an sRGB8888 tuple as resolved by `resolveThemePaint`/`resolveThemeAppearancePalettes`. */
export function rgba8ToCss([r, g, b, a]: Rgba8): string {
  return `rgba(${r}, ${g}, ${b}, ${(a / 255).toFixed(3)})`;
}

/** @emoji 🎨 6-digit hex for an sRGB8888 tuple's RGB channels (alpha is shown separately — swatch labels need both). */
export function rgba8ToHex([r, g, b]: Rgba8): string {
  const channel = (n: number) => n.toString(16).padStart(2, "0");
  return `#${channel(r)}${channel(g)}${channel(b)}`;
}
//#endregion 🔖Color

//#region 🔖Layout
/** @emoji 📄 Shared page chrome for a docs-style gallery story (fullscreen layout, scrollable, theme-aware text color). */
export const galleryPageStyle: CSSProperties = {
  fontFamily: "system-ui, -apple-system, sans-serif",
  padding: 24,
  display: "flex",
  flexDirection: "column",
  gap: 32,
  color: "inherit",
  boxSizing: "border-box",
  minHeight: "100%",
  overflow: "auto",
};

export const galleryCardStyle: CSSProperties = {
  display: "flex",
  flexDirection: "column",
  gap: 16,
  padding: 16,
  borderRadius: 8,
  border: "1px solid rgba(128, 128, 128, 0.3)",
};
//#endregion 🔖Layout

//#region 🔖SwatchGrid
/** @emoji 🧩 One resolved paint name → sRGB8888 pair, as produced by `Object.entries(resolveThemeAppearancePalettes(theme, appearance)[group])`. */
export type SwatchEntry = readonly [string, Rgba8];

/** @emoji 🟪 A titled grid of color swatches — the shared primitive for every `ThemePaletteGroup` rendered across the `styling` scope's stories. */
export function SwatchGrid({ title, entries }: { readonly title: string; readonly entries: readonly SwatchEntry[] }): ReactElement {
  return (
    <section style={{ display: "flex", flexDirection: "column", gap: 8 }}>
      <h4 style={{ margin: 0, fontSize: 12, opacity: 0.65, textTransform: "uppercase", letterSpacing: "0.06em" }}>
        {title} <span style={{ opacity: 0.6 }}>({entries.length})</span>
      </h4>
      {entries.length === 0 ? (
        <div style={{ fontSize: 12, opacity: 0.5, fontStyle: "italic" }}>no paints in this group</div>
      ) : (
        <div style={{ display: "grid", gridTemplateColumns: "repeat(auto-fill, minmax(168px, 1fr))", gap: 8 }}>
          {entries.map(([name, rgba]) => (
            <div key={name} style={{ display: "flex", alignItems: "center", gap: 8, border: "1px solid rgba(128, 128, 128, 0.25)", borderRadius: 6, padding: 6, minWidth: 0 }}>
              <div
                style={{
                  width: 28,
                  height: 28,
                  borderRadius: 4,
                  flexShrink: 0,
                  background: `repeating-conic-gradient(#8884 0% 25%, transparent 0% 50%) 50% / 10px 10px, ${rgba8ToCss(rgba)}`,
                  backgroundBlendMode: "normal",
                  border: "1px solid rgba(128, 128, 128, 0.4)",
                }}
              />
              <div style={{ display: "flex", flexDirection: "column", minWidth: 0 }}>
                <span style={{ fontSize: 12, fontFamily: "monospace", overflow: "hidden", textOverflow: "ellipsis", whiteSpace: "nowrap" }}>{name}</span>
                <span style={{ fontSize: 11, opacity: 0.6, fontFamily: "monospace" }}>
                  {rgba8ToHex(rgba)} · a{rgba[3]}
                </span>
              </div>
            </div>
          ))}
        </div>
      )}
    </section>
  );
}
//#endregion 🔖SwatchGrid

//#region 🔖DataTable
/** @emoji 📊 A minimal readable table for a generated-tokens record — used by `Tokens.stories.tsx` for colors/spacing/radii/strokes/opacities. */
export function DataTable({ columns, rows }: { readonly columns: readonly string[]; readonly rows: readonly (readonly ReactNode[])[] }): ReactElement {
  return (
    <div style={{ overflowX: "auto" }}>
      <table style={{ borderCollapse: "collapse", fontSize: 12, width: "100%" }}>
        <thead>
          <tr>
            {columns.map((col) => (
              <th key={col} style={{ textAlign: "left", padding: "4px 10px", borderBottom: "1px solid rgba(128, 128, 128, 0.4)", opacity: 0.7, fontWeight: 600 }}>
                {col}
              </th>
            ))}
          </tr>
        </thead>
        <tbody>
          {rows.map((row, rowIndex) => (
            // biome-ignore lint: rows are a stable generated snapshot, index keys are fine
            <tr key={rowIndex}>
              {row.map((cell, cellIndex) => (
                // biome-ignore lint: see above
                <td key={cellIndex} style={{ padding: "4px 10px", borderBottom: "1px solid rgba(128, 128, 128, 0.15)", fontFamily: cellIndex === 0 ? "monospace" : undefined, whiteSpace: "nowrap" }}>
                  {cell}
                </td>
              ))}
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
//#endregion 🔖DataTable
