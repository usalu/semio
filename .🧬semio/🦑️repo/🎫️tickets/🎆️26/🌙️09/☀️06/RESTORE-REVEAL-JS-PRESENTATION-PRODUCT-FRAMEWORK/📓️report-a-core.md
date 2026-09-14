# Report A — product scaffold, core package, root wiring

Agent A. Scope: `🧰️framework/🛍️products/🎤️presentation` scaffold + `📦️packages/🟦️typescript` + root wiring.
Untouched: `🎯️targets/⚛️react` (B), the projektetage consumer (C), the product-root `🧪️tests/`, `🔮️oracle/`, `.vscode` (D), and everything under `✏️s/`.

## Files created

| Path | What |
|---|---|
| `🧰️framework/🛍️products/🎤️presentation/README.md` | Title, intro, `## Layout` table, `## Commands`, `## Domain model` (the old `🗑️generated/old-framework-124/AGENTS.md` with `# X` → `### X`, `## X` → `#### X`, sentences verbatim). No AGENTS.md was created. |
| `🧰️framework/🛍️products/🎤️presentation/🟦️.ts` | Product barrel, `export * from "./📦️packages/🟦️typescript/🟦️.ts"`. |
| `…/📦️packages/🟦️typescript/🟦️.ts` | The core, copied from `✏️s/🔌️plugins/🎞️animate/🎛️apps/🎬️presentation/⚡️implementations/🟦️typescript/🟦️.ts` (3133 lines). |
| `…/📦️packages/🟦️typescript/🧪️tests/🟦️.ts` | Vitest config (root = package dir, alias `@semio-tech/presentation`, node env, `include` the case suite, `includeSource ["🟦️.ts"]`, `passWithNoTests: false`). |
| `…/📦️packages/🟦️typescript/🧪️tests/🧭️slide-glob-assembly/🟦️.ts` | Ported from the animate `🧪️index.test.ts`; import `../../🟦️.ts`, describe `presentation core slide glob assembly`. |
| `…/📦️packages/🟦️typescript/package.json` | `@semio-tech/presentation` 0.1.0, module, private, `exports["."] = "./🟦️.ts"`, `semio.role framework` / `semio.id presentation`, LGPL-3.0-or-later, `bundleKind library`, devDeps typescript ^5.9.3 + vitest ^4.0.17, `nx.includedScripts []`, `$schema` with 5 × `../`. |
| `…/📦️packages/🟦️typescript/📋️project.json` | `test`, `test-quick`, `test-long`, `test-exhaustive` (`nx:run-commands`, cwd = the emoji package path, `forwardAllArgs`), `namedInputs.default = ["{projectRoot}/**/*"]`. |
| `…/📦️packages/🟦️typescript/📜️script.ts` | `TestScript` → `resolveTestLevel(segments)` then `runVitest(this.root, rest, "🧪️tests/🟦️.ts")`; `ScriptRouter`, `runBundleScriptMain`, default command `test`. |

## Files changed

- `package.json` (root)
  - `workspaces`: added `🧰️framework/🛍️products/🎤️presentation/📦️packages/🟦️typescript` and `…/🎯️targets/⚛️react`, immediately before the `📓️print` entry.
  - `scripts`: `dev:mit-bestand:projektetage` (after `dev:mit-bestand:demonstrator`), `build:mit-bestand:projektetage` (after `build:mit-bestand:demonstrator`), `test:presentation` and `test:presentation:react` (after `test:storybook`).
  - Validated with `bun -e 'JSON.parse(…)'`.
- `🧰️framework/🛍️products/🔣️.json` — added the `🎤️presentation` member after `📓️print`.

## The port and its remaining "animate" mentions

The copy is byte-identical to the animate source except the header docstring, which is now
`/** @emoji 🎤️ \`@semio-tech/presentation\` — render-independent declarative presentation model (reveal.js-oriented morph ids). */`.
`animate-presentation-core` / `@semio-tech/animate-presentation-core` no longer occur.

`grep -n animate` still reports the following — all domain words, **not** package references, so they were left alone:

| Line | Mention |
|---|---|
| 18, 1041, 1078, 1847, 1894, 2419 | reveal.js `auto-animate` / `data-auto-animate-id` — the reveal.js API name. |
| 362 | "animate video exports" — scene clip metadata prose. |
| 818 | a prompt string quoting the reveal.js auto-animate rule. |
| 1989–1996, 2001 | `animatePlayAppDefinition` with `id: "animate"`, `controllerId: "animate"`, `playEntryKind: "animate"`, re-exported as `presentationPlayAppDefinition` — a play-app registration id, not a package name. Renaming it would change the host contract; left for whoever owns the play-app registry. |

## Two bugs found and fixed in the ported core

Both were already broken in the animate source. They were invisible there because the animate vitest
config lists only the renderer files in `includeSource` — the core's own `import.meta.vitest` block was
never collected — and the one file suite it does run fails there too
(`bun ./📜️script.ts test` in `✏️s/🔌️plugins/🎞️animate/📦️packages/🟦️typescript` reproduces it).
The animate plugin was left untouched as instructed; the fixes are in the product copy only.

1. **`🎞️slide` prefix.** `parsePresentationSlideFilePath` and `parsePresentationThoughtFilePath`
   matched `/(?:^|\/)slide\//u`, while `presentationSlideFilePath` / `presentationThoughtFilePath`,
   every docstring, and the real projektetage tree all use `🎞️slide/`. Every glob key therefore
   parsed to `null` and `loadPresentationFromSlideGlob` returned a deck with zero chapters.
   Both regexes now match `🎞️slide/`.

2. **Decorative emoji prefixes on path segments.** Real folders are `🌷️Einführung`, `🪻️Einleitung`,
   `🟣️Gedanke Schweiz`, … and the in-source tests expect the entity name to be the text after the
   emoji. Added and applied in both parsers:

   ```ts
   export function presentationPathSegmentName(segment: string): string {
     const name = segment.replace(/^[\p{Extended_Pictographic}‍️\u{1F3FB}-\u{1F3FF}]+\s*/u, "");
     return name.length > 0 ? name : segment;
   }
   ```

   `\p{Emoji_Component}` was deliberately not used — it includes `0`–`9`, `#` and `*`, which would
   eat leading digits from a legitimate name.

Two stale expectations in the in-source block were corrected to the canonical paths the builders
actually emit: `🎞️slide/Hauptteil/Einführung/Einleitung/Titel.ts` and
`🎞️slide/Hauptteil/Einführung/Einleitung.ts` (the old literals said `slide/…` and `…/🟦️Einleitung.ts`).

**Consequence for Agent C:** chapter, sequence, thought and fallback slide names are now emoji-free,
so the deck's `?chapter=…&sequence=…&thought=…&slide=…` query and the overview labels read
`Einführung`, not `🌷️Einführung`.

## Test output

`bun ./📜️script.ts test <level>` in `🧰️framework/🛍️products/🎤️presentation/📦️packages/🟦️typescript`
(called directly — the nx graph is broken by an unrelated duplicate project):

| Level | Result |
|---|---|
| `fundamental` | 2 files, **56 passed / 56**, 0.60 s |
| `quick` | 2 files, **56 passed / 56**, 0.60 s |
| `long` | 2 files, **56 passed / 56**, 0.79 s |
| `exhaustive` | 2 files, **56 passed / 56**, 1.24 s (v8 coverage on) |

55 of the 56 are the core's in-source cases — collected for the first time — plus the
`🧭️slide-glob-assembly` case suite.

Raw logs: `🗑️generated/core-test-quick-1.txt` (the five original failures),
`core-test-quick-2.txt` (after fix 1), `core-test-quick-3.txt` (green).
