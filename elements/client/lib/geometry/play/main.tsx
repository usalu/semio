// #region 🧲Header
// 💻 elements/client/lib/geometry/play/main.tsx — Vite entry: mounts geometry play {@link WorkbenchView} (React-only surface).
// #endregion 🧲Header

import { getLevelBgClass, LevelProvider, mountReactApp, WorkbenchView } from "@elements/ui";

//#region 🔖Mount
import "./globals.css";
import { bootstrapGeometryPlayWorkbench } from "./react.tsx";

void bootstrapGeometryPlayWorkbench().then((workbench) => {
	mountReactApp(
		<LevelProvider>
			<WorkbenchView workbench={workbench} className={getLevelBgClass(0)} />
		</LevelProvider>,
	);
});
//#endregion 🔖Mount
