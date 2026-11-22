# Terrain Storage and Chunk Audit

SQLite projection and reconstruction preserve independently sized rows and columns and signed millimetre samples. Binary mutation tests cover i32 endpoints. The SQLite TypeScript test checks that the previous `rows` field is rejected.

The browser picker sends UTF-8 text in ordered parts of at most 32,768 bytes. The mounted framework routes those parts through `ImportStaging`, which returns a complete payload to the GIS receiver. Partial parts produce an empty invocation result. The addressed command route now keeps the assembled arguments across yielded preparation turns, as the addressed action route already did.

The Terrain raster picker admits 65,536 bytes. Its default action-to-command bridge previously capped the *encoded* command at 65,536 bytes including the command envelope. The raster import now uses the file tool's declared wire limit. Other commands retain the ordinary limit. The existing picker progress, cancellation, and retained raster processing are in use.

The shared fixture includes a 65 × 65 grid at the exact 65,536-byte picker boundary. It defines two transport chunks, signed i32 sample endpoints, spatial bounds, and the different NoData marker written by the canonical export. The mounted law requires that each transport callback leave the Store unchanged until settlement, that settlement complete once, and that Undo and Redo restore the exact snapshots. The browser witness checks one history entry and exact canonical ASCII exports before and after Undo and Redo.

GDAL's AAIGrid reader independently verified all sample values and coordinates. The native ticket probe compiled the actual Rust ASCII codec against an isolated schema stand-in. It accepted exactly 65,536 bytes, refused the same input with a 65,535-byte limit, and round-tripped the signed raster through canonical export. The updated mounted and browser witnesses require a fresh registered component build. The separate mounted command was cancelled during source admission to avoid competing with the active shared native build.
