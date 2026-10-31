# Local Source-Face Grouping

The original exact census found 25 opaque-visibility signatures across 16 family/axis groups. Nine extra signatures coincided with translation-dependent world-depth bucket partitions. This was not trigonometric axis drift or capsule-cache eviction.

The permanent schema-first contract at shared `🧪️cases/👁️occlusion/📦️grouping` selects all seven counterexample family/axis groups and 16 representative placements from that census. Each test independently transforms every source vertex with Rhino3dm, then compares an exact Counter of all actual placed source polygons. Only intrinsic grouping membership and keys failed before the change: seven observed failures in 6.885 s.

The correction changes only the grouping coordinates in `_assembly_items`: rounded source-local normal and source-local plane depth are computed before rigid placement. Actual world polygons, their normals, source vertex coordinates, current artist lighting and semantic ownership remain unchanged. No cache-key rounding or substitute geometry is introduced.

The first green run passed in 8.336 s. Strict AJV validation accepts the seven contracts and rejects three malformed variants. The strengthened run passes in 7.996 s and also proves fresh current semantic groups and material-underpaint owners for each translated placement. The final 140-slot signature census remains pending the separate cover-helper save. The separate bounded cover-helper optimization belongs to the execution worker.
