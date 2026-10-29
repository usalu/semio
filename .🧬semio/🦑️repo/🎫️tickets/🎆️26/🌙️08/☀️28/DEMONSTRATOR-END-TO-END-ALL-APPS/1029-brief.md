# 🧭️ 1029 Fleet Brief — mit Bestand Demonstrator End To End (wave 1029)

Ticket `26/08/28/DEMONSTRATOR-END-TO-END-ALL-APPS` (folder = C = `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS`). Coordinator = main chat. NEVER open/close/reopen the ticket.

ALL rules of `C/1028-brief.md` apply (read it fully first, incl. addenda 17:15 detached-wait, 18:35 gate is policy, 19:55 free-fix). Overrides for 1029:
- Generated output goes to `C/🗑️generated/1029/<you>-*.{txt,rc}`; scripts to `C/1029-tools/<you>/`; result file `C/1029-result-<you>.md` (status line first, keep it updated as you go).
- Goal: `@semio-tech/mit-bestand-demonstrator:activate-prototype` (Bazel) green → `serve` on 6029 → `test-e2e` green; all 8 panes render. Coordinator alone runs activate/prepare/serve/test-e2e/dependencies sync|lock.
- State at start (01:58 10/29): deps-wasm GREEN; flow DSL-native migration done; Dictionary::from_value livelock fixed (fwwasm). Last 3d-puzzle wasm check (`C/🗑️generated/1028/puzzleplugin-chk78.txt`) fails ONLY in `semio-framework-plugin-instance-dispatch` (8 errors). Other component lanes have not been re-checked since ~23:00 10/28.
- No sub-agents, foreground commands only (detached check + .rc marker + bounded wait loops per Addendum 17:15). Never end your turn while your own check is queued; never pattern-kill.
- Peers (other sessions, Codex) edit the same tree concurrently: re-read before every edit, compile-atomic edits, do not revert foreign changes, no compat shims, no limit raises, no `#[allow]`.
- Final message: 5 lines + result path.
