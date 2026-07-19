// #region 🧲Header
// 💻 .storybook/stories/styling/ThemeRoundtrip.stories.tsx
// Specs: Interactive `parseUiTheme` ⇄ `serializeUiTheme` round-trip demo — edit theme JSON in a textarea, see it parsed, resolved into swatches, and re-serialized.
// Summary: `parseUiTheme` resolves every paint once (surfacing unknown token refs immediately per its docstring), so the story's `try/catch` around `JSON.parse` + `parseUiTheme` doubles as a live validator; `serializeUiTheme` re-renders canonical JSON so a dev can confirm the round-trip is lossless (`JSON.parse(serializeUiTheme(parseUiTheme(x))) ≍ x`, per the `theme.ts` vitest suite this mirrors).
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

import type { Meta, StoryObj } from "@storybook/react";
import { useMemo, useState, type ReactElement } from "react";

import { parseUiTheme, resolveThemeAppearancePalettes, semioTheme, serializeUiTheme, builtinUiThemes, type ThemeAppearanceName, type ThemePaletteGroup, type UiTheme } from "@semio-tech/ui-styling";
import { galleryCardStyle, galleryPageStyle, SwatchGrid } from "../../styling/index.tsx";

//#region 🔖Fixtures
const PALETTE_GROUPS: readonly ThemePaletteGroup[] = ["board", "map", "canvas", "chrome"];
const APPEARANCES: readonly ThemeAppearanceName[] = ["light", "dark"];

/** @emoji 💥 `semioTheme()` with one board paint pointed at a token that doesn't exist in `colors` — demonstrates `parseUiTheme` throwing loudly instead of silently rendering a broken color. */
function brokenThemeJson(): string {
  const theme = structuredClone(semioTheme()) as UiTheme;
  const broken: UiTheme = { ...theme, appearances: { ...theme.appearances, light: { ...theme.appearances.light, board: { ...theme.appearances.light.board, labelFill: { token: "not-a-real-token" } } } } };
  return JSON.stringify(broken, null, 2);
}
//#endregion 🔖Fixtures

//#region 🔖ParseResult
type ParseResult = { readonly theme: UiTheme; readonly error?: undefined } | { readonly theme?: undefined; readonly error: string };

/** @emoji 🔎 `JSON.parse` + `parseUiTheme` in one guarded step — either yields a validated `UiTheme` or the exact message `parseUiTheme` throws (missing palette group, unknown token ref, wrong field type, …). */
function tryParseThemeJson(text: string): ParseResult {
  try {
    return { theme: parseUiTheme(JSON.parse(text)) };
  } catch (error) {
    return { error: error instanceof Error ? error.message : String(error) };
  }
}
//#endregion 🔖ParseResult

//#region 🔖RoundtripHost
function ThemeRoundtripHost({ initialJson }: { readonly initialJson: string }): ReactElement {
  const [text, setText] = useState(initialJson);
  const result = useMemo(() => tryParseThemeJson(text), [text]);
  const serialized = result.theme ? serializeUiTheme(result.theme) : undefined;
  const roundtripStatus = useMemo(() => {
    if (!serialized) return undefined;
    try {
      return JSON.stringify(JSON.parse(text)) === JSON.stringify(JSON.parse(serialized)) ? "matches" : "reformatted";
    } catch {
      return undefined;
    }
  }, [text, serialized]);

  return (
    <div style={{ ...galleryPageStyle, flexDirection: "row", flexWrap: "wrap", alignItems: "flex-start" }}>
      <div style={{ flex: "1 1 380px", minWidth: 320, display: "flex", flexDirection: "column", gap: 8 }}>
        <h3 style={{ margin: 0 }}>theme JSON (edit me)</h3>
        <textarea
          data-testid="theme-roundtrip-input"
          value={text}
          onChange={(event) => setText(event.target.value)}
          spellCheck={false}
          style={{ fontFamily: "monospace", fontSize: 11, minHeight: 480, resize: "vertical", padding: 8, borderRadius: 6, border: "1px solid rgba(128, 128, 128, 0.4)", background: "transparent", color: "inherit" }}
        />
        {result.error ? (
          <div data-testid="theme-roundtrip-error" style={{ fontSize: 12, color: "#a60009", fontFamily: "monospace", whiteSpace: "pre-wrap" }}>
            parseUiTheme threw: {result.error}
          </div>
        ) : (
          <div data-testid="theme-roundtrip-error" style={{ fontSize: 12, opacity: 0.6 }}>
            parses cleanly
          </div>
        )}
      </div>

      <div style={{ flex: "1 1 380px", minWidth: 320, display: "flex", flexDirection: "column", gap: 16 }}>
        <h3 style={{ margin: 0 }}>parseUiTheme(json) → swatch preview</h3>
        {result.theme ? (
          <div style={{ display: "grid", gridTemplateColumns: "repeat(auto-fit, minmax(280px, 1fr))", gap: 12 }}>
            {APPEARANCES.map((appearance) => {
              const palettes = resolveThemeAppearancePalettes(result.theme!, appearance);
              return (
                <div key={appearance} style={galleryCardStyle}>
                  <h4 style={{ margin: 0, textTransform: "capitalize" }}>{appearance}</h4>
                  {PALETTE_GROUPS.map((group) => (
                    <SwatchGrid key={group} title={group} entries={Object.entries(palettes[group])} />
                  ))}
                </div>
              );
            })}
          </div>
        ) : (
          <div style={{ fontSize: 12, opacity: 0.6, fontStyle: "italic" }}>fix the JSON error on the left to see a preview</div>
        )}

        <h3 style={{ margin: 0 }}>serializeUiTheme(parsed) → round-trip</h3>
        <textarea
          data-testid="theme-roundtrip-output"
          readOnly
          value={serialized ?? ""}
          style={{ fontFamily: "monospace", fontSize: 11, minHeight: 200, padding: 8, borderRadius: 6, border: "1px solid rgba(128, 128, 128, 0.25)", background: "rgba(128, 128, 128, 0.06)", color: "inherit" }}
        />
        {roundtripStatus && (
          <div data-testid="theme-roundtrip-status" style={{ fontSize: 12, color: roundtripStatus === "matches" ? "#7eb77f" : "#fa9500" }}>
            {roundtripStatus === "matches" ? "round-trip is byte-identical after JSON normalization" : "round-trip re-parses to an equivalent object (whitespace/key-order only diff)"}
          </div>
        )}
      </div>
    </div>
  );
}
//#endregion 🔖RoundtripHost

const meta = {
  title: "🎨styling/ThemeRoundtrip",
  component: ThemeRoundtripHost,
  parameters: {
    layout: "fullscreen",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof ThemeRoundtripHost>;

export default meta;

type Story = StoryObj<typeof meta>;

//#region 🔖Stories
export const SemioTheme: Story = {
  args: {
    initialJson: serializeUiTheme(semioTheme()),
  },
};

export const MonoTheme: Story = {
  args: {
    initialJson: serializeUiTheme(builtinUiThemes().find((theme) => theme.id === "mono") ?? semioTheme()),
  },
};

export const BrokenTokenReference: Story = {
  args: {
    initialJson: brokenThemeJson(),
  },
};
//#endregion 🔖Stories
