# Coordinator Phase 10 Mandated Orchestration Boundary

Date: 2026-08-23
Owner: `/root` coordinator
Verdict: the plan's literal Bun/Nx removal is superseded by repository instructions; no false zero-dependency claim is permitted.

## Instruction conflict

The attached plan's Phase 10 text requires removing “Nx/Bun-orchestration” and its final gate counts
all third-party build/test/tooling packages. The workspace `AGENTS.md` has higher-priority mandatory
requirements:

- Bun **must** be the package manager;
- Nx **must** be the task runner;
- `project.json` targets **must** call the local `📜️script.ts` command surface; and
- `package.json` scripts **must** call Nx.

Removing or bypassing Bun/Nx would violate the repository contract. The coordinator and executor
fleet must therefore preserve that boundary even though the attached plan asks for its removal.

## Enforceable Phase 10 interpretation

All other third-party UI, codegen, documentation, test, bundling, linting, release, browser-driving,
and native-host dependencies remain in scope for owned replacements and deletion. Bun and Nx are
treated as a mandated repository toolchain boundary, not as a completed owned replacement. Their
manifest and lock identities remain visible in every dependency audit; they must never be hidden,
allowlisted as zero, or omitted from totals.

The final dependency report must publish both counts:

1. the literal third-party count, including every mandated Bun/Nx package identity; and
2. the removable-boundary count after excluding only the explicitly mandated Bun/Nx toolchain.

Phase 10 can be declared complete only as “all permitted removals complete; mandated Bun/Nx
boundary retained.” It cannot truthfully claim the attached plan's literal zero across build and
tooling unless the workspace instruction is explicitly changed by the developer. No agent is
authorized to edit `AGENTS.md`.

## Operational consequences

- Permanent scripts continue to live only in the appropriate `📜️script.ts`.
- All task execution continues through Bun and Nx as required.
- The dependency freeze must reject any identity other than the accepted current ratchet and must
  not use the mandate as permission for additional packages.
- No Phase 9/10 or master ticket closes while unrelated runtime/UI dependency identities remain.
- This precedence decision changes no current accepted dependency count: the live boundary remains
  63 Rust plus 66 JavaScript identities until a separately audited removal lands.

This report records a hard instruction boundary; it does not waive any other Phase 9 or Phase 10
gate and is not a completion claim.
