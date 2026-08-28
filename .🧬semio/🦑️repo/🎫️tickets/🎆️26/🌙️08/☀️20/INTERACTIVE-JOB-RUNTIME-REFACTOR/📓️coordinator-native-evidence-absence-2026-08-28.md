# Native Evidence Absence — 2026-08-28

At 2026-08-27T22:03:39Z (2026-08-28 local), exact read-only checks confirm three raw log paths are absent. The coordinator executed no deletion, relocation, cleanup, restoration or modifying Git command. No cause or recovery location is established.

## Actual Check

```text
ls: /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-plugin-awaited-checkpoint-green-r1-2026-08-27.txt: No such file or directory
ls: /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-plugin-awaited-idempotent-green-r1-2026-08-27.txt: No such file or directory
ls: /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-plugin-patchtracker-exhaustive-r6-2026-08-27.txt: No such file or directory
-rw-r--r--@ 1 ueli  staff   6791 Aug 28 00:00 /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/📓️plugin-awaited-completion-r1-native-2026-08-27.md
-rw-r--r--@ 1 ueli  staff  34646 Aug 27 23:58 /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/📓️plugin-patchtracker-exhaustive-r6-native-2026-08-27.md
-rw-r--r--@ 1 ueli  staff   2758 Aug 27 23:59 /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-plugin-awaited-generated-green-r1-2026-08-27.txt
```

The R6 raw log was successfully read earlier in this same continuation: tool357cae returned its actual test output and footer `30 tests run:29 passed,1 failed,492 skipped`,0.646s, followed by Nx failure. A later exact read returned ENOENT and the ls check above confirmed it. This is not a guessed path or a source-only claim. Its sibling Markdown report survives and retains copied execution output.

The awaited-completion report currently contains literal `cat: ... No such file or directory` instead of raw output for checkpoint and direct idempotent registration. The root has read the report but not those two missing raw footers. Their SIGABRT/1PASS outcomes remain executor-reported until an original retained artifact or actual tool result is reviewed. The generated registration output survives and was read:1PASS,521unselected,0.016s, Nx success. Do not label placeholder stderr as native evidence.

The compiler owner has been notified and asked to retain any original nextest artifacts/tool results, clearly mark the missing logs, and not recreate the originals. Disjoint source work and scoped compilation continue. No source is restored or changed in response to this absence.

