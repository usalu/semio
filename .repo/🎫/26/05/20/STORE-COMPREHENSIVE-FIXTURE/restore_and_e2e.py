"""Restore lib.rs from HEAD, reassemble gap mod, install comprehensive E2E (atomic)."""
import importlib.util
import subprocess
from pathlib import Path

root = Path(r"c:\git\semio")
lib = root / "semio" / "client" / "lib" / "rs" / "lib.rs"
ticket = Path(__file__).parent

head = subprocess.check_output(
    ["git", "-C", str(root), "show", "HEAD:semio/client/lib/rs/lib.rs"],
    text=True,
    encoding="utf-8",
)
lib.write_text(head, encoding="utf-8")

spec = importlib.util.spec_from_file_location("reassemble", ticket / "reassemble_gap_mod.py")
mod = importlib.util.module_from_spec(spec)
spec.loader.exec_module(mod)

text = lib.read_text(encoding="utf-8")
if "pub mod kit_store_comprehensive_e2e" in text:
    start = text.index("pub mod kit_store_comprehensive_e2e")
    start = text.rindex("//#region", 0, start)
    end = text.index("//#endregion", start)
    end = text.index("\n", text.index("kit_store_comprehensive_e2e", end)) + 1
    lib.write_text(text[:start] + text[end:], encoding="utf-8")

spec2 = importlib.util.spec_from_file_location(
    "install_e2e", root / ".repo" / "install_comprehensive_e2e_on_head.py"
)
install = importlib.util.module_from_spec(spec2)
spec2.loader.exec_module(install)
