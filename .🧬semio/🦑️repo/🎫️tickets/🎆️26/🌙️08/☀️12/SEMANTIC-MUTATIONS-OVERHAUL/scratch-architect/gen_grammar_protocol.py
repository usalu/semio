#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Rewrite the mutations facet's 📖️component.grammar.semio (text) and 💾️binary/📡️component.protocol.semio
(binary tags) honestly per SEMANTIC-MUTATIONS-OVERHAUL's Phase 3 recipe: one rule/record per real
mutation slug, tags assigned 1..N in dispatch-enum variant order."""
import json, re

plan = json.load(open("/tmp/architect_plan_full.json", encoding="utf-8"))
DISPATCH = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"
GRAMMAR = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📖️component.grammar.semio"
PROTOCOL = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️标准/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/📡️component.protocol.semio"
PROTOCOL = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/📡️component.protocol.semio"

# dispatch-enum variant order (authoritative order for binary tags 1..N)
dispatch_src = open(DISPATCH, encoding="utf-8").read()
enum_body_m = re.search(r"pub enum ProgramMutation \{(.*?)\n\}", dispatch_src, re.DOTALL)
variant_order = re.findall(r"^\s{4}([A-Za-z0-9]+)\(", enum_body_m.group(1), re.MULTILINE)
assert len(variant_order) == 266, len(variant_order)

by_struct = {p["struct_name"]: p for p in plan}

def fields_of(struct_body):
    return re.findall(r"pub\s+(\w+):\s*([^,\n]+),", struct_body)

def snake_to_kebab_field(name):
    return name.replace("_", "-")

# ---- grammar rules ----
rule_lines = []
alt_names = []
noun_blocks = set()
for struct_name in variant_order:
    p = by_struct[struct_name]
    slug = p["kind_slug"]
    alt_names.append(slug)
    fields = fields_of(p["struct_body"])
    parts = [f'"{slug}"']
    for fname, ftype in fields:
        ftype = ftype.strip()
        if fname == "id":
            parts.append("SP id")
        elif fname.startswith("new_"):
            parts.append(f"SP text")
        elif ftype == "EntityId":
            parts.append("SP id")
        else:
            # structured/noun payload -> opaque block, named after the field
            block = f"{snake_to_kebab_field(fname)}-block"
            noun_blocks.add(block)
            parts.append(f"SP {block}")
    rule_lines.append(f"{slug} = {' '.join(parts)}")

grammar = []
grammar.append("dialect grammar")
grammar.append("grammar program.mutations")
grammar.append("extension program")
grammar.append("start line")
grammar.append("")
# alternation, wrapped ~4 per line for readability like sequence's example
alt_chunks = []
for i in range(0, len(alt_names), 4):
    alt_chunks.append(" / ".join(alt_names[i:i+4]))
grammar.append("line = " + ("\n     / ".join(alt_chunks)))
grammar.append("")
grammar.extend(rule_lines)
grammar.append("")
for block in sorted(noun_blocks):
    grammar.append(f'{block} = "{{" NL OCTET+ "}}"')
grammar.append("id = OCTET+")
grammar.append("text = OCTET+")
grammar.append("number = OCTET+")
grammar.append("boolean = \"true\" / \"false\"")

open(GRAMMAR, "w", encoding="utf-8").write("\n".join(grammar) + "\n")
print("grammar written:", len(grammar), "lines")

# ---- binary protocol: one record per variant, tags 1..N in dispatch order ----
proto = []
proto.append("dialect protocol")
proto.append("protocol program.mutations")
proto.append("version 1")
proto.append("schema stdio.json")
proto.append("start record")
proto.append("framing magic 0x8953f83f7d340d0a")
proto.append("header fixed 32")
proto.append("field format_major u16")
proto.append("field format_minor u16")
proto.append("field flags u32")
proto.append("field domain_tag u32")
proto.append("field header_crc32 u32")
proto.append("")
for i, struct_name in enumerate(variant_order, start=1):
    proto.append(f"record {struct_name} tag {i}")
proto.append("")
proto.append("segment payload varint bytes")
proto.append("footer fixed 64")
proto.append("field artifact_mark utf8")
proto.append("field body_crc32 u32")

open(PROTOCOL, "w", encoding="utf-8").write("\n".join(proto) + "\n")
print("protocol written:", len(proto), "lines")
