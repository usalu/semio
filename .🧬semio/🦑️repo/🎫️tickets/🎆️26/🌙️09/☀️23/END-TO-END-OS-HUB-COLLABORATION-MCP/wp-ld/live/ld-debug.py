#!/usr/bin/env python3
"""🔎️ Temporary `[DEBUG] ld` timing lines for the item-3 live repro (staged submit held during a cut); `--reverse` removes them.
usage: ld-debug.py [--reverse]"""
import pathlib
import sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules")
WORKER = ROOT / "🏪️store/👷️worker/🟦️.ts"
MAILBOX = ROOT / "🔌️plugin/🌐️browser-bundle/🎯️action-handoff/📮️requests/🟦️.ts"
EDITS = [
    (WORKER, """  private enqueueTurn<T>(work: () => Promise<T>, allowClosing = false): Promise<T> {
    const operation = this.turnTail.then(async () => {
      if (this.closed && !allowClosing) throw new Error("document browser actor: closed turn lane");
      return work();
    });""", """  private enqueueTurn<T>(work: () => Promise<T>, allowClosing = false): Promise<T> {
    const ldQueued = Date.now(), ldStack = (new Error().stack ?? "").split("\\n").slice(2, 4).map((line) => line.trim()).join(" < ");
    console.warn(`[DEBUG] ld turn-queued t=${ldQueued} from=${ldStack}`);
    const operation = this.turnTail.then(async () => {
      console.warn(`[DEBUG] ld turn-start waited=${Date.now() - ldQueued} t=${Date.now()} from=${ldStack}`);
      if (this.closed && !allowClosing) throw new Error("document browser actor: closed turn lane");
      return work();
    });
    void operation.then(() => console.warn(`[DEBUG] ld turn-end ran=${Date.now() - ldQueued} t=${Date.now()} from=${ldStack}`), () => console.warn(`[DEBUG] ld turn-fail t=${Date.now()} from=${ldStack}`));"""),
    (WORKER, """    const request = parseBrowserActorActionRequestV1(raw);
    let invoked = false;
    try {
      await this.awaitUiQuiescence();""", """    const request = parseBrowserActorActionRequestV1(raw);
    let invoked = false;
    const ldAt = Date.now();
    console.warn(`[DEBUG] ld action-in seq=${request.actionSequence} kind=${request.payload.kind} suspended=${this.suspended} patch=${this.pendingUiPatch !== null} refresh=${this.viewRefresh !== null} t=${ldAt}`);
    try {
      await this.awaitUiQuiescence();
      console.warn(`[DEBUG] ld action-quiescent seq=${request.actionSequence} waited=${Date.now() - ldAt} t=${Date.now()}`);"""),
    (WORKER, """  refreshHostView(): void {
    if (this.closed || !this.activation || this.viewRefresh) return;""", """  refreshHostView(): void {
    console.warn(`[DEBUG] ld view-refresh-request busy=${this.viewRefresh !== null} suspended=${this.suspended} t=${Date.now()}`);
    if (this.closed || !this.activation || this.viewRefresh) return;"""),
    (MAILBOX, """      this.inFlight = next;
      next.timer = setTimeout(() => this.expire(next), this.timeoutMs);""", """      this.inFlight = next;
      console.warn(`[DEBUG] ld mailbox-send seq=${next.request.actionSequence} waiting=${this.queue.length} t=${Date.now()}`);
      next.timer = setTimeout(() => this.expire(next), this.timeoutMs);"""),
    (MAILBOX, """    return new Promise<BrowserActorActionResultV1>((resolve, reject) => {
      this.queue.push({ request, resolve, reject, timer: null });""", """    return new Promise<BrowserActorActionResultV1>((resolve, reject) => {
      console.warn(`[DEBUG] ld mailbox-queue seq=${request.actionSequence} inFlight=${this.inFlight?.request.actionSequence ?? "-"} waiting=${this.queue.length} t=${Date.now()}`);
      this.queue.push({ request, resolve, reject, timer: null });"""),
    (WORKER, """    socket.binaryType = "arraybuffer";
    state.socket = socket;
    let sustainedHealthTimer""", """    socket.binaryType = "arraybuffer";
    const ldClose = socket.close.bind(socket);
    socket.close = (code?: number, reason?: string) => {
      console.warn(`[DEBUG] ld socket-close code=${code ?? "-"} reason=${reason ?? "-"} t=${Date.now()} from=${(new Error().stack ?? "").split("\\n").slice(2, 6).map((line) => line.trim().replace(/^at /u, "").replace(/\\(http[^)]*\\/([^/)]+)\\)/gu, "($1)")).join(" < ")}`);
      ldClose(code, reason);
    };
    state.socket = socket;
    let sustainedHealthTimer"""),
    (WORKER, """async function requireArtifactRebootstrap(state: ArtifactState): Promise<void> {
  state.remoteFoldedOverLocal = false;""", """async function requireArtifactRebootstrap(state: ArtifactState): Promise<void> {
  console.warn(`[DEBUG] ld rebootstrap-required folded=${state.remoteFoldedOverLocal} pending=${state.pendingMutations.length} t=${Date.now()} from=${(new Error().stack ?? "").split("\\n").slice(2, 4).map((line) => line.trim()).join(" < ").slice(0, 300)}`);
  state.remoteFoldedOverLocal = false;"""),
    (WORKER, """function rejectArtifactBootstrap(state: ArtifactState, error: unknown, owner = state.artifactBootstrapOwner): void {""", """function rejectArtifactBootstrap(state: ArtifactState, error: unknown, owner = state.artifactBootstrapOwner): void {
  console.warn(`[DEBUG] ld bootstrap-rejected error=${error instanceof Error ? error.message : String(error)} t=${Date.now()}`);"""),
]


def main() -> int:
    reverse = "--reverse" in sys.argv
    texts = {path: path.read_text(encoding="utf-8") for path in {edit[0] for edit in EDITS}}
    for path, before, after in EDITS:
        find, put = (after, before) if reverse else (before, after)
        if texts[path].count(find) != 1:
            print(f"anchor x{texts[path].count(find)} in {path.name}: {find[:60]!r}", file=sys.stderr)
            return 1
        texts[path] = texts[path].replace(find, put, 1)
    for path, text in texts.items():
        path.write_text(text, encoding="utf-8")
    print("reverse ok" if reverse else "debug ok")
    return 0


if __name__ == "__main__":
    sys.exit(main())
