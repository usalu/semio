# Virtual UI Handoff Review

The review film demonstrates physical understanding followed by ventilation. Its bodies contain two current native authored clause snapshots per lesson; this is a focused UI demonstration, not a regenerated full curriculum. Physical content uses the power-first scene; ventilation uses the volume scene. All original source-frame content is contained inside its lesson window.

The joined film has 96 fully decoded H.264 frames at 960×540, six fps, 16.000000 seconds. Component frame counts and durations: [(48, 8.0), (48, 8.0)]. All fixed navbar/footer pixels match throughout. Both component first/last frames and the adjacent physical-last/ventilation-first frames are byte-identical in decoded RGB. The joined frame count equals the exact sum of both components, and FFmpeg independently decodes the entire film without errors.

Video SHA-256: 8de6959da570b1c8eba454da59f36997678d7a7cc26568d018ac261bb35bde14

Decoded overview RGB SHA-256: 81fff30d63f2bd2d1d6a1ef15fcf88cd536bfaf0fe88d81463232419ecb544a4

The production envelope is tested separately against all five lesson IDs with real synthetic narrated sources, frame counts, waveform timing and native geometry. See virtual-ui-tests.md and virtual-ui-audit.md. Review media is retained; temporary render caches, logs and PNG sequences are removed at ticket close.
