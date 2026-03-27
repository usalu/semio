// #region 🔖Header
// [👤semio📚engine💻mcpapp](repo://p/u/semio/b/l/engine/f/mcp-app.tsx)

// 2026 Ueli Saluz <ueli@semio-tech.com>

// Specs: MCP App entry point. Exclusively uses @semio/ui components.
// Summary: MCP App entry mounts McpDesignViewer or McpKitViewer from @semio/ui per #root data-mcp-viewer.

// #endregion 🔖Header

import { mountMcpDesignViewer, mountMcpKitViewer } from "@semio/ui";
import "@semio/ui/globals.css";
import { createRoot } from "react-dom/client";

const root = document.getElementById("root");
if (!root) throw new Error("Missing #root element");
const mode = root.getAttribute("data-mcp-viewer") ?? "design";
// Hard fallback: if the JS bundle crashes before React mounts, hosts will still show
// something useful instead of a blank panel.
root.innerHTML = `<div style="padding:16px; font-family: ui-sans-serif, system-ui, sans-serif; color:#fafafa; background:#1e1e1e; min-height:100dvh; display:flex; align-items:center; justify-content:center;">
  <div style="max-width:720px; text-align:center;">
    <div style="font-weight:700; margin-bottom:8px;">[DEBUG] Starting semio MCP App</div>
    <div style="opacity:0.85;">Viewer mode: ${mode}</div>
  </div>
</div>`;
try {
  if (mode === "kit") {
    mountMcpKitViewer(createRoot);
  } else {
    mountMcpDesignViewer(createRoot);
  }
} catch (e) {
  const msg = e instanceof Error ? e.message : String(e);
  root.innerHTML = `<div style="padding:16px; font-family: ui-sans-serif, system-ui, sans-serif; color:#f87171; background:#1e1e1e; min-height:100dvh; display:flex; align-items:center; justify-content:center;">
    <div style="max-width:720px; text-align:center;">
      <div style="font-weight:700; margin-bottom:8px;">[DEBUG] Kit/Design Viewer crashed</div>
      <div style="white-space:pre-wrap; opacity:0.95;">${msg}</div>
    </div>
  </div>`;
}
