import { Sketchpad } from "@semio/js";
import React from "react";
import { createRoot } from "react-dom/client";
import "./globals.css";
import "./register-apps";

// render the app
createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <div className="h-screen w-screen">
      <Sketchpad embedded />
    </div>
  </React.StrictMode>,
);
