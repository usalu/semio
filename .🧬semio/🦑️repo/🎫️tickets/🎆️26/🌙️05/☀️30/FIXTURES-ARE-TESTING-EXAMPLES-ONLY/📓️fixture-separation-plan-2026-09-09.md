# Fixture and Asset Separation

## Objective

Make every fixture an example used only for testing. Remove fixtures from all test-case folders and place them under fixtures folders at the correct language-neutral semantic scope. Eliminate production dependencies on fixtures; production static files belong under assets. Audit every fixture and all consumers, preserve behavior, update generators/runners/contracts, add permanent enforcement and verify the full repository. Coordinate with Astra extra-high, use Sol extra-high execution agents and Terra extra-high read-only audit agents at maximum useful concurrency. Preserve other concurrent work.

## Current Evidence and Coordination

The current root AGENTS.md was read. The real repository MCP `repo://goals` resource listed all goals; AI-optimized Repo is the appropriate repository-wide owner. This existing Assets Fixtures Separation ticket was reopened through the real MCP with management integration disabled. Prior completion concerned older puzzle/CAD paths and does not prove the current objective. The current worktree is authoritative.

## Completion Requirements

1. Every fixture is testing-only example input or expected output, not a production dependency or a misplaced implementation/contract.
2. No fixture files or fixture subtrees remain inside authored test-case folders.
3. Fixtures live under the nearest shared language-neutral semantic owner; shared scopes follow actual consumers.
4. Production static data is owned under assets; runtime and build consumers do not depend on fixtures.
5. All moved data, imports, includes, runner discovery, schema references, and generators agree.
6. Permanent regression checks enforce the distinctions with neutral fixtures and independent oracles where relevant.
7. Repository-wide audits plus focused actual runtimes prove the final state; unrelated failures are reported without weakening checks.

## Fleet

Initial lanes: Sol execution for framework fixtures, Sol execution for product/plugin fixtures, Terra read-only repository-wide classification and production-edge audit. Coordinator owns schema/policy/runners and integration. Rotate audits after bounded execution lanes finish to use multiple independent Terra auditors across the goal.
