#!/usr/bin/env python3
"""🎫 H14 test-only (rule 22): the frame-deadline Ack law (`a_batch_committed_past_the_frame_deadline_is_still_acknowledged`, hold 3)
dropped its three welcome/bootstrap/subscription gate permits instead of forgetting them (3 `unused SemaphorePermit` warnings; a
dropped permit returns to its semaphore, so a later wait on that gate would pass stale). The law now forgets them like its other
gates. Region-guarded to the law's own body (a peer law repeats the same lines). Idempotent; `--dry-run` reports."""
import sys

PATH = "/Users/ueli/Documents/semio/🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs"
DRY = "--dry-run" in sys.argv
text = open(PATH, encoding="utf-8").read()
start = text.index("    fn a_batch_committed_past_the_frame_deadline_is_still_acknowledged() {")
end = text.index("\n    #[tokio::test]", start)
body = text[start:end]
states = []
for old in ('.expect("pre-Welcome deadline").expect("pre-Welcome");', '.expect("post-Welcome deadline").expect("post-Welcome");', '.expect("subscription deadline").expect("subscription");'):
    new = old[:-1] + ".forget();"
    if new in body and old not in body:
        states.append("done")
    elif body.count(old) == 1:
        body = body.replace(old, new)
        states.append("replace")
    else:
        print(f"PROBLEM: {old} found {body.count(old)}")
        sys.exit(1)
print(f"states {states}")
if DRY:
    print("dry-run clean")
elif "replace" in states:
    open(PATH, "w", encoding="utf-8").write(text[:start] + body + text[end:])
    print("applied")
else:
    print("nothing to apply")
