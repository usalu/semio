// #region 🧲Header

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details. You should have received a copy of the GNU Affero General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Entry point for the playground React app for interactive experimentation.

// #endregion 🧲Header

// #region 🛎️Entrypoint
// Play application entrypoint registering sketchpad apps and rendering the root.
// Entrypoint MUST register all app configs before rendering the Sketchpad component.

import { Sketchpad, appRegistry, designConfig, docsConfig, feedbackConfig, homeConfig, kitConfig, typeConfig } from "@semio/sketchpad";
import { InMemoryKitStore, SemioStoreKitLineHost, type SketchpadKitStoreFactory } from "@semio/react";
import { createRoot, type Root } from "react-dom/client";
import "./globals.css";

appRegistry.register(designConfig);
appRegistry.register(docsConfig);
appRegistry.register(feedbackConfig);
appRegistry.register(homeConfig);
appRegistry.register(kitConfig);
appRegistry.register(typeConfig);

const temporaryKitStoreFactory: SketchpadKitStoreFactory = (kit) => new InMemoryKitStore(kit);

const nativeStoreUrl = (import.meta as { env?: Record<string, string | undefined> }).env?.["VITE_SEMIO_STORE_URL"]?.trim();
const useNativeStoreLine =
  (import.meta as { env?: Record<string, string | undefined> }).env?.["VITE_SEMIO_NATIVE_STORE"] === "1" && nativeStoreUrl != null && nativeStoreUrl !== "";

type RootHostElement = HTMLElement & { __semioReactRoot__?: Root };

const getOrCreateDomRoot = (element: HTMLElement): Root => {
  const rootHost = element as RootHostElement;
  if (!rootHost.__semioReactRoot__) {
    rootHost.__semioReactRoot__ = createRoot(element);
  }
  return rootHost.__semioReactRoot__;
};

const rootElement = document.getElementById("root");
if (!rootElement) {
  throw new Error('Play root element "#root" was not found.');
}

getOrCreateDomRoot(rootElement).render(
  <div className="h-screen w-screen">
    {useNativeStoreLine ? (
      <SemioStoreKitLineHost baseUrl={nativeStoreUrl!} fallback={<div className="p-4 text-sm opacity-80">Connecting to semio-store…</div>}>
        <Sketchpad importKitUrls={[]} />
      </SemioStoreKitLineHost>
    ) : (
      <Sketchpad temporaryKitStoreFactory={temporaryKitStoreFactory} importKitUrls={["/metabolism.zip"]} />
    )}
  </div>,
);
// #endregion 🛎️Entrypoint
