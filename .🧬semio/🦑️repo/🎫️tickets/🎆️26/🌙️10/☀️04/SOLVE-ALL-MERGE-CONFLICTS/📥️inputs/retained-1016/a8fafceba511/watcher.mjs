async function startNxWatcherAfterPreparation(prepared, active, start) {
  await prepared;
  return active() ? start() : void 0;
}
export {
  startNxWatcherAfterPreparation
};
