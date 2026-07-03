// #region 🧲Header
/** @emoji 🛝 CAD app renderer contribution — loaded only via `./play` subpath. */
// #endregion 🧲Header

import type { AppRendererContribution, PlaygroundMountProps } from "@semio-tech/framework-platform-core";
import type { Platform } from "@semio-tech/framework-playground-renderer-react";
import { CadPlayRoot } from "./index.tsx";
import { cadPlaySceneSurfaceIdForPane, cadPlayWindowBodies } from "@semio-tech/cad-js-renderer-core";
import {
  CadPlaySurfaceHost,
  CadPlayCatalogPanelDefinition,
  CadPlayDetailsPanelDefinition,
  CadPlayHierarchyPanelDefinition,
} from "./index.tsx";

const CAD_PLAY_PANES = ["shape", "building", "energy", "structure-classic"] as const;

/** @emoji 🛝 CAD app renderer for playground and OS shells. */
export const cadAppRenderer: AppRendererContribution = {
  windowBodies: cadPlayWindowBodies,
  surfaceHosts: Object.fromEntries(CAD_PLAY_PANES.map((pane) => [cadPlaySceneSurfaceIdForPane(pane), CadPlaySurfaceHost])),
  panelTabs: {
    workbench: [new CadPlayHierarchyPanelDefinition(), new CadPlayCatalogPanelDefinition()],
    details: [new CadPlayDetailsPanelDefinition()],
  },
  mountChrome: ({ runtime }) => <CadPlayRoot runtime={runtime as Platform} />,
};
