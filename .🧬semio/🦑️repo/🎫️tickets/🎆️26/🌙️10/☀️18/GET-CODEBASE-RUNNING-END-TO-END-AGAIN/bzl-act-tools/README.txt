wrap.ts runs `bun test <files>` under a genuinely issued process envelope (the real runner normally supplies it):
  SEMIO_WORKSPACE_ROOT="$PWD" SEMIO_TEST_ARTIFACT_DIR="<ticket>/🗑️generated/x" bun <this dir>/wrap.ts ./<test file>
Plain `bun test` fails the envelope-bound tests with "Original parent Script process invocation required".
