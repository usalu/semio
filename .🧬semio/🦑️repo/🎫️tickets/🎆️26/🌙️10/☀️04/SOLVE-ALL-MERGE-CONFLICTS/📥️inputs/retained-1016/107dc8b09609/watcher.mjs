function waitForNxWatcher(watcher) {
  return new Promise((accept, reject) => {
    let pending = "";
    const started = Date.now(), progress = setInterval(() => console.log(`[nx] Waiting for source watcher and project graph (${Math.floor((Date.now() - started) / 1e3)}s); Ctrl+C cancels`), 15e3);
    const finish = (error) => {
      clearInterval(progress);
      watcher.removeListener("close", exited);
      watcher.removeListener("error", failed);
      watcher.stdout.removeListener("data", received);
      if (error) reject(error);
      else accept();
    };
    const exited = (code, signal) => finish(new Error(`Nx source watcher exited before readiness (${signal ?? code ?? "unknown status"})`));
    const failed = (error) => finish(error);
    const received = (chunk) => {
      pending = (pending + chunk.toString()).slice(-8192);
      if (pending.includes("watch process waiting...")) finish();
    };
    watcher.once("close", exited);
    watcher.once("error", failed);
    watcher.stdout.on("data", received);
  });
}
function activateWhenNxWatcherReady(watcher, prepared, active, activate, failed) {
  void Promise.all([waitForNxWatcher(watcher), prepared]).then(() => {
    if (active()) activate();
  }).catch((error) => {
    if (active()) failed(error);
  });
}
export {
  activateWhenNxWatcherReady,
  waitForNxWatcher
};
