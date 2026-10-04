# Plugin IO Terminal Five Port Review

Read-only refreshed source and current compiler-log snapshot,2026-10-03. No source/Cargo execution. JSON cold-retirement candidate is still design-only; no lifecycle proof is inferred.

Root's five measured plugin consumers now use the actual IoError cause at declared message terminals: reactor jobs line567 moves `error.cause.into_message()` into its job fault; plugin root route faults lines41979/41989 borrow cause.message; snapshot rejection lines41980/41990 move cause.into_message and retain the actual diagnostics through encode_diagnostics. Successful export/import diagnostics remain unchanged. No blanket From<String>, default cause or refusal-kind message classifier was introduced in these inspected edits. This retains the existing terminal wire/Fault shape while correctly accessing the new internal cause structure; it does not claim that those message-only terminals expose typed kind.

At inspection `🗑️generated/root-authentic-tiff19-typed-plugin-terminal-paired-current.log` contained626957 bytes with no located current error or could-not-compile summary. Its tail was ordinary plugin warnings, so the process was still pending; zero current error lines is not a completed Native19 result.
