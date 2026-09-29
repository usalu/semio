"""⏳️ S20 window-3 prepared patch: a document-archive load yields the page between polls.

`AppChannelClient.loadDocumentArchive` re-polled with `await Promise.resolve()`: replies arrive through the instance's
outcome iterator, so a load that stays `running` (raster, measured 28 12:0x: status 1/1 `running` for > 10 min) spun a
microtask-only loop — the page's own 5 s timers stretched to ~35 s, the Tasks window barely repainted and its Cancel could
not be pressed in time. Each poll now waits for the next macrotask (`setTimeout` 0), so timers, input and rendering run
between polls; a cancelled load still re-polls until the guest reports its terminal state.

Usage: python3 s20-patch-archive-poll-yield.py [--dry-run]   (idempotent)
"""
import sys
from pathlib import Path

OS = Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts")
DRY = "--dry-run" in sys.argv

HELPER_ANCHOR = "//#endregion 🏠️LocalQueryOwnership\n\n/**\n * 📡️ Typed facade over one plugin instance's app channel"
HELPER = """//#endregion 🏠️LocalQueryOwnership

/** ⏳️ Resolves on the next macrotask — where a document-archive load waits between polls, so the page's timers, input
 * and rendering (the Tasks window's Cancel) run while the guest works; a microtask-only re-poll starved them. */
function nextArchivePollTurnV1(): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, 0));
}

/**
 * 📡️ Typed facade over one plugin instance's app channel"""
RUNNING_OLD = """      if (status.state === "pending" || status.state === "running") {
        await Promise.resolve();
        continue;
      }"""
RUNNING_NEW = """      if (status.state === "pending" || status.state === "running") {
        await nextArchivePollTurnV1();
        continue;
      }"""
CANCELLED_OLD = """      if (acknowledgeError && status.state === "cancelled") {
        await Promise.resolve();
        continue;
      }"""
CANCELLED_NEW = """      if (acknowledgeError && status.state === "cancelled") {
        await nextArchivePollTurnV1();
        continue;
      }"""



#: 🏁️ Set-level landing markers `(repo path, text)` — `None` = the set deletes that file. All present → the set is
#: landed and nothing is applied (per-hunk checks alone cannot see an insert whose text a later codemod reworded).
LANDED = [('🧰️framework/🛍️products/💻️os/🟦️.ts', 'function nextArchivePollTurnV1(): Promise<void> {')]


def landed_guard() -> bool:
    """🏁️ True when every landing marker is in the tree; a partial landing is a conflict, never a second write."""
    tree = Path("/Users/ueli/Documents/semio")
    present = [(not (tree / rel).exists()) if marker is None else ((tree / rel).exists() and marker in (tree / rel).read_text()) for rel, marker in LANDED]
    if all(present):
        print("landed: every set marker is in the tree — nothing to apply")
        return True
    if any(present):
        raise SystemExit(f"CONFLICT: set partially landed (markers {present}) — nothing written")
    return False


def main() -> None:
    if landed_guard():
        return
    text = OS.read_text()
    notes = []
    for name, old, new in [("helper", HELPER_ANCHOR, HELPER), ("running poll", RUNNING_OLD, RUNNING_NEW), ("cancelled poll", CANCELLED_OLD, CANCELLED_NEW)]:
        if new in text:
            notes.append(f"applied   {name}")
        elif text.count(old) == 1:
            text = text.replace(old, new)
            notes.append(f"apply     {name}")
        else:
            notes.append(f"CONFLICT  {name}: anchor found {text.count(old)}×")
    print("\n".join(notes))
    if any(line.startswith("CONFLICT") for line in notes):
        raise SystemExit("conflict: nothing written")
    if not DRY:
        OS.write_text(text)
    print("dry-run" if DRY else "applied")


if __name__ == "__main__":
    main()
