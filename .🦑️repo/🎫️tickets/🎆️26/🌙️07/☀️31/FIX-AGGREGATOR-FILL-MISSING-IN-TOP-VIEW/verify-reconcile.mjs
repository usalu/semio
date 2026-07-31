/**
 * Standalone unit check mirroring reconcileCommittedRevealCutoffs (renderer index.tsx).
 * Full framework-renderer-react vitest cannot load here (missing surface paint wasm pkgs).
 */
function createRevealCutoffStore() {
  const values = new Map();
  return {
    get: (groupId) => values.get(groupId),
    set: (groupId, value) => values.set(groupId, value),
  };
}

function reconcileCommittedRevealCutoffs(store, committedRef, revealCutoffs) {
  for (const [groupId, value] of Object.entries(revealCutoffs)) {
    if (committedRef.current[groupId] === value) continue;
    committedRef.current = { ...committedRef.current, [groupId]: value };
    store.set(groupId, value);
  }
}

const GROUP = "puzzle3d-fill";
const store = createRevealCutoffStore();
const committedRef = { current: {} };

reconcileCommittedRevealCutoffs(store, committedRef, { [GROUP]: 0 });
if (store.get(GROUP) !== 0) throw new Error("expected committed 0");

store.set(GROUP, 17);
reconcileCommittedRevealCutoffs(store, committedRef, { [GROUP]: 0 });
if (store.get(GROUP) !== 17) throw new Error("fillBuildTick must not clobber live drag");

reconcileCommittedRevealCutoffs(store, committedRef, { [GROUP]: 17 });
if (store.get(GROUP) !== 17) throw new Error("commit sync failed");

console.log("ok: reconcileCommittedRevealCutoffs");
