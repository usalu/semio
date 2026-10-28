DOC = """🦀 Staged batch `consumption` (lane td-misc): the Cargo workspace-selection law wrote its scratch workspaces into a hard-coded folder of the 10-13 ticket (`.🧬semio/.../☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/🗑️generated`), which exists on this one checkout only (a mirror, a fresh clone, CI: ENOENT, and the rejection closes the shared Original so the next law fails too); it now uses the suite's artifact directory, else the system temporary directory, materialized (the selection owner refuses symlinked roots such as macOS `/var`)."""
P = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🎚️mode/🦀️profile/🧪️tests/📥️consumption/🟦️.ts"
EDITS = [
 (P, [
  ('import{mkdtempSync,mkdirSync,writeFileSync,rmSync,readFileSync}from"node:fs";\n', 'import{mkdtempSync,mkdirSync,writeFileSync,rmSync,readFileSync,realpathSync}from"node:fs";\nimport{tmpdir}from"node:os";\n'),
  ('const sink=resolve(process.cwd(),".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/🗑️generated");', 'const sink=realpathSync(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir());'),
 ]),
]
