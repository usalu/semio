# Log

## Investigation

Explored the monorepo structure:
- Found 5 vite.config files in js/ folder: semio, vscode, play, sketchpad, temp
- Found 1 vitest.config.ts at root
- Only 2 have actual test configurations:
  - Root: `vitest.config.ts` → tests `repo.tests.ts`
  - `js/semio/vite.config.ts` → tests `semio.test.ts`

The Vitest extension is detecting all vite.config files as potential projects, hence the warning.

## Solution

In Vitest v4, the workspace configuration changed. The `test.workspace` option was removed and replaced with `test.projects`.

Updated `vitest.config.ts` to use the new `test.projects` configuration:

```typescript
export default defineConfig({
  test: {
    projects: [
      {
        test: {
          name: "repo",
          include: ["repo.tests.ts"],
          testTimeout: 60000,
        },
      },
      "./js/semio/vite.config.ts",
    ],
  },
});
```

This explicitly defines the projects with vitest tests, preventing the extension from detecting all vite.config files as separate projects.

Also added `name: "semio"` to `js/semio/vite.config.ts` test configuration for proper project identification.

### Verification

Both projects are now properly recognized:
- `npx vitest run --project repo --passWithNoTests` → works
- `npx vitest run --project semio --passWithNoTests` → works
- `npx vitest run --passWithNoTests` → runs tests from both projects
