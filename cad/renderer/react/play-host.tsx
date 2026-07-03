// #region 🧲Header
/** @emoji 🛝 Playground play host for Cad — loaded only via `./play` subpath. */
// #endregion 🧲Header

import type { ReactElement } from "react";
import { type Playground, type PlaygroundChromeBoot, bootPlayground, mountPlaygroundApp } from "@semio-tech/framework-playground-renderer-react";

let cadPlayChromeRegistered = false;

function registerCadPlayPlaygroundHosts(): void {
  if (cadPlayChromeRegistered) return;
  cadPlayChromeRegistered = true;
  registerCadPlaySurfaceHosts();
}

function CadPlayPlaygroundChrome({ playground }: { readonly playground: Playground }): ReactElement {
  return <CadPlayRoot runtime={playground.runtime} />;
}

export function mountCadPlayChrome(playground: Playground, rootId = "root"): void {
  mountPlaygroundApp(<CadPlayPlaygroundChrome playground={playground} />, rootId);
}

const cadPlayChromeBoot: PlaygroundChromeBoot = {
  registerHosts: registerCadPlayPlaygroundHosts,
  mount: mountCadPlayChrome,
};

export function bootCadPlay(playground: Playground, rootId = "root"): void {
  bootPlayground(playground, cadPlayChromeBoot, rootId);
}
//#endregion 🔖CadPlayHost