const runBun = () => {
  throw new Error("select a runner");
};
async function prepareJavascriptDependencies(mode, workspace, signal, runner = runBun) {
  signal.throwIfAborted();
  await runner(["install", mode === "sync" ? "--frozen-lockfile" : "--lockfile-only"], workspace, signal);
  signal.throwIfAborted();
}
export {
  prepareJavascriptDependencies
};
