# Plan

## 1. Analysis
- Analyze `js/compose/sketchpad/Sketchpad.tsx` to understand current toolbar rendering.
- Analyze how apps register toolbar sections (panels).
- Analyze `js/compose/sketchpad/Home.tsx`, `Kit.tsx`, `Design.tsx`, `Type.tsx` to see how they currently use the toolbar/tools.

## 2. Refactoring `Sketchpad.tsx` Toolbar Rendering
- Remove the existing `Toolbar` component or refactor it completely to match the "center-split" requirement.
- Implement the "Tool Bar" (left) and "Tool Settings Panel" (right) concept.
- The `Toolbar` needs to be a fixed `h-medium` or `h-large` band at the bottom.
- Ensure strict centering.

## 3. Tool State Management
- We need a clear "Active Tool" state.
- `Selection`, `Filter`, `Create` should be the top-level tools.
- `Filter` and `Create` might need to be promoted to "tools" if they aren't already, or mapped to existing concepts. Currently `Filter` and `Create` are often just actions or panels.
- For `Design` and `Type` apps, `Selection` is a tool. `Filter` and `Create` are usually on the `Home` or `Kit` apps.

## 4. App-Specific Adaptations
- **Home App:** `Filter` (search/sort), `Create` (new kit/import).
- **Kit App:** `Filter` (search/kinds), `Create` (new type/design/etc), `Selection`.
- **Design App:** `Selection` (modes), `Create` (pieces?), `Filter` (layers?).
- **Type App:** `Selection`, `Create` (connectors?), `Filter`?
- **Quality App:** `Selection`?

## 5. Implementation Details
- **Selection Tool:** Upward dropdown for modes.
- **Settings Panel:** Dynamic content based on active tool.
- **Styling:** No animations. Flat band.

## 6. Execution
- Modify `Sketchpad.tsx` to implement the new layout logic.
- Update app configurations/registrations to provide the correct "Tool Settings" content.
- Ensure "No animations".
