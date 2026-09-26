#!/usr/bin/env python3
"""WG8 s12: the cross-shell browser driver runs one of two legs (`CROSS_MODE=edits|cursors`), matching the two native laws."""
import pathlib

PATH = pathlib.Path("/Users/ueli/Documents/semio/.tmp-ticket/wp-wg8/cross-shell.mjs")
text = PATH.read_text()


def swap(old, new):
    global text
    assert text.count(old) == 1, old[:80]
    text = text.replace(old, new)


swap(
    "const t0 = Date.now();",
    'const MODE = process.env.CROSS_MODE ?? "edits";\nconst t0 = Date.now();',
)
swap(
    "  const mounted = await until(240_000, (shell) => Number.isFinite(shell.handleKinds) && shell.opening === null);",
    '  const mounted = await until(240_000, (shell) => shell.opening === null && (MODE === "cursors" ? shell.boards.some((box) => box.w > 100 && box.h > 100) : Number.isFinite(shell.handleKinds)));',
)
cursor_start = text.index('  await awaitNative("cursor", 300_000);')
cursor_end = text.index('  const beforeNativeEdit = (await read()).handleKinds;')
cursor_leg = text[cursor_start:cursor_end]
edits_end = text.index('  await awaitNative("done", 300_000);\n} catch (error) {')
edits_leg = text[cursor_end:edits_end]
indent = lambda block: "".join(("  " + line if line.strip() else line) for line in block.splitlines(keepends=True))
text = (
    text[:cursor_start]
    + '  if (MODE === "cursors") {\n'
    + indent(cursor_leg).rstrip("\n")
    + "\n  } else {\n"
    + indent(edits_leg).rstrip("\n")
    + "\n  }\n"
    + text[edits_end:]
)
PATH.write_text(text)
print("driver modes ok")
