# UI Band Zero-Active-Consumer Dissolution Packet

## Verdict

`Band` has zero active production consumers. The React barrel is assembly, and the only other active-scope references are stories. No `Band` registration exists in `🔣️components.json`; Storybook uses a generic story glob. Excluded `compose` and legacy areas provide no active consumer and remain untouched.

## Baseline

- Band component: `430517eaf3df5afa7c7eab1d0226ccac8f035b02b19d69b3911310b622f9c2bc`, clean.
- Band story: `f40d7e201aa5171a7505f6b7c7ee3ef5878d50d8a8cc917959d1c297d9d37b16`, clean.
- Strip story: `5eeb15ae9ac0a6fe61db324148461724e7fa175ed7b24e75c0a2235b743f27c6`, clean.
- React index: `f6936957c8044acaa7af426e671d9a9fe83491ca2c2b4146c9b6a242e77c1aa2`, dirty only from the accepted Steps registrar deletion.

## Terra Lease

Writable paths:

- delete `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎗️Band/🟦️component.tsx`;
- delete its `🧪️story.tsx`; the file's extra Strip examples are redundant examples and do not justify keeping a dead-component story;
- in `🎟️Strip/🧪️story.tsx`, remove the `Band` import and the `BandDefault` story region, preserving every Strip story;
- unique `📓️terra-ui-band-zero-active-consumer-dissolution-acceptance.md`.

The Sol coordinator exclusively owns the shared React index. After Terra's source checkpoint, the coordinator will rehash and remove only the adjacent Band import/export region, preserving Steps and ordered Card registrar work.

## Validation

Use `apply_patch` only and no modifying Git commands. After coordinator signal, verify zero active-scope Band symbol/path/import/JSX references and classify excluded areas separately. Run JS-only Nx targets as registered and available:

```text
bun nx run @semio-tech/ui-react:lint --skip-nx-cache
bun nx run @semio-tech/ui-react:typecheck --skip-nx-cache
bun nx run @semio-tech/ui-react:test-quick --skip-nx-cache
bun nx run @semio-tech/ui-react:build --skip-nx-cache
```

Do not invent missing targets. Record each actual target result, story/static evidence, and scoped ordinary/cached diff checks.
