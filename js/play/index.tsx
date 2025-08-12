import { Sketchpad } from "@semio/js";
import { FC } from "react";
import { createRoot } from "react-dom/client";
import { HashRouter, Route, Routes } from "react-router";
import "./globals.css";

const SketchpadWrapper: FC = () => {
  return (
    <div className="h-screen w-screen">
      <Sketchpad userId="play" />
    </div>
  );
};

createRoot(document.getElementById("root")!).render(
  <HashRouter>
    <Routes>
      <Route path="/" element={<SketchpadWrapper />} />
    </Routes>
  </HashRouter>,
);
