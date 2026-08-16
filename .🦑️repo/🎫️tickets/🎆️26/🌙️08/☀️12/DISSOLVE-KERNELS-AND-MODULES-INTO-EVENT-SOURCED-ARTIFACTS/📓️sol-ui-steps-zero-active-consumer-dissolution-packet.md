# UI Steps Zero-Active-Consumer Dissolution Packet

## Scope and Evidence

The `Steps` UI component has no active production consumer. Its only production-looking package consumer is under `compose`, which taxonomy excludes structurally. The remaining active references are package assembly and stories; neither counts toward the production consumer minimum.

Current clean fingerprints:

- `🐾️Steps/🟦️component.tsx`: `1f78cd7e97337707c8abcb5be5602e87a6314ca34a562016ecb8361932307c7c`;
- `🐾️Steps/🧪️story.tsx`: `5ef52b52061f2f68aaa1fe1456faa845d3a2589bee7ff1f6b4d3cbee9722f259`;
- `↕️Collapsible/🧪️story.tsx`: `f855a47474d00745cd02f00b6f50bcda64bcf146dab6eb2599e0e10b648906f8`;
- React package index: `c3b144495c317d83c5a9911e0fca0568732ac33bacef87ccbad2920be15eed22`.

Delete the zero-consumer component and its exclusive story. In the Collapsible story, remove the `Steps` import and replace its example-only wrapper with the equivalent semantic ordered-list markup so the remaining Collapsible story stays valid. Do not touch `compose` because it is excluded; no compatibility export is retained.

The Sol coordinator exclusively owns the shared React package index and will remove only its adjacent `Steps` import/export pair after the Terra source patch lands. Terra must rehash that index after the coordinator signal and must not edit it.

## Writable Paths

Terra may write only:

- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🐾️Steps/🟦️component.tsx` (delete);
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🐾️Steps/🧪️story.tsx` (delete);
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/↕️Collapsible/🧪️story.tsx`;
- unique `📓️terra-ui-steps-zero-active-consumer-dissolution-acceptance.md`.

## Validation

Use `apply_patch` only and no modifying Git command. Verify active-scope references are zero after the coordinator registrar update, while explicitly classifying any excluded `compose` reference. Run:

```text
bun nx run @semio-tech/ui-react:lint --skip-nx-cache
bun nx run @semio-tech/framework:test-quick --skip-nx-cache
```

Also run scoped ordinary/cached `git diff --check`. Record source-complete and green separately; do not start Cargo directly.
