import { loadPlaygroundApp } from "../../../../../framework/product/playground/core/js/app-registry.ts";
import { PlaygroundView } from "../../../../../framework/product/playground/renderer/react/index.tsx";
import { registerCadPlayDeclarativeBodies } from "../../../../../cad/renderer/core/js/index.ts";
import { reactHostPort } from "../../../../../ui/react/index.tsx";
import { renderToString } from "react-dom/server";

registerCadPlayDeclarativeBodies();
const app = await loadPlaygroundApp("cad");
if (!app) throw new Error("cad app missing");
const runtime = app.createPlayground().runtime;
try {
  const html = renderToString(reactHostPort.createElement(PlaygroundView, { runtime, defaultAppId: "cad-play" }));
  console.log("html length", html.length);
  console.log(html.slice(0, 500));
} catch (error) {
  console.error("render failed", error);
}
