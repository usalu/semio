# [DEBUG] Verification

- `bun nx run @semio-tech/ui-react:test-quick -- --run --testNamePattern=Popover`: 1 passed.
- `bun nx run @semio-tech/ui-react:test-quick -- --run --testNamePattern=tabBarHost`: 2 passed.
- `bun nx run @semio-tech/ui-react:test-quick -- --run --testNamePattern=select.menus`: 1 passed.
- Full `@semio-tech/ui-react:test-quick`: 471 passed, 9 pre-existing unrelated failures (unchanged baseline: gumball mock, icon catalog, CanvasPickMenu event mock, window pane icon, four tree assertions, driver cleanup).
- `@semio-tech/ui-react:typecheck`: blocked by existing missing generated modules and unrelated type errors.
- `@semio-tech/ui-react:lint` on `📦index.tsx`: blocked by the existing missing `react-hooks/exhaustive-deps` rule definition at line 8031.
- In-app browser computed-style proof was unavailable because browser policy blocks inline local test pages; the temporary blank tab was closed.
