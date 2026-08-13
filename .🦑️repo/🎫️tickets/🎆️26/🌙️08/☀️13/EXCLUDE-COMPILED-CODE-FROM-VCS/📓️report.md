# Exclude Compiled Code From Version Control Report

## Summary
Audited the repository for compiled artifacts (such as WASM files, `jco transpile` outputs, generated JS/TS bindings, and extension module build directories).

## Findings & Actions Taken
1. **Discovered Tracked Transpiled Artifacts**:
   - `jco transpile` generated JavaScript components, TypeScript declaration files, WASI interface declarations (`interfaces/*.d.ts`), host shims, and plugin worker scripts were being tracked inside `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/🔌️extension-modules/` (228 tracked files).

2. **Updated `.gitignore`**:
   - Added patterns under `#--------------------------------------OUTPUTS--------------------------------------`:
     - `**/🔌️extension-modules/`
     - `**/*.cwasm`
     - `**/*_component.js`
     - `**/*_component.d.ts`
     - `**/*_component.core.wasm`

3. **Untracked Compiled Files from Index**:
   - Removed `🔌️extension-modules/` from Git cache using `git rm --cached -r`.
   - Verified that `git check-ignore` correctly matches files in `🔌️extension-modules/`.
