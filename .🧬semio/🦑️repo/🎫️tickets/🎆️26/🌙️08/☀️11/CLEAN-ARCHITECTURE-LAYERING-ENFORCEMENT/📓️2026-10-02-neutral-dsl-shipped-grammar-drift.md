# 🧾️ Original Shipped Grammar Drift During DSL Extraction

The immutable original 503-input S grammar authority is unchanged. DSL extraction did not edit any S grammar asset. The current shared tree differs at 25 physical assets; current files are preserved, with exact original/current evidence below. This is an independent owner reconciliation frontier, not an admitted substitution of the original cohort.

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🧬️schema/🧬️mutations/📝️text/📖️.grammar.semio

Original SHA-256: `5a7f09367f1a147cac5ea451cdeda20e2cee5ce0f5af01e80636b796c9e2d650`. Current SHA-256: `28030cfd77fbf99e046c82b6c63c8d6feed3afc8592388408452055234a82253`.

Original:

```text
dialect grammar
grammar stdio.las.mutations
start op

# Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: real one-line
# OpText grammar matching `LasMutation`'s hand-rolled `print_las_mutation`/`parse_las_mutation`
# (../🦀️.rs's `TopLevel` region) -- replacing the pre-existing placeholder (`header =
# 'schema' SP 'stdio.las' NL` / `body = payload NL?` / `payload = *OCTET`) which used pseudo-ABNF
# syntax entirely outside this framework's real dialect and did not match `print_las_mutation`'s
# actual output at all. Every keyword/field-key token below is copied verbatim from a real,
# directly-observed `print_op()` shape for every variant -- not invented:
#   set-version major=1 minor=4
#   insert-vlr index=1 vlr=[<hex>,200,<hex>,<hex>]
#   set-snapshot snapshot=[<header-25-tuple>,[<vlr>,...],[<point>,...]]
# `set-snapshot`'s payload is a genuinely unbounded, arbitrarily-nested composite (a whole
# `LasHeader` positional 25-tuple plus two runtime-length `vlrs`/`points` lists) -- modeled
# honestly as `REST` (M1's raw-span terminal, reads the ORIGINAL source text to EOF past whatever
# the shared lexer already fragmented, so it correctly swallows arbitrarily-nested `[`/`,` content
# verbatim), the same honest-boundary treatment zip's own `set-snapshot`/`add-entry` variants use
# for their own nested-block payloads. Every OTHER variant (13 of 14, every field a plain scalar or
# one bounded fixed-shape record) is modeled precisely, field-for-field, token-for-token below.

op = set-snapshot | set-version | set-system-identifier | set-software-info | set-creation-date | set-scale-and-offset | set-bounds | set-points-by-return | insert-vlr | remove-vlr | set-vlr-data | insert-point | remove-point | set-point

# `f64::to_string()` prints a whole value like `100.0` as `100` (no decimal point) -- both lex
# through the shared lexer, `FLOAT` for a value with a `.`/exponent, `INT` otherwise.
number = FLOAT | INT

# Genuinely unbounded nested-record payload (`snapshot=[ ... ]`) -- see module doc comment.
set-snapshot = "set-snapshot" REST

set-version = "set-version" "major" "=" INT "minor" "=" INT

set-system-identifier = "set-system-identifier" "system-identifier" "=" hex

set-software-info = "set-software-info" "generating-software" "=" hex

set-creation-date = "set-creation-date" "day-of-year" "=" INT "year" "=" INT

set-scale-and-offset = "set-scale-and-offset" "scale" "=" f64x3 "offset" "=" f64x3

set-bounds = "set-bounds" "max" "=" f64x3 "min" "=" f64x3

set-points-by-return = "set-points-by-return" "counts" "=" u32x5

insert-vlr = "insert-vlr" "index" "=" INT "vlr" "=" vlr-body

remove-vlr = "remove-vlr" "index" "=" INT

set-vlr-data = "set-vlr-data" "index" "=" INT "data" "=" hex

insert-point = "insert-point" "index" "=" INT "point" "=" point-body

remove-point = "remove-point" "index" "=" INT

set-point = "set-point" "index" "=" INT "point" "=" point-body

f64x3 = "[" number "," number "," number "]"

u32x5 = "[" INT "," INT "," INT "," INT "," INT "]"

# `enc_vlr`'s full positional 4-tuple (user_id, record_id, description, data) -- same bounded
# fixed-shape record the sibling diff facet's own `vlr-body` production describes (own copy, no
# cross-facet grammar sharing).
vlr-body = "[" hex "," INT "," hex "," hex "]"

# `enc_point`'s full positional 14-tuple, field order matching `LasPoint`'s struct declaration
# exactly (x, y, z, intensity, return_number, number_of_returns, scan_direction_flag,
# edge_of_flight_line, classification, scan_angle_rank, user_data, point_source_id, gps_time, rgb).
point-body = "[" number "," number "," number "," INT "," INT "," INT "," INT "," INT "," INT "," INT "," INT "," INT "," gps-opt "," rgb-opt "]"

gps-opt = "[" "0" "]" | "[" "1" "," number "]"
rgb-opt = "[" "0" "]" | "[" "1" "," rgb-value "]"
rgb-value = "[" INT "," INT "," INT "]"
```

Current:

```text
dialect grammar
grammar stdio.las.mutations
start op

# Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: real one-line
# OpText grammar matching `LasMutation`'s hand-rolled `print_las_mutation`/`parse_las_mutation`
# (../🦀️.rs's `TopLevel` region) -- replacing the pre-existing placeholder (`header =
# 'schema' SP 'stdio.las' NL` / `body = payload NL?` / `payload = *OCTET`) which used pseudo-ABNF
# syntax entirely outside this framework's real dialect and did not match `print_las_mutation`'s
# actual output at all. Every keyword/field-key token below is copied verbatim from a real,
# directly-observed `print_op()` shape for every variant -- not invented:
#   set-version major=1 minor=4
#   insert-vlr index=1 vlr=[<hex>,200,<hex>,<hex>]
#   set-snapshot snapshot=[<header-25-tuple>,[<vlr>,...],[<point>,...]]
# `set-snapshot`'s payload is a genuinely unbounded, arbitrarily-nested composite (a whole
# `LasHeader` positional 25-tuple plus two runtime-length `vlrs`/`points` lists) -- modeled
# honestly as `REST` (M1's raw-span terminal, reads the ORIGINAL source text to EOF past whatever
# the shared lexer already fragmented, so it correctly swallows arbitrarily-nested `[`/`,` content
# verbatim), the same honest-boundary treatment zip's own `set-snapshot`/`add-entry` variants use
# for their own nested-block payloads. Every OTHER variant (13 of 14, every field a plain scalar or
# one bounded fixed-shape record) is modeled precisely, field-for-field, token-for-token below.

op = set-snapshot | patch-snapshot | set-version | set-system-identifier | set-software-info | set-creation-date | set-scale-and-offset | set-bounds | set-points-by-return | insert-vlr | remove-vlr | set-vlr-data | insert-point | remove-point | set-point

# `f64::to_string()` prints a whole value like `100.0` as `100` (no decimal point) -- both lex
# through the shared lexer, `FLOAT` for a value with a `.`/exponent, `INT` otherwise.
number = FLOAT | INT

# Genuinely unbounded nested-record payload (`snapshot=[ ... ]`) -- see module doc comment.
patch-snapshot = "patch-snapshot" "patch" "=" hex
set-snapshot = "set-snapshot" REST

set-version = "set-version" "major" "=" INT "minor" "=" INT

set-system-identifier = "set-system-identifier" "system-identifier" "=" hex

set-software-info = "set-software-info" "generating-software" "=" hex

set-creation-date = "set-creation-date" "day-of-year" "=" INT "year" "=" INT

set-scale-and-offset = "set-scale-and-offset" "scale" "=" f64x3 "offset" "=" f64x3

set-bounds = "set-bounds" "max" "=" f64x3 "min" "=" f64x3

set-points-by-return = "set-points-by-return" "counts" "=" u32x5

insert-vlr = "insert-vlr" "index" "=" INT "vlr" "=" vlr-body

remove-vlr = "remove-vlr" "index" "=" INT

set-vlr-data = "set-vlr-data" "index" "=" INT "data" "=" hex

insert-point = "insert-point" "index" "=" INT "point" "=" point-body

remove-point = "remove-point" "index" "=" INT

set-point = "set-point" "index" "=" INT "point" "=" point-body

f64x3 = "[" number "," number "," number "]"

u32x5 = "[" INT "," INT "," INT "," INT "," INT "]"

# `enc_vlr`'s full positional 4-tuple (user_id, record_id, description, data) -- same bounded
# fixed-shape record the sibling diff facet's own `vlr-body` production describes (own copy, no
# cross-facet grammar sharing).
vlr-body = "[" hex "," INT "," hex "," hex "]"

# `enc_point`'s full positional 14-tuple, field order matching `LasPoint`'s struct declaration
# exactly (x, y, z, intensity, return_number, number_of_returns, scan_direction_flag,
# edge_of_flight_line, classification, scan_angle_rank, user_data, point_source_id, gps_time, rgb).
point-body = "[" number "," number "," number "," INT "," INT "," INT "," INT "," INT "," INT "," INT "," INT "," INT "," gps-opt "," rgb-opt "]"

gps-opt = "[" "0" "]" | "[" "1" "," number "]"
rgb-opt = "[" "0" "]" | "[" "1" "," rgb-value "]"
rgb-value = "[" INT "," INT "," INT "]"
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/📖️.grammar.semio

Original SHA-256: `4d3034bb11468eda56f05725f7dfbb33209609a2b57a592b3350450d999633ce`. Current SHA-256: `c672e1fd80b585984e26ed687f7e9b5434d47cecaf68694647beb3bffd381262`.

Original:

```text
dialect grammar stdio.epw.mutations
root = mutation
; wire text is the hand-rolled `keyword arg=value ...` token line `protocol::OpText` prints
; (see 🦀️.rs `print_epw_mutation`/`parse_epw_mutation`) — not JSON. String/struct
; args are lowercase hex (see ../🔺️diff/📝️text/📖️.grammar.semio for the shared
; `hexstr`/`record`/`location`/`data-periods` encodings).
mutation = "set-snapshot" SP "snapshot=" hexsnapshot
         / "set-location" SP "location=" hexbrackets
         / "set-design-conditions" SP "value=" hexstr
         / "set-typical-extreme-periods" SP "value=" hexstr
         / "set-ground-temperatures" SP "value=" hexstr
         / "set-holidays-dst" SP "value=" hexstr
         / "set-comments-1" SP "value=" hexstr
         / "set-comments-2" SP "value=" hexstr
         / "set-data-periods" SP "data-periods=" hexbrackets
         / "insert-record" SP "index=" INT SP "record=" hexbrackets
         / "remove-record" SP "index=" INT
         / "set-record-field" SP "record-index=" INT SP "field-index=" INT SP "value=" hexstr
SP = " "
hexstr = *HEXDIG
hexbrackets = "[" *(hexstr ",") "]"
hexsnapshot = "[" 9(hexpart ",") records "]"
```

Current:

```text
dialect grammar stdio.epw.mutations
root = mutation
; wire text is the hand-rolled `keyword arg=value ...` token line `protocol::OpText` prints
; (see 🦀️.rs `print_epw_mutation`/`parse_epw_mutation`) — not JSON. String/struct
; args are lowercase hex (see ../🔺️diff/📝️text/📖️.grammar.semio for the shared
; `hexstr`/`record`/`location`/`data-periods` encodings).
mutation = "set-snapshot" SP "snapshot=" hexsnapshot
         / "patch-snapshot" SP "patch=" hexstr
         / "set-location" SP "location=" hexbrackets
         / "set-design-conditions" SP "value=" hexstr
         / "set-typical-extreme-periods" SP "value=" hexstr
         / "set-ground-temperatures" SP "value=" hexstr
         / "set-holidays-dst" SP "value=" hexstr
         / "set-comments-1" SP "value=" hexstr
         / "set-comments-2" SP "value=" hexstr
         / "set-data-periods" SP "data-periods=" hexbrackets
         / "insert-record" SP "index=" INT SP "record=" hexbrackets
         / "remove-record" SP "index=" INT
         / "set-record-field" SP "record-index=" INT SP "field-index=" INT SP "value=" hexstr
SP = " "
hexstr = *HEXDIG
hexbrackets = "[" *(hexstr ",") "]"
hexsnapshot = "[" 9(hexpart ",") records "]"
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📝️text/📖️.grammar.semio

Original SHA-256: `0acd635b773d6d250e95fc2db4588964677343840b9adccc364475dc31343335`. Current SHA-256: `223658d87bd696622f6b2e2aa2e0a14ff4c4842044a06c29b84f750e7ecb538e`.

Original:

```text
dialect grammar
grammar stdio.zip.mutations
start op

# 🧬 Generic DslVariants syntax for logical archive comment and name-keyed decompressed members.
op = set-snapshot | set-archive-comment | add-entry | remove-entry | rename-entry | set-entry-data
text-value = IDENT | TEXT
bool-value = "true" | "false"

set-snapshot = "set-snapshot" REST
set-archive-comment = "set-archive-comment" "comment" "=" text-value "comment-utf8" "=" bool-value
add-entry = "add-entry" REST
remove-entry = "remove-entry" "name" "=" text-value
rename-entry = "rename-entry" "name" "=" text-value "new-name" "=" text-value
set-entry-data = "set-entry-data" "name" "=" text-value "data" "=" text-value
```

Current:

```text
dialect grammar
grammar stdio.zip.mutations
start op

# 🧬 Generic DslVariants syntax for logical archive comment and name-keyed decompressed members.
op = set-snapshot | patch-snapshot | set-archive-comment | add-entry | remove-entry | rename-entry | set-entry-data
text-value = IDENT | TEXT
bool-value = "true" | "false"

patch-snapshot = "patch-snapshot" "patch" "=" hex
set-snapshot = "set-snapshot" REST
set-archive-comment = "set-archive-comment" "comment" "=" text-value "comment-utf8" "=" bool-value
add-entry = "add-entry" REST
remove-entry = "remove-entry" "name" "=" text-value
rename-entry = "rename-entry" "name" "=" text-value "new-name" "=" text-value
set-entry-data = "set-entry-data" "name" "=" text-value "data" "=" text-value
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/7️⃣87a/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/📖️.grammar.semio

Original SHA-256: `3936ed2716fcfcd137cb46bd92e7a5cd87aa9d79707308b79bcde3e17944678a`. Current SHA-256: `a8351726014795e883b982d08d05c2ffd41d5e4b4cf438355f418997ca4ce5bb`.

Original:

```text
dialect grammar
grammar stdio.gif.mutations
start op

# Real one-line op-text form this artifact's `dsl::DslOps`-derived `OpText::print_op`/
# `parse_op` (../🦀️.rs, `dsl::print`/`dsl::parse` over the derived `DslVariants`
# spec) ACTUALLY emits — captured verbatim from a real `mutation.print_op()` call over every
# variant (P2-FG2 debug probe, deleted after capture; see this artifact's report). Kebab-case
# keyword + kebab-case `field=value` tokens; a `#[dsl(block)]`/struct-valued field prints as
# `field { inner-fields }`; ANY `Option<T>` field (block-struct OR plain scalar) is OMITTED
# ENTIRELY, not even an empty marker, when `None` — this dialect's mutations text form has no
# tri-state bracket tag at all (that `[0]`/`[1,value]` convention is this artifact's DIFF
# codec's own SEPARATE hand-rolled grammar, confirmed NOT shared by the mutations text form —
# `SetFrameTransparency { transparent_index: None }` prints as bare `"...index=0"`, no
# `transparent-index` token whatsoever). A `Vec<T>` field prints as `field=[ item-fields
# item-fields ... ]` — items enclosed in explicit braces (the parser preserves each
# record boundary and optional field presence, separately from positional-tuple value
# grammars). A plain `String` field (not `#[dsl(base64)]`-tagged) prints as a BARE, unquoted
# `IDENT` token (the shared lexer's `is_ident_continue` allows `.`/`-`/`_`/`/` — confirmed by
# reading `🔍️lexer/🦀️.rs` directly, NOT assumed — so `schema=stdio.gif` lexes as one
# `Ident`, never `Text`); only a `#[dsl(base64)]`-tagged `Vec<u8>` payload (`indices`) prints
# QUOTED, matching `Terminal(TEXT)`.
comment none

op = set-snapshot-op | set-screen-size-op | set-global-color-table-op | set-background-color-index-op | set-pixel-aspect-ratio-op | insert-image-op | remove-image-op | move-image-op | set-image-geometry-op | set-image-pixels-op | set-image-interlace-op

set-snapshot-op = "set-snapshot" "snapshot" "{" snapshot-fields "}"
set-screen-size-op = "set-screen-size" "width" "=" INT "height" "=" INT
set-global-color-table-op = "set-global-color-table" gct-clause?
set-background-color-index-op = "set-background-color-index" "index" "=" INT
set-pixel-aspect-ratio-op = "set-pixel-aspect-ratio" "ratio" "=" INT
insert-image-op = "insert-image" "index" "=" INT "image" "{" image-fields "}"
remove-image-op = "remove-image" "index" "=" INT
move-image-op = "move-image" "from" "=" INT "to" "=" INT
set-image-geometry-op = "set-image-geometry" "index" "=" INT "left" "=" INT "top" "=" INT "width" "=" INT "height" "=" INT
set-image-pixels-op = "set-image-pixels" "index" "=" INT "indices" "=" TEXT
set-image-interlace-op = "set-image-interlace" "index" "=" INT "interlace" "=" BOOL

# `GifSnapshot`'s own derived record shape — non-block scalar fields FIRST (declaration
# order), block/`Vec`-of-struct fields LAST (declaration order among those): `schema` is the
# document's real hand-rolled envelope-id string, `gct` an optional Global Color Table block,
# `images` the flattened image list.
snapshot-fields = "schema" "=" IDENT "width" "=" INT "height" "=" INT "background-color-index" "=" INT "pixel-aspect-ratio" "=" INT gct-clause? "images" "=" "[" image-item* "]"

gct-clause = "gct" "{" color-table-fields "}"
lct-clause = "lct" "{" color-table-fields "}"
color-table-fields = "sorted" "=" BOOL "colors" "=" "[" rgb-item* "]"
rgb-item = "{" "r" "=" INT "g" "=" INT "b" "=" INT "}"

# `GifImage`'s own derived record shape — scalar fields (`left`/`top`/`width`/`height`/
# `interlace`/`indices`, the last a base64-quoted `#[dsl(base64)]` byte vector) first, the one
# block field (`lct`, `Option<GifColorTable>`) last, omitted entirely when `None`. Reused both
# for `insert-image`'s mandatory `image { ... }` payload and for each flattened item inside a
# snapshot's `images=[ ... ]` list.
image-fields = "left" "=" INT "top" "=" INT "width" "=" INT "height" "=" INT "interlace" "=" BOOL "indices" "=" TEXT lct-clause?

image-item = "{" image-fields "}"
```

Current:

```text
dialect grammar
grammar stdio.gif.mutations
start op

# Real one-line op-text form this artifact's `dsl::DslOps`-derived `OpText::print_op`/
# `parse_op` (../🦀️.rs, `dsl::print`/`dsl::parse` over the derived `DslVariants`
# spec) ACTUALLY emits — captured verbatim from a real `mutation.print_op()` call over every
# variant (P2-FG2 debug probe, deleted after capture; see this artifact's report). Kebab-case
# keyword + kebab-case `field=value` tokens; a `#[dsl(block)]`/struct-valued field prints as
# `field { inner-fields }`; ANY `Option<T>` field (block-struct OR plain scalar) is OMITTED
# ENTIRELY, not even an empty marker, when `None` — this dialect's mutations text form has no
# tri-state bracket tag at all (that `[0]`/`[1,value]` convention is this artifact's DIFF
# codec's own SEPARATE hand-rolled grammar, confirmed NOT shared by the mutations text form —
# `SetFrameTransparency { transparent_index: None }` prints as bare `"...index=0"`, no
# `transparent-index` token whatsoever). A `Vec<T>` field prints as `field=[ item-fields
# item-fields ... ]` — items enclosed in explicit braces (the parser preserves each
# record boundary and optional field presence, separately from positional-tuple value
# grammars). A plain `String` field (not `#[dsl(base64)]`-tagged) prints as a BARE, unquoted
# `IDENT` token (the shared lexer's `is_ident_continue` allows `.`/`-`/`_`/`/` — confirmed by
# reading `🔍️lexer/🦀️.rs` directly, NOT assumed — so `schema=stdio.gif` lexes as one
# `Ident`, never `Text`); only a `#[dsl(base64)]`-tagged `Vec<u8>` payload (`indices`) prints
# QUOTED, matching `Terminal(TEXT)`.
comment none

op = set-snapshot-op | patch-snapshot-op | set-screen-size-op | set-global-color-table-op | set-background-color-index-op | set-pixel-aspect-ratio-op | insert-image-op | remove-image-op | move-image-op | set-image-geometry-op | set-image-pixels-op | set-image-interlace-op

patch-snapshot-op = "patch-snapshot" "patch" "=" hex
set-snapshot-op = "set-snapshot" "snapshot" "{" snapshot-fields "}"
set-screen-size-op = "set-screen-size" "width" "=" INT "height" "=" INT
set-global-color-table-op = "set-global-color-table" gct-clause?
set-background-color-index-op = "set-background-color-index" "index" "=" INT
set-pixel-aspect-ratio-op = "set-pixel-aspect-ratio" "ratio" "=" INT
insert-image-op = "insert-image" "index" "=" INT "image" "{" image-fields "}"
remove-image-op = "remove-image" "index" "=" INT
move-image-op = "move-image" "from" "=" INT "to" "=" INT
set-image-geometry-op = "set-image-geometry" "index" "=" INT "left" "=" INT "top" "=" INT "width" "=" INT "height" "=" INT
set-image-pixels-op = "set-image-pixels" "index" "=" INT "indices" "=" TEXT
set-image-interlace-op = "set-image-interlace" "index" "=" INT "interlace" "=" BOOL

# `GifSnapshot`'s own derived record shape — non-block scalar fields FIRST (declaration
# order), block/`Vec`-of-struct fields LAST (declaration order among those): `schema` is the
# document's real hand-rolled envelope-id string, `gct` an optional Global Color Table block,
# `images` the flattened image list.
snapshot-fields = "schema" "=" IDENT "width" "=" INT "height" "=" INT "background-color-index" "=" INT "pixel-aspect-ratio" "=" INT gct-clause? "images" "=" "[" image-item* "]"

gct-clause = "gct" "{" color-table-fields "}"
lct-clause = "lct" "{" color-table-fields "}"
color-table-fields = "sorted" "=" BOOL "colors" "=" "[" rgb-item* "]"
rgb-item = "{" "r" "=" INT "g" "=" INT "b" "=" INT "}"

# `GifImage`'s own derived record shape — scalar fields (`left`/`top`/`width`/`height`/
# `interlace`/`indices`, the last a base64-quoted `#[dsl(base64)]` byte vector) first, the one
# block field (`lct`, `Option<GifColorTable>`) last, omitted entirely when `None`. Reused both
# for `insert-image`'s mandatory `image { ... }` payload and for each flattened item inside a
# snapshot's `images=[ ... ]` list.
image-fields = "left" "=" INT "top" "=" INT "width" "=" INT "height" "=" INT "interlace" "=" BOOL "indices" "=" TEXT lct-clause?

image-item = "{" image-fields "}"
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📝️text/📖️.grammar.semio

Original SHA-256: `02552075640803c2cdc337edef3d51f3d83db29239be8e3dd2afcb8cb3376bca`. Current SHA-256: `c8fe140eaf40c9d33cefcce5a046eec51397473102fa827f97e45a14048b9b5e`.

Original:

```text
dialect grammar
grammar stdio.gif.89a.mutations
start op

# Real one-line op-text form this artifact's `dsl::DslOps`-derived `OpText::print_op`/
# `parse_op` (../🦀️.rs, `dsl::print`/`dsl::parse` over the derived `DslVariants`
# spec) ACTUALLY emits — captured verbatim from a real `mutation.print_op()` call over every
# variant (P2-FG2 debug probe, deleted after capture; see this artifact's report and 87a's own
# sibling grammar's identical shape). Kebab-case keyword + kebab-case `field=value` tokens; a
# `#[dsl(block)]`/struct-valued field prints as `field { inner-fields }`; ANY `Option<T>`
# field (block-struct OR plain scalar — e.g. `loop_count`/`transparent_index`) is OMITTED
# ENTIRELY, not even a marker, when `None` (confirmed live: `SetFrameTransparency {
# transparent_index: None }` prints as bare `"...index=0"`, no `transparent-index` token at
# all — this dialect's mutations text form has no tri-state bracket tag, unlike the DIFF
# codec's own SEPARATE hand-rolled `[0]`/`[1,value]` grammar). A `Vec<StructT>` field prints
# as `field=[ item-fields item-fields ... ]` — items back-to-back with NO separator (the
# parser relies on each item's own FIXED field count); a `Vec<String>` field prints as
# `field=[ bare-value bare-value ... ]` (no per-item key, just the raw string tokens); a fixed
# `[u8; N]` field (`GifAppExtension::identifier`/`auth_code`) prints as `field=v1,v2,...,vN` —
# comma-separated raw ints, NO enclosing brackets at all. A plain `String` field (not
# `#[dsl(base64)]`-tagged) prints as a BARE, unquoted `IDENT` token (the shared lexer's
# `is_ident_continue` allows `.`/`-`/`_`/`/` — confirmed by reading `🔍️lexer/🦀️.rs`
# directly, NOT assumed — so `schema=stdio.gif.89a` lexes as one `Ident`); only a
# `#[dsl(base64)]`-tagged `Vec<u8>` payload (`indices`/`data`) prints QUOTED, matching
# `Terminal(TEXT)`. `GifDisposal` (a plain unit-variant enum) prints as its own bare kebab-case
# keyword (`unspecified`/`do-not-dispose`/`restore-to-background`/`restore-to-previous`).
comment none

op = set-snapshot-op | set-screen-size-op | set-global-color-table-op | set-background-color-index-op | set-pixel-aspect-ratio-op | set-loop-count-op | insert-frame-op | remove-frame-op | move-frame-op | set-frame-geometry-op | set-frame-pixels-op | set-frame-interlace-op | set-frame-delay-op | set-frame-disposal-op | set-frame-transparency-op | set-frame-user-input-op | insert-comment-op | remove-comment-op | add-app-extension-op | remove-app-extension-op

set-snapshot-op = "set-snapshot" "snapshot" "{" snapshot-fields "}"
set-screen-size-op = "set-screen-size" "width" "=" INT "height" "=" INT
set-global-color-table-op = "set-global-color-table" gct-clause?
set-background-color-index-op = "set-background-color-index" "index" "=" INT
set-pixel-aspect-ratio-op = "set-pixel-aspect-ratio" "ratio" "=" INT
set-loop-count-op = "set-loop-count" loop-count-clause?
insert-frame-op = "insert-frame" "index" "=" INT "frame" "{" frame-fields "}"
remove-frame-op = "remove-frame" "index" "=" INT
move-frame-op = "move-frame" "from" "=" INT "to" "=" INT
set-frame-geometry-op = "set-frame-geometry" "index" "=" INT "left" "=" INT "top" "=" INT "width" "=" INT "height" "=" INT
set-frame-pixels-op = "set-frame-pixels" "index" "=" INT "indices" "=" TEXT
set-frame-interlace-op = "set-frame-interlace" "index" "=" INT "interlace" "=" BOOL
set-frame-delay-op = "set-frame-delay" "index" "=" INT "delay-cs" "=" INT
set-frame-disposal-op = "set-frame-disposal" "index" "=" INT "disposal" "=" disposal-word
set-frame-transparency-op = "set-frame-transparency" "index" "=" INT transparent-index-clause?
set-frame-user-input-op = "set-frame-user-input" "index" "=" INT "user-input" "=" BOOL
insert-comment-op = "insert-comment" "index" "=" INT "text" "=" IDENT
remove-comment-op = "remove-comment" "index" "=" INT
add-app-extension-op = "add-app-extension" "index" "=" INT "extension" "{" app-extension-fields "}"
remove-app-extension-op = "remove-app-extension" "index" "=" INT

# `GifSnapshot` (89a)'s own derived record shape — non-block scalar fields FIRST (declaration
# order), block/`Vec`-of-struct fields LAST (declaration order among those): `gct` an optional
# Global Color Table block, `frames`/`comments`/`app-extensions` the three flattened lists.
snapshot-fields = "schema" "=" IDENT "width" "=" INT "height" "=" INT "background-color-index" "=" INT "pixel-aspect-ratio" "=" INT loop-count-clause? gct-clause? "frames" "=" "[" frame-item* "]" "comments" "=" "[" IDENT* "]" "app-extensions" "=" "[" app-extension-item* "]"

loop-count-clause = "loop-count" "=" INT
gct-clause = "gct" "{" color-table-fields "}"
lct-clause = "lct" "{" color-table-fields "}"
color-table-fields = "sorted" "=" BOOL "colors" "=" "[" rgb-item* "]"
rgb-item = "{" "r" "=" INT "g" "=" INT "b" "=" INT "}"

transparent-index-clause = "transparent-index" "=" INT
disposal-word = "unspecified" | "do-not-dispose" | "restore-to-background" | "restore-to-previous"

plain-text-clause = "plain-text" "{" plain-text-fields "}"
plain-text-fields = "left" "=" INT "top" "=" INT "width" "=" INT "height" "=" INT "cell-width" "=" INT "cell-height" "=" INT "fg-color-index" "=" INT "bg-color-index" "=" INT "text" "=" IDENT

# `GifFrame`'s own derived record shape — scalar fields first (`left`/`top`/`width`/`height`/
# `interlace`/`indices`/`delay-cs`/`disposal`/`transparent-index`(scalar option)/`user-input`),
# the two block fields (`lct`, `plain-text`) last, each omitted entirely when absent. Reused
# both for `insert-frame`'s mandatory `frame { ... }` payload and for each flattened item
# inside a snapshot's `frames=[ ... ]` list.
frame-fields = "left" "=" INT "top" "=" INT "width" "=" INT "height" "=" INT "interlace" "=" BOOL "indices" "=" TEXT "delay-cs" "=" INT "disposal" "=" disposal-word transparent-index-clause? "user-input" "=" BOOL lct-clause? plain-text-clause?

# `GifAppExtension`'s own derived record shape — `identifier`/`auth_code` are fixed-size byte
# arrays (`[u8; 8]`/`[u8; 3]`), printed as bare comma-separated ints with NO enclosing
# brackets; `data` is the one `#[dsl(base64)]`-tagged `Vec<u8>` payload.
app-extension-fields = "identifier" "=" byte8-list "auth-code" "=" byte3-list "data" "=" TEXT
byte8-list = INT "," INT "," INT "," INT "," INT "," INT "," INT "," INT
byte3-list = INT "," INT "," INT

frame-item = "{" frame-fields "}"
app-extension-item = "{" app-extension-fields "}"
```

Current:

```text
dialect grammar
grammar stdio.gif.89a.mutations
start op

# Real one-line op-text form this artifact's `dsl::DslOps`-derived `OpText::print_op`/
# `parse_op` (../🦀️.rs, `dsl::print`/`dsl::parse` over the derived `DslVariants`
# spec) ACTUALLY emits — captured verbatim from a real `mutation.print_op()` call over every
# variant (P2-FG2 debug probe, deleted after capture; see this artifact's report and 87a's own
# sibling grammar's identical shape). Kebab-case keyword + kebab-case `field=value` tokens; a
# `#[dsl(block)]`/struct-valued field prints as `field { inner-fields }`; ANY `Option<T>`
# field (block-struct OR plain scalar — e.g. `loop_count`/`transparent_index`) is OMITTED
# ENTIRELY, not even a marker, when `None` (confirmed live: `SetFrameTransparency {
# transparent_index: None }` prints as bare `"...index=0"`, no `transparent-index` token at
# all — this dialect's mutations text form has no tri-state bracket tag, unlike the DIFF
# codec's own SEPARATE hand-rolled `[0]`/`[1,value]` grammar). A `Vec<StructT>` field prints
# as `field=[ item-fields item-fields ... ]` — items back-to-back with NO separator (the
# parser relies on each item's own FIXED field count); a `Vec<String>` field prints as
# `field=[ bare-value bare-value ... ]` (no per-item key, just the raw string tokens); a fixed
# `[u8; N]` field (`GifAppExtension::identifier`/`auth_code`) prints as `field=v1,v2,...,vN` —
# comma-separated raw ints, NO enclosing brackets at all. A plain `String` field (not
# `#[dsl(base64)]`-tagged) prints as a BARE, unquoted `IDENT` token (the shared lexer's
# `is_ident_continue` allows `.`/`-`/`_`/`/` — confirmed by reading `🔍️lexer/🦀️.rs`
# directly, NOT assumed — so `schema=stdio.gif.89a` lexes as one `Ident`); only a
# `#[dsl(base64)]`-tagged `Vec<u8>` payload (`indices`/`data`) prints QUOTED, matching
# `Terminal(TEXT)`. `GifDisposal` (a plain unit-variant enum) prints as its own bare kebab-case
# keyword (`unspecified`/`do-not-dispose`/`restore-to-background`/`restore-to-previous`).
comment none

op = set-snapshot-op | patch-snapshot-op | set-screen-size-op | set-global-color-table-op | set-background-color-index-op | set-pixel-aspect-ratio-op | set-loop-count-op | insert-frame-op | remove-frame-op | move-frame-op | set-frame-geometry-op | set-frame-pixels-op | set-frame-interlace-op | set-frame-delay-op | set-frame-disposal-op | set-frame-transparency-op | set-frame-user-input-op | insert-comment-op | remove-comment-op | add-app-extension-op | remove-app-extension-op

patch-snapshot-op = "patch-snapshot" "patch" "=" hex
set-snapshot-op = "set-snapshot" "snapshot" "{" snapshot-fields "}"
set-screen-size-op = "set-screen-size" "width" "=" INT "height" "=" INT
set-global-color-table-op = "set-global-color-table" gct-clause?
set-background-color-index-op = "set-background-color-index" "index" "=" INT
set-pixel-aspect-ratio-op = "set-pixel-aspect-ratio" "ratio" "=" INT
set-loop-count-op = "set-loop-count" loop-count-clause?
insert-frame-op = "insert-frame" "index" "=" INT "frame" "{" frame-fields "}"
remove-frame-op = "remove-frame" "index" "=" INT
move-frame-op = "move-frame" "from" "=" INT "to" "=" INT
set-frame-geometry-op = "set-frame-geometry" "index" "=" INT "left" "=" INT "top" "=" INT "width" "=" INT "height" "=" INT
set-frame-pixels-op = "set-frame-pixels" "index" "=" INT "indices" "=" TEXT
set-frame-interlace-op = "set-frame-interlace" "index" "=" INT "interlace" "=" BOOL
set-frame-delay-op = "set-frame-delay" "index" "=" INT "delay-cs" "=" INT
set-frame-disposal-op = "set-frame-disposal" "index" "=" INT "disposal" "=" disposal-word
set-frame-transparency-op = "set-frame-transparency" "index" "=" INT transparent-index-clause?
set-frame-user-input-op = "set-frame-user-input" "index" "=" INT "user-input" "=" BOOL
insert-comment-op = "insert-comment" "index" "=" INT "text" "=" IDENT
remove-comment-op = "remove-comment" "index" "=" INT
add-app-extension-op = "add-app-extension" "index" "=" INT "extension" "{" app-extension-fields "}"
remove-app-extension-op = "remove-app-extension" "index" "=" INT

# `GifSnapshot` (89a)'s own derived record shape — non-block scalar fields FIRST (declaration
# order), block/`Vec`-of-struct fields LAST (declaration order among those): `gct` an optional
# Global Color Table block, `frames`/`comments`/`app-extensions` the three flattened lists.
snapshot-fields = "schema" "=" IDENT "width" "=" INT "height" "=" INT "background-color-index" "=" INT "pixel-aspect-ratio" "=" INT loop-count-clause? gct-clause? "frames" "=" "[" frame-item* "]" "comments" "=" "[" IDENT* "]" "app-extensions" "=" "[" app-extension-item* "]"

loop-count-clause = "loop-count" "=" INT
gct-clause = "gct" "{" color-table-fields "}"
lct-clause = "lct" "{" color-table-fields "}"
color-table-fields = "sorted" "=" BOOL "colors" "=" "[" rgb-item* "]"
rgb-item = "{" "r" "=" INT "g" "=" INT "b" "=" INT "}"

transparent-index-clause = "transparent-index" "=" INT
disposal-word = "unspecified" | "do-not-dispose" | "restore-to-background" | "restore-to-previous"

plain-text-clause = "plain-text" "{" plain-text-fields "}"
plain-text-fields = "left" "=" INT "top" "=" INT "width" "=" INT "height" "=" INT "cell-width" "=" INT "cell-height" "=" INT "fg-color-index" "=" INT "bg-color-index" "=" INT "text" "=" IDENT

# `GifFrame`'s own derived record shape — scalar fields first (`left`/`top`/`width`/`height`/
# `interlace`/`indices`/`delay-cs`/`disposal`/`transparent-index`(scalar option)/`user-input`),
# the two block fields (`lct`, `plain-text`) last, each omitted entirely when absent. Reused
# both for `insert-frame`'s mandatory `frame { ... }` payload and for each flattened item
# inside a snapshot's `frames=[ ... ]` list.
frame-fields = "left" "=" INT "top" "=" INT "width" "=" INT "height" "=" INT "interlace" "=" BOOL "indices" "=" TEXT "delay-cs" "=" INT "disposal" "=" disposal-word transparent-index-clause? "user-input" "=" BOOL lct-clause? plain-text-clause?

# `GifAppExtension`'s own derived record shape — `identifier`/`auth_code` are fixed-size byte
# arrays (`[u8; 8]`/`[u8; 3]`), printed as bare comma-separated ints with NO enclosing
# brackets; `data` is the one `#[dsl(base64)]`-tagged `Vec<u8>` payload.
app-extension-fields = "identifier" "=" byte8-list "auth-code" "=" byte3-list "data" "=" TEXT
byte8-list = INT "," INT "," INT "," INT "," INT "," INT "," INT "," INT
byte3-list = INT "," INT "," INT

frame-item = "{" frame-fields "}"
app-extension-item = "{" app-extension-fields "}"
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📝️text/📖️.grammar.semio

Original SHA-256: `99994a0d0df0cbb8e2d8825a6dcbc7a7e2d21232f08e9002c9dd3c168b933b0f`. Current SHA-256: `d3472e8580756e127b4d140273f6950c15d6581cb3511aaf9f4ba810c1d38bb8`.

Original:

```text
dialect grammar
grammar svg.mutations
extension svg
start document

# Direct descriptor identities: set-declaration, set-doctype, insert-element, remove-element, set-element-name, set-attribute, set-text, set-view-box, set-transform.
document = "svg-mutation" "payload" "=" hex

set-snapshot-op = "set-snapshot" "snapshot" "=" (IDENT | TEXT)
```

Current:

```text
dialect grammar
grammar svg.mutations
extension svg
start document

# Direct descriptor identities: set-declaration, set-doctype, insert-element, remove-element, set-element-name, set-attribute, set-text, set-view-box, set-transform.
document = "svg-mutation" "payload" "=" hex

patch-snapshot-op = "patch-snapshot" "patch" "=" hex
set-snapshot-op = "set-snapshot" "snapshot" "=" (IDENT | TEXT)
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/📖️.grammar.semio

Original SHA-256: `bc6c6b96ff2b2acdd3bd3446b70d1bbad650aaf16f19cbcd4f9220b5a3bf7a6f`. Current SHA-256: `0b9d4fc3059a341fa0d994bf19e251c8eea1a407b8036968f60b76dfa9725495`.

Original:

```text
# 📖️ P2-FG1: real `OpText::print_op`/`parse_op` shape for `IfcMutation`, hand-rolled since
# `#[derive(dsl::DslOps)]` fails (real `cargo check` output — `IfcValue`/`IfcSnapshot`/`IfcEntity:
# DslField` unsatisfied, see `🧬️mutations/🦀️.rs`'s own doc comment). One `keyword
# key=value ...` line, space-separated, keys in the field's declared struct order — every literal
# below copied verbatim from `print_ifc_mutation`'s real match arms
# (`✏️s/…/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`), not
# invented. Replaces the old ABNF placeholder this file used to contain.
dialect grammar
grammar stdio.ifc.mutation
start document

document = set-snapshot | set-file-description | set-file-name | set-file-schema | insert-entity | remove-entity | set-entity-name | set-entity-arg | insert-entity-arg | remove-entity-arg

set-snapshot = "set-snapshot" "snapshot" "=" ifc-snapshot
set-file-description = "set-file-description" "values" "=" ifc-value-list
set-file-name = "set-file-name" "values" "=" ifc-value-list
set-file-schema = "set-file-schema" "values" "=" ifc-value-list
insert-entity = "insert-entity" "index" "=" INT "entity" "=" entity
remove-entity = "remove-entity" "id" "=" INT
set-entity-name = "set-entity-name" "id" "=" INT "name" "=" hex
set-entity-arg = "set-entity-arg" "id" "=" INT "index" "=" INT "value" "=" ifc-value
insert-entity-arg = "insert-entity-arg" "id" "=" INT "index" "=" INT "value" "=" ifc-value
remove-entity-arg = "remove-entity-arg" "id" "=" INT "index" "=" INT

# `IfcValue`'s tag scheme (`enc_ifc_value`/`dec_ifc_value`): `U`/`D` Unset/Derived are BARE letters
# (no brackets — unlike STEP's own `StepValue`, per this artifact's own doc comment: "payload-free
# variants are the bare letter, never ambiguous since every token boundary is whitespace/`,`/`;`"),
# `I[n]` Integer, `R[n]` Real, `S[hex]` String, `E[hex]` Enum, `F[n]` reFerence, `A[items]`
# Aggregate (recursive, bare comma-list, no self-bracket), `T[hex,[items]]` TypedValue (recursive,
# name + its own bracketed item list).
ifc-value = "U" | "D" | "I" "[" INT "]" | "R" "[" real-num "]" | "S" "[" hex "]" | "E" "[" hex "]" | "F" "[" INT "]" | "A" "[" value-csv "]" | "T" "[" hex "," "[" value-csv "]" "]"
real-num = INT | FLOAT
value-csv = {ifc-value {"," ifc-value}*}?
# `enc_ifc_value_list`: a SELF-bracketed comma list — every reference to this production already
# includes its own `[` `]`, never re-wrapped by the caller (matches `enc_entity`'s own format string
# exactly: `[id,hexname,ARGSLIST,COMPLEXLIST]` with no extra brackets around `ARGSLIST`).
ifc-value-list = "[" value-csv "]"

# `enc_complex_type`/`complex-item`: `[hexname,ARGSLIST]` (`ARGSLIST` = a self-bracketed
# `ifc-value-list`). `enc_complex_list`/`complex-list`: a SELF-bracketed comma list, same convention.
complex-item = "[" hex "," ifc-value-list "]"
complex-list = "[" complex-csv "]"
complex-csv = {complex-item {"," complex-item}*}?

# `enc_entity`: `[id,hexname,ARGSLIST,COMPLEXLIST]` — both list fields already self-bracketed.
entity = "[" INT "," hex "," ifc-value-list "," complex-list "]"
entity-list = "[" entity-csv "]"
entity-csv = {entity {"," entity}*}?

# `enc_ifc_header`/`ifc-header`: `[fileDescriptionList,fileNameList,fileSchemaList]` — three
# self-bracketed `ifc-value-list`s (IFC's `IfcHeader` stores each HEADER record as a raw
# `Vec<IfcValue>`, never a typed struct like STEP's own `StepFileDescription`/etc.).
ifc-header = "[" ifc-value-list "," ifc-value-list "," ifc-value-list "]"
# `enc_ifc_snapshot` (only reachable via `SetSnapshot`'s payload): `[schema,header,[entities]]`.
ifc-snapshot = "[" hex "," ifc-header "," entity-list "]"
```

Current:

```text
# 📖️ P2-FG1: real `OpText::print_op`/`parse_op` shape for `IfcMutation`, hand-rolled since
# `#[derive(dsl::DslOps)]` fails (real `cargo check` output — `IfcValue`/`IfcSnapshot`/`IfcEntity:
# DslField` unsatisfied, see `🧬️mutations/🦀️.rs`'s own doc comment). One `keyword
# key=value ...` line, space-separated, keys in the field's declared struct order — every literal
# below copied verbatim from `print_ifc_mutation`'s real match arms
# (`✏️s/…/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`), not
# invented. Replaces the old ABNF placeholder this file used to contain.
dialect grammar
grammar stdio.ifc.mutation
start document

document = set-snapshot | patch-snapshot | set-file-description | set-file-name | set-file-schema | insert-entity | remove-entity | set-entity-name | set-entity-arg | insert-entity-arg | remove-entity-arg

patch-snapshot = "patch-snapshot" "patch" "=" hex
set-snapshot = "set-snapshot" "snapshot" "=" ifc-snapshot
set-file-description = "set-file-description" "values" "=" ifc-value-list
set-file-name = "set-file-name" "values" "=" ifc-value-list
set-file-schema = "set-file-schema" "values" "=" ifc-value-list
insert-entity = "insert-entity" "index" "=" INT "entity" "=" entity
remove-entity = "remove-entity" "id" "=" INT
set-entity-name = "set-entity-name" "id" "=" INT "name" "=" hex
set-entity-arg = "set-entity-arg" "id" "=" INT "index" "=" INT "value" "=" ifc-value
insert-entity-arg = "insert-entity-arg" "id" "=" INT "index" "=" INT "value" "=" ifc-value
remove-entity-arg = "remove-entity-arg" "id" "=" INT "index" "=" INT

# `IfcValue`'s tag scheme (`enc_ifc_value`/`dec_ifc_value`): `U`/`D` Unset/Derived are BARE letters
# (no brackets — unlike STEP's own `StepValue`, per this artifact's own doc comment: "payload-free
# variants are the bare letter, never ambiguous since every token boundary is whitespace/`,`/`;`"),
# `I[n]` Integer, `R[n]` Real, `S[hex]` String, `E[hex]` Enum, `F[n]` reFerence, `A[items]`
# Aggregate (recursive, bare comma-list, no self-bracket), `T[hex,[items]]` TypedValue (recursive,
# name + its own bracketed item list).
ifc-value = "U" | "D" | "I" "[" INT "]" | "R" "[" real-num "]" | "S" "[" hex "]" | "E" "[" hex "]" | "F" "[" INT "]" | "A" "[" value-csv "]" | "T" "[" hex "," "[" value-csv "]" "]"
real-num = INT | FLOAT
value-csv = {ifc-value {"," ifc-value}*}?
# `enc_ifc_value_list`: a SELF-bracketed comma list — every reference to this production already
# includes its own `[` `]`, never re-wrapped by the caller (matches `enc_entity`'s own format string
# exactly: `[id,hexname,ARGSLIST,COMPLEXLIST]` with no extra brackets around `ARGSLIST`).
ifc-value-list = "[" value-csv "]"

# `enc_complex_type`/`complex-item`: `[hexname,ARGSLIST]` (`ARGSLIST` = a self-bracketed
# `ifc-value-list`). `enc_complex_list`/`complex-list`: a SELF-bracketed comma list, same convention.
complex-item = "[" hex "," ifc-value-list "]"
complex-list = "[" complex-csv "]"
complex-csv = {complex-item {"," complex-item}*}?

# `enc_entity`: `[id,hexname,ARGSLIST,COMPLEXLIST]` — both list fields already self-bracketed.
entity = "[" INT "," hex "," ifc-value-list "," complex-list "]"
entity-list = "[" entity-csv "]"
entity-csv = {entity {"," entity}*}?

# `enc_ifc_header`/`ifc-header`: `[fileDescriptionList,fileNameList,fileSchemaList]` — three
# self-bracketed `ifc-value-list`s (IFC's `IfcHeader` stores each HEADER record as a raw
# `Vec<IfcValue>`, never a typed struct like STEP's own `StepFileDescription`/etc.).
ifc-header = "[" ifc-value-list "," ifc-value-list "," ifc-value-list "]"
# `enc_ifc_snapshot` (only reachable via `SetSnapshot`'s payload): `[schema,header,[entities]]`.
ifc-snapshot = "[" hex "," ifc-header "," entity-list "]"
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📝️text/📖️.grammar.semio

Original SHA-256: `c7b3fcd04dd59f477a713c1cc739c4212ea2b3ccd8885662a077d9d3c46b6c4e`. Current SHA-256: `8822229432dee429c72c11f3aa2725513832f3f75cdd6e9bdc7b83df7742636b`.

Original:

```text
# 📖️ Real `OpText::print_op`/`parse_op` shape for `Ifc2x3Mutation`, hand-rolled since
# `#[derive(dsl::DslOps)]` fails (`Part21Value`/`Part21Instance`/`Ifc2x3Snapshot`: no `DslField` impl
# — identical root cause `4`'s own `IfcMutation` doc comment documents for the isomorphic shape).
# Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: replaces the prior
# literal-JSON `serde_json::to_string`/`from_str` shortcut (the LAST standard-specific
# `POLICY_STDIO_JSON_TRANSFER_BAN` violation named anywhere in this program's own census) AND the old
# ABNF placeholder this file used to contain. One `keyword key=value ...` line, space-separated —
# every literal below copied verbatim from `print_ifc2x3_mutation`'s real match arms.
dialect grammar
grammar stdio.ifc.2x3.mutation
start document

document = set-snapshot | upsert-instance | remove-instance | set-header

set-snapshot = "set-snapshot" "snapshot" "=" ifc2x3-snapshot
upsert-instance = "upsert-instance" "instance" "=" part21-instance
remove-instance = "remove-instance" "id" "=" INT
set-header = "set-header" "header" "=" part21-header

# `Part21Value`'s tag scheme (`enc_part21_value`/`dec_part21_value`) — same isomorphic shape `4`'s
# own `IfcValue` tag scheme uses: `U`/`D` Unset/Derived bare letters, `I[n]` Integer, `R[n]` Real,
# `S[hex]` Str, `E[hex]` Enum, `F[n]` reFerence, `A[items]` List (recursive), `T[hex,[items]]` Typed
# (recursive).
part21-value = "U" | "D" | "I" "[" INT "]" | "R" "[" real-num "]" | "S" "[" hex "]" | "E" "[" hex "]" | "F" "[" INT "]" | "A" "[" value-csv "]" | "T" "[" hex "," "[" value-csv "]" "]"
real-num = INT | FLOAT
value-csv = {part21-value {"," part21-value}*}?
# `enc_part21_value_list`: a SELF-bracketed comma list — every reference to this production already
# includes its own `[` `]`, never re-wrapped by the caller.
part21-value-list = "[" value-csv "]"

# `enc_part21_header`/`part21-header`: `[fileDescriptionList,fileNameList,fileSchemaList]` — three
# self-bracketed `part21-value-list`s (`Part21Header` stores each HEADER record as a raw
# `Vec<Part21Value>`, never a typed struct).
part21-header = "[" part21-value-list "," part21-value-list "," part21-value-list "]"

# `enc_part21_instance`/`part21-instance`: `[id,[entity,entity,...]]` — 1 entry for a simple
# instance, 2+ for a real spec-legal COMPLEX instance (ISO 10303-21 §4.2, `Part21Instance.entities`).
# Each entity is `[hexname,ARGSLIST]` (`ARGSLIST` = a self-bracketed `part21-value-list`).
entity-item = "[" hex "," part21-value-list "]"
entity-list = {entity-item {"," entity-item}*}?
part21-instance = "[" INT "," "[" entity-list "]" "]"
instance-list = "[" instance-csv "]"
instance-csv = {part21-instance {"," part21-instance}*}?

# `enc_ifc2x3_snapshot` (only reachable via `SetSnapshot`'s payload):
# `[schema,header,[instances],edmPreamble]`.
ifc2x3-snapshot = "[" hex "," part21-header "," instance-list "," optional-edm-preamble "]"
optional-edm-preamble = "[" "0" "]" | "[" "1" "," edm-preamble "]"
edm-preamble = "[" hex "," hex "," hex "," hex "," hex "," hex "," hex "," hex "," hex "," hex "," hex "," hex "," hex "," hex "," hex "," hex "]"
```

Current:

```text
# 📖️ Real `OpText::print_op`/`parse_op` shape for `Ifc2x3Mutation`, hand-rolled since
# `#[derive(dsl::DslOps)]` fails (`Part21Value`/`Part21Instance`/`Ifc2x3Snapshot`: no `DslField` impl
# — identical root cause `4`'s own `IfcMutation` doc comment documents for the isomorphic shape).
# Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: replaces the prior
# literal-JSON `serde_json::to_string`/`from_str` shortcut (the LAST standard-specific
# `POLICY_STDIO_JSON_TRANSFER_BAN` violation named anywhere in this program's own census) AND the old
# ABNF placeholder this file used to contain. One `keyword key=value ...` line, space-separated —
# every literal below copied verbatim from `print_ifc2x3_mutation`'s real match arms.
dialect grammar
grammar stdio.ifc.2x3.mutation
start document

document = set-snapshot | patch-snapshot | upsert-instance | remove-instance | set-header

patch-snapshot = "patch-snapshot" "patch" "=" hex
set-snapshot = "set-snapshot" "snapshot" "=" ifc2x3-snapshot
upsert-instance = "upsert-instance" "instance" "=" part21-instance
remove-instance = "remove-instance" "id" "=" INT
set-header = "set-header" "header" "=" part21-header

# `Part21Value`'s tag scheme (`enc_part21_value`/`dec_part21_value`) — same isomorphic shape `4`'s
# own `IfcValue` tag scheme uses: `U`/`D` Unset/Derived bare letters, `I[n]` Integer, `R[n]` Real,
# `S[hex]` Str, `E[hex]` Enum, `F[n]` reFerence, `A[items]` List (recursive), `T[hex,[items]]` Typed
# (recursive).
part21-value = "U" | "D" | "I" "[" INT "]" | "R" "[" real-num "]" | "S" "[" hex "]" | "E" "[" hex "]" | "F" "[" INT "]" | "A" "[" value-csv "]" | "T" "[" hex "," "[" value-csv "]" "]"
real-num = INT | FLOAT
value-csv = {part21-value {"," part21-value}*}?
# `enc_part21_value_list`: a SELF-bracketed comma list — every reference to this production already
# includes its own `[` `]`, never re-wrapped by the caller.
part21-value-list = "[" value-csv "]"

# `enc_part21_header`/`part21-header`: `[fileDescriptionList,fileNameList,fileSchemaList]` — three
# self-bracketed `part21-value-list`s (`Part21Header` stores each HEADER record as a raw
# `Vec<Part21Value>`, never a typed struct).
part21-header = "[" part21-value-list "," part21-value-list "," part21-value-list "]"

# `enc_part21_instance`/`part21-instance`: `[id,[entity,entity,...]]` — 1 entry for a simple
# instance, 2+ for a real spec-legal COMPLEX instance (ISO 10303-21 §4.2, `Part21Instance.entities`).
# Each entity is `[hexname,ARGSLIST]` (`ARGSLIST` = a self-bracketed `part21-value-list`).
entity-item = "[" hex "," part21-value-list "]"
entity-list = {entity-item {"," entity-item}*}?
part21-instance = "[" INT "," "[" entity-list "]" "]"
instance-list = "[" instance-csv "]"
instance-csv = {part21-instance {"," part21-instance}*}?

# `enc_ifc2x3_snapshot` (only reachable via `SetSnapshot`'s payload):
# `[schema,header,[instances],edmPreamble]`.
ifc2x3-snapshot = "[" hex "," part21-header "," instance-list "," optional-edm-preamble "]"
optional-edm-preamble = "[" "0" "]" | "[" "1" "," edm-preamble "]"
edm-preamble = "[" hex "," hex "," hex "," hex "," hex "," hex "," hex "," hex "," hex "," hex "," hex "," hex "," hex "," hex "," hex "," hex "]"
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/🖊️markup/🧬️schema/🧬️mutations/📝️text/📖️.grammar.semio

Original SHA-256: `bc6f2d5f9d2ce91a5ac8999f00da40609cbad838a8ed2c0e05c0e98564d5aaba`. Current SHA-256: `9b7cb94fe326b726283d4617d8821d9ee41d4e69528f0755e637c01cefcd8ec1`.

Original:

```text
dialect grammar
grammar stdio.bcf.mutations
extension bcf
start document
comment none

# FG-wave: real one-line `OpText::print_op`/`parse_op` shape ALREADY emitted by
# `../../🦀️.rs` (`print_bcf_mutation`/`parse_bcf_mutation`), replacing the F6-era
# `*OCTET` hex-dump placeholder. Grammar: `keyword arg=value arg=value ...` (space-separated), one
# alternative per `BcfMutation` variant -- every keyword/arg-name token below is copied verbatim
# from the real `format!(...)` call sites, not invented.
#
# `hex` (bare `hex`, referenced but deliberately NOT defined as a production) is the framework's
# built-in `hex` MACRO -- `📖️grammar-recipe.md` §1.3(f)/§3 pitfall 2, used here for every
# `enc_str`/`enc_bytes` (hex-encoded UTF-8 string / raw byte) field. `[0]`/`[1,x]` is
# `encode_option`'s uniform tri-state shape (`📖️grammar-recipe.md` §1.4); `viewpoint-ref`'s
# `Option<Option<String>>` nests it twice (`opt-opt-hex`).
document = set-snapshot | set-version | insert-topic | remove-topic | set-topic-markup | insert-comment | remove-comment | set-comment | insert-viewpoint | remove-viewpoint | set-viewpoint-camera | set-viewpoint-components | set-viewpoint-snapshot

set-snapshot = "set-snapshot" "snapshot" "=" bcf-snapshot
set-version = "set-version" "version" "=" hex
insert-topic = "insert-topic" "topic" "=" topic
remove-topic = "remove-topic" "guid" "=" hex
set-topic-markup = "set-topic-markup" "guid" "=" hex "title" "=" opt-hex "description" "=" opt-hex "status" "=" opt-hex "priority" "=" opt-hex "labels" "=" opt-label-list "creation-date" "=" opt-hex "creation-author" "=" opt-hex
insert-comment = "insert-comment" "topic-guid" "=" hex "comment" "=" bcf-comment
remove-comment = "remove-comment" "topic-guid" "=" hex "guid" "=" hex
set-comment = "set-comment" "topic-guid" "=" hex "guid" "=" hex "date" "=" opt-hex "author" "=" opt-hex "text" "=" opt-hex "viewpoint-ref" "=" opt-opt-hex
insert-viewpoint = "insert-viewpoint" "topic-guid" "=" hex "viewpoint" "=" viewpoint
remove-viewpoint = "remove-viewpoint" "topic-guid" "=" hex "guid" "=" hex
set-viewpoint-camera = "set-viewpoint-camera" "topic-guid" "=" hex "guid" "=" hex "camera" "=" opt-camera
set-viewpoint-components = "set-viewpoint-components" "topic-guid" "=" hex "guid" "=" hex "components" "=" opt-components
set-viewpoint-snapshot = "set-viewpoint-snapshot" "topic-guid" "=" hex "guid" "=" hex "snapshot" "=" opt-hex

# ── Tri-state wrappers (`encode_option`, `📖️grammar-recipe.md` §1.4) ─────────────────────────────
opt-hex = "[" "0" "]" | "[" "1" "," hex "]"
opt-opt-hex = "[" "0" "]" | "[" "1" "," opt-hex "]"
opt-label-list = "[" "0" "]" | "[" "1" "," label-list "]"
opt-camera = "[" "0" "]" | "[" "1" "," camera "]"
opt-components = "[" "0" "]" | "[" "1" "," components "]"
bool-lit = "0" | "1"
# 🧭️ A raw (non-hex-encoded) numeric literal -- `enc_point3`/`enc_camera`'s own `{}`-formatted f64
# fields (`p.x`/`field_of_view`/...) print as plain decimal text (Rust's `Display` for a
# whole-number f64 omits the trailing `.0`, e.g. `60.0` -> `"60"`), so this covers BOTH lexer
# shapes the same real value can take, matching the recipe's own `step-value = FLOAT | DOTENUM |
# INT` worked example for exactly this "may or may not have a fractional part" case.
bcf-float = INT | FLOAT

# ── Restated whole-item value shapes -- verbatim from this artifact's own `../🔺️diff/🦀️.rs`
# `enc_bcf_snapshot`/`enc_topic`/`enc_comment`/`enc_viewpoint`/`enc_camera`/`enc_point3`/
# `enc_components`/`enc_visibility`/`enc_coloring`/`enc_part` (reused, per that file's own
# `pub(crate)` primitives-reuse note, by THIS file's `OpText`/`OpBinary` impl too) ───────────────
label-list = "[" label-item* "]"
label-item = hex ","?

# `BcfSnapshot` -- `[schema,version,topics,parts]` (`enc_bcf_snapshot`, `SetSnapshot`'s whole-payload
# encoding only).
bcf-snapshot = "[" hex "," hex "," topics-list "," parts-list "]"
topics-list = "[" topic-item* "]"
topic-item = topic ","?
parts-list = "[" part-item* "]"
part-item = part ","?

# `BcfTopic` -- `[guid,title,description,status,priority,labels,creation-date,creation-author,comments,viewpoints]`.
topic = "[" hex "," hex "," hex "," hex "," hex "," label-list "," hex "," hex "," comments-list "," viewpoints-list "]"
comments-list = "[" comment-item* "]"
comment-item = bcf-comment ","?
viewpoints-list = "[" viewpoint-item* "]"
viewpoint-item = viewpoint ","?

# `BcfComment` -- `[guid,date,author,text,viewpoint-ref]`. Named `bcf-comment`, NOT the bare
# `comment` this value's own real name would suggest -- `comment` collides with the five RESERVED
# header-directive keywords (`extension`/`use`/`start`/`comment`/`string`,
# `📖️grammar-recipe.md` §3 pitfall 3); `parse_grammar`'s unified header/production loop parses ANY
# leading ident matching one of those five words as a header directive, WHEREVER it appears in the
# file (same collision xml's own grammar file's `xml-comment` rename documents).
bcf-comment = "[" hex "," hex "," hex "," hex "," opt-hex "]"

# `BcfViewpoint` -- `[guid,camera,components,snapshot]`.
viewpoint = "[" hex "," opt-camera "," opt-components "," opt-hex "]"

# `BcfCamera` -- `P[view-point,direction,up-vector,field-of-view]` (Perspective) /
# `O[...,view-to-world-scale]` (Orthogonal) -- single-letter tag prefix, the `xs:choice` made
# concrete (same convention docx's own `x-node` `E`/`T`/`D`/`M`/`P` tags use).
camera = "P" "[" point3 "," point3 "," point3 "," bcf-float "]" | "O" "[" point3 "," point3 "," point3 "," bcf-float "]"
point3 = "[" bcf-float "," bcf-float "," bcf-float "]"

# `BcfComponents` -- `[selection,visibility,coloring]`; `BcfVisibility` -- `[default,exceptions]`;
# `BcfColoring` -- `[color,components]`.
components = "[" label-list "," visibility "," coloring-list "]"
visibility = "[" bool-lit "," label-list "]"
coloring-list = "[" coloring-item* "]"
coloring-item = coloring ","?
coloring = "[" hex "," label-list "]"

# `BcfRawPart` -- `[name,data]`.
part = "[" hex "," hex "]"
```

Current:

```text
dialect grammar
grammar stdio.bcf.mutations
extension bcf
start document
comment none

# FG-wave: real one-line `OpText::print_op`/`parse_op` shape ALREADY emitted by
# `../../🦀️.rs` (`print_bcf_mutation`/`parse_bcf_mutation`), replacing the F6-era
# `*OCTET` hex-dump placeholder. Grammar: `keyword arg=value arg=value ...` (space-separated), one
# alternative per `BcfMutation` variant -- every keyword/arg-name token below is copied verbatim
# from the real `format!(...)` call sites, not invented.
#
# `hex` (bare `hex`, referenced but deliberately NOT defined as a production) is the framework's
# built-in `hex` MACRO -- `📖️grammar-recipe.md` §1.3(f)/§3 pitfall 2, used here for every
# `enc_str`/`enc_bytes` (hex-encoded UTF-8 string / raw byte) field. `[0]`/`[1,x]` is
# `encode_option`'s uniform tri-state shape (`📖️grammar-recipe.md` §1.4); `viewpoint-ref`'s
# `Option<Option<String>>` nests it twice (`opt-opt-hex`).
document = set-snapshot | patch-snapshot | set-version | insert-topic | remove-topic | set-topic-markup | insert-comment | remove-comment | set-comment | insert-viewpoint | remove-viewpoint | set-viewpoint-camera | set-viewpoint-components | set-viewpoint-snapshot

patch-snapshot = "patch-snapshot" "patch" "=" hex
set-snapshot = "set-snapshot" "snapshot" "=" bcf-snapshot
set-version = "set-version" "version" "=" hex
insert-topic = "insert-topic" "topic" "=" topic
remove-topic = "remove-topic" "guid" "=" hex
set-topic-markup = "set-topic-markup" "guid" "=" hex "title" "=" opt-hex "description" "=" opt-hex "status" "=" opt-hex "priority" "=" opt-hex "labels" "=" opt-label-list "creation-date" "=" opt-hex "creation-author" "=" opt-hex
insert-comment = "insert-comment" "topic-guid" "=" hex "comment" "=" bcf-comment
remove-comment = "remove-comment" "topic-guid" "=" hex "guid" "=" hex
set-comment = "set-comment" "topic-guid" "=" hex "guid" "=" hex "date" "=" opt-hex "author" "=" opt-hex "text" "=" opt-hex "viewpoint-ref" "=" opt-opt-hex
insert-viewpoint = "insert-viewpoint" "topic-guid" "=" hex "viewpoint" "=" viewpoint
remove-viewpoint = "remove-viewpoint" "topic-guid" "=" hex "guid" "=" hex
set-viewpoint-camera = "set-viewpoint-camera" "topic-guid" "=" hex "guid" "=" hex "camera" "=" opt-camera
set-viewpoint-components = "set-viewpoint-components" "topic-guid" "=" hex "guid" "=" hex "components" "=" opt-components
set-viewpoint-snapshot = "set-viewpoint-snapshot" "topic-guid" "=" hex "guid" "=" hex "snapshot" "=" opt-hex

# ── Tri-state wrappers (`encode_option`, `📖️grammar-recipe.md` §1.4) ─────────────────────────────
opt-hex = "[" "0" "]" | "[" "1" "," hex "]"
opt-opt-hex = "[" "0" "]" | "[" "1" "," opt-hex "]"
opt-label-list = "[" "0" "]" | "[" "1" "," label-list "]"
opt-camera = "[" "0" "]" | "[" "1" "," camera "]"
opt-components = "[" "0" "]" | "[" "1" "," components "]"
bool-lit = "0" | "1"
# 🧭️ A raw (non-hex-encoded) numeric literal -- `enc_point3`/`enc_camera`'s own `{}`-formatted f64
# fields (`p.x`/`field_of_view`/...) print as plain decimal text (Rust's `Display` for a
# whole-number f64 omits the trailing `.0`, e.g. `60.0` -> `"60"`), so this covers BOTH lexer
# shapes the same real value can take, matching the recipe's own `step-value = FLOAT | DOTENUM |
# INT` worked example for exactly this "may or may not have a fractional part" case.
bcf-float = INT | FLOAT

# ── Restated whole-item value shapes -- verbatim from this artifact's own `../🔺️diff/🦀️.rs`
# `enc_bcf_snapshot`/`enc_topic`/`enc_comment`/`enc_viewpoint`/`enc_camera`/`enc_point3`/
# `enc_components`/`enc_visibility`/`enc_coloring`/`enc_part` (reused, per that file's own
# `pub(crate)` primitives-reuse note, by THIS file's `OpText`/`OpBinary` impl too) ───────────────
label-list = "[" label-item* "]"
label-item = hex ","?

# `BcfSnapshot` -- `[schema,version,topics,parts]` (`enc_bcf_snapshot`, `SetSnapshot`'s whole-payload
# encoding only).
bcf-snapshot = "[" hex "," hex "," topics-list "," parts-list "]"
topics-list = "[" topic-item* "]"
topic-item = topic ","?
parts-list = "[" part-item* "]"
part-item = part ","?

# `BcfTopic` -- `[guid,title,description,status,priority,labels,creation-date,creation-author,comments,viewpoints]`.
topic = "[" hex "," hex "," hex "," hex "," hex "," label-list "," hex "," hex "," comments-list "," viewpoints-list "]"
comments-list = "[" comment-item* "]"
comment-item = bcf-comment ","?
viewpoints-list = "[" viewpoint-item* "]"
viewpoint-item = viewpoint ","?

# `BcfComment` -- `[guid,date,author,text,viewpoint-ref]`. Named `bcf-comment`, NOT the bare
# `comment` this value's own real name would suggest -- `comment` collides with the five RESERVED
# header-directive keywords (`extension`/`use`/`start`/`comment`/`string`,
# `📖️grammar-recipe.md` §3 pitfall 3); `parse_grammar`'s unified header/production loop parses ANY
# leading ident matching one of those five words as a header directive, WHEREVER it appears in the
# file (same collision xml's own grammar file's `xml-comment` rename documents).
bcf-comment = "[" hex "," hex "," hex "," hex "," opt-hex "]"

# `BcfViewpoint` -- `[guid,camera,components,snapshot]`.
viewpoint = "[" hex "," opt-camera "," opt-components "," opt-hex "]"

# `BcfCamera` -- `P[view-point,direction,up-vector,field-of-view]` (Perspective) /
# `O[...,view-to-world-scale]` (Orthogonal) -- single-letter tag prefix, the `xs:choice` made
# concrete (same convention docx's own `x-node` `E`/`T`/`D`/`M`/`P` tags use).
camera = "P" "[" point3 "," point3 "," point3 "," bcf-float "]" | "O" "[" point3 "," point3 "," point3 "," bcf-float "]"
point3 = "[" bcf-float "," bcf-float "," bcf-float "]"

# `BcfComponents` -- `[selection,visibility,coloring]`; `BcfVisibility` -- `[default,exceptions]`;
# `BcfColoring` -- `[color,components]`.
components = "[" label-list "," visibility "," coloring-list "]"
visibility = "[" bool-lit "," label-list "]"
coloring-list = "[" coloring-item* "]"
coloring-item = coloring ","?
coloring = "[" hex "," label-list "]"

# `BcfRawPart` -- `[name,data]`.
part = "[" hex "," hex "]"
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📝️text/📖️.grammar.semio

Original SHA-256: `0a938341bede253cbe619cce110f9a0446ffe8a89213f92310e7d686ea76b043`. Current SHA-256: `bb43e4aa0fc3ea957f98ec10fd728abd863c644f644968e4d9e3bf321a373be7`.

Original:

```text
# 📖️ P2-FG1: real `OpText::print_op`/`parse_op` shape for `StepMutation`, hand-rolled since
# `#[derive(dsl::DslOps)]` fails (real `cargo check` output, `StepEntity`/`StepValue: DslField`
# unsatisfied — see `🧬️mutations/🦀️.rs`'s own doc comment). One `keyword key=value ...`
# line, space-separated, keys in the field's declared struct order. Every literal below is copied
# verbatim from `print_step_mutation`'s real match arms
# (`✏️s/…/📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🦀️.rs`), not
# invented. Replaces the old serde-JSON `{"mutation":"...",...}` ABNF placeholder this file used to
# contain — that shape was never emitted by the real hand-rolled codec even before this rewrite.
dialect grammar
grammar stdio.step.mutation
start document

document = set-snapshot | set-file-description | set-file-name | set-file-schema | insert-entity | remove-entity | set-entity-name | set-entity-arg | insert-entity-arg | remove-entity-arg

set-snapshot = "set-snapshot" "snapshot" "=" step-snapshot
set-file-description = "set-file-description" "file-description" "=" file-description
set-file-name = "set-file-name" "file-name" "=" file-name
set-file-schema = "set-file-schema" "file-schema" "=" file-schema
insert-entity = "insert-entity" "index" "=" INT "entity" "=" entity
remove-entity = "remove-entity" "id" "=" INT
set-entity-name = "set-entity-name" "id" "=" INT "name" "=" hex
set-entity-arg = "set-entity-arg" "id" "=" INT "arg-index" "=" INT "value" "=" step-value
insert-entity-arg = "insert-entity-arg" "id" "=" INT "arg-index" "=" INT "value" "=" step-value
remove-entity-arg = "remove-entity-arg" "id" "=" INT "arg-index" "=" INT

# `StepValue`'s single-uppercase-letter tag scheme (`enc_value`/`dec_value`): `U[]` Unset, `D[]`
# Derived, `I[n]` Integer, `R[n]` Real (Rust's bare `Display` for `f64` — never forces a decimal
# point, unlike Part-21 text's own `format_real`, so a whole real prints INT-shaped), `S[hex]`
# String, `E[hex]` Enum, `F[n]` reFerence (`R` already taken by Real), `A[items]` Aggregate
# (recursive list), `T[hex,value]` TypedValue (recursive, one wrapped value).
step-value = "U" "[" "]" | "D" "[" "]" | "I" "[" INT "]" | "R" "[" real-num "]" | "S" "[" hex "]" | "E" "[" hex "]" | "F" "[" INT "]" | "A" "[" value-list "]" | "T" "[" hex "," step-value "]"
real-num = INT | FLOAT
value-list = {step-value {"," step-value}*}?

# `enc_entity`: `[id,hexname,[args],[complex]]`. `enc_complex`/`complex-item`: `[hexname,[args]]`.
entity = "[" INT "," hex "," "[" value-list "]" "," "[" complex-list "]" "]"
complex-list = {complex-item {"," complex-item}*}?
complex-item = "[" hex "," "[" value-list "]" "]"
entity-list = {entity {"," entity}*}?
hex-list = {hex {"," hex}*}?

# `enc_file_description`: `[[desc-strings],implementation-level]`.
file-description = "[" "[" hex-list "]" "," hex "]"
# `enc_file_name`: `[name,timestamp,[author],[organization],preprocessor,originating,authorization]`.
file-name = "[" hex "," hex "," "[" hex-list "]" "," "[" hex-list "]" "," hex "," hex "," hex "]"
# `enc_file_schema`: `[schema-strings]`.
file-schema = "[" hex-list "]"

# `enc_step_snapshot` (only reachable via `SetSnapshot`'s payload): `[schema,fileDescription,
# fileName,fileSchema,[entities]]`.
step-snapshot = "[" hex "," file-description "," file-name "," file-schema "," "[" entity-list "]" "]"
```

Current:

```text
# 📖️ P2-FG1: real `OpText::print_op`/`parse_op` shape for `StepMutation`, hand-rolled since
# `#[derive(dsl::DslOps)]` fails (real `cargo check` output, `StepEntity`/`StepValue: DslField`
# unsatisfied — see `🧬️mutations/🦀️.rs`'s own doc comment). One `keyword key=value ...`
# line, space-separated, keys in the field's declared struct order. Every literal below is copied
# verbatim from `print_step_mutation`'s real match arms
# (`✏️s/…/📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🦀️.rs`), not
# invented. Replaces the old serde-JSON `{"mutation":"...",...}` ABNF placeholder this file used to
# contain — that shape was never emitted by the real hand-rolled codec even before this rewrite.
dialect grammar
grammar stdio.step.mutation
start document

document = set-snapshot | patch-snapshot | set-file-description | set-file-name | set-file-schema | insert-entity | remove-entity | set-entity-name | set-entity-arg | insert-entity-arg | remove-entity-arg

patch-snapshot = "patch-snapshot" "patch" "=" hex
set-snapshot = "set-snapshot" "snapshot" "=" step-snapshot
set-file-description = "set-file-description" "file-description" "=" file-description
set-file-name = "set-file-name" "file-name" "=" file-name
set-file-schema = "set-file-schema" "file-schema" "=" file-schema
insert-entity = "insert-entity" "index" "=" INT "entity" "=" entity
remove-entity = "remove-entity" "id" "=" INT
set-entity-name = "set-entity-name" "id" "=" INT "name" "=" hex
set-entity-arg = "set-entity-arg" "id" "=" INT "arg-index" "=" INT "value" "=" step-value
insert-entity-arg = "insert-entity-arg" "id" "=" INT "arg-index" "=" INT "value" "=" step-value
remove-entity-arg = "remove-entity-arg" "id" "=" INT "arg-index" "=" INT

# `StepValue`'s single-uppercase-letter tag scheme (`enc_value`/`dec_value`): `U[]` Unset, `D[]`
# Derived, `I[n]` Integer, `R[n]` Real (Rust's bare `Display` for `f64` — never forces a decimal
# point, unlike Part-21 text's own `format_real`, so a whole real prints INT-shaped), `S[hex]`
# String, `E[hex]` Enum, `F[n]` reFerence (`R` already taken by Real), `A[items]` Aggregate
# (recursive list), `T[hex,value]` TypedValue (recursive, one wrapped value).
step-value = "U" "[" "]" | "D" "[" "]" | "I" "[" INT "]" | "R" "[" real-num "]" | "S" "[" hex "]" | "E" "[" hex "]" | "F" "[" INT "]" | "A" "[" value-list "]" | "T" "[" hex "," step-value "]"
real-num = INT | FLOAT
value-list = {step-value {"," step-value}*}?

# `enc_entity`: `[id,hexname,[args],[complex]]`. `enc_complex`/`complex-item`: `[hexname,[args]]`.
entity = "[" INT "," hex "," "[" value-list "]" "," "[" complex-list "]" "]"
complex-list = {complex-item {"," complex-item}*}?
complex-item = "[" hex "," "[" value-list "]" "]"
entity-list = {entity {"," entity}*}?
hex-list = {hex {"," hex}*}?

# `enc_file_description`: `[[desc-strings],implementation-level]`.
file-description = "[" "[" hex-list "]" "," hex "]"
# `enc_file_name`: `[name,timestamp,[author],[organization],preprocessor,originating,authorization]`.
file-name = "[" hex "," hex "," "[" hex-list "]" "," "[" hex-list "]" "," hex "," hex "," hex "]"
# `enc_file_schema`: `[schema-strings]`.
file-schema = "[" hex-list "]"

# `enc_step_snapshot` (only reachable via `SetSnapshot`'s payload): `[schema,fileDescription,
# fileName,fileSchema,[entities]]`.
step-snapshot = "[" hex "," file-description "," file-name "," file-schema "," "[" entity-list "]" "]"
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/📖️.grammar.semio

Original SHA-256: `66ea308bd5d8e9d931c877b80d63979a7dcc81ea9640a1c4844cf8ae1674b4b6`. Current SHA-256: `b2d91b36aca8396b9bbbed8a6a36ffa43441bec61d47a8f780e7710128c89cd4`.

Original:

```text
dialect grammar stdio.tsv.mutations
root = mutation
; wire text is the hand-rolled `keyword arg=value ...` token line `protocol::OpText` prints
; (see 🦀️.rs `print_tsv_mutation`/`parse_tsv_mutation`) — not JSON.
mutation = "set-snapshot" SP "snapshot=" hexsnapshot
         / "set-trailing-newline" SP "trailing-newline=" BIT
         / "set-line-ending" SP "line-ending=" ("lf" / "crlf")
         / "insert-row" SP "index=" INT SP "row=" hexrow
         / "remove-row" SP "index=" INT
         / "set-cell" SP "row-index=" INT SP "field-index=" INT SP "value=" hexstr
SP = " "
BIT = "0" / "1"
hexstr = *HEXDIG
hexrow = "[" *(hexstr ",") "]"
hexsnapshot = "[" hexstr "," BIT "," ("lf" / "crlf") ",[" *(hexrow ",") "]]"
```

Current:

```text
dialect grammar stdio.tsv.mutations
root = mutation
; wire text is the hand-rolled `keyword arg=value ...` token line `protocol::OpText` prints
; (see 🦀️.rs `print_tsv_mutation`/`parse_tsv_mutation`) — not JSON.
mutation = "set-snapshot" SP "snapshot=" hexsnapshot
         / "patch-snapshot" SP "patch=" hexstr
         / "set-trailing-newline" SP "trailing-newline=" BIT
         / "set-line-ending" SP "line-ending=" ("lf" / "crlf")
         / "insert-row" SP "index=" INT SP "row=" hexrow
         / "remove-row" SP "index=" INT
         / "set-cell" SP "row-index=" INT SP "field-index=" INT SP "value=" hexstr
SP = " "
BIT = "0" / "1"
hexstr = *HEXDIG
hexrow = "[" *(hexstr ",") "]"
hexsnapshot = "[" hexstr "," BIT "," ("lf" / "crlf") ",[" *(hexrow ",") "]]"
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📝️text/📖️.grammar.semio

Original SHA-256: `95e19b3f39da36c5b81983d87036c8a369cbfa30dd6e664f014b39451d4440d7`. Current SHA-256: `4d7e8c543a11c9aebea3f344bd590064d07972d67dc0d299b0bf55a3000d012a`.

Original:

```text
dialect grammar
grammar stdio.xlsx.mutations
extension xlsx
start document
comment none

# Real one-line `OpText::print_op`/`parse_op` shape of `../../🦀️.rs` (`print_xlsx_mutation`/
# `parse_xlsx_mutation`, `🔖️MutationCodec` region): `keyword arg=value arg=value ...`, one alternative
# per `XlsxMutation` variant, every keyword/arg-name token copied from the real `format!(...)` call sites.
#
# `set-snapshot`'s `snapshot` and `set-cell`/`remove-cell`'s `address` carry the lowercase hex of the
# value's RFC 8259 JSON (`enc_xlsx_snapshot`/`enc_cell_address`: `enc_str(dsl::json::to_json_string(..))`),
# so both are the framework's built-in `hex` MACRO (bare `hex`, deliberately not defined as a production --
# `📖️grammar-recipe.md` §1.3(f)/§3 pitfall 2). `[0]`/`[1,x]` is `encode_option`'s tri-state shape (§1.4).
document = set-snapshot | insert-sheet | remove-sheet | rename-sheet | set-cell | remove-cell | insert-shared-string | remove-shared-string | set-shared-string

set-snapshot = "set-snapshot" "snapshot" "=" hex
insert-sheet = "insert-sheet" "sheet" "=" sheet
remove-sheet = "remove-sheet" "name" "=" hex
rename-sheet = "rename-sheet" "name" "=" hex "new-name" "=" hex
set-cell = "set-cell" "address" "=" hex "value" "=" cell-value
remove-cell = "remove-cell" "address" "=" hex
insert-shared-string = "insert-shared-string" "value" "=" hex
remove-shared-string = "remove-shared-string" "index" "=" INT
set-shared-string = "set-shared-string" "index" "=" INT "value" "=" hex

# ── `XlsxSheet` -- `[name,[cells]]` (`enc_sheet`) ────────────────────────────────────────────────
sheet = "[" hex "," "[" cell-list? "]" "]"
cell-list = cell {"," cell}*
cell = "[" INT "," INT "," cell-value "]"

# ── `XlsxCellValue` (`enc_cell_value`) -- single-letter tag immediately followed by the bracketed
# positional payload: `N`umber, `S`hared-string index, `I`nline string, `B`oolean, e`R`ror, `F`ormula
# with its optional cached value, `E`mpty. ────────────────────────────────────────────────────────────
cell-value = "N" "[" number "]" | "S" "[" INT "]" | "I" "[" hex "]" | "B" "[" bool-lit "]" | "R" "[" hex "]" | "F" "[" hex "," opt-cell-value "]" | "E" "[" "]"
opt-cell-value = "[" "0" "]" | "[" "1" "," cell-value "]"
bool-lit = "0" | "1"
number = INT | FLOAT
```

Current:

```text
dialect grammar
grammar stdio.xlsx.mutations
extension xlsx
start document
comment none

# Real one-line `OpText::print_op`/`parse_op` shape of `../../🦀️.rs` (`print_xlsx_mutation`/
# `parse_xlsx_mutation`, `🔖️MutationCodec` region): `keyword arg=value arg=value ...`, one alternative
# per `XlsxMutation` variant, every keyword/arg-name token copied from the real `format!(...)` call sites.
#
# `set-snapshot`'s `snapshot` and `set-cell`/`remove-cell`'s `address` carry the lowercase hex of the
# value's RFC 8259 JSON (`enc_xlsx_snapshot`/`enc_cell_address`: `enc_str(dsl::json::to_json_string(..))`),
# so both are the framework's built-in `hex` MACRO (bare `hex`, deliberately not defined as a production --
# `📖️grammar-recipe.md` §1.3(f)/§3 pitfall 2). `[0]`/`[1,x]` is `encode_option`'s tri-state shape (§1.4).
document = set-snapshot | patch-snapshot | insert-sheet | remove-sheet | rename-sheet | set-cell | remove-cell | insert-shared-string | remove-shared-string | set-shared-string

patch-snapshot = "patch-snapshot" "patch" "=" hex
set-snapshot = "set-snapshot" "snapshot" "=" hex
insert-sheet = "insert-sheet" "sheet" "=" sheet
remove-sheet = "remove-sheet" "name" "=" hex
rename-sheet = "rename-sheet" "name" "=" hex "new-name" "=" hex
set-cell = "set-cell" "address" "=" hex "value" "=" cell-value
remove-cell = "remove-cell" "address" "=" hex
insert-shared-string = "insert-shared-string" "value" "=" hex
remove-shared-string = "remove-shared-string" "index" "=" INT
set-shared-string = "set-shared-string" "index" "=" INT "value" "=" hex

# ── `XlsxSheet` -- `[name,[cells]]` (`enc_sheet`) ────────────────────────────────────────────────
sheet = "[" hex "," "[" cell-list? "]" "]"
cell-list = cell {"," cell}*
cell = "[" INT "," INT "," cell-value "]"

# ── `XlsxCellValue` (`enc_cell_value`) -- single-letter tag immediately followed by the bracketed
# positional payload: `N`umber, `S`hared-string index, `I`nline string, `B`oolean, e`R`ror, `F`ormula
# with its optional cached value, `E`mpty. ────────────────────────────────────────────────────────────
cell-value = "N" "[" number "]" | "S" "[" INT "]" | "I" "[" hex "]" | "B" "[" bool-lit "]" | "R" "[" hex "]" | "F" "[" hex "," opt-cell-value "]" | "E" "[" "]"
opt-cell-value = "[" "0" "]" | "[" "1" "," cell-value "]"
bool-lit = "0" | "1"
number = INT | FLOAT
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📝️text/📖️.grammar.semio

Original SHA-256: `0218594cfa749d999309982ccc68a0a24ffac436c9d84db9349970e6d63a8977`. Current SHA-256: `5682d5797385571f2cdafd17ed8aa5cf193ce2ab25280fa684ae72dbdf987d97`.

Original:

```text
dialect grammar
grammar stdio.docx.mutations
extension docx
start document
comment none

# Real one-line `OpText::print_op`/`parse_op` shape of `../../🦀️.rs` (`print_docx_mutation`/
# `parse_docx_mutation`): `keyword arg=value arg=value ...`, one alternative per `DocxMutation` variant,
# every keyword/arg-name token copied from the real `format!(...)` call sites.
#
# `set-snapshot`'s `snapshot` and every XML `address`/`parent` carry the lowercase hex of the value's
# RFC 8259 JSON (`enc_docx_snapshot`/`enc_xml_address`: `hex_encode(dsl::json::to_json_string(..))`), so
# they are the framework's built-in `hex` MACRO (bare `hex`, deliberately not defined as a production --
# `📖️grammar-recipe.md` §1.3(f)/§3 pitfall 2). `[0]`/`[1,x]` is `encode_option`'s tri-state shape (§1.4).
document = set-snapshot | insert-block | remove-block | set-block-content | set-run-text | replace-xml-node | set-run-formatting | set-paragraph-style | insert-table-row | remove-table-row | insert-xml-node | remove-xml-node | insert-style | remove-style | set-style-name | set-style-based-on | set-part | remove-part

set-snapshot = "set-snapshot" "snapshot" "=" hex
insert-block = "insert-block" "path" "=" block-path "block" "=" block
remove-block = "remove-block" "path" "=" block-path
set-block-content = "set-block-content" "path" "=" block-path "block" "=" block
set-run-text = "set-run-text" "address" "=" hex "text" "=" hex
replace-xml-node = "replace-xml-node" "address" "=" hex "node" "=" x-node
set-run-formatting = "set-run-formatting" "address" "=" hex "bold" "=" bool-lit "italic" "=" bool-lit "underline" "=" bool-lit
set-paragraph-style = "set-paragraph-style" "address" "=" hex "style-id" "=" opt-hex
insert-table-row = "insert-table-row" "address" "=" hex "index" "=" INT "cells" "=" text-list
remove-table-row = "remove-table-row" "address" "=" hex "index" "=" INT
insert-xml-node = "insert-xml-node" "parent" "=" hex "index" "=" INT "node" "=" x-node
remove-xml-node = "remove-xml-node" "parent" "=" hex "index" "=" INT "expected-name" "=" hex "revision" "=" hex
insert-style = "insert-style" "style" "=" style
remove-style = "remove-style" "id" "=" hex
set-style-name = "set-style-name" "id" "=" hex "name" "=" hex
set-style-based-on = "set-style-based-on" "id" "=" hex "based-on" "=" opt-hex
set-part = "set-part" "path" "=" hex "content-type" "=" hex "bytes" "=" hex
remove-part = "remove-part" "path" "=" hex

bool-lit = "0" | "1"
opt-hex = "[" "0" "]" | "[" "1" "," hex "]"
text-list = "[" text-item* "]"
text-item = hex ","?

# ── `DocxBlockPath` -- `[[segment,...],index]` (`enc_block_path`) ────────────────────────────────
block-path = "[" segments-list "," INT "]"
segments-list = "[" segment-item* "]"
segment-item = segment ","?
segment = "[" INT "," INT "," INT "]"

# ── Whole-item value shapes `print_docx_mutation` reuses from `../🔺️diff/🦀️.rs` (`enc_block`/`enc_paragraph`/
# `enc_run`/`enc_table`/`enc_style`, positional `[...]` records, `P`/`T` block tags) ──────────────────────
block = "P" paragraph | "T" table
blocks-list = "[" block-item* "]"
block-item = block ","?
paragraph = "[" runs-list "," opt-hex "," extra-list "]"
runs-list = "[" run-item* "]"
run-item = run ","?
run = "[" hex "," bool-lit "," bool-lit "," bool-lit "," extra-list "]"
extra-list = "[" x-node-item* "]"
x-node-item = x-node ","?
table = "[" rows-list "," extra-list "]"
rows-list = "[" row-item* "]"
row-item = row ","?
row = "[" cells-list "," extra-list "]"
cells-list = "[" cell-item* "]"
cell-item = cell ","?
cell = "[" blocks-list "," extra-list "]"
style = "[" hex "," hex "," opt-hex "]"

# `XmlNode` value shape (`enc_xml_node`): `E`lement `[name,[attrs],[children]]`, `T`ext, `D`ata (CDATA), com`M`ent,
# `P`rocessing instruction `[target,data]`.
x-node = "E" "[" hex "," "[" x-attr-item* "]" "," "[" x-node-item* "]" "]" | "T" "[" hex "]" | "D" "[" hex "]" | "M" "[" hex "]" | "P" "[" hex "," hex "]"
x-attr-item = "[" hex "," hex "]" ","?
```

Current:

```text
dialect grammar
grammar stdio.docx.mutations
extension docx
start document
comment none

# Real one-line `OpText::print_op`/`parse_op` shape of `../../🦀️.rs` (`print_docx_mutation`/
# `parse_docx_mutation`): `keyword arg=value arg=value ...`, one alternative per `DocxMutation` variant,
# every keyword/arg-name token copied from the real `format!(...)` call sites.
#
# `set-snapshot`'s `snapshot` and every XML `address`/`parent` carry the lowercase hex of the value's
# RFC 8259 JSON (`enc_docx_snapshot`/`enc_xml_address`: `hex_encode(dsl::json::to_json_string(..))`), so
# they are the framework's built-in `hex` MACRO (bare `hex`, deliberately not defined as a production --
# `📖️grammar-recipe.md` §1.3(f)/§3 pitfall 2). `[0]`/`[1,x]` is `encode_option`'s tri-state shape (§1.4).
document = set-snapshot | patch-snapshot | insert-block | remove-block | set-block-content | set-run-text | replace-xml-node | set-run-formatting | set-paragraph-style | insert-table-row | remove-table-row | insert-xml-node | remove-xml-node | insert-style | remove-style | set-style-name | set-style-based-on | set-part | remove-part

patch-snapshot = "patch-snapshot" "patch" "=" hex
set-snapshot = "set-snapshot" "snapshot" "=" hex
insert-block = "insert-block" "path" "=" block-path "block" "=" block
remove-block = "remove-block" "path" "=" block-path
set-block-content = "set-block-content" "path" "=" block-path "block" "=" block
set-run-text = "set-run-text" "address" "=" hex "text" "=" hex
replace-xml-node = "replace-xml-node" "address" "=" hex "node" "=" x-node
set-run-formatting = "set-run-formatting" "address" "=" hex "bold" "=" bool-lit "italic" "=" bool-lit "underline" "=" bool-lit
set-paragraph-style = "set-paragraph-style" "address" "=" hex "style-id" "=" opt-hex
insert-table-row = "insert-table-row" "address" "=" hex "index" "=" INT "cells" "=" text-list
remove-table-row = "remove-table-row" "address" "=" hex "index" "=" INT
insert-xml-node = "insert-xml-node" "parent" "=" hex "index" "=" INT "node" "=" x-node
remove-xml-node = "remove-xml-node" "parent" "=" hex "index" "=" INT "expected-name" "=" hex "revision" "=" hex
insert-style = "insert-style" "style" "=" style
remove-style = "remove-style" "id" "=" hex
set-style-name = "set-style-name" "id" "=" hex "name" "=" hex
set-style-based-on = "set-style-based-on" "id" "=" hex "based-on" "=" opt-hex
set-part = "set-part" "path" "=" hex "content-type" "=" hex "bytes" "=" hex
remove-part = "remove-part" "path" "=" hex

bool-lit = "0" | "1"
opt-hex = "[" "0" "]" | "[" "1" "," hex "]"
text-list = "[" text-item* "]"
text-item = hex ","?

# ── `DocxBlockPath` -- `[[segment,...],index]` (`enc_block_path`) ────────────────────────────────
block-path = "[" segments-list "," INT "]"
segments-list = "[" segment-item* "]"
segment-item = segment ","?
segment = "[" INT "," INT "," INT "]"

# ── Whole-item value shapes `print_docx_mutation` reuses from `../🔺️diff/🦀️.rs` (`enc_block`/`enc_paragraph`/
# `enc_run`/`enc_table`/`enc_style`, positional `[...]` records, `P`/`T` block tags) ──────────────────────
block = "P" paragraph | "T" table
blocks-list = "[" block-item* "]"
block-item = block ","?
paragraph = "[" runs-list "," opt-hex "," extra-list "]"
runs-list = "[" run-item* "]"
run-item = run ","?
run = "[" hex "," bool-lit "," bool-lit "," bool-lit "," extra-list "]"
extra-list = "[" x-node-item* "]"
x-node-item = x-node ","?
table = "[" rows-list "," extra-list "]"
rows-list = "[" row-item* "]"
row-item = row ","?
row = "[" cells-list "," extra-list "]"
cells-list = "[" cell-item* "]"
cell-item = cell ","?
cell = "[" blocks-list "," extra-list "]"
style = "[" hex "," hex "," opt-hex "]"

# `XmlNode` value shape (`enc_xml_node`): `E`lement `[name,[attrs],[children]]`, `T`ext, `D`ata (CDATA), com`M`ent,
# `P`rocessing instruction `[target,data]`.
x-node = "E" "[" hex "," "[" x-attr-item* "]" "," "[" x-node-item* "]" "]" | "T" "[" hex "]" | "D" "[" hex "]" | "M" "[" hex "]" | "P" "[" hex "," hex "]"
x-attr-item = "[" hex "," hex "]" ","?
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔺️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/📖️.grammar.semio

Original SHA-256: `4a172592b5cca716bd42a38d635222fa35610437cc5a9d681523c956615c8107`. Current SHA-256: `3710ba60f78bc999e163e35fc19a1132d6c507aca52fae41d07d340f4fe8f59d`.

Original:

```text
dialect grammar
grammar stdio.stl.mutations
extension stl
start document

# FG1: real `OpText::print_op`/`parse_op` shape — F6's hand-rolled `print_stl_op`/`parse_stl_op`
# (🧬️mutations/🦀️.rs), replacing the F6-era placeholder that described a serde-JSON
# `{"mutation":"...", ...}` wire struct the codec never emits (F6 reverted the `dsl::DslOps` derive
# after a real, reproduced `dsl`-grammar bug — see `f6-stl-report.md` and this artifact's own
# `🧬️mutations`/`🔺️diff` module doc comments for the full root-cause citation; out of this wave's
# ownership boundary to fix). One keyword line, space-separated `key=value` tokens, every variant's
# args ALWAYS present (no sparse omission, unlike the sibling diff grammar). Every keyword/field-key
# token below is copied verbatim from `print_stl_op`'s real match arms, not invented:
#   set-snapshot snapshot=[<hex>,<hex>,[<triangle>,...]]
#   set-solid-name name=<hex>
#   insert-triangle index=<INT> triangle=[<vec3>,<vertices>]
#   remove-triangle index=<INT>
#   set-triangle-normal index=<INT> normal=<vec3>
#   set-triangle-vertices index=<INT> vertices=<vertices>
# `name`/`solid_name`/`schema` (inside `snapshot=`) are lowercase hex (`hex_encode_str`) — the
# mandatory `hex` MACRO (bare `hex`, no matching production — `Symbol::Ref` falls back
# production-then-macro), never a hand-rolled `{INT|IDENT}*` (P2-P1-fix Bug 3). `normal`/`vertices`/
# `triangle`/`snapshot`'s numeric payload reuses the exact `[...]`-bracketed grammar
# `🔺️diff::component`'s `enc_vec3`/`enc_vertices`/`enc_triangle` primitives emit — every array level
# gets its own bracket (the depth marker `dsl`'s own flat `Shape::Tuple` printer is missing), which
# is exactly what sidesteps that bug: `split_top_level`'s bracket-depth-aware comma split recovers
# nesting unambiguously.
document = set-snapshot | set-solid-name | insert-triangle | remove-triangle | set-triangle-normal | set-triangle-vertices

set-snapshot = "set-snapshot" "snapshot" "=" snapshot-value
set-solid-name = "set-solid-name" "name" "=" hex
insert-triangle = "insert-triangle" "index" "=" INT "triangle" "=" triangle-value
remove-triangle = "remove-triangle" "index" "=" INT
set-triangle-normal = "set-triangle-normal" "index" "=" INT "normal" "=" vec3-value
set-triangle-vertices = "set-triangle-vertices" "index" "=" INT "vertices" "=" vertices-value

# `enc_snapshot`: `[hex(schema),hex(solid_name),[triangle,triangle,...]]` — the one type
# `🔺️diff::component`'s own primitives don't need (only `SetSnapshot`'s payload does), defined in
# `🧬️mutations::component`'s own `enc_snapshot`/`dec_snapshot`.
snapshot-value = "[" hex "," hex "," "[" triangle-list? "]" "]"
triangle-list = triangle-value {"," triangle-value}*

# `enc_triangle`/`enc_vertices`/`enc_vec3` — copied verbatim from `🔺️diff::component`'s own value
# grammar (see that facet's grammar for the identical productions, reused here since `stl`'s
# mutations facet reuses `diff`'s codec primitives directly, same intra-artifact pattern `svg`'s
# `SvgMutation` uses over `SvgDiff`'s primitives).
triangle-value = "[" vec3-value "," vertices-value "]"
vertices-value = "[" vec3-value "," vec3-value "," vec3-value "]"
vec3-value = "[" number "," number "," number "]"
number = INT | FLOAT
```

Current:

```text
dialect grammar
grammar stdio.stl.mutations
extension stl
start document

# FG1: real `OpText::print_op`/`parse_op` shape — F6's hand-rolled `print_stl_op`/`parse_stl_op`
# (🧬️mutations/🦀️.rs), replacing the F6-era placeholder that described a serde-JSON
# `{"mutation":"...", ...}` wire struct the codec never emits (F6 reverted the `dsl::DslOps` derive
# after a real, reproduced `dsl`-grammar bug — see `f6-stl-report.md` and this artifact's own
# `🧬️mutations`/`🔺️diff` module doc comments for the full root-cause citation; out of this wave's
# ownership boundary to fix). One keyword line, space-separated `key=value` tokens, every variant's
# args ALWAYS present (no sparse omission, unlike the sibling diff grammar). Every keyword/field-key
# token below is copied verbatim from `print_stl_op`'s real match arms, not invented:
#   set-snapshot snapshot=[<hex>,<hex>,[<triangle>,...]]
#   set-solid-name name=<hex>
#   insert-triangle index=<INT> triangle=[<vec3>,<vertices>]
#   remove-triangle index=<INT>
#   set-triangle-normal index=<INT> normal=<vec3>
#   set-triangle-vertices index=<INT> vertices=<vertices>
# `name`/`solid_name`/`schema` (inside `snapshot=`) are lowercase hex (`hex_encode_str`) — the
# mandatory `hex` MACRO (bare `hex`, no matching production — `Symbol::Ref` falls back
# production-then-macro), never a hand-rolled `{INT|IDENT}*` (P2-P1-fix Bug 3). `normal`/`vertices`/
# `triangle`/`snapshot`'s numeric payload reuses the exact `[...]`-bracketed grammar
# `🔺️diff::component`'s `enc_vec3`/`enc_vertices`/`enc_triangle` primitives emit — every array level
# gets its own bracket (the depth marker `dsl`'s own flat `Shape::Tuple` printer is missing), which
# is exactly what sidesteps that bug: `split_top_level`'s bracket-depth-aware comma split recovers
# nesting unambiguously.
document = set-snapshot | patch-snapshot | set-solid-name | insert-triangle | remove-triangle | set-triangle-normal | set-triangle-vertices

patch-snapshot = "patch-snapshot" "patch" "=" hex
set-snapshot = "set-snapshot" "snapshot" "=" snapshot-value
set-solid-name = "set-solid-name" "name" "=" hex
insert-triangle = "insert-triangle" "index" "=" INT "triangle" "=" triangle-value
remove-triangle = "remove-triangle" "index" "=" INT
set-triangle-normal = "set-triangle-normal" "index" "=" INT "normal" "=" vec3-value
set-triangle-vertices = "set-triangle-vertices" "index" "=" INT "vertices" "=" vertices-value

# `enc_snapshot`: `[hex(schema),hex(solid_name),[triangle,triangle,...]]` — the one type
# `🔺️diff::component`'s own primitives don't need (only `SetSnapshot`'s payload does), defined in
# `🧬️mutations::component`'s own `enc_snapshot`/`dec_snapshot`.
snapshot-value = "[" hex "," hex "," "[" triangle-list? "]" "]"
triangle-list = triangle-value {"," triangle-value}*

# `enc_triangle`/`enc_vertices`/`enc_vec3` — copied verbatim from `🔺️diff::component`'s own value
# grammar (see that facet's grammar for the identical productions, reused here since `stl`'s
# mutations facet reuses `diff`'s codec primitives directly, same intra-artifact pattern `svg`'s
# `SvgMutation` uses over `SvgDiff`'s primitives).
triangle-value = "[" vec3-value "," vertices-value "]"
vertices-value = "[" vec3-value "," vec3-value "," vec3-value "]"
vec3-value = "[" number "," number "," number "]"
number = INT | FLOAT
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🧬️schema/🧬️mutations/📝️text/📖️.grammar.semio

Original SHA-256: `c69dd9fb70e984a90e8dd81c25fc44a1b6fb560e2c3bfaf8fb691e2eb1170a34`. Current SHA-256: `3da99664d9bffb4de07666b062a22722ba1b742c071705d1c3742a6fa7b9c9fc`.

Original:

```text
dialect grammar
grammar stdio.dxf.mutations
extension dxf
start document

# P2-FG1: real one-line `OpText::print_op`/`parse_op` shape ALREADY emitted by
# `🏅️standards/🔖️r12/🪆️subsets/📰️header/🧬️schema/🧬️mutations/🦀️.rs`'s `print_dxf_mutation`/
# `parse_dxf_mutation` (`keyword arg=value ...`, space-separated, `=`-joined tokens — F6's own
# hand-rolled convention, reusing `🔺️diff`'s `pub(crate)` item-literal encoders `enc_header_var`/
# `enc_layer`/`enc_style`/`enc_linetype`/`enc_dxf_entity`/`enc_block`/`enc_dxf_snapshot` verbatim,
# per that file's own module doc comment). Every keyword/field-name literal below is copied from
# the real `format!(...)`/`match keyword` call sites, not invented.
#
# `hex` (bare `hex`, deliberately NOT a production in this file) is the framework's built-in `hex`
# MACRO (`📖️grammar/🦀️.rs`'s `default_macros`) — `Symbol::Ref` falls back
# production→macro, `Recognizer::match_macro_span` tries the largest token span first and
# backtracks correctly, which is why a hand-rolled `{INT|IDENT}*` production is wrong here (P2
# recipe pitfall #2: a hex run mixing digits/letters fragments across INT/IDENT tokens at the
# lexer level, and a naive Star has no backtracking to stop short of a following bareword literal
# — `hex` the macro handles both, validated end-to-end by this file's own `ops_grammar_conformance_law`).
#
# Every nested item literal below (`header-var`/`layer-item`/`style-item`/`linetype-item`/
# `dxf-entity`/`block-item`/`dxf-snapshot`) mirrors `🔺️diff/🦀️.rs`'s real `enc_*`/`dec_*`
# functions field-for-field, in the SAME positional order those functions emit — see that file's
# `#region 🔖️ItemCodecs`/`#region 🔖️EntityValueCodecs`. `dxf-entity`'s 8-arm tag-prefixed shape
# (`L`=Line,`C`=Circle,`A`=Arc,`W`=Polyline,`T`=Text,`S`=Solid,`I`=Insert,`O`=Other) is bounded, not
# self-recursive — a `block-item`'s own `entity-list` references `dxf-entity`, but no `DxfEntity`
# variant nests another `DxfEntity` (R12 has no entity-in-entity nesting), so this whole grammar
# has a fixed maximum depth, genuinely exercised end-to-end (incl. every entity kind, a POLYLINE
# with vertices, and a `SetSnapshot` payload nesting a block-with-entity) by
# `ops_grammar_conformance_law` against real `print_op` output for every `DxfMutation` variant.
document = set-snapshot | set-header-var | remove-header-var | insert-layer | remove-layer | set-layer | insert-style | remove-style | set-style | insert-linetype | remove-linetype | set-linetype | insert-entity | remove-entity | set-entity | insert-block | remove-block | set-block

set-snapshot = "set-snapshot" "snapshot" "=" dxf-snapshot

set-header-var = "set-header-var" "name" "=" hex "header-var" "=" header-var
remove-header-var = "remove-header-var" "name" "=" hex

insert-layer = "insert-layer" "index" "=" INT "layer" "=" layer-item
remove-layer = "remove-layer" "name" "=" hex
set-layer = "set-layer" "name" "=" hex "layer" "=" layer-item

insert-style = "insert-style" "index" "=" INT "style" "=" style-item
remove-style = "remove-style" "name" "=" hex
set-style = "set-style" "name" "=" hex "style" "=" style-item

insert-linetype = "insert-linetype" "index" "=" INT "linetype" "=" linetype-item
remove-linetype = "remove-linetype" "name" "=" hex
set-linetype = "set-linetype" "name" "=" hex "linetype" "=" linetype-item

insert-entity = "insert-entity" "index" "=" INT "entity" "=" dxf-entity
remove-entity = "remove-entity" "index" "=" INT
set-entity = "set-entity" "index" "=" INT "entity" "=" dxf-entity

insert-block = "insert-block" "index" "=" INT "block" "=" block-item
remove-block = "remove-block" "index" "=" INT
set-block = "set-block" "index" "=" INT "block" "=" block-item

# `enc_f64` (`format!("{v}")`) prints a bare decimal, lexing as `INT` (no `.`) or `FLOAT` (has `.`).
num = INT | FLOAT
point3 = "[" num "," num "," num "]"
points4 = "[" point3 "," point3 "," point3 "," point3 "]"

# `enc_dxf_value`/`dec_dxf_value` — `S[hex]`/`I[digits]`/`D[float]`/`P[x,y,z]`.
dxf-value = "S" "[" hex "]" | "I" "[" INT "]" | "D" "[" num "]" | "P" "[" num "," num "," num "]"
group-code = "[" INT "," dxf-value "]"
group-codes = "[" "]" | "[" group-code {"," group-code}* "]"

vertex = "[" num "," num "," num "," num "," group-codes "]"
vertex-list = "[" "]" | "[" vertex {"," vertex}* "]"

# `enc_dxf_entity`/`dec_dxf_entity` — one tag letter per typed `DxfEntity` variant.
dxf-entity = "L" "[" point3 "," point3 "," hex "," group-codes "]" | "C" "[" point3 "," num "," hex "," group-codes "]" | "A" "[" point3 "," num "," num "," num "," hex "," group-codes "]" | "W" "[" vertex-list "," INT "," hex "," group-codes "]" | "T" "[" point3 "," num "," hex "," hex "," group-codes "]" | "S" "[" points4 "," hex "," group-codes "]" | "I" "[" hex "," point3 "," point3 "," num "," hex "," group-codes "]" | "O" "[" hex "," group-codes "]"
entity-list = "[" "]" | "[" dxf-entity {"," dxf-entity}* "]"

header-var = "[" hex "," INT "," dxf-value "," group-codes "]"
header-var-list = "[" "]" | "[" header-var {"," header-var}* "]"

layer-item = "[" hex "," INT "," hex "," INT "," group-codes "]"
layer-list = "[" "]" | "[" layer-item {"," layer-item}* "]"

style-item = "[" hex "," INT "," hex "," group-codes "]"
style-list = "[" "]" | "[" style-item {"," style-item}* "]"

linetype-item = "[" hex "," INT "," hex "," group-codes "]"
linetype-list = "[" "]" | "[" linetype-item {"," linetype-item}* "]"

block-item = "[" hex "," point3 "," entity-list "," group-codes "]"
block-list = "[" "]" | "[" block-item {"," block-item}* "]"

dxf-tag = "[" INT "," hex "]"
dxf-tag-list = "[" "]" | "[" dxf-tag {"," dxf-tag}* "]"
other-table = "[" hex "," dxf-tag-list "]"
other-table-list = "[" "]" | "[" other-table {"," other-table}* "]"

dxf-tables = "[" layer-list "," style-list "," linetype-list "]"

# `enc_dxf_snapshot`/`dec_dxf_snapshot` — the WHOLE snapshot, only reachable via `SetSnapshot`'s
# own payload (`🔺️diff/🦀️.rs`'s own doc comment on `enc_dxf_snapshot` names this exactly).
dxf-snapshot = "[" hex "," header-var-list "," dxf-tables "," other-table-list "," block-list "," entity-list "]"
```

Current:

```text
dialect grammar
grammar stdio.dxf.mutations
extension dxf
start document

# P2-FG1: real one-line `OpText::print_op`/`parse_op` shape ALREADY emitted by
# `🏅️standards/🔖️r12/🪆️subsets/📰️header/🧬️schema/🧬️mutations/🦀️.rs`'s `print_dxf_mutation`/
# `parse_dxf_mutation` (`keyword arg=value ...`, space-separated, `=`-joined tokens — F6's own
# hand-rolled convention, reusing `🔺️diff`'s `pub(crate)` item-literal encoders `enc_header_var`/
# `enc_layer`/`enc_style`/`enc_linetype`/`enc_dxf_entity`/`enc_block`/`enc_dxf_snapshot` verbatim,
# per that file's own module doc comment). Every keyword/field-name literal below is copied from
# the real `format!(...)`/`match keyword` call sites, not invented.
#
# `hex` (bare `hex`, deliberately NOT a production in this file) is the framework's built-in `hex`
# MACRO (`📖️grammar/🦀️.rs`'s `default_macros`) — `Symbol::Ref` falls back
# production→macro, `Recognizer::match_macro_span` tries the largest token span first and
# backtracks correctly, which is why a hand-rolled `{INT|IDENT}*` production is wrong here (P2
# recipe pitfall #2: a hex run mixing digits/letters fragments across INT/IDENT tokens at the
# lexer level, and a naive Star has no backtracking to stop short of a following bareword literal
# — `hex` the macro handles both, validated end-to-end by this file's own `ops_grammar_conformance_law`).
#
# Every nested item literal below (`header-var`/`layer-item`/`style-item`/`linetype-item`/
# `dxf-entity`/`block-item`/`dxf-snapshot`) mirrors `🔺️diff/🦀️.rs`'s real `enc_*`/`dec_*`
# functions field-for-field, in the SAME positional order those functions emit — see that file's
# `#region 🔖️ItemCodecs`/`#region 🔖️EntityValueCodecs`. `dxf-entity`'s 8-arm tag-prefixed shape
# (`L`=Line,`C`=Circle,`A`=Arc,`W`=Polyline,`T`=Text,`S`=Solid,`I`=Insert,`O`=Other) is bounded, not
# self-recursive — a `block-item`'s own `entity-list` references `dxf-entity`, but no `DxfEntity`
# variant nests another `DxfEntity` (R12 has no entity-in-entity nesting), so this whole grammar
# has a fixed maximum depth, genuinely exercised end-to-end (incl. every entity kind, a POLYLINE
# with vertices, and a `SetSnapshot` payload nesting a block-with-entity) by
# `ops_grammar_conformance_law` against real `print_op` output for every `DxfMutation` variant.
document = set-snapshot | patch-snapshot | set-header-var | remove-header-var | insert-layer | remove-layer | set-layer | insert-style | remove-style | set-style | insert-linetype | remove-linetype | set-linetype | insert-entity | remove-entity | set-entity | insert-block | remove-block | set-block

patch-snapshot = "patch-snapshot" "patch" "=" hex
set-snapshot = "set-snapshot" "snapshot" "=" dxf-snapshot

set-header-var = "set-header-var" "name" "=" hex "header-var" "=" header-var
remove-header-var = "remove-header-var" "name" "=" hex

insert-layer = "insert-layer" "index" "=" INT "layer" "=" layer-item
remove-layer = "remove-layer" "name" "=" hex
set-layer = "set-layer" "name" "=" hex "layer" "=" layer-item

insert-style = "insert-style" "index" "=" INT "style" "=" style-item
remove-style = "remove-style" "name" "=" hex
set-style = "set-style" "name" "=" hex "style" "=" style-item

insert-linetype = "insert-linetype" "index" "=" INT "linetype" "=" linetype-item
remove-linetype = "remove-linetype" "name" "=" hex
set-linetype = "set-linetype" "name" "=" hex "linetype" "=" linetype-item

insert-entity = "insert-entity" "index" "=" INT "entity" "=" dxf-entity
remove-entity = "remove-entity" "index" "=" INT
set-entity = "set-entity" "index" "=" INT "entity" "=" dxf-entity

insert-block = "insert-block" "index" "=" INT "block" "=" block-item
remove-block = "remove-block" "index" "=" INT
set-block = "set-block" "index" "=" INT "block" "=" block-item

# `enc_f64` (`format!("{v}")`) prints a bare decimal, lexing as `INT` (no `.`) or `FLOAT` (has `.`).
num = INT | FLOAT
point3 = "[" num "," num "," num "]"
points4 = "[" point3 "," point3 "," point3 "," point3 "]"

# `enc_dxf_value`/`dec_dxf_value` — `S[hex]`/`I[digits]`/`D[float]`/`P[x,y,z]`.
dxf-value = "S" "[" hex "]" | "I" "[" INT "]" | "D" "[" num "]" | "P" "[" num "," num "," num "]"
group-code = "[" INT "," dxf-value "]"
group-codes = "[" "]" | "[" group-code {"," group-code}* "]"

vertex = "[" num "," num "," num "," num "," group-codes "]"
vertex-list = "[" "]" | "[" vertex {"," vertex}* "]"

# `enc_dxf_entity`/`dec_dxf_entity` — one tag letter per typed `DxfEntity` variant.
dxf-entity = "L" "[" point3 "," point3 "," hex "," group-codes "]" | "C" "[" point3 "," num "," hex "," group-codes "]" | "A" "[" point3 "," num "," num "," num "," hex "," group-codes "]" | "W" "[" vertex-list "," INT "," hex "," group-codes "]" | "T" "[" point3 "," num "," hex "," hex "," group-codes "]" | "S" "[" points4 "," hex "," group-codes "]" | "I" "[" hex "," point3 "," point3 "," num "," hex "," group-codes "]" | "O" "[" hex "," group-codes "]"
entity-list = "[" "]" | "[" dxf-entity {"," dxf-entity}* "]"

header-var = "[" hex "," INT "," dxf-value "," group-codes "]"
header-var-list = "[" "]" | "[" header-var {"," header-var}* "]"

layer-item = "[" hex "," INT "," hex "," INT "," group-codes "]"
layer-list = "[" "]" | "[" layer-item {"," layer-item}* "]"

style-item = "[" hex "," INT "," hex "," group-codes "]"
style-list = "[" "]" | "[" style-item {"," style-item}* "]"

linetype-item = "[" hex "," INT "," hex "," group-codes "]"
linetype-list = "[" "]" | "[" linetype-item {"," linetype-item}* "]"

block-item = "[" hex "," point3 "," entity-list "," group-codes "]"
block-list = "[" "]" | "[" block-item {"," block-item}* "]"

dxf-tag = "[" INT "," hex "]"
dxf-tag-list = "[" "]" | "[" dxf-tag {"," dxf-tag}* "]"
other-table = "[" hex "," dxf-tag-list "]"
other-table-list = "[" "]" | "[" other-table {"," other-table}* "]"

dxf-tables = "[" layer-list "," style-list "," linetype-list "]"

# `enc_dxf_snapshot`/`dec_dxf_snapshot` — the WHOLE snapshot, only reachable via `SetSnapshot`'s
# own payload (`🔺️diff/🦀️.rs`'s own doc comment on `enc_dxf_snapshot` names this exactly).
dxf-snapshot = "[" hex "," header-var-list "," dxf-tables "," other-table-list "," block-list "," entity-list "]"
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧬️schema/🧬️mutations/📝️text/📖️.grammar.semio

Original SHA-256: `47bb3d1b6f798a8a99930180da8741e60d743b8a19869a06dad983a1e4172b9f`. Current SHA-256: `c0a0b5181632a6d3936c44c3d3da782f1afe8650a887cb06d66ee80c3cf31905`.

Original:

```text
dialect grammar
grammar stdio.obj.mutations
start op
comment none

op = set-snapshot | insert-vertex | remove-vertex | set-vertex | insert-texcoord | remove-texcoord | set-texcoord | insert-normal | remove-normal | set-normal | insert-face | remove-face | set-face | set-group | remove-group | set-object | remove-object | set-mtllib | set-usemtl | set-smoothing-groups | set-unknown-statements
set-snapshot = "set-snapshot" "snapshot" "{" snapshot "}"
insert-vertex = "insert-vertex" "index" "=" INT "vertex" vertex
remove-vertex = "remove-vertex" "index" "=" INT
set-vertex = "set-vertex" "index" "=" INT "vertex" vertex
insert-texcoord = "insert-texcoord" "index" "=" INT "texcoord" texcoord
remove-texcoord = "remove-texcoord" "index" "=" INT
set-texcoord = "set-texcoord" "index" "=" INT "texcoord" texcoord
insert-normal = "insert-normal" "index" "=" INT "normal" normal
remove-normal = "remove-normal" "index" "=" INT
set-normal = "set-normal" "index" "=" INT "normal" normal
insert-face = "insert-face" "index" "=" INT "face" face
remove-face = "remove-face" "index" "=" INT
set-face = "set-face" "index" "=" INT "face" face
set-group = "set-group" "name" "=" text "faces" "=" "[" INT* "]"
remove-group = "remove-group" "name" "=" text
set-object = "set-object" "name" "=" text "faces" "=" "[" INT* "]"
remove-object = "remove-object" "name" "=" text
set-mtllib = "set-mtllib" {"mtllib" "=" text}?
set-usemtl = "set-usemtl" "usemtl" "=" "[" material* "]"
set-smoothing-groups = "set-smoothing-groups" "smoothing-groups" "=" "[" smoothing* "]"
set-unknown-statements = "set-unknown-statements" "unknown-statements" "=" "[" unknown* "]"
snapshot = "schema" "=" text {"mtllib" "=" text}? "vertices" "=" "[" vertex* "]" "texcoords" "=" "[" texcoord* "]" "normals" "=" "[" normal* "]" "faces" "=" "[" face* "]" "groups" "=" "[" membership* "]" "objects" "=" "[" membership* "]" "usemtl" "=" "[" material* "]" "smoothing-groups" "=" "[" smoothing* "]" "unknown-statements" "=" "[" unknown* "]"
vertex = "{" "x" "=" number "y" "=" number "z" "=" number {"w" "=" number}? "}"
texcoord = "{" "u" "=" number "v" "=" number {"w" "=" number}? "}"
normal = "{" "x" "=" number "y" "=" number "z" "=" number "}"
face = "{" "vertices" "=" "[" corner* "]" "}"
corner = "{" "vertex" "=" INT {"texcoord" "=" INT}? {"normal" "=" INT}? "}"
membership = "{" "name" "=" text "faces" "=" "[" INT* "]" "}"
material = "{" "face-index-from" "=" INT "material" "=" text "}"
smoothing = "{" "face-index-from" "=" INT {"group" "=" INT}? "}"
unknown = "{" "line-index" "=" INT "raw" "=" text "}"
number = FLOAT | INT | IDENT
text = IDENT | TEXT
```

Current:

```text
dialect grammar
grammar stdio.obj.mutations
start op
comment none

op = set-snapshot | patch-snapshot | insert-vertex | remove-vertex | set-vertex | insert-texcoord | remove-texcoord | set-texcoord | insert-normal | remove-normal | set-normal | insert-face | remove-face | set-face | set-group | remove-group | set-object | remove-object | set-mtllib | set-usemtl | set-smoothing-groups | set-unknown-statements
patch-snapshot = "patch-snapshot" "patch" "=" hex
set-snapshot = "set-snapshot" "snapshot" "{" snapshot "}"
insert-vertex = "insert-vertex" "index" "=" INT "vertex" vertex
remove-vertex = "remove-vertex" "index" "=" INT
set-vertex = "set-vertex" "index" "=" INT "vertex" vertex
insert-texcoord = "insert-texcoord" "index" "=" INT "texcoord" texcoord
remove-texcoord = "remove-texcoord" "index" "=" INT
set-texcoord = "set-texcoord" "index" "=" INT "texcoord" texcoord
insert-normal = "insert-normal" "index" "=" INT "normal" normal
remove-normal = "remove-normal" "index" "=" INT
set-normal = "set-normal" "index" "=" INT "normal" normal
insert-face = "insert-face" "index" "=" INT "face" face
remove-face = "remove-face" "index" "=" INT
set-face = "set-face" "index" "=" INT "face" face
set-group = "set-group" "name" "=" text "faces" "=" "[" INT* "]"
remove-group = "remove-group" "name" "=" text
set-object = "set-object" "name" "=" text "faces" "=" "[" INT* "]"
remove-object = "remove-object" "name" "=" text
set-mtllib = "set-mtllib" {"mtllib" "=" text}?
set-usemtl = "set-usemtl" "usemtl" "=" "[" material* "]"
set-smoothing-groups = "set-smoothing-groups" "smoothing-groups" "=" "[" smoothing* "]"
set-unknown-statements = "set-unknown-statements" "unknown-statements" "=" "[" unknown* "]"
snapshot = "schema" "=" text {"mtllib" "=" text}? "vertices" "=" "[" vertex* "]" "texcoords" "=" "[" texcoord* "]" "normals" "=" "[" normal* "]" "faces" "=" "[" face* "]" "groups" "=" "[" membership* "]" "objects" "=" "[" membership* "]" "usemtl" "=" "[" material* "]" "smoothing-groups" "=" "[" smoothing* "]" "unknown-statements" "=" "[" unknown* "]"
vertex = "{" "x" "=" number "y" "=" number "z" "=" number {"w" "=" number}? "}"
texcoord = "{" "u" "=" number "v" "=" number {"w" "=" number}? "}"
normal = "{" "x" "=" number "y" "=" number "z" "=" number "}"
face = "{" "vertices" "=" "[" corner* "]" "}"
corner = "{" "vertex" "=" INT {"texcoord" "=" INT}? {"normal" "=" INT}? "}"
membership = "{" "name" "=" text "faces" "=" "[" INT* "]" "}"
material = "{" "face-index-from" "=" INT "material" "=" text "}"
smoothing = "{" "face-index-from" "=" INT {"group" "=" INT}? "}"
unknown = "{" "line-index" "=" INT "raw" "=" text "}"
number = FLOAT | INT | IDENT
text = IDENT | TEXT
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/📖️.grammar.semio

Original SHA-256: `7a05f8310cbd7b86bae7337794577dc8cf22718108baedc2461dfc4bf03c24ca`. Current SHA-256: `a2ff9928e9b68eed7a67e1e9e8c102d6b8fd43b0b3d79fe4cf1363fb91fc4d6c`.

Original:

```text
dialect grammar
grammar stdio.ply.mutations
start op

# 🎫️26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION FG3: real one-line
# `print_op`/`parse_op` form this artifact's hand-rolled `OpText` (../🦀️.rs
# `print_ply_mutation`/`parse_ply_mutation`) emits — kebab-case keyword + space-separated
# kebab-case `arg=value` tokens, one match arm per `PlyMutation` variant (no `DslVariants`
# scaffolding — nothing here derives it, `#[derive(dsl::DslOps)]` is confirmed rejected, see the
# module doc comment's real `cargo check` citation). This REPLACES the pre-Phase-2 placeholder
# this file used to contain, which described a `serde_json::to_string`-shaped JSON object line —
# stale since F6 already replaced the real `OpText`/`OpBinary` impls with this hand-rolled
# grammar; the placeholder never matched what `print_op` actually emits.
#
# Reuses the sibling `../../🔺️diff/📝️text/📖️.grammar.semio`'s own `ply-value`/
# `ply-property`/`element-value`/`row-value`/`format-tag` shapes verbatim (this artifact's own
# hand-rolled Rust codec reuses `PlyDiff`'s grammar primitives the identical way — see
# `../🦀️.rs`'s own module doc comment) — RESTATED here rather than referenced, per the
# recipe's own "cross-artifact/cross-facet `use` doesn't resolve at walk time yet" rule (§2.1;
# same honest-restatement precedent zip's OPC family and svg's own xml-derived grammar establish).
comment none

op = set-snapshot-op | set-format-op | insert-comment-op | remove-comment-op | add-element-op | remove-element-op | insert-row-op | remove-row-op | set-row-property-op

set-snapshot-op = "set-snapshot" "snapshot" "=" snapshot-value
set-format-op = "set-format" "format" "=" format-tag
insert-comment-op = "insert-comment" "index" "=" INT "comment" "=" hex
remove-comment-op = "remove-comment" "index" "=" INT
add-element-op = "add-element" "index" "=" INT "element" "=" element-value
remove-element-op = "remove-element" "name" "=" hex
insert-row-op = "insert-row" "element-name" "=" hex "index" "=" INT "row" "=" row-value
remove-row-op = "remove-row" "element-name" "=" hex "index" "=" INT
set-row-property-op = "set-row-property" "element-name" "=" hex "row-index" "=" INT "property-name" "=" hex "value" "=" ply-value

format-tag = "a" | "l" | "b"

# `PlySnapshot` — positional `[schema,format,[comments],[elements]]` tuple — `enc_snapshot`/
# `dec_snapshot` (../🦀️.rs).
snapshot-value = "[" hex "," format-tag "," "[" comment-list? "]" "," "[" element-list? "]" "]"
comment-list = hex {"," hex}*
element-list = element-value {"," element-value}*

# `PlyElement` — positional `[name,count,[properties],[rows]]` tuple.
element-value = "[" hex "," INT "," "[" property-list? "]" "," "[" row-list? "]" "]"
property-list = ply-property {"," ply-property}*
ply-property = "S" "[" hex "," scalar-kind "]" | "L" "[" hex "," scalar-kind "," scalar-kind "]"
scalar-kind = "c" | "C" | "s" | "w" | "i" | "u" | "f" | "d"

row-list = row-value {"," row-value}*
row-value = "[" value-list? "]"
value-list = ply-value {"," ply-value}*

number = INT | FLOAT
ply-value = "c" "[" INT "]" | "C" "[" INT "]" | "s" "[" INT "]" | "w" "[" INT "]" | "i" "[" INT "]" | "u" "[" INT "]" | "f" "[" number "]" | "d" "[" number "]" | "L" "[" value-list? "]"

# `hex` (bare `hex`, deliberately NOT defined as a production) is the framework's built-in `hex`
# MACRO — see the sibling diff grammar's own doc comment, and `📖️grammar-recipe.md` §1.3(f)/§3
# pitfall #2, for why a hand-rolled `{INT|IDENT}*` production is wrong here.
```

Current:

```text
dialect grammar
grammar stdio.ply.mutations
start op

# 🎫️26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION FG3: real one-line
# `print_op`/`parse_op` form this artifact's hand-rolled `OpText` (../🦀️.rs
# `print_ply_mutation`/`parse_ply_mutation`) emits — kebab-case keyword + space-separated
# kebab-case `arg=value` tokens, one match arm per `PlyMutation` variant (no `DslVariants`
# scaffolding — nothing here derives it, `#[derive(dsl::DslOps)]` is confirmed rejected, see the
# module doc comment's real `cargo check` citation). This REPLACES the pre-Phase-2 placeholder
# this file used to contain, which described a `serde_json::to_string`-shaped JSON object line —
# stale since F6 already replaced the real `OpText`/`OpBinary` impls with this hand-rolled
# grammar; the placeholder never matched what `print_op` actually emits.
#
# Reuses the sibling `../../🔺️diff/📝️text/📖️.grammar.semio`'s own `ply-value`/
# `ply-property`/`element-value`/`row-value`/`format-tag` shapes verbatim (this artifact's own
# hand-rolled Rust codec reuses `PlyDiff`'s grammar primitives the identical way — see
# `../🦀️.rs`'s own module doc comment) — RESTATED here rather than referenced, per the
# recipe's own "cross-artifact/cross-facet `use` doesn't resolve at walk time yet" rule (§2.1;
# same honest-restatement precedent zip's OPC family and svg's own xml-derived grammar establish).
comment none

op = set-snapshot-op | patch-snapshot-op | set-format-op | insert-comment-op | remove-comment-op | add-element-op | remove-element-op | insert-row-op | remove-row-op | set-row-property-op

patch-snapshot-op = "patch-snapshot" "patch" "=" hex
set-snapshot-op = "set-snapshot" "snapshot" "=" snapshot-value
set-format-op = "set-format" "format" "=" format-tag
insert-comment-op = "insert-comment" "index" "=" INT "comment" "=" hex
remove-comment-op = "remove-comment" "index" "=" INT
add-element-op = "add-element" "index" "=" INT "element" "=" element-value
remove-element-op = "remove-element" "name" "=" hex
insert-row-op = "insert-row" "element-name" "=" hex "index" "=" INT "row" "=" row-value
remove-row-op = "remove-row" "element-name" "=" hex "index" "=" INT
set-row-property-op = "set-row-property" "element-name" "=" hex "row-index" "=" INT "property-name" "=" hex "value" "=" ply-value

format-tag = "a" | "l" | "b"

# `PlySnapshot` — positional `[schema,format,[comments],[elements]]` tuple — `enc_snapshot`/
# `dec_snapshot` (../🦀️.rs).
snapshot-value = "[" hex "," format-tag "," "[" comment-list? "]" "," "[" element-list? "]" "]"
comment-list = hex {"," hex}*
element-list = element-value {"," element-value}*

# `PlyElement` — positional `[name,count,[properties],[rows]]` tuple.
element-value = "[" hex "," INT "," "[" property-list? "]" "," "[" row-list? "]" "]"
property-list = ply-property {"," ply-property}*
ply-property = "S" "[" hex "," scalar-kind "]" | "L" "[" hex "," scalar-kind "," scalar-kind "]"
scalar-kind = "c" | "C" | "s" | "w" | "i" | "u" | "f" | "d"

row-list = row-value {"," row-value}*
row-value = "[" value-list? "]"
value-list = ply-value {"," ply-value}*

number = INT | FLOAT
ply-value = "c" "[" INT "]" | "C" "[" INT "]" | "s" "[" INT "]" | "w" "[" INT "]" | "i" "[" INT "]" | "u" "[" INT "]" | "f" "[" number "]" | "d" "[" number "]" | "L" "[" value-list? "]"

# `hex` (bare `hex`, deliberately NOT defined as a production) is the framework's built-in `hex`
# MACRO — see the sibling diff grammar's own doc comment, and `📖️grammar-recipe.md` §1.3(f)/§3
# pitfall #2, for why a hand-rolled `{INT|IDENT}*` production is wrong here.
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🌊️flow/🧬️schema/🧬️mutations/📝️text/📖️.grammar.semio

Original SHA-256: `77874af578a0248caf36f98f72baab4f989053ed41d9114217f52c1e57ab265a`. Current SHA-256: `99e3efb3dcdca99fb76b3ac6a1e5d7da14760d5fd5b6a8009fcbf53c75b2c4e0`.

Original:

```text
dialect grammar
grammar semio.flow.mutations
extension semio
start op

# P2 pilot (flow): real one-line `OpText::print_op`/`parse_op` shape ALREADY emitted by
# `🧬️mutations/🦀️.rs` (`print_flow_mutation`/`parse_flow_mutation`) —
# "keyword arg=value ..." (space-separated; space is lexer trivia at this dialect level). Every
# keyword/field-name token below is copied verbatim from the real `format!(...)` call sites in that
# file's `print_flow_mutation`, not invented.
op = set-snapshot | insert-node | remove-node | set-node-kind | set-node-label | set-node-position | set-node-param | remove-node-param | insert-edge | remove-edge | set-edge-endpoints | set-edge-kind | drag-nodes

set-snapshot = "set-snapshot" "snapshot" "=" snapshot-lit
insert-node = "insert-node" "node" "=" node
remove-node = "remove-node" "id" "=" hex
set-node-kind = "set-node-kind" "id" "=" hex "kind" "=" hex
set-node-label = "set-node-label" "id" "=" hex "label" "=" hex
set-node-position = "set-node-position" "id" "=" hex "position" "=" point2
set-node-param = "set-node-param" "id" "=" hex "key" "=" hex "value" "=" hex
remove-node-param = "remove-node-param" "id" "=" hex "key" "=" hex
insert-edge = "insert-edge" "edge" "=" edge
remove-edge = "remove-edge" "id" "=" hex
set-edge-endpoints = "set-edge-endpoints" "id" "=" hex "from" "=" port "to" "=" port
set-edge-kind = "set-edge-kind" "id" "=" hex "kind" "=" hex
drag-nodes = "drag-nodes" "targets" "=" "[" hex-list? "]" "dx" "=" number "dy" "=" number
hex-list = hex {"," hex}*

# `enc_semio_flow_snapshot`/`dec_semio_flow_snapshot` — `[hex(schema),[node,...],[edge,...]]`.
snapshot-lit = "[" hex "," "[" node-list? "]" "," "[" edge-list? "]" "]"
node-list = node {"," node}*
edge-list = edge {"," edge}*

# `enc_node`/`enc_edge`/`enc_param`/`enc_point2`/`enc_port_ref` — the same full-VALUE codecs
# `📸️snapshot/🦀️.rs`'s own text DSL grammar declares (restated here so this leaf grammar
# is self-contained, matching the repo's existing per-facet convention).
node = "[" hex "," hex "," hex "," "[" param-list? "]" "," point2 "]"
param-list = param {"," param}*
param = "[" hex "," hex "]"
point2 = "[" number "," number "]"
edge = "[" hex "," port "," port "," hex "]"
port = "[" hex "," hex "]"

number = INT | FLOAT

# `hex` (bare `hex`, deliberately NOT defined as a production here) is the framework's built-in
# `hex` MACRO — a run of lowercase hex digits (`enc_str`/`hex_encode`'s own alphabet), incl. the
# empty string. `Symbol::Ref` already falls back from "no production of this name" to "macro of
# this name", so referencing `hex` with no matching production routes here automatically. Using a
# hand-rolled `{INT|IDENT}*` production instead would be wrong: it never backtracks and would
# silently swallow the next literal keyword token (e.g. `id=<hex> kind=<hex>` — the literal `kind`
# tokenizes as a plain `IDENT`, exactly like a stray hex letter run would).
```

Current:

```text
dialect grammar
grammar semio.flow.mutations
extension semio
start op

# P2 pilot (flow): real one-line `OpText::print_op`/`parse_op` shape ALREADY emitted by
# `🧬️mutations/🦀️.rs` (`print_flow_mutation`/`parse_flow_mutation`) —
# "keyword arg=value ..." (space-separated; space is lexer trivia at this dialect level). Every
# keyword/field-name token below is copied verbatim from the real `format!(...)` call sites in that
# file's `print_flow_mutation`, not invented.
op = set-snapshot | patch-snapshot | insert-node | remove-node | set-node-kind | set-node-label | set-node-position | set-node-param | remove-node-param | insert-edge | remove-edge | set-edge-endpoints | set-edge-kind | drag-nodes

patch-snapshot = "patch-snapshot" "patch" "=" hex
set-snapshot = "set-snapshot" "snapshot" "=" snapshot-lit
insert-node = "insert-node" "node" "=" node
remove-node = "remove-node" "id" "=" hex
set-node-kind = "set-node-kind" "id" "=" hex "kind" "=" hex
set-node-label = "set-node-label" "id" "=" hex "label" "=" hex
set-node-position = "set-node-position" "id" "=" hex "position" "=" point2
set-node-param = "set-node-param" "id" "=" hex "key" "=" hex "value" "=" hex
remove-node-param = "remove-node-param" "id" "=" hex "key" "=" hex
insert-edge = "insert-edge" "edge" "=" edge
remove-edge = "remove-edge" "id" "=" hex
set-edge-endpoints = "set-edge-endpoints" "id" "=" hex "from" "=" port "to" "=" port
set-edge-kind = "set-edge-kind" "id" "=" hex "kind" "=" hex
drag-nodes = "drag-nodes" "targets" "=" "[" hex-list? "]" "dx" "=" number "dy" "=" number
hex-list = hex {"," hex}*

# `enc_semio_flow_snapshot`/`dec_semio_flow_snapshot` — `[hex(schema),[node,...],[edge,...]]`.
snapshot-lit = "[" hex "," "[" node-list? "]" "," "[" edge-list? "]" "]"
node-list = node {"," node}*
edge-list = edge {"," edge}*

# `enc_node`/`enc_edge`/`enc_param`/`enc_point2`/`enc_port_ref` — the same full-VALUE codecs
# `📸️snapshot/🦀️.rs`'s own text DSL grammar declares (restated here so this leaf grammar
# is self-contained, matching the repo's existing per-facet convention).
node = "[" hex "," hex "," hex "," "[" param-list? "]" "," point2 "]"
param-list = param {"," param}*
param = "[" hex "," hex "]"
point2 = "[" number "," number "]"
edge = "[" hex "," port "," port "," hex "]"
port = "[" hex "," hex "]"

number = INT | FLOAT

# `hex` (bare `hex`, deliberately NOT defined as a production here) is the framework's built-in
# `hex` MACRO — a run of lowercase hex digits (`enc_str`/`hex_encode`'s own alphabet), incl. the
# empty string. `Symbol::Ref` already falls back from "no production of this name" to "macro of
# this name", so referencing `hex` with no matching production routes here automatically. Using a
# hand-rolled `{INT|IDENT}*` production instead would be wrong: it never backtracks and would
# silently swallow the next literal keyword token (e.g. `id=<hex> kind=<hex>` — the literal `kind`
# tokenizes as a plain `IDENT`, exactly like a stray hex letter run would).
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎬️video/🧬️schema/🧬️mutations/📝️text/📖️.grammar.semio

Original SHA-256: `e7b22c802b28b9539c6aa2b11a2af95e9988119f89ab0c6bd315bf27b4e5bc33`. Current SHA-256: `1b69ddb9dcef76f871d30f155f2b0dc5c6dc532c5cc92ed1703590c4d56c3b61`.

Original:

```text
dialect grammar
grammar stdio.semio.video.mutations
extension semio
start op

# Video wave: real one-line `OpText::print_op`/`parse_op` shape ALREADY emitted by
# `🧬️mutations/🦀️.rs` (`print_semio_video_mutation`/`parse_semio_video_mutation`) —
# "keyword arg=value ..." (space-separated; space is lexer trivia at this dialect level). Every
# keyword/field-name token below is copied verbatim from the real `format!(...)` call sites in that
# file's `print_semio_video_mutation`, not invented.
op = set-snapshot | insert-stream | remove-stream | set-stream-meta | insert-sample | remove-sample | set-sample-data | set-sample-flags

set-snapshot = "set-snapshot" "snapshot" "=" snapshot-lit
insert-stream = "insert-stream" "index" "=" INT "stream" "=" stream
remove-stream = "remove-stream" "index" "=" INT
set-stream-meta = "set-stream-meta" "index" "=" INT "kind" "=" kind "codec" "=" hex "width" "=" INT "height" "=" INT "rate" "=" rational
insert-sample = "insert-sample" "stream-index" "=" INT "index" "=" INT "sample" "=" sample
remove-sample = "remove-sample" "stream-index" "=" INT "index" "=" INT
set-sample-data = "set-sample-data" "stream-index" "=" INT "index" "=" INT "data" "=" hex
set-sample-flags = "set-sample-flags" "stream-index" "=" INT "index" "=" INT "pts" "=" INT "key" "=" bool

# `enc_semio_video_snapshot`/`dec_semio_video_snapshot` — `[hex(schema),[stream,...]]`.
snapshot-lit = "[" hex "," "[" stream-list? "]" "]"
stream-list = stream {"," stream}*

# `enc_stream`/`enc_sample`/`enc_rational` — the same full-VALUE codecs
# `📸️snapshot/📝️text/📖️.grammar.semio` declares (restated here so this leaf grammar is
# self-contained, matching the repo's existing per-facet convention).
stream = "[" kind "," hex "," INT "," INT "," rational "," "[" sample-list? "]" "]"
sample-list = sample {"," sample}*
sample = "[" INT "," bool "," hex "]"
rational = "[" INT "," INT "]"
kind = "V" | "A" | "S"
bool = "0" | "1"

# `hex` (bare `hex`, deliberately NOT defined as a production here) is the framework's built-in
# `hex` MACRO — a run of lowercase hex digits (`enc_str`/`hex_encode`'s own alphabet), incl. the
# empty string. `Symbol::Ref` already falls back from "no production of this name" to "macro of
# this name", so referencing `hex` with no matching production routes here automatically. Using a
# hand-rolled `{INT|IDENT}*` production instead would be wrong: it never backtracks and would
# silently swallow the next literal keyword token (e.g. `index=<hex> stream=<stream>` — a stray hex
# letter run would desync the parse exactly like the recipe's own documented pitfall #2).
```

Current:

```text
dialect grammar
grammar stdio.semio.video.mutations
extension semio
start op

# Video wave: real one-line `OpText::print_op`/`parse_op` shape ALREADY emitted by
# `🧬️mutations/🦀️.rs` (`print_semio_video_mutation`/`parse_semio_video_mutation`) —
# "keyword arg=value ..." (space-separated; space is lexer trivia at this dialect level). Every
# keyword/field-name token below is copied verbatim from the real `format!(...)` call sites in that
# file's `print_semio_video_mutation`, not invented.
op = set-snapshot | patch-snapshot | insert-stream | remove-stream | set-stream-meta | insert-sample | remove-sample | set-sample-data | set-sample-flags

patch-snapshot = "patch-snapshot" "patch" "=" hex
set-snapshot = "set-snapshot" "snapshot" "=" snapshot-lit
insert-stream = "insert-stream" "index" "=" INT "stream" "=" stream
remove-stream = "remove-stream" "index" "=" INT
set-stream-meta = "set-stream-meta" "index" "=" INT "kind" "=" kind "codec" "=" hex "width" "=" INT "height" "=" INT "rate" "=" rational
insert-sample = "insert-sample" "stream-index" "=" INT "index" "=" INT "sample" "=" sample
remove-sample = "remove-sample" "stream-index" "=" INT "index" "=" INT
set-sample-data = "set-sample-data" "stream-index" "=" INT "index" "=" INT "data" "=" hex
set-sample-flags = "set-sample-flags" "stream-index" "=" INT "index" "=" INT "pts" "=" INT "key" "=" bool

# `enc_semio_video_snapshot`/`dec_semio_video_snapshot` — `[hex(schema),[stream,...]]`.
snapshot-lit = "[" hex "," "[" stream-list? "]" "]"
stream-list = stream {"," stream}*

# `enc_stream`/`enc_sample`/`enc_rational` — the same full-VALUE codecs
# `📸️snapshot/📝️text/📖️.grammar.semio` declares (restated here so this leaf grammar is
# self-contained, matching the repo's existing per-facet convention).
stream = "[" kind "," hex "," INT "," INT "," rational "," "[" sample-list? "]" "]"
sample-list = sample {"," sample}*
sample = "[" INT "," bool "," hex "]"
rational = "[" INT "," INT "]"
kind = "V" | "A" | "S"
bool = "0" | "1"

# `hex` (bare `hex`, deliberately NOT defined as a production here) is the framework's built-in
# `hex` MACRO — a run of lowercase hex digits (`enc_str`/`hex_encode`'s own alphabet), incl. the
# empty string. `Symbol::Ref` already falls back from "no production of this name" to "macro of
# this name", so referencing `hex` with no matching production routes here automatically. Using a
# hand-rolled `{INT|IDENT}*` production instead would be wrong: it never backtracks and would
# silently swallow the next literal keyword token (e.g. `index=<hex> stream=<stream>` — a stray hex
# letter run would desync the parse exactly like the recipe's own documented pitfall #2).
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🏛️model/🧬️schema/🧬️mutations/📝️text/📖️.grammar.semio

Original SHA-256: `6872b726a133f1c8cf4f5ecfd9abe5257d8b28acf29f21d447516e15d39316e8`. Current SHA-256: `6f571c9c087027b8c11a3be4549e4932045ef89fba56cccdb50cfa750975c507`.

Original:

```text
dialect grammar
grammar semio.model.mutations
extension semio
start op

# P2 pilot (model): real one-line `OpText::print_op`/`parse_op` shape ALREADY emitted by
# `🧬️mutations/🦀️.rs` (`print_semio_model_mutation`/`parse_semio_model_mutation`) —
# "keyword arg=value ..." (space-separated; space is lexer trivia at this dialect level). Every
# keyword/field-name token below is copied verbatim from the real `format!(...)` call sites in that
# file's `print_semio_model_mutation`, not invented.
op = set-snapshot | insert-spatial-node | remove-spatial-node | set-spatial-node | insert-element | remove-element | set-element | insert-relation | remove-relation | set-relation

set-snapshot = "set-snapshot" "snapshot" "=" snapshot-lit
insert-spatial-node = "insert-spatial-node" "node" "=" spatial-node
remove-spatial-node = "remove-spatial-node" "id" "=" hex
set-spatial-node = "set-spatial-node" "id" "=" hex "kind" "=" option-spatial-kind "name" "=" option-hex "parent_id" "=" option-option-hex "placement" "=" option-transform
insert-element = "insert-element" "element" "=" element
remove-element = "remove-element" "id" "=" hex
set-element = "set-element" "id" "=" hex "class" "=" option-element-class "placement" "=" option-transform "geometry" "=" option-geometry-ref "spatial_id" "=" option-option-hex "psets" "=" option-psets
insert-relation = "insert-relation" "relation" "=" relation
remove-relation = "remove-relation" "id" "=" hex
set-relation = "set-relation" "id" "=" hex "kind" "=" option-relation-kind "from" "=" option-hex "to" "=" option-hex

# `enc_semio_model_snapshot`/`dec_semio_model_snapshot` — `[hex(schema),[spatial-node,...],[element,...],[relation,...]]`.
snapshot-lit = "[" hex "," "[" spatial-list? "]" "," "[" element-list? "]" "," "[" relation-list? "]" "]"
spatial-list = spatial-node {"," spatial-node}*
element-list = element {"," element}*
relation-list = relation {"," relation}*

# `enc_spatial_node`/`enc_element`/`enc_relation` — the same full-VALUE codecs
# `📸️snapshot/📝️text/📖️.grammar.semio` declares (restated here so this leaf grammar is
# self-contained, matching the repo's existing per-facet convention).
spatial-node = "[" hex "," spatial-kind "," hex "," option-hex "," transform "]"
spatial-kind = "S" | "B" | "T" | "P"
element = "[" hex "," element-class "," transform "," geometry-ref "," option-hex "," "[" pset-list? "]" "]"
element-class = "WA" | "SL" | "CO" | "BE" | "DO" | "WI" | "RO" | "ST" | "FU" | "OT" "[" hex "]"
geometry-ref = "N" | "B" "[" hex "]" | "M" "[" hex "]"
pset-list = property-set {"," property-set}*
property-set = "[" hex "," "[" property-list? "]" "]"
property-list = property {"," property}*
property = "[" hex "," pset-value "]"
pset-value = "T" "[" hex "]" | "N" "[" number "]" | "B" "[" bit "]"
bit = "0" | "1"
relation = "[" hex "," relation-kind "," hex "," hex "]"
relation-kind = "AG" | "CI" | "CN" | "FV" | "VE" | "OT" "[" hex "]"
transform = "[" point3 "," quat "," point3 "]"
point3 = "[" number "," number "," number "]"
quat = "[" number "," number "," number "," number "]"

# Tri-state `Option<T>`/doubly-tri-state `Option<Option<T>>` tags (recipe §1.4's "tri-state"
# pattern, verbatim shape) — one per `Set*` variant's optional field.
option-hex = "[" "0" "]" | "[" "1" "," hex "]"
option-option-hex = "[" "0" "]" | "[" "1" "," option-hex "]"
option-spatial-kind = "[" "0" "]" | "[" "1" "," spatial-kind "]"
option-transform = "[" "0" "]" | "[" "1" "," transform "]"
option-element-class = "[" "0" "]" | "[" "1" "," element-class "]"
option-geometry-ref = "[" "0" "]" | "[" "1" "," geometry-ref "]"
option-psets = "[" "0" "]" | "[" "1" "," "[" pset-list? "]" "]"
option-relation-kind = "[" "0" "]" | "[" "1" "," relation-kind "]"

number = INT | FLOAT | IDENT | "-" "inf"

# `hex` (bare `hex`, deliberately NOT defined as a production here) is the framework's built-in
# `hex` MACRO — a run of lowercase hex digits (`enc_str`/`hex_encode`'s own alphabet), incl. the
# empty string. `Symbol::Ref` already falls back from "no production of this name" to "macro of
# this name", so referencing `hex` with no matching production routes here automatically. Using a
# hand-rolled `{INT|IDENT}*` production instead would be wrong: it never backtracks and would
# silently swallow the next literal keyword token (e.g. `id=<hex> kind=<hex>` — the literal `kind`
# tokenizes as a plain `IDENT`, exactly like a stray hex letter run would).
```

Current:

```text
dialect grammar
grammar semio.model.mutations
extension semio
start op

# P2 pilot (model): real one-line `OpText::print_op`/`parse_op` shape ALREADY emitted by
# `🧬️mutations/🦀️.rs` (`print_semio_model_mutation`/`parse_semio_model_mutation`) —
# "keyword arg=value ..." (space-separated; space is lexer trivia at this dialect level). Every
# keyword/field-name token below is copied verbatim from the real `format!(...)` call sites in that
# file's `print_semio_model_mutation`, not invented.
op = set-snapshot | patch-snapshot | insert-spatial-node | remove-spatial-node | set-spatial-node | insert-element | remove-element | set-element | insert-relation | remove-relation | set-relation

patch-snapshot = "patch-snapshot" "patch" "=" hex
set-snapshot = "set-snapshot" "snapshot" "=" snapshot-lit
insert-spatial-node = "insert-spatial-node" "node" "=" spatial-node
remove-spatial-node = "remove-spatial-node" "id" "=" hex
set-spatial-node = "set-spatial-node" "id" "=" hex "kind" "=" option-spatial-kind "name" "=" option-hex "parent_id" "=" option-option-hex "placement" "=" option-transform
insert-element = "insert-element" "element" "=" element
remove-element = "remove-element" "id" "=" hex
set-element = "set-element" "id" "=" hex "class" "=" option-element-class "placement" "=" option-transform "geometry" "=" option-geometry-ref "spatial_id" "=" option-option-hex "psets" "=" option-psets
insert-relation = "insert-relation" "relation" "=" relation
remove-relation = "remove-relation" "id" "=" hex
set-relation = "set-relation" "id" "=" hex "kind" "=" option-relation-kind "from" "=" option-hex "to" "=" option-hex

# `enc_semio_model_snapshot`/`dec_semio_model_snapshot` — `[hex(schema),[spatial-node,...],[element,...],[relation,...]]`.
snapshot-lit = "[" hex "," "[" spatial-list? "]" "," "[" element-list? "]" "," "[" relation-list? "]" "]"
spatial-list = spatial-node {"," spatial-node}*
element-list = element {"," element}*
relation-list = relation {"," relation}*

# `enc_spatial_node`/`enc_element`/`enc_relation` — the same full-VALUE codecs
# `📸️snapshot/📝️text/📖️.grammar.semio` declares (restated here so this leaf grammar is
# self-contained, matching the repo's existing per-facet convention).
spatial-node = "[" hex "," spatial-kind "," hex "," option-hex "," transform "]"
spatial-kind = "S" | "B" | "T" | "P"
element = "[" hex "," element-class "," transform "," geometry-ref "," option-hex "," "[" pset-list? "]" "]"
element-class = "WA" | "SL" | "CO" | "BE" | "DO" | "WI" | "RO" | "ST" | "FU" | "OT" "[" hex "]"
geometry-ref = "N" | "B" "[" hex "]" | "M" "[" hex "]"
pset-list = property-set {"," property-set}*
property-set = "[" hex "," "[" property-list? "]" "]"
property-list = property {"," property}*
property = "[" hex "," pset-value "]"
pset-value = "T" "[" hex "]" | "N" "[" number "]" | "B" "[" bit "]"
bit = "0" | "1"
relation = "[" hex "," relation-kind "," hex "," hex "]"
relation-kind = "AG" | "CI" | "CN" | "FV" | "VE" | "OT" "[" hex "]"
transform = "[" point3 "," quat "," point3 "]"
point3 = "[" number "," number "," number "]"
quat = "[" number "," number "," number "," number "]"

# Tri-state `Option<T>`/doubly-tri-state `Option<Option<T>>` tags (recipe §1.4's "tri-state"
# pattern, verbatim shape) — one per `Set*` variant's optional field.
option-hex = "[" "0" "]" | "[" "1" "," hex "]"
option-option-hex = "[" "0" "]" | "[" "1" "," option-hex "]"
option-spatial-kind = "[" "0" "]" | "[" "1" "," spatial-kind "]"
option-transform = "[" "0" "]" | "[" "1" "," transform "]"
option-element-class = "[" "0" "]" | "[" "1" "," element-class "]"
option-geometry-ref = "[" "0" "]" | "[" "1" "," geometry-ref "]"
option-psets = "[" "0" "]" | "[" "1" "," "[" pset-list? "]" "]"
option-relation-kind = "[" "0" "]" | "[" "1" "," relation-kind "]"

number = INT | FLOAT | IDENT | "-" "inf"

# `hex` (bare `hex`, deliberately NOT defined as a production here) is the framework's built-in
# `hex` MACRO — a run of lowercase hex digits (`enc_str`/`hex_encode`'s own alphabet), incl. the
# empty string. `Symbol::Ref` already falls back from "no production of this name" to "macro of
# this name", so referencing `hex` with no matching production routes here automatically. Using a
# hand-rolled `{INT|IDENT}*` production instead would be wrong: it never backtracks and would
# silently swallow the next literal keyword token (e.g. `id=<hex> kind=<hex>` — the literal `kind`
# tokenizes as a plain `IDENT`, exactly like a stray hex letter run would).
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🧬️schema/🧬️mutations/📝️text/📖️.grammar.semio

Original SHA-256: `2904d94b63e0e36b4ab46533c2feac7505117c5d818725be0404bb08f0350504`. Current SHA-256: `30590ca29786a319e42b900da2cf659f7e45ebf3e5d9054d4095a741ba74d023`.

Original:

```text
dialect grammar
grammar semio.cad.mutations
extension semio
start op

# cad wave: real one-line `OpText::print_op`/`parse_op` shape emitted by
# `🧬️mutations/🦀️.rs` (`print_cad_mutation`/`parse_cad_mutation`) —
# "keyword arg=value ..." (space-separated; space is lexer trivia at this dialect level). Every
# keyword/field-name token below is copied verbatim from the real `format!(...)` call sites in that
# file's `print_cad_mutation`, not invented.
op = set-snapshot | add-layer | remove-layer | set-layer | add-block | remove-block | set-block-base-point | add-entity | remove-entity | set-entity-layer | set-entity-geometry | add-block-entity | remove-block-entity | set-block-entity-layer | set-block-entity-geometry

set-snapshot = "set-snapshot" "snapshot" "=" snapshot-lit
add-layer = "add-layer" "layer" "=" layer
remove-layer = "remove-layer" "name" "=" hex
set-layer = "set-layer" "name" "=" hex "color-index" "=" option-i32 "line-type" "=" option-str "visible" "=" option-bool
add-block = "add-block" "block" "=" block
remove-block = "remove-block" "name" "=" hex
set-block-base-point = "set-block-base-point" "name" "=" hex "base-point" "=" point2
add-entity = "add-entity" "entity" "=" entity-record
remove-entity = "remove-entity" "handle" "=" hex
set-entity-layer = "set-entity-layer" "handle" "=" hex "layer" "=" hex
set-entity-geometry = "set-entity-geometry" "handle" "=" hex "entity" "=" entity
add-block-entity = "add-block-entity" "block-name" "=" hex "entity" "=" entity-record
remove-block-entity = "remove-block-entity" "block-name" "=" hex "handle" "=" hex
set-block-entity-layer = "set-block-entity-layer" "block-name" "=" hex "handle" "=" hex "layer" "=" hex
set-block-entity-geometry = "set-block-entity-geometry" "block-name" "=" hex "handle" "=" hex "entity" "=" entity

option-i32 = "[" "0" "]" | "[" "1" "," i32 "]"
option-str = "[" "0" "]" | "[" "1" "," hex "]"
option-bool = "[" "0" "]" | "[" "1" "," bool "]"

# `enc_cad_snapshot`/`dec_cad_snapshot` — `[hex(schema),[layer,...],[block,...],[entityRecord,...]]`.
snapshot-lit = "[" hex "," "[" layer-items? "]" "," "[" block-items? "]" "," "[" entity-record-items? "]" "]"
layer-items = layer {"," layer}*
block-items = block {"," block}*
entity-record-items = entity-record {"," entity-record}*

# `enc_layer`/`enc_block`/`enc_entity_record`/`enc_entity` — the same full-VALUE codecs
# `📸️snapshot/📝️text/📖️.grammar.semio` declares (restated here so this leaf grammar is
# self-contained, matching the repo's existing per-facet convention).
layer = "[" hex "," i32 "," hex "," bool "]"
entity-record = "[" hex "," hex "," entity "]"
entity-record-list = "[" entity-record-items? "]"
block = "[" hex "," point2 "," entity-record-list "]"

entity = line | arc | circle | ellipse | polyline | text-entity | insert | solid | dimension
line = "L" "[" point2 "," point2 "]"
arc = "A" "[" point2 "," number "," number "," number "]"
circle = "C" "[" point2 "," number "]"
ellipse = "E" "[" point2 "," point2 "," number "," number "," number "]"
polyline = "P" "[" point2-list "," bool "]"
text-entity = "T" "[" point2 "," number "," number "," hex "]"
insert = "I" "[" hex "," point2 "," point2 "," number "]"
solid = "S" "[" point2 "," point2 "," point2 "," point2 "]"
dimension = "D" "[" point2 "," point2 "," number "," hex "]"

point2 = "[" number "," number "]"
point2-list = "[" point2-items? "]"
point2-items = point2 {"," point2}*

i32 = INT
bool = "0" | "1"

number = INT | FLOAT | IDENT | "-" "inf"

# `hex` (bare `hex`, deliberately NOT defined as a production here) is the framework's built-in
# `hex` MACRO — a run of lowercase hex digits (`enc_str`/`hex_encode`'s own alphabet), incl. the
# empty string. `Symbol::Ref` already falls back from "no production of this name" to "macro of
# this name", so referencing `hex` with no matching production routes here automatically. Using a
# hand-rolled `{INT|IDENT}*` production instead would be wrong: it never backtracks and would
# silently swallow the next literal keyword token (e.g. `handle=<hex> layer=<hex>` — the literal
# `layer` tokenizes as a plain `IDENT`, exactly like a stray hex letter run would).
```

Current:

```text
dialect grammar
grammar semio.cad.mutations
extension semio
start op

# cad wave: real one-line `OpText::print_op`/`parse_op` shape emitted by
# `🧬️mutations/🦀️.rs` (`print_cad_mutation`/`parse_cad_mutation`) —
# "keyword arg=value ..." (space-separated; space is lexer trivia at this dialect level). Every
# keyword/field-name token below is copied verbatim from the real `format!(...)` call sites in that
# file's `print_cad_mutation`, not invented.
op = set-snapshot | patch-snapshot | add-layer | remove-layer | set-layer | add-block | remove-block | set-block-base-point | add-entity | remove-entity | set-entity-layer | set-entity-geometry | add-block-entity | remove-block-entity | set-block-entity-layer | set-block-entity-geometry

patch-snapshot = "patch-snapshot" "patch" "=" hex
set-snapshot = "set-snapshot" "snapshot" "=" snapshot-lit
add-layer = "add-layer" "layer" "=" layer
remove-layer = "remove-layer" "name" "=" hex
set-layer = "set-layer" "name" "=" hex "color-index" "=" option-i32 "line-type" "=" option-str "visible" "=" option-bool
add-block = "add-block" "block" "=" block
remove-block = "remove-block" "name" "=" hex
set-block-base-point = "set-block-base-point" "name" "=" hex "base-point" "=" point2
add-entity = "add-entity" "entity" "=" entity-record
remove-entity = "remove-entity" "handle" "=" hex
set-entity-layer = "set-entity-layer" "handle" "=" hex "layer" "=" hex
set-entity-geometry = "set-entity-geometry" "handle" "=" hex "entity" "=" entity
add-block-entity = "add-block-entity" "block-name" "=" hex "entity" "=" entity-record
remove-block-entity = "remove-block-entity" "block-name" "=" hex "handle" "=" hex
set-block-entity-layer = "set-block-entity-layer" "block-name" "=" hex "handle" "=" hex "layer" "=" hex
set-block-entity-geometry = "set-block-entity-geometry" "block-name" "=" hex "handle" "=" hex "entity" "=" entity

option-i32 = "[" "0" "]" | "[" "1" "," i32 "]"
option-str = "[" "0" "]" | "[" "1" "," hex "]"
option-bool = "[" "0" "]" | "[" "1" "," bool "]"

# `enc_cad_snapshot`/`dec_cad_snapshot` — `[hex(schema),[layer,...],[block,...],[entityRecord,...]]`.
snapshot-lit = "[" hex "," "[" layer-items? "]" "," "[" block-items? "]" "," "[" entity-record-items? "]" "]"
layer-items = layer {"," layer}*
block-items = block {"," block}*
entity-record-items = entity-record {"," entity-record}*

# `enc_layer`/`enc_block`/`enc_entity_record`/`enc_entity` — the same full-VALUE codecs
# `📸️snapshot/📝️text/📖️.grammar.semio` declares (restated here so this leaf grammar is
# self-contained, matching the repo's existing per-facet convention).
layer = "[" hex "," i32 "," hex "," bool "]"
entity-record = "[" hex "," hex "," entity "]"
entity-record-list = "[" entity-record-items? "]"
block = "[" hex "," point2 "," entity-record-list "]"

entity = line | arc | circle | ellipse | polyline | text-entity | insert | solid | dimension
line = "L" "[" point2 "," point2 "]"
arc = "A" "[" point2 "," number "," number "," number "]"
circle = "C" "[" point2 "," number "]"
ellipse = "E" "[" point2 "," point2 "," number "," number "," number "]"
polyline = "P" "[" point2-list "," bool "]"
text-entity = "T" "[" point2 "," number "," number "," hex "]"
insert = "I" "[" hex "," point2 "," point2 "," number "]"
solid = "S" "[" point2 "," point2 "," point2 "," point2 "]"
dimension = "D" "[" point2 "," point2 "," number "," hex "]"

point2 = "[" number "," number "]"
point2-list = "[" point2-items? "]"
point2-items = point2 {"," point2}*

i32 = INT
bool = "0" | "1"

number = INT | FLOAT | IDENT | "-" "inf"

# `hex` (bare `hex`, deliberately NOT defined as a production here) is the framework's built-in
# `hex` MACRO — a run of lowercase hex digits (`enc_str`/`hex_encode`'s own alphabet), incl. the
# empty string. `Symbol::Ref` already falls back from "no production of this name" to "macro of
# this name", so referencing `hex` with no matching production routes here automatically. Using a
# hand-rolled `{INT|IDENT}*` production instead would be wrong: it never backtracks and would
# silently swallow the next literal keyword token (e.g. `handle=<hex> layer=<hex>` — the literal
# `layer` tokenizes as a plain `IDENT`, exactly like a stray hex letter run would).
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🧬️schema/🧬️mutations/📝️text/📖️.grammar.semio

Original SHA-256: `d48f43dd54fd40e8377e26c3bcb84bb6c22db19fdf9beb24bfa914b9deb5d072`. Current SHA-256: `c6c5d117fda8a05102d377ee6873a7cd36ca2de4a939ceb434a8b48b8df0f323`.

Original:

```text
dialect grammar
grammar stdio.semio.document.mutations
extension semio
start op

# ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION document wave: real one-line
# `OpText::print_op`/`parse_op` shape ALREADY emitted by `🦀️.rs`'s
# `print_document_mutation`/`parse_document_mutation` — "keyword arg=value ..." (space-separated;
# space is lexer trivia at this dialect level). Every keyword/field-name token below is copied
# verbatim from the real `format!(...)` call sites in that file's `print_document_mutation`, not
# invented.
op = set-snapshot | insert-block | remove-block | set-block-content | set-paragraph-style | set-heading-level | set-list-ordered | set-run-text | set-run-style | set-image-block | insert-style | remove-style | set-style-name | set-style-based-on | insert-image | remove-image | set-image-bytes

set-snapshot = "set-snapshot" "snapshot" "=" snapshot-lit
insert-block = "insert-block" "path" "=" block-path "block" "=" block
remove-block = "remove-block" "path" "=" block-path
set-block-content = "set-block-content" "path" "=" block-path "block" "=" block
set-paragraph-style = "set-paragraph-style" "path" "=" block-path "style-id" "=" option-hex
set-heading-level = "set-heading-level" "path" "=" block-path "level" "=" INT
set-list-ordered = "set-list-ordered" "path" "=" block-path "ordered" "=" bool
set-run-text = "set-run-text" "path" "=" block-path "run-index" "=" INT "text" "=" hex
set-run-style = "set-run-style" "path" "=" block-path "run-index" "=" INT "style" "=" run-style
set-image-block = "set-image-block" "path" "=" block-path "image-id" "=" hex "alt" "=" hex "width" "=" option-num "height" "=" option-num
insert-style = "insert-style" "style" "=" style
remove-style = "remove-style" "id" "=" hex
set-style-name = "set-style-name" "id" "=" hex "name" "=" hex
set-style-based-on = "set-style-based-on" "id" "=" hex "based-on" "=" option-hex
insert-image = "insert-image" "image" "=" image
remove-image = "remove-image" "id" "=" hex
set-image-bytes = "set-image-bytes" "id" "=" hex "mime" "=" hex "bytes" "=" hex

# `enc_block_path`/`enc_path_segment` — `[[segments],index]`, segments tag-prefixed
# `Q[blockIndex]` (Quote) / `L[blockIndex,item]` (ListItem) / `T[blockIndex,row,cell]` (TableCell).
block-path = "[" "[" segment-items? "]" "," INT "]"
segment-items = segment {"," segment}*
segment = "Q" "[" INT "]" | "L" "[" INT "," INT "]" | "T" "[" INT "," INT "," INT "]"

# `enc_snapshot` — `[[styles],[images],[blocks]]`.
snapshot-lit = "[" "[" style-items? "]" "," "[" image-items? "]" "," "[" block-items? "]" "]"
style-items = style {"," style}*
image-items = image {"," image}*
block-items = block {"," block}*

# `enc_style`/`enc_image`/`enc_block`/`enc_run`/`enc_run_style` — the same full-VALUE codecs
# `../../📸️snapshot/📝️text/📖️.grammar.semio` declares (restated here so this leaf grammar
# is self-contained, matching the repo's existing per-facet convention).
style = "[" hex "," hex "," option-hex "]"
image = "[" hex "," hex "," hex "]"
run = "[" hex "," run-style "]"
run-style = "[" bool "," bool "," bool "," option-num "," option-hex "," option-hex "," option-hex "]"
list-item = "[" "[" block-items? "]" "]"
row = "[" "[" cell-items? "]" "]"
cell = "[" "[" block-items? "]" "]"
cell-items = cell {"," cell}*
block = "P" "[" option-hex "," "[" run-items? "]" "]" | "H" "[" INT "," option-hex "," "[" run-items? "]" "]" | "L" "[" bool "," "[" list-item-items? "]" "]" | "T" "[" "[" row-items? "]" "]" | "C" "[" option-hex "," hex "]" | "Q" "[" "[" block-items? "]" "]" | "I" "[" hex "," hex "," option-num "," option-num "]" | "B" "[" "]"
run-items = run {"," run}*
list-item-items = list-item {"," list-item}*
row-items = row {"," row}*

option-hex = "[" "0" "]" | "[" "1" "," hex "]"
# `enc_f64` prints `f64::to_bits()` (a `u64` bit pattern) as plain decimal digits, NOT a float
# literal — every numeric leaf (`u8` via `enc_u8`, `f64` via `enc_f64`) is a bare `INT` token.
option-num = "[" "0" "]" | "[" "1" "," INT "]"
bool = "0" | "1"

# `hex` (bare `hex`, deliberately NOT defined as a production here) is the framework's built-in
# `hex` MACRO — a run of lowercase hex digits (`enc_str`/`hex_encode`'s own alphabet), incl. the
# empty string. `Symbol::Ref`'s production->macro fallback resolves bare `hex` automatically, with
# real backtracking — a hand-rolled `{INT|IDENT}*` production would be wrong here (see the grammar
# recipe's own §3 pitfall #2: it would silently swallow the next literal keyword token, e.g.
# `path=<block-path> block=<block>` — the literal `block` tokenizes as a plain `IDENT`, exactly
# like a stray hex letter run would).
```

Current:

```text
dialect grammar
grammar stdio.semio.document.mutations
extension semio
start op

# ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION document wave: real one-line
# `OpText::print_op`/`parse_op` shape ALREADY emitted by `🦀️.rs`'s
# `print_document_mutation`/`parse_document_mutation` — "keyword arg=value ..." (space-separated;
# space is lexer trivia at this dialect level). Every keyword/field-name token below is copied
# verbatim from the real `format!(...)` call sites in that file's `print_document_mutation`, not
# invented.
op = set-snapshot | patch-snapshot | insert-block | remove-block | set-block-content | set-paragraph-style | set-heading-level | set-list-ordered | set-run-text | set-run-style | set-image-block | insert-style | remove-style | set-style-name | set-style-based-on | insert-image | remove-image | set-image-bytes

patch-snapshot = "patch-snapshot" "patch" "=" hex
set-snapshot = "set-snapshot" "snapshot" "=" snapshot-lit
insert-block = "insert-block" "path" "=" block-path "block" "=" block
remove-block = "remove-block" "path" "=" block-path
set-block-content = "set-block-content" "path" "=" block-path "block" "=" block
set-paragraph-style = "set-paragraph-style" "path" "=" block-path "style-id" "=" option-hex
set-heading-level = "set-heading-level" "path" "=" block-path "level" "=" INT
set-list-ordered = "set-list-ordered" "path" "=" block-path "ordered" "=" bool
set-run-text = "set-run-text" "path" "=" block-path "run-index" "=" INT "text" "=" hex
set-run-style = "set-run-style" "path" "=" block-path "run-index" "=" INT "style" "=" run-style
set-image-block = "set-image-block" "path" "=" block-path "image-id" "=" hex "alt" "=" hex "width" "=" option-num "height" "=" option-num
insert-style = "insert-style" "style" "=" style
remove-style = "remove-style" "id" "=" hex
set-style-name = "set-style-name" "id" "=" hex "name" "=" hex
set-style-based-on = "set-style-based-on" "id" "=" hex "based-on" "=" option-hex
insert-image = "insert-image" "image" "=" image
remove-image = "remove-image" "id" "=" hex
set-image-bytes = "set-image-bytes" "id" "=" hex "mime" "=" hex "bytes" "=" hex

# `enc_block_path`/`enc_path_segment` — `[[segments],index]`, segments tag-prefixed
# `Q[blockIndex]` (Quote) / `L[blockIndex,item]` (ListItem) / `T[blockIndex,row,cell]` (TableCell).
block-path = "[" "[" segment-items? "]" "," INT "]"
segment-items = segment {"," segment}*
segment = "Q" "[" INT "]" | "L" "[" INT "," INT "]" | "T" "[" INT "," INT "," INT "]"

# `enc_snapshot` — `[[styles],[images],[blocks]]`.
snapshot-lit = "[" "[" style-items? "]" "," "[" image-items? "]" "," "[" block-items? "]" "]"
style-items = style {"," style}*
image-items = image {"," image}*
block-items = block {"," block}*

# `enc_style`/`enc_image`/`enc_block`/`enc_run`/`enc_run_style` — the same full-VALUE codecs
# `../../📸️snapshot/📝️text/📖️.grammar.semio` declares (restated here so this leaf grammar
# is self-contained, matching the repo's existing per-facet convention).
style = "[" hex "," hex "," option-hex "]"
image = "[" hex "," hex "," hex "]"
run = "[" hex "," run-style "]"
run-style = "[" bool "," bool "," bool "," option-num "," option-hex "," option-hex "," option-hex "]"
list-item = "[" "[" block-items? "]" "]"
row = "[" "[" cell-items? "]" "]"
cell = "[" "[" block-items? "]" "]"
cell-items = cell {"," cell}*
block = "P" "[" option-hex "," "[" run-items? "]" "]" | "H" "[" INT "," option-hex "," "[" run-items? "]" "]" | "L" "[" bool "," "[" list-item-items? "]" "]" | "T" "[" "[" row-items? "]" "]" | "C" "[" option-hex "," hex "]" | "Q" "[" "[" block-items? "]" "]" | "I" "[" hex "," hex "," option-num "," option-num "]" | "B" "[" "]"
run-items = run {"," run}*
list-item-items = list-item {"," list-item}*
row-items = row {"," row}*

option-hex = "[" "0" "]" | "[" "1" "," hex "]"
# `enc_f64` prints `f64::to_bits()` (a `u64` bit pattern) as plain decimal digits, NOT a float
# literal — every numeric leaf (`u8` via `enc_u8`, `f64` via `enc_f64`) is a bare `INT` token.
option-num = "[" "0" "]" | "[" "1" "," INT "]"
bool = "0" | "1"

# `hex` (bare `hex`, deliberately NOT defined as a production here) is the framework's built-in
# `hex` MACRO — a run of lowercase hex digits (`enc_str`/`hex_encode`'s own alphabet), incl. the
# empty string. `Symbol::Ref`'s production->macro fallback resolves bare `hex` automatically, with
# real backtracking — a hand-rolled `{INT|IDENT}*` production would be wrong here (see the grammar
# recipe's own §3 pitfall #2: it would silently swallow the next literal keyword token, e.g.
# `path=<block-path> block=<block>` — the literal `block` tokenizes as a plain `IDENT`, exactly
# like a stray hex letter run would).
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🧬️schema/🧬️mutations/📝️text/📖️.grammar.semio

Original SHA-256: `25e9b588e61440bd70dadbdae5d662f3473e660840eac794cb906b9680dc075b`. Current SHA-256: `d0e3033c0d12d6aaf64c624dadee51947b965d21c2ea817537b64d5e2d347c0c`.

Original:

```text
dialect grammar
grammar stdio.semio.presentation.mutations
extension semio
start op

# ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION presentation wave: real one-line
# `OpText::print_op`/`parse_op` shape ALREADY emitted by `🦀️.rs`'s
# `print_presentation_mutation`/`parse_presentation_mutation` — "keyword arg=value ..."
# (space-separated; space is lexer trivia at this dialect level). This facet's text codec was
# already real, hand-rolled hex/bracket, pre-wave (the ticket's own earlier "14-variant vocabulary"
# phase — actually 15 counting `NoMutation`) — only the BINARY frame needed upgrading this wave.
# Every keyword/field-name token below is copied verbatim from the real `format!(...)` call sites in
# that file's `print_presentation_mutation`, not invented.
op = set-snapshot | insert-slide | remove-slide | set-slide-layout | set-slide-notes | insert-shape | remove-shape | set-shape-frame | set-text-box-blocks | insert-master | remove-master | insert-layout | remove-layout | set-layout-master

set-snapshot = "set-snapshot" "snapshot" "=" snapshot-lit
insert-slide = "insert-slide" "index" "=" INT "slide" "=" slide
remove-slide = "remove-slide" "index" "=" INT
set-slide-layout = "set-slide-layout" "index" "=" INT "layout-id" "=" option-hex
set-slide-notes = "set-slide-notes" "index" "=" INT "notes" "=" "[" block-items? "]"
insert-shape = "insert-shape" "slide-index" "=" INT "shape-index" "=" INT "shape" "=" shape
remove-shape = "remove-shape" "slide-index" "=" INT "shape-index" "=" INT
set-shape-frame = "set-shape-frame" "slide-index" "=" INT "shape-index" "=" INT "frame" "=" frame
set-text-box-blocks = "set-text-box-blocks" "slide-index" "=" INT "shape-index" "=" INT "blocks" "=" "[" block-items? "]"
insert-master = "insert-master" "master" "=" master
remove-master = "remove-master" "id" "=" hex
insert-layout = "insert-layout" "layout" "=" layout
remove-layout = "remove-layout" "id" "=" hex
set-layout-master = "set-layout-master" "id" "=" hex "master-id" "=" hex

# `enc_presentation_snapshot` — `[schema,[masters],[layouts],[slides]]`.
snapshot-lit = "[" hex "," "[" master-items? "]" "," "[" layout-items? "]" "," "[" slide-items? "]" "]"

# `master`/`layout`/`🎞️slide`/`shape`/`frame`/`point2`/`image`/`placeholder-kind`/`row`/`cell`/`block`
# — the same full-VALUE codecs `../../📸️snapshot/📝️text/📖️.grammar.semio` declares
# (restated here so this leaf grammar is self-contained, matching the repo's existing per-facet
# convention). `block`'s own `doc-row`/`doc-cell` sub-productions are copied from document's real
# grammar verbatim (renamed to avoid colliding with this file's own `row`/`cell`, which encode
# `SlideTableRow`/`SlideTableCell`).
master = "[" hex "," "[" shape-items? "]" "]"
layout = "[" hex "," hex "," "[" shape-items? "]" "]"
slide = "[" hex "," option-hex "," "[" shape-items? "]" "," "[" block-items? "]" "]"
master-items = master {"," master}*
layout-items = layout {"," layout}*
slide-items = slide {"," slide}*
shape-items = shape {"," shape}*
shape = "X" "[" frame "," "[" block-items? "]" "]" | "P" "[" frame "," image "]" | "T" "[" frame "," "[" row-items? "]" "]" | "H" "[" frame "," placeholder-kind "]"
frame = "[" point2 "," INT "," INT "]"
point2 = "[" INT "," INT "]"
image = "[" hex "," hex "," hex "]"
placeholder-kind = "T" | "S" | "B" | "F" | "N" | "D" | "O" "[" hex "]"
row-items = row {"," row}*
row = "[" cell-items? "]"
cell-items = cell {"," cell}*
cell = "[" block-items? "]"
block-items = block {"," block}*
block = "P" "[" option-hex "," "[" run-items? "]" "]" | "H" "[" INT "," option-hex "," "[" run-items? "]" "]" | "L" "[" bool "," "[" list-item-items? "]" "]" | "T" "[" "[" doc-row-items? "]" "]" | "C" "[" option-hex "," hex "]" | "Q" "[" "[" block-items? "]" "]" | "I" "[" hex "," hex "," option-num "," option-num "]" | "B" "[" "]"
run-items = run {"," run}*
run = "[" hex "," run-style "]"
run-style = "[" bool "," bool "," bool "," option-num "," option-hex "," option-hex "," option-hex "]"
list-item-items = list-item {"," list-item}*
list-item = "[" "[" block-items? "]" "]"
doc-row-items = doc-row {"," doc-row}*
doc-row = "[" "[" doc-cell-items? "]" "]"
doc-cell-items = doc-cell {"," doc-cell}*
doc-cell = "[" "[" block-items? "]" "]"

option-hex = "[" "0" "]" | "[" "1" "," hex "]"
# `enc_f64` prints `f64::to_bits()` (a `u64` bit pattern) as plain decimal digits, NOT a float
# literal — every numeric leaf is a bare `INT` token.
option-num = "[" "0" "]" | "[" "1" "," INT "]"
bool = "0" | "1"

# `hex` (bare `hex`, deliberately NOT defined as a production here) is the framework's built-in
# `hex` MACRO — a run of lowercase hex digits (`enc_str`/`hex_encode`'s own alphabet), incl. the
# empty string. `Symbol::Ref`'s production->macro fallback resolves bare `hex` automatically, with
# real backtracking — a hand-rolled `{INT|IDENT}*` production would be wrong here (see the grammar
# recipe's own §3 pitfall #2: it would silently swallow the next literal keyword token, e.g.
# `id=<hex> master-id=<hex>` — the literal `master-id` tokenizes as a plain `IDENT`, exactly like a
# stray hex letter run would).
```

Current:

```text
dialect grammar
grammar stdio.semio.presentation.mutations
extension semio
start op

# ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION presentation wave: real one-line
# `OpText::print_op`/`parse_op` shape ALREADY emitted by `🦀️.rs`'s
# `print_presentation_mutation`/`parse_presentation_mutation` — "keyword arg=value ..."
# (space-separated; space is lexer trivia at this dialect level). This facet's text codec was
# already real, hand-rolled hex/bracket, pre-wave (the ticket's own earlier "14-variant vocabulary"
# phase — actually 15 counting `NoMutation`) — only the BINARY frame needed upgrading this wave.
# Every keyword/field-name token below is copied verbatim from the real `format!(...)` call sites in
# that file's `print_presentation_mutation`, not invented.
op = set-snapshot | patch-snapshot | insert-slide | remove-slide | set-slide-layout | set-slide-notes | insert-shape | remove-shape | set-shape-frame | set-text-box-blocks | insert-master | remove-master | insert-layout | remove-layout | set-layout-master

patch-snapshot = "patch-snapshot" "patch" "=" hex
set-snapshot = "set-snapshot" "snapshot" "=" snapshot-lit
insert-slide = "insert-slide" "index" "=" INT "slide" "=" slide
remove-slide = "remove-slide" "index" "=" INT
set-slide-layout = "set-slide-layout" "index" "=" INT "layout-id" "=" option-hex
set-slide-notes = "set-slide-notes" "index" "=" INT "notes" "=" "[" block-items? "]"
insert-shape = "insert-shape" "slide-index" "=" INT "shape-index" "=" INT "shape" "=" shape
remove-shape = "remove-shape" "slide-index" "=" INT "shape-index" "=" INT
set-shape-frame = "set-shape-frame" "slide-index" "=" INT "shape-index" "=" INT "frame" "=" frame
set-text-box-blocks = "set-text-box-blocks" "slide-index" "=" INT "shape-index" "=" INT "blocks" "=" "[" block-items? "]"
insert-master = "insert-master" "master" "=" master
remove-master = "remove-master" "id" "=" hex
insert-layout = "insert-layout" "layout" "=" layout
remove-layout = "remove-layout" "id" "=" hex
set-layout-master = "set-layout-master" "id" "=" hex "master-id" "=" hex

# `enc_presentation_snapshot` — `[schema,[masters],[layouts],[slides]]`.
snapshot-lit = "[" hex "," "[" master-items? "]" "," "[" layout-items? "]" "," "[" slide-items? "]" "]"

# `master`/`layout`/`🎞️slide`/`shape`/`frame`/`point2`/`image`/`placeholder-kind`/`row`/`cell`/`block`
# — the same full-VALUE codecs `../../📸️snapshot/📝️text/📖️.grammar.semio` declares
# (restated here so this leaf grammar is self-contained, matching the repo's existing per-facet
# convention). `block`'s own `doc-row`/`doc-cell` sub-productions are copied from document's real
# grammar verbatim (renamed to avoid colliding with this file's own `row`/`cell`, which encode
# `SlideTableRow`/`SlideTableCell`).
master = "[" hex "," "[" shape-items? "]" "]"
layout = "[" hex "," hex "," "[" shape-items? "]" "]"
slide = "[" hex "," option-hex "," "[" shape-items? "]" "," "[" block-items? "]" "]"
master-items = master {"," master}*
layout-items = layout {"," layout}*
slide-items = slide {"," slide}*
shape-items = shape {"," shape}*
shape = "X" "[" frame "," "[" block-items? "]" "]" | "P" "[" frame "," image "]" | "T" "[" frame "," "[" row-items? "]" "]" | "H" "[" frame "," placeholder-kind "]"
frame = "[" point2 "," INT "," INT "]"
point2 = "[" INT "," INT "]"
image = "[" hex "," hex "," hex "]"
placeholder-kind = "T" | "S" | "B" | "F" | "N" | "D" | "O" "[" hex "]"
row-items = row {"," row}*
row = "[" cell-items? "]"
cell-items = cell {"," cell}*
cell = "[" block-items? "]"
block-items = block {"," block}*
block = "P" "[" option-hex "," "[" run-items? "]" "]" | "H" "[" INT "," option-hex "," "[" run-items? "]" "]" | "L" "[" bool "," "[" list-item-items? "]" "]" | "T" "[" "[" doc-row-items? "]" "]" | "C" "[" option-hex "," hex "]" | "Q" "[" "[" block-items? "]" "]" | "I" "[" hex "," hex "," option-num "," option-num "]" | "B" "[" "]"
run-items = run {"," run}*
run = "[" hex "," run-style "]"
run-style = "[" bool "," bool "," bool "," option-num "," option-hex "," option-hex "," option-hex "]"
list-item-items = list-item {"," list-item}*
list-item = "[" "[" block-items? "]" "]"
doc-row-items = doc-row {"," doc-row}*
doc-row = "[" "[" doc-cell-items? "]" "]"
doc-cell-items = doc-cell {"," doc-cell}*
doc-cell = "[" "[" block-items? "]" "]"

option-hex = "[" "0" "]" | "[" "1" "," hex "]"
# `enc_f64` prints `f64::to_bits()` (a `u64` bit pattern) as plain decimal digits, NOT a float
# literal — every numeric leaf is a bare `INT` token.
option-num = "[" "0" "]" | "[" "1" "," INT "]"
bool = "0" | "1"

# `hex` (bare `hex`, deliberately NOT defined as a production here) is the framework's built-in
# `hex` MACRO — a run of lowercase hex digits (`enc_str`/`hex_encode`'s own alphabet), incl. the
# empty string. `Symbol::Ref`'s production->macro fallback resolves bare `hex` automatically, with
# real backtracking — a hand-rolled `{INT|IDENT}*` production would be wrong here (see the grammar
# recipe's own §3 pitfall #2: it would silently swallow the next literal keyword token, e.g.
# `id=<hex> master-id=<hex>` — the literal `master-id` tokenizes as a plain `IDENT`, exactly like a
# stray hex letter run would).
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔊️audio/🧬️schema/🧬️mutations/📝️text/📖️.grammar.semio

Original SHA-256: `9575dabc941f10c078e5343330628570d86664b0dd6d32a0839285949e29f957`. Current SHA-256: `9770c1790c5a3eaa7c3ea3d2124f047724bd06615db2a8abd75cc9810c0159ea`.

Original:

```text
dialect grammar
grammar stdio.semio.audio.mutations
extension semio
start op

# ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION audio wave: real one-line
# `OpText::print_op`/`parse_op` shape ALREADY emitted by `🧬️mutations/🦀️.rs`
# (`print_audio_mutation`/`parse_audio_mutation`) — `keyword payload...` (space-separated; space
# is lexer trivia at this dialect level, matching how the real text splits on the FIRST space via
# `split_once(' ')`). Every keyword token below is copied verbatim from the real `format!(...)`
# call sites in that file's `print_audio_mutation`, not invented.
op = set-snapshot | set-sample-rate | set-format | insert-channel | remove-channel | set-channel-samples | insert-tag | remove-tag | set-tag-value

set-snapshot = "set-snapshot" snapshot
set-sample-rate = "set-sample-rate" INT
set-format = "set-format" format
insert-channel = "insert-channel" INT channel
remove-channel = "remove-channel" INT
set-channel-samples = "set-channel-samples" INT channel
insert-tag = "insert-tag" INT tag
remove-tag = "remove-tag" INT
set-tag-value = "set-tag-value" INT hex

# `snapshot` — the same full-VALUE codec `📸️snapshot/🦀️.rs`'s own text DSL grammar
# declares (restated here so this leaf grammar is self-contained, matching the repo's existing
# per-facet convention): schema-hex, sampleRate, format, channels, tags.
snapshot = "[" hex "," INT "," format "," "[" channel-list? "]" "," "[" tag-list? "]" "]"
channel-list = channel {"," channel}*
channel = "[" sample-list? "]"
sample-list = hex {"," hex}*
tag-list = tag {"," tag}*
tag = "[" hex "," hex "]"
format = "pcm8" | "pcm16" | "pcm24" | "pcm32" | "f32" | "f64"

# `hex` (bare `hex`, deliberately NOT defined as a production here) is the framework's built-in
# `hex` MACRO — a run of lowercase hex digits, incl. the empty string. `Symbol::Ref` already falls
# back from "no production of this name" to "macro of this name", so referencing `hex` with no
# matching production routes here automatically (recipe pitfall #2 — a hand-rolled `{INT|IDENT}*`
# production would silently swallow the next literal keyword token instead).
```

Current:

```text
dialect grammar
grammar stdio.semio.audio.mutations
extension semio
start op

# ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION audio wave: real one-line
# `OpText::print_op`/`parse_op` shape ALREADY emitted by `🧬️mutations/🦀️.rs`
# (`print_audio_mutation`/`parse_audio_mutation`) — `keyword payload...` (space-separated; space
# is lexer trivia at this dialect level, matching how the real text splits on the FIRST space via
# `split_once(' ')`). Every keyword token below is copied verbatim from the real `format!(...)`
# call sites in that file's `print_audio_mutation`, not invented.
op = set-snapshot | patch-snapshot | set-sample-rate | set-format | insert-channel | remove-channel | set-channel-samples | insert-tag | remove-tag | set-tag-value

patch-snapshot = "patch-snapshot" "patch" "=" hex
set-snapshot = "set-snapshot" snapshot
set-sample-rate = "set-sample-rate" INT
set-format = "set-format" format
insert-channel = "insert-channel" INT channel
remove-channel = "remove-channel" INT
set-channel-samples = "set-channel-samples" INT channel
insert-tag = "insert-tag" INT tag
remove-tag = "remove-tag" INT
set-tag-value = "set-tag-value" INT hex

# `snapshot` — the same full-VALUE codec `📸️snapshot/🦀️.rs`'s own text DSL grammar
# declares (restated here so this leaf grammar is self-contained, matching the repo's existing
# per-facet convention): schema-hex, sampleRate, format, channels, tags.
snapshot = "[" hex "," INT "," format "," "[" channel-list? "]" "," "[" tag-list? "]" "]"
channel-list = channel {"," channel}*
channel = "[" sample-list? "]"
sample-list = hex {"," hex}*
tag-list = tag {"," tag}*
tag = "[" hex "," hex "]"
format = "pcm8" | "pcm16" | "pcm24" | "pcm32" | "f32" | "f64"

# `hex` (bare `hex`, deliberately NOT defined as a production here) is the framework's built-in
# `hex` MACRO — a run of lowercase hex digits, incl. the empty string. `Symbol::Ref` already falls
# back from "no production of this name" to "macro of this name", so referencing `hex` with no
# matching production routes here automatically (recipe pitfall #2 — a hand-rolled `{INT|IDENT}*`
# production would silently swallow the next literal keyword token instead).
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔢️value/🧬️schema/🧬️mutations/📝️text/📖️.grammar.semio

Original SHA-256: `766d6f5fdd21b6cd3dd1abb4fbcbe739c074c7f2f5c552519365c6f83ae86fc9`. Current SHA-256: `5100df0133a145c5b343d968df4eb9a6588cfdad0e110d52e076a3dee39e5928`.

Original:

```text
dialect grammar
grammar semio.value.mutations
extension semio
start document

# `protocol::OpText::print_op`/`parse_op` (`🦀️.rs`'s `print_value_mutation`/
# `parse_value_mutation`) — `keyword arg=value ...` (space-separated), one keyword per variant.
# Every keyword/field-name token below is copied verbatim from the real `format!(...)` call sites,
# never invented. `document`'s own alternation stays ONE PHYSICAL LINE (recipe pitfall #4) by
# referencing each variant's own named production instead of inlining its full body.
document = set-snapshot | set-value | set-map-entry | remove-map-entry | insert-list-item | remove-list-item | set-node | remove-node

set-snapshot = "set-snapshot" "snapshot" "=" snapshot
set-value = "set-value" "path" "=" path "value" "=" value
set-map-entry = "set-map-entry" "path" "=" path "key" "=" hex "value" "=" value
remove-map-entry = "remove-map-entry" "path" "=" path "key" "=" hex
insert-list-item = "insert-list-item" "path" "=" path "index" "=" INT "value" "=" value
remove-list-item = "remove-list-item" "path" "=" path "index" "=" INT
set-node = "set-node" "id" "=" hex "value" "=" value
remove-node = "remove-node" "id" "=" hex

# `enc_path`/`dec_path` — a bracketed, comma-separated list of tagged segments: `K[hex(key)]`
# (map key) or `I[index]` (list index).
path = "[" path-item* "]"
path-item = path-segment ","?
path-segment = "K" "[" hex "]" | "I" "[" INT "]"

# `enc_semio_snapshot`/`dec_semio_snapshot` — `[hex(schema),value,[valueNode,...]]`.
snapshot = "[" hex "," value "," "[" value-node* "]" "]"
value-node = hex ":" value ","?

# `enc_semio_value`/`dec_semio_value` — restated here (identical to
# ../../🔺️diff/📝️text/📖️.grammar.semio's `value`) so this leaf grammar is self-contained.
# Genuinely recursive: `L`/`M` reference `list-item`/`map-item` which reference `value` back —
# exercised end-to-end by `ops_grammar_conformance_law` against real `print_op` output for every
# `SemioValueMutation` variant, incl. nested list/map payload values and a multi-segment mixed
# `SemioValuePath`.
value = "Z" | "B" "[" bit "]" | "I" "[" hex "]" | "F" "[" hex "]" | "S" "[" hex "]" | "Y" "[" hex "]" | "L" "[" list-item* "]" | "M" "[" map-item* "]" | "R" "[" hex "]"
list-item = value ","?
map-item = hex ":" value ","?
bit = "0" | "1"

# `hex` (bare `hex`, deliberately NOT defined as a production here) is the framework's built-in
# `hex` MACRO (`📖️grammar/🦀️.rs`'s `default_macros`/`macro_hex_ok`) — a run of lowercase
# hex digits, incl. the empty string. `Symbol::Ref` already falls back from "no production of this
# name" to "macro of this name", so referencing `hex` with no matching production routes here
# automatically. This MUST stay a macro, not a `hex = (HEXDIG HEXDIG)*`-shaped production: a
# production's `Star` is a single greedy pass with NO backtracking, so a `hex` run directly followed
# by an unrelated bareword literal it shares a token KIND with (e.g. `key=<hex> value=<value>` — the
# literal `value` tokenizes as a plain `IDENT`, exactly like a stray hex letter run) gets silently
# swallowed into the greedy `hex` Star, desyncing everything after it — the macro's own backtracking
# (`Recognizer::match_macro_span`, largest-span-first) is the only correct mechanism, per the
# recipe's own pitfall #2.
```

Current:

```text
dialect grammar
grammar semio.value.mutations
extension semio
start document

# `protocol::OpText::print_op`/`parse_op` (`🦀️.rs`'s `print_value_mutation`/
# `parse_value_mutation`) — `keyword arg=value ...` (space-separated), one keyword per variant.
# Every keyword/field-name token below is copied verbatim from the real `format!(...)` call sites,
# never invented. `document`'s own alternation stays ONE PHYSICAL LINE (recipe pitfall #4) by
# referencing each variant's own named production instead of inlining its full body.
document = set-snapshot | patch-snapshot | set-value | set-map-entry | remove-map-entry | insert-list-item | remove-list-item | set-node | remove-node

patch-snapshot = "patch-snapshot" "patch" "=" hex
set-snapshot = "set-snapshot" "snapshot" "=" snapshot
set-value = "set-value" "path" "=" path "value" "=" value
set-map-entry = "set-map-entry" "path" "=" path "key" "=" hex "value" "=" value
remove-map-entry = "remove-map-entry" "path" "=" path "key" "=" hex
insert-list-item = "insert-list-item" "path" "=" path "index" "=" INT "value" "=" value
remove-list-item = "remove-list-item" "path" "=" path "index" "=" INT
set-node = "set-node" "id" "=" hex "value" "=" value
remove-node = "remove-node" "id" "=" hex

# `enc_path`/`dec_path` — a bracketed, comma-separated list of tagged segments: `K[hex(key)]`
# (map key) or `I[index]` (list index).
path = "[" path-item* "]"
path-item = path-segment ","?
path-segment = "K" "[" hex "]" | "I" "[" INT "]"

# `enc_semio_snapshot`/`dec_semio_snapshot` — `[hex(schema),value,[valueNode,...]]`.
snapshot = "[" hex "," value "," "[" value-node* "]" "]"
value-node = hex ":" value ","?

# `enc_semio_value`/`dec_semio_value` — restated here (identical to
# ../../🔺️diff/📝️text/📖️.grammar.semio's `value`) so this leaf grammar is self-contained.
# Genuinely recursive: `L`/`M` reference `list-item`/`map-item` which reference `value` back —
# exercised end-to-end by `ops_grammar_conformance_law` against real `print_op` output for every
# `SemioValueMutation` variant, incl. nested list/map payload values and a multi-segment mixed
# `SemioValuePath`.
value = "Z" | "B" "[" bit "]" | "I" "[" hex "]" | "F" "[" hex "]" | "S" "[" hex "]" | "Y" "[" hex "]" | "L" "[" list-item* "]" | "M" "[" map-item* "]" | "R" "[" hex "]"
list-item = value ","?
map-item = hex ":" value ","?
bit = "0" | "1"

# `hex` (bare `hex`, deliberately NOT defined as a production here) is the framework's built-in
# `hex` MACRO (`📖️grammar/🦀️.rs`'s `default_macros`/`macro_hex_ok`) — a run of lowercase
# hex digits, incl. the empty string. `Symbol::Ref` already falls back from "no production of this
# name" to "macro of this name", so referencing `hex` with no matching production routes here
# automatically. This MUST stay a macro, not a `hex = (HEXDIG HEXDIG)*`-shaped production: a
# production's `Star` is a single greedy pass with NO backtracking, so a `hex` run directly followed
# by an unrelated bareword literal it shares a token KIND with (e.g. `key=<hex> value=<value>` — the
# literal `value` tokenizes as a plain `IDENT`, exactly like a stray hex letter run) gets silently
# swallowed into the greedy `hex` Star, desyncing everything after it — the macro's own backtracking
# (`Recognizer::match_macro_span`, largest-span-first) is the only correct mechanism, per the
# recipe's own pitfall #2.
```
