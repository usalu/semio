#!/usr/bin/env python3
"""🧾️ Re-derives an aggregate's binary and text exports (`💾️binary/{🥋️.ksy,🔠️.abnf,🌶️.spicy}`, `📝️text/{🔤️.ebnf,🅰️.g4,🔣️.json}`)
from its authoritative `💾️binary/📡️.protocol.semio` records, for the byte-authoritative png and bmp vocabularies whose
exports still listed the retired kinds (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING). The kind order is the protocol's
record order; the formats are exactly the ones the vocabulary generator emits. Idempotent; `--check` exits 1 while any
export differs.

@see ../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/📡️.protocol.semio
"""
import json
import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parents[7]
ARTIFACTS = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/"
AGGREGATES = {
    "png": (ARTIFACTS + "📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations", "https://json.schemas.assets.semio-tech.com/s/stdio/png/1.2/any/mutations/text.json", "PngMutationText"),
    "bmp": (ARTIFACTS + "🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations", "https://json.schemas.assets.semio-tech.com/s/stdio/bmp/v3/any/mutations/text.json", "BmpMutationText"),
}


def records(protocol: str) -> list[tuple[str, int]]:
    return [(kind, int(tag)) for kind, tag in re.findall(r"^record ([a-z0-9-]+) tag=(\d+)$", protocol, re.M)]


def exports(name: str, kinds: list[tuple[str, int]], text_id: str, text_title: str) -> dict[str, str]:
    module = f"Stdio_{name}_mutation"
    tags = ", ".join(f"{kind}={tag}" for kind, tag in kinds)
    opcodes = [kind for kind, _ in kinds]
    return {
        "💾️binary/🥋️.ksy": f"meta:\n  id: stdio_{name}_mutation\n  endian: le\nseq:\n  - id: format\n    type: u1\n    valid: 1\n  - id: tag\n    type: u1\n    enum: mutation_kind\n  - id: payload\n    size-eos: true\nenums:\n  mutation_kind:\n" + "".join(f"    {tag}: {kind.replace('-', '_')}\n" for kind, tag in kinds),
        "💾️binary/🔠️.abnf": f"; Direct binary frame: version, descriptor tag, leaf-owned payload.\n; {tags}\nmutation = %x01 tag *OCTET\ntag = " + " / ".join(f"%x{tag:02X}" for _, tag in kinds) + "\nOCTET = %x00-FF\n",
        "💾️binary/🌶️.spicy": f"module {module};\n# Direct tags: {tags}\npublic type Mutation = unit {{ format: uint8; tag: uint8; payload: bytes &eod; }};\n",
        "📝️text/🔤️.ebnf": "(* Direct text registry; payload productions are owned by each leaf. *)\nmutation = opcode, { \" \", argument } ;\nopcode = " + " | ".join(f'"{kind}"' for kind in opcodes) + " ;\nargument = name, \"=\", value ;\nname = letter, { letter | digit | \"-\" } ;\nvalue = character, { character } ;\n",
        "📝️text/🅰️.g4": f"grammar {module};\n// Leaf payload grammars are authoritative in the direct text facets.\nmutation: opcode argument* EOF;\nopcode: " + " | ".join(f"'{kind}'" for kind in opcodes) + ";\nargument: WORD '=' VALUE;\nWORD: [a-zA-Z][a-zA-Z0-9_-]*;\nVALUE: ~[ \\t\\r\\n]+;\nWS: [ \\t\\r\\n]+ -> skip;\n",
        "📝️text/🔣️.json": json.dumps({"$schema": "http://json-schema.org/draft-07/schema#", "$id": text_id, "title": text_title, "type": "string", "pattern": "^(?:" + "|".join(opcodes) + ")(?: |$)"}, indent=2) + "\n",
    }


def main() -> int:
    check = "--check" in sys.argv
    pending = 0
    for name, (directory, text_id, text_title) in AGGREGATES.items():
        base = ROOT / directory
        kinds = records((base / "💾️binary/📡️.protocol.semio").read_text(encoding="utf-8"))
        for relative, content in exports(name, kinds, text_id, text_title).items():
            path = base / relative
            if path.read_text(encoding="utf-8") != content:
                pending += 1
                print(f"{'pending' if check else 'written'}: {name} {relative}")
                if not check:
                    path.write_text(content, encoding="utf-8")
    print(f"{pending} export(s) {'pending' if check else 'written'}")
    return 1 if check and pending else 0


if __name__ == "__main__":
    sys.exit(main())
