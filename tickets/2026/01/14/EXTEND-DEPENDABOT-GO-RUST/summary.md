# Summary: Extend Dependabot with Go and Rust

## Changes
Extended the dependabot configuration with:
- **Go (gomod)**: `/go/cli`, `/go/mcp`, `/go/repo`, `/go/semio`
- **Rust (cargo)**: `/rs/semio`

## Follow-up: Stale PRs

The PRs for `System.Collections.Immutable`, `Grasshopper`, `System.Drawing.Common`, and `System.Resources.Extensions` are **stale** - they were created before the ignore rules were added to the config.

**Action required**: Manually close these PRs on GitHub. Dependabot does not automatically close existing PRs when ignore rules are added. Future updates for these packages will be ignored.
