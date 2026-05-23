// #region 🧲Header
// 💻 elements/client/lib/board/play/index.tsx — Board play bootstrap: non-React entry that defers to the React runtime module.
// #endregion 🧲Header

import { mountReactApp } from "@elements/ui";

void import("./react.tsx").then((m) => {
	mountReactApp(m.createBoardPlayElement());
});