"""🎚️ Rehearses the staged ToUnicode admission using exact PDF lexer and CMap modules."""

import json
from pathlib import Path
import sys

TICKET = Path(__file__).resolve().parent
OUT = TICKET / "🗑️generated" / "font-unicode"


def exercise(base):
    cases = json.loads((OUT / "expect.json").read_text())["cases"]
    def quoted(value): return json.dumps(value, ensure_ascii=False)
    records = "\n".join(
        f'        (include_bytes!({quoted((OUT / (row["name"] + ".cmap")).as_posix())}), {row["width"]}, &{row["codes"]}, {quoted(row["text"])}),'
        for row in cases
    )
    contract = (TICKET / "controlled-tounicode-contract.rs").read_text().replace("@ORACLE@", records)

    def patch(scratch, main, base):
        cmap = scratch / "cmap.rs"
        staged = (OUT / "cmap.rs").read_text().replace("🎚️controlled/🦀️.rs", (OUT / "controlled.rs").as_posix())
        original = base / "🔨️modules/🔤️fonts/🗺️cmap/🦀️.rs"
        staged = staged.replace("🧪️tests/🔬️unit/🦀️.rs", (original.parent / "🧪️tests/🔬️unit/🦀️.rs").as_posix())
        cmap.write_text(staged)
        font = scratch / "font-codec.rs"
        font.write_text(font.read_text().replace(original.as_posix(), cmap.as_posix()))
        main.write_text(main.read_text() + "\nmod direct_tounicode_contract {\n" + contract + "\n}\n")

    wrapper = TICKET / "📜️content-program-native.py"
    source = wrapper.read_text().replace("        binary = scratch /", "        patch(scratch, source, base)\n        binary = scratch /")
    cursor = {"__file__": str(wrapper), "patch": patch}
    exec(compile(source, str(wrapper), "exec"), cursor)
    cursor["exercise"](base / "🔨️modules/🖋️content", OUT)


if __name__ == "__main__":
    exercise(Path(sys.argv[1]).resolve())
