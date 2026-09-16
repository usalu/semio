// #region 🧲️Header
/** @emoji ♻️ Keeps the Demonstrator's merged activation receipt current while its dev server runs.
 * Node-only on purpose: it is imported by `🏗️builder/🌐️vite/🟦️.ts`, never by the browser-shared
 * `🔨️modules/🧩️runtime/🟦️.ts`. */
// #endregion 🧲️Header

import type { Plugin } from "vite";
import { observeActivationReceipts } from "../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🟦️.ts";
import { demonstratorActivationLaneReceiptDirectories, publishDemonstratorUnionReceipt } from "../🟦️.ts";

/** @emoji 👀️ Republishes the union receipt whenever ANY lane completes a new activation.
 *
 * `semioActivationVitePlugin` watches exactly one receipt directory, and no framework lane can ever
 * produce the Demonstrator's cross-app union — so this plugin owns the fan-in: it subscribes to every
 * lane and folds them back into the one directory that plugin is pointed at. It must therefore be
 * registered BEFORE `semioActivationVitePlugin`, so the union is fresh before that plugin takes its
 * first snapshot.
 *
 * A failed merge is logged, never thrown: a lane receipt is rewritten atomically but a lane can be
 * mid-rebuild and legitimately disagree with its peers for a moment, and killing the dev server over a
 * transient disagreement would cost a full cold plugin boot. */
export function demonstratorUnionReceiptVitePlugin(options: { readonly workspace: string }): Plugin {
  const observers: { close: () => void }[] = [];
  const report = (error: unknown): void => console.error(`[demonstrator] union activation receipt refused: ${error instanceof Error ? error.message : String(error)}`);
  const dispose = (): void => { while (observers.length > 0) { try { observers.pop()!.close(); } catch { /* a watcher already torn down by its own error */ } } };
  return {
    name: "demonstrator-union-activation-receipt",
    apply: "serve",
    configureServer(server) {
      const republish = (): void => { try { publishDemonstratorUnionReceipt(options.workspace); } catch (error) { report(error); } };
      for (const directory of demonstratorActivationLaneReceiptDirectories(options.workspace)) {
        try { observers.push(observeActivationReceipts(directory, republish, report)); } catch (error) { report(error); }
      }
      server.httpServer?.once("close", dispose);
    },
    closeBundle: dispose,
  };
}
