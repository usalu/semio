import { writeFileSync, existsSync, readFileSync } from "node:fs";
import { join } from "node:path";

const repoRoot = join(import.meta.dir, "../../../../../..");
const pluginsRoot = join(repoRoot, "✏️s/🔌️plugins");

/** @typedef {{ plugin: string, artifact: string, id: string, family: "scene"|"embed", doc: string, op: string, diff: string }} Spec */

const props = `props = "{" prop* "}"
prop = IDENT "=" (TEXT | FLOAT | INT | BOOL | list | map | NULL)
list = "[" value* "]"
map = "{" prop* "}"
value = TEXT | FLOAT | INT | BOOL | list | map | NULL
`;

function header(id, role, family) {
  const start = role === "document" ? "document" : role === "op" ? "operation" : "diff";
  return `dialect grammar
grammar ${id}.${role}
extension ${id}
use family-${family}
start ${start}

`;
}

function packProtocol(id) {
  return `dialect protocol
protocol ${id}.pack
version 1
framing magic 0x8953504B0D0A1A0A
header fixed 32
field format_major u16
field format_minor u16
field flags u32
field header_crc32 u32
segment kind u8
segment flags u8
segment payload varint bytes
record field id u16 type tag
field tag varint
field body bytes
footer fixed 84
`;
}

function sprProtocol(id) {
  return `dialect protocol
protocol ${id}.spr
version 1
framing record
field format u8
field ordinal varint
field body bytes
chain hash u64
`;
}

function grammarTs(id, facet) {
  return `/** 🧩 ${id} ${facet} WASM facade — parse/print delegates to the plugin Rust crate. */
export function parseDsl(text: string): unknown {
  throw new Error("wire ${id} ${facet} parseDsl to plugin WASM");
}
export function printDsl(value: unknown): string {
  throw new Error("wire ${id} ${facet} printDsl to plugin WASM");
}
`;
}

function protocolTs(id, facet) {
  return `/** 🧩 ${id} ${facet} WASM facade — encode/decode delegates to the plugin Rust crate. */
export function encode(value: unknown): Uint8Array {
  throw new Error("wire ${id} ${facet} encode to plugin WASM");
}
export function decode(bytes: Uint8Array): unknown {
  throw new Error("wire ${id} ${facet} decode to plugin WASM");
}
`;
}

/** Artifact-specific document grammars rooted in 🦀️component.rs field/keyword names. */
const specs = {
  raster: {
    plugin: "🖨️raster",
    artifact: "🖨️raster",
    id: "raster",
    family: "scene",
    document: `document = schema-line layers-block assets-map?
schema-line = "schema" "=" IDENT {"id" "=" IDENT}? {"title" "=" TEXT}?
layers-block = "layers" "{" layer* "}"
layer = pixel-layer | group-layer | adjustment-layer
pixel-layer = "pixel" layer-props transform-block? mask-block?
group-layer = "group" layer-props transform-block? children-block?
adjustment-layer = "adjustment" layer-props transform-block? params-map?
layer-props = {"id" "=" IDENT}? {"name" "=" TEXT}? {"visible" "=" BOOL}? {"opacity" "=" FLOAT}? {"blend" "=" IDENT}? {"width" "=" INT}? {"height" "=" INT}? {"image" "=" IDENT}? {"kind" "=" IDENT}?
transform-block = "transform" "{" prop* "}"
mask-block = "mask" "{" prop* "}"
children-block = "children" "{" layer* "}"
params-map = "params" "=" "{" prop* "}"
assets-map = "assets" "=" "{" asset* "}"
asset = IDENT "=" {"mime" "=" TEXT}? {"data" "=" TEXT}?
${props}`,
    operation: `operation = add-layer | remove-layer | patch-layer | move-layer | replace-document
add-layer = "add-layer" {"parent" "=" IDENT}? {"index" "=" INT}? layer
remove-layer = "remove-layer" "id" "=" IDENT
patch-layer = "patch-layer" "id" "=" IDENT patch-block
move-layer = "move-layer" "id" "=" IDENT {"parent" "=" IDENT}? {"index" "=" INT}?
replace-document = "replace-document" document-block
patch-block = "patch" "{" prop* "}"
document-block = "document" "{" prop* "}"
layer = pixel-layer | group-layer | adjustment-layer
pixel-layer = "pixel" prop*
group-layer = "group" prop*
adjustment-layer = "adjustment" prop*
${props}`,
    diff: `diff = step*
step = add-layer | remove-layer | patch-layer | move-layer | replace
add-layer = "add-layer" {"parent" "=" IDENT}? {"index" "=" INT}? layer
remove-layer = "remove-layer" "id" "=" IDENT
patch-layer = "patch-layer" "id" "=" IDENT "patch" "{" prop* "}"
move-layer = "move-layer" "id" "=" IDENT {"parent" "=" IDENT}? {"index" "=" INT}?
replace = "replace" "{" prop* "}"
layer = "pixel" prop* | "group" prop* | "adjustment" prop*
${props}`,
  },
  present: {
    plugin: "🎞️animate",
    artifact: "🎬️present",
    id: "present",
    family: "scene",
    document: `document = schema-line source-block tiles-table
schema-line = "schema" "=" IDENT
source-block = "source" "{" source-prop* frame-block? "}"
source-prop = {"src" "=" TEXT} | {"kind" "=" IDENT} | {"source-aspect" "=" FLOAT} | {"pdf-page" "=" INT} | prop
frame-block = "frame" "{" {"x" "=" FLOAT}? {"y" "=" FLOAT}? {"width" "=" FLOAT}? {"height" "=" FLOAT}? "}"
tiles-table = "tiles" "[" "id" ":" "TEXT" "name" ":" "TEXT" "crop" ":" "BLOCK" "]" "{" tile* "}"
tile = IDENT TEXT "{" {"x" "=" FLOAT}? {"y" "=" FLOAT}? {"width" "=" FLOAT}? {"height" "=" FLOAT}? "}"
${props}`,
    operation: `operation = tiles-add | tiles-remove | tiles-move | tiles-patch | set-source | set-tiles | set-deck
tiles-add = "tiles-add" {"index" "=" INT}? tile-block
tiles-remove = "tiles-remove" "id" "=" IDENT
tiles-move = "tiles-move" "id" "=" IDENT "to" "=" INT
tiles-patch = "tiles-patch" "id" "=" IDENT patch-block
set-source = "set-source" source-block
set-tiles = "set-tiles" tiles-table
set-deck = "set-deck" deck-block
tile-block = "tile" "{" prop* frame-block? "}"
source-block = "source" "{" prop* frame-block? "}"
frame-block = "frame" "{" prop* "}"
patch-block = "patch" "{" prop* frame-block? "}"
deck-block = "deck" "{" prop* "}"
tiles-table = "tiles" "{" tile-block* "}"
${props}`,
    diff: `diff = {"source" "=" source-block}? {"tiles" "=" collection-diff}?
source-block = "source" "{" prop* frame-block? "}"
frame-block = "frame" "{" prop* "}"
collection-diff = "{" {"added" "=" list}? {"removed" "=" list}? {"moved" "=" list}? {"modified" "=" list}? "}"
${props}`,
  },
  lowpoly: {
    plugin: "💠️lowpoly",
    artifact: "💠️lowpoly",
    id: "lowpoly",
    family: "scene",
    document: `document = schema-line objects-field
schema-line = "schema" "=" IDENT
objects-field = "objects" "=" "[" object* "]" | "objects" "=" object*
object = "{" object-prop* "}" | object-inline
object-inline = {"id" "=" IDENT} {"name" "=" TEXT}? {"smooth-shading" "=" BOOL}? {"mesh-json" "=" TEXT}? transform-block? paint-layers?
object-prop = {"id" "=" IDENT} | {"name" "=" TEXT} | {"smooth-shading" "=" BOOL} | {"mesh-json" "=" TEXT} | transform-block | paint-layers | prop
transform-block = "transform" "{" {"x" "=" FLOAT}? {"y" "=" FLOAT}? {"z" "=" FLOAT}? prop* "}"
paint-layers = "paint-layers" "=" "[" paint-layer* "]" | "paint-layers" "{" paint-layer* "}"
paint-layer = "{" {"name" "=" TEXT}? {"visible" "=" BOOL}? {"opacity" "=" FLOAT}? {"blend-mode" "=" IDENT}? {"pixels" "=" TEXT}? "}"
${props}`,
    operation: `operation = objects-add | objects-remove | objects-move | objects-patch | add-paint-layer | remove-paint-layer | patch-paint-layer | paint-stroke | set-projection
objects-add = "objects-add" {"index" "=" INT}? object-block
objects-remove = "objects-remove" "id" "=" IDENT
objects-move = "objects-move" "id" "=" IDENT "to" "=" INT
objects-patch = "objects-patch" "id" "=" IDENT patch-block
add-paint-layer = "add-paint-layer" "object-id" "=" IDENT {"index" "=" INT}? paint-layer-block
remove-paint-layer = "remove-paint-layer" "object-id" "=" IDENT "index" "=" INT
patch-paint-layer = "patch-paint-layer" "object-id" "=" IDENT "index" "=" INT patch-block
paint-stroke = "paint-stroke" "object-id" "=" IDENT "layer-index" "=" INT runs-table
set-projection = "set-projection" projection-block
object-block = "object" "{" prop* "}"
paint-layer-block = "paint-layer" "{" prop* "}"
patch-block = "patch" "{" prop* "}"
projection-block = "projection" "{" prop* "}"
runs-table = "runs" "{" run* "}"
run = "run" "offset" "=" INT "bytes" "=" TEXT
${props}`,
    diff: `diff = {"objects" "=" collection-diff}? {"projection" "=" projection-block}?
collection-diff = "{" {"added" "=" list}? {"removed" "=" list}? {"moved" "=" list}? {"modified" "=" list}? "}"
projection-block = "projection" "{" prop* "}"
${props}`,
  },
  draw: {
    plugin: "🖍️draw",
    artifact: "🖍️draw",
    id: "draw",
    family: "scene",
    document: `document = schema-line layers-block assets-map?
schema-line = "schema" "=" IDENT {"id" "=" IDENT}? {"title" "=" TEXT}?
layers-block = "layers" "{" layer* "}"
layer = shape-layer | path-layer | text-layer | image-layer | group-layer | boolean-layer | trace-layer
shape-layer = "shape" base-block geometry-block*
path-layer = "path" base-block segments-block?
text-layer = "text" base-block {"x" "=" FLOAT}? {"y" "=" FLOAT}? {"content" "=" TEXT}? {"size" "=" FLOAT}?
image-layer = "image" base-block {"image-key" "=" IDENT}? {"width" "=" FLOAT}? {"height" "=" FLOAT}?
group-layer = "group" base-block children-block?
boolean-layer = "boolean" base-block {"operation" "=" IDENT}? {"children" "=" list}?
trace-layer = "trace" base-block {"source-key" "=" IDENT}? params-block?
base-block = "base" "{" base-prop* transform-block? attributes-block? "}"
base-prop = {"id" "=" IDENT} | {"name" "=" TEXT} | {"visible" "=" BOOL} | {"locked" "=" BOOL} | {"opacity" "=" FLOAT} | {"blend-mode" "=" IDENT} | prop
transform-block = "transform" "{" prop* "}"
attributes-block = "attributes" "{" fill-block? stroke-block? "}"
fill-block = "fill" "{" fill-body* "}"
fill-body = "solid" prop* | "linear" prop* | "radial" prop* | prop
stroke-block = "stroke" "{" prop* "}"
geometry-block = "rect" "{" prop* "}" | "ellipse" "{" prop* "}" | "circle" "{" prop* "}" | "line" "{" prop* "}" | "polygon" "{" prop* "}" | {"shape-kind" "=" IDENT}
segments-block = "segments" "{" segment* "}"
segment = "M" FLOAT "," FLOAT | "L" FLOAT "," FLOAT | "Q" FLOAT "," FLOAT FLOAT "," FLOAT | "C" FLOAT "," FLOAT FLOAT "," FLOAT FLOAT "," FLOAT | "A" FLOAT "," FLOAT FLOAT BOOL BOOL FLOAT "," FLOAT | "Z"
children-block = "children" "{" layer* "}"
params-block = "params" "{" prop* "}"
assets-map = "assets" "=" "{" asset* "}"
asset = IDENT "=" {"mime" "=" TEXT}? {"data" "=" TEXT}?
${props}`,
    operation: `operation = set-layer-visible | set-layer-locked | set-layer-opacity | set-layer-blend-mode | set-layer-name | set-layer-transform | set-fill | set-stroke | set-boolean-operation | set-trace-params | add-layer | duplicate-layer | remove-layer | reorder-layer | set-document
set-layer-visible = "set-layer-visible" "layer-id" "=" IDENT "visible" "=" BOOL
set-layer-locked = "set-layer-locked" "layer-id" "=" IDENT "locked" "=" BOOL
set-layer-opacity = "set-layer-opacity" "layer-id" "=" IDENT "opacity" "=" FLOAT
set-layer-blend-mode = "set-layer-blend-mode" "layer-id" "=" IDENT "blend-mode" "=" IDENT
set-layer-name = "set-layer-name" "layer-id" "=" IDENT "name" "=" TEXT
set-layer-transform = "set-layer-transform" "layer-id" "=" IDENT transform-block
set-fill = "set-fill" "layer-id" "=" IDENT fill-block
set-stroke = "set-stroke" "layer-id" "=" IDENT stroke-block
set-boolean-operation = "set-boolean-operation" "layer-id" "=" IDENT "boolean-operation" "=" IDENT
set-trace-params = "set-trace-params" "layer-id" "=" IDENT params-block
add-layer = "add-layer" {"parent" "=" IDENT}? {"index" "=" INT}? layer
duplicate-layer = "duplicate-layer" "layer-id" "=" IDENT
remove-layer = "remove-layer" "layer-id" "=" IDENT
reorder-layer = "reorder-layer" "layer-id" "=" IDENT "index" "=" INT
set-document = "set-document" document-block
layer = "shape" prop* | "path" prop* | "text" prop* | "image" prop* | "group" prop* | "boolean" prop* | "trace" prop*
transform-block = "transform" "{" prop* "}"
fill-block = "fill" "{" prop* "}"
stroke-block = "stroke" "{" prop* "}"
params-block = "params" "{" prop* "}"
document-block = "document" "{" prop* "}"
${props}`,
    diff: `diff = layer-patch*
layer-patch = "layer-patch" "layer-id" "=" IDENT base-patch? tree-patch?
base-patch = "base" "{" {"name" "=" TEXT}? {"visible" "=" BOOL}? {"locked" "=" BOOL}? {"opacity" "=" FLOAT}? {"blend-mode" "=" IDENT}? prop* "}"
tree-patch = "tree" "{" {"add" "=" layer}? {"remove" "=" IDENT}? {"reorder" "=" INT}? "}"
layer = "shape" prop* | "path" prop* | "text" prop* | "image" prop* | "group" prop* | "boolean" prop* | "trace" prop*
${props}`,
  },
  layout: {
    plugin: "📏️layout",
    artifact: "📏️layout",
    id: "layout",
    family: "scene",
    document: `document = schema-line grid-block? character-styles? paragraph-styles? stories? links? parent-pages? spreads? pages? print-target?
schema-line = "schema" "=" IDENT {"name" "=" TEXT}? {"print-target" "=" IDENT}?
grid-block = "grid" "{" {"baseline-grid" "=" FLOAT}? {"baseline-offset" "=" FLOAT}? {"snap-to-baseline" "=" BOOL}? "}"
character-styles = "character-styles" "=" list | "character-styles" table-header "{" style-row* "}"
paragraph-styles = "paragraph-styles" table-header "{" style-row* "}"
stories = "stories" table-header "{" story-row* "}"
links = "links" "=" list | "links" table-header "{" link-row* "}"
parent-pages = "parent-pages" "=" "[" parent-page* "]"
spreads = "spreads" "=" list | "spreads" table-header "{" spread-row* "}"
pages = "pages" "=" "[" page* "]"
page = page-props margins-block? columns-block? layer-ids? frames-block? guides-table? layers-table? overrides-table?
page-props = {"id" "=" IDENT} {"name" "=" TEXT}? {"spread-id" "=" IDENT}? {"parent-page-id" "=" IDENT}? {"width" "=" FLOAT}? {"height" "=" FLOAT}?
parent-page = page-props frames-block? layers-table?
margins-block = "margins" "{" {"top" "=" FLOAT}? {"right" "=" FLOAT}? {"bottom" "=" FLOAT}? {"left" "=" FLOAT}? "}"
columns-block = "columns" "{" {"count" "=" INT}? {"gutter" "=" FLOAT}? "}"
layer-ids = "layer-ids" "=" list
frames-block = "frames" "{" frame* "}"
frame = frame-kind frame-props bounds-block? inset-block?
frame-kind = "rect" | "text" | "image"
frame-props = {"id" "=" IDENT} {"layer-id" "=" IDENT}? {"story-id" "=" IDENT}? {"thread-next" "=" IDENT}? {"link-id" "=" IDENT}? {"columns" "=" INT}? {"wrap-mode" "=" IDENT}? {"fill" "=" list}?
bounds-block = "bounds" "{" {"x" "=" FLOAT}? {"y" "=" FLOAT}? {"width" "=" FLOAT}? {"height" "=" FLOAT}? {"rotation" "=" FLOAT}? "}"
inset-block = "inset" "{" prop* "}"
guides-table = "guides" table-header "{" guide-row* "}"
layers-table = "layers" table-header "{" layer-row* "}"
overrides-table = "overrides" table-header "{" override-row* "}"
table-header = "[" IDENT ":" IDENT {IDENT ":" IDENT}* "]"
style-row = IDENT prop*
story-row = IDENT TEXT prop*
link-row = IDENT prop*
spread-row = IDENT prop*
guide-row = prop*
layer-row = IDENT TEXT BOOL BOOL list
override-row = prop*
print-target = "print-target" "=" IDENT | "print-target" "=" NULL
${props}`,
    operation: `operation = add-frame | remove-frame | patch-frame | set-data-fields
add-frame = "add-frame" "page-id" "=" IDENT {"index" "=" INT}? frame-block
remove-frame = "remove-frame" "page-id" "=" IDENT "frame-id" "=" IDENT
patch-frame = "patch-frame" "page-id" "=" IDENT "frame-id" "=" IDENT patch-block
set-data-fields = "set-data-fields" {"json" "=" TEXT}?
frame-block = "frame" "{" prop* bounds-block? "}"
bounds-block = "bounds" "{" prop* "}"
patch-block = "patch" "{" {"wrap-mode" "=" IDENT}? {"columns" "=" INT}? prop* "}"
${props}`,
    diff: `diff = operation*
operation = add-frame | remove-frame | patch-frame | set-data-fields
add-frame = "add-frame" "page-id" "=" IDENT frame-block
remove-frame = "remove-frame" "page-id" "=" IDENT "frame-id" "=" IDENT
patch-frame = "patch-frame" "page-id" "=" IDENT "frame-id" "=" IDENT patch-block
set-data-fields = "set-data-fields" {"json" "=" TEXT}?
frame-block = "frame" "{" prop* "}"
patch-block = "patch" "{" prop* "}"
${props}`,
  },
  shooting: {
    plugin: "🎥️shooting",
    artifact: "🎥️shooting",
    id: "shooting",
    family: "scene",
    document: `document = schema-line scene-block assets-table shots-table saved-cameras-table?
schema-line = "schema" "=" IDENT {"active-shot-id" "=" IDENT}? {"active-asset-id" "=" IDENT}?
scene-block = "scene" "{" background? sun-block? ambient-block? shadow-block? material-block? "}"
background = "background" "=" TEXT
sun-block = "sun" "{" {"enabled" "=" BOOL}? {"azimuth" "=" FLOAT}? {"elevation" "=" FLOAT}? {"intensity" "=" FLOAT}? {"color" "=" TEXT}? "}"
ambient-block = "ambient" "{" {"intensity" "=" FLOAT}? {"color" "=" TEXT}? "}"
shadow-block = "shadow" "{" {"enabled" "=" BOOL}? {"opacity" "=" FLOAT}? {"softness" "=" FLOAT}? "}"
material-block = "material" "{" {"color" "=" TEXT}? {"metalness" "=" FLOAT}? {"roughness" "=" FLOAT}? {"emissive" "=" TEXT}? {"emissive-intensity" "=" FLOAT}? "}"
assets-table = "assets" table-header "{" asset-row* "}"
shots-table = "shots" table-header "{" shot-row* "}"
saved-cameras-table = "saved-cameras" table-header "{" saved-camera* "}"
table-header = "[" IDENT ":" IDENT {IDENT ":" IDENT}* "]"
asset-row = IDENT TEXT TEXT IDENT coord? orientation? scale?
shot-row = IDENT TEXT INT INT IDENT IDENT TEXT camera-ref?
saved-camera = "saved-camera" {"id" "=" IDENT} {"label" "=" TEXT}? camera-block?
camera-block = "camera" "{" prop* "}"
camera-ref = IDENT | "_"
coord = "@" FLOAT "," FLOAT "," FLOAT | "_"
orientation = tuple | "_"
scale = tuple | "_"
tuple = FLOAT "," FLOAT "," FLOAT | "_"
${props}`,
    operation: `operation = set-active-shot | set-active-asset | set-shot-camera | patch-scene | translate-assets | rotate-assets | scale-assets | set-fixture | assets-add | assets-remove | assets-move | assets-patch | shots-add | shots-remove | shots-move | shots-patch | saved-cameras-add | saved-cameras-remove | saved-cameras-move | saved-cameras-patch
set-active-shot = "active-shot" "=" IDENT | "set-active-shot" "id" "=" IDENT
set-active-asset = "active-asset" "=" IDENT | "set-active-asset" "id" "=" IDENT
set-shot-camera = "set-shot-camera" "shot-id" "=" IDENT camera-block
patch-scene = "patch-scene" scene-patch
translate-assets = "translate-assets" "ids" "=" list "delta" "=" tuple
rotate-assets = "rotate-assets" "ids" "=" list "delta" "=" tuple
scale-assets = "scale-assets" "ids" "=" list "factor" "=" tuple
set-fixture = "set-fixture" fixture-block
assets-add = "assets-add" {"index" "=" INT}? asset-block
assets-remove = "assets-remove" "id" "=" IDENT
assets-move = "assets-move" "id" "=" IDENT "to" "=" INT
assets-patch = "assets-patch" "id" "=" IDENT patch-block
shots-add = "shots-add" {"index" "=" INT}? shot-block
shots-remove = "shots-remove" "id" "=" IDENT
shots-move = "shots-move" "id" "=" IDENT "to" "=" INT
shots-patch = "shots-patch" "id" "=" IDENT patch-block
saved-cameras-add = "saved-cameras-add" {"index" "=" INT}? saved-camera-block
saved-cameras-remove = "saved-cameras-remove" "id" "=" IDENT
saved-cameras-move = "saved-cameras-move" "id" "=" IDENT "to" "=" INT
saved-cameras-patch = "saved-cameras-patch" "id" "=" IDENT patch-block
scene-patch = "scene" "{" prop* "}"
camera-block = "camera" "{" prop* "}"
fixture-block = "fixture" "{" prop* "}"
asset-block = "asset" "{" prop* "}"
shot-block = "shot" "{" prop* "}"
saved-camera-block = "saved-camera" "{" prop* "}"
patch-block = "patch" "{" prop* "}"
tuple = FLOAT "," FLOAT "," FLOAT
${props}`,
    diff: `diff = {"scene" "=" scene-patch}? {"assets" "=" collection-diff}? {"shots" "=" collection-diff}? {"saved-cameras" "=" collection-diff}? {"active-shot-id" "=" IDENT}? {"active-asset-id" "=" IDENT}?
scene-patch = "{" prop* "}"
collection-diff = "{" {"added" "=" list}? {"removed" "=" list}? {"moved" "=" list}? {"modified" "=" list}? "}"
${props}`,
  },
  remodel: {
    plugin: "📸️remodel",
    artifact: "📸️remodel",
    id: "remodel",
    family: "scene",
    document: `document = schema-line streams-table? assets-map? calibration-block? params-block? gcps-table? job-block? results-block?
schema-line = "schema" "=" IDENT {"id" "=" IDENT}? {"title" "=" TEXT}?
streams-table = "streams" table-header "{" stream-row* "}"
assets-map = "assets" "=" "{" asset* "}"
calibration-block = "calibration" "{" cameras-table? rig-table? "}"
params-block = "params" "{" ingest-block? feature-block? match-block? sfm-block? dense-block? mesh-block? motion-block? geo-block? "}"
gcps-table = "gcps" table-header "{" gcp-row* "}"
job-block = "job" "{" prop* "}"
results-block = "results" "{" sparse-block? dense-block-result? mesh-block-result? trajectory-block? tracks-table? geo-products-block? qc-block? "}"
table-header = "[" IDENT ":" IDENT {IDENT ":" IDENT}* "]"
stream-row = IDENT prop*
asset = IDENT "=" {"mime" "=" TEXT}? {"data" "=" TEXT}? {"width" "=" INT}? {"height" "=" INT}?
cameras-table = "cameras" "{" camera* "}"
camera = "{" {"id" "=" IDENT} prop* "}"
rig-table = "rig" "{" rig-extrinsic* "}"
rig-extrinsic = "{" {"camera-id" "=" IDENT} prop* "}"
gcp-row = IDENT TEXT tuple prop*
ingest-block = "ingest" "{" prop* "}"
feature-block = "feature" "{" prop* "}"
match-block = "match" "{" prop* "}"
sfm-block = "sfm" "{" prop* "}"
dense-block = "dense" "{" prop* "}"
mesh-block = "mesh" "{" prop* "}"
motion-block = "motion" "{" prop* "}"
geo-block = "geo" "{" prop* "}"
sparse-block = "sparse" "{" prop* "}"
dense-block-result = "dense" "{" prop* "}"
mesh-block-result = "mesh" "{" prop* "}"
trajectory-block = "trajectory" "{" prop* "}"
tracks-table = "tracks" "{" prop* "}"
geo-products-block = "geo-products" "{" prop* "}"
qc-block = "qc" "{" prop* "}"
tuple = FLOAT "," FLOAT "," FLOAT
${props}`,
    operation: `operation = set-streams | set-asset | set-calibration | set-gcps | set-ingest-params | set-feature-params | set-match-params | set-sfm-params | set-dense-params | set-mesh-params | set-motion-params | set-geo-params | set-job | set-sparse | set-dense | set-mesh-result | set-trajectory | set-tracks | set-geo-products | set-qc
set-streams = "set-streams" streams-table
set-asset = "set-asset" "id" "=" IDENT asset-block
set-calibration = "set-calibration" calibration-block
set-gcps = "set-gcps" gcps-table
set-ingest-params = "set-ingest-params" params-block
set-feature-params = "set-feature-params" params-block
set-match-params = "set-match-params" params-block
set-sfm-params = "set-sfm-params" params-block
set-dense-params = "set-dense-params" params-block
set-mesh-params = "set-mesh-params" params-block
set-motion-params = "set-motion-params" params-block
set-geo-params = "set-geo-params" params-block
set-job = "set-job" job-block
set-sparse = "set-sparse" {"id" "=" IDENT}? sparse-block
set-dense = "set-dense" {"id" "=" IDENT}? dense-block
set-mesh-result = "set-mesh-result" mesh-block
set-trajectory = "set-trajectory" {"id" "=" IDENT}? trajectory-block
set-tracks = "set-tracks" tracks-table
set-geo-products = "set-geo-products" {"id" "=" IDENT}? geo-products-block
set-qc = "set-qc" {"id" "=" IDENT}? qc-block
streams-table = "streams" "{" prop* "}"
asset-block = "asset" "{" prop* "}"
calibration-block = "calibration" "{" prop* "}"
gcps-table = "gcps" "{" prop* "}"
params-block = "params" "{" prop* "}"
job-block = "job" "{" prop* "}"
sparse-block = "sparse" "{" prop* "}"
dense-block = "dense" "{" prop* "}"
mesh-block = "mesh" "{" prop* "}"
trajectory-block = "trajectory" "{" prop* "}"
tracks-table = "tracks" "{" prop* "}"
geo-products-block = "geo-products" "{" prop* "}"
qc-block = "qc" "{" prop* "}"
${props}`,
    diff: `diff = empty | set-streams | set-asset | set-calibration | set-gcps | set-ingest-params | set-feature-params | set-match-params | set-sfm-params | set-dense-params | set-mesh-params | set-motion-params | set-geo-params | set-job | set-sparse | set-dense | set-mesh-result | set-trajectory | set-tracks | set-geo-products | set-qc
empty = "empty"
set-streams = "set-streams" "{" prop* "}"
set-asset = "set-asset" "id" "=" IDENT "{" prop* "}"
set-calibration = "set-calibration" "{" prop* "}"
set-gcps = "set-gcps" "{" prop* "}"
set-ingest-params = "set-ingest-params" "{" prop* "}"
set-feature-params = "set-feature-params" "{" prop* "}"
set-match-params = "set-match-params" "{" prop* "}"
set-sfm-params = "set-sfm-params" "{" prop* "}"
set-dense-params = "set-dense-params" "{" prop* "}"
set-mesh-params = "set-mesh-params" "{" prop* "}"
set-motion-params = "set-motion-params" "{" prop* "}"
set-geo-params = "set-geo-params" "{" prop* "}"
set-job = "set-job" "{" prop* "}"
set-sparse = "set-sparse" "{" prop* "}"
set-dense = "set-dense" "{" prop* "}"
set-mesh-result = "set-mesh-result" "{" prop* "}"
set-trajectory = "set-trajectory" "{" prop* "}"
set-tracks = "set-tracks" "{" prop* "}"
set-geo-products = "set-geo-products" "{" prop* "}"
set-qc = "set-qc" "{" prop* "}"
${props}`,
  },
  note: {
    plugin: "🗒️note",
    artifact: "🗒️note",
    id: "note",
    family: "embed",
    document: `document = schema-line blocks-block? settings* assets-map?
schema-line = "schema" "=" IDENT {"id" "=" IDENT}? {"title" "=" TEXT}?
blocks-block = "blocks" "{" block* "}"
block = text-block | image-block | table-block | math-block | stroke-block | group-block
text-block = "text" block-props paragraphs?
image-block = "image" block-props {"image-key" "=" IDENT}?
table-block = "table" block-props {"columns" "=" list}? {"rows" "=" list}?
math-block = "math" block-props tex-field {"display-mode" "=" BOOL}?
stroke-block = "stroke" block-props {"points" "=" list}? {"stroke-width" "=" FLOAT}? {"color" "=" list}?
group-block = "group" block-props children-block?
block-props = {"id" "=" IDENT} {"name" "=" TEXT}? {"x" "=" FLOAT}? {"y" "=" FLOAT}? {"width" "=" FLOAT}? {"height" "=" FLOAT}? {"rotation" "=" FLOAT}? {"visible" "=" BOOL}? {"locked" "=" BOOL}? {"font-size" "=" FLOAT}? {"font-weight" "=" TEXT}? {"align" "=" IDENT}?
paragraphs = "paragraphs" "{" paragraph* "}"
paragraph = "p" "{" run* "}"
run = "r" TEXT {"bold" "=" BOOL}? {"italic" "=" BOOL}? {"underline" "=" BOOL}? {"link" "=" TEXT}?
tex-field = "tex" "=" (TEXT | fence)
fence = "\`\`\`" IDENT TEXT "\`\`\`"
children-block = "children" "{" block* "}"
settings = {"grid-visible" "=" BOOL} | {"grid-spacing" "=" FLOAT} | {"grid-subdivisions" "=" FLOAT} | {"grid-opacity" "=" FLOAT} | {"snap-enabled" "=" BOOL} | {"snap-grid-spacing" "=" FLOAT} | {"pencil-width" "=" FLOAT} | {"eraser-radius" "=" FLOAT}
assets-map = "assets" "=" "{" asset* "}"
asset = IDENT "=" {"mime" "=" TEXT}? {"data" "=" TEXT}? {"width" "=" FLOAT}? {"height" "=" FLOAT}?
${props}`,
    operation: `operation = set-grid-visible | set-grid-spacing | set-grid-subdivisions | set-grid-opacity | set-snap-enabled | set-snap-grid-spacing | set-pencil-width | set-eraser-radius | set-blocks | put-asset | remove-asset | set-document
set-grid-visible = "set-grid-visible" "visible" "=" BOOL
set-grid-spacing = "set-grid-spacing" "spacing" "=" FLOAT
set-grid-subdivisions = "set-grid-subdivisions" "subdivisions" "=" FLOAT
set-grid-opacity = "set-grid-opacity" "opacity" "=" FLOAT
set-snap-enabled = "set-snap-enabled" "enabled" "=" BOOL
set-snap-grid-spacing = "set-snap-grid-spacing" "spacing" "=" FLOAT
set-pencil-width = "set-pencil-width" "width" "=" FLOAT
set-eraser-radius = "set-eraser-radius" "radius" "=" FLOAT
set-blocks = "set-blocks" "{" block* "}"
put-asset = "put-asset" "id" "=" IDENT asset-block
remove-asset = "remove-asset" "id" "=" IDENT
set-document = "set-document" document-block
block = "text" prop* | "image" prop* | "table" prop* | "math" prop* | "stroke" prop* | "group" prop*
asset-block = "asset" "{" prop* "}"
document-block = "document" "{" prop* "}"
${props}`,
    diff: `diff = {"operation" "=" operation}?
operation = set-grid-visible | set-grid-spacing | set-grid-subdivisions | set-grid-opacity | set-snap-enabled | set-snap-grid-spacing | set-pencil-width | set-eraser-radius | set-blocks | put-asset | remove-asset | set-document
set-grid-visible = "set-grid-visible" "visible" "=" BOOL
set-grid-spacing = "set-grid-spacing" "spacing" "=" FLOAT
set-grid-subdivisions = "set-grid-subdivisions" "subdivisions" "=" FLOAT
set-grid-opacity = "set-grid-opacity" "opacity" "=" FLOAT
set-snap-enabled = "set-snap-enabled" "enabled" "=" BOOL
set-snap-grid-spacing = "set-snap-grid-spacing" "spacing" "=" FLOAT
set-pencil-width = "set-pencil-width" "width" "=" FLOAT
set-eraser-radius = "set-eraser-radius" "radius" "=" FLOAT
set-blocks = "set-blocks" "{" block* "}"
put-asset = "put-asset" "id" "=" IDENT asset-block
remove-asset = "remove-asset" "id" "=" IDENT
set-document = "set-document" document-block
block = "text" prop* | "image" prop* | "table" prop* | "math" prop* | "stroke" prop* | "group" prop*
asset-block = "asset" "{" prop* "}"
document-block = "document" "{" prop* "}"
${props}`,
  },
};

const facetMap = [
  ["🗣️dsl", "document", "grammar"],
  ["🔧️op", "operation", "grammar"],
  ["🔺️diff", "diff", "grammar"],
  ["🎒️pack", "pack", "protocol"],
  ["📡️spr", "spr", "protocol"],
];

let updated = 0;
const touched = [];

for (const spec of Object.values(specs)) {
  const base = join(pluginsRoot, spec.plugin, "🗿️artifacts", spec.artifact);
  for (const [facet, role, kind] of facetMap) {
    const dir = join(base, facet);
    if (!existsSync(dir)) continue;
    if (kind === "grammar") {
      const path = join(dir, "📖️component.grammar.semio");
      const body = header(spec.id, role === "document" ? "document" : role === "operation" ? "op" : "diff", spec.family)
        + (role === "document" ? `document = \n` : role === "operation" ? `operation = \n` : `diff = \n`);
      // Fix: use proper bodies without the mistaken rewrite
      const startName = role === "document" ? "document" : role === "operation" ? "operation" : "diff";
      const grammarBody = role === "document" ? spec.document : role === "operation" ? spec.operation : spec.diff;
      // Ensure start production is first matching line - bodies already define start nonterminal
      const content = header(spec.id, role === "document" ? "document" : role === "operation" ? "op" : "diff", spec.family) + grammarBody;
      writeFileSync(path, content.endsWith("\n") ? content : content + "\n");
      updated++;
      touched.push(path);
      const ts = join(dir, "🟦️component.ts");
      writeFileSync(ts, grammarTs(spec.id, facet));
      updated++;
      touched.push(ts);
    } else {
      const path = join(dir, "📡️component.protocol.semio");
      const content = role === "pack" ? packProtocol(spec.id) : sprProtocol(spec.id);
      // artifact-specific segment annotation via comment-free named payload kind field aliases
      const specific = role === "pack"
        ? content.replace(
            "segment kind u8\nsegment flags u8\nsegment payload varint bytes\n",
            `segment kind u8\nsegment flags u8\nsegment payload varint bytes\nsegment ${spec.id}_projection bytes\n`,
          )
        : content.replace(
            "field body bytes\n",
            `field body bytes\nfield ${spec.id}_op_payload bytes\n`,
          );
      writeFileSync(path, specific);
      updated++;
      touched.push(path);
      const ts = join(dir, "🟦️component.ts");
      writeFileSync(ts, protocolTs(spec.id, facet));
      updated++;
      touched.push(ts);
    }
  }

  // Update empty package barrel stubs (skip animate — owns react entry already)
  if (spec.plugin !== "🎞️animate") {
    const idx = join(pluginsRoot, spec.plugin, "📦️packages/🟦️typescript/📦️index.ts");
    if (existsSync(idx)) {
      const prev = readFileSync(idx, "utf8");
      if (prev.includes("export {}") || prev.includes("re-export artifact")) {
        const art = `../../🗿️artifacts/${spec.artifact}`;
        const body = `/** 🧩 ${spec.id} facet WASM facades — re-export artifact 🟦️component.ts leaves. */
export {
  parseDsl as parse${spec.id[0].toUpperCase()}${spec.id.slice(1)}Dsl,
  printDsl as print${spec.id[0].toUpperCase()}${spec.id.slice(1)}Dsl,
} from "${art}/🗣️dsl/🟦️component.ts";
export {
  parseDsl as parse${spec.id[0].toUpperCase()}${spec.id.slice(1)}Op,
  printDsl as print${spec.id[0].toUpperCase()}${spec.id.slice(1)}Op,
} from "${art}/🔧️op/🟦️component.ts";
export {
  parseDsl as parse${spec.id[0].toUpperCase()}${spec.id.slice(1)}Diff,
  printDsl as print${spec.id[0].toUpperCase()}${spec.id.slice(1)}Diff,
} from "${art}/🔺️diff/🟦️component.ts";
export {
  encode as encode${spec.id[0].toUpperCase()}${spec.id.slice(1)}Pack,
  decode as decode${spec.id[0].toUpperCase()}${spec.id.slice(1)}Pack,
} from "${art}/🎒️pack/🟦️component.ts";
export {
  encode as encode${spec.id[0].toUpperCase()}${spec.id.slice(1)}Spr,
  decode as decode${spec.id[0].toUpperCase()}${spec.id.slice(1)}Spr,
} from "${art}/📡️spr/🟦️component.ts";
`;
        writeFileSync(idx, body);
        updated++;
        touched.push(idx);
      }
    }
  }
}

// animate present package: add a dedicated wasm facade re-export file only if a stub barrel exists separately — leave react index alone.

console.log(JSON.stringify({ updated, count: touched.length, touched }, null, 2));
