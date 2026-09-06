# Browser Actor Public Integration

September 6 continuation. The full end-to-end goal remains active. This slice
binds metadata, not executable actor delivery or an operational GIS frontend.

## Implemented Boundary

Directory plan and execution lease records now require `browserActor` in their
canonical schema, TypeScript and Rust twins. Plan identity is path-free; lease
identity adds only a bounded byte count. Both parsers bind the source component
and descriptor digests to their selected package and enforce the renderer:
closed actors require Wasm, while React/WGPU require explicit `none`.

The shared projectors reject an absent/invalid closed actor length and a length
attached to `none`. Rust now returns a Result, and all native call sites were
migrated. Shared equality includes the complete actor identity and ordered
import interfaces, not a subset of actor hashes.

Hub retained plan authority copies only the verified catalog selection. Plan
exchange and socket revalidation compare the actor too. The manifest derives
its actor length from the retained catalog byte owner; neither the request nor
the previous plan supplies that length. Native socket authority retains the
plan actor. The browser Worker compares it before retaining component or
descriptor bodies. Its inference test fixture now parses a complete lease
instead of casting a partial object to the full public type.

All three neutral fixture families were migrated. The independent plan,
browser-open and execution-lease oracles include actor admission/equality.
AJV also validates the canonical Directory plan and lease definitions and
rejects a missing actor field. The existing three asset routes remain the
entire body allowlist. No actor body route, child Worker, activation, rendering,
or AI approval is introduced by this patch.

## Executed Evidence

- Source 9967 was RED at the production plan parser's required-field boundary
  after the new closed actor fixture was added and before implementation.
- Source 20587 was GREEN: metadata49, private catalog26, body8, raw length8,
  complete lease48 field substitutions, 12 byte vectors, 11 lifecycle vectors.
- Browser 75781 was GREEN: three actual Worker-module Vitest laws, 273 skipped,
  with fake HTTP responses. It verifies/rejects the neutral closed-Wasm lease
  and preserves the explicit renderer-unavailable result. It is not a live
  browser-to-Hub-to-GIS process journey.
- Source 46448 was GREEN after adding both canonical schema checks and the
  generic plan/browser-open parity oracles (20 plan mutations, 5 exchange
  mutations, 25 browser-open hostile rows, 71 execution-lease hostile rows).
- Browser-module25581 also passed15 GIS inference-port and generic document-open
  tests (261 skipped), exercising the complete parsed inference lease fixture.
- Rust parsing/formatting and scoped diff whitespace checks passed.
- Native identity 89974/mFFJtE and 53460/b6ILiw each passed replication group00
  (13 laws) but kernel group01 failed to compile at the concurrently evolving
  recovery test. These runs do not qualify the new native actor integration.
  Sol repaired the first nonexistent `current()` cohort; the next enum cohort
  was reported directly to its owner. The registered kernel group now contains
  four exact laws, including full plan and native socket lease integration.

## Authenticated HTTP Coverage Added

Terra's current audit found no metadata bypass but a composed-test gap: the
Hub HTTP test catalog previously always returned no actor body. Its existing
authenticated asset route law now includes a closed Wasm selection with a
retained synthetic three-byte body. It checks the exact public actor identity,
verified-owner-derived length, plan propagation, secret/path exclusion, denial
of closed-without-body and none-with-body, and an invented actor route.

This new HTTP row is not yet executed. Root native 92576 runs the existing
catalog target, now five Hub library laws followed by this one binary HTTP
law. The earlier native49777/Gt5a6S was BUILD RED in VCS dependencies; the Home
agent has repaired and source-qualified that dependency cohort. Synthetic
`abc` bytes are never GIS execution evidence.

## Concurrent DB and UI State

Mount 67216/CemxLK built and discovered current laws, then failed law0 after
30.09 seconds at Database shutdown. Diagnostic rerun 65755/D9sh1b passed
laws0–13, then failed law14, the new resolved-but-unconsumed catalog result
drain test, at `terminal_is_empty()`. The previous timeout did not reproduce;
that is not proof it is fixed. Sol owns the diagnosis and consuming exact-three
Store/DB witness bridge. Terra is auditing that cross-crate authority split.

Home native retries continue in the separate public-member cache. Current
source51 covers the repaired async fixture cohort; native group02 remains
unqualified. Source-green fixtures are not a functioning Home UI claim.

## Remaining Full-Goal Work

Qualify current native actor catalog and HTTP laws, close the DB shutdown and
recovery boundary, and build the first-party child Worker owner before actor
delivery. Real GIS component materialization, contained activation, scoped
effects, the concrete per-document fixed-three approval committer, two-user
collaboration/admin revocation and actual frontend acceptance remain required.
The current Map inference service is deterministic native inference, not an
external-model request. Original P4 acceptance also requires the authenticated
`semio-os-mcp` client path: discovery, cold shared Map inspection, typed inference,
approval, progress/cancel/undo, and observation in the shell and a second
collaborator. A Shell proposal panel alone does not satisfy it. No goal or
ticket completion is claimed.
