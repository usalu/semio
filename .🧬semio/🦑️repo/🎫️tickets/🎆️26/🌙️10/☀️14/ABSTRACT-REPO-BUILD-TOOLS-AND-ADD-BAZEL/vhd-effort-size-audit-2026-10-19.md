# VHD / Native Image Storage: Size and Effort Evidence

Measured read-only on 2026-10-19. These are current working-tree size metrics, not historical time records. No honest person-hour figure can be inferred from line counts. Concurrent edits can change totals after this snapshot.

## Measured Scope

Base: `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage`.

| Scope | Files | Nonblank lines |
|---|---:|---:|
| Image-focused lifecycle subtotal | 93 | 9,411 |
| Broader storage support / publication subtotal | 37 | 14,109 |
| Shared root tests and schema | 10 | 2,388 |
| Root storage implementation | 1 | 182 |
| Entire bounded native storage subtree | 141 | 26,090 |

The image-focused subtotal includes fixed, platform, macOS/Linux/Windows image backends, birth, cleanup and mapping. It is cross-platform native-image work, not Windows-VHD-only effort. Its 93 files comprise **26 implementation files / 1,411 nonblank lines**, **18 schema/data files / 3,134 lines**, and **49 test-path files / 4,866 lines**. Test-path counts include fixtures and schemas beneath test directories.

The broader subtotal is not a claim that all 37 files can survive unchanged. Publication and metadata often encode mounted-storage assumptions and must be reviewed. Conversely, deleting the entire 141-file subtree would overstate VHD-specific removal because journals, resource accounting, ownership and publication mechanics are reusable concepts.

## Per-directory Counts

| Directory | Files | Nonblank lines |
|---|---:|---:|
| fixed | 35 | 5,155 |
| platform | 14 | 791 |
| macOS image | 11 | 383 |
| Linux image | 1 | 7 |
| Windows image | 1 | 7 |
| birth | 4 | 617 |
| cleanup | 26 | 2,422 |
| mapping | 1 | 29 |
| publication | 26 | 13,691 |
| measurement | 4 | 28 |
| journal | 1 | 74 |
| borrower | 1 | 47 |
| port | 1 | 22 |
| metadata | 4 | 247 |
| shared root tests | 9 | 380 |
| shared root schema | 1 | 2,008 |

The seven-line platform image entry files delegate substantial behavior to shared fixed/platform code. They do not mean Windows integration cost seven lines. Also, many implementation definitions occupy one long line, while schemas are expanded over many lines; LOC is especially unsuitable as a complexity or time estimate here.

## What This Says About Effort

This is substantial infrastructure with considerable schema and test investment, not a tiny VHD utility. The current scope does not establish elapsed engineering time, contributor time, prior discarded designs or debugging effort. It also omits integration consumers elsewhere in repository caching, dependency provisioning, Nx/Bazel orchestration, general filesystem descriptor code and older ticket research.

Removing mandatory images is therefore an architecture change, not deleting a file or a few mount calls. A coherent change must separate cooperative directory budgets from strict enforcing-backend capabilities; update setup and task admission; retain publication, cancellation and concurrent ownership behavior; rework backend-specific tests; and validate Windows/macOS/Linux/devcontainer/CI routes. Existing live images require a deliberate data-preservation decision. Directory accounting must never be represented as an equivalent hard physical quota.

Recommended scope for estimation: first stop requiring images for normal setup/build while preserving a separately selected strict backend. Deleting every strict implementation immediately would discard more existing work and broaden the change. A responsible calendar estimate requires a bounded design and a consumer/call-site inventory; this audit intentionally provides no invented person-hour conversion.

## Reproduction

PowerShell, from C:/git/semio:

```powershell
$base = '🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage'
Get-ChildItem -LiteralPath $base -Directory | ForEach-Object {
  $files = @(rg --files -- $_.FullName)
  $lines = ($files | ForEach-Object {
    @(Get-Content -LiteralPath $_ | Where-Object { $_.Trim().Length -gt 0 }).Count
  } | Measure-Object -Sum).Sum
  [pscustomobject]@{ Directory = $_.Name; Files = $files.Count; Nonblank = $lines }
}
```

Classification uses test-path membership first (`🧪️tests`), then `.json` as schema/data, then implementation extensions `.ts/.rs/.go/.cs/.py/.mjs/.cjs`. Image-focused directories: `🧱️fixed`, `🌐️platform`, `🍎️image`, `🐧️image`, `🪟️image`, `🌱️birth`, `🧹️cleanup`, `🔒️mapping`. Broader directories: `📏️measurement`, `📜️journal`, `📤️publication`, `📥️borrower`, `🔌️port`, `🧾️metadata`. Root schema/tests are reported separately. `rg --files` uses its ordinary ignore rules; no global cache traversal was performed.

No code edits, modifying Git operations, tests or output files outside this report were produced.
