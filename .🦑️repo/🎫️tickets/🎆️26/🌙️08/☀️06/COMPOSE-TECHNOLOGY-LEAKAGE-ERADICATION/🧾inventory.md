# Compose leakage inventory (path/package coupling)

## Delete (compose-only surfaces outside ./compose)
- `.storybook/compose/**`
- `.storybook/stories/compose/**`
- `framework/modules/assets/🏛️compose/**` (compose brand icons)
- compose-named logos/images under framework assets (brand leak)

## Edit (remove compose roots/aliases/stubs)
- `.storybook/main.ts` — compose js/fixture aliases
- `.storybook/scopes.ts` — compose scopes + aliases
- `.storybook/globals.css` — @source compose
- `framework/.../styling/.../script.ts` — scan roots + net palette out to compose
- `framework/.../styling/.../vite-elements-assets.ts` — sketchpad stubs
- `framework/.../ui/.../script.ts` — compose refs
- `framework/products/os/component.ts` — compose refs
- `s/.../reasoning/...wires.dsl.semio` — compose/fixture path
- repo fixtures path comments `compose/asset/...` → framework-relative

## Keep
- `s/plugins/puzzle/.../🌉️compose` domain composition engine (not ./compose tech)
- `./compose/**` itself (legacy island)
- repo skip/exempt entries that isolate compose (not integrate)
