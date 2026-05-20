"""Move kit_store_comprehensive_e2e from inside worker to crate root after wasm_bridge."""
from pathlib import Path

lib = Path(r"c:\git\semio\semio\client\lib\rs\lib.rs")
text = lib.read_text(encoding="utf-8")
start = text.index("kit_store_comprehensive_e2e")
start = text.rindex("//#region", 0, start)
end = text.index("//#endregion", start)
end = text.index("kit_store_comprehensive_e2e", end)
end = text.index("\n", text.index("//#endregion", end)) + 1
end = text.index("\n", end) + 1
block = text[start:end].rstrip() + "\n\n"
text = text[:start] + text[end:]
anchor = text.index("pub mod wasm_bridge")
re = anchor
while True:
    re = text.index("//#endregion", re + 1)
    line_end = text.index("\n", re)
    if "wasm_bridge" in text[re:line_end]:
        break
insert_at = line_end + 1
text = text[:insert_at] + "\n" + block + text[insert_at:]
lib.write_text(text, encoding="utf-8")
print("moved e2e to crate root after wasm_bridge")
