# Artifact I/O Boundaries and Mutation Separation

## Objective

Artifact schema owns semantic snapshots, diffs, mutations, and inferences. Artifact I/O owns binary protocols, text/DSL grammars, serializers, and deserializers. Representations cannot be domain mutations or children of semantic schema.

## Architecture

- Native bidirectional codecs: `🚪️io/{💾️binary,📝️text}/{📸️snapshot,🔺️diff,🧬️mutations,💡️inferences}/[semantic-member]`.
- Foreign codecs remain under import/deserializers and export/serializers.
- Semantic modules contain no codec aliases, codec implementations, or representation metadata.
- Publication, ownership, replay, and store behavior are separate from wire conversion.
- Existing framework I/O contracts provide limits, cancellation, and resource handling; no new runtime dependencies or compatibility shapes.

## Fleet

Four concurrent slots: primary coordinator plus three workers. Read-only GPT 6.1 Sol Light/low exploration completed first; GPT 6.1 Sol High execution handles relocation, infrastructure enforcement, and runtime extraction. Requested primary setting is GPT 6.1 Sol Extra High. Repo MCP's older model allowlist requires `codex` in ticket metadata; this report preserves the actual requested model settings.

## Work Boundaries

1. Relocation: codec paths, module ownership, language imports, fixture and configuration references.
2. Infrastructure: taxonomy, discovery, authoring, builders, architectural policies, language-agnostic contracts and executable gates.
3. Runtime: inline codec extraction, generation3d publication/host separation, runtime regression tests.
4. Coordination: shared contracts, integration checks, independent audits, ticket completion.

## Validation

Record failing ownership cases before edits; run language-agnostic fixture contracts with third-party validation. Verify physical paths and Rust/TypeScript module ownership, then run appropriate artifact runtime and source checks. Preserve exact command outcomes in Markdown reports. Generated logs are temporary and removed before ticket closure. Concurrent unrelated edits remain intact; no modifying Git commands or worktrees.

## Infrastructure

Read `repo://goals` through the configured repo MCP stdio entry point before reopening the existing I/O ticket. A failed reopen renamed the ticket folder before rejecting its model metadata; the successful reopen uses the resulting canonical path `26/08/10/ARTIFACT-I-O-BOUNDARIES-AND-MUTATION-SEPARATION`.
