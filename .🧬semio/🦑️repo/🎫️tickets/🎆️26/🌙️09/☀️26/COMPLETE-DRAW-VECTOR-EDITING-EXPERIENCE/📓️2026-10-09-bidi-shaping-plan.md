# ↔️ Original Unicode16 Direction and Shaping

## 📚️ Normative Source

Read the actual [Unicode16 UAX9 revision50](https://www.unicode.org/reports/tr9/tr9-50.html), dated2024-09-02. Direct historical opens initially failed; following the official revision52 ->51 ->50 previous-version links returned the pinned document. The data authority remains the already provisioned Unicode16 property/mirror/bracket tables. No external runtime bidi library is introduced.

UAX9 keeps source text in logical order and derives display ordering. Its paragraph, explicit embedding/isolate, weak/neutral, bracket, implicit-level and line reordering stages must all precede the final visual glyph placement. Shaping operates on logical level runs; the result is then reordered. Treating scalar reversal as shaping would break ligatures and attached marks.

## 🧬️ Authored Corpus and Authority

The new scalar-indexed neutral corpus covers empty/LTR/RTL paragraphs, mixed Arabic/Hebrew numbers, paired brackets, override, RLI/FSI and whitespace reset. Expected visible indices deliberately exclude formatting controls while retaining original source ordinals. An installed bidi-js test oracle verifies BMP fixture goldens; it is test-only. Its embedding implementation indexes UTF16 code units, so it cannot independently establish astral scalar behavior. The first-party producer must use admitted Unicode16 scalar properties for all codepoints.

## 🧵️ Required Producer

A native/TypeScript cursor will borrow the original normalized scalar sequence and original immutable Unicode bytes per turn, capturing source identities. Typed private owners hold cells, matching isolates, level runs, isolating run sequences, bracket pairs, resolved levels and visual indices. Every table probe, cell update, pair search, gap move and output copy advances under a bounded work grant; cancellation preserves the original source and withholds output. Native retirement must carry the actual owners through the existing four-currency controlled authority.

Composition must group logical scalars by resolved level and font, retain original grapheme/UTF8 clusters, mirror odd-level characters through the original Unicode table before cmap resolution, apply GSUB/GPOS in logical runs, and reorder complete glyph clusters with actual advances. Semantic Draw text remains the original authored value. The current shared run has no bidi composition yet; existing font/scene green receipts do not prove it.

## 🧪️ Status

Schema and twelve handcrafted neutral input/output fixtures are registered through the existing font-shape owner. Actual TypeScript cursor replay passed19 tests /279142 assertions /strict source EXIT0. Broader replay passed20 tests /279910 assertions, including768 direction/weak/neutral/isolate/bracket combinations. Actual native font_ replay passed26 tests /0 failures /EXIT0, including the original scalar bidi law at grants1/7/4096 and seven cancellations with observed normal-work zero heap release and caller-funded physical close. Frozen receipts: source-bidi-producer-green.log, source-bidi-combinations-green.log and native-bidi-producer-green.log. Native private owners use original PagedList backing; fixed bounded status/bracket stacks do not hide heap ownership. Composed FontRun integration remains pending. Registered native export route9 remains independently live. No provisioning scripts or manifests changed for this feature.
## 🧫️ Oracle Paragraph Boundary

The broader test first failed on original `(אב)` followed by LF and a later whitespace-only paragraph. Inspection of installed bidi-js showed its L1 backward reset uses `j >= 0` across paragraph boundaries, so the later paragraph rewrites the earlier separator level. The test now runs the independent embedding oracle per its authored paragraph before combining scalar-indexed results, preserving the normative paragraph-local L1 reset. Frozen actual red receipt: source-bidi-combinations-red.log. The first-party cursor was not changed to reproduce the oracle defect.
## 📝️ Composed Native and TypeScript Run

The actual native FontRun now retains normalization, bidi, grapheme, mirrored scalar owners and typed scalar-to-cluster/seen/visual-index tables. Current shipped Latin/Common faces shape whole graphemes in their native leftward-to-rightward buffer direction after resolved visual run selection, preserving attached marks and same-face/same-level feature boundaries. GSUB ligature and nested contextual replacement merge the minimum original UTF8 cluster through a bounded input scan, matching actual Rustybuzz cluster semantics when original graphemes are reversed. Source text remains the original authored value, including explicit controls.

Three shared Run fixtures validate right override AV, right override if ligature and mirrored (AV). Actual native output matched Rustybuzz glyph IDs, original source clusters, positions and advances at grants1/7/4096, then physically closed under real four-currency grants. Actual native font_ replay:26 passed/0failed/EXIT0; frozen native-composed-bidi-green.log. Actual TS font-outline replay:8passed/0failed/7204749assertions/strict source EXIT0; original OpenType contours and Sharp pixels matched all three new composed cases. Frozen source-composed-bidi-green.log. Earlier real failure returned A,V instead of V,A (source-composed-bidi-red.log).

A second actual red found non-default-ignorable BN input U+10FFFF silently disappearing from visual order. Run admission now probes the original Unicode property and explicitly refuses that unsupported nonignorable source; cancellation/failure withholds output while original text/font and all private owners remain available for controlled close. Frozen source-composed-bidi-red2.log and native-composed-bidi-red.log retain those failures. Native assertions took10.52s in final green under existing quick policy; the earlier inherited fundamental15s gate also reached a timeout after reporting its actual assertion failure. No budget or provisioning script was changed.

Three Draw scene twins are authored using actual native Rustybuzz goldens and remain in the live scene-text source replay. No fresh registered native PNG/SVG/PDF acceptance is implied. Arbitrary native RTL-script feature selection/joining beyond the shipped faces is still a separate required font capability; all existing font source coverage refusals remain explicit.

## 🎬️ Actual Draw Scene Replay

The fresh scene-text owner passed44 tests /0failures /786852assertions /strict source EXIT0. Three new right-direction/mirrored scene twins each ran the shared scene paint/SVG/PDF/raster/picking law, actual PNG/source-abort law and independent OpenType/Sharp solid-pixel law. Semantic authored controls stay in the original text owner while resolved contours carry visual glyph authority. Actual owner runtime79.87s; frozen source-scene-bidi-green.log. Native SceneText twins are authored using explicitly directed Rustybuzz oracle input, but no Draw native scene/export assertions have run because registered route9 stopped in stdio-semio before Draw. The native primitive and composed FontRun receipts do not replace that pending production proof.
