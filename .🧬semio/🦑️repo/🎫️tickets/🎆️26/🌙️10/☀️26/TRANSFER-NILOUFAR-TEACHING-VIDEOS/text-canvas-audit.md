# Low Resolution Pango Canvas Diagnosis

Read-only product investigation. No teaching source was changed while the render pool was active. The retained `probe-text-canvas.py` and `probe-text-canvas-floor.py` inputs used the existing pinned Python/Manim environment through repo and created only private generated caches/results. Both processes ended0; no probe descendants remain alive.

## Deterministic Reproduction

The exact three failed strings, actual resolved Georgia face, authored target sizes22/24, line-spacing defaults, ligature setting and existing glyph fallback policy were tested against independent private text caches. The kerning interface internally renders these targets at size60 before scaling down.

| Text | Target size | Required nonspace glyphs | 320×180 | 320×480 | 854×180 | 854×480 | 1920×1080 |
| --- | ---: | ---: | --- | --- | --- | --- | --- |
| `erst beide Ströme geführt → Filter, Feuchte, Wärmerückgewinnung möglich` |22|63|60/fail|60/fail|63/pass|63/pass|63/pass|
| `volle Lüftungsverlust von 1500 W bleibt, η` |24|36|35/fail|35/fail|36/pass|36/pass|36/pass|
| `Nur der Anteil 1 − η bleibt als Verlust: bei η` |24|36|35/fail|35/fail|36/pass|36/pass|36/pass|

Height enlargement alone does not resolve the failure. Increasing canvas width does. The successful854×180,854×480 and1920×1080glyph point geometry hashes match exactly for each string. The two η strings need no font fallback; the arrow string retains the already accepted Cambria Math override. No font change, ligature removal or missing-character policy explains the difference.

## Primary Implementation Evidence

Installed Manim0.22 `mobject/text/text_mobject.py:835–865` passes `config.pixel_width` and `config.pixel_height` directly to `manimpango.text2svg`. Installed ManimPango `cmanimpango.pyx:61` creates a Cairo SVG surface at those dimensions; lines98–101set Pango layout width to that same width. The Cairo/Pango output therefore depends on requested movie pixel width even though Text SVG vectors should be reusable across output profiles.

Failed320SVGs are valid XML with a320wide viewport. For the η trials they contain no use of the final η glyph, whereas the matching854SVG includes it. The long arrow trial also changes Pango line layout and loses three nonspace glyphs. Manim's strict `_gen_chars` sees fewer imported SVG glyph objects than nonspace characters and raises its generic ligature diagnostic. The error text names a possible ligature problem, but this controlled experiment isolates output-width-dependent text SVG generation instead.

Manim's `_text2hash` hashes text/font/styles/font-size/line-spacing/ligature/color settings and omits canvas dimensions. The failed and successful SVGs consequently have identical16hex basenames (`bbfaea5ea4f5e7ee`, `b59f65845e159e76`, `3e610b5e296f33d8`). Reusing an existing valid-but-clipped SVG after changing only canvas behavior would preserve the failure. Private fresh caches or a distinct low-width canvas cache identity are necessary.

## Bounded Fix Proof

Inside an outer320×180config, a real Scene was created and retained its320×180camera. Constructing only the three Text objects under a temporary854canvas produced exactly the same glyph point bytes as their854×480baseline. The global output config and existing camera dimensions remained320×180after each construction. The independent854×180control demonstrates that only a minimum width is needed for these failures.

The smallest proposed product change is inside the existing font/Text interface: temporarily raise Pango text construction width to at least854, restore config immediately, preserve all854+ behavior, and namespace low-width SVG cache identity so existing clipped cache entries cannot be accepted. Keep fonts, sizes, scene construction, authored text and timings unchanged. Add neutral exact-string cases, real framework vector/cache assertions and camera/config restoration assertions before implementation. The parent explicitly authorized this fix only after the stable render owner confirms cancellation and zero descendants.

## Product Validation

The stable failed render was cancelled by its owner, who confirmed controller exit and zero captured descendants before any product write. Added exact-string cases and two framework-backed product tests before implementation. The initial test reproduced Manim's fewer-glyph failure. The first width-only fix passed all nine font tests and the complete32-test suite. The parent then refined the policy to a minimum854×480canvas and a fixed-length cache digest; the revised expected digest failed against the old prefix before implementing that refinement.

The final font interface temporarily raises only deficient text-canvas dimensions to854×480during the original Text constructor. Context restoration preserves global pixel/frame settings on both normal return and an injected framework exception; the existing Scene camera remains320×180. Fonts, target sizes, fallback rules, scene construction and authored timings remain unchanged. Text constructed at854×480and1920×1080retains its original16hex SVG cache name and exactly identical vector point arrays. Low-canvas SVG keys are `SHA256(original-key + "|text-canvas:" + canvas-profile)[:16]`, so valid stale narrow-canvas SVGs cannot be reused and filenames gain no length.

The final canonical native test entry completed32tests in30.661seconds, exit0. All nine font tests passed, including the independent FontTools cmap oracle,44-symbol geometry, the three exact failures, stale SVG isolation, repeated cache reuse, default/high cache identity, low-profile vector equality and success/error config restoration. The sole skip is the existing Windows symbolic-link privilege test (WinError1314). The real Manim segment/frame composition oracle and all canonical import/discovery/order/cancellation/provenance tests passed.

Final changed product files after the previous freeze:

| Teaching-relative file | SHA256 |
| --- | --- |
| `🏛️architecture/⚡️energy/🏗️authoring/🔤️fonts/🐍️.py` | `4c55b9d6a7372ce8de70a4de29d627f2093dc6fa27deedcc953f07350e7b2796` |
| `🏛️architecture/⚡️energy/🏗️authoring/🔤️fonts/🧪️tests/🐍️.py` | `389f69d4b87504c18275cb1c98cedc1f3c7c7b745ae874a41248a1d33a6fe83c` |
| `🏛️architecture/⚡️energy/🏗️authoring/🔤️fonts/🧪️tests/🔣️.cases.json` | `3e6fd1a48d4f1bf1e7529b875410640ac9721dc77fccfe265327d805a727825e` |

All owned probes/tests ended; no descendants remain. Product source is frozen for a fresh complete current-layout render. Earlier failed smoke outputs are evidence only and must not be relabelled as renders using this helper.
