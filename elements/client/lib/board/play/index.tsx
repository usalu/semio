// #region 🧲Header
// 💻 elements/client/lib/board/play/index.tsx — Board play bootstrap: non-React entry that defers to the React runtime module.
// #endregion 🧲Header

import { mountAsyncReactApp } from "@elements/ui";

void mountAsyncReactApp(async () => (await import("./react.tsx")).createBoardPlayElement());