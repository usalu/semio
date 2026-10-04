#!/usr/bin/env python3
"""🪆️ S4-TEXT (session 4, design §20.15): jack's content edits leave the parent lane. The eight parent leaves that read the
composed `content` child (create/delete-node, create/delete-edge, rename-node, move-node, change-/remove-data-property) are
deleted with every registry that names them; the parent vocabulary keeps `set-query` (tag 0). Content edits are child-lane
leaves of the shared `s.stdio.semio@v1/graph` vocabulary, published by the editor (query runs, rails, reorganize).
Every rewrite asserts its anchor; files are staged and written together; `--check` is a dry run. Run from the repo root."""
import json
import re
import shutil
import sys
from pathlib import Path

J = Path("✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack")
S = J / "🏅️standards/🔖️1/🪆️subsets/✳️any"
M = S / "🧬️schema/🧬️mutations"
F = S / "🧫️fixtures/🧬️mutations"
GONE = ["➕️create-node", "🗑️delete-node", "🌉️create-edge", "✂️delete-edge", "✏️rename-node", "📍️move-node", "🔧️change-data-property", "🧹️remove-data-property"]
GONE_MODS = ["create_node", "delete_node", "create_edge", "delete_edge", "rename_node", "move_node", "change_data_property", "remove_data_property"]
staged: dict = {}
deleted: list = []


def text(path: Path) -> str:
    return staged.get(path) or path.read_text()


def write(path: Path, content: str) -> None:
    staged[path] = content


def replace(path: Path, old: str, new: str, count: int = 1) -> None:
    content = text(path)
    found = content.count(old)
    if found != count:
        sys.exit(f"{path}: expected {count} of {old[:80]!r}, found {found}")
    staged[path] = content.replace(old, new)


write(M / "🦀️.rs", '''//! ⚡️ `trinity.graph` semantic mutation aggregate — the parent lane edits the document's own query only.
//!
//! The scene lives in the composed `content` child (`s.stdio.semio@v1/graph`); every scene edit is a child-lane leaf of
//! that shared vocabulary (design §20.15), so no parent leaf reads the child.

use crate::standards::v1::subsets::any::schema::diff::JackDiff;
use crate::JackSnapshot;

pub use super::set_query::{set_query, SetQuery};

//#region 🔖️Aggregate
/// 🧮️ Parent-lane trinity graph mutation vocabulary.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = JackSnapshot, diff = JackDiff, schema = "s.trinity.jack")]
pub enum TrinityGraphMutation {
    SetQuery(SetQuery),
}
//#endregion 🔖️Aggregate

//#region 🧪️StructuralCorrespondence
#[cfg(test)]
#[path = "🧪️tests/🔬️structural-correspondence/🦀️.rs"]
mod structural_correspondence_tests;
//#endregion 🧪️StructuralCorrespondence
''')

replace(M / "💾️binary/🦀️.rs", '''pub const BINARY_TAG_REGISTRY: &[(&str, u8)] = &[
    ("CreateNode", super::create_node::binary::BINARY_TAG),
    ("DeleteNode", super::delete_node::binary::BINARY_TAG),
    ("CreateEdge", super::create_edge::binary::BINARY_TAG),
    ("DeleteEdge", super::delete_edge::binary::BINARY_TAG),
    ("RenameNode", super::rename_node::binary::BINARY_TAG),
    ("MoveNode", super::move_node::binary::BINARY_TAG),
    ("ChangeDataProperty", super::change_data_property::binary::BINARY_TAG),
    ("RemoveDataProperty", super::remove_data_property::binary::BINARY_TAG),
    ("SetQuery", super::set_query::binary::BINARY_TAG),
];''', '''pub const BINARY_TAG_REGISTRY: &[(&str, u8)] = &[("SetQuery", super::set_query::binary::BINARY_TAG)];''')

write(M / "💾️binary/📡️.protocol.semio", '''dialect protocol
protocol jack.mutations
version 1
schema trinity.jack.mutations
start record
framing record

# Real op frame (`dsl::variants_binary::encode_tagged_op`): `format u8` (`OP_BINARY_FORMAT`) then `tag varint`,
# then the kind's framework record body (`os_pack::encode_record_body`). Each record is one mutation kind at its
# wire tag; this file is the only source of those tags, which the codec looks up by the kind's keyword.
header fixed 2
field format u8
field tag varint
record set-query tag=0
field payload bytes
''')

replace(M / "📝️text/🦀️.rs", '''pub const TEXT_OPCODE_REGISTRY: &[(&str, &str)] = &[
    ("CreateNode", super::create_node::text::TEXT_OPCODE),
    ("DeleteNode", super::delete_node::text::TEXT_OPCODE),
    ("CreateEdge", super::create_edge::text::TEXT_OPCODE),
    ("DeleteEdge", super::delete_edge::text::TEXT_OPCODE),
    ("RenameNode", super::rename_node::text::TEXT_OPCODE),
    ("MoveNode", super::move_node::text::TEXT_OPCODE),
    ("ChangeDataProperty", super::change_data_property::text::TEXT_OPCODE),
    ("RemoveDataProperty", super::remove_data_property::text::TEXT_OPCODE),
    ("SetQuery", super::set_query::text::TEXT_OPCODE),
];''', '''pub const TEXT_OPCODE_REGISTRY: &[(&str, &str)] = &[("SetQuery", super::set_query::text::TEXT_OPCODE)];''')

write(M / "📝️text/📖️.grammar.semio", '''dialect grammar
grammar jack.mutations
extension jack
start line

line = set-query

set-query = "set-query" SP text

text = OCTET+
''')

write(M / "📝️text/🔤️.ebnf", '''(* ebnf trinity.jack.mutations *)
(* ISO-14977-style mirror of the normative 📖️.grammar.semio (same production names, kebab-case -> "space case"). *)
line = set query ;
set query = 'set-query', space, text ;
text = octet, { octet } ;

(* Framework dialect-primitive terminals, given one concrete definition each for a self-contained mirror. *)
octet = ? any single raw byte ? ;
space = ' ' ;
''')

write(M / "📝️text/🅰️.g4", '''// 🅰️ ANTLR4 mirror of the normative 📖️.grammar.semio (same production names, kebab-case -> camelCase).
grammar Trinity_jack_mutations;

line: setQuery ;
setQuery: 'set-query' SP text ;
text: OCTET+ ;

// 📐 Framework dialect-primitive terminals (not defined in the .semio itself).
OCTET: . ;
SP: ' ' ;
''')

write(M / "🔣️.json", json.dumps({
    "$schema": "http://json-schema.org/draft-07/schema#",
    "$id": "https://json.schemas.assets.semio-tech.com/s/trinity/jack/mutations.json",
    "title": "JackMutation",
    "oneOf": [{"$ref": "https://json.schemas.assets.semio-tech.com/s/trinity/jack/mutation/set-query/schema.json"}],
}, indent=2, ensure_ascii=False) + "\n")

write(M / "🔗️.graphql", '''# 🧬️ Jack direct mutation aggregate.
input SetQueryInput { value: String! }

input JackMutationInput {
  setQuery: SetQueryInput
}
''')

write(M / "🛰️.proto", '''syntax = "proto3";
package semio.s.trinity.jack.mutation;

message SetQuery { string value = 1; }

message JackMutation {
  oneof mutation {
    SetQuery set_query = 1;
  }
}
''')

write(M / "🟦️.ts", '''/** 🧩️ Jack direct-mutation discriminated union. */
import type { SetQuery } from "./🔎️set-query/🟦️.ts";

export type JackMutation = { mutation: "setQuery" } & SetQuery;
''')

replace(M / "🔎️set-query/🔣️.json", '"binaryTag": 8,', '"binaryTag": 0,')

root = J / "🦀️.rs"
content = text(root)
for module, directory in zip(GONE_MODS, GONE):
    block = re.compile(r"                        #\[path = \"\.\"\]\n                        pub mod " + module + r" \{\n(?:                            .*\n)+?                        \}\n")
    found = block.findall(content)
    if len(found) != 1 or directory not in found[0]:
        sys.exit(f"{root}: module block {module} found {len(found)} times")
    content = block.sub("", content)
staged[root] = content
replace(root, '''/// 📍️ Cross-artifact node movement constructor used by Rewriting's Jack-backed editor world.
pub use crate::standards::v1::subsets::any::schema::mutations::move_node::move_node;
''', "")

for directory in GONE:
    for base in (M, F):
        path = base / directory
        if not path.is_dir():
            sys.exit(f"{path}: expected a leaf tree")
        deleted.append(path)

if "--check" not in sys.argv:
    for path, content in staged.items():
        path.write_text(content)
    for path in deleted:
        shutil.rmtree(path)
print(f"jack child lane: {len(staged)} files {'checked' if '--check' in sys.argv else 'written'}, {len(deleted)} leaf trees {'to delete' if '--check' in sys.argv else 'deleted'}")
