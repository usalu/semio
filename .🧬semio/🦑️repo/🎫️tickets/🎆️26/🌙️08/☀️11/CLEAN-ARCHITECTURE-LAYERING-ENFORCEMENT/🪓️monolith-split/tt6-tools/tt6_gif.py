"""🎞️ Regenerates the GIF 87a and 89a diff facets (JSON Schema, GraphQL, protobuf, TypeScript interface and parser) and their snapshot GraphQL and protobuf from the hand-owned Rust diff model. usage: python3 tt6_gif.py"""
import json, re

R = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/"
SPEC = {
    "87a": dict(base=R + "7️⃣87a/🪆️subsets/✳️any/🧬️schema/", sub="any", frames="image", Frame="Image"),
    "89a": dict(base=R + "9️⃣89a/🪆️subsets/🧱️base/🧬️schema/", sub="base", frames="frame", Frame="Frame"),
}
URL = "https://json.schemas.assets.semio-tech.com/s/stdio/gif/{v}/{sub}/"


def write(path, text):
    open(path, "w", encoding="utf8", newline="").write(text)


def guard_block(path):
    t = open(path, encoding="utf8").read()
    i = t.index("//#region 🚪️Parsers")
    j = t.index("//#endregion 🚪️Parsers") + len("//#endregion 🚪️Parsers")
    block = t[i:j]
    prefix = re.search(r"export class (\w+)GuardRefusal", block).group(1)
    return block, prefix


def schema_json(v, sub, frames):
    u = URL.format(v=v, sub=sub)
    snap = u + "snapshot.json"
    def sref(name):
        return {"$ref": f"{snap}#/definitions/{name}"}
    def nullable(r):
        return {"anyOf": [{"type": "null"}, r]}
    art = lambda d: {**d, "x-semio-state": "artifact"}
    defs = {}
    if v == "87a":
        defs["GifImageDiff"] = {"type": "object", "additionalProperties": False, "properties": {
            "left": sref("u32"), "top": sref("u32"), "width": sref("u32"), "height": sref("u32"), "interlace": {"type": "boolean"},
            "lct": nullable(sref("colorTable")), "indices": {"type": "array", "items": sref("byte")}}}
        defs["GifImageModified"] = {"type": "object", "additionalProperties": False, "required": ["index", "diff"], "properties": {"index": {"type": "integer", "minimum": 0}, "diff": {"$ref": "#/$defs/GifImageDiff"}}}
        defs["GifImageAdded"] = {"type": "object", "additionalProperties": False, "required": ["index", "image"], "properties": {"index": {"type": "integer", "minimum": 0}, "image": sref("image")}}
        defs["GifImagesDiff"] = {"type": "object", "additionalProperties": False, "properties": {
            "removed": {"type": "array", "items": {"type": "integer", "minimum": 0}},
            "modified": {"type": "array", "items": {"$ref": "#/$defs/GifImageModified"}},
            "added": {"type": "array", "items": {"$ref": "#/$defs/GifImageAdded"}}}}
        props = {"width": art(sref("u32")), "height": art(sref("u32")), "gct": art(nullable(sref("colorTable"))), "backgroundColorIndex": art(sref("byte")),
                 "pixelAspectRatio": art(sref("byte")), "images": art({"$ref": "#/$defs/GifImagesDiff"})}
    else:
        defs["GifFrameDiff"] = {"type": "object", "additionalProperties": False, "properties": {
            "left": sref("u32"), "top": sref("u32"), "width": sref("u32"), "height": sref("u32"), "interlace": {"type": "boolean"},
            "lct": nullable(sref("colorTable")), "indices": {"type": "array", "items": sref("byte")}, "delayCs": sref("u16"),
            "disposal": {"enum": ["unspecified", "doNotDispose", "restoreToBackground", "restoreToPrevious"]},
            "transparentIndex": nullable(sref("byte")), "userInput": {"type": "boolean"}, "plainText": nullable(sref("plainText"))}}
        defs["GifFrameModified"] = {"type": "object", "additionalProperties": False, "required": ["index", "diff"], "properties": {"index": {"type": "integer", "minimum": 0}, "diff": {"$ref": "#/$defs/GifFrameDiff"}}}
        defs["GifFrameAdded"] = {"type": "object", "additionalProperties": False, "required": ["index", "frame"], "properties": {"index": {"type": "integer", "minimum": 0}, "frame": sref("frame")}}
        defs["GifFramesDiff"] = {"type": "object", "additionalProperties": False, "properties": {
            "removed": {"type": "array", "items": {"type": "integer", "minimum": 0}},
            "modified": {"type": "array", "items": {"$ref": "#/$defs/GifFrameModified"}},
            "added": {"type": "array", "items": {"$ref": "#/$defs/GifFrameAdded"}}}}
        defs["GifCommentModified"] = {"type": "object", "additionalProperties": False, "required": ["index", "text"], "properties": {"index": {"type": "integer", "minimum": 0}, "text": {"type": "string"}}}
        defs["GifCommentAdded"] = {"type": "object", "additionalProperties": False, "required": ["index", "text"], "properties": {"index": {"type": "integer", "minimum": 0}, "text": {"type": "string"}}}
        defs["GifCommentsDiff"] = {"type": "object", "additionalProperties": False, "properties": {
            "removed": {"type": "array", "items": {"type": "integer", "minimum": 0}},
            "modified": {"type": "array", "items": {"$ref": "#/$defs/GifCommentModified"}},
            "added": {"type": "array", "items": {"$ref": "#/$defs/GifCommentAdded"}}}}
        defs["GifAppExtensionModified"] = {"type": "object", "additionalProperties": False, "required": ["index", "extension"], "properties": {"index": {"type": "integer", "minimum": 0}, "extension": sref("application")}}
        defs["GifAppExtensionAdded"] = {"type": "object", "additionalProperties": False, "required": ["index", "extension"], "properties": {"index": {"type": "integer", "minimum": 0}, "extension": sref("application")}}
        defs["GifAppExtensionsDiff"] = {"type": "object", "additionalProperties": False, "properties": {
            "removed": {"type": "array", "items": {"type": "integer", "minimum": 0}},
            "modified": {"type": "array", "items": {"$ref": "#/$defs/GifAppExtensionModified"}},
            "added": {"type": "array", "items": {"$ref": "#/$defs/GifAppExtensionAdded"}}}}
        props = {"width": art(sref("u32")), "height": art(sref("u32")), "gct": art(nullable(sref("colorTable"))), "backgroundColorIndex": art(sref("byte")),
                 "pixelAspectRatio": art(sref("byte")), "loopCount": art(nullable(sref("u16"))), "frames": art({"$ref": "#/$defs/GifFramesDiff"}),
                 "comments": art({"$ref": "#/$defs/GifCommentsDiff"}), "appExtensions": art({"$ref": "#/$defs/GifAppExtensionsDiff"})}
    doc = {"$schema": "http://json-schema.org/draft-07/schema#", "$id": u + "diff.json", "title": "GifDiff", "type": "object", "additionalProperties": False, "properties": props, "$defs": defs}
    return doc


def graphql(v):
    snap = ""
    out = ["type GifImageDiff { left: Int top: Int width: Int height: Int interlace: Boolean lct: GifColorTable indices: [Int!] }",
           "type GifImageModified { index: Int! diff: GifImageDiff! }",
           "type GifImageAdded { index: Int! image: GifImage! }",
           "type GifImagesDiff { removed: [Int!] modified: [GifImageModified!] added: [GifImageAdded!] }",
           "type GifDiff { width: Int height: Int gct: GifColorTable backgroundColorIndex: Int pixelAspectRatio: Int images: GifImagesDiff }"]
    if v == "89a":
        out = ["enum GifDisposal { UNSPECIFIED DO_NOT_DISPOSE RESTORE_TO_BACKGROUND RESTORE_TO_PREVIOUS }",
               "type GifFrameDiff { left: Int top: Int width: Int height: Int interlace: Boolean lct: GifColorTable indices: [Int!] delayCs: Int disposal: GifDisposal transparentIndex: Int userInput: Boolean plainText: GifPlainText }",
               "type GifFrameModified { index: Int! diff: GifFrameDiff! }",
               "type GifFrameAdded { index: Int! frame: GifFrame! }",
               "type GifFramesDiff { removed: [Int!] modified: [GifFrameModified!] added: [GifFrameAdded!] }",
               "type GifCommentModified { index: Int! text: String! }",
               "type GifCommentAdded { index: Int! text: String! }",
               "type GifCommentsDiff { removed: [Int!] modified: [GifCommentModified!] added: [GifCommentAdded!] }",
               "type GifAppExtensionModified { index: Int! extension: GifAppExtension! }",
               "type GifAppExtensionAdded { index: Int! extension: GifAppExtension! }",
               "type GifAppExtensionsDiff { removed: [Int!] modified: [GifAppExtensionModified!] added: [GifAppExtensionAdded!] }",
               "type GifDiff { width: Int height: Int gct: GifColorTable backgroundColorIndex: Int pixelAspectRatio: Int loopCount: Int frames: GifFramesDiff comments: GifCommentsDiff appExtensions: GifAppExtensionsDiff }"]
    return "\n".join(out) + "\n"


def proto(v):
    h = 'syntax = "proto3";\npackage semio.s_stdio_gif.diff;\nimport "s_stdio_gif/snapshot.proto";\n\n'
    S = "semio.s_stdio_gif.snapshot."
    m = [f"message GifColorTableSlot {{ optional {S}GifColorTable value = 1; }}"]
    if v == "87a":
        m += [f"message GifImageDiff {{ optional uint32 left = 1; optional uint32 top = 2; optional uint32 width = 3; optional uint32 height = 4; optional bool interlace = 5; optional GifColorTableSlot lct = 6; optional bytes indices = 7; }}",
              "message GifImageModified { uint32 index = 1; GifImageDiff diff = 2; }",
              f"message GifImageAdded {{ uint32 index = 1; {S}GifImage image = 2; }}",
              "message GifImagesDiff { repeated uint32 removed = 1; repeated GifImageModified modified = 2; repeated GifImageAdded added = 3; }",
              "message GifDiff { optional uint32 width = 1; optional uint32 height = 2; optional GifColorTableSlot gct = 3; optional uint32 background_color_index = 4; optional uint32 pixel_aspect_ratio = 5; optional GifImagesDiff images = 6; }"]
    else:
        m += [f"message GifTransparentIndexSlot {{ optional uint32 value = 1; }}",
              f"message GifPlainTextSlot {{ optional {S}GifPlainText value = 1; }}",
              f"message GifLoopCountSlot {{ optional uint32 value = 1; }}",
              f"message GifFrameDiff {{ optional uint32 left = 1; optional uint32 top = 2; optional uint32 width = 3; optional uint32 height = 4; optional bool interlace = 5; optional GifColorTableSlot lct = 6; optional bytes indices = 7; optional uint32 delay_cs = 8; optional {S}GifDisposal disposal = 9; optional GifTransparentIndexSlot transparent_index = 10; optional bool user_input = 11; optional GifPlainTextSlot plain_text = 12; }}",
              "message GifFrameModified { uint32 index = 1; GifFrameDiff diff = 2; }",
              f"message GifFrameAdded {{ uint32 index = 1; {S}GifFrame frame = 2; }}",
              "message GifFramesDiff { repeated uint32 removed = 1; repeated GifFrameModified modified = 2; repeated GifFrameAdded added = 3; }",
              "message GifCommentModified { uint32 index = 1; string text = 2; }",
              "message GifCommentAdded { uint32 index = 1; string text = 2; }",
              "message GifCommentsDiff { repeated uint32 removed = 1; repeated GifCommentModified modified = 2; repeated GifCommentAdded added = 3; }",
              f"message GifAppExtensionModified {{ uint32 index = 1; {S}GifAppExtension extension = 2; }}",
              f"message GifAppExtensionAdded {{ uint32 index = 1; {S}GifAppExtension extension = 2; }}",
              "message GifAppExtensionsDiff { repeated uint32 removed = 1; repeated GifAppExtensionModified modified = 2; repeated GifAppExtensionAdded added = 3; }",
              "message GifDiff { optional uint32 width = 1; optional uint32 height = 2; optional GifColorTableSlot gct = 3; optional uint32 background_color_index = 4; optional uint32 pixel_aspect_ratio = 5; optional GifLoopCountSlot loop_count = 6; optional GifFramesDiff frames = 7; optional GifCommentsDiff comments = 8; optional GifAppExtensionsDiff app_extensions = 9; }"]
    return h + "\n".join(m) + "\n"


def snapshot_graphql(v):
    g = ["type GifRgb { r: Int! g: Int! b: Int! }", "type GifColorTable { sorted: Boolean! colors: [GifRgb!]! }"]
    if v == "87a":
        g += ["type GifImage { left: Int! top: Int! width: Int! height: Int! interlace: Boolean! lct: GifColorTable indices: [Int!]! }",
              "type GifSnapshot {\n  schema: String! @state(class: ARTIFACT)\n  width: Int! @state(class: ARTIFACT)\n  height: Int! @state(class: ARTIFACT)\n  gct: GifColorTable @state(class: ARTIFACT)\n  backgroundColorIndex: Int! @state(class: ARTIFACT)\n  pixelAspectRatio: Int! @state(class: ARTIFACT)\n  images: [GifImage!]! @state(class: ARTIFACT)\n}"]
    else:
        g += ["enum GifDisposal { UNSPECIFIED DO_NOT_DISPOSE RESTORE_TO_BACKGROUND RESTORE_TO_PREVIOUS }",
              "type GifPlainText { left: Int! top: Int! width: Int! height: Int! cellWidth: Int! cellHeight: Int! fgColorIndex: Int! bgColorIndex: Int! text: String! }",
              "type GifAppExtension { identifier: [Int!]! authCode: [Int!]! data: [Int!]! }",
              "type GifFrame { left: Int! top: Int! width: Int! height: Int! interlace: Boolean! lct: GifColorTable indices: [Int!]! delayCs: Int! disposal: GifDisposal! transparentIndex: Int userInput: Boolean! plainText: GifPlainText }",
              "type GifSnapshot {\n  schema: String! @state(class: ARTIFACT)\n  width: Int! @state(class: ARTIFACT)\n  height: Int! @state(class: ARTIFACT)\n  gct: GifColorTable @state(class: ARTIFACT)\n  backgroundColorIndex: Int! @state(class: ARTIFACT)\n  pixelAspectRatio: Int! @state(class: ARTIFACT)\n  loopCount: Int @state(class: ARTIFACT)\n  frames: [GifFrame!]! @state(class: ARTIFACT)\n  comments: [String!]! @state(class: ARTIFACT)\n  appExtensions: [GifAppExtension!]! @state(class: ARTIFACT)\n}"]
    return "\n".join(g) + "\n"


def snapshot_proto(v):
    h = 'syntax = "proto3";\npackage semio.s_stdio_gif.snapshot;\n\n'
    m = ["message GifRgb { uint32 r = 1; uint32 g = 2; uint32 b = 3; }", "message GifColorTable { bool sorted = 1; repeated GifRgb colors = 2; }"]
    if v == "87a":
        m += ["message GifImage { uint32 left = 1; uint32 top = 2; uint32 width = 3; uint32 height = 4; bool interlace = 5; optional GifColorTable lct = 6; bytes indices = 7; }",
              "message GifSnapshot { string schema = 1; uint32 width = 2; uint32 height = 3; optional GifColorTable gct = 4; uint32 background_color_index = 5; uint32 pixel_aspect_ratio = 6; repeated GifImage images = 7; }"]
    else:
        m += ["enum GifDisposal { GIF_DISPOSAL_UNSPECIFIED = 0; GIF_DISPOSAL_DO_NOT_DISPOSE = 1; GIF_DISPOSAL_RESTORE_TO_BACKGROUND = 2; GIF_DISPOSAL_RESTORE_TO_PREVIOUS = 3; }",
              "message GifPlainText { uint32 left = 1; uint32 top = 2; uint32 width = 3; uint32 height = 4; uint32 cell_width = 5; uint32 cell_height = 6; uint32 fg_color_index = 7; uint32 bg_color_index = 8; string text = 9; }",
              "message GifAppExtension { bytes identifier = 1; bytes auth_code = 2; bytes data = 3; }",
              "message GifFrame { uint32 left = 1; uint32 top = 2; uint32 width = 3; uint32 height = 4; bool interlace = 5; optional GifColorTable lct = 6; bytes indices = 7; uint32 delay_cs = 8; GifDisposal disposal = 9; optional uint32 transparent_index = 10; bool user_input = 11; optional GifPlainText plain_text = 12; }",
              "message GifSnapshot { string schema = 1; uint32 width = 2; uint32 height = 3; optional GifColorTable gct = 4; uint32 background_color_index = 5; uint32 pixel_aspect_ratio = 6; optional uint32 loop_count = 7; repeated GifFrame frames = 8; repeated string comments = 9; repeated GifAppExtension app_extensions = 10; }"]
    return h + "\n".join(m) + "\n"


def ts_file(v, guard, P):
    g = lambda kind: f"{P}Guard{kind}"
    head = ['/** 🔺️ Sparse GIF%s document delta over the owned snapshot: whole-value screen scalars, tri-state palette slots and index-keyed row triples (`removed` base indices, `modified` row deltas, `added` rows with their final index). */' % v]
    if v == "87a":
        head.append('import type { GifColorTable, GifImage } from "../📸️snapshot/🟦️.ts";')
        head.append("""
export interface GifImageDiff { left?: number; top?: number; width?: number; height?: number; interlace?: boolean; lct?: GifColorTable | null; indices?: number[] }
export interface GifImageModified { index: number; diff: GifImageDiff }
export interface GifImageAdded { index: number; image: GifImage }
export interface GifImagesDiff { removed?: number[]; modified?: GifImageModified[]; added?: GifImageAdded[] }
export interface GifDiff { width?: number; height?: number; gct?: GifColorTable | null; backgroundColorIndex?: number; pixelAspectRatio?: number; images?: GifImagesDiff }
""")
    else:
        head.append('import type { GifAppExtension, GifColorTable, GifDisposal, GifFrame, GifPlainText } from "../📸️snapshot/🟦️.ts";')
        head.append("""
export interface GifFrameDiff { left?: number; top?: number; width?: number; height?: number; interlace?: boolean; lct?: GifColorTable | null; indices?: number[]; delayCs?: number; disposal?: GifDisposal; transparentIndex?: number | null; userInput?: boolean; plainText?: GifPlainText | null }
export interface GifFrameModified { index: number; diff: GifFrameDiff }
export interface GifFrameAdded { index: number; frame: GifFrame }
export interface GifFramesDiff { removed?: number[]; modified?: GifFrameModified[]; added?: GifFrameAdded[] }
export interface GifCommentModified { index: number; text: string }
export interface GifCommentAdded { index: number; text: string }
export interface GifCommentsDiff { removed?: number[]; modified?: GifCommentModified[]; added?: GifCommentAdded[] }
export interface GifAppExtensionModified { index: number; extension: GifAppExtension }
export interface GifAppExtensionAdded { index: number; extension: GifAppExtension }
export interface GifAppExtensionsDiff { removed?: number[]; modified?: GifAppExtensionModified[]; added?: GifAppExtensionAdded[] }
export interface GifDiff { width?: number; height?: number; gct?: GifColorTable | null; backgroundColorIndex?: number; pixelAspectRatio?: number; loopCount?: number | null; frames?: GifFramesDiff; comments?: GifCommentsDiff; appExtensions?: GifAppExtensionsDiff }
""")
    head.append(guard)
    body = []
    body.append(f"""
const known = (row: Readonly<Record<string, unknown>>, at: string, keys: readonly string[]): void => {{
  for (const key of Object.keys(row)) if (!keys.includes(key)) throw new {P}GuardRefusal(`${{at}}.${{key}}`, "unknown property");
}};
const present = <T>(row: Readonly<Record<string, unknown>>, key: string, at: string, parse: (value: unknown, at: string) => T): T | undefined => (row[key] === undefined ? undefined : parse(row[key], `${{at}}.${{key}}`));
const nullable = <T>(parse: (value: unknown, at: string) => T) => (value: unknown, at: string): T | null => (value === null ? null : parse(value, at));
const byte = (value: unknown, at: string): number => {g('Integer')}(value, at, {{ minimum: 0, maximum: 255 }});
const word = (value: unknown, at: string): number => {g('Integer')}(value, at, {{ minimum: 0, maximum: 65535 }});
const dword = (value: unknown, at: string): number => {g('Integer')}(value, at, {{ minimum: 0, maximum: 4294967295 }});
const index = (value: unknown, at: string): number => {g('Integer')}(value, at, {{ minimum: 0 }});
const list = <T>(value: unknown, at: string, parse: (value: unknown, at: string) => T): T[] => {g('Array')}(value, at).map((item, position) => parse(item, `${{at}}[${{position}}]`));
const octets = (value: unknown, at: string): number[] => list(value, at, byte);
const exactOctets = (count: number) => (value: unknown, at: string): number[] => {{ const found = {g('Array')}(value, at, {{ minItems: count, maxItems: count }}); return found.map((item, position) => byte(item, `${{at}}[${{position}}]`)); }};
""")
    # shared snapshot-row parsers (snapshot module owns the types only)
    body.append(f"""
const rgb = (value: unknown, at: string): {{ r: number; g: number; b: number }} => {{ const row = {g('Object')}(value, at); known(row, at, ["r", "g", "b"]); return {{ r: byte(row["r"], `${{at}}.r`), g: byte(row["g"], `${{at}}.g`), b: byte(row["b"], `${{at}}.b`) }}; }};
const colorTable = (value: unknown, at: string): GifColorTable => {{ const row = {g('Object')}(value, at); known(row, at, ["sorted", "colors"]); return {{ sorted: {g('Boolean')}(row["sorted"], `${{at}}.sorted`), colors: list(row["colors"], `${{at}}.colors`, rgb) }}; }};
""")
    if v == "87a":
        body.append(f"""
const image = (value: unknown, at: string): GifImage => {{ const row = {g('Object')}(value, at); known(row, at, ["left", "top", "width", "height", "interlace", "lct", "indices"]); return {{ left: dword(row["left"], `${{at}}.left`), top: dword(row["top"], `${{at}}.top`), width: dword(row["width"], `${{at}}.width`), height: dword(row["height"], `${{at}}.height`), interlace: {g('Boolean')}(row["interlace"], `${{at}}.interlace`), lct: nullable(colorTable)(row["lct"], `${{at}}.lct`), indices: octets(row["indices"], `${{at}}.indices`) }}; }};

export function parseGifImageDiff(value: unknown, at = "$"): GifImageDiff {{
  const row = {g('Object')}(value, at);
  known(row, at, ["left", "top", "width", "height", "interlace", "lct", "indices"]);
  return {{
    left: present(row, "left", at, dword), top: present(row, "top", at, dword), width: present(row, "width", at, dword), height: present(row, "height", at, dword),
    interlace: present(row, "interlace", at, {g('Boolean')}), lct: present(row, "lct", at, nullable(colorTable)), indices: present(row, "indices", at, octets),
  }};
}}
export function parseGifImageModified(value: unknown, at = "$"): GifImageModified {{
  const row = {g('Object')}(value, at);
  known(row, at, ["index", "diff"]);
  return {{ index: index(row["index"], `${{at}}.index`), diff: parseGifImageDiff(row["diff"], `${{at}}.diff`) }};
}}
export function parseGifImageAdded(value: unknown, at = "$"): GifImageAdded {{
  const row = {g('Object')}(value, at);
  known(row, at, ["index", "image"]);
  return {{ index: index(row["index"], `${{at}}.index`), image: image(row["image"], `${{at}}.image`) }};
}}
export function parseGifImagesDiff(value: unknown, at = "$"): GifImagesDiff {{
  const row = {g('Object')}(value, at);
  known(row, at, ["removed", "modified", "added"]);
  return {{ removed: present(row, "removed", at, (items, where) => list(items, where, index)), modified: present(row, "modified", at, (items, where) => list(items, where, parseGifImageModified)), added: present(row, "added", at, (items, where) => list(items, where, parseGifImageAdded)) }};
}}
export function parseGifDiff(value: unknown, at = "$"): GifDiff {{
  const row = {g('Object')}(value, at);
  known(row, at, ["width", "height", "gct", "backgroundColorIndex", "pixelAspectRatio", "images"]);
  return {{
    width: present(row, "width", at, dword), height: present(row, "height", at, dword), gct: present(row, "gct", at, nullable(colorTable)),
    backgroundColorIndex: present(row, "backgroundColorIndex", at, byte), pixelAspectRatio: present(row, "pixelAspectRatio", at, byte), images: present(row, "images", at, parseGifImagesDiff),
  }};
}}
""")
    else:
        body.append(f"""
const disposal = (value: unknown, at: string): GifDisposal => {g('Member')}(value, at, ["unspecified", "doNotDispose", "restoreToBackground", "restoreToPrevious"] as const);
const plainText = (value: unknown, at: string): GifPlainText => {{ const row = {g('Object')}(value, at); known(row, at, ["left", "top", "width", "height", "cellWidth", "cellHeight", "fgColorIndex", "bgColorIndex", "text"]); return {{ left: dword(row["left"], `${{at}}.left`), top: dword(row["top"], `${{at}}.top`), width: dword(row["width"], `${{at}}.width`), height: dword(row["height"], `${{at}}.height`), cellWidth: byte(row["cellWidth"], `${{at}}.cellWidth`), cellHeight: byte(row["cellHeight"], `${{at}}.cellHeight`), fgColorIndex: byte(row["fgColorIndex"], `${{at}}.fgColorIndex`), bgColorIndex: byte(row["bgColorIndex"], `${{at}}.bgColorIndex`), text: {g('String')}(row["text"], `${{at}}.text`) }}; }};
const application = (value: unknown, at: string): GifAppExtension => {{ const row = {g('Object')}(value, at); known(row, at, ["identifier", "authCode", "data"]); return {{ identifier: identifier(row["identifier"], `${{at}}.identifier`), authCode: authCode(row["authCode"], `${{at}}.authCode`), data: octets(row["data"], `${{at}}.data`) }}; }};
const frame = (value: unknown, at: string): GifFrame => {{ const row = {g('Object')}(value, at); known(row, at, ["left", "top", "width", "height", "interlace", "lct", "indices", "delayCs", "disposal", "transparentIndex", "userInput", "plainText"]); return {{ left: dword(row["left"], `${{at}}.left`), top: dword(row["top"], `${{at}}.top`), width: dword(row["width"], `${{at}}.width`), height: dword(row["height"], `${{at}}.height`), interlace: {g('Boolean')}(row["interlace"], `${{at}}.interlace`), lct: nullable(colorTable)(row["lct"], `${{at}}.lct`), indices: octets(row["indices"], `${{at}}.indices`), delayCs: word(row["delayCs"], `${{at}}.delayCs`), disposal: disposal(row["disposal"], `${{at}}.disposal`), transparentIndex: nullable(byte)(row["transparentIndex"], `${{at}}.transparentIndex`), userInput: {g('Boolean')}(row["userInput"], `${{at}}.userInput`), plainText: nullable(plainText)(row["plainText"], `${{at}}.plainText`) }}; }};
const identifier = (value: unknown, at: string): GifAppExtension["identifier"] => {{ const o = exactOctets(8)(value, at); return [o[0]!, o[1]!, o[2]!, o[3]!, o[4]!, o[5]!, o[6]!, o[7]!]; }};
const authCode = (value: unknown, at: string): GifAppExtension["authCode"] => {{ const o = exactOctets(3)(value, at); return [o[0]!, o[1]!, o[2]!]; }};

export function parseGifFrameDiff(value: unknown, at = "$"): GifFrameDiff {{
  const row = {g('Object')}(value, at);
  known(row, at, ["left", "top", "width", "height", "interlace", "lct", "indices", "delayCs", "disposal", "transparentIndex", "userInput", "plainText"]);
  return {{
    left: present(row, "left", at, dword), top: present(row, "top", at, dword), width: present(row, "width", at, dword), height: present(row, "height", at, dword),
    interlace: present(row, "interlace", at, {g('Boolean')}), lct: present(row, "lct", at, nullable(colorTable)), indices: present(row, "indices", at, octets), delayCs: present(row, "delayCs", at, word),
    disposal: present(row, "disposal", at, disposal), transparentIndex: present(row, "transparentIndex", at, nullable(byte)), userInput: present(row, "userInput", at, {g('Boolean')}), plainText: present(row, "plainText", at, nullable(plainText)),
  }};
}}
export function parseGifFrameModified(value: unknown, at = "$"): GifFrameModified {{
  const row = {g('Object')}(value, at);
  known(row, at, ["index", "diff"]);
  return {{ index: index(row["index"], `${{at}}.index`), diff: parseGifFrameDiff(row["diff"], `${{at}}.diff`) }};
}}
export function parseGifFrameAdded(value: unknown, at = "$"): GifFrameAdded {{
  const row = {g('Object')}(value, at);
  known(row, at, ["index", "frame"]);
  return {{ index: index(row["index"], `${{at}}.index`), frame: frame(row["frame"], `${{at}}.frame`) }};
}}
export function parseGifFramesDiff(value: unknown, at = "$"): GifFramesDiff {{
  const row = {g('Object')}(value, at);
  known(row, at, ["removed", "modified", "added"]);
  return {{ removed: present(row, "removed", at, (items, where) => list(items, where, index)), modified: present(row, "modified", at, (items, where) => list(items, where, parseGifFrameModified)), added: present(row, "added", at, (items, where) => list(items, where, parseGifFrameAdded)) }};
}}
export function parseGifCommentModified(value: unknown, at = "$"): GifCommentModified {{
  const row = {g('Object')}(value, at);
  known(row, at, ["index", "text"]);
  return {{ index: index(row["index"], `${{at}}.index`), text: {g('String')}(row["text"], `${{at}}.text`) }};
}}
export function parseGifCommentAdded(value: unknown, at = "$"): GifCommentAdded {{
  const row = {g('Object')}(value, at);
  known(row, at, ["index", "text"]);
  return {{ index: index(row["index"], `${{at}}.index`), text: {g('String')}(row["text"], `${{at}}.text`) }};
}}
export function parseGifCommentsDiff(value: unknown, at = "$"): GifCommentsDiff {{
  const row = {g('Object')}(value, at);
  known(row, at, ["removed", "modified", "added"]);
  return {{ removed: present(row, "removed", at, (items, where) => list(items, where, index)), modified: present(row, "modified", at, (items, where) => list(items, where, parseGifCommentModified)), added: present(row, "added", at, (items, where) => list(items, where, parseGifCommentAdded)) }};
}}
export function parseGifAppExtensionModified(value: unknown, at = "$"): GifAppExtensionModified {{
  const row = {g('Object')}(value, at);
  known(row, at, ["index", "extension"]);
  return {{ index: index(row["index"], `${{at}}.index`), extension: application(row["extension"], `${{at}}.extension`) }};
}}
export function parseGifAppExtensionAdded(value: unknown, at = "$"): GifAppExtensionAdded {{
  const row = {g('Object')}(value, at);
  known(row, at, ["index", "extension"]);
  return {{ index: index(row["index"], `${{at}}.index`), extension: application(row["extension"], `${{at}}.extension`) }};
}}
export function parseGifAppExtensionsDiff(value: unknown, at = "$"): GifAppExtensionsDiff {{
  const row = {g('Object')}(value, at);
  known(row, at, ["removed", "modified", "added"]);
  return {{ removed: present(row, "removed", at, (items, where) => list(items, where, index)), modified: present(row, "modified", at, (items, where) => list(items, where, parseGifAppExtensionModified)), added: present(row, "added", at, (items, where) => list(items, where, parseGifAppExtensionAdded)) }};
}}
export function parseGifDiff(value: unknown, at = "$"): GifDiff {{
  const row = {g('Object')}(value, at);
  known(row, at, ["width", "height", "gct", "backgroundColorIndex", "pixelAspectRatio", "loopCount", "frames", "comments", "appExtensions"]);
  return {{
    width: present(row, "width", at, dword), height: present(row, "height", at, dword), gct: present(row, "gct", at, nullable(colorTable)),
    backgroundColorIndex: present(row, "backgroundColorIndex", at, byte), pixelAspectRatio: present(row, "pixelAspectRatio", at, byte), loopCount: present(row, "loopCount", at, nullable(word)),
    frames: present(row, "frames", at, parseGifFramesDiff), comments: present(row, "comments", at, parseGifCommentsDiff), appExtensions: present(row, "appExtensions", at, parseGifAppExtensionsDiff),
  }};
}}
""")
    return "\n".join(head) + "\n" + "\n".join(body)


for v, s in SPEC.items():
    base = s["base"]
    guard, prefix = guard_block(base + "🔺️diff/🟦️.ts")
    write(base + "🔺️diff/🔣️.json", json.dumps(schema_json(v, s["sub"], s["frames"]), ensure_ascii=False, indent=2) + "\n")
    write(base + "🔺️diff/🔗️.graphql", graphql(v))
    write(base + "🔺️diff/🛰️.proto", proto(v))
    write(base + "🔺️diff/🟦️.ts", ts_file(v, guard, prefix))
    write(base + "📸️snapshot/🔗️.graphql", snapshot_graphql(v))
    write(base + "📸️snapshot/🛰️.proto", snapshot_proto(v))
    print("wrote", v)
