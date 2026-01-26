# Plan: Extend Dependabot with Go and Rust

## Objective
Add Go (gomod) and Rust (cargo) package ecosystems to the dependabot configuration.

## Current State
The `.github/dependabot.yml` currently includes:
- npm (3 directories: /js/semio, /js/desktop, /js/docs)
- uv (1 directory: /py/engine)
- nuget (2 directories: /net/Semio, /net/Semio.Grasshopper)

## Directories to Add

### Go (gomod ecosystem)
Found go.mod files in:
- `/go/cli`
- `/go/mcp`
- `/go/repo`
- `/go/semio`

### Rust (cargo ecosystem)
Found Cargo.toml in:
- `/rs/semio`

## Implementation Steps
1. Add 4 gomod entries for each Go module directory
2. Add 1 cargo entry for the Rust crate
3. Use weekly schedule consistent with existing entries

## Changes
Edit `.github/dependabot.yml` to add the new package ecosystem entries.
