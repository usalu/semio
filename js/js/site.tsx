import { Sketchpad } from "@semio/js";
import { createRoot } from "react-dom/client";
import "./globals.css";

// render the app
// NOTE: StrictMode disabled temporarily to avoid double rendering during performance testing
createRoot(document.getElementById("root")!).render(
  <div className="h-screen w-screen">
    <Sketchpad />
  </div>,
);
