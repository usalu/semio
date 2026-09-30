# Windows Clone Path Budget

The stock checkout law is a clone root of at most 20 UTF-16 units (`pathBudget.cloneRootMax`). A file path may use 239 units below that root (`260 - 1`). `CreateDirectoryW` rejects a directory at `MAX_PATH - 12` (248), so a directory may use 227 units below the same root.

## What failed

Five tracked ticket copies were longer than 239 units, up to 271. They live under `END-TO-END-OS-HUB-COLLABORATION-MCP` work-package payloads (`wp-rs1/set/base`, `wp-en2/payload/energy-epjson/base`, `wp-en2/payload/gltf-any-reader/base`) and mirror plugin trees. One more payload file, `wp-en2/payload/gltf-production-inverse/base/.../🧬️mutations/🦀️.rs`, was 236 units but its directory was 229, which `CreateDirectoryW` rejects for a 20-unit clone root. Those copies are removed. The canonical plugin files stay.

Five product fixture directories were 228–230 units, over the directory budget by 1–3. The case leaf was shortened and the live references updated:

- `💦️drops-the-provided-humidification-to-1-point-25-kg-per-hour` → `💦️drops-the-provided-humidification-1-point-25-kg-per-hour`
- `🚪️raises-the-infiltration-allowance-to-52-point-5-m3-per-hour` → `🚪️raises-the-infiltration-allowance-52-point-5-m3-per-hour`
- `☁️raises-the-required-humidification-to-3-point-5-kg-per-hour` → `☁️raises-the-required-humidification-3-point-5-kg-per-hour`
- `✅️replace-query-result-applied` → `✅️replace-query-result-apply` (trinity jack results only)
- `✅️set-editor-selection-applied` → `✅️set-editor-selection-apply` (trinity jack editor only)

Historical ticket snapshots were left unchanged.

## Result

Working tree after the edit: longest file 238 UTF-16 units, longest directory 227, no Windows-illegal components, no component over 255. A clone root of 20 units stays within `CreateFileW` (259) and `CreateDirectoryW` (247). Longer roots still need `git clone -c core.longpaths=true`, which `setup git` records in the clone.
