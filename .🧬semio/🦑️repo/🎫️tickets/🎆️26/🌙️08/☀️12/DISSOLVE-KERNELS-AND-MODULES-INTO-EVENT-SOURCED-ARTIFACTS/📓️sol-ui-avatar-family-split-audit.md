# UI Avatar Family Split Audit

## Baseline

- Broad Avatar component SHA-256: `c60c0476d145d7bcc465281e511d666ad7ffea0550feabb077cc2a9e6e5d3289`, clean.
- Broad Avatar story SHA-256: `2c2cf8d6a7c2b74fa1edf924d7f2a9c63e48644841c7f1ff1b1d15a6eacf06bc`, clean.
- HistoryTable consumer SHA-256: `3b8e2828fe9ce02dfc7f19c51696bf9ffc6bd4006414660e57074e3eeb405c49`, clean.
- VirtualFileSystem consumer SHA-256: `2778d5472f243ba79fa8ce19488f26b86660184d1143f78eb6f4d3677ca73590`, clean.
- DragAndDrop story SHA-256: `0f1cc63ccd3b030672830cf811cce74cc29b8fd40de430d66e3867507463f531`, clean.
- React index at audit time: `f4415689af8fadf41714bde7b4bc7181169804a7b878ee25411791ec8d5abf59`.

## Responsibility Split

The broad Avatar file combines three responsibilities that must be evaluated separately:

1. Base Radix Avatar/Image/Fallback primitives: no independent production consumer; they only implement variants in the same file.
2. DraggableAvatar: zero production consumers; only its exclusive Avatar story and a DragAndDrop example use it.
3. TableAvatar: two independent active production consumers, HistoryTable and VirtualFileSystem. Table stories are example-only.

## Decision

Delete DraggableAvatar and its contract. Keep the base primitives private inside a specifically named `TableAvatar` component and stop exporting them. Move the qualifying TableAvatar implementation/contract/story to `elements/📻️TableAvatar`, update both direct production imports, and keep the DragAndDrop story by replacing its decorative DraggableAvatar with local non-component markup. The shared React package exports only `TableAvatar` and `TableAvatarProps` from the new semantic path. Radix Avatar remains a valid dependency because the qualifying TableAvatar implementation uses it.
