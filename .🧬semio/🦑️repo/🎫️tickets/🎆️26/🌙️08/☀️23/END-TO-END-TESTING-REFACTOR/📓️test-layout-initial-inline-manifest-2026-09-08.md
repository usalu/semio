# Initial JavaScript and TypeScript Test Attribution

This read-only audit compares the pre-goal tree at `6152f9ca6a0fbb55aa61992077837a230996b51d` with the current tree. It covers JavaScript and TypeScript files that were already beneath `🧪️tests` before the goal, excluding `*.test.*` and `*.spec.*` filenames owned by the separate legacy-filename lane. The two late TypeScript moves recorded in `📓️test-layout-late-typescript-2026-09-08.md` are excluded.

The initial snapshot contained 519 direct JavaScript/TypeScript implementation leaves after those two exclusions. Of 274 baseline paths, 212 moved or were removed from their original path. The audit proves 179 old-to-current canonical mappings: 24 by normalized-content equality after import/comment elision, 145 by a unique shared Vitest identity, 2 by semantic-owner preservation with full current test-identity equality, and 8 by same-owner assertion/export identity. It leaves 33 baseline paths without a canonical-destination attribution.

The guarded-registration pass starts from the current runner map. It requires three facts for attribution: the current caller explicitly imports the canonical destination under a guard, the same caller at the baseline contained an inline Vitest body, and baseline/current test identities overlap. This proves 71 distinct caller-to-destination pairs. Sixty-four guarded rows have no shared identity and are listed as unmatched.

## Exact Authored Paths

```json
[
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/📓️test-layout-initial-inline-manifest-2026-09-08.md",
  "♻️mit-bestand/🎤️präsentation/📅️33.projektetage/📦️packages/🟦️typescript/📦️index.ts",
  "♻️mit-bestand/🎤️präsentation/📅️33.projektetage/🧪️tests/🧪️projektetage-deck/🟦️.ts",
  "♻️mit-bestand/🧺️demonstrator/📜️script.ts",
  "♻️mit-bestand/🧺️demonstrator/🧪️tests/🧪️demonstratorruntimebuildvariants/🟦️.ts",
  "♻️mit-bestand/🧺️demonstrator/🧪️tests/🧪️scheduledemonstratoridle/🟦️.ts",
  "♻️mit-bestand/🧺️demonstrator/🪧️brand.ts",
  "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🍄️hexagonal-mushroom-column/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🍄️hexagonal-mushroom-column/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🍩️sphere-cut-with-torus/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🍩️sphere-cut-with-torus/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🐚️box-shell-preview/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🐚️box-shell-preview/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📐️box-fillet-preview/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📐️box-fillet-preview/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📦️rectangle-extrude-volume/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📦️rectangle-extrude-volume/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧲️sphere-box-fuse/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧲️sphere-box-fuse/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧹️face-sweep-extrude/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧹️face-sweep-extrude/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🪢️rectangle-wire-preview/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🪢️rectangle-wire-preview/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🌊️flow/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🎞️animate/🎛️apps/🎬️presentation/⚡️implementations/🟦️typescript/🟦️.ts",
  "✏️s/🔌️plugins/🎞️animate/🎛️apps/🎬️presentation/🧪️tests/🧪️loadpresentationfromslideglob/🟦️.ts",
  "✏️s/🔌️plugins/🎞️animate/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📺️renderer/⚛️react/🔨️modules/📝️markdown-html-compiler/🧪️tests/🧪️owned-markdown-html-compiler/🟦️.ts",
  "✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📺️renderer/⚛️react/🔨️modules/📝️markdown-html-compiler/🟦️.ts",
  "✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📺️renderer/⚛️react/🔨️modules/🔌️pdf-canvas-port/🧪️tests/🧪️pdfcanvasresourceowner/🟦️.ts",
  "✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📺️renderer/⚛️react/🔨️modules/🔌️pdf-canvas-port/🟦️.ts",
  "✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📺️renderer/⚛️react/🧪️tests/🧪️compilemarkdowntohtml/🟦️.tsx",
  "✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📺️renderer/⚛️react/🟦️.tsx",
  "✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/📦️packages/🟨️javascript/🧪️tests/🟨️.js",
  "✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🧪️tests/📌️retained-actions/🟨️.js",
  "✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🏗️fem/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️actions/🧪️tests/🧪️semio-tech-cad-js-core-box-display-committed/🟦️.ts",
  "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️actions/🟦️.ts",
  "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎰️stately/🧪️tests/🧪️semio-tech-cad-js-stately/🟦️.ts",
  "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎰️stately/🟦️.ts",
  "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏃️runtime/🧪️tests/🧪️semio-tech-cad-js-runtime/🟦️.ts",
  "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏃️runtime/🟦️.ts",
  "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📺️renderer/🧪️tests/🧪️repluserfacingsuggestiondetail/🟦️.tsx",
  "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📺️renderer/🟦️.tsx",
  "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🗿️artifact/🧪️tests/🧪️semio-tech-cad-js-core-interactions/🟦️.ts",
  "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🗿️artifact/🟦️.ts",
  "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧪️tests/🧪️semio-tech-cad-js-query-parse/🟦️.ts",
  "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🟦️.ts",
  "✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/🧪️tests/🧪️semio-tech-cad-js-module-aec-building-structure/🟦️.ts",
  "✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/🟦️.ts",
  "✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/🧪️tests/🧪️semio-tech-cad-js-module-aec-building/🟦️.ts",
  "✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/🟦️.ts",
  "✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/🧪️tests/🧪️semio-tech-cad-js-module-spatial-shape/🟦️.ts",
  "✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/🟦️.ts",
  "✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/🧪️tests/🧪️semio-tech-cad-js-module-aec-building-energy/🟦️.ts",
  "✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/🟦️.ts",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏢️high-consequence-office/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏢️high-consequence-office/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🔥️retail-hydrocarbon-fire/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🔥️retail-hydrocarbon-fire/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🛢️liquid-retaining-fem-anchor/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🛢️liquid-retaining-fem-anchor/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🔩️high-strength-connection/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🔩️high-strength-connection/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌉️composite-bridge-girder/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌉️composite-bridge-girder/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧱️loadbearing-wall/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧱️loadbearing-wall/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌉️glulam-footbridge/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌉️glulam-footbridge/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏠️aluminium-roof-purlin/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏠️aluminium-roof-purlin/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏢️seismic-rc-frame/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏢️seismic-rc-frame/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/📸️remodel/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🛰️synthetic-orbit/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🛰️synthetic-orbit/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🧩️suite/🟦️.ts",
  "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-600/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-600/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-600FF/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-600FF/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-610/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-610/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-620/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-620/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-630/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-630/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-640/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-640/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-650/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-650/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-900/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-900/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-900FF/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-900FF/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-910/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-910/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-920/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-920/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-930/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-930/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-940/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-940/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-950/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-950/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🔱️trinity/🔨️modules/🔌️jack/🧠️lsp/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🧩️suite/🟦️.ts",
  "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✅️valid/🧬️schema/🧪️tests/🧪️stdio-xml-valid-conformance-mirror/🟦️.ts",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✅️valid/🧬️schema/🟦️.ts",
  "✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/🎯️targets/⚛️5d-react/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/🎯️targets/⚛️5d-react/🟦️.tsx",
  "✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌲️concrete-forest/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌲️concrete-forest/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏗️nakagin-capsule-tower/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏗️nakagin-capsule-tower/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌙️capsule-dream/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌙️capsule-dream/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌲️concrete-forest/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌲️concrete-forest/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏗️nakagin-capsule-tower/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏗️nakagin-capsule-tower/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌲️concrete-forest/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌲️concrete-forest/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏗️nakagin-capsule-tower/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏗️nakagin-capsule-tower/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🧪️compose5d-preparetopologymodel/🟦️.tsx",
  "✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/➡️hexagonal-cut-concrete-forest-right/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/➡️hexagonal-cut-concrete-forest-right/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌲️hexagonal-cut-concrete-forest-left/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌲️hexagonal-cut-concrete-forest-left/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🧩️suite/🟦️.ts",
  "✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌲️hexagonal-cut-concrete-forest-left/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌲️hexagonal-cut-concrete-forest-left/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏢️nakagin-capsule/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏢️nakagin-capsule/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🧩️suite/🟦️.ts",
  "✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌲️hexagonal-cut-concrete-forest-left/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌲️hexagonal-cut-concrete-forest-left/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏢️nakagin-capsule/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏢️nakagin-capsule/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
  "✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts",
  "✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/📐️geometry/🧪️tests/🧪️semio-tech-cad-js-core-vec/🟦️.ts",
  "✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/📐️geometry/🟦️.ts",
  "✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🗺️spatial/🧪️tests/🧪️semio-tech-cad-js-core-model-commit-mesh/🟦️.ts",
  "✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🗺️spatial/🟦️.ts",
  "✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🧪️tests/🧪️semio-tech-cad-js-spatial-kernel-semio/🟦️.ts",
  "✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🟦️.ts",
  "✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧱️brepjs/🧪️tests/🧪️semio-tech-cad-js-brepjs/🟦️.ts",
  "✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧱️brepjs/🟦️.ts",
  "🌎️hub/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
  "🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
  "🌎️hub/🔨️modules/🛡️admin/🧱️elements/📚️I18n/🧪️tests/🧪️admin-i18n/🟦️.tsx",
  "🌎️hub/🔨️modules/🛡️admin/🧱️elements/📚️I18n/🟦️.tsx",
  "🧪️tests/🟦️.ts",
  "🧰️framework/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
  "🧰️framework/📦️packages/🟦️typescript/🟦️.ts",
  "🧰️framework/🔨️modules/◻️2d/🧪️tests/🧪️semio-tech-s-2d-js/🟦️.ts",
  "🧰️framework/🔨️modules/◻️2d/🟦️.ts",
  "🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/🧪️tests/🧪️kernelreturncontentframing-matches-the-shared-stream-and-independent-fra/🟦️.ts",
  "🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/🟦️.ts",
  "🧰️framework/🔨️modules/🎠️kernel/🧪️tests/🧪️createturnoutcomebroadcast/🟦️.ts",
  "🧰️framework/🔨️modules/🎠️kernel/🟦️.ts",
  "🧰️framework/🔨️modules/🎭️actor/📃️page/🧪️tests/🧪️actorbytepage-matches-shared-vectors-and-node-buffer-for-every-fixed-wor/🟦️.ts",
  "🧰️framework/🔨️modules/🎭️actor/📃️page/🟦️.ts",
  "🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🧪️tests/🧪️actorreturnresponseframing-uses-canonical-vectors-with-no-payload-copies/🟦️.ts",
  "🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🟦️.ts",
  "🧰️framework/🔨️modules/🎭️actor/📤️return/🧪️tests/🧪️actorreturn-codecs-load-natively-in-node-strip-only-mode-and-preserve-ev/🟦️.ts",
  "🧰️framework/🔨️modules/🎭️actor/📤️return/🟦️.ts",
  "🧰️framework/🔨️modules/🎭️actor/📥️cold-pair/🧪️tests/🧪️cold-pair-wit-status-codec-agrees-with-the-neutral-schema-and-rejects-ev/🟦️.ts",
  "🧰️framework/🔨️modules/🎭️actor/📥️cold-pair/🟦️.ts",
  "🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
  "🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🟦️.ts",
  "🧰️framework/🔨️modules/🎭️actor/📬️mailbox/🧪️tests/🧪️createboundedmailbox/🟦️.ts",
  "🧰️framework/🔨️modules/🎭️actor/📬️mailbox/🟦️.ts",
  "🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🧪️tests/🧪️shardclient-reserved-response-settlement/🟦️.ts",
  "🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts",
  "🧰️framework/🔨️modules/🎭️actor/🧪️tests/🧪️turnscheduler-lane-priority/🟦️.ts",
  "🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🧪️tests/🧪️ownedactorturnoutput/🟦️.ts",
  "🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🟦️.ts",
  "🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🧪️tests/🧪️actor-instance-close-fault-publication-fixture-preserves-watchdog-and-te/🟦️.ts",
  "🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🩹️patch/🧪️tests/🧪️actor-ui-patch-receipt-matches-shared-canonical-vectors-and-the-independ/🟦️.ts",
  "🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🩹️patch/🟦️.ts",
  "🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🟦️.ts",
  "🧰️framework/🔨️modules/📡️replication/🧪️tests/🧪️artifact-bootstrap-protocol/🟦️.ts",
  "🧰️framework/🔨️modules/📡️replication/🟦️.ts",
  "🧰️framework/🔨️modules/🔄️machine/🧪️tests/🧪️semio-tech-machine/🟦️.ts",
  "🧰️framework/🔨️modules/🔄️machine/🟦️.ts",
  "🧰️framework/🔨️modules/🕸️graph/🧪️tests/🧩️suite/🟦️.ts",
  "🧰️framework/🔨️modules/🕸️graph/🧪️tests/🟦️.ts",
  "🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/📜️script.ts",
  "🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/🧪️tests/🟦️.ts",
  "🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🟦️typescript/🟦️.ts",
  "🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🧩️suite/🟦️.ts",
  "🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🧪️levels-oklabmix/🟦️.ts",
  "🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🧪️playgroundflowwasmdevstubplugin/🟦️.ts",
  "🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🧪️theme-resolve/🟦️.ts",
  "🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🟦️.ts",
  "🧰️framework/🔨️modules/🖱️ui/🎨️styling/🟦️.ts",
  "🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️tests/🧪️package-export/🟦️.ts",
  "🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️tests/🟦️.ts",
  "🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🔨️modules/🕹️control-keybinding-context/🧪️tests/🧩️component/🟦️.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🔨️modules/🕹️control-keybinding-context/🧪️tests/🟦️.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧪️tests/📦️react-package-export/🟦️.ts",
  "🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🟦️.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🧪️tests/🧪️typednodefields-preflights-every-capture-before-transfer-under-private-r/🟦️.ts",
  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️.ts",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/↕️Collapsible/🧪️tests/🧩️component/🟦️.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/↕️Collapsible/🧪️tests/🟦️.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/⌨️Command/🧪️tests/🧩️component/🟦️.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/⌨️Command/🧪️tests/🟦️.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/☑️Checkbox/🧪️tests/🧩️component/🟦️.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/☑️Checkbox/🧪️tests/🟦️.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🧪️tests/🧩️component/🟦️.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🧪️tests/🟦️.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎚️Slider/🧪️tests/🧩️component/🟦️.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎚️Slider/🧪️tests/🟦️.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎛️ToggleGroup/🧪️tests/🧩️component/🟦️.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎛️ToggleGroup/🧪️tests/🟦️.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/💬️Dialog/🧪️tests/🧩️component/🟦️.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/💬️Dialog/🧪️tests/🟦️.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/📋️MenuItem/🧪️tests/🧩️component/🟦️.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/📋️MenuItem/🧪️tests/🟦️.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/📑️Tabs/🧪️tests/🧩️component/🟦️.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/📑️Tabs/🧪️tests/🟦️.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/📨️UIDialog/🧪️tests/🧩️component/🟦️.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/📨️UIDialog/🧪️tests/🟦️.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/📻️TableAvatar/🧪️tests/🧩️component/🟦️.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/📻️TableAvatar/🧪️tests/🟦️.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔀️Toggle/🧪️tests/🧩️component/🟦️.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔀️Toggle/🧪️tests/🟦️.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔽️Select/🧪️tests/🧩️component/🟦️.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔽️Select/🧪️tests/🟦️.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🕸️Diagram/🧪️tests/🧩️component/🟦️.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🕸️Diagram/🧪️tests/🟦️.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🗨️Popover/🧪️tests/🧩️component/🟦️.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🗨️Popover/🧪️tests/🟦️.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🧾️Form/🧪️tests/🧩️component/🟦️.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🧾️Form/🧪️tests/🟦️.tsx",
  "🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript/📜️script.ts",
  "🧰️framework/🔨️modules/🖼️assets/🥽️mesh/🧪️tests/🧩️suite/🟦️.ts",
  "🧰️framework/🔨️modules/🖼️assets/🥽️mesh/🧪️tests/🟦️.ts",
  "🧰️framework/🔨️modules/🖼️assets/🧪️tests/🧪️metabolism-icon-codegen/🟦️.ts",
  "🧰️framework/🔨️modules/🗺️surface/🧪️tests/🧩️suite/🟦️.ts",
  "🧰️framework/🔨️modules/🗺️surface/🧪️tests/🟦️.ts",
  "🧰️framework/🔨️modules/🧊️3d/🧪️tests/🧪️semio-tech-geometry-brep-js/🟦️.ts",
  "🧰️framework/🔨️modules/🧊️3d/🟦️.ts",
  "🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🧪️hostresolvedargs/🟦️.ts",
  "🧰️framework/🔨️modules/🛂️manifest/🟦️.ts",
  "🧰️framework/🧪️tests/🧪️docklayoutstore/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/⚡️effect-backbone.ts",
  "🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/🧪️tests/🧪️chunkkey/🟦️.tsx",
  "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/🟦️.tsx",
  "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🎨️react-renderer/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🎨️react-renderer/🧪️tests/🧪️canvaseventbindingcontroller/🟦️.tsx",
  "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🎨️react-renderer/🟦️.tsx",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧪️tests/🧪️resolvemcpbinarypath/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📦️packages/🟨️javascript/🧪️tests/🟨️.js",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📦️packages/🟨️javascript/🧪️tests/mock-flow-bridge.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧪️tests/⏱️consumed-browser-clock/🟨️.js",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧪️tests/🎭️mock-flow-bridge/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧪️tests/🎮️browser-interactive-job-port.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧪️tests/📨️browser-frame-transport.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧪️tests/🧩️package-integration.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️tests/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🎮️browser-interactive-job-port/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/📨️browser-frame-transport/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔌️plugin-runtime/🟦️.tsx",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧩️package-integration/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📃️UiDocumentStore/🧪️tests/🧪️typedwire/🟦️.tsx",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📃️UiDocumentStore/🟦️.tsx",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧪️tests/🧪️unknown-component-placeholder/🟦️.tsx",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🎯️action-handoff/🧪️tests/🧪️browser-actor-action-handoff-validates-the-neutral-schema-and-exact-owne/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🎯️action-handoff/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🩹️patch-handoff/🧪️tests/🧪️browser-actor-patch-handoff-validates-the-neutral-schema-and-exact-owner/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🩹️patch-handoff/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏪️store/📥️store.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏪️store/🧪️tests/🧪️authored-extension-installation-identity/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏪️store/🧪️tests/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📤️return/🧪️tests/🧪️pluginreturnwit-maps-every-canonical-drive-to-the-exact-wit-nesting-and/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📤️return/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📥️poll/🏘️composition/🧪️tests/🧪️pluginpollcompositionwit-preserves-the-canonical-six-scalars-and-exact-n/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📥️poll/🏘️composition/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🌲️fixture-projection/📜️script.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🌲️fixture-projection/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/🌳️tree/🧪️tests/🧪️rendertree/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/🌳️tree/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/🎬️media/🧪️tests/🧪️rendermedia/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/🎬️media/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/📃️document/🧪️tests/🧪️renderdocument/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/📃️document/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/📊️table/🧪️tests/🧪️rendertable/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/📊️table/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/🖼️image/🧪️tests/🧪️renderimage/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/🖼️image/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/🧊️mesh/🧪️tests/🧪️rendermesh/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/🧊️mesh/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/🧪️tests/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/🧪️tests/🧪️semio-tech-framework-os-shell-reduce/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/🧬️schema/🧪️tests/🧪️os-shell-schema-module/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/🧬️schema/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧪️tests/📤️macro-exports/📜️script.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧪️tests/📤️macro-exports/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧪️tests/🛂️mutation-source-authority/📜️script.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧪️tests/🛂️mutation-source-authority/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/🧪️tests/🧹️executable-source/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/vitest.config.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️ticket-owned-browser-host-staging/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🧪️tests/🧪️backbone-envelope-io/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🧪️tests/🧪️effectbackbone-capability-gating/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🧪️tests/🧪️space-artifact-creation-owner/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts",
  "🧰️framework/🛍️products/💻️os/🟦️.ts",
  "🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🖨️pipeline/🟦️.ts",
  "🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🟦️.ts",
  "🧰️framework/🛍️products/🖥️server/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/📜️script.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🕸️daemon/📜️script.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🕸️daemon/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎫️ticket-role-routing/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎭️source-roster-roles/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📋️mutation-inventory/🎫️ticket-role-routing/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📋️mutation-inventory/🎭️source-roster-roles/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📋️mutation-inventory/📸️source-index-capture/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📋️mutation-inventory/🧾️source-file-facts/🔮️oracle/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📋️mutation-inventory/🧾️source-file-facts/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📣️typescript-declaration-facts/🔮️oracle/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📸️source-index-capture/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔮️source-file-facts-oracle/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔮️typescript-declaration-facts-oracle/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧾️source-file-facts/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/🚪️source-admission-io/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/🚪️source-admission/🧪️io/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/👁️observe/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🧪️tests/⚙️transaction-process-ownership/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🧪️tests/🧪️transaction-process-ownership/🧪️test/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🧪️tests/🧪️transaction-process-ownership/🟦️.ts"
]
```

## Proven Baseline Test-File Moves

| Baseline source | Current canonical destination | Evidence |
| --- | --- | --- |
| `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo-session; ships primary asset` |
| `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo; ships primary asset` |
| `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo-session; ships primary asset` |
| `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo; ships primary asset` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo-session; ships primary asset` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo; ships primary asset` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo-session; ships primary asset` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🍄️hexagonal-mushroom-column/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🍄️hexagonal-mushroom-column/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `hexagonal-mushroom-column; ships primary asset` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🍩️sphere-cut-with-torus/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🍩️sphere-cut-with-torus/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `sphere-cut-with-torus; ships primary asset` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🐚️box-shell-preview/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🐚️box-shell-preview/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `box-shell-preview; ships primary asset` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📐️box-fillet-preview/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📐️box-fillet-preview/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `box-fillet-preview; ships primary asset` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📦️rectangle-extrude-volume/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📦️rectangle-extrude-volume/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `rectangle-extrude-volume; ships primary asset` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧲️sphere-box-fuse/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧲️sphere-box-fuse/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `sphere-box-fuse; ships primary asset` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧹️face-sweep-extrude/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧹️face-sweep-extrude/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `face-sweep-extrude; ships primary asset` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🪢️rectangle-wire-preview/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🪢️rectangle-wire-preview/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `rectangle-wire-preview; ships primary asset` |
| `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo-session; ships primary asset` |
| `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo; ships primary asset` |
| `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo-session; ships primary asset` |
| `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo; ships primary asset` |
| `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo-session; ships primary asset` |
| `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo; ships primary asset` |
| `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo-session; ships primary asset` |
| `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo; ships primary asset` |
| `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo-session; ships primary asset` |
| `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo; ships primary asset` |
| `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo-session; ships primary asset` |
| `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo; ships primary asset` |
| `✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo; ships primary asset` |
| `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/📦️packages/🟨️javascript/🧪️tests/🟨️.js` | `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🧪️tests/📌️retained-actions/🟨️.js` | same semantic owner; all three distinctive sequence fixture assertion messages match |
| `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo-session; ships primary asset` |
| `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo; ships primary asset` |
| `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo-session; ships primary asset` |
| `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo; ships primary asset` |
| `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo-session; ships primary asset` |
| `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo; ships primary asset` |
| `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo-session; ships primary asset` |
| `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo; ships primary asset` |
| `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo-session; ships primary asset` |
| `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo; ships primary asset` |
| `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo-session; ships primary asset` |
| `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo; ships primary asset` |
| `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo-session; ships primary asset` |
| `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo; ships primary asset` |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo-session; ships primary asset` |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo; ships primary asset` |
| `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo-session; ships primary asset` |
| `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo; ships primary asset` |
| `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo-session; ships primary asset` |
| `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo; ships primary asset` |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo-session; ships primary asset` |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏢️high-consequence-office/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏢️high-consequence-office/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `high-consequence-office; ships primary asset` |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo-session; ships primary asset` |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo; ships primary asset` |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo-session; ships primary asset` |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo; ships primary asset` |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo-session; ships primary asset` |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo; ships primary asset` |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo-session; ships primary asset` |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🔥️retail-hydrocarbon-fire/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🔥️retail-hydrocarbon-fire/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `retail-hydrocarbon-fire; ships primary asset` |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo-session; ships primary asset` |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🛢️liquid-retaining-fem-anchor/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🛢️liquid-retaining-fem-anchor/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `liquid-retaining-fem-anchor; ships primary asset` |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo-session; ships primary asset` |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo; ships primary asset` |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo-session; ships primary asset` |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo; ships primary asset` |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo-session; ships primary asset` |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🔩️high-strength-connection/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🔩️high-strength-connection/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `high-strength-connection; ships primary asset` |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo-session; ships primary asset` |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌉️composite-bridge-girder/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌉️composite-bridge-girder/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `composite-bridge-girder; ships primary asset` |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo-session; ships primary asset` |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo; ships primary asset` |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo-session; ships primary asset` |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧱️loadbearing-wall/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧱️loadbearing-wall/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `loadbearing-wall; ships primary asset` |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo-session; ships primary asset` |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌉️glulam-footbridge/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌉️glulam-footbridge/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `glulam-footbridge; ships primary asset` |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo-session; ships primary asset` |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏠️aluminium-roof-purlin/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏠️aluminium-roof-purlin/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `aluminium-roof-purlin; ships primary asset` |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo-session; ships primary asset` |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏢️seismic-rc-frame/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏢️seismic-rc-frame/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `seismic-rc-frame; ships primary asset` |
| `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo-session; ships primary asset` |
| `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo; ships primary asset` |
| `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo-session; ships primary asset` |
| `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo; ships primary asset` |
| `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo-session; ships primary asset` |
| `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo; ships primary asset` |
| `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🛰️synthetic-orbit/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🛰️synthetic-orbit/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `synthetic-orbit; ships primary asset` |
| `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🧩️suite/🟦️.ts` | unique-vitest-test-identity; `remodeling fixture oracle; discovers at least one vector for every mutation directory that ships tests` |
| `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo; ships primary asset` |
| `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-600/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-600/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `bestest-600; ships primary asset` |
| `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-600FF/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-600FF/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `bestest-600ff; ships primary asset` |
| `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-610/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-610/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `bestest-610; ships primary asset` |
| `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-620/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-620/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `bestest-620; ships primary asset` |
| `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-630/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-630/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `bestest-630; ships primary asset` |
| `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-640/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-640/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `bestest-640; ships primary asset` |
| `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-650/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-650/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `bestest-650; ships primary asset` |
| `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-900/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-900/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `bestest-900; ships primary asset` |
| `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-900FF/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-900FF/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `bestest-900ff; ships primary asset` |
| `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-910/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-910/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `bestest-910; ships primary asset` |
| `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-920/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-920/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `bestest-920; ships primary asset` |
| `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-930/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-930/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `bestest-930; ships primary asset` |
| `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-940/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-940/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `bestest-940; ships primary asset` |
| `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-950/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-950/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `bestest-950; ships primary asset` |
| `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo-session; ships primary asset` |
| `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo; ships primary asset` |
| `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo-session; ships primary asset` |
| `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo; ships primary asset` |
| `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo-session; ships primary asset` |
| `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo; ships primary asset` |
| `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo-session; ships primary asset` |
| `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo; ships primary asset` |
| `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo-session; ships primary asset` |
| `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo; ships primary asset` |
| `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🧩️suite/🟦️.ts` | normalized-content-with-import-comment-elision; `raster io — bmp v3 parity; ${name}: the TypeScript writer reproduces the Rust bytes` |
| `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo-session; ships primary asset` |
| `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo; ships primary asset` |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `example 🎬️demo-session; ships a non-empty cmd demo script` |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌲️concrete-forest/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌲️concrete-forest/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `example 🌲️concrete-forest; ships a non-empty dsl asset` |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏗️nakagin-capsule-tower/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏗️nakagin-capsule-tower/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `example 🏗️nakagin-capsule-tower; ships a non-empty dsl asset` |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `example 🎬️demo-session; ships a non-empty cmd demo script` |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌙️capsule-dream/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌙️capsule-dream/🧪️tests/🧩️example/🟦️.ts` | normalized-content-with-import-comment-elision; `capsule-dream example; exposes stable id and dsl url` |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌲️concrete-forest/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌲️concrete-forest/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `example 🌲️concrete-forest; ships a non-empty dsl asset` |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏗️nakagin-capsule-tower/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏗️nakagin-capsule-tower/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `example 🏗️nakagin-capsule-tower; ships a non-empty dsl asset` |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `example 🎬️demo-session; ships a non-empty cmd demo script` |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌲️concrete-forest/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌲️concrete-forest/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `example 🌲️concrete-forest; ships a non-empty dsl asset` |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏗️nakagin-capsule-tower/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏗️nakagin-capsule-tower/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `example 🏗️nakagin-capsule-tower; ships a non-empty dsl asset` |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/➡️hexagonal-cut-concrete-forest-right/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/➡️hexagonal-cut-concrete-forest-right/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `hexagonal-cut-concrete-forest-right; ships primary asset` |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌲️hexagonal-cut-concrete-forest-left/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌲️hexagonal-cut-concrete-forest-left/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `hexagonal-cut-concrete-forest-left; ships primary asset` |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🧩️suite/🟦️.ts` | normalized-content-with-import-comment-elision; `block2d io; ${asset}: the TypeScript json writer is a fixed point on the Rust bytes` |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌲️hexagonal-cut-concrete-forest-left/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌲️hexagonal-cut-concrete-forest-left/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `hexagonal-cut-concrete-forest-left; ships primary asset` |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏢️nakagin-capsule/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏢️nakagin-capsule/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `nakagin-capsule; ships primary asset` |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🧩️suite/🟦️.ts` | normalized-content-with-import-comment-elision; `block5d io; ${asset}: the TypeScript json writer is a fixed point on the Rust bytes` |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌲️hexagonal-cut-concrete-forest-left/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌲️hexagonal-cut-concrete-forest-left/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `hexagonal-cut-concrete-forest-left; ships primary asset` |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏢️nakagin-capsule/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏢️nakagin-capsule/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `nakagin-capsule; ships primary asset` |
| `✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo-session; ships primary asset` |
| `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo-session; ships primary asset` |
| `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo; ships primary asset` |
| `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo-session; ships primary asset` |
| `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts` | unique-vitest-test-identity; `demo; ships primary asset` |
| `🧰️framework/🔨️modules/🕸️graph/🧪️tests/🟦️.ts` | `🧰️framework/🔨️modules/🕸️graph/🧪️tests/🧩️suite/🟦️.ts` | unique-vitest-test-identity; `explicit output identities preserve independent manifest IDs and reject ambiguous paths; the producer writes exactly declared nested paths and refuses symlink traversal` |
| `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🟦️.ts` | `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🧩️suite/🟦️.ts` | same semantic owner; all 53 Vitest labels match current suite; caller runtime verified |
| `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️tests/🧪️package-export/🟦️.ts` | `🧰️framework/🔨️modules/🖱️ui/🧪️tests/📦️react-package-export/🟦️.ts` | unique-vitest-test-identity; `package entry and self-alias resolve to the canonical React source` |
| `🧰️framework/🔨️modules/🖱️ui/🔨️modules/🕹️control-keybinding-context/🧪️tests/🟦️.tsx` | `🧰️framework/🔨️modules/🖱️ui/🔨️modules/🕹️control-keybinding-context/🧪️tests/🧩️component/🟦️.tsx` | normalized-content-with-import-comment-elision; `owned hotkeys; normalizes mod to Meta on Apple platforms and Control elsewhere` |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/↕️Collapsible/🧪️tests/🟦️.tsx` | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/↕️Collapsible/🧪️tests/🧩️component/🟦️.tsx` | normalized-content-with-import-comment-elision; `Collapsible; updates uncontrolled state while keeping content mounted and associated` |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/⌨️Command/🧪️tests/🟦️.tsx` | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/⌨️Command/🧪️tests/🧩️component/🟦️.tsx` | normalized-content-with-import-comment-elision; `Command; folds Turkish-sensitive I forms without inheriting the host locale` |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/☑️Checkbox/🧪️tests/🟦️.tsx` | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/☑️Checkbox/🧪️tests/🧩️component/🟦️.tsx` | normalized-content-with-import-comment-elision; `Checkbox; forwards native change events and its input ref` |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🧪️tests/🟦️.tsx` | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🧪️tests/🧩️component/🟦️.tsx` | normalized-content-with-import-comment-elision; `TreeSection branch disclosure; preserves controlled branch state and its owned disclosure association` |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎚️Slider/🧪️tests/🟦️.tsx` | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎚️Slider/🧪️tests/🧩️component/🟦️.tsx` | normalized-content-with-import-comment-elision; `Slider; normalizes invalid ranges, steps, tuple values, and ready clamps` |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎛️ToggleGroup/🧪️tests/🟦️.tsx` | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎛️ToggleGroup/🧪️tests/🧩️component/🟦️.tsx` | normalized-content-with-import-comment-elision; `ToggleGroup; owns uncontrolled single selection and exact pressed state` |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/💬️Dialog/🧪️tests/🟦️.tsx` | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/💬️Dialog/🧪️tests/🧩️component/🟦️.tsx` | normalized-content-with-import-comment-elision; `Dialog; owns uncontrolled state, exact slots, stable associations, modal isolation, and focus return` |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📋️MenuItem/🧪️tests/🟦️.tsx` | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📋️MenuItem/🧪️tests/🧩️component/🟦️.tsx` | normalized-content-with-import-comment-elision; `MenuItem` |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📑️Tabs/🧪️tests/🟦️.tsx` | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📑️Tabs/🧪️tests/🧩️component/🟦️.tsx` | normalized-content-with-import-comment-elision; `Tabs; owns uncontrolled selection, associations, visibility, state, and distinct group IDs` |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📨️UIDialog/🧪️tests/🟦️.tsx` | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📨️UIDialog/🧪️tests/🧩️component/🟦️.tsx` | normalized-content-with-import-comment-elision; `UIDialog accessibility; isolates and dismisses dialogs within each owning Shell without blocking a sibling Shell` |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📻️TableAvatar/🧪️tests/🟦️.tsx` | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📻️TableAvatar/🧪️tests/🧩️component/🟦️.tsx` | normalized-content-with-import-comment-elision; `TableAvatar; shows a semantic fallback until the current image loads and restores it on error` |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔀️Toggle/🧪️tests/🟦️.tsx` | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔀️Toggle/🧪️tests/🧩️component/🟦️.tsx` | normalized-content-with-import-comment-elision; `Toggle; updates uncontrolled pressed state and proposes controlled changes` |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔽️Select/🧪️tests/🟦️.tsx` | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔽️Select/🧪️tests/🧩️component/🟦️.tsx` | normalized-content-with-import-comment-elision; `Select; owns fallback value, projected text, pointer selection, and focus return` |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🕸️Diagram/🧪️tests/🟦️.tsx` | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🕸️Diagram/🧪️tests/🧩️component/🟦️.tsx` | unique-vitest-test-identity; `owned Diagram directed layout; assigns stable ranks in every direction and respects variable node dimensions` |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🗨️Popover/🧪️tests/🟦️.tsx` | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🗨️Popover/🧪️tests/🧩️component/🟦️.tsx` | normalized-content-with-import-comment-elision; `Popover; owns uncontrolled, default, and controlled-lag state and trigger associations` |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🧾️Form/🧪️tests/🟦️.tsx` | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🧾️Form/🧪️tests/🧩️component/🟦️.tsx` | normalized-content-with-import-comment-elision; `Form; keeps native form ownership, uncancelled Enter, and submission` |
| `🧰️framework/🔨️modules/🖼️assets/🥽️mesh/🧪️tests/🟦️.ts` | `🧰️framework/🔨️modules/🖼️assets/🥽️mesh/🧪️tests/🧩️suite/🟦️.ts` | same semantic owner; all 3 Vitest labels match current suite; sibling metabolism suite shares none |
| `🧰️framework/🔨️modules/🗺️surface/🧪️tests/🟦️.ts` | `🧰️framework/🔨️modules/🗺️surface/🧪️tests/🧩️suite/🟦️.ts` | unique-vitest-test-identity; `surface compiler companions keep their exact paired identity in the handpicked output owner` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📦️packages/🟨️javascript/🧪️tests/🟨️.js` | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧪️tests/⏱️consumed-browser-clock/🟨️.js` | same semantic owner; identical exported testFlowBrowserClock identity |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📦️packages/🟨️javascript/🧪️tests/mock-flow-bridge.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧪️tests/🎭️mock-flow-bridge/🟦️.ts` | normalized-content-with-import-comment-elision |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧪️tests/🎮️browser-interactive-job-port.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🎮️browser-interactive-job-port/🟦️.ts` | normalized-content-with-import-comment-elision; `browser interactive job port; fails closed before payload ownership on duplicate, slot, and byte saturation` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧪️tests/📨️browser-frame-transport.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/📨️browser-frame-transport/🟦️.ts` | unique-vitest-test-identity; `browser frame worker transport; posts one transferable boot and remains fail-closed until the Worker acknowledges` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧪️tests/🧩️package-integration.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧩️package-integration/🟦️.ts` | unique-vitest-test-identity; `framework renderer wgpu plugin bridge; builds a JS bridge whose manifest() is synchronous JSON, matching ProgramBridge.rs's Reflect::get(handle, \` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🌲️fixture-projection/📜️script.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🌲️fixture-projection/🟦️.ts` | same semantic owner; identical exported testFixtureProjectionRetirement identity |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧪️tests/📤️macro-exports/📜️script.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧪️tests/📤️macro-exports/🟦️.ts` | same semantic owner; exact macro-export assertion and debug identity |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧪️tests/🛂️mutation-source-authority/📜️script.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧪️tests/🛂️mutation-source-authority/🟦️.ts` | same semantic owner; exact mutation-source-authority assertion and debug identity |
| `🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🟦️.ts` | `🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🖨️pipeline/🟦️.ts` | same semantic owner; identical exported verifyPrintPipelineQuick identity and current command import |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/📜️script.ts` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts` | TypeScript final extraction evidence; identical exported testCommandInputs identity |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🕸️daemon/📜️script.ts` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🕸️daemon/🟦️.ts` | same semantic owner; identical exported testGraphCoalescing identity |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📋️mutation-inventory/🎫️ticket-role-routing/🟦️.ts` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎫️ticket-role-routing/🟦️.ts` | unique-vitest-test-identity; `mutation ticket role routing vectors are closed and every field participates; mutation ticket role routing reaches only the mocked N admission boundary` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📋️mutation-inventory/🎭️source-roster-roles/🟦️.ts` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎭️source-roster-roles/🟦️.ts` | unique-vitest-test-identity; `mutation source roster roles vectors are closed; mutation source roster roles match the current public source record` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📋️mutation-inventory/📸️source-index-capture/🟦️.ts` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📸️source-index-capture/🟦️.ts` | unique-vitest-test-identity; `mutation source index captures admitted registered role files without a second collector` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📋️mutation-inventory/🧾️source-file-facts/🔮️oracle/🟦️.ts` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔮️source-file-facts-oracle/🟦️.ts` | normalized-content-with-import-comment-elision |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📋️mutation-inventory/🧾️source-file-facts/🟦️.ts` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧾️source-file-facts/🟦️.ts` | unique-vitest-test-identity; `mutation source-file facts vectors are closed and cover the registered source chains; mutation source-file facts reference oracle has strict standalone types` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📣️typescript-declaration-facts/🔮️oracle/🟦️.ts` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔮️typescript-declaration-facts-oracle/🟦️.ts` | normalized-content-with-import-comment-elision |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/🚪️source-admission/🧪️io/🟦️.ts` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/🚪️source-admission-io/🟦️.ts` | unique-vitest-test-identity; `taxonomy source admission IO; neutral cases satisfy independent Ajv schema` |
| `🧰️framework/🛍️products/🦑️repo/🧪️tests/🧪️transaction-process-ownership/🧪️test/🟦️.ts` | `🧰️framework/🛍️products/🦑️repo/🧪️tests/⚙️transaction-process-ownership/🟦️.ts` | unique-vitest-test-identity; `process observation has closed neutral schema and independent JSON authority; actual process decoder preserves 64-bit birth tokens through two compilers and SmartBuffer` |

## Proven Guarded Production Extraction

| Baseline source | Current canonical destination | Evidence |
| --- | --- | --- |
| `♻️mit-bestand/🎤️präsentation/📅️33.projektetage/📦️packages/🟦️typescript/📦️index.ts` | `♻️mit-bestand/🎤️präsentation/📅️33.projektetage/🧪️tests/🧪️projektetage-deck/🟦️.ts` | guarded import and shared Vitest identity: `projektetage deck; declares intro plus expanded render slides` |
| `♻️mit-bestand/🧺️demonstrator/📜️script.ts` | `♻️mit-bestand/🧺️demonstrator/🧪️tests/🧪️demonstratorruntimebuildvariants/🟦️.ts` | guarded import and shared Vitest identity: `demonstratorRuntimeBuildVariants; builds one additional artifact for six pane runtime variants` |
| `♻️mit-bestand/🧺️demonstrator/🪧️brand.ts` | `♻️mit-bestand/🧺️demonstrator/🧪️tests/🧪️scheduledemonstratoridle/🟦️.ts` | guarded import and shared Vitest identity: `scheduleDemonstratorIdle; never enters the idle queue before the minimum delay` |
| `✏️s/🔌️plugins/🎞️animate/🎛️apps/🎬️presentation/⚡️implementations/🟦️typescript/🟦️.ts` | `✏️s/🔌️plugins/🎞️animate/🎛️apps/🎬️presentation/🧪️tests/🧪️loadpresentationfromslideglob/🟦️.ts` | guarded import and shared Vitest identity: `loadPresentationFromSlideGlob; assembles chapters, sequences, thoughts, and ordered slides from slide paths` |
| `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📺️renderer/⚛️react/🔨️modules/📝️markdown-html-compiler/🟦️.ts` | `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📺️renderer/⚛️react/🔨️modules/📝️markdown-html-compiler/🧪️tests/🧪️owned-markdown-html-compiler/🟦️.ts` | guarded import and shared Vitest identity: `owned markdown html compiler; matches the installed compiler for` |
| `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📺️renderer/⚛️react/🔨️modules/🔌️pdf-canvas-port/🟦️.ts` | `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📺️renderer/⚛️react/🔨️modules/🔌️pdf-canvas-port/🧪️tests/🧪️pdfcanvasresourceowner/🟦️.ts` | guarded import and shared Vitest identity: `PdfCanvasResourceOwner; cancels the render before cleaning the page and destroying the document` |
| `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📺️renderer/⚛️react/🟦️.tsx` | `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📺️renderer/⚛️react/🧪️tests/🧪️compilemarkdowntohtml/🟦️.tsx` | guarded import and shared Vitest identity: `compileMarkdownToHtml; renders GFM tables as HTML` |
| `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️actions/🟦️.ts` | `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️actions/🧪️tests/🧪️semio-tech-cad-js-core-box-display-committed/🟦️.ts` | guarded import and shared Vitest identity: `@semio-tech/cad-js/core box display committed; keeps box-preview visible for committed state` |
| `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎰️stately/🟦️.ts` | `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎰️stately/🧪️tests/🧪️semio-tech-cad-js-stately/🟦️.ts` | guarded import and shared Vitest identity: `@semio-tech/cad-js/stately; buildSpatialStatelyMachineCatalogView lists scoped interactions with edges and mermaid` |
| `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏃️runtime/🟦️.ts` | `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏃️runtime/🧪️tests/🧪️semio-tech-cad-js-runtime/🟦️.ts` | guarded import and shared Vitest identity: `@semio-tech/cad-js/runtime; loads model definition manifests and catalogs` |
| `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📺️renderer/🟦️.tsx` | `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📺️renderer/🧪️tests/🧪️repluserfacingsuggestiondetail/🟦️.tsx` | guarded import and shared Vitest identity: `replUserFacingSuggestionDetail; keeps short shortcut keys and drops machine ids` |
| `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🗿️artifact/🟦️.ts` | `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🗿️artifact/🧪️tests/🧪️semio-tech-cad-js-core-interactions/🟦️.ts` | guarded import and shared Vitest identity: `@semio-tech/cad-js/core interactions; auto-commits curve.arc as one arc edge between start and end` |
| `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🟦️.ts` | `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧪️tests/🧪️semio-tech-cad-js-query-parse/🟦️.ts` | guarded import and shared Vitest identity: `@semio-tech/cad-js/query parse; parses MATCH RETURN with property access` |
| `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/🟦️.ts` | `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/🧪️tests/🧪️semio-tech-cad-js-module-aec-building-structure/🟦️.ts` | guarded import and shared Vitest identity: `@semio-tech/cad-js-module-aec-building-structure; computes structure stability stats with finite outputs` |
| `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/🟦️.ts` | `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/🧪️tests/🧪️semio-tech-cad-js-module-aec-building/🟦️.ts` | guarded import and shared Vitest identity: `@semio-tech/cad-js-module-aec-building; maps STEP layer names to building typologies` |
| `✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/🟦️.ts` | `✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/🧪️tests/🧪️semio-tech-cad-js-module-spatial-shape/🟦️.ts` | guarded import and shared Vitest identity: `@semio-tech/cad-js-module-spatial-shape; computes geometry stats for solid-backed objects` |
| `✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/🟦️.ts` | `✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/🧪️tests/🧪️semio-tech-cad-js-module-aec-building-energy/🟦️.ts` | guarded import and shared Vitest identity: `@semio-tech/cad-js-module-aec-building-energy; computes energy demand stats with finite outputs` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✅️valid/🧬️schema/🟦️.ts` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✅️valid/🧬️schema/🧪️tests/🧪️stdio-xml-valid-conformance-mirror/🟦️.ts` | guarded import and shared Vitest identity: `stdio.xml valid conformance mirror; conforming doctype reports only the always-on advisory` |
| `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/🎯️targets/⚛️5d-react/🟦️.tsx` | `✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🧪️compose5d-preparetopologymodel/🟦️.tsx` | guarded import and shared Vitest identity: `compose5d + prepareTopologyModel; flattens a fixed root and derived child with fastener x/y` |
| `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/📐️geometry/🟦️.ts` | `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/📐️geometry/🧪️tests/🧪️semio-tech-cad-js-core-vec/🟦️.ts` | guarded import and shared Vitest identity: `@semio-tech/cad-js/core vec; adds and distances` |
| `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🗺️spatial/🟦️.ts` | `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🗺️spatial/🧪️tests/🧪️semio-tech-cad-js-core-model-commit-mesh/🟦️.ts` | guarded import and shared Vitest identity: `@semio-tech/cad-js/core model commit mesh; appendCommittedMeshFaceToModel adds one mesh face from a triangle mesh` |
| `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🟦️.ts` | `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🧪️tests/🧪️semio-tech-cad-js-spatial-kernel-semio/🟦️.ts` | guarded import and shared Vitest identity: `@semio-tech/cad-js/spatial-kernel/semio; createBoxFromCorners volume matches axis-aligned footprint×height` |
| `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧱️brepjs/🟦️.ts` | `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧱️brepjs/🧪️tests/🧪️semio-tech-cad-js-brepjs/🟦️.ts` | guarded import and shared Vitest identity: `@semio-tech/cad-js/brepjs; createBoxFromCorners volume matches axis-aligned footprint×height` |
| `🌎️hub/🔨️modules/🛡️admin/🧱️elements/📚️I18n/🟦️.tsx` | `🌎️hub/🔨️modules/🛡️admin/🧱️elements/📚️I18n/🧪️tests/🧪️admin-i18n/🟦️.tsx` | guarded import and shared Vitest identity: `admin i18n; has an identical key set in en and de` |
| `🧰️framework/📦️packages/🟦️typescript/🟦️.ts` | `🧰️framework/🧪️tests/🧪️docklayoutstore/🟦️.ts` | guarded import and shared Vitest identity: `DockLayoutStore; returns null when nothing persisted` |
| `🧰️framework/🔨️modules/◻️2d/🟦️.ts` | `🧰️framework/🔨️modules/◻️2d/🧪️tests/🧪️semio-tech-s-2d-js/🟦️.ts` | guarded import and shared Vitest identity: `@semio-tech/s-2d-js; recognizes drawing refs` |
| `🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/🟦️.ts` | `🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/🧪️tests/🧪️kernelreturncontentframing-matches-the-shared-stream-and-independent-fra/🟦️.ts` | guarded import and shared Vitest identity: `KernelReturnContentFraming matches the shared stream and independent frame encoding at every split; KernelReturnContentFraming rejects every shared section-order violation and exact counted bodies` |
| `🧰️framework/🔨️modules/🎠️kernel/🟦️.ts` | `🧰️framework/🔨️modules/🎠️kernel/🧪️tests/🧪️createturnoutcomebroadcast/🟦️.ts` | guarded import and shared Vitest identity: `createTurnOutcomeBroadcast; multicasts one pushed value to EVERY live subscriber, not a shared drain-once FIFO` |
| `🧰️framework/🔨️modules/🎭️actor/📃️page/🟦️.ts` | `🧰️framework/🔨️modules/🎭️actor/📃️page/🧪️tests/🧪️actorbytepage-matches-shared-vectors-and-node-buffer-for-every-fixed-wor/🟦️.ts` | guarded import and shared Vitest identity: `ActorBytePage matches shared vectors and Node Buffer for every fixed word; ActorBytePage rejects invalid selected fields and nonzero padding without invoking getters` |
| `🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🟦️.ts` | `🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🧪️tests/🧪️actorreturnresponseframing-uses-canonical-vectors-with-no-payload-copies/🟦️.ts` | guarded import and shared Vitest identity: `ActorReturnResponseFraming uses canonical vectors with no payload copies or backing escape; ActorReturnResponseFraming keeps malformed bodies and incomplete authority failed` |
| `🧰️framework/🔨️modules/🎭️actor/📤️return/🟦️.ts` | `🧰️framework/🔨️modules/🎭️actor/📤️return/🧪️tests/🧪️actorreturn-codecs-load-natively-in-node-strip-only-mode-and-preserve-ev/🟦️.ts` | guarded import and shared Vitest identity: `ActorReturn codecs load natively in Node strip-only mode and preserve every shared vector; ActorReturnDrive matches the shared canonical vectors and independent LEB128 bytes` |
| `🧰️framework/🔨️modules/🎭️actor/📥️cold-pair/🟦️.ts` | `🧰️framework/🔨️modules/🎭️actor/📥️cold-pair/🧪️tests/🧪️cold-pair-wit-status-codec-agrees-with-the-neutral-schema-and-rejects-ev/🟦️.ts` | guarded import and shared Vitest identity: `cold pair WIT status codec agrees with the neutral schema and rejects every hostile authority` |
| `🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🟦️.ts` | `🧰️framework/🔨️modules/🎭️actor/🧪️tests/🧪️turnscheduler-lane-priority/🟦️.ts` | guarded import and shared Vitest identity: `TurnScheduler lane priority; dispatches by lane priority, not arrival order, when a batch lands before the first pick` |
| `🧰️framework/🔨️modules/🎭️actor/📬️mailbox/🟦️.ts` | `🧰️framework/🔨️modules/🎭️actor/📬️mailbox/🧪️tests/🧪️createboundedmailbox/🟦️.ts` | guarded import and shared Vitest identity: `createBoundedMailbox; overflow is rejected (not a silent drop) when nothing lower-priority exists to evict` |
| `🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts` | `🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🧪️tests/🧪️shardclient-reserved-response-settlement/🟦️.ts` | guarded import and shared Vitest identity: `ShardWorkerBootstrap declares only original metadata preparation and close methods; ShardWorkerBootstrap shared closing prefix ${prefix} ${closing} cannot admit a UI descendant` |
| `🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🟦️.ts` | `🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🧪️tests/🧪️ownedactorturnoutput/🟦️.ts` | guarded import and shared Vitest identity: `OwnedActorTurnOutput; ActorResponseAdmission declares conserved metadata and separate grants without receiver or refund authority` |
| `🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🩹️patch/🟦️.ts` | `🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🩹️patch/🧪️tests/🧪️actor-ui-patch-receipt-matches-shared-canonical-vectors-and-the-independ/🟦️.ts` | guarded import and shared Vitest identity: `actor UI patch receipt matches shared canonical vectors and the independent LEB128 encoder; actor UI patch receipt rejects invalid authority and enforces exact zero or one patch pairing` |
| `🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🟦️.ts` | `🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🧪️tests/🧪️actor-instance-close-fault-publication-fixture-preserves-watchdog-and-te/🟦️.ts` | guarded import and shared Vitest identity: `actor instance close fault publication fixture preserves watchdog and terminal-outcome precedence; actor instance close native value fixture accounts exact descendant text and independent cloned structure` |
| `🧰️framework/🔨️modules/📡️replication/🟦️.ts` | `🧰️framework/🔨️modules/📡️replication/🧪️tests/🧪️artifact-bootstrap-protocol/🟦️.ts` | guarded import and shared Vitest identity: `artifact bootstrap protocol; validates the neutral descriptor and SHA-256 values with AJV and Node crypto` |
| `🧰️framework/🔨️modules/🔄️machine/🟦️.ts` | `🧰️framework/🔨️modules/🔄️machine/🧪️tests/🧪️semio-tech-machine/🟦️.ts` | guarded import and shared Vitest identity: `@semio-tech/machine; flat table-driven machine advances red -> green -> yellow -> red` |
| `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/📜️script.ts` | `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🧪️levels-oklabmix/🟦️.ts` | guarded import and shared Vitest identity: `levels: oklabMix; t=0 returns a and t=1 returns b unchanged` |
| `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🟦️typescript/🟦️.ts` | `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🧪️theme-resolve/🟦️.ts` | guarded import and shared Vitest identity: `theme resolve; resolveThemePaint resolves a token ref` |
| `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🟦️.ts` | `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🧪️playgroundflowwasmdevstubplugin/🟦️.ts` | guarded import and shared Vitest identity: `playgroundFlowWasmDevStubPlugin; resolves bare @semio-tech/flow-core to the wasm-pack entry, not the stub` |
| `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx` | `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🟦️.tsx` | guarded import and shared Vitest identity: `owned locale detector retirement; normalizes the closed shell locale domain` |
| `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️.ts` | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🧪️tests/🧪️typednodefields-preflights-every-capture-before-transfer-under-private-r/🟦️.ts` | guarded import and shared Vitest identity: `TypedNodeFields preflights every capture before transfer under private reference saturation` |
| `🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript/📜️script.ts` | `🧰️framework/🔨️modules/🖼️assets/🧪️tests/🧪️metabolism-icon-codegen/🟦️.ts` | guarded import and shared Vitest identity: `metabolism icon codegen; maps metabolism stems to Rust variants` |
| `🧰️framework/🔨️modules/🧊️3d/🟦️.ts` | `🧰️framework/🔨️modules/🧊️3d/🧪️tests/🧪️semio-tech-geometry-brep-js/🟦️.ts` | guarded import and shared Vitest identity: `@semio-tech/geometry-brep-js; isRenderableMeshTransfer accepts triangle meshes` |
| `🧰️framework/🔨️modules/🛂️manifest/🟦️.ts` | `🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🧪️hostresolvedargs/🟦️.ts` | guarded import and shared Vitest identity: `🔖️HostResolvedArgs; encodeArtifactKindChoice matches the contract's pinned byte-identical fixture` |
| `🧰️framework/🛍️products/💻️os/⚡️effect-backbone.ts` | `🧰️framework/🛍️products/💻️os/🧪️tests/🧪️effectbackbone-capability-gating/🟦️.ts` | guarded import and shared Vitest identity: `EffectBackbone capability gating; mirrors backbone_send_is_rejected_without_the_capability: send is rejected without the messaging.backbone:<uri> capability` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/🟦️.tsx` | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/🧪️tests/🧪️chunkkey/🟦️.tsx` | guarded import and shared Vitest identity: `chunkKey; buckets origins by chunk size` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🎨️react-renderer/🟦️.tsx` | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🎨️react-renderer/🧪️tests/🧪️canvaseventbindingcontroller/🟦️.tsx` | guarded import and shared Vitest identity: `CanvasEventBindingController; disposes registered listeners` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🟦️.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧪️tests/🧪️resolvemcpbinarypath/🟦️.ts` | guarded import and shared Vitest identity: `resolveMcpBinaryPath; accepts an independently executable process artifact` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📃️UiDocumentStore/🟦️.tsx` | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📃️UiDocumentStore/🧪️tests/🧪️typedwire/🟦️.tsx` | guarded import and shared Vitest identity: `TypedWire; OwnedResidentScalarDeclaration` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx` | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔌️plugin-runtime/🟦️.tsx` | guarded import and shared Vitest identity: `RendererResidentComposition never replaces a closing composition ledger; RendererResidentComposition shares one exact ledger and preserves both consumers' charges` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx` | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧪️tests/🧪️unknown-component-placeholder/🟦️.tsx` | guarded import and shared Vitest identity: `unknown component placeholder; renders a visible placeholder and never nothing for an unregistered component type` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🎯️action-handoff/🟦️.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🎯️action-handoff/🧪️tests/🧪️browser-actor-action-handoff-validates-the-neutral-schema-and-exact-owne/🟦️.ts` | guarded import and shared Vitest identity: `browser actor action handoff validates the neutral schema and exact owner with an independent oracle` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🩹️patch-handoff/🟦️.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🩹️patch-handoff/🧪️tests/🧪️browser-actor-patch-handoff-validates-the-neutral-schema-and-exact-owner/🟦️.ts` | guarded import and shared Vitest identity: `browser actor patch handoff validates the neutral schema and exact owner with an independent oracle` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏪️store/📥️store.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏪️store/🧪️tests/🧪️authored-extension-installation-identity/🟦️.ts` | guarded import and shared Vitest identity: `authored extension installation identity; retains the declared physical name independently of the public extension ID` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📤️return/🟦️.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📤️return/🧪️tests/🧪️pluginreturnwit-maps-every-canonical-drive-to-the-exact-wit-nesting-and/🟦️.ts` | guarded import and shared Vitest identity: `PluginReturnWit maps every canonical drive to the exact WIT nesting and u64 request; PluginReturnWit matches the shared fixed result vectors and exact enum subset` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📥️poll/🏘️composition/🟦️.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📥️poll/🏘️composition/🧪️tests/🧪️pluginpollcompositionwit-preserves-the-canonical-six-scalars-and-exact-n/🟦️.ts` | guarded import and shared Vitest identity: `PluginPollCompositionWit preserves the canonical six scalars and exact nested field names; PluginPollCompositionWit checks every WIT u64 before conversion and keeps number and bigint dialects distinct` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/🌳️tree/🟦️.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/🌳️tree/🧪️tests/🧪️rendertree/🟦️.ts` | guarded import and shared Vitest identity: `renderTree; expands nested children recursively` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/🎬️media/🟦️.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/🎬️media/🧪️tests/🧪️rendermedia/🟦️.ts` | guarded import and shared Vitest identity: `renderMedia; renders duration, position, and kind as key-value entries` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/📃️document/🟦️.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/📃️document/🧪️tests/🧪️renderdocument/🟦️.ts` | guarded import and shared Vitest identity: `renderDocument; renders one child per page` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/📊️table/🟦️.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/📊️table/🧪️tests/🧪️rendertable/🟦️.ts` | guarded import and shared Vitest identity: `renderTable; serializes columns and rows into the table scene` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/🖼️image/🟦️.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/🖼️image/🧪️tests/🧪️renderimage/🟦️.ts` | guarded import and shared Vitest identity: `renderImage; builds a base64 data URI from mime + base64` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/🧊️mesh/🟦️.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/🧊️mesh/🧪️tests/🧪️rendermesh/🟦️.ts` | guarded import and shared Vitest identity: `renderMesh; carries the JSON blobs into the world3d scene` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/🧬️schema/🟦️.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/🧬️schema/🧪️tests/🧪️os-shell-schema-module/🟦️.ts` | guarded import and shared Vitest identity: `os.shell schema module; declares draft-07 and the owned $id` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/🟦️.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/🧪️tests/🧪️semio-tech-framework-os-shell-reduce/🟦️.ts` | guarded import and shared Vitest identity: `@semio-tech/framework-os-shell reduce; is pure and increments revision` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️ticket-owned-browser-host-staging/🟦️.ts` | guarded import and shared Vitest identity: `ticket-owned browser host staging; matches the neutral schema and closes only the exact Space, GIS and support module set` |
| `🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts` | `🧰️framework/🛍️products/💻️os/🧪️tests/🧪️space-artifact-creation-owner/🟦️.ts` | guarded import and shared Vitest identity: `space artifact creation owner; publishes only the canonical selected-current catalog for the exact Space` |
| `🧰️framework/🛍️products/💻️os/🟦️.ts` | `🧰️framework/🛍️products/💻️os/🧪️tests/🧪️backbone-envelope-io/🟦️.ts` | guarded import and shared Vitest identity: `backbone envelope io; readBackboneEnvelope retries a transient transport failure and then succeeds, with no real sleep` |

## Unmatched Baseline Paths

The remaining 33 baseline paths have no proven canonical current destination. Twenty-seven are Vitest wrappers without a literal test identity, four contain neither assertion nor self-test markers, and two have a known noncanonical relocation recorded below. No row is classified as non-executable.

```json
[
  {
    "source": "✏️s/🔌️plugins/🌊️flow/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    "classification": "vitest-wrapper-no-literal-identity",
    "reason": "no-unique-content-or-test-identity"
  },
  {
    "source": "✏️s/🔌️plugins/🎞️animate/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    "classification": "vitest-wrapper-no-literal-identity",
    "reason": "no-unique-content-or-test-identity"
  },
  {
    "source": "✏️s/🔌️plugins/🏗️fem/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    "classification": "vitest-wrapper-no-literal-identity",
    "reason": "no-unique-content-or-test-identity"
  },
  {
    "source": "✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    "classification": "vitest-wrapper-no-literal-identity",
    "reason": "no-unique-content-or-test-identity"
  },
  {
    "source": "✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    "classification": "vitest-wrapper-no-literal-identity",
    "reason": "no-unique-content-or-test-identity"
  },
  {
    "source": "✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    "classification": "vitest-wrapper-no-literal-identity",
    "reason": "no-unique-content-or-test-identity"
  },
  {
    "source": "✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    "classification": "vitest-wrapper-no-literal-identity",
    "reason": "no-unique-content-or-test-identity"
  },
  {
    "source": "✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    "classification": "vitest-wrapper-no-literal-identity",
    "reason": "no-unique-content-or-test-identity"
  },
  {
    "source": "✏️s/🔌️plugins/📸️remodel/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    "classification": "vitest-wrapper-no-literal-identity",
    "reason": "no-unique-content-or-test-identity"
  },
  {
    "source": "✏️s/🔌️plugins/🔱️trinity/🔨️modules/🔌️jack/🧠️lsp/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    "classification": "vitest-wrapper-no-literal-identity",
    "reason": "no-unique-content-or-test-identity"
  },
  {
    "source": "✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/🎯️targets/⚛️5d-react/🧪️tests/🟦️.ts",
    "classification": "no-assertion-or-selftest-marker",
    "reason": "no-unique-content-or-test-identity"
  },
  {
    "source": "✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    "classification": "vitest-wrapper-no-literal-identity",
    "reason": "no-unique-content-or-test-identity"
  },
  {
    "source": "🌎️hub/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    "classification": "vitest-wrapper-no-literal-identity",
    "reason": "no-unique-content-or-test-identity"
  },
  {
    "source": "🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    "classification": "vitest-wrapper-no-literal-identity",
    "reason": "no-unique-content-or-test-identity"
  },
  {
    "source": "🧪️tests/🟦️.ts",
    "classification": "vitest-wrapper-no-literal-identity",
    "reason": "no-unique-content-or-test-identity"
  },
  {
    "source": "🧰️framework/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    "classification": "vitest-wrapper-no-literal-identity",
    "reason": "no-unique-content-or-test-identity"
  },
  {
    "source": "🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    "classification": "vitest-wrapper-no-literal-identity",
    "reason": "no-unique-content-or-test-identity"
  },
  {
    "source": "🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/🧪️tests/🟦️.ts",
    "classification": "vitest-wrapper-no-literal-identity",
    "reason": "no-unique-content-or-test-identity"
  },
  {
    "source": "🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️tests/🟦️.ts",
    "classification": "vitest-wrapper-no-literal-identity",
    "reason": "no-unique-content-or-test-identity"
  },
  {
    "source": "🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    "classification": "vitest-wrapper-no-literal-identity",
    "reason": "no-unique-content-or-test-identity"
  },
  {
    "source": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    "classification": "vitest-wrapper-no-literal-identity",
    "reason": "no-unique-content-or-test-identity"
  },
  {
    "source": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🎨️react-renderer/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    "classification": "vitest-wrapper-no-literal-identity",
    "reason": "no-unique-content-or-test-identity"
  },
  {
    "source": "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    "classification": "vitest-wrapper-no-literal-identity",
    "reason": "no-unique-content-or-test-identity"
  },
  {
    "source": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️tests/🟦️.ts",
    "classification": "vitest-wrapper-no-literal-identity",
    "reason": "no-unique-content-or-test-identity"
  },
  {
    "source": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏪️store/🧪️tests/🟦️.ts",
    "classification": "no-assertion-or-selftest-marker",
    "reason": "no-unique-content-or-test-identity"
  },
  {
    "source": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/🟦️.ts",
    "classification": "vitest-wrapper-no-literal-identity",
    "reason": "no-unique-content-or-test-identity"
  },
  {
    "source": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/🧪️tests/🟦️.ts",
    "classification": "vitest-wrapper-no-literal-identity",
    "reason": "no-unique-content-or-test-identity"
  },
  {
    "source": "🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    "classification": "vitest-wrapper-no-literal-identity",
    "reason": "no-unique-content-or-test-identity"
  },
  {
    "source": "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    "currentPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/vitest.config.ts",
    "classification": "test-wrapper-config-relocated",
    "reason": "testLevelAtLeast is Vitest configuration; relocated to current config, not a canonical test case"
  },
  {
    "source": "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/🧪️tests/🧹️executable-source/🟦️.ts",
    "classification": "no-assertion-or-selftest-marker",
    "reason": "no-unique-content-or-test-identity"
  },
  {
    "source": "🧰️framework/🛍️products/🖥️server/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    "classification": "vitest-wrapper-no-literal-identity",
    "reason": "no-unique-content-or-test-identity"
  },
  {
    "source": "🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    "classification": "no-assertion-or-selftest-marker",
    "reason": "no-unique-content-or-test-identity"
  },
  {
    "source": "🧰️framework/🛍️products/🦑️repo/🧪️tests/🧪️transaction-process-ownership/🟦️.ts",
    "currentPath": "🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/👁️observe/🟦️.ts",
    "classification": "production-source-relocated",
    "reason": "R100 content relocation to current production observer; no canonical-test attribution inferred"
  }
]
```

## Known Noncanonical Relocations

These two baseline paths have concrete current counterparts, but neither is a canonical test destination and therefore neither contributes to the 179 canonical mappings.

```json
[
  {
    "source": "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    "currentPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/vitest.config.ts",
    "classification": "test-wrapper-config-relocated",
    "reason": "testLevelAtLeast is Vitest configuration; relocated to current config, not a canonical test case"
  },
  {
    "source": "🧰️framework/🛍️products/🦑️repo/🧪️tests/🧪️transaction-process-ownership/🟦️.ts",
    "currentPath": "🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/👁️observe/🟦️.ts",
    "classification": "production-source-relocated",
    "reason": "R100 content relocation to current production observer; no canonical-test attribution inferred"
  }
]
```
## Unmatched Guarded Registrations

These 64 current guarded imports originate from a baseline caller with an inline Vitest body but lack a shared baseline/current test identity. They are deliberately not attributed:

```json
[
  {
    "caller": "📜️script.ts",
    "destination": "✏️s/🔌️plugins/🏗️fem/🧪️tests/🔬️tool-job-fem-live-visual-publication/🟦️.ts",
    "reason": "no-shared-vitest-test-identity"
  },
  {
    "caller": "📜️script.ts",
    "destination": "✏️s/🔌️plugins/🏗️fem/🧪️tests/🔬️tool-job-fem-numerical-microcursor/🟦️.ts",
    "reason": "no-shared-vitest-test-identity"
  },
  {
    "caller": "📜️script.ts",
    "destination": "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧪️tests/🔬️cad-presence-retirement/🟦️.ts",
    "reason": "no-shared-vitest-test-identity"
  },
  {
    "caller": "📜️script.ts",
    "destination": "✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🔬️interactivity-puzzle-fill-envelope/🟦️.ts",
    "reason": "no-shared-vitest-test-identity"
  },
  {
    "caller": "📜️script.ts",
    "destination": "✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🔬️interactivity-puzzle-fill-p4e/🟦️.ts",
    "reason": "no-shared-vitest-test-identity"
  },
  {
    "caller": "📜️script.ts",
    "destination": "✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🔬️interactivity-puzzle-fill-preview-json/🟦️.ts",
    "reason": "no-shared-vitest-test-identity"
  },
  {
    "caller": "📜️script.ts",
    "destination": "✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🔬️tool-job-puzzle-reserved-routes/🟦️.ts",
    "reason": "no-shared-vitest-test-identity"
  },
  {
    "caller": "📜️script.ts",
    "destination": "🧰️framework/🔨️modules/⏱️trace/⏱️clock/🧪️tests/🔬️tool-job-telemetry-contention/🟦️.ts",
    "reason": "no-shared-vitest-test-identity"
  },
  {
    "caller": "📜️script.ts",
    "destination": "🧰️framework/🔨️modules/⏳️async/🤝️cooperative/🧪️tests/🔬️tool-job-cooperative-maintenance/🟦️.ts",
    "reason": "no-shared-vitest-test-identity"
  },
  {
    "caller": "🧰️framework/🔨️modules/📡️replication/🟦️.ts",
    "destination": "🧰️framework/🔨️modules/📡️replication/🧪️tests/🧪️document-backbone-envelope-batch/🟦️.ts",
    "reason": "no-shared-vitest-test-identity"
  },
  {
    "caller": "🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/📜️script.ts",
    "destination": "🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🧩️suite/🟦️.ts",
    "reason": "no-shared-vitest-test-identity"
  },
  {
    "caller": "📜️script.ts",
    "destination": "🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️interactivity-mounted-prepared-render/🟦️.ts",
    "reason": "no-shared-vitest-test-identity"
  },
  {
    "caller": "📜️script.ts",
    "destination": "🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️interactivity-mounted-layout-text/🟦️.ts",
    "reason": "no-shared-vitest-test-identity"
  },
  {
    "caller": "📜️script.ts",
    "destination": "🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️interactivity-mounted-surface-lane/🟦️.ts",
    "reason": "no-shared-vitest-test-identity"
  },
  {
    "caller": "📜️script.ts",
    "destination": "🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️interactivity-prepared-raster-producer/🟦️.ts",
    "reason": "no-shared-vitest-test-identity"
  },
  {
    "caller": "📜️script.ts",
    "destination": "🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🔬️window-view-context/🟦️.ts",
    "reason": "no-shared-vitest-test-identity"
  },
  {
    "caller": "📜️script.ts",
    "destination": "🧰️framework/🔨️modules/🛂️manifest/🪟️view-context/🧪️tests/🪟️resolved-host-context/🟦️.ts",
    "reason": "no-shared-vitest-test-identity"
  },
  {
    "caller": "📜️script.ts",
    "destination": "🧰️framework/🔨️modules/🧵️job/⏱️budget/🧪️tests/🔬️tool-job-microsecond-budget/🟦️.ts",
    "reason": "no-shared-vitest-test-identity"
  },
  {
    "caller": "📜️script.ts",
    "destination": "🧰️framework/🔨️modules/🧵️job/🧪️tests/🔬️tool-job-coverage/🟦️.ts",
    "reason": "no-shared-vitest-test-identity"
  },
  {
    "caller": "📜️script.ts",
    "destination": "🧰️framework/🔨️modules/🧵️job/🧪️tests/🔬️tool-job-fixed-operation-registry/🟦️.ts",
    "reason": "no-shared-vitest-test-identity"
  },
  {
    "caller": "📜️script.ts",
    "destination": "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🚚️transport/🧪️tests/🔬️interactivity-mcp-http-transport/🟦️.ts",
    "reason": "no-shared-vitest-test-identity"
  },
  {
    "caller": "📜️script.ts",
    "destination": "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/📑️copy/🧪️tests/🔬️flow-selected-copy/🟦️.ts",
    "reason": "no-shared-vitest-test-identity"
  },
  {
    "caller": "📜️script.ts",
    "destination": "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/🧪️tests/🔬️flow-typed-retirement/🟦️.ts",
    "reason": "no-shared-vitest-test-identity"
  },
  {
    "caller": "📜️script.ts",
    "destination": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️tests/🔬️interactivity-store-sync/🟦️.ts",
    "reason": "no-shared-vitest-test-identity"
  },
  {
    "caller": "📜️script.ts",
    "destination": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️tool-job-artifact-envelope-rejection-transfer/🟦️.ts",
    "reason": "no-shared-vitest-test-identity"
  },
  {
    "caller": "📜️script.ts",
    "destination": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🧪️tests/🔬️canonical-error-progress/🟦️.ts",
    "reason": "no-shared-vitest-test-identity"
  },
  {
    "caller": "📜️script.ts",
    "destination": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🧪️tests/🔬️store-canonical-edit-sealer/🟦️.ts",
    "reason": "no-shared-vitest-test-identity"
  },
  {
    "caller": "📜️script.ts",
    "destination": "🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🗿️artifacts/📖️playbook/🧬️generation/🧪️tests/🔬️procedural-generation-root/🟦️.ts",
    "reason": "no-shared-vitest-test-identity"
  },
  {
    "caller": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts",
    "destination": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🎮️browser-interactive-job-port/🟦️.ts",
    "reason": "no-shared-vitest-test-identity"
  },
  {
    "caller": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts",
    "destination": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/📨️browser-frame-transport/🟦️.ts",
    "reason": "no-shared-vitest-test-identity"
  },
  {
    "caller": "📜️script.ts",
    "destination": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts",
    "reason": "no-shared-vitest-test-identity"
  },
  {
    "caller": "📜️script.ts",
    "destination": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️interactivity-live-reconcile/🟦️.ts",
    "reason": "no-shared-vitest-test-identity"
  },
  {
    "caller": "📜️script.ts",
    "destination": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️interactivity-mounted-engine-surface-lifetime/🟦️.ts",
    "reason": "no-shared-vitest-test-identity"
  },
  {
    "caller": "📜️script.ts",
    "destination": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️interactivity-mounted-frame-transaction/🟦️.ts",
    "reason": "no-shared-vitest-test-identity"
  },
  {
    "caller": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts",
    "destination": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧩️package-integration/🟦️.ts",
    "reason": "no-shared-vitest-test-identity"
  },
  {
    "caller": "📜️script.ts",
    "destination": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🪟️view-context/🧪️tests/🪟️host-opening-context/🟦️.ts",
    "reason": "no-shared-vitest-test-identity"
  },
  {
    "caller": "📜️script.ts",
    "destination": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🧪️tests/🔬️interactivity-shard-executor/🟦️.ts",
    "reason": "no-shared-vitest-test-identity"
  },
  {
    "caller": "📜️script.ts",
    "destination": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-job-drawing-gesture-operation-owner/🟦️.ts",
    "reason": "no-shared-vitest-test-identity"
  },
  {
    "caller": "📜️script.ts",
    "destination": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-job-factory-proof-join/🟦️.ts",
    "reason": "no-shared-vitest-test-identity"
  },
  {
    "caller": "📜️script.ts",
    "destination": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-job-latest-wins/🟦️.ts",
    "reason": "no-shared-vitest-test-identity"
  },
  {
    "caller": "📜️script.ts",
    "destination": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-job-live-fixed-replay/🟦️.ts",
    "reason": "no-shared-vitest-test-identity"
  },
  {
    "caller": "📜️script.ts",
    "destination": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧪️tests/🔬️tool-job-artifact-retained-command/🟦️.ts",
    "reason": "no-shared-vitest-test-identity"
  },
  {
    "caller": "📜️script.ts",
    "destination": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧪️tests/🔬️tool-job-checkpoint/🟦️.ts",
    "reason": "no-shared-vitest-test-identity"
  },
  {
    "caller": "📜️script.ts",
    "destination": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧪️tests/🔬️tool-job-owner-factory-resolution/🟦️.ts",
    "reason": "no-shared-vitest-test-identity"
  },
  {
    "caller": "📜️script.ts",
    "destination": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧪️tests/🔬️tool-job-scalar-config-cohort/🟦️.ts",
    "reason": "no-shared-vitest-test-identity"
  },
  {
    "caller": "📜️script.ts",
    "destination": "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🧪️tests/🔬️interactivity-artifact-submit/🟦️.ts",
    "reason": "no-shared-vitest-test-identity"
  },
  {
    "caller": "📜️script.ts",
    "destination": "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🧪️tests/🔬️interactivity-database-capability-open/🟦️.ts",
    "reason": "no-shared-vitest-test-identity"
  },
  {
    "caller": "📜️script.ts",
    "destination": "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🧪️tests/🔬️interactivity-database-catalog-bootstrap/🟦️.ts",
    "reason": "no-shared-vitest-test-identity"
  },
  {
    "caller": "📜️script.ts",
    "destination": "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🧪️tests/🔬️interactivity-database-catalog-read/🟦️.ts",
    "reason": "no-shared-vitest-test-identity"
  },
  {
    "caller": "📜️script.ts",
    "destination": "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🧪️tests/🔬️interactivity-database-create-catalog/🟦️.ts",
    "reason": "no-shared-vitest-test-identity"
  },
  {
    "caller": "📜️script.ts",
    "destination": "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🧪️tests/🔬️interactivity-db-io-b1-b6/🟦️.ts",
    "reason": "no-shared-vitest-test-identity"
  },
  {
    "caller": "📜️script.ts",
    "destination": "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🧪️tests/🔬️interactivity-db-io/🟦️.ts",
    "reason": "no-shared-vitest-test-identity"
  },
  {
    "caller": "📜️script.ts",
    "destination": "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🧪️tests/🔬️interactivity-artifact-history/🟦️.ts",
    "reason": "no-shared-vitest-test-identity"
  },
  {
    "caller": "📜️script.ts",
    "destination": "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🧪️tests/🔬️interactivity-database-compaction/🟦️.ts",
    "reason": "no-shared-vitest-test-identity"
  },
  {
    "caller": "📜️script.ts",
    "destination": "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🧪️tests/🔬️interactivity-database-sync-hello/🟦️.ts",
    "reason": "no-shared-vitest-test-identity"
  },
  {
    "caller": "📜️script.ts",
    "destination": "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🧪️tests/🔬️interactivity-db-io-caller-migration/🟦️.ts",
    "reason": "no-shared-vitest-test-identity"
  },
  {
    "caller": "📜️script.ts",
    "destination": "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🧪️tests/🔬️interactivity-db-io-direct-writer/🟦️.ts",
    "reason": "no-shared-vitest-test-identity"
  },
  {
    "caller": "📜️script.ts",
    "destination": "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🧪️tests/🔬️interactivity-p1q-r4/🟦️.ts",
    "reason": "no-shared-vitest-test-identity"
  },
  {
    "caller": "📜️script.ts",
    "destination": "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🧪️tests/🔬️interactivity-vcs-bridge/🟦️.ts",
    "reason": "no-shared-vitest-test-identity"
  },
  {
    "caller": "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts",
    "destination": "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️multi-shell-harness/🟦️.tsx",
    "reason": "no-shared-vitest-test-identity"
  },
  {
    "caller": "📜️script.ts",
    "destination": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🧪️tests/🔬️interactivity-all-app-discovery/🟦️.ts",
    "reason": "no-shared-vitest-test-identity"
  },
  {
    "caller": "📜️script.ts",
    "destination": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🧪️tests/🔬️interactivity-runtime-source/🟦️.ts",
    "reason": "no-shared-vitest-test-identity"
  },
  {
    "caller": "📜️script.ts",
    "destination": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧪️tests/🔬️dependency-js-lock-parity/🟦️.ts",
    "reason": "no-shared-vitest-test-identity"
  },
  {
    "caller": "📜️script.ts",
    "destination": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧪️tests/🔬️dependency-truth/🟦️.ts",
    "reason": "no-shared-vitest-test-identity"
  }
]
```

## Method

Baseline files came from `git ls-tree -r 6152f9ca6a0fbb55aa61992077837a230996b51d`; current direct leaves came from `rg --files --no-ignore --hidden` and the canonical leaf pattern `🧪️tests/<case>/<implementation>`. Content evidence uses SHA-256 equality after removing imports, comments, and whitespace. Test identity is the literal label of `describe`, `it`, or `test` calls. The two owner-preserving additions require the complete baseline label set in the same semantic owner and exclude sibling-suite overlap. The eight assertion/export additions require the same semantic owner plus an exact exported function or distinctive assertion identity. Guarded registration evidence is read from `📓️test-layout-runner-map-2026-09-08.md` and confirmed by a current import-specifier search in the caller.
