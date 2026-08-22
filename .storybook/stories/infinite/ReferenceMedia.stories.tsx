// #region 🧲️Header
// 💻️ .storybook/story/infinite/ReferenceMedia.stories.tsx
// Specs: Exercise `referenceMediaPort`/`referenceMediaKindFromUrl` (`framework/ui/js/react/index.tsx`) against the real files in `infinite/fixture/` — a sketch PNG, two floor-plan raster scans, and a PDF.
// Summary: `infinite/fixture/*` isn't served by any registered Storybook static-dir route for this scope, so each fixture is brought in via a Vite asset `import` (the PDF via the `?url` suffix, since PDFs aren't in Vite's default `assetsInclude`) — that gives `referenceMediaPort.loadReferenceTexture` a real fetchable URL with zero extra plumbing. The loaded `THREE.Texture`'s backing image/canvas is blitted onto a plain 2D `<canvas>` so the story needs no r3f `<Canvas>` at all.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

import type { Meta, StoryObj } from "@storybook/react-vite";
import { referenceMediaKindFromUrl, referenceMediaPort } from "@semio-tech/ui-react";
import { useEffect, useRef, useState, type ReactElement } from "react";

import sketchUrl from "../../../framework/product/os/module/infinite/fixture/sketch.png";
import abbauAufbauUrl from "../../../framework/product/os/module/infinite/fixture/abbau-aufbau-masterarbeit-grundriss.jpg";
import rathausAhlenUrl from "../../../framework/product/os/module/infinite/fixture/rathaus-ahlen-grundriss.png";
import sitePdfUrl from "../../../framework/product/os/module/infinite/fixture/site.pdf?url";

//#region StoryHost
type StoryReferenceMediaStatus = "loading" | "loaded" | "error";

/** @emoji 🖼️ Loads one `infinite/fixture/*` file through the real `referenceMediaPort`, then blits the resolved texture's backing image/canvas onto a plain 2D canvas for display. */
function ReferenceMediaPreview({ label, url, page }: { readonly label: string; readonly url: string; readonly page?: number }): ReactElement {
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const [status, setStatus] = useState<StoryReferenceMediaStatus>("loading");
  const [dims, setDims] = useState<{ readonly width: number; readonly height: number } | null>(null);
  const mediaKind = referenceMediaKindFromUrl(url);

  useEffect(() => {
    if (!mediaKind) {
      setStatus("error");
      return;
    }
    let cancelled = false;
    setStatus("loading");
    setDims(null);
    referenceMediaPort
      .loadReferenceTexture({ url, mediaKind, page })
      .then((loaded) => {
        if (cancelled) {
          loaded.texture.dispose();
          return;
        }
        setDims({ width: loaded.width, height: loaded.height });
        const canvas = canvasRef.current;
        const image = loaded.texture.image as CanvasImageSource | undefined;
        if (canvas && image) {
          canvas.width = loaded.width;
          canvas.height = loaded.height;
          canvas.getContext("2d")?.drawImage(image, 0, 0, loaded.width, loaded.height);
        }
        loaded.texture.dispose();
        setStatus("loaded");
      })
      .catch(() => {
        if (!cancelled) setStatus("error");
      });
    return () => {
      cancelled = true;
    };
  }, [url, mediaKind, page]);

  return (
    <div style={{ display: "flex", flexDirection: "column", gap: 4, minWidth: 0 }}>
      <div data-testid={`reference-media-status-${label}`} style={{ fontSize: 11 }}>
        {label} — kind: {mediaKind ?? "unknown"} — status: {status}
        {dims ? ` — ${dims.width}×${dims.height}` : ""}
      </div>
      <canvas ref={canvasRef} style={{ maxWidth: "100%", height: "auto", border: "1px solid #8888", background: "#0000" }} />
    </div>
  );
}

function ReferenceMediaStoryHost({ entries }: { readonly entries: readonly { readonly label: string; readonly url: string; readonly page?: number }[] }): ReactElement {
  return (
    <div className="semio-reference-media-story" style={{ display: "grid", gridTemplateColumns: "repeat(auto-fit, minmax(220px, 1fr))", gap: 12, padding: 12, width: "100%", height: "100%", overflow: "auto", boxSizing: "border-box" }}>
      {entries.map((entry) => (
        <ReferenceMediaPreview key={entry.label} label={entry.label} url={entry.url} page={entry.page} />
      ))}
    </div>
  );
}
//#endregion StoryHost

const meta = {
  title: "♾️infinite/ReferenceMedia",
  component: ReferenceMediaStoryHost,
  parameters: {
    layout: "fullscreen",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof ReferenceMediaStoryHost>;

export default meta;

type Story = StoryObj<typeof meta>;

/** 🖼️ All four real `infinite/fixture/*` files: a sketch PNG, two floor-plan scans (JPG/PNG), and page 1 of a PDF. */
export const AllFixtures: Story = {
  args: {
    entries: [
      { label: "sketch.png", url: sketchUrl },
      { label: "abbau-aufbau-masterarbeit-grundriss.jpg", url: abbauAufbauUrl },
      { label: "rathaus-ahlen-grundriss.png", url: rathausAhlenUrl },
      { label: "site.pdf", url: sitePdfUrl, page: 1 },
    ],
  },
};

/** 🖼️ Just the PDF fixture, rasterized via `referenceMediaPort`'s `pdfjs-dist` path. */
export const PdfOnly: Story = {
  args: {
    entries: [{ label: "site.pdf", url: sitePdfUrl, page: 1 }],
  },
};
