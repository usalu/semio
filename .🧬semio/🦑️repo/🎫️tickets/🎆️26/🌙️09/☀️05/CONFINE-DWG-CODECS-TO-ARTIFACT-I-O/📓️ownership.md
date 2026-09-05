# DWG Artifact Ownership

DWG encoding and decoding now belong to the Stdio DWG artifact. Note, Layout, and Presentation compose their conversions inside their own artifact I/O. The framework no longer selects the `native-dwg-codec` feature or imports the DWG artifact codec.

## Changes

- Removed Stdio's `native-dwg-codec` feature and the OS host's Stdio dependency. The lockfile's host package has no Stdio dependency. DWG remains in the full artifact catalog.
- Split SVG conversion into generic, framework-owned `SvgPolyline` extraction and DWG artifact-owned `polylines_to_dwg_bytes`. Moved DWG-to-SVG conversion into the DWG artifact I/O and rewired Note, Layout, and Presentation callers.
- Removed unused framework DWG registrars, drawing/mesh exports, and the three DWG-specific Flow ABI operations. Updated the authoritative ABI schema, JavaScript method tables, and generated browser declarations together.
- Renamed the framework's mesh import service from DWG-specific names to `MeshDocumentImporter`, `MeshImportRequest`, `MeshImportResult`, `mesh_import`, and `import_mesh`. Updated CAD and Procedural registrations and builder laws.
- Removed the unused spatial-kernel DWG convenience export, which depended on the removed Flow codec entry point. Existing domain artifact I/O still provides DWG import/export.
- Moved GIS DWG projection and CAD/Raster DWG import tests from schema/editor facets into artifact I/O. Removed unused Shooting/Puzzle import callbacks and their obsolete self-tests.
- Made BREP kernel mesh import/export generic over supplied codec interfaces. Its DWG mesh implementations now live under BREP artifact I/O, and Flow operators call that I/O boundary.
- Added a language-neutral ownership fixture and independent AJV JSON Schema oracle. The existing Home I/O source check scans framework, general modules, and other artifact facets for escaped codec APIs; generated distribution bundles and registration descriptors are excluded. Added an artifact-owned polyline round-trip fixture and Rust test. Updated Home's native test gate to reference the generic SVG extraction test.

## Validation

- Test-first ownership check failed on the original framework violations, then passed after the boundary changes (initial four fixture cases). The final fixture expands this to eight allowed/rejected paths.
- `bun nx run semio-framework-os-flow-core:declarations --skip-nx-cache` passed: 105 schema methods matched the runtime prototype and TypeScript parser; three hostile declaration fixtures were rejected.
- Rust syntax parsing passed for the changed host, plugin service/builder, Flow drawing/component/protocol, and DWG I/O sources using `rustfmt --emit stdout` through Nx. This is a parse check, not a full crate compilation.
- Targeted `git diff --check` passed.
- Final ownership check passed: eight language-neutral cases agreed with AJV; framework, general modules, and other artifact facets were clean. The Home I/O oracle passed with four direct formats, four shared codecs, 36 catalog artifacts, and 35 consumer manifests.
- Home identity source gate passed: ten checks, including the corrected `workflow::tests::svg_path_extraction_preserves_transformed_geometry` test reference.
- Focused native runtime passed: usvg parsed a translated SVG rectangle, the artifact codec produced 359 DWG bytes, decoding preserved the exact four transformed vertices, closed flag, and `walls` layer, and artifact SVG rendering produced the expected 4×4 extent. Empty drawing and invalid SVG cases passed too.
- Final additional syntax checks passed: five TypeScript files and 13 Rust files covering the moved artifact helpers/tests and BREP/Flow changes. No full type-check or full native test pass is implied.
- All 53 ticket file paths were verified to exist, and the final targeted `git diff --check` passed.

## Validation Limits

- The browser test failed inside the pre-existing WASM binary at `flow_bridge_send` with `RuntimeError: Unreachable code should not be executed`. It did not reach the edited conversions; no complete browser runtime pass is claimed.
- The full native OS test exhausted the repository command time budget while compiling shared dependencies. The Stdio native test was cancelled during dependency compilation. Neither reached its selected Rust test, so neither is reported as passing.
- Later Nx CLI attempts failed while constructing the shared project graph. The focused checks use Nx's installed run-commands executor directly from the retained ticket script, with Bun, to avoid that unrelated graph construction dependency.
- The focused native harness compiles exact source excerpts for SVG extraction and the AC1015 polyline writer/reader/rendering path against the existing usvg library. It excludes unused R2010 decoder functions and is not a substitute for a full artifact crate build or DWG conformance suite.

## Repository Handling

Read `repo://goals` through the configured repository MCP. The older dissolution ticket was already open and its parent lookup failed, so this narrower ticket tracks the correction under `🎯aioptimizedrepo`. Concurrent edits were preserved. No modifying Git commands or worktrees were used. Generated logs and binaries are temporary; the ticket input script and this report remain.
