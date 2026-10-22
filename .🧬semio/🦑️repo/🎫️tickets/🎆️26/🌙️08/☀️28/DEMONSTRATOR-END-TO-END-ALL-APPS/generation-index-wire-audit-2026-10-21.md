# Generation Index Wire And Read Bounds

Read-only source audit after actual128MiB worker inflation refusal; no cache payloads read.

Exact owner: /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🟨️.mjs and /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧬️schema/🔣️.json. Policy.maximumIndexBytes is536870912. Producer store(P.indexes,index,.json.br,maximumIndexBytes) canonical-JSON serializes then Brotli-compresses; store checks packed byte length only and names file by packedSHA256. Normal readIndex checks that hash, then unpack(bytes,maximumIndexBytes) enforces512MiB expanded bound and readFactIndexV1 validates structure. Thus generic closure worker128MiB expansion bound is stricter than normalreader for a valid512MiB index. The actual file is proven >128MiB expanded; no claim is made whether it fits512MiB without decoding.

FactIndex is closed version/owners/rows/classes; version1; owners strings; rows closed kind/path/fact with fact arbitraryJSON; classes closed r/o integer arrays referring to row/owner indices. readFactIndexV1 validates exact top/class keys and reference ranges, but its normalreader does not invoke full Row schema validation. Source/lock reference strings may appear in arbitrary row.fact content, so none can be skipped or treated as optional.

A limited decoder can retain only parser state, topkeys/types, current row shape, final owner/row counts, class index maxima/range evidence, and unique Source/lock references while scanning every key/string. It must retain existing UTF8/JSON syntax, depth/node/read/output bounds and packedhash, not silently skip a generation payload. Canonical JSON puts classes before owners/rows; range validation must defer against final counts. Streaming references alone without closedshape/range checks is insufficient. A per-kind materialized bound would need actual memory evidence, not a blanket increase.

Producer store packed-only check also means producer can publish bytes whose expansion exceeds its normalreader512MiB; this audit does not attribute that condition to the current file. No Source edit or Native launch was performed.

## Typed decoder independent review boundary

Saved bounded decoder admits one producer-capped512MiB expanded Buffer, then64KiB fatalUTF8 slices with limited retainedtoken/reference state, rather than whole JSON tree. Grammar/number/stringescape/refcarry/chunk/depth/token work inspected. Concrete schema parity gaps at this read: owners element role permits anyJSON rather than string; rows permits anyJSON rather than closed nonemptykind/stringpath/fact; frontier directories/members elements do not yet enforce exacttuple3/2. Tests cover33grammar/reference/depth cases but not these missing type/shape negatives. Parent/Runtime informed for finite schema-first correction. This does not assert omitted references: all stringkeys/scalars are still scanned. No source/cache/native action by reviewer.

## Normative decoder correction readback

42case correction reviewed clean: required keys derive from actual FactIndex/FactClass/Row schema; owners strings, rows closednonemptykind/stringpath/fact, frontier exacttuple3/2. ActualAjv validates accepted neutralwires; nine negative type/shape vectors cover prior gaps. Parser still validates fatalUTF8/JSONgrammar/escapes/depth/tokens, all keys/string refs and deferred class ranges, without materializing fullfact object. Perkind expansion bound follows actual generation512MiB policy while generic materialized paths keep128MiB/4m limits. No concrete parser blocker remains at this read; combined signoff awaits root heap/Source authority boundary.

## Final Source custody and heap harness readback

Generation owner/schema now included in storage currentguard/Graph loadedroster/inference revisionfiles;9source19state fixture/index5 preserved. Standalone probe initially used rawhex rather than SEMIO markers; independent review identified both mismatches before admission, parent corrected them plus actual generationSchema port. Final small wire JSONoracle compares marker and scanner expects source:digest; large exact512MiB wire has262144unique marker owners, class o262143/r0 and one valid closedRow with bounded paddingstring.64KiB chunks/RSS sampling and model-only limitation intact. Structural review clean; actual heap probe execution/result owned by root, not this reviewer.
