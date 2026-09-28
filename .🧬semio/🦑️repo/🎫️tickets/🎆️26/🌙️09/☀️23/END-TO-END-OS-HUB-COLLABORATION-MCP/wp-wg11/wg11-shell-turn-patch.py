#!/usr/bin/env python3
"""🧵️ WG11 session 14b — prepared patch for window 3 (item 3, RUST_MIN_STACK root fix): a shell turn's state machine lives on the heap.

Measured (static call graph of WG10's 14:12 debug renderer test binary, `wg11-stack-paths.py`; live gate `gate-b3-1` step 8 overflowed
libtest's 2 MiB thread, `gate-b3-2` passed only under RUST_MIN_STACK=8 MiB): `ShellState::dispatch_action`'s poll frame is 841 KB although
its future is ~55 KB — a debug build gives every awaited child future its own stack slots at every await site (no stack colouring), and
its heavy children (`apply_os_command` ~55 KB ×3, `dispatch_command` ~42 KB ×3, `apply_mutations` ~37 KB, `handle_hub_workspace_action`
~34 KB, `apply_shell_uri` ~33 KB, `handle_sync_action` ~30 KB, …) are all inline. The same children inflate `dispatch_command`,
`apply_ops_inner` and `handle_shell_hit` (2.69 MB). Step 8's stack: test 250 KB + drive 56 KB + dispatch_action 841 KB +
apply_mutations 75 KB + apply_ops_inner 152 KB + refresh_ui subtree 359 KB > 2 MiB with libtest's own frames.

Fix: the heavy shell turns return `ShellTurn<'a, R>` — the turn's body boxed at its DEFINITION (`shell_turn(async move { … })`), so every
caller's await site holds a pointer instead of the turn's whole state machine, and the recursion through `dispatch_action` is broken at
the definition (the call-site `Box::pin(self.<turn>(…))` wrappers that sidestepped E0733 go). `ShellTurn` is `Send` on native (the
runtime's `spawn_dispatch_reserved` requires `Send` futures) and not on wasm32. Law (`⚙️settings-general-layout`): a framework setting
dispatch — `dispatch_action` → `note_shell_setting_command` → `dispatch_action` for the history note — completes on a 1 MiB thread
(the Windows main-thread default) in a debug build; red before (two nested 841 KB poll frames).

Dry run by default; `--apply` writes (every anchor and every definition asserted exactly once).
"""

import difflib
import re
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
SHELL = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs"
LAWS = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/⚙️settings-general-layout/🦀️.rs"
TURNS = [
    "dispatch_action",
    "dispatch_command",
    "apply_os_command",
    "apply_mutations",
    "apply_shell_uri",
    "handle_hub_workspace_action",
    "handle_sync_action",
    "handle_checkin_action",
    "execute_staged_action",
    "handle_replay_shell_command",
    "observe_invocation_history",
    "set_extension_enabled",
]
TURN_TYPES = """/// 🧵️ One shell turn's state machine, boxed at the turn's definition (ticket 26/09/23 session 14b): a caller's await site holds this
/// pointer instead of the whole turn, which a debug build otherwise copies into its own stack slots at every await site
/// (`dispatch_action`'s poll frame was 841 KB and the live hub gate's first authored edit overflowed a 2 MiB test thread).
#[cfg(not(target_arch = "wasm32"))]
pub type ShellTurn<'a, R> = std::pin::Pin<Box<dyn std::future::Future<Output = R> + Send + 'a>>;
/// 🧵️ See the native [`ShellTurn`]; a browser turn is not `Send`.
#[cfg(target_arch = "wasm32")]
pub type ShellTurn<'a, R> = std::pin::Pin<Box<dyn std::future::Future<Output = R> + 'a>>;

/// 📦️ Moves one turn's body to the heap as a [`ShellTurn`]; `R` comes from the turn's declared output, so `?` inside infers.
#[cfg(not(target_arch = "wasm32"))]
fn shell_turn<'a, R>(turn: impl std::future::Future<Output = R> + Send + 'a) -> ShellTurn<'a, R> {
    Box::pin(turn)
}
/// 📦️ See the native [`shell_turn`].
#[cfg(target_arch = "wasm32")]
fn shell_turn<'a, R>(turn: impl std::future::Future<Output = R> + 'a) -> ShellTurn<'a, R> {
    Box::pin(turn)
}

"""
LAW = '''
/// 🧵️ LAW (ticket 26/09/23 session 14b, WG11 — live gate step 8 overflowed libtest's 2 MiB thread in debug): a shell turn's state machine
/// lives on the heap, so a framework setting dispatch — `dispatch_action` → `note_shell_setting_command` → `dispatch_action` again for the
/// history note — completes on a 1 MiB thread (the Windows main-thread default) in a debug build. Red before: `dispatch_action`'s own
/// poll frame was 841 KB, nested twice.
#[test]
fn a_framework_setting_dispatch_completes_on_a_one_mebibyte_thread() {
    let completed = std::thread::Builder::new()
        .stack_size(1024 * 1024)
        .spawn(|| {
            let mut shell = ShellState::new(Vec::new(), String::new());
            shell.session = Some(ActiveSession { plugin_id: "test".into(), instance_id: 1, app: super::command_registry_tests::test_app(Vec::new(), Vec::new()), view_state: ViewModel::default() });
            semio_framework_async::block_on(shell.dispatch_action(ActionDescriptor { controller_id: "framework".into(), action: "setAppearance".into(), args: crate::action_args_json!({ "value": "dark" }) })).map(|()| shell.appearance_id.clone())
        })
        .expect("a 1 MiB shell thread starts")
        .join();
    assert_eq!(completed.ok().and_then(Result::ok).as_deref(), Some("dark"), "the setting dispatch completes and applies without overflowing a 1 MiB stack");
}
'''
LAW_NAME = "a_framework_setting_dispatch_completes_on_a_one_mebibyte_thread"
PRE_EDITS = [
    (
        """        // 🧱️ `Box::pin` breaks a real call-graph cycle (`dispatch_action` → `handle_sync_action` →
        // `attach_sync_backbone` → `checkpoint_before_detach` → here → `dispatch_action` again) that
        // `rustc` rightly refuses to size without it (E0733) — the cycle is never actually walked at
        // runtime for a `commitCheckpoint` action (its `controller_id` is the session's own app, never
        // `"framework.sync"`), but the async-fn state machine's size is inferred statically regardless
        // of which branch executes.
""",
        "",
    ),
    (
        """    /// `Box::pin` sidesteps `dispatch_action` calling itself inside its own generated future
    /// (rustc E0733) — the recursion itself is exactly what the ticket calls for. No-ops without an
    /// active session: nothing to log a shell command against.""",
        """    /// The recursion itself is exactly what the ticket calls for; `dispatch_action` is a [`ShellTurn`], boxed at its definition.
    /// No-ops without an active session: nothing to log a shell command against.""",
    ),
]


def code_mask(source: str) -> bytearray:
    """1 = the character is Rust code (not inside a string, char literal or comment)."""
    mask = bytearray(len(source))
    i, n = 0, len(source)
    while i < n:
        c = source[i]
        if source.startswith("//", i):
            j = source.find("\n", i)
            i = n if j < 0 else j
            continue
        if source.startswith("/*", i):
            depth, j = 1, i + 2
            while j < n and depth:
                if source.startswith("/*", j):
                    depth, j = depth + 1, j + 2
                elif source.startswith("*/", j):
                    depth, j = depth - 1, j + 2
                else:
                    j += 1
            i = j
            continue
        raw = re.match(r'b?r(#*)"', source[i : i + 260]) if c in "rb" and (i == 0 or not (source[i - 1].isalnum() or source[i - 1] == "_")) else None
        if raw:
            close = '"' + raw.group(1)
            j = source.find(close, i + raw.end())
            i = n if j < 0 else j + len(close)
            continue
        if c == '"' or (c == "b" and source.startswith('b"', i) and not (source[i - 1].isalnum() or source[i - 1] == "_")):
            j = i + (2 if c == "b" else 1)
            while j < n and source[j] != '"':
                j += 2 if source[j] == "\\" else 1
            i = j + 1
            continue
        if c == "'":
            char = re.match(r"'(\\.|\\u\{[0-9a-fA-F]+\}|\\x[0-9a-fA-F]{2}|[^\\'])'", source[i : i + 12])
            if char:
                i += char.end()
                continue
        mask[i] = 1
        i += 1
    return mask


def matching(source: str, mask: bytearray, open_at: int, pair: str) -> int:
    depth = 0
    for j in range(open_at, len(source)):
        if not mask[j]:
            continue
        if source[j] == pair[0]:
            depth += 1
        elif source[j] == pair[1]:
            depth -= 1
            if depth == 0:
                return j
    sys.exit(f"unbalanced {pair} from offset {open_at}")


def reindent(body: str, mask: bytearray, start: int) -> str:
    """Indents every body line one level deeper unless the line continues a string literal or a block comment."""
    out, offset = [], start
    for index, line in enumerate(body.split("\n")):
        out.append("    " + line if index > 0 and line and mask[offset - 1] else line)
        offset += len(line) + 1
    return "\n".join(out)


def lifetime_params(params: str) -> str:
    params = params.replace("&mut self", "&'a mut self", 1) if "&mut self" in params else params.replace("&self", "&'a self", 1)
    return re.sub(r"&(?!')(mut\s+)?", lambda m: "&'a " + (m.group(1) or ""), params)


def box_turn(source: str, name: str) -> str:
    mask = code_mask(source)
    found = [m for m in re.finditer(r"(?m)^(\s*)((?:pub(?:\(crate\))? )?)async fn " + name + r"\(", source) if mask[m.start(2) if m.group(2) else m.end(1)]]
    if len(found) != 1:
        sys.exit(f"`async fn {name}(` occurs {len(found)}x")
    head = found[0]
    indent = head.group(1)
    params_open = head.end() - 1
    params_close = matching(source, mask, params_open, "()")
    body_open = next(j for j in range(params_close, len(source)) if mask[j] and source[j] == "{")
    body_close = matching(source, mask, body_open, "{}")
    between = source[params_close + 1 : body_open].strip()
    output = between[2:].strip() if between.startswith("->") else "()"
    if between and not between.startswith("->"):
        sys.exit(f"{name}: unexpected text between parameters and body: {between!r}")
    params = lifetime_params(source[params_open : params_close + 1])
    body = source[body_open + 1 : body_close]
    signature = f"{indent}{head.group(2)}fn {name}<'a>{params} -> ShellTurn<'a, {output}> {{"
    wrapped = f"\n{indent}    shell_turn(async move {{{reindent(body, mask, body_open + 1).rstrip()}\n{indent}    }})\n{indent}}}"
    return source[: head.start()] + signature + wrapped + source[body_close + 1 :]


def unwrap_call_site_boxes(source: str, name: str) -> tuple[str, int]:
    count = 0
    while True:
        mask = code_mask(source)
        at = next((m.start() for m in re.finditer(r"Box::pin\(self\." + name + r"\(", source) if mask[m.start()]), None)
        if at is None:
            return source, count
        close = matching(source, mask, at + len("Box::pin"), "()")
        source = source[:at] + source[at + len("Box::pin(") : close] + source[close + 1 :]
        count += 1


def main():
    apply = "--apply" in sys.argv
    before = SHELL.read_text(encoding="utf-8")
    if "type ShellTurn<'a, R>" in before:
        sys.exit("ShellTurn already present — landed already")
    after = before
    for old, new in PRE_EDITS:
        if after.count(old) != 1:
            sys.exit(f"anchor occurs {after.count(old)}x: {old[:90]!r}")
        after = after.replace(old, new)
    unwrapped = {}
    for name in TURNS:
        after = box_turn(after, name)
        after, unwrapped[name] = unwrap_call_site_boxes(after, name)
    mask = code_mask(after)
    impl_at = [m.start() for m in re.finditer(r"(?m)^impl ShellState \{", after) if mask[m.start()]]
    dispatch_at = after.index("pub fn dispatch_action<'a>")
    host = max(at for at in impl_at if at < dispatch_at)
    after = after[:host] + TURN_TYPES + after[host:]
    laws_before = LAWS.read_text(encoding="utf-8")
    if LAW_NAME in laws_before:
        sys.exit("law already present")
    laws_after = laws_before.rstrip("\n") + "\n" + LAW
    for path, old, new in ((SHELL, before, after), (LAWS, laws_before, laws_after)):
        sys.stdout.writelines(difflib.unified_diff(old.splitlines(True), new.splitlines(True), path.parent.name + "/" + path.name, path.parent.name + "/" + path.name + " (patched)", n=1))
    print(f"\ncall-site Box::pin unwrapped: {unwrapped}")
    if apply:
        SHELL.write_text(after, encoding="utf-8")
        LAWS.write_text(laws_after, encoding="utf-8")
    print(f"\n{'APPLIED' if apply else 'DRY RUN'}: 2 files, {len(TURNS)} turns boxed at their definition")


if __name__ == "__main__":
    main()
