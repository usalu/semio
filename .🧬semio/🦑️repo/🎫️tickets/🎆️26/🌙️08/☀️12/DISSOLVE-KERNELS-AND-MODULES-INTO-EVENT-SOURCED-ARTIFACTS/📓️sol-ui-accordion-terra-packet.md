# Terra Packet UI-Accordion-01: Zero-Consumer Dissolution

## Preconditions

- Baseline HEAD: `5a1367dfcc90630c52dc2ec4de9526babe8d70f4`
- Read root and applicable nested `AGENTS.md` before editing.
- Use no modifying Git command. Use `apply_patch` only.
- Rehash and verify the two Terra-owned source files are clean and match:
  - component: `b1be5b04a682aa3f40eb245bc07a36a721e9c19cdf844b3fc98496157ae55d81`
  - story: `3d8b211775aa46966203aca0725b3b963d3e92c3b12aade2896f2e1a16a615af`
- Verify the coordinator-owned React index remains at the announced serialized hash before the source checkpoint. Do not edit it.

## Terra Writable Paths

1. `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪗️Accordion/🟦️component.tsx`
2. `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪗️Accordion/🧪️story.tsx`
3. One unique ticket acceptance record named `📓️terra-ui-accordion-zero-active-consumer-dissolution-acceptance.md`.

## Coordinator Registrar Request

After Terra deletes both source files and reports their absence, the coordinator alone will remove from the shared React index:

- `import * as AccordionPrimitive from "@radix-ui/react-accordion";`
- the complete `// #region 🛒️Accordion` import/re-export region through its matching end marker.

Terra must wait for the coordinator's new index hash before final scans and gates.

## Required Result

- Delete the zero-consumer component and its exclusive story.
- Do not introduce a wrapper, alias, compatibility export, substitute component, or module.
- Do not touch dependencies, lockfiles, generated census output, Storybook configuration, other UI leaves, or the shared index.

## Validation

After registrar signal:

1. Confirm the Accordion directory has no authored files.
2. Active-scope stale scans for the four exported identifiers, direct path imports, and JSX consumers; exclude tickets/history, generated outputs, dependencies, build outputs, `compose`, hub, mit-bestand, and taxonomy legacy/exempt areas.
3. Scoped ordinary and cached `git diff --check` without mutating the index.
4. `bun nx run @semio-tech/ui-react:lint --skip-nx-cache`
5. `bun nx run @semio-tech/ui-react:typecheck --skip-nx-cache`
6. `bun nx run @semio-tech/ui-react:test-quick --skip-nx-cache`
7. `bun nx run @semio-tech/ui-react:build --skip-nx-cache`

Record exact exit codes and distinguish lease regressions from the already documented broad UI drift. Do not fix unrelated failures.
