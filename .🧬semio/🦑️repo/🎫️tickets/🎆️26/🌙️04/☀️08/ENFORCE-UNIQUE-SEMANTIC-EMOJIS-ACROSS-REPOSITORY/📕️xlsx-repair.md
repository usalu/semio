# XLSX Handpicked Emoji Repair

The ECMA-376 subsets are `🧱️base`, `🔒️strict`, and `🌉️transitional`. Oracles use `🔮️`, options `☑️`, mutation suites `🔀️`, and the formula-expansion scenario `🧮️`.

Handpicked Base operations/fixtures: sheet insertion `➕️`, sheet removal `➖️`, renaming `🏷️`, cell writing `✍️`, cell clearing `🧽️`, shared-string insertion `📥️`, shared-string removal `📤️`, shared-string editing `🔤️`, whole snapshot `📸️`. Structural operations/fixtures distinguish conformance setting `✅️`, conformance removal `🚫️`, main namespace `🌐️`, relationships namespace `🔗️`, worksheet content type `🏷️`, VML insertion `✒️`, and VML removal `🧹️`. Every carrier is `⬅️before.xlsx` or `➡️after.xlsx`.

Updated exact source references, Stdio mounts, oracle manifests, mutation catalog scenario paths, and the stale ZIP Base documentation coordinate. Existing September 2 XLSX generator inputs now have explicit authored fixture-directory names and corrected subset/oracle/carrier coordinates; no filename-selection algorithm was introduced or executed.

Verification on 2026-09-05:

- Final renamed-tree statute audit: 345 files, 236 directories, 581 governed entries; every finding category zero.
- Fixture verifier: 22 bundles, zero file problems, exit 0.
- All 42 JSON files parse; 22 manifests and 44 file paths resolve with exact SHA-256 and byte lengths.
- The bundled independent openpyxl 3.1.5 runtime successfully read all 16 Base XLSX fixture carriers.
- Both historical XLSX generator inputs parse as Python ASTs. They were not run against the worktree, preserving fixture bytes and recorded hashes.
- Current source scan contains no old subset coordinates, local oracle paths, bare carrier paths, or old snapshot mutation names.
