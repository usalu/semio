# Models Trace Footer Layout

The actual late models movie showed the German loss-record phrase colliding with the next live counter. Evidence: generated/iv/bim-beats/bim-117-b24f3a52-02.png, trace at 156.50 seconds. Exact current canonical counts are 82 loss records and 13 loss groups; the small captured second count was visually ambiguous.

## Measured Native RED

The portable JSON fixture specifies English/German, counter/label sizes 21/17, minimum native gap 0.15, content bounds ±6.6, all integer values in each current counter range, and digit-boundary cases 0/9/10. The test evaluates the actual footer expression from the current authored _trace AST, using the real scene/native live text implementation.

RED in 32.763 seconds: German at 0/0 has adjacent gaps [0.15,-0.1531632441666665,0.15]. The negative middle gap independently confirms the overlap. English passed its full native counter sweep before the German failure.

## Local Repair

Only models.py changes: _trace_footer measures the widest native number across each actual model counter range and each localized label width, then centers four segments with 0.15 spacing. Fixed live counter anchors reserve those widths, so updater refreshes do not undo layout. Original counter 21/label 17 typography, wording and existing live callbacks remain unchanged.

Before source SHA256:3e1010e97e5551c197b4a12b1dca26e13e85d1574d24d86644d8eeeda57e6a03.
After source SHA256:1f607f4472a678025532e5d26a51d743abdea53cb45cde5c8c80f37bde36db5a.

The publisher owns the final normal models refresh; no duplicate full source encoder or global wrapper edit is started.


## Native GREEN

The full focused test passes in 69.013 seconds against stable current runtime inputs. It checks both locales, every integer value in each current live range, digit boundaries, all native adjacent gaps, content bounds, original counter dimensions against independent native reference text, active updater callbacks, and nonempty Cairo output.

The minimum allowed gap is 0.15 scene units; no counter or wording is shrunk. Endpoint images: generated/im/models-footer-en.png and models-footer-de.png.

The permanent JSON fixture and native driver live under models/🧪️tests/📏️footer and are registered in authoring/📜️script.ts. Registration preserved concurrent entries with a fresh byte comparison and atomic replacement. No wrapper restart is required.
