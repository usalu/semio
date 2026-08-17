# Terra UI Accordion Zero-Active-Consumer Dissolution Acceptance

## Baseline And Source Checkpoint

- Baseline HEAD: `5a1367dfcc90630c52dc2ec4de9526babe8d70f4`.
- The clean component fingerprint matched `b1be5b04a682aa3f40eb245bc07a36a721e9c19cdf844b3fc98496157ae55d81`.
- The clean exclusive-story fingerprint matched `3d8b211775aa46966203aca0725b3b963d3e92c3b12aade2896f2e1a16a615af`.
- Deleted only `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪗️Accordion/🟦️component.tsx` and `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪗️Accordion/🧪️story.tsx`.
- The `🪗️Accordion` directory now has zero authored files. No wrapper, alias, compatibility export, substitute component, dependency, lockfile, generated output, or Storybook configuration was added or changed by this lease.

## Serialized Registrar Integration

- Before the source checkpoint, the coordinator-owned React index was externally modified but hash-stable at `01005e76dbc844cbaa2e9c8b2e6b7727bfd3d575f7ef887e62c3f1ce249c4a52`; Terra did not edit it.
- The coordinator then exclusively removed the Radix Accordion primitive import and the complete Accordion import/re-export region. Its accepted post-registrar SHA-256 was `1ae126cc1dd3f5a47c201ca9af485397205d3d8b3cc48e40dd8c902de9cf5f29`.
- During the final gates, the coordinator's separately documented DiagramNode registrar advanced that shared index to `6efa99283af7df14639d1f301456690d3d16860156ed8d2bf1087094a2bfc2fc`. Its final targeted scan still has zero `AccordionPrimitive`, `Accordion`, `AccordionContent`, `AccordionItem`, `AccordionTrigger`, and `🪗️Accordion` matches.

## Final Static And Diff Evidence

- Active-scope scans excluded tickets/history, generated and build output, dependencies, `compose`, `🌎️hub`, `♻️mit-bestand`, and legacy/exempt taxonomy paths. The identifier, direct-path import, and JSX-consumer scans each produced zero results (`rg` exit `1`, the expected no-match status).
- Scoped ordinary `git diff --check` over the two deleted sources and shared React index exited `0` with no output.
- Scoped cached `git diff --check` over the same paths exited `0` with no output; the cached lease diff is empty.

## Registered Nx Gates

| Target | Exit | Classification |
| --- | ---: | --- |
| `bun nx run @semio-tech/ui-react:lint --skip-nx-cache` | 0 | Passed. |
| `bun nx run @semio-tech/ui-react:typecheck --skip-nx-cache` | 1 | Documented broad framework/UI API drift: missing plugin and statechart symbols, unresolved manifest/UI types, generated declarations, translation mismatches, and unrelated React index type errors. No Accordion-family diagnostic occurred. |
| `bun nx run @semio-tech/ui-react:test-quick --skip-nx-cache` | 1 | External concurrent UI registrar state: the test observed a stale `DiagramNode` source import in the shared React index. The subsequently serialized DiagramNode registrar removed that import; this lease neither owns DiagramNode nor has an Accordion-family diagnostic. |
| `bun nx run @semio-tech/ui-react:build --skip-nx-cache` | 1 | Documented broad Storybook integration drift: `.storybook/stories/ui/🌳OntologyTree.stories.tsx` cannot resolve `@semio-tech/coda-desktop/renderer`, before any Accordion-related diagnostic. |

The static evidence establishes no Accordion lease regression. The nonzero gates are external broad UI drift or a concurrent registrar handoff and were not repaired here.
