import { default as Metabolism } from "@semio/assets/semio/kit_metabolism.json";
import { DesignEditor, extractFilesAndCreateUrls } from "@semio/js";
import { FC, useState } from "react";
import { createRoot } from "react-dom/client";
import { HashRouter, Route, Routes } from "react-router";
import "./globals.css";

const SketchpadWrapper: FC = () => {
  const [fileUrls, setFileUrls] = useState<Map<string, string>>(new Map());
  extractFilesAndCreateUrls("metabolism.zip").then((urls) => {
    setFileUrls(urls);
  });
  if (fileUrls.size === 0) return <div>Loading...</div>;
  return (
    <div className="h-screen w-screen">
      <DesignEditor initialKit={Metabolism} designId={{ name: "Nakagin Capsule Tower" }} fileUrls={fileUrls} />
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
