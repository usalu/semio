# Phase 10 Owned PDF Canvas Port

<!-- #region Outcome -->

## Outcome

Packet 3 is implemented. The Animate presentation renderer no longer imports or declares `react-pdf`. It now loads the already-retained `pdfjs-dist` implementation through workspace-owned structural contracts and renders each page into an owned canvas.

The owned lifecycle cancels an active render before page cleanup, disposes the document or unresolved loading task on source changes and unmount, rejects documents from superseded loads, and repeats page cancellation/cleanup on page, scale, and zoom changes. The canvas retains the existing cover/scroll sizing and page-navigation behavior while exposing an accessible image label, polite loading status, and alert error status.

<!-- #endregion Outcome -->

<!-- #region Files -->

## Intentional Files

- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📺️renderer/⚛️react/🔨️modules/🔌️pdf-canvas-port/🟦️component.ts`: owned contracts, resource owner, accessible status/bitmap helpers, and four focused inline tests.
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📺️renderer/⚛️react/🟦️component.tsx`: PDF.js adapter, canvas view, cancellation/disposal wiring, owned exports, and migrated selectors in existing navigation/sizing tests.
- `✏️s/🔌️plugins/🎞️animate/📦️packages/🟦️typescript/package.json`: replaced `react-pdf` with direct `pdfjs-dist`.
- `✏️s/🔌️plugins/🎞️animate/📦️packages/🟦️typescript/🟦️vitest.setup.ts`: replaced the React-PDF mock with the retained PDF.js implementation mock.
- `✏️s/🔌️plugins/🎞️animate/📦️packages/🟦️typescript/🧪️vitest.config.ts`: registered the owned port as inline-test source.
- `♻️mit-bestand/🎤️präsentation/📅️33.projektetage/🎨️globals.css`: direct consumer gate repair, changing only its stale UI stylesheet import to the live owned stylesheet.
- `bun.lock`: Bun resolution after the manifest replacement; this shared lock also contains concurrent dependency packets.
- This report.

The concurrent removal of `@types/reveal.js` visible in the Animate manifest diff is not part of this packet.

<!-- #endregion Files -->

<!-- #region Verification -->

## Verification

- `bun install --ignore-scripts`: pass; lockfile saved and `react-pdf` removed.
- Focused owned-port Vitest selection: pass, 1 file and 4 tests.
  - render cancellation → page cleanup → document disposal ordering;
  - unresolved-load cancellation and stale-document rejection;
  - loading/ready/error accessibility;
  - device-pixel bitmap sizing.
- `node_modules/.bin/eslint <owned-pdf-port>`: pass.
- `bun build <presentation-renderer> --outdir /tmp/semio-pdf-packet-build --external '*'`: pass; TSX syntax/transformation completed.
- `bun nx run @semio-tech/animate-js:test-quick --skip-nx-cache`: pass, but the current `quick` level filter collects zero test files.
- `bun ./📜️script.ts verify dependencies`: pass at 164 current identities, 74 removed from baseline; `js:react-pdf` is explicitly in the removed set.
- `bun ./📜️script.ts verify dependencies parity js`: pass; `manifests=83 external-rows=287 evidenced=138 unowned=149 undeclared-imports=0`.
- Targeted `git diff --check`: pass.
- Source and lock census: zero `react-pdf` rows/imports/mocks under Animate and `bun.lock`.

<!-- #endregion Verification -->

<!-- #region Blockers -->

## Honest Blockers

- The full Animate test target cannot collect the presentation renderer because `🧪️vitest.config.ts` points `@semio-tech/animate-present-core` at the missing path `✏️s/🔌️plugins/🎞️animate/🎛️apps/🎬️present/⚡️implementations/🟦️typescript/📦️index.ts`. This precedes the PDF packet and is also the remaining consumer-build blocker.
- The first consumer build stopped on a stale UI stylesheet import. The authorized one-line import repair moved the build past CSS generation. The rerun then stopped on the same missing Animate present-core alias target before reaching PDF code.
- Whole-file ESLint reports pre-existing unused-variable findings in the large renderer and two pre-existing no-unused-expression findings in its setup polyfill. The new owned port passes isolated ESLint.

<!-- #endregion Blockers -->

<!-- #region FormatterIncident -->

## Formatter Incident

The command `bun ./📜️script.ts format check --help` was intended as help discovery but the router treated it as the whole-repository format command and applied broad formatting before completing in about eight seconds. This was reported immediately to the coordinating agent; no git restore or other destructive reconciliation was attempted because the worktree is shared.

Other exact formatter probes were:

- `bunx biome check <renderer> <setup> <package>`: exited successfully with no diagnostics while Bun resolved packages and saved the shared lock.
- `node_modules/.bin/biome check --write ...`: failed immediately because that binary does not exist.
- `node_modules/.bin/prettier --write <renderer> <setup> <package>`: completed; renderer 513 ms, setup 13 ms, package unchanged. This preceded the coordinator's instruction to run no further formatting commands.

<!-- #endregion FormatterIncident -->
