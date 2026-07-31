# Projektetage Eingabeprozess Manuelles Prüfen Slide

Added slides to thought **Eingabeprozess** (Bauteilportal → Systematik):

- **Manuelles Prüfen** — `eingabeprozess-formular.png`
- **Output** — rich-rendered `eingabeprozess-output.json` via new `json` embodiment

## Framework

- `JsonEmbodiment` in `@semio-tech/framework-presentation-core`
- `json.ts` tree renderer + `JsonMorphView` in React renderer

## Files

- `mit-bestand/präsentation/33.projektetage/slide/Bauteilportal/Systematik/Eingabeprozess/ManuellesPrüfen.ts`
- `mit-bestand/präsentation/33.projektetage/slide/Bauteilportal/Systematik/Eingabeprozess/Output.ts`
- `mit-bestand/präsentation/33.projektetage/index.ts`
- `framework/product/presentation/core/index.ts`
- `framework/product/presentation/renderer/react/json.ts`
- `framework/product/presentation/renderer/react/index.tsx`
- `framework/product/presentation/renderer/react/globals.css`
- `framework/product/presentation/renderer/react/vitest.config.ts`

## Verification

```bash
cd .repo/🎫️/26/06/16/PROJEKTETAGE-EINGABEPROZESS-MANUELLES-PR-FEN-SLIDE
bunx vitest run --config vitest.config.ts -t "output json|loads every slide"
cd framework/product/presentation/renderer/react
bun ./📜️script.ts test -- json.ts
```
