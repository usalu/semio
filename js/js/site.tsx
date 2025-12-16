import { Sketchpad } from "@semio/js";
import { createRoot } from "react-dom/client";
import "./globals.css";



createRoot(document.getElementById("root")!).render(
  <div className="h-screen w-screen">
    <Sketchpad />
  </div>,
);
