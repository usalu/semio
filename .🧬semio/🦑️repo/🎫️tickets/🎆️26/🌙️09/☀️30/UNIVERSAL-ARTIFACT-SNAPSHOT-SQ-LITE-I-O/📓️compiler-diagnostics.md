# Captured Compiler Diagnostics

The registered Nextest runner captures compiler output and used to replay only stderr lines beginning with an error header or `Caused by:`. Human rustc diagnostics therefore lost source locations, highlighted expressions, field notes and the cause body. This hid actionable prerequisite errors while SQLite provider tests were compiling.

A retained language-neutral Cargo diagnostic fixture defines the complete expected human diagnostic. Its workspace contract also invokes the independent system rustc on a deliberately invalid owned record access, then checks that the captured text retains the actual source span and equals the original diagnostic block. No temporary script is created.

The initial registered Nx filter (session67655) executed the two existing structured JSON laws successfully and failed the new human diagnostic law exactly on the missing span and cause body. The implementation now retains complete human stderr from its first error/cause when no structured compiler error is available. Structured JSON diagnostics preserve the existing warning filtering behavior.

The first verification retry (session93414) stopped at project graph construction during concurrent Puzzle, renderer and writer test-host changes, before running the three tests. After the shared graph settled, fresh registered session88169 passed all three filtered laws with eight assertions, including the independent rustc invocation. No result from a blocked invocation was counted as an executed test.
