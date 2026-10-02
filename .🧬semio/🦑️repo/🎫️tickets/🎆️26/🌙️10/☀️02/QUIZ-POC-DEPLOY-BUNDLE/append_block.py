"""Appends (or replaces) a marked block at the end of a source file: `python append_block.py <target> <block file> <marker>`.

Everything from the first line containing the marker's opening text to the end of the target is replaced by the block,
so the script can be run again while the block is still being written. The target is rewritten in place (same length or
longer only: a shorter rewrite of a file a running dev server holds open can leave stale bytes on Windows)."""
import io
import sys

target, block_path, marker = sys.argv[1], sys.argv[2], sys.argv[3]
source = io.open(target, encoding="utf-8").read()
block = io.open(block_path, encoding="utf-8").read()
at = source.find(marker)
kept = source if at < 0 else source[:at]
result = kept.rstrip("\n") + "\n" + block
if len(result.encode("utf-8")) < len(source.encode("utf-8")):
    raise SystemExit("the result is shorter than the file: use the editor instead")
io.open(target, "w", encoding="utf-8", newline="\n").write(result)
print(f"{len(source)} -> {len(result)} characters")
