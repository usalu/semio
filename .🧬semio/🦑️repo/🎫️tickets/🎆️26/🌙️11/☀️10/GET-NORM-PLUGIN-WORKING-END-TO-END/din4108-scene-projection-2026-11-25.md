# DIN 4108 Scene Projection

The existing thousand-element scene test measured 80.223446917 seconds for projection, after evaluating the report. The test allows five seconds and the limit is unchanged. The baseline is recorded in `coord-din4108-scene-final.log` at line 96903.

The whole-scene path already emitted geometry in one pass. For each emitted element, zone and bridge, the binding code scanned the report and rebuilt the document's three ID vectors for every applicable check. With checks proportional to elements, that work grew cubically. The clause lookup made another report scan per element. The structural-scene highlighter also made a new linear search through entities for every highlighted owner.

DIN 4108 now builds one borrowed index of governing checks and first clause rows per fragment. It uses the shared severity and utilisation comparison, normalizes derived subject IDs, and retains the first applicable row on ties. Clause lookup keeps the first report row for each check ID, including duplicates. The shared highlighter indexes emitted entity positions once, preserving preference for an exact ID and falling back to the first entity in emission order.

The new binding regression covers orphan checks, derived IDs, ties, informative and non-finite rows, and duplicate check IDs. Static whitespace checks passed.

The first verification was stopped by the shared DSL record model's source boundary at 262,873 bytes, 729 over the 262,144-byte limit. The model had gained controlled paged-map encoding. Moving the existing concrete field retirement implementation into the crate that owns `FieldValue` preserved that change and reduced the record model to 261,321 bytes with the same 26 files. The existing `native_encoding::retire_field` call site now re-exports that implementation. The retry passed the unchanged source limits and was admitted with one compiler worker.

The complete DIN 4108 scene suite passed all 20 tests, including the indexed binding regression, scene validation and the independent roof diagram oracle. The thousand-element projection took 66.42025 milliseconds on the same host, about 1,208 times faster than the original measured run. The full test suite finished in 0.24 seconds after compilation. The timing and result are recorded in `coord-din4108-scene-indexed-retry.log` at lines 96552 and 96577. The temporary timing print was removed after capture and the original five-second performance test is intact.
