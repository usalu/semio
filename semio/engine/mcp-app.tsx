// #region 🧲Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// Specs: MCP App entry point. Exclusively uses @semio/ui components.
// Summary: MCP App entry mounts the appropriate viewer (kit, design, scene, diagram) from @semio/ui per #root data-mcp-viewer.

// #endregion 🧲Header

import { mountMcpDesignViewer, mountMcpDiagramViewer, mountMcpKitViewer, mountMcpSceneViewer } from "@semio/ui";
import "@semio/ui/globals.css";
import { createRoot } from "react-dom/client";

const root = document.getElementById("root");
if (!root) throw new Error("Missing #root element");
const mode = root.getAttribute("data-mcp-viewer") ?? "design";
try {
  if (mode === "kit") {
    mountMcpKitViewer(createRoot);
  } else if (mode === "scene") {
    mountMcpSceneViewer(createRoot);
  } else if (mode === "diagram") {
    mountMcpDiagramViewer(createRoot);
  } else {
    mountMcpDesignViewer(createRoot);
  }
} catch (e) {
  const msg = e instanceof Error ? e.message : String(e);
  root.innerHTML = `<div style="padding:16px; font-family: var(--font-sans, ui-sans-serif, system-ui, sans-serif); color:var(--destructive-foreground, #f87171); background:var(--base, #1e1e1e); min-height:100dvh; display:flex; align-items:center; justify-content:center;">
    <div style="max-width:720px; text-align:center;">
      <div style="font-weight:700; margin-bottom:8px;">semio MCP viewer crashed (mode: ${mode})</div>
      <div style="white-space:pre-wrap; opacity:0.95;">${msg}</div>
    </div>
  </div>`;
}
