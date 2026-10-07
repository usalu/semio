# Current Compiled Descriptor Zero-Touch Route

The existing owner route is `@semio-tech/plugin-registry:rebuild-all`. It dispatches the actual registry `📜️script.ts rebuild-all [--from <step>] [--to <step>]`, takes the queued exclusive `wasm-build` lease, and executes the authored Nx chain. The declared order is provenance, mutation-authority, guest-framework, components, generate, check, activate-s, verify-s, flow-core-bindings, preflight-catalog, publish-catalog. `--to check` prepares and verifies current descriptors and registry without publishing the trusted catalog; `--from components --to check` is valid only when the three input gates have already passed against the same current tree.

The components step runs `bun nx run-many -t describe materialize-dev --exclude @semio-tech/os-plugin-describe-rs --parallel=2`. Every inferred component `describe` target depends on actual `component-dev`. The descriptor compiler reads exactly the owned `<crate>/dist/component-dev/<crate_name>.wasm`, extracts the component core through jco, probes the real component's descriptor, and writes the checked JSON and descriptor pack pair at the component owner root. No static source manifest can substitute for this operation, and no direct `describe component --manifest ...` should bypass its Nx prerequisite.

The concrete Animate component is `🌎️hub/🧩️compositions/🎞️animate/📦️packages/🦀️rust/Cargo.toml`, package `semio-hub-animate`, authored component identity `semio:animate`, project `@semio-tech/animate-plugin`. It is separate from `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation`, which is an artifact implementation without the plugin component manifest. Its minimal owned refresh is `bun nx run @semio-tech/animate-plugin:describe`; the actual target builds component-dev first.

The S variant belongs to plugin `space`, concrete component `🌎️hub/🧩️compositions/🪐️space/📦️packages/🦀️rust`, project `@semio-tech/space-plugin`. The generated TypeScript playground registry names `s` and that source owner. The compiled playground selector requires that same owner's current descriptor. After component describe/materialize and registry generate/check, the existing registry `session s` route creates and stages the real session. Its freshness and identity guards must remain intact.

Current generated diagnostic evidence explicitly withholds Animate because its descriptor app channel is 20 while the host channel is 23. Other rows show the same stale channel. This explains the larger source TypeScript registry and smaller compiled JSON inventory. Refreshing only a session or editing the generated JSON cannot repair those compiled identities. The current S host and its runtime closure need genuine current component builds, descriptor probes, and materialization.

The ordinary dev plugin build route is also available, but its preparation reads the generated compiled projection before building; it cannot supply descriptors for owners the compiled projection already withholds. The independent inferred `describe` targets and registered all-plugin rebuild chain establish the prerequisite correctly.

This audit is read-only. No compiled descriptor, generated registry, module, prepared session, lease, or compiler guard was edited or fabricated. Root coordinates the distribution owner and can dispatch the existing rebuild chain once its input gates and concurrent wasm ownership are accounted for.

Evidence owners:

- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔁️rebuild/🟦️.ts`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔁️rebuild/🔣️.json`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🧭️session/🟦️.ts`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🩺️diagnostics.json`
- `/Users/ueli/Documents/semio/🌎️hub/🧩️compositions/🎞️animate/📦️packages/🦀️rust/Cargo.toml`
- `/Users/ueli/Documents/semio/🌎️hub/🧩️compositions/🎞️animate/📦️packages/🦀️rust/📋️project.json`
- `/Users/ueli/Documents/semio/🌎️hub/🧩️compositions/🪐️space/📦️packages/🦀️rust/📋️project.json`
