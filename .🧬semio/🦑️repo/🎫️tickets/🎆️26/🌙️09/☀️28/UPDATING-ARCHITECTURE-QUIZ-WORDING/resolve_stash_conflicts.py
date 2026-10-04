"""Resolve the stale stash-pop conflict in the library nx plugin's cacheInternals export by taking the union of both sides."""
import pathlib

path = pathlib.Path(r"C:\git\semio\🧰️framework\🛍️products\🦑️repo\🔨️modules\📚️library\🟨️.mjs")
text = path.read_text(encoding="utf-8")
start = text.index("<<<<<<< Updated upstream\nexport const cacheInternals")
middle = text.index("=======\n", start)
end = text.index(">>>>>>> Stashed changes\n", middle)
upstream = text[start + len("<<<<<<< Updated upstream\n"):middle]
assert upstream.rstrip().endswith("importEdgeCacheRoot };"), upstream[-80:]
merged = upstream.replace("importEdgeCacheRoot };", "importEdgeCacheRoot, nxTrackedSourceFile, walkCargoToml };")
text = text[:start] + merged + text[end + len(">>>>>>> Stashed changes\n"):]
assert "<<<<<<<" not in text and ">>>>>>>" not in text
path.write_text(text, encoding="utf-8", newline="")
print("resolved")
