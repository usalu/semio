# Archives completed .cursor/plans into existing ticket folders (topic-matched).
$ErrorActionPreference = "Stop"
$root = git -C $PSScriptRoot rev-parse --show-toplevel 2>$null
if ([string]::IsNullOrWhiteSpace($root)) { throw "git rev-parse failed; run from a clone" }
$root = (Resolve-Path -LiteralPath $root).Path
$plans = Join-Path $root ".cursor\plans"
$repoDir = Join-Path $root ".repo"
$ticketRoot = Get-ChildItem -LiteralPath $repoDir -Directory -ErrorAction Stop |
  Where-Object { Test-Path -LiteralPath (Join-Path $_.FullName "26") } | Select-Object -First 1
if (-not $ticketRoot) { throw "No ticket year folder under a direct child of $repoDir" }
$ticketRoot = $ticketRoot.FullName

$map = [ordered]@{
  "typesafe_compose_boundary_bad64f75.plan.md" = "26\04\26\TYPESAFE-COMPOSE-JS-STORES"
  "typesafe_js_stores_dd2ab6f5.plan.md" = "26\04\26\TYPESAFE-COMPOSE-JS-STORES"
  "per-entity_js_stores_01e22c55.plan.md" = "26\04\26\TYPESAFE-COMPOSE-JS-STORES"
  "entity_stores_refactor_1b45145a.plan.md" = "26\04\26\ENTITY-STORE-REFACTOR"
  "kit_command_scopes_b02844b8.plan.md" = "26\04\26\KIT-COMMAND-SCOPE-REFACTOR"
  "typed_graphql_schema_b0ab4db1.plan.md" = "26\04\26\ADD-COMPOSE-GRAPHQL-SCHEMA-BUILD-COMMAND"
  "react_js_store_boundary_f503c535.plan.md" = "26\04\26\REACT-JS-STORE-BOUNDARY-REFACTOR"
  "sketchpad_store_refactor_2f0da985.plan.md" = "26\04\26\SKETCHPAD-STATE-STORE-REFACTOR"
  "strict_compose_layering_refactor_205dc73c.plan.md" = "26\04\26\SKETCHPAD-LAYERING-SHELL"
  "repo-cli-loc-command_c6535d9f.plan.md" = "26\04\27\LOC-COMMAND-CATEGORIES-SCANNER-AND-CONTRIBUTOR-FILTER"
  "single-async-kitstore-export_0b8c7452.plan.md" = "26\04\26\SINGLE-ASYNC-KITSTORE-EXPORT-IN-COMPOSE-JS"
  "single-async-kitstore-export_ad4841bd.plan.md" = "26\04\26\SINGLE-ASYNC-KITSTORE-EXPORT-IN-COMPOSE-JS"
  "kit_hook_refactor_c3b7b4a7.plan.md" = "26\04\26\SKETCHPAD-KIT-HOOK-REFACTOR"
  "compose_layer_parity_f8c77d13.plan.md" = "26\04\26\TYPESAFE-COMPOSE-JS-STORES"
  "semantic_kit_diffs_62cc83e2.plan.md" = "26\04\25\ADD-KIT-CHANGE-DIFF-INVERSION-FUNCTIONS"
  "semantic-kit-commands_6d7919dd.plan.md" = "26\04\25\SEMANTIC-KIT-COMMAND-EVENTS"
  "graphql_kit_control_5d73e8e5.plan.md" = "26\04\25\KIT-GRAPHQL-CATALOG-ONLY"
  "graphql_kit_control_plane_afc09280.plan.md" = "26\04\25\GRAPHQL-COMMAND-COLLAPSE"
  "kit_data_single-source-of-truth_refactor_392d08c9.plan.md" = "26\04\25\KIT-DATA-SSOT-HOOK-READ"
  "sketchpad_kit_data_hooks-only_5e48b49b.plan.md" = "26\04\25\KIT-DATA-SSOT-HOOK-READ"
  "rs_read_command_overhaul_011f30b6.plan.md" = "26\04\26\SCOPED-KIT-READ-REFACTOR"
  "align_js_react_algorithms_00e8a761.plan.md" = "26\04\26\REMOVE-NATIVE-ALGORITHM-ADAPTERS"
  "kit_store_backbone_generalization_fe75d494.plan.md" = "26\04\26\CLEAN-STATELESS-KIT-STORES-AND-KIT-COMMAND-REQUESTS"
  "kit_vcs_schema_consolidation_d9574154.plan.md" = "26\04\23\IMPLEMENT-KIT-VCS-POSTGRES-SCHEMA"
  "rust_kit_canonical_schema_00b97d36.plan.md" = "26\04\25\UNIFY-GRAPH-QL-MUTATIONS-AND-REMOVE-JSON-SCALARS"
  "compose_store_rust_sidecar_7d31cf17.plan.md" = "26\04\26\COMPOSE-STORE-GRAPHQL-API"
  "commands_return_diffs,_central_apply_08e68b1c.plan.md" = "26\04\25\REMOVE-DUPLICATED-RUN-WRAPPERS-ON-CHANGE-COMMANDS-IN-COMPOSE-RS"
  "kit_store_story_0a521ca8.plan.md" = "26\04\23\KIT-STORE-GIT-KRAKEN-STYLE-HISTORY-WINDOW"
  "granular_kit_change_commands_07c2a9cc.plan.md" = "26\04\25\GRAPH-QL-SEMANTIC-COMMAND-SHELL"
  "kit_vcs_command_document_cdecfba5.plan.md" = "26\04\23\KIT-STORE-GIT-KRAKEN-STYLE-HISTORY-WINDOW"
  "finish_sketchpad_rust_migration_v2_84802df9.plan.md" = "26\04\22\FINISH-RUST-STORE-HOOK-MIGRATION"
  "finish_sketchpad_rust_migration_2d46733d.plan.md" = "26\04\22\FINISH-RUST-STORE-HOOK-MIGRATION"
  "sketchpad_full_rust_migration_76777353.plan.md" = "26\04\22\FINISH-RUST-STORE-HOOK-MIGRATION"
  "kit_version_control_layer_b17090d2.plan.md" = "26\04\23\IMPLEMENT-KIT-VCS-POSTGRES-SCHEMA"
  "sketchpad_kit-state_removal_c891deb3.plan.md" = "26\04\26\SKETCHPAD-STATE-STORE-REFACTOR"
  "rust_worker_hook_pipeline_52b2f61e.plan.md" = "26\04\20\CREATE-COMPOSE-REACT-BUNDLE-WITH-RUST-WORKER-STORE"
  "compose-rs_oo_refactor_8871ebc2.plan.md" = "26\04\21\CONSOLIDATE-COMPOSE-RS-INTO-LIB-FILE"
  "store_suffix_and_4-tier_dtos_1ced1b2f.plan.md" = "26\04\26\TYPESAFE-COMPOSE-JS-STORES"
  "compose-rs_oo_pointer_rewrite_bba7ee9f.plan.md" = "26\04\21\CONSOLIDATE-COMPOSE-RS-INTO-LIB-FILE"
  "compose-rs_oo_refactor_9ada65a0.plan.md" = "26\04\21\CONSOLIDATE-COMPOSE-RS-INTO-LIB-FILE"
}

foreach ($name in $map.Keys) {
  $src = Join-Path $plans $name
  if (-not (Test-Path -LiteralPath $src)) {
    Write-Warning "Missing source: $src"
    continue
  }
  $rel = $map[$name]
  $destDir = Join-Path $ticketRoot $rel
  if (-not (Test-Path -LiteralPath $destDir)) {
    throw "Destination ticket folder not found: $destDir"
  }
  $dest = Join-Path $destDir $name
  if (Test-Path -LiteralPath $dest) {
    throw "Refusing to overwrite: $dest"
  }
  Move-Item -LiteralPath $src -Destination $dest
  Write-Host "Moved $name -> $rel"
}

$left = Get-ChildItem -LiteralPath $plans -File -ErrorAction SilentlyContinue
if ($left.Count -gt 0) {
  throw "Plans remain under .cursor/plans: $($left.Name -join ', ')"
}
Write-Host "All plan files relocated."
