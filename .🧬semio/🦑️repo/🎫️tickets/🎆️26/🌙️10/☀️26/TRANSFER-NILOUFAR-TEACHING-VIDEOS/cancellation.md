# Owned Process-Tree Cancellation and CLI Encoding

## Fix

run_command now isolates POSIX commands into a new session and terminates only that owned process group, first with SIGTERM and then SIGKILL for any remaining descendants. On Windows, cancellation invokes hidden system taskkill /PID <the owned Popen PID> /T /F, covering virtual-environment redirectors, the real Python interpreter, and their descendants. The original interruption is re-raised after cleanup. Successful completion still returns None, and nonzero completion still raises CalledProcessError with the original command and exit code.

CLI main configures both stdout and stderr as UTF-8 where reconfigure is available. Imported library code does not change host stream encodings. Existing stdout configuration already made --help pass under hostile cp1252 before this change; the reproduced remaining defect was stderr escaping the non-ASCII invalid video value instead of emitting UTF-8. Both output paths now retain the actual characters.

## Test-Driven Evidence

Language-neutral cases were added to video_runtime.tests.json and consumed by test_video_runtime.py. The process fixture launches a dedicated child and grandchild and atomically records their actual OS process IDs under a temporary ticket-generated directory. A controlled KeyboardInterrupt occurs only after both are confirmed alive. The baseline left the grandchild running ([False, True]); fixture cleanup removed that owned process. After the fix, both processes were confirmed stopped ([False, False]). The test never enumerates or signals actual rendering jobs or unrelated processes.

The hostile-encoding regression runs the real standalone CLI with PYTHONIOENCODING=cp1252 and PYTHONUTF8=0. It checks --help success and a non-ASCII --video error with the normal exit code 2, verifying UTF-8 characters and absence of UnicodeEncodeError. The baseline error-output case failed, then passed after configuring stderr alongside stdout.

The complete runtime suite passed 10 tests with actual Manim discovery enabled in 9.018 seconds. A follow-up return-status test passed both success (0) and failure (7) cases in 0.230 seconds. A final combined run is recorded below after completion. The curriculum contract additionally checks that the five series use five distinct intros, consuming the existing intro_count fixture field.

The real process-tree test ran on the current native Windows host. The same test has a POSIX path for future native macOS/Linux and devcontainer execution; those hosts were not available for an actual process test in this session.

## Files

- 🎓️teaching/video_runtime.py
- 🎓️teaching/render_videos.py
- 🎓️teaching/test_video_runtime.py
- 🎓️teaching/video_runtime.tests.json

All generated fixture metadata and temporary test directories stay under the ticket's generated output root. No authored scene changed, no actual rendering process was stopped, and this agent owns no long-running render.

## Final Combined Verification

The final combined suite completed successfully: all 11 runtime tests passed in 7.522 seconds with actual Manim discovery enabled. The repo test command exited 0, and all owned child/grandchild fixtures and test-runner processes had completed before handoff.
