# Scientific Numeric Window Remaining Branch Audit — 2026-10-05

Read-only source follow-up; no compiler launched. The shared physical36 and mixed fixtures do not establish all numeric branches. Field window447 only sets numeric extents, frame477 draws chrome without applying authored keys, and window_keys755 is the separate override operation. A literal frame scan produced candidates, not defects; helpers can apply keys earlier.

Confirmed direct candidates in current source:

| Owner/branch | Source evidence | Bounded existing-owner completion |
|---|---|---|
| biology/genome-track gene-structure,domain,variant,pileup,transcript,synteny |1283 sets span×[0,1],1288–1294 enters chrome directly; coverage1318 separately applies window_keys |Apply authored keys after initial span window before dispatch, preserving coverage's postextent override; native feature points use mx/my1340–1390 so both axes affect body. Test noncoverage modes separately. |
| surface/climograph hyetograph |440–446 computes padded rainfall x and reversed y then frames without keys |Apply window_keys after computed window before frame; D3 rainfall bars use actual native map. Preserve reversed omitted default, explicit ascending/descending ranges each own override. |
| surface/geo-section seismic |555–556 sets [0,traces]×[4,0] then frame, no keys |Apply keys after window before frame. Trace data555 onward are x/y points passed through numeric polyline, so domain/range affects true body. Preserve time-down omitted default. |
| surface/climate-series small-multiple |337–341 sets per-panel height=(40−3(n−1))/n and auto extent then frame |Use parent authored height in panel budget and retain scoped parent dimensions. Apply authored keys after each series auto extent; intended common versus independent panel windows needs explicit contract, not a silent ignored override. |
| signal/music roll,midi,rhythm |599–641 directly reads domain but recomputes y from low/high or[0,1] and no keys |Domain already meaningful; authored range currently discarded. Apply keys after mode-specific y window before frame; assert MIDI/pitch rows and rhythm lane positions under actual projected range. |
| geometry/construction coordinate |136 sets hardcoded numeric window then forces axes/grid true and draws raw-mm outline/vertices |Domain/range and explicit axes/grid are overwritten/unread for this branch. It is a mixed physical body with numeric chrome; use an existing physical boundary around this branch with truthful raw-mm default extents, preserve explicit controls. Do not merely alter tick labels. |

These findings require actual neutral RED and subsequent PDF body/chrome GREEN before claims. Current mixed-numeric fixture covers orbital+6 pitch branches; mixed-physical covers6 branches plus newly queued decision lattice, not these source paths. No completion claim follows from the previously green216 physical cases.

A global frame→window_keys hook would reduce numeric omissions but can be too late for sampled-data timing, nested physical transforms and panel defaults. The smallest safe repairs are after each branch finishes its default window and before frame/body mapping. Preserve explicit axis pairs independently, localization and empty/constant data handling. Other literal-scan candidates (math density/conformal, physics levels/wave and skewt) may correctly apply keys in earlier helper/registration paths and must be adjudicated before adding repairs.

## Current owner binding at source read

Stable inputs are retained in all-family-mode-vocabulary-source; changed biology/field/math inputs are additionally bound in scientific-mode-current-stage-independent-binding. The following hashes identify the owner bytes used for this source audit, not a compiler receipt.

| Owner | SHA-256 |
|---|---|
| semio-viz-scientific-field.sty | 63E76C197A48A17E07DAF81DF97426E03A9B8A9FEBE861D1ECD90A27F1F1F899 |
| semio-viz-scientific-biology.sty | 5B1C85306DC1F4E4706B155464C96CF91FE599F34DA6B83F80E9F979F2242EE4 |
| semio-viz-scientific-surface.sty | A04790A75B05F2FC945CA7A783A336ADC5EA29F7097A3205B0B90BEC819D3018 |
| semio-viz-scientific-signal.sty | D31E8214211FAF4737FC5534E0F4D73AA02B6C5F447CEEB94CF57A7EDDD7C713 |
| semio-viz-scientific-geometry.sty | C1728F0E9507329DF00EA1FB8BC773E3CA2A7BDECFF7063149703BC8FB15BC6B |
| semio-viz-scientific-mathematics.sty | 1195B36F5BD4BB88BA98EE5C1244A168D5758E4B84A16F21746BB4AAA619CB37 |
| semio-viz-scientific-physics.sty | 164C84AF065FB7B3B0057D96DD1400587F15BB40FFEB5368172C87F35F0A18DD |
