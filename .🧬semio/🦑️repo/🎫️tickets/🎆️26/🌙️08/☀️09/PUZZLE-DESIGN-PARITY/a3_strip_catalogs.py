from pathlib import Path
import re
puzzle = Path(open("/tmp/puzzle_path.txt").read().strip())
# Backup and strip kind-catalogs to empty default for parse diagnosis
for rel in [
    "🗿️artifacts/🖐️5d/📚️examples/🌲️concrete-forest/🖼️assets/🗣️forest.dsl.semio",
]:
    path = puzzle / rel
    text = path.read_text()
    # Replace kind-catalogs= ... before kind-compatibility with empty
    text2, n = re.subn(
        r'kind-catalogs=\n.*?(\nkind-compatibility )',
        r'kind-catalogs=\nparts [id:TEXT name:TEXT label:TEXT description:TEXT icon:TEXT image:TEXT unit:TEXT abstract:BOOL base-kinds:LIST representations:LIST grips:LIST attributes:LIST authors:LIST] {\n}\ngrips [id:TEXT code:TEXT label:TEXT order:NUM compatible-with:LIST description:TEXT icon:TEXT color:TEXT default-rope-kind:REF] {\n}\nfasteners [id:TEXT name:TEXT label:TEXT] {\n}\nropes [id:TEXT name:TEXT label:TEXT default-fastener-kind:REF] {\n}\1',
        text,
        count=1,
        flags=re.S,
    )
    print('n', n)
    path.write_text(text2)
    print(path.read_text()[:600])
