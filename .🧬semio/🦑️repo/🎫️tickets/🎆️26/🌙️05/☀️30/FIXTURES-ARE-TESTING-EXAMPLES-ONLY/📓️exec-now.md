# Current bounded executable metadata

Original Source observations/hashes/path lengths in `📥️exec-now.json`; no execution or Source changes.

Flow test-session-source is under project targets, cache false, command `bun ./📜️script.ts test-session-source`, owner cwd {projectRoot}. The defining script router registers that command and controlled repository test execution resolves the original session TS via import.meta.dir/../../../host. Live and seed register the same Nx command, workspace cwd, 4_gate group and order900.039408, immediately after900.039407 in live. No alternate permanent script was introduced.

Terrain test now calls owner-relative `bun ./📜️script.ts test`; inferer owner cwd therefore agrees with the script path. Its existing script resolves the actual test through import.meta.dir and controlled execution. Plugin test-lifecycle is under targets, not namedInputs; every current namedInputs value in all three projects is an array. Both defining callers remain existing permanent script commands.

All six actual Source endpoint absolute paths are below256 codepoints and UTF16 units. Shared launch whole hashes are not promoted to authorship. Metadata coherence does not establish runtime success, cache behavior or producer receipts.
