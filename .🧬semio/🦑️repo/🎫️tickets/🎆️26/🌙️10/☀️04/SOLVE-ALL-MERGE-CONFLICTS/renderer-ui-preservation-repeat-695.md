# Extracted Renderer Presence And Interaction Wiring, Repeat 695

Incoming runtime-scoped presence and typed hovered interaction changes are preserved in the current extracted React hook families. Runtime keys are produced at the exact current state/presence hook, returned through current inferred core types and borrowed by consumers. Pointer cache state flows from Ink session owner to pointer/root consumers. World hover state changes from an ID to the authored InteractionTarget across its current brush, hover and pointer owners; action dispatch uses the actual existing targets builder. Existing core composition and module separation remain. All edited sources parsed with TypeScript createSourceFile; no compiler, tests, build or runtime.

- "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/✏️TextEditor/🧰️surface-state-and-delivery/🟦️.ts"
- "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/✏️TextEditor/👥️session-presence-and-caret/🟦️.ts"
- "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/✏️TextEditor/🟦️.tsx"
- "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🧭️fit-and-presence/🟦️.ts"
- "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx"
- "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🏗️brush-mesh-and-menus/🟦️.ts"
- "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/👆️pointer-and-drop/🟦️.ts"
- "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/👇️hover-and-placement/🟦️.ts"
- "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖋️InkCanvasHost/🧰️ink-session-state/🟦️.ts"
- "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖋️InkCanvasHost/👆️ink-pointer/🟦️.ts"
- "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖋️InkCanvasHost/🟦️.tsx"
- "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖥️Board2dHost/👆️board-pointer-and-keyboard/🟦️.ts"
- "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖥️Board2dHost/🖱️board-context-menu-and-drop/🟦️.ts"
- "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖥️Board2dHost/🟦️.tsx"
