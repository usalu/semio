---
name: nx-monorepo-setup
overview: Bring the existing npm workspace and Nx task graph into a consistent, validated shape for the monorepo. The work will keep the current package-based Nx model, fix workspace drift, standardize project metadata and command wiring, and verify the graph plus key build/test targets.
todos:
 - id: open-ticket
   content: Open an implementation ticket under the appropriate repo goal before non-markdown edits.
   status: completed
 - id: normalize-workspaces
   content: Fix root npm workspace membership and regenerate the lockfile.
   status: completed
 - id: tighten-nx-json
   content: Update Nx workspace-level defaults, named inputs, and layout metadata.
   status: completed
 - id: standardize-projects
   content: Normalize Nx metadata and targets in existing package and project manifests.
   status: completed
 - id: validate-graph
   content: Run Nx discovery and representative targets, then fix any graph or target errors.
   status: completed
 - id: close-ticket
   content: Close the repo ticket with files changed and validation summary.
   status: completed
isProject: false
---

# Setup Nx Properly for the Monorepo

## Current Findings

- The repo uses npm workspaces with Nx 21, not pnpm or Yarn, and most Nx projects are inferred from package manifests in [package.json](package.json).
- [nx.json](nx.json) only has generic target defaults and the Python plugin; there are no named inputs, default base, or workspace layout hints.
- The workspace list has drift: [package.json](package.json) still includes missing `semio/studio`, while existing Nx-marked packages like [semio/store/package.json](semio/store/package.json) and [semio/assets/images/package.json](semio/assets/images/package.json) are not listed as root workspaces.
- Some package metadata is inconsistent, including schema paths and sourceRoot values in [semio/assets/package.json](semio/assets/package.json), [semio/store/package.json](semio/store/package.json), and related package manifests.
- Root scripts and VS Code launch commands mix npm package names and path-like Nx project ids, which makes the graph harder to reason about and easier to break.

## Implementation Plan

- Open a new repo ticket under the AI-optimized Repo goal before editing, because no open ticket currently covers Nx setup directly.
- Normalize root workspace membership in [package.json](package.json): remove stale entries, add missing package roots that are intended Nx/npm projects, and regenerate [package-lock.json](package-lock.json) with npm so the lockfile matches.
- Tighten [nx.json](nx.json): add stable named inputs, defaultBase, workspace layout, and shared target defaults that cache build/test/lint/update/publish while keeping dev uncached.
- Standardize package-level Nx metadata across package manifests that already opt into Nx: fix incorrect `$schema` paths, sourceRoot values, projectType where clearly wrong, and target declarations that duplicate scripts incorrectly.
- Standardize root scripts and launch wiring to use canonical Nx project ids consistently, preserving the existing dev/build/publish entry points used by [.vscode/launch.json](.vscode/launch.json).
- Add or repair explicit `project.json` targets only for non-package projects such as [semio/graphql/project.json](semio/graphql/project.json), keeping package-based projects in package manifests.

## Validation Plan

- Run npm lockfile/install validation after workspace changes.
- Run Nx graph/project discovery checks such as `npx nx show projects` and targeted `npx nx show project ...` for representative JS, Rust, Go, Python, .NET, and schema projects.
- Run representative targets instead of the whole monorepo first: `semio/graphql` build, core JS/react/sketchpad checks, repo client/server builds, and at least one Storybook-backed UI build.
- Fix any project graph, missing target, or stale workspace errors found during validation, then close the ticket with the exact files changed and commands run.
