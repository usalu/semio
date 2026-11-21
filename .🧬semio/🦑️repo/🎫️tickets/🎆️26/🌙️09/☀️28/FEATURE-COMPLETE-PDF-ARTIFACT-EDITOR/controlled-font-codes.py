"""🔢️ Drives staged PDF fixed-width codes through real content and page program cursors."""

import json
from pathlib import Path
import re
import sys

TICKET = Path(__file__).resolve().parent
OUT = TICKET / "🗑️generated" / "font-codes"


def exercise(base):
    substitutions = {name: Path(path) for name, path in json.loads((OUT / "stages.json").read_text()).items()}
    contract = json.loads((OUT / "expect.json").read_text())
    existing = (TICKET / "controlled-font-codes.rs").read_text()
    cases = []
    for row in contract["cases"]:
        columns = [json.dumps(row["name"]), str(row["width"]), json.dumps(row["program"]), f'&{row["show"]}',
                   ", ".join(f"&{item}" for item in row["array"]), ", ".join(str(float(item)) for item in row["adjustments"])]
        name, width, program, show, array, adjustments = columns
        cases.append(f"        ({name}, {width}, {program}, {show}, &[{array}], &[{adjustments}]),")
    existing = existing.replace("@CASES@", "\n".join(cases))

    def staged_path(original, scratch, name):
        file = scratch / (name + ".rs")
        relative = original.relative_to(base).as_posix()
        source = substitutions.get(relative, original).read_text()
        def relocate(match):
            dependency = (original.parent / match[1]).resolve()
            chosen = substitutions.get(dependency.relative_to(base).as_posix(), dependency)
            return f'#[path = "{chosen.as_posix()}"]'
        file.write_text(re.sub(r'#\[path = "([^"]+\.rs)"\]', relocate, source))
        return file

    def patch(scratch, main, base):
        source = main.read_text()
        content = base / "🔨️modules/🖋️content/🦀️.rs"
        page = base / "🔨️modules/⬇️lift/🧩️lifter-and-document/📜️page-content/🦀️.rs"
        source = source.replace(content.as_posix(), staged_path(content, scratch, "native-content").as_posix())
        source = source.replace(page.as_posix(), staged_path(page, scratch, "native-page").as_posix())
        main.write_text(source + "\nmod fixed_codes_contract {\n" + existing + "\n}\n")

    wrapper = TICKET / "📜️content-program-native.py"
    source = wrapper.read_text().replace("        binary = scratch /", "        patch(scratch, source, base)\n        binary = scratch /")
    cursor = {"__file__": str(wrapper), "patch": patch}
    exec(compile(source, str(wrapper), "exec"), cursor)
    cursor["exercise"](base / "🔨️modules/🖋️content", OUT, base / "🔨️modules/⬇️lift/🧩️lifter-and-document/📜️page-content")


if __name__ == "__main__":
    exercise(Path(sys.argv[1]).resolve())
