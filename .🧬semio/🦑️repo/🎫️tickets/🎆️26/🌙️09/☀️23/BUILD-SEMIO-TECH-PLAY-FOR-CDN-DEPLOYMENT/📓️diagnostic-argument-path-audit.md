# Diagnostic Argument Path Audit

Real pinned Nx exec run on 2026-10-07T00:54:23.283Z.

Status: PASSED.

A retained script under a literal directory with spaces received all neutral fixture arguments exactly, including POSIX and Windows-style workspace strings and emoji. The existing third-party string-argv parser independently returned the same arguments from the installed Nx serializer's quoted form.

Observed runtime:

```json
{
  "platform": "darwin",
  "script": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT/🔎️arguments/📂️path with spaces/📜️script.ts",
  "cwd": "/Users/ueli/Documents/semio",
  "project": "workspace",
  "arguments": [
    "/Users/Example User/Work Projects/semio",
    "C:\\Users\\Example User\\Work Projects\\semio",
    "emoji 🧪️ argument with spaces"
  ]
}
```

The pinned Nx 23.2.0 exec implementation wraps each parsed argument in double quotes before execSync, preserving ordinary spaces. It does not escape embedded double quotes; the earlier inline-code quoting failure is consistent with that source. Current diagnostic commands pass filenames and plain commands, not inline code. No spaced-path defect was demonstrated and no launch rewrite is required. Native Windows execution was not available: the Windows path argument round-trip ran on macOS and does not claim a cmd.exe runtime test. The quoted Windows filename pattern is supported by the inspected serializer, but that platform remains an execution boundary.

The browser-capabilities probe can derive workspace from its own import.meta.dirname just as the other probes do, reducing one argument. This is an optional simplification; its existing quoted workspace argument has no demonstrated space failure.
