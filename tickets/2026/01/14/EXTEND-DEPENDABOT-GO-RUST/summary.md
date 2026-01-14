# Summary: Extend Dependabot with Go and Rust

Extended the dependabot configuration to include Go and Rust package ecosystems.

## Changes
Added 5 new package ecosystem entries to `.github/dependabot.yml`:
- **Go (gomod)**: `/go/cli`, `/go/mcp`, `/go/repo`, `/go/semio`
- **Rust (cargo)**: `/rs/semio`

All new entries use the weekly schedule consistent with existing configuration.
