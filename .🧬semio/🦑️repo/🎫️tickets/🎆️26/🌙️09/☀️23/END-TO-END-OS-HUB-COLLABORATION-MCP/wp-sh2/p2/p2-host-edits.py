#!/usr/bin/env python3
"""🐚️ SH2 P2 host edits, re-appliable onto any current content of the two shells (peers edit them daily): the React
ShellHost feeds Space surfaces bounded directory batches and no longer relays `touchArtifact` through a background index
session; the wgpu shell drops its `touchArtifact` relay. `python3 p2-host-edits.py <root>` rewrites `<root>`'s two files
from the LIVE tree's content."""
import os, sys

REPO = "/Users/ueli/Documents/semio"
ROOT = sys.argv[1]
SHELLHOST = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx"
WGPU = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs"
WGPU_TEST = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-identity-directory-presence/🦀️.rs"


class Text:
    def __init__(self, relative):
        self.relative, self.text = relative, open(os.path.join(REPO, relative), encoding="utf-8").read()

    def swap(self, old, new, count=1):
        assert self.text.count(old) == count, (self.relative, self.text.count(old), old[:120])
        self.text = self.text.replace(old, new)

    def cut(self, start, end_marker, keep_end=False, lstrip=False):
        s = self.text.index(start)
        e = self.text.index(end_marker, s)
        e = e if keep_end else e + len(end_marker)
        rest = self.text[e:]
        self.text = self.text[:s] + (rest.lstrip("\n") if lstrip else rest)

    def write(self):
        target = os.path.join(ROOT, self.relative)
        os.makedirs(os.path.dirname(target), exist_ok=True)
        open(target, "w", encoding="utf-8").write(self.text)


host = Text(SHELLHOST)
host.swap('import { SpaceDirectoryHistoryV1 } from "./📇️space-directory/🟦️.ts";', 'import { SpaceDirectoryHistoryV1, spaceDirectoryBatchesV1 } from "./📇️space-directory/🟦️.ts";')
host.swap("import { admitDocumentOpeningV1, BackgroundDocumentSessionsV1, browserDocumentMountIsCurrentV1,", "import { admitDocumentOpeningV1, browserDocumentMountIsCurrentV1,")
host.swap("const HUB_CATALOG_REFRESH_MS = 60_000;\n", "const HUB_CATALOG_REFRESH_MS = 60_000;\n/** 📇️ How long a Space surface waits before it is fed again after a retryable refusal of a directory batch. */\nconst SPACE_DIRECTORY_REFEED_MS = 1_000;\n")
host.swap("          // landed (see `touchSpaceIndexArtifact`'s call site).", "          // landed (see `requestHubCheckIn`'s call site).")
host.cut("  /** 📇️ Set by the `DirectoryLane` region below once it's defined", "  const dispatchDirectoryEventsRef = useRef<(events: readonly DirectoryEvent[]) => void>(() => {});\n")
host.swap("  /** ⚖️ Same ref-forwarding idiom as {@link dispatchDirectoryEventsRef} — `ensureBackboneWorker`'s", "  /** ⚖️ Same ref-forwarding idiom as `onActionRef` — `ensureBackboneWorker`'s")
host.swap("   * idiom `onActionRef`/`dispatchDirectoryEventsRef` already use: assigned as a plain statement right", "   * idiom `onActionRef` already uses: assigned as a plain statement right")
host.cut("  type BackgroundSpaceIndexSession = {", "  const backgroundSpaceIndexSessionsRef = useRef(new BackgroundDocumentSessionsV1<BackgroundSpaceIndexSession>());\n")
host.swap("""  const spaceDirectoryFoldPendingRef = useRef(false);
  const foldSpaceDirectoryRef = useRef<() => void>(() => {});""", """  const spaceDirectoryFoldPendingRef = useRef(false);
  const foldSpaceDirectoryRef = useRef<() => void>(() => {});
  /** 📇️ What each mounted Space surface has been fed, by runtime key: its app instance, the frontier of the last applied
   * batch, and whether a feed is running (a change meanwhile marks `again`, so the running feed sends it next). */
  const spaceDirectoryFeedsRef = useRef<Map<string, { readonly instanceId: number; throughSeqInclusive: number; feeding: boolean; again: boolean }>>(new Map());""")
host.swap("""        // relays the same events for every OTHER client; a duplicate here is harmless — the Space
        // index fold re-derives the same config snapshot from the same batch. Home is never folded:
        // its sealed page lane wakes on the broadcast instead (`dispatchDirectoryEventBatch`).""",
          """        // relays the same events for every OTHER client; a duplicate here is harmless — the history
        // keeps each `seq` once, so the Space index is fed each event once. Home is never folded:
        // its sealed page lane wakes on the broadcast instead (`🔖️DirectoryLane`).""")
host.swap("      retirements.push(backgroundSpaceIndexSessionsRef.current.close());\n", "")
host.swap("      void backgroundSpaceIndexSessionsRef.current.retire(entry.scope.spaceId, owned => owned.clientInstanceId === entry.clientInstanceId).catch(error => undefined);\n", "")
host.swap("""    const spaceDirectory = spaceDirectoryHistoriesRef.current.get(runtimeKey);
    if (spaceDirectory !== undefined) {
      spaceDirectoryHistoriesRef.current.delete(runtimeKey);""", """    const spaceDirectory = spaceDirectoryHistoriesRef.current.get(runtimeKey);
    spaceDirectoryFeedsRef.current.delete(runtimeKey);
    if (spaceDirectory !== undefined) {
      spaceDirectoryHistoriesRef.current.delete(runtimeKey);""")
s = host.text.index("  //#region 🔖️DirectoryLane\n")
e = host.text.index("  //#endregion 🔖️DirectoryLane\n") + len("  //#endregion 🔖️DirectoryLane\n")
host.text = host.text[:s] + '''  //#region 🔖️DirectoryLane
  /** 📇️ ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS §C6 — feeds the current Space surface (editor or
   * viewer) every event of its space it has not been fed yet, as bounded `foldDirectoryEvents` batches
   * ({@link spaceDirectoryBatchesV1}: at most one hub page each, each continuing the previous frontier; a new app instance —
   * a remount, a reload — starts from the origin). The batches go out one at a time, each awaited, so they fold in order; a
   * batch that is not applied forgets the frontier, so the next feed restarts from the origin, and a retryable refusal feeds
   * again after {@link SPACE_DIRECTORY_REFEED_MS} so a short shortage never freezes the table. Only the Space index still
   * folds raw events. Home never receives them: its projection has exactly one writer, the receipt-sealed
   * `applyDirectoryEventPage` lane, and a live event only WAKES the worker's acknowledged stream into fetching the next
   * sealed page (a raw fold there advanced the projection cursor past its receipt — ticket 26/09/18 U5 §6b). */
  foldSpaceDirectoryRef.current = () => {
    const current = sessionRef.current;
    if (current === null || !hostConfig || current.app.dialect === undefined || dialectCoordinate(current.app.dialect) !== dialectCoordinate(SPACE_INDEX_DIALECT)) return;
    for (const [runtimeKey, entry] of openDocumentSessionsRef.current) {
      if (entry.session.pluginId !== current.pluginId || entry.session.instanceId !== current.instanceId) continue;
      const history = spaceDirectoryHistoriesRef.current.get(runtimeKey);
      if (history === undefined) return;
      const known = spaceDirectoryFeedsRef.current.get(runtimeKey);
      const feed = known?.instanceId === current.instanceId ? known : { instanceId: current.instanceId, throughSeqInclusive: 0, feeding: false, again: false };
      spaceDirectoryFeedsRef.current.set(runtimeKey, feed);
      if (feed.feeding) {
        feed.again = true;
        return;
      }
      feed.feeding = true;
      void (async () => {
        try {
          do {
            feed.again = false;
            for (const batch of spaceDirectoryBatchesV1(history, feed.throughSeqInclusive)) {
              const outcome = await onActionRef.current({ controllerId: current.app.controllerId, action: "foldDirectoryEvents", args: { spaceId: batch.spaceId, afterSeqExclusive: batch.afterSeqExclusive, eventsJson: batch.eventsJson } });
              if (spaceDirectoryFeedsRef.current.get(runtimeKey) !== feed) return;
              if (outcome.kind !== "applied") {
                feed.throughSeqInclusive = 0;
                if (outcome.kind === "superseded" || outcome.retryable) window.setTimeout(() => foldSpaceDirectoryRef.current(), SPACE_DIRECTORY_REFEED_MS);
                return;
              }
              feed.throughSeqInclusive = batch.throughSeqInclusive;
            }
          } while (feed.again);
        } finally {
          feed.feeding = false;
        }
      })();
      return;
    }
  };
  //#endregion 🔖️DirectoryLane
''' + host.text[e:]
host.swap("""  // edits), explicit check-in (`#s-checkin`), checkpoint-on-close, and the post-checkpoint
  // `TouchArtifact` relay to the space index. Placed ahead of `🧰️FooterUtilityLeaves`/`🔄️SyncLeaf`""",
          """  // edits), explicit check-in (`#s-checkin`), checkpoint-on-close, and the post-checkpoint
  // hub Check In it requests. Placed ahead of `🧰️FooterUtilityLeaves`/`🔄️SyncLeaf`""")
host.cut("  useEffect(() => {\n    void backgroundSpaceIndexSessionsRef.current.retain(", "  }, [identity?.hubBaseUrl, identity?.userId, loadedPlugins]);\n")
host.cut("  /** 📌️ §C5 item 6 — after a successful checkpoint, `TouchArtifact` the space's `index` document so", "    [currentDocumentId, currentDocumentRuntimeKey, retireDocumentAttachment],\n  );\n", lstrip=True)
host.swap("""  // 📌️ §C5 item 6 continued — `TouchArtifact` fires once per checkpoint THIS shell asked for,
  // detected as the focused program's `currentCheckpointId` changing away from whatever it was the last
  // time this effect ran (never on the initial mount/session-snapshot, which isn't a checkpoint WE
  // just made).""",
          """  // 📌️ §C5 item 6 — a checkpoint THIS shell asked for requests its hub Check In once it lands, detected
  // as the focused program's `currentCheckpointId` changing away from whatever it was the last time this
  // effect ran (never on the initial mount/session-snapshot, which isn't a checkpoint WE just made). The
  // hub then publishes `artifact.checkpoint-published` to every member, and each member's Space index
  // folds it into the row's "Updated" (`📇️space-directory`) — no shell touches the index document.""")
host.swap("""    if (currentDocumentRuntimeKey !== null) requestHubCheckIn(currentDocumentRuntimeKey);
    const spaceId = openSpaceIdRef.current;
    if (spaceId && currentDocumentId && currentDocumentId !== S_SPACE_INDEX_DOCUMENT_ID) {
      void touchSpaceIndexArtifact(spaceId, currentDocumentId);
    }
  }, [historyCheckpointId, currentDocumentId, currentDocumentRuntimeKey, requestHubCheckIn, touchSpaceIndexArtifact]);""",
          """    if (currentDocumentRuntimeKey !== null) requestHubCheckIn(currentDocumentRuntimeKey);
  }, [historyCheckpointId, currentDocumentRuntimeKey, requestHubCheckIn]);""")
host.swap("""    const documentId = currentDocumentId;
    const spaceId = openSpaceIdRef.current;
    return () => {
      if (uncommittedEditCountRef.current > 0) {
        dispatchCheckpoint("auto");
        if (spaceId && documentId !== S_SPACE_INDEX_DOCUMENT_ID) void touchSpaceIndexArtifact(spaceId, documentId);
      }
    };""", """    return () => {
      if (uncommittedEditCountRef.current > 0) dispatchCheckpoint("auto");
    };""")
for gone in ("touchSpaceIndex", "backgroundSpaceIndex", "BackgroundSpaceIndex", "dispatchDirectoryEvent", "TouchArtifact", "touchArtifact"):
    assert gone not in host.text, gone
host.write()

wgpu = Text(WGPU)
wgpu.swap("""/// 📌️ ticket §C5 item 6 — the pure decision `observe_invocation_history` uses to fire `TouchArtifact`:
/// a checkpoint id change only counts when THIS shell itself dispatched the checkpoint that produced
/// it (`checkpoint_dispatched`) — a checkpoint id present from the very first snapshot (nothing
/// "landed", the session just mounted with a pre-existing one) or one made by a REMOTE peer must never
/// trigger a redundant `TouchArtifact` of our own.""",
          """/// 📌️ ticket §C5 item 6 — the pure decision `observe_invocation_history` uses to request the hub Check In
/// of a landed checkpoint: a checkpoint id change only counts when THIS shell itself dispatched the
/// checkpoint that produced it (`checkpoint_dispatched`) — a checkpoint id present from the very first
/// snapshot (nothing "landed", the session just mounted with a pre-existing one) or one made by a REMOTE
/// peer must never trigger a Check In of our own.""")
wgpu.swap("""    /// 🧾️ The most recent checkpoint id `HistoryPatch.currentCheckpointId` reported — compared against
    /// its previous value to detect "a checkpoint landed" (§C5 item 6, `TouchArtifact`).""",
          """    /// 🧾️ The most recent checkpoint id `HistoryPatch.currentCheckpointId` reported — compared against
    /// its previous value to detect "a checkpoint landed" (§C5 item 6, the hub Check In).""")
wgpu.swap("""    /// for; cleared once the resulting `history_current_checkpoint_id` change is observed and
    /// `TouchArtifact` fires — distinguishes "a checkpoint we asked for landed" from "the session just""",
          """    /// for; cleared once the resulting `history_current_checkpoint_id` change is observed and its hub
    /// Check In is requested — distinguishes "a checkpoint we asked for landed" from "the session just""")
wgpu.swap("""    /// into the running projection, updates the idle clock, and — the instant a checkpoint THIS shell
    /// itself asked for (`checkpoint_dispatched`) is observed to have actually landed — fires
    /// `TouchArtifact` on the space index (§C5 item 6). Called from both `dispatch_action` and
    /// `dispatch_command` right after their own `program.handle_action`/`handle_command` call.
    ///
    /// 🪐️ `TouchArtifact` on the space index needs the native `ArtifactHost` document-sync
    /// backbone (it opens a second actor for the `s.space` index document). The browser build
    /// links no such host, so the checkpoint still lands and the projection still folds — only
    /// the index touch is skipped, and skipping it is visible in this one place.""",
          """    /// into the running projection, updates the idle clock, and — the instant a checkpoint THIS shell
    /// itself asked for (`checkpoint_dispatched`) is observed to have actually landed — requests its hub
    /// Check In (§C5 item 6); the hub then publishes `artifact.checkpoint-published` to every member and
    /// each member's Space index folds it into the row's "Updated". Called from both `dispatch_action`
    /// and `dispatch_command` right after their own `program.handle_action`/`handle_command` call.""")
wgpu.swap("""                let Some(document_id) = self.sync_channel.as_ref().map(|channel| channel.document_id.clone()) else { return };
                self.request_hub_check_in(&space_id, &document_id);
                if document_id != S_SPACE_INDEX_DOCUMENT_ID {
                    self.touch_space_index_artifact(&space_id, &document_id).await;
                }""",
          """                let Some(document_id) = self.sync_channel.as_ref().map(|channel| channel.document_id.clone()) else { return };
                self.request_hub_check_in(&space_id, &document_id);""")
wgpu.swap("""    /// success-detection effect" note) — `dispatch_checkpoint`/`observe_invocation_history` still run
    /// the normal detection+`TouchArtifact` path; this call site just doesn't await or fail the caller""",
          """    /// success-detection effect" note) — `dispatch_checkpoint`/`observe_invocation_history` still run
    /// the normal detection+Check In path; this call site just doesn't await or fail the caller""")
wgpu.cut("    /// 📌️ ticket §C5 item 6 — `TouchArtifact` on the space's `index` document after a successful", "    /// 📌️ ticket §C5 item 3 — explicit check-in's message-prompt dialog funnel", keep_end=True)
wgpu.swap("""    /// clock, checkpoint-landed detection + `TouchArtifact`) before anything else touches `self`.""",
          """    /// clock, checkpoint-landed detection + the hub Check In) before anything else touches `self`.""")
for gone in ("TouchArtifact", "touch_space_index_artifact", "touchArtifact"):
    assert gone not in wgpu.text, gone
wgpu.write()

test = Text(WGPU_TEST)
test.swap("""/// 🧪️ Verify item: "TouchArtifact follows a checkpoint" — the pure decision
/// `observe_invocation_history` uses to fire it.
///
/// 🎯️ We asked for a checkpoint, and a NEW checkpoint id landed — fire.
///
/// 🎯️ We asked for a checkpoint, but nothing changed (same id, e.g. a no-op reply) — no fire.
///
/// 🎯️ A checkpoint id changed, but THIS shell never dispatched one (a remote peer's checkpoint,
/// or the session mounting with a pre-existing id) — must never fire our own TouchArtifact.
///
/// 🎯️ No checkpoint id at all yet — nothing landed.
#[test]
fn touch_artifact_follows_only_a_checkpoint_this_shell_itself_dispatched() {""",
          """/// 🧪️ Verify item: "a hub Check In follows a checkpoint" — the pure decision
/// `observe_invocation_history` uses to request it.
///
/// 🎯️ We asked for a checkpoint, and a NEW checkpoint id landed — request.
///
/// 🎯️ We asked for a checkpoint, but nothing changed (same id, e.g. a no-op reply) — no request.
///
/// 🎯️ A checkpoint id changed, but THIS shell never dispatched one (a remote peer's checkpoint,
/// or the session mounting with a pre-existing id) — must never request a Check In of our own.
///
/// 🎯️ No checkpoint id at all yet — nothing landed.
#[test]
fn a_hub_check_in_follows_only_a_checkpoint_this_shell_itself_dispatched() {""")
test.write()
print("host edits written into", ROOT)
