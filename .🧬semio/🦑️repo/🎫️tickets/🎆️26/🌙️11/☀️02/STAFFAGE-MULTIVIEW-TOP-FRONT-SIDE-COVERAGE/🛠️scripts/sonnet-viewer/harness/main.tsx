import { createRoot } from "react-dom/client";
import { useState } from "react";
import type { Asset } from "@teaching/staffage";
import walkerDocument from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧫️fixtures/🚶️reference-walker/🔣️.json";
import { ArchiveProvider } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🎯️targets/⚛️react/🔨️modules/🧭️context/🟦️.tsx";
import { translator } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🎯️targets/⚛️react/🔨️modules/🌐️i18n/🟦️.ts";
import { CatalogStore } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🎯️targets/⚛️react/🔨️modules/📚️catalog/🟦️.ts";
import { ArchiveStore, memoryStorage } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🎯️targets/⚛️react/🔨️modules/💾️persistence/🟦️.ts";
import { Viewer } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🎯️targets/⚛️react/🔨️modules/🎛️viewer/🟦️.tsx";
import { DEFAULT_VIEWER, viewerFromParams, type ViewerState } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🎯️targets/⚛️react/🔨️modules/🗺️routing/🟦️.ts";

const walker = walkerDocument as unknown as Asset;
const params = new URLSearchParams(location.search);
const language = params.get("lang") === "de" ? "de" : "en";
const catalog = new CatalogStore({ manifest: async () => { throw new Error("unused"); }, category: async () => null });
const store = new ArchiveStore(memoryStorage(), ["dark-line"]);
store.dispatch({ type: "preferences-changed", patch: { motion: "reduced", language } });

function App() {
  const [viewer, setViewer] = useState<ViewerState>({ ...DEFAULT_VIEWER, ...viewerFromParams(params), selection: { kind: "clip", clip: "walk" }, timeS: Number(params.get("t") ?? 0.5) });
  (window as unknown as { viewer: ViewerState }).viewer = viewer;
  return (
    <ArchiveProvider language={language} t={translator(language)} catalog={catalog} store={store}>
      <div style={{ padding: 12, maxWidth: 1400 }}>
        <Viewer asset={walker} viewer={viewer} onChange={(patch) => setViewer((current) => ({ ...current, ...patch }))} />
      </div>
    </ArchiveProvider>
  );
}
createRoot(document.getElementById("root")!).render(<App />);
