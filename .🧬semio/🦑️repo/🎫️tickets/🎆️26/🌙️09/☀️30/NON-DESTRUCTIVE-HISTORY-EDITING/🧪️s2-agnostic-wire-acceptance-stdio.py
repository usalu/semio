"""🔌️ Wires `history_edit_acceptance_law!` into the stdio artifact crates whose unit tests build a registered editor from its
`create_*_editor()` definition: one line appended at the end of that unit test file, once (S2-AGNOSTIC, design §16.3)."""
import re, subprocess
ROOT = "/Users/ueli/Documents/semio"
paths = subprocess.run(["git", "grep", "-l", "new_registered_app::<EditorApp<", "--", "✏️s/🔌️plugins/🗄️stdio/*.rs"], cwd=ROOT, capture_output=True, text=True).stdout.split()
for path in paths:
    text = open(f"{ROOT}/{path}", encoding="utf-8").read()
    found = re.search(r"new_registered_app::<EditorApp<([A-Za-z0-9_]+)>, _>\(async \{ semio_framework_plugin::App \{ definition: ([a-z0-9_]+)\(\)", text)
    if found is None or "history_edit_acceptance_law!" in text:
        print("skip", path, found is None)
        continue
    editor, definition = found.groups()
    line = f'semio_framework_plugin::history_edit_acceptance_law!("stdio", {editor}, || semio_framework_plugin::App {{ definition: {definition}(), examples: Vec::new() }}, "../..");\n'
    open(f"{ROOT}/{path}", "w", encoding="utf-8").write(text.rstrip("\n") + "\n\n" + line)
    print("wired", editor, path)
