# Energy and Demonstrator Example Test Repair

The Energy and Demonstrator demo tests each imported a nonexistent `🧪️artifact.ts` helper. Their DSL paths already resolved to the exact current assets. The existing package routers merely printed success without executing either test.

Each existing `📜️script.ts` test route now invokes its one retained example test explicitly through Bun. The first uncached Nx runs both failed on the missing import. Exact one-line imports now use `bun:test`; neither asset nor test assertion changed. Fresh uncached `@semio-tech/energy-js:test` and `@semio-tech/demonstrator-js:test` runs each pass one test and one assertion (52 ms and 62 ms respectively). No native engine result is implied.

Files changed are each plugin's `📦️packages/🟦️typescript/📜️script.ts` and its existing `📚️examples/🎬️demo/🧪️tests/🟦️.ts` under Energy's `🔋️model` or Demonstrator's `🎪️playground`, standard 1 / any subset. Existing launch targets are registered exactly once in both the live and seed launch files; both parse as JSONC. There is no generated rename script, fixture regeneration, runtime dependency or Git mutation.
