# Proctor Bootstrap Repair

Observed 2026-10-19. This bounded repair covers the proctor bootstrap and its focused regression suite. No native compiler volume was opened, no full native proctor build was attempted, and nothing was published.

## Changes

- Restored the missing `ProctorArtifact`, `ProctorExecutable`, `proctorBuildMode`, and source-directory exports and removed malformed declarations and undefined build locals.
- The build requires its actual issued invocation, derives its mode from the original native caller, refuses cross-mode requests, and selects the proctor package manifest explicitly.
- Preparation and native compilation use the current store callback's issued phase control, finite compiler storage, original network authority, and repository Cargo input admission.
- Completed native artifacts are accepted through `consume` and `withCapturedCargoArtifactsTaskV1`; immutable pinning happens while that genuine capture is live. The pinned artifact and compiler backing remain retained through children and byte exports, then close together.
- Restored metadata owner projection and asynchronous manifest closure as distinct authored source queries. Manifest closure imports now resolve the current repository source owners directly.
- Repaired the pinning regression to use actual issued test controls and ticket-contained temporary files. Added a language-neutral lifecycle fixture and bridge regression covering phase propagation, immutable pinning, child/export drain, and callback retirement. AJV validates portable contracts, and Noble SHA-256 independently validates pinned executable digests.

## Evidence

- Before repair, TypeScript parsing reported 17 syntax diagnostics in the bootstrap. After repair, parsing reported zero.
- Final focused Vitest execution: **20 tests passed, 1 test file passed**. It exercised real immutable pinning, leases, issued process controls, and bounded byte export. The bridge regression replaces native storage/compiler ports with explicit test seams; its success does not prove a native build.
- The compiler-volume-free TypeScript diagnostic program reported zero diagnostics scoped to the bootstrap, manifest closure, and focused regression file.
- Runtime assertions emit `[DEBUG]` messages for immutable generation retention and for compiler phase, capture, child drain, and export retirement.

## Reusable Commands

Run in `C:/git/semio`, in PowerShell:

```powershell
$env:SEMIO_TICKET_DIR = (Resolve-Path -LiteralPath '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️19/PREPARE-ARCHITEKTUR-UND-TECHNOLOGIE-FOR-END-TO-END-DEPLOYMENT').Path
node node_modules/vitest/vitest.mjs run --config '🎓️teaching/🏛️architecture/📚️course/🧪️tests/🎚️config/🟦️.ts' '🧪️tests/🏗️proctor-build/🟦️.ts' --maxWorkers=1
```

The following diagnostic reads the full configured TypeScript graph, emits no files, and filters the result to this repair. Remove or broaden the filter for the full course diagnostic gate.

```powershell
node --input-type=module -e "import ts from 'typescript'; const p='🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/tsconfig.json'; const read=ts.readConfigFile(p,ts.sys.readFile);const config=ts.parseJsonConfigFileContent(read.config,ts.sys,ts.getDirectoryPath(p));const program=ts.createProgram(config.fileNames,config.options);const result=ts.getPreEmitDiagnostics(program).filter(d=>d.file?.fileName.includes('🧪️tests/🏗️proctor-build/')||d.file?.fileName.includes('🛂️proctor/🏗️bootstrap/'));console.log('[DEBUG] Scoped proctor bootstrap and regression TypeScript diagnostics:',result.map(d=>({file:d.file.fileName,line:d.file.getLineAndCharacterOfPosition(d.start??0).line+1,message:ts.flattenDiagnosticMessageText(d.messageText,' ')})));process.exitCode=Number(result.length>0);"
```

## Files

- `🎓️teaching/🛂️proctor/🏗️bootstrap/🟦️.ts`
- `🎓️teaching/🛂️proctor/🏗️bootstrap/🕸️manifest-closure/🟦️.ts`
- `🎓️teaching/🛂️proctor/🏗️bootstrap/🧫️fixtures/🔣️.json`
- `🎓️teaching/🏛️architecture/📚️course/🧪️tests/🏗️proctor-build/🟦️.ts`
- This retained report.

Temporary test run directories were removed by their `finally` blocks. Shared ticket generated-directory cleanup and ticket closure belong to the parent task.
