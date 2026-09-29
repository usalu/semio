"""🔎️ C12 TEMPORARY `[DEBUG] c12 splice` probes in the React TextEditor's splice host (live wave p33, STEP 11/13 root cause):
logs every deliver (path, host seq/base/unapplied, local), every sent splice, every whole-text fallback, every host (re)creation
and every scene receive (applied, remote, unapplied, shown). usage: python3 <this> --apply | --revert (default: dry run)"""
import sys

PATH = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/✏️TextEditor/🟦️.tsx"
HUNKS = [
    ("""          const spliceHost = spliceHostRef.current;
          if (spliceTypingRef.current && spliceHost !== null) {
            const sent = readOnlyRef.current ? null : sendTextEditorSpliceV1(spliceHost, next.text);
            if (sent !== null) {
""", """          const spliceHost = spliceHostRef.current;
          console.info("[DEBUG] c12 splice deliver", JSON.stringify({ mode: spliceTypingRef.current, host: spliceHost === null ? null : { seq: spliceHost.seq, base: spliceHost.base.slice(-60), remote: spliceHost.remote.slice(-60), unapplied: spliceHost.unapplied.map((entry) => entry.seq) }, local: next.text.slice(-60), start: next.start, end: next.end }));
          if (spliceTypingRef.current && spliceHost !== null) {
            const sent = readOnlyRef.current ? null : sendTextEditorSpliceV1(spliceHost, next.text);
            if (sent !== null) {
              console.info("[DEBUG] c12 splice sent", JSON.stringify({ seq: sent.seq, splice: sent.splice }));
"""),
    ("""          const echo = echoStateRef.current;
          const guestHasText""", """          console.info("[DEBUG] c12 splice fallback-whole-text", JSON.stringify({ mode: spliceTypingRef.current, host: spliceHostRef.current !== null, local: next.text.slice(-60) }));
          const echo = echoStateRef.current;
          const guestHasText"""),
    ("""          spliceHostRef.current = textEditorSpliceHostV1(scene.buffer, textEditorAppliedSpliceV1(scene.selectionJson));
""", """          spliceHostRef.current = textEditorSpliceHostV1(scene.buffer, textEditorAppliedSpliceV1(scene.selectionJson));
          console.info("[DEBUG] c12 splice host", JSON.stringify({ resync, buffer: scene.buffer.slice(-60), applied: textEditorAppliedSpliceV1(scene.selectionJson), local: session?.text().slice(-60) ?? null }));
"""),
    ("""          spliceHostRef.current = received.host;
""", """          console.info("[DEBUG] c12 splice receive", JSON.stringify({ applied: textEditorAppliedSpliceV1(scene.selectionJson), remote: scene.buffer.slice(-60), unapplied: spliceHostRef.current.unapplied.map((entry) => entry.seq), local: local.slice(-60), show: received.show === null ? null : received.show.text.slice(-60), seq: received.host.seq }));
          spliceHostRef.current = received.host;
"""),
]
text = open(PATH, encoding="utf-8").read()
mode = "apply" if "--apply" in sys.argv else "revert" if "--revert" in sys.argv else "dry"
plan = []
for old, new in HUNKS:
    source, target = (new, old) if mode == "revert" else (old, new)
    if target in text and source not in text:
        plan.append("present")
    elif text.count(source) == 1:
        text = text.replace(source, target)
        plan.append("hunk")
    else:
        plan.append(f"PROBLEM {text.count(source)}")
print(mode, plan)
if mode != "dry" and not any(p.startswith("PROBLEM") for p in plan):
    open(PATH, "w", encoding="utf-8").write(text)
    print("written")
