// #region 🧲KitWorkerEntry
// Comlink entry for [`createKitStoreClient`]: hosts [`KitWorkerApi`] in a dedicated module worker.
// 2026 Ueli Saluz <ueli@semio-tech.com> — GNU LGPL-3.0 or later
import * as Comlink from "comlink";
import { KitWorkerApi } from "./index.ts";

Comlink.expose(new KitWorkerApi());
// #endregion 🧲KitWorkerEntry
