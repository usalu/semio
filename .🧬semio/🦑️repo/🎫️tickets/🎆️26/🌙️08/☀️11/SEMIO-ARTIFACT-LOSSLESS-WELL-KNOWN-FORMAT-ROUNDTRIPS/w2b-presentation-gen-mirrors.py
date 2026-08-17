#!/usr/bin/env python3
# Scratch generator (ticket-folder-local, not a permanent repo script) for the presentation
# subset's facet mirrors (ts/graphql/json/proto) + grammar leaves (text 8 / binary 6), matching
# the real Rust shapes in schema/{snapshot,diff,mutations}/component.rs and schema/component.rs.
import os
import json

ROOT = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️presentation/🧬️schema"

def w(path, content):
    full = os.path.join(ROOT, path)
    os.makedirs(os.path.dirname(full), exist_ok=True)
    with open(full, "w", encoding="utf-8") as f:
        f.write(content)
    print("wrote", full)

# ---------------------------------------------------------------------------
# SNAPSHOT facet mirrors
# ---------------------------------------------------------------------------
w("📸️snapshot/🟦️component.ts", """/** 🧬️ SemioPresentationSnapshot — masters/layouts/slides -> shapes (TextBox/Picture/Table/
 * Placeholder) + per-slide notes. `DocBlock` is document's own type (imported, not redefined). */
import type { DocBlock } from "../../../document/schema/snapshot/component";

export interface SemioPoint2 { x: number; y: number; }

export interface SlideFrame { origin: SemioPoint2; width: number; height: number; }

export interface SlidePictureImage { assetId: string; mime: string; bytes: number[]; }

export type PlaceholderKind =
  | { kind: "title" } | { kind: "subtitle" } | { kind: "body" } | { kind: "footer" }
  | { kind: "slideNumber" } | { kind: "dateTime" } | { kind: "other"; value: string };

export interface SlideTableCell { blocks: DocBlock[]; }
export interface SlideTableRow { cells: SlideTableCell[]; }

export type SlideShape =
  | { shapeKind: "textBox"; frame: SlideFrame; blocks: DocBlock[] }
  | { shapeKind: "picture"; frame: SlideFrame; image: SlidePictureImage }
  | { shapeKind: "table"; frame: SlideFrame; rows: SlideTableRow[] }
  | { shapeKind: "placeholder"; frame: SlideFrame; kind: PlaceholderKind };

export interface SlideMaster { id: string; shapes: SlideShape[]; }
export interface SlideLayout { id: string; masterId: string; shapes: SlideShape[]; }
export interface Slide { id: string; layoutId?: string | null; shapes: SlideShape[]; notes: DocBlock[]; }

export interface SemioPresentationSnapshot {
  /** @state persistent */ schema: string;
  /** @state persistent */ masters: SlideMaster[];
  /** @state persistent */ layouts: SlideLayout[];
  /** @state persistent */ slides: Slide[];
}
""")

w("📸️snapshot/🔣️component.json", """{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "SemioPresentationSnapshot",
  "type": "object",
  "required": ["schema", "masters", "layouts", "slides"],
  "properties": {
    "schema": { "type": "string" },
    "masters": { "type": "array", "items": { "$ref": "#/definitions/SlideMaster" } },
    "layouts": { "type": "array", "items": { "$ref": "#/definitions/SlideLayout" } },
    "slides": { "type": "array", "items": { "$ref": "#/definitions/Slide" } }
  },
  "definitions": {
    "SemioPoint2": { "type": "object", "required": ["x", "y"], "properties": { "x": { "type": "number" }, "y": { "type": "number" } } },
    "SlideFrame": { "type": "object", "required": ["origin", "width", "height"], "properties": { "origin": { "$ref": "#/definitions/SemioPoint2" }, "width": { "type": "number" }, "height": { "type": "number" } } },
    "SlidePictureImage": { "type": "object", "required": ["assetId", "mime", "bytes"], "properties": { "assetId": { "type": "string" }, "mime": { "type": "string" }, "bytes": { "type": "array", "items": { "type": "integer", "minimum": 0, "maximum": 255 } } } },
    "PlaceholderKind": { "type": "object", "required": ["kind"], "properties": { "kind": { "enum": ["title", "subtitle", "body", "footer", "slideNumber", "dateTime", "other"] }, "value": { "type": "string" } } },
    "SlideTableCell": { "type": "object", "required": ["blocks"], "properties": { "blocks": { "type": "array" } } },
    "SlideTableRow": { "type": "object", "required": ["cells"], "properties": { "cells": { "type": "array", "items": { "$ref": "#/definitions/SlideTableCell" } } } },
    "SlideShape": {
      "type": "object", "required": ["shapeKind", "frame"],
      "properties": {
        "shapeKind": { "enum": ["textBox", "picture", "table", "placeholder"] },
        "frame": { "$ref": "#/definitions/SlideFrame" },
        "blocks": { "type": "array" },
        "image": { "$ref": "#/definitions/SlidePictureImage" },
        "rows": { "type": "array", "items": { "$ref": "#/definitions/SlideTableRow" } },
        "kind": { "$ref": "#/definitions/PlaceholderKind" }
      }
    },
    "SlideMaster": { "type": "object", "required": ["id", "shapes"], "properties": { "id": { "type": "string" }, "shapes": { "type": "array", "items": { "$ref": "#/definitions/SlideShape" } } } },
    "SlideLayout": { "type": "object", "required": ["id", "masterId", "shapes"], "properties": { "id": { "type": "string" }, "masterId": { "type": "string" }, "shapes": { "type": "array", "items": { "$ref": "#/definitions/SlideShape" } } } },
    "Slide": { "type": "object", "required": ["id", "shapes", "notes"], "properties": { "id": { "type": "string" }, "layoutId": { "type": ["string", "null"] }, "shapes": { "type": "array", "items": { "$ref": "#/definitions/SlideShape" } }, "notes": { "type": "array" } } }
  }
}
""")

w("📸️snapshot/🔗️component.graphql", """# 🧬️ SemioPresentationSnapshot — masters/layouts/slides -> shapes + notes.
type SemioPoint2 { x: Float!, y: Float! }
type SlideFrame { origin: SemioPoint2!, width: Float!, height: Float! }
type SlidePictureImage { assetId: String!, mime: String!, bytes: [Int!]! }

enum PlaceholderKindTag { TITLE, SUBTITLE, BODY, FOOTER, SLIDE_NUMBER, DATE_TIME, OTHER }
type PlaceholderKind { kind: PlaceholderKindTag!, value: String }

enum ShapeKind { TEXT_BOX, PICTURE, TABLE, PLACEHOLDER }
type SlideShape {
  shapeKind: ShapeKind!
  frame: SlideFrame!
  blocks: [DocBlock!]
  image: SlidePictureImage
  rows: [SlideTableRow!]
  kind: PlaceholderKind
}
type SlideTableCell { blocks: [DocBlock!]! }
type SlideTableRow { cells: [SlideTableCell!]! }

type SlideMaster { id: String!, shapes: [SlideShape!]! }
type SlideLayout { id: String!, masterId: String!, shapes: [SlideShape!]! }
type Slide { id: String!, layoutId: String, shapes: [SlideShape!]!, notes: [DocBlock!]! }

type SemioPresentationSnapshot {
  schema: String!
  masters: [SlideMaster!]!
  layouts: [SlideLayout!]!
  slides: [Slide!]!
}
""")

w("📸️snapshot/🛰️component.proto", """// 🧬️ SemioPresentationSnapshot — masters/layouts/slides -> shapes + notes.
syntax = "proto3";
package stdio.semio.presentation.snapshot;

message SemioPoint2 { double x = 1; double y = 2; }
message SlideFrame { SemioPoint2 origin = 1; double width = 2; double height = 3; }
message SlidePictureImage { string asset_id = 1; string mime = 2; bytes data = 3; }

message PlaceholderKind {
  enum Tag { TITLE = 0; SUBTITLE = 1; BODY = 2; FOOTER = 3; SLIDE_NUMBER = 4; DATE_TIME = 5; OTHER = 6; }
  Tag tag = 1;
  string value = 2; // set only when tag == OTHER
}

message SlideTableCell { repeated bytes blocks = 1; } // DocBlock (document subset's own wire form)
message SlideTableRow { repeated SlideTableCell cells = 1; }

message SlideShape {
  enum Kind { TEXT_BOX = 0; PICTURE = 1; TABLE = 2; PLACEHOLDER = 3; }
  Kind shape_kind = 1;
  SlideFrame frame = 2;
  repeated bytes blocks = 3;          // TextBox only (DocBlock wire form)
  SlidePictureImage image = 4;        // Picture only
  repeated SlideTableRow rows = 5;    // Table only
  PlaceholderKind placeholder_kind = 6; // Placeholder only
}

message SlideMaster { string id = 1; repeated SlideShape shapes = 2; }
message SlideLayout { string id = 1; string master_id = 2; repeated SlideShape shapes = 3; }
message Slide { string id = 1; optional string layout_id = 2; repeated SlideShape shapes = 3; repeated bytes notes = 4; }

message SemioPresentationSnapshot {
  string schema = 1;
  repeated SlideMaster masters = 2;
  repeated SlideLayout layouts = 3;
  repeated Slide slides = 4;
}
""")

print("snapshot facet mirrors done")

# ---------------------------------------------------------------------------
# DIFF facet mirrors
# ---------------------------------------------------------------------------
w("🔺️diff/🟦️component.ts", """/** 🔺️ SemioPresentationDiff — handcrafted sparse diff. Generic triple types are this subset's own
 * local copy (see the Rust file's module doc comment for why). */
import type { SlideFrame, SlidePictureImage, PlaceholderKind, SlideShape, Slide, SlideMaster, SlideLayout } from "../📸️snapshot/component";
import type { DocBlock } from "../../../document/schema/snapshot/component";

export interface IndexModified<D> { index: number; diff: D; }
export interface IndexAdded<T> { index: number; item: T; }
export interface IndexedTripleDiff<D, T> { removed: number[]; modified: IndexModified<D>[]; added: IndexAdded<T>[]; }
export interface NamedModified<K, D> { key: K; diff: D; }
export interface NamedTripleDiff<K, D, T> { removed: K[]; modified: NamedModified<K, D>[]; added: T[]; }

export interface SlideFrameDiff { origin?: { x: number; y: number }; width?: number; height?: number; }
export interface SlidePictureImageDiff { assetId?: string; mime?: string; bytes?: number[]; }
export type DocBlocksDiff = IndexedTripleDiff<DocBlock, DocBlock>; // whole-value (D = T), see doc comment

export type SlideShapeDiff =
  | { shapeKind: "textBox"; frame?: SlideFrameDiff; blocks?: DocBlocksDiff }
  | { shapeKind: "picture"; frame?: SlideFrameDiff; image?: SlidePictureImageDiff }
  | { shapeKind: "table"; frame?: SlideFrameDiff; rows?: IndexedTripleDiff<SlideTableRowDiff, unknown> }
  | { shapeKind: "placeholder"; frame?: SlideFrameDiff; kind?: PlaceholderKind }
  | { shapeKind: "replace"; shape: SlideShape };

export interface SlideTableCellDiff { blocks?: DocBlocksDiff; }
export interface SlideTableRowDiff { cells?: IndexedTripleDiff<SlideTableCellDiff, unknown>; }

export type SlideShapesDiff = IndexedTripleDiff<SlideShapeDiff, SlideShape>;
export interface SlideMasterDiff { shapes?: SlideShapesDiff; }
export interface SlideLayoutDiff { masterId?: string; shapes?: SlideShapesDiff; }
export interface SlideDiff {
  /** tri-state: absent = unchanged, null = cleared, string = set */
  layoutId?: string | null;
  shapes?: SlideShapesDiff;
  notes?: DocBlocksDiff;
}

export interface SemioPresentationDiff {
  masters?: NamedTripleDiff<string, SlideMasterDiff, SlideMaster>;
  layouts?: NamedTripleDiff<string, SlideLayoutDiff, SlideLayout>;
  slides?: IndexedTripleDiff<SlideDiff, Slide>;
}
""")

w("🔺️diff/🔣️component.json", """{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "SemioPresentationDiff",
  "type": "object",
  "properties": {
    "masters": { "$ref": "#/definitions/NamedTripleDiff" },
    "layouts": { "$ref": "#/definitions/NamedTripleDiff" },
    "slides": { "$ref": "#/definitions/IndexedTripleDiff" }
  },
  "definitions": {
    "IndexedTripleDiff": { "type": "object", "properties": { "removed": { "type": "array", "items": { "type": "integer" } }, "modified": { "type": "array" }, "added": { "type": "array" } } },
    "NamedTripleDiff": { "type": "object", "properties": { "removed": { "type": "array" }, "modified": { "type": "array" }, "added": { "type": "array" } } },
    "SlideFrameDiff": { "type": "object", "properties": { "origin": { "type": "object" }, "width": { "type": "number" }, "height": { "type": "number" } } },
    "SlideDiff": { "type": "object", "properties": { "layoutId": { "type": ["string", "null"] }, "shapes": { "$ref": "#/definitions/IndexedTripleDiff" }, "notes": { "$ref": "#/definitions/IndexedTripleDiff" } } }
  }
}
""")

w("🔺️diff/🔗️component.graphql", """# 🔺️ SemioPresentationDiff — sparse, handcrafted.
type IndexModified { index: Int!, diffJson: String! }
type IndexAdded { index: Int!, itemJson: String! }
type IndexedTripleDiffMeta { removed: [Int!]!, modified: [IndexModified!]!, added: [IndexAdded!]! }
type NamedModified { key: String!, diffJson: String! }
type NamedTripleDiffMeta { removed: [String!]!, modified: [NamedModified!]!, added: [String!]! }

type SlideFrameDiff { origin: SemioPoint2, width: Float, height: Float }
type SlideDiff { layoutId: String, shapesJson: String, notesJson: String }

type SemioPresentationDiff {
  mastersJson: String
  layoutsJson: String
  slidesJson: String
}
""")

w("🔺️diff/🛰️component.proto", """// 🔺️ SemioPresentationDiff — sparse, handcrafted; generic triples own local message shapes.
syntax = "proto3";
package stdio.semio.presentation.diff;

message IndexModified { uint64 index = 1; bytes diff = 2; }
message IndexAdded { uint64 index = 1; bytes item = 2; }
message IndexedTripleDiff { repeated uint64 removed = 1; repeated IndexModified modified = 2; repeated IndexAdded added = 3; }
message NamedModified { string key = 1; bytes diff = 2; }
message NamedTripleDiff { repeated string removed = 1; repeated NamedModified modified = 2; repeated bytes added = 3; }

message SemioPresentationDiff {
  optional NamedTripleDiff masters = 1;
  optional NamedTripleDiff layouts = 2;
  optional IndexedTripleDiff slides = 3;
}
""")

print("diff facet mirrors done")

# ---------------------------------------------------------------------------
# MUTATIONS facet mirrors
# ---------------------------------------------------------------------------
w("🧬️mutations/🟦️component.ts", """/** 🧬️ SemioPresentationMutation — named-variant mutation vocabulary, discriminated by `mutation`. */
import type { SemioPresentationSnapshot, Slide, SlideShape, SlideFrame, SlideMaster, SlideLayout } from "../📸️snapshot/component";
import type { DocBlock } from "../../../document/schema/snapshot/component";

export type SemioPresentationMutation =
  | { mutation: "noMutation" }
  | { mutation: "setSnapshot"; snapshot: SemioPresentationSnapshot }
  | { mutation: "insertSlide"; index: number; slide: Slide }
  | { mutation: "removeSlide"; index: number }
  | { mutation: "setSlideLayout"; index: number; layoutId?: string | null }
  | { mutation: "setSlideNotes"; index: number; notes: DocBlock[] }
  | { mutation: "insertShape"; slideIndex: number; shapeIndex: number; shape: SlideShape }
  | { mutation: "removeShape"; slideIndex: number; shapeIndex: number }
  | { mutation: "setShapeFrame"; slideIndex: number; shapeIndex: number; frame: SlideFrame }
  | { mutation: "setTextBoxBlocks"; slideIndex: number; shapeIndex: number; blocks: DocBlock[] }
  | { mutation: "insertMaster"; master: SlideMaster }
  | { mutation: "removeMaster"; id: string }
  | { mutation: "insertLayout"; layout: SlideLayout }
  | { mutation: "removeLayout"; id: string }
  | { mutation: "setLayoutMaster"; id: string; masterId: string };
""")

w("🧬️mutations/🔣️component.json", """{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "SemioPresentationMutation",
  "type": "object",
  "required": ["mutation"],
  "properties": {
    "mutation": {
      "enum": ["noMutation", "setSnapshot", "insertSlide", "removeSlide", "setSlideLayout", "setSlideNotes",
                "insertShape", "removeShape", "setShapeFrame", "setTextBoxBlocks",
                "insertMaster", "removeMaster", "insertLayout", "removeLayout", "setLayoutMaster"]
    },
    "index": { "type": "integer" },
    "slideIndex": { "type": "integer" },
    "shapeIndex": { "type": "integer" },
    "id": { "type": "string" },
    "masterId": { "type": "string" },
    "layoutId": { "type": ["string", "null"] }
  }
}
""")

w("🧬️mutations/🔗️component.graphql", """# 🧬️ SemioPresentationMutation — named-variant vocabulary.
enum PresentationMutationTag {
  NO_MUTATION, SET_SNAPSHOT, INSERT_SLIDE, REMOVE_SLIDE, SET_SLIDE_LAYOUT, SET_SLIDE_NOTES,
  INSERT_SHAPE, REMOVE_SHAPE, SET_SHAPE_FRAME, SET_TEXTBOX_BLOCKS,
  INSERT_MASTER, REMOVE_MASTER, INSERT_LAYOUT, REMOVE_LAYOUT, SET_LAYOUT_MASTER
}

type SemioPresentationMutation {
  mutation: PresentationMutationTag!
  index: Int
  slideIndex: Int
  shapeIndex: Int
  id: String
  masterId: String
  layoutId: String
  snapshotJson: String
  slideJson: String
  shapeJson: String
  frameJson: String
  blocksJson: String
  masterJson: String
  layoutJson: String
}
""")

w("🧬️mutations/🛰️component.proto", """// 🧬️ SemioPresentationMutation — named-variant vocabulary, one message per variant.
syntax = "proto3";
package stdio.semio.presentation.mutations;

message NoMutation {}
message SetSnapshot { bytes snapshot = 1; }
message InsertSlide { uint64 index = 1; bytes slide = 2; }
message RemoveSlide { uint64 index = 1; }
message SetSlideLayout { uint64 index = 1; optional string layout_id = 2; }
message SetSlideNotes { uint64 index = 1; repeated bytes notes = 2; }
message InsertShape { uint64 slide_index = 1; uint64 shape_index = 2; bytes shape = 3; }
message RemoveShape { uint64 slide_index = 1; uint64 shape_index = 2; }
message SetShapeFrame { uint64 slide_index = 1; uint64 shape_index = 2; bytes frame = 3; }
message SetTextBoxBlocks { uint64 slide_index = 1; uint64 shape_index = 2; repeated bytes blocks = 3; }
message InsertMaster { bytes master = 1; }
message RemoveMaster { string id = 1; }
message InsertLayout { bytes layout = 1; }
message RemoveLayout { string id = 1; }
message SetLayoutMaster { string id = 1; string master_id = 2; }

message SemioPresentationMutation {
  oneof body {
    NoMutation no_mutation = 1;
    SetSnapshot set_snapshot = 2;
    InsertSlide insert_slide = 3;
    RemoveSlide remove_slide = 4;
    SetSlideLayout set_slide_layout = 5;
    SetSlideNotes set_slide_notes = 6;
    InsertShape insert_shape = 7;
    RemoveShape remove_shape = 8;
    SetShapeFrame set_shape_frame = 9;
    SetTextBoxBlocks set_textbox_blocks = 10;
    InsertMaster insert_master = 11;
    RemoveMaster remove_master = 12;
    InsertLayout insert_layout = 13;
    RemoveLayout remove_layout = 14;
    SetLayoutMaster set_layout_master = 15;
  }
}
""")

print("mutations facet mirrors done")

# ---------------------------------------------------------------------------
# Top-level "Artifact" facet mirrors (schema/component.*)
# ---------------------------------------------------------------------------
w("🟦️component.ts", """/** 🧬️ SemioPresentationArtifact — full artifact state, mirrors SemioPresentationSnapshot. */
import type { SlideMaster, SlideLayout, Slide } from "./📸️snapshot/component";

export interface SemioPresentationArtifact {
  schema: string;
  masters: SlideMaster[];
  layouts: SlideLayout[];
  slides: Slide[];
}
""")

w("🔣️component.json", """{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "SemioPresentationArtifact",
  "type": "object",
  "required": ["schema", "masters", "layouts", "slides"],
  "properties": {
    "schema": { "type": "string" },
    "masters": { "type": "array" },
    "layouts": { "type": "array" },
    "slides": { "type": "array" }
  }
}
""")

w("🔗️component.graphql", """# 🧬️ SemioPresentationArtifact — full artifact state.
type SemioPresentationArtifact {
  schema: String!
  masters: [SlideMaster!]!
  layouts: [SlideLayout!]!
  slides: [Slide!]!
}
""")

w("🛰️component.proto", """// 🧬️ SemioPresentationArtifact — full artifact state.
syntax = "proto3";
package stdio.semio.presentation;

import "📸️snapshot/component.proto";

message SemioPresentationArtifact {
  string schema = 1;
  repeated stdio.semio.presentation.snapshot.SlideMaster masters = 2;
  repeated stdio.semio.presentation.snapshot.SlideLayout layouts = 3;
  repeated stdio.semio.presentation.snapshot.Slide slides = 4;
}
""")

print("top-level artifact facet mirrors done")

# ---------------------------------------------------------------------------
# Grammar leaves — SNAPSHOT facet (text 8 + binary 6): honest envelope+hex-payload shape,
# matching the real `ArtifactDsl`/`ArtifactPack` impl (wrap_text/wrap_binary around a
# serde_json-encoded payload) — same convention docx's own snapshot-level grammar leaves use.
# ---------------------------------------------------------------------------
SNAP_ENVELOPE_ID = "s.stdio.semio.presentation"

w("📸️snapshot/📝️text/📖️component.grammar.semio", f"""dialect grammar stdio.semio.presentation.snapshot
root = document
document = header body
header = 'schema' SP '{SNAP_ENVELOPE_ID}' NL
body = payload NL?
; payload is the hex encoding of the serde_json-serialized SemioPresentationSnapshot -- the
; STRUCTURED shape of that payload is normatively described by the sibling 🔣️component.json
; JSON Schema, not re-derived here (this grammar describes the on-disk TEXT ENVELOPE only).
payload = *OCTET
""")

w("📸️snapshot/📝️text/🅰️component.g4", """grammar Semio_semio_presentation_snapshot;
document: header body EOF;
header: 'schema' WS 'stdio.semio.presentation.snapshot' NL;
body: PAYLOAD NL?;
WS: ' ';
NL: '\\n';
PAYLOAD: [0-9a-f]* ; // hex(serde_json(SemioPresentationSnapshot)) -- see sibling JSON Schema
""")

w("📸️snapshot/📝️text/🔤️component.ebnf", """(* SemioPresentationSnapshot text envelope. *)
document = header , body ;
header   = "schema" , " " , "stdio.semio.presentation.snapshot" , newline ;
body     = payload , [ newline ] ;
payload  = { hexdigit } ;
hexdigit = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "a" | "b" | "c" | "d" | "e" | "f" ;
newline  = "\\n" ;
""")

w("📸️snapshot/📝️text/🔗️component.graphql", """# Text-envelope shape descriptor (not the payload's own structured schema -- see snapshot/component.graphql).
type SnapshotTextEnvelope { header: String!, hexPayload: String! }
""")

w("📸️snapshot/📝️text/🔣️component.json", """{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "SemioPresentationSnapshotTextEnvelope",
  "type": "object",
  "required": ["header", "hexPayload"],
  "properties": {
    "header": { "const": "schema stdio.semio.presentation.snapshot" },
    "hexPayload": { "type": "string", "pattern": "^[0-9a-f]*$" }
  }
}
""")

w("📸️snapshot/📝️text/🛰️component.proto", """// Text-envelope shape descriptor (structured payload is snapshot/component.proto).
syntax = "proto3";
package stdio.semio.presentation.snapshot.text;
message Envelope { string header = 1; string hex_payload = 2; }
""")

w("📸️snapshot/📝️text/🟦️component.ts", """/** Text-envelope shape descriptor. */
export interface SnapshotTextEnvelope { header: "schema stdio.semio.presentation.snapshot"; hexPayload: string; }
""")

w("📸️snapshot/📝️text/🦀️component.rs", """//! 📝️ Text representation codec surface for `stdio.semio.presentation` (snapshot).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
""")

w("📸️snapshot/💾️binary/🥋️component.ksy", """meta:
  id: semio_presentation_snapshot
  endian: le
doc: |
  Binary envelope for `stdio.semio.semio.presentation` snapshots: `SemioEnvelope` header
  (component=pack, version) followed by the serde_json-serialized `SemioPresentationSnapshot`
  payload verbatim (opaque at this binary-envelope level -- see the sibling JSON Schema for the
  payload's own structured shape).
seq:
  - id: envelope_id_len
    type: u4
  - id: envelope_id
    type: str
    size: envelope_id_len
    encoding: UTF-8
  - id: component_tag
    type: u1
  - id: version
    type: u4
  - id: payload_len
    type: u8
  - id: payload
    size: payload_len
""")

w("📸️snapshot/💾️binary/🌶️component.spicy", """module SemioPresentationSnapshot;

public type Envelope = unit {
    envelope_id_len: uint32;
    envelope_id: bytes &size=self.envelope_id_len;
    component_tag: uint8;
    version: uint32;
    payload_len: uint64;
    payload: bytes &size=self.payload_len; # opaque serde_json bytes -- see snapshot/component.json
};
""")

w("📸️snapshot/💾️binary/📡️component.protocol.semio", """protocol stdio.semio.presentation.snapshot.binary
envelope = envelope_id_len:u32 envelope_id:bytes[envelope_id_len] component:u8 version:u32 payload_len:u64 payload:bytes[payload_len]
; payload = serde_json(SemioPresentationSnapshot) verbatim
""")

w("📸️snapshot/💾️binary/🔠️component.abnf", """envelope    = envelope-id-len envelope-id component-tag version payload-len payload
envelope-id-len = 4OCTET  ; u32 LE
envelope-id     = *OCTET  ; UTF-8, envelope-id-len bytes
component-tag   = OCTET
version         = 4OCTET  ; u32 LE
payload-len     = 8OCTET  ; u64 LE
payload         = *OCTET  ; serde_json(SemioPresentationSnapshot), payload-len bytes
""")

w("📸️snapshot/💾️binary/🟦️component.ts", """/** Binary envelope shape descriptor (payload is opaque JSON bytes at this level). */
export interface SnapshotBinaryEnvelope { envelopeId: string; componentTag: number; version: number; payload: Uint8Array; }
""")

w("📸️snapshot/💾️binary/🦀️component.rs", """//! 💾️ Binary representation codec surface for `stdio.semio.presentation` (snapshot).

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
""")

print("snapshot grammar leaves done")

# ---------------------------------------------------------------------------
# Grammar leaves — DIFF facet: real hand-rolled `masters=... layouts=... slides=...` token
# grammar (matches `print_presentation_diff`/`parse_presentation_diff` exactly) — NOT an
# `*OCTET` catch-all, per this ticket's grammar-honesty requirement.
# ---------------------------------------------------------------------------
w("🔺️diff/📝️text/📖️component.grammar.semio", """dialect grammar stdio.semio.presentation.diff
root = line
line = [token (SP token)*]
token = "masters=" triple | "layouts=" triple | "slides=" triple
; triple is the shared `[removed];[modified];[added]` shape every IndexedTripleDiff/
; NamedTripleDiff instantiation in this subset uses (bracket-depth-aware, hex-encoded strings).
triple = "[" csv-list "];[" kv-list "];[" csv-or-list "]"
csv-list = [item ("," item)*]
kv-list = [kv ("," kv)*]
kv = key ":" value
csv-or-list = [value ("," value)*]
item = *OCTET
key = *DIGIT
value = *OCTET
""")

w("🔺️diff/📝️text/🅰️component.g4", """grammar Semio_semio_presentation_diff;
line: (token (WS token)*)? EOF;
token: MASTERS_EQ triple | LAYOUTS_EQ triple | SLIDES_EQ triple;
MASTERS_EQ: 'masters=';
LAYOUTS_EQ: 'layouts=';
SLIDES_EQ: 'slides=';
triple: '[' list ']' ';' '[' list ']' ';' '[' list ']';
list: (ITEM (',' ITEM)*)?;
ITEM: (~[,;\\[\\]])+;
WS: ' ';
""")

w("🔺️diff/📝️text/🔤️component.ebnf", """(* SemioPresentationDiff token grammar. *)
line   = [ token , { " " , token } ] ;
token  = "masters=" , triple | "layouts=" , triple | "slides=" , triple ;
triple = "[" , list , "];[" , list , "];[" , list , "]" ;
list   = [ item , { "," , item } ] ;
item   = { ? any char except , ; [ ] ? } ;
""")

w("🔺️diff/📝️text/🔗️component.graphql", """type PresentationDiffToken { key: String!, tripleRaw: String! }
type SemioPresentationDiffLine { tokens: [PresentationDiffToken!]! }
""")

w("🔺️diff/📝️text/🔣️component.json", """{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "SemioPresentationDiffLine",
  "type": "object",
  "properties": {
    "tokens": {
      "type": "array",
      "items": { "type": "object", "properties": { "key": { "enum": ["masters", "layouts", "slides"] }, "tripleRaw": { "type": "string" } } }
    }
  }
}
""")

w("🔺️diff/📝️text/🛰️component.proto", """syntax = "proto3";
package stdio.semio.presentation.diff.text;
message Token { string key = 1; string triple_raw = 2; }
message Line { repeated Token tokens = 1; }
""")

w("🔺️diff/📝️text/🟦️component.ts", """export interface PresentationDiffToken { key: "masters" | "layouts" | "slides"; tripleRaw: string; }
""")

w("🔺️diff/📝️text/🦀️component.rs", """//! 📝️ Text representation codec surface for `stdio.semio.presentation` (diff).

pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
""")

w("🔺️diff/💾️binary/🥋️component.ksy", """meta:
  id: semio_presentation_diff
doc: |
  Binary = the UTF-8 bytes of the diff facet's own text grammar (space-separated
  `masters=[...]  layouts=[...] slides=[...]` tokens) verbatim -- the same simplification the
  hand-rolled `protocol::DiffCodec::encode_diff` impl uses (`self.print_diff().into_bytes()`).
seq:
  - id: line_utf8
    type: str
    size-eos: true
    encoding: UTF-8
""")

w("🔺️diff/💾️binary/🌶️component.spicy", """module SemioPresentationDiff;
public type Binary = unit {
    line: bytes &eod; # UTF-8 text of the diff grammar, verbatim
};
""")

w("🔺️diff/💾️binary/📡️component.protocol.semio", """protocol stdio.semio.presentation.diff.binary
binary = line:utf8
; line is exactly the diff facet's own 📝️text grammar output
""")

w("🔺️diff/💾️binary/🔠️component.abnf", """binary = line
line   = *OCTET  ; UTF-8 bytes of the 📝️text facet's grammar output, verbatim
""")

w("🔺️diff/💾️binary/🟦️component.ts", """export type DiffBinary = Uint8Array; // UTF-8 bytes of the text-facet grammar output
""")

w("🔺️diff/💾️binary/🦀️component.rs", """//! 💾️ Binary representation codec surface for `stdio.semio.presentation` (diff).

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
""")

print("diff grammar leaves done")

# ---------------------------------------------------------------------------
# Grammar leaves — MUTATIONS facet: real hand-rolled `keyword arg=value ...` token grammar
# (matches `print_presentation_mutation`/`parse_presentation_mutation` exactly).
# ---------------------------------------------------------------------------
KEYWORDS = ["no-mutation", "set-snapshot", "insert-slide", "remove-slide", "set-slide-layout", "set-slide-notes",
            "insert-shape", "remove-shape", "set-shape-frame", "set-textbox-blocks",
            "insert-master", "remove-master", "insert-layout", "remove-layout", "set-layout-master"]
kw_alt = " | ".join(f'"{k}"' for k in KEYWORDS)

w("🧬️mutations/📝️text/📖️component.grammar.semio", f"""dialect grammar stdio.semio.presentation.mutations
root = line
line = "no-mutation" | keyword (SP arg)*
keyword = {kw_alt}
arg = name "=" value
name = 1*(ALPHA / "-")
value = *OCTET
""")

w("🧬️mutations/📝️text/🅰️component.g4", """grammar Semio_semio_presentation_mutations;
line: NO_MUTATION | KEYWORD (WS arg)*;
arg: NAME '=' VALUE;
NO_MUTATION: 'no-mutation';
KEYWORD: [a-z-]+;
NAME: [a-z-]+;
VALUE: (~[ ])*;
WS: ' ';
""")

ebnf_mutations = (
    "(* SemioPresentationMutation token grammar. *)\n"
    "line    = \"no-mutation\" | keyword , { \" \" , arg } ;\n"
    f"keyword = {kw_alt} ;\n"
    "arg     = name , \"=\" , value ;\n"
    "name    = { letter | \"-\" } ;\n"
    "value   = { ? any char except space ? } ;\n"
)
w("🧬️mutations/📝️text/🔤️component.ebnf", ebnf_mutations)

w("🧬️mutations/📝️text/🔗️component.graphql", """enum PresentationMutationKeyword {
  NO_MUTATION, SET_SNAPSHOT, INSERT_SLIDE, REMOVE_SLIDE, SET_SLIDE_LAYOUT, SET_SLIDE_NOTES,
  INSERT_SHAPE, REMOVE_SHAPE, SET_SHAPE_FRAME, SET_TEXTBOX_BLOCKS,
  INSERT_MASTER, REMOVE_MASTER, INSERT_LAYOUT, REMOVE_LAYOUT, SET_LAYOUT_MASTER
}
type PresentationMutationArg { name: String!, value: String! }
type SemioPresentationMutationLine { keyword: PresentationMutationKeyword!, args: [PresentationMutationArg!]! }
""")

w("🧬️mutations/📝️text/🔣️component.json", f"""{{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "SemioPresentationMutationLine",
  "type": "object",
  "required": ["keyword"],
  "properties": {{
    "keyword": {{ "enum": {json.dumps(KEYWORDS)} }},
    "args": {{ "type": "array", "items": {{ "type": "object", "properties": {{ "name": {{ "type": "string" }}, "value": {{ "type": "string" }} }} }} }}
  }}
}}
""")

w("🧬️mutations/📝️text/🛰️component.proto", """syntax = "proto3";
package stdio.semio.presentation.mutations.text;
message Arg { string name = 1; string value = 2; }
message Line { string keyword = 1; repeated Arg args = 2; }
""")

w("🧬️mutations/📝️text/🟦️component.ts", f"""export type PresentationMutationKeyword =
  {" | ".join(repr(k) for k in KEYWORDS)};
export interface PresentationMutationArg {{ name: string; value: string; }}
""")

w("🧬️mutations/📝️text/🦀️component.rs", """//! 📝️ Text representation codec surface for `stdio.semio.presentation` (mutations).

pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
""")

w("🧬️mutations/💾️binary/🥋️component.ksy", """meta:
  id: semio_presentation_mutations
doc: |
  Binary = the UTF-8 bytes of the mutations facet's own text grammar (`keyword arg=value ...`)
  verbatim, same simplification `protocol::OpBinary::encode_op` uses (`self.print_op().into_bytes()`).
seq:
  - id: line_utf8
    type: str
    size-eos: true
    encoding: UTF-8
""")

w("🧬️mutations/💾️binary/🌶️component.spicy", """module SemioPresentationMutations;
public type Binary = unit {
    line: bytes &eod; # UTF-8 text of the mutation grammar, verbatim
};
""")

w("🧬️mutations/💾️binary/📡️component.protocol.semio", """protocol stdio.semio.presentation.mutations.binary
binary = line:utf8
; line is exactly the mutations facet's own 📝️text grammar output
""")

w("🧬️mutations/💾️binary/🔠️component.abnf", """binary = line
line   = *OCTET  ; UTF-8 bytes of the 📝️text facet's grammar output, verbatim
""")

w("🧬️mutations/💾️binary/🟦️component.ts", """export type MutationBinary = Uint8Array; // UTF-8 bytes of the text-facet grammar output
""")

w("🧬️mutations/💾️binary/🦀️component.rs", """//! 💾️ Binary representation codec surface for `stdio.semio.presentation` (mutations).

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
""")

print("mutations grammar leaves done")
print("ALL DONE")
