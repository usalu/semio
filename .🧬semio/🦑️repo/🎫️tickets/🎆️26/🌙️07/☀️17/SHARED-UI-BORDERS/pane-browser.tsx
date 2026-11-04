import * as React from "react";
import { createRoot, type Root } from "react-dom/client";
import { Mode, uiDataLabel } from "/🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";
import { Pane } from "/🧰️framework/🔨️modules/🖱️ui/🧱️elements/📌️WindowPane/🟦️.tsx";
import { createShellScope, ShellScopeProvider } from "/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🐚️ShellScope/🟦️.tsx";
import { createMemoryStoragePort } from "/🧰️framework/🔨️modules/🖱️ui/🔨️modules/🗄️storage-port/🟦️.ts";

let root: Root;
export function mount({ direction, folded, anchor }) {
  const element = document.getElementById("pane-probe") ?? document.body.appendChild(Object.assign(document.createElement("div"), { id: "pane-probe" }));
  element.style.cssText = "position:fixed;inset:0;background:white;z-index:99999";
  root ??= createRoot(element);
  const scope = createShellScope({ storage: createMemoryStoragePort(), initialLocale: "en" });
  root.render(<ShellScopeProvider scope={scope}><main className="semio-scope" dir={direction} ref={node => { scope.rootRef.current = node; }} style={{ width: 800, height: 500 }}>
    <Mode windows={[{ id: "pane-window", iconId: "app-window", title: uiDataLabel("Window"), actionPane: <button>Action</button>, actionsFolded: folded, measures: <button>Option</button>, measuresFolded: folded, utilityBar: <button>Utility</button>, utilityBarFolded: folded, children: <Pane id="contributed-pane" anchor={anchor} icon="app-window" label="Contributed" folded={folded}><button>Contributed content</button></Pane> }]} activeWindowId="pane-window" />
  </main></ShellScopeProvider>);
}
