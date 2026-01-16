# Log: Fix CRLF Line Endings in post-create.sh

## 2026-01-15

### Analysis
The devcontainer post-create script is failing with `$'\r': command not found` errors. This is a classic symptom of Windows-style CRLF line endings in a shell script that's being executed in a Linux environment.

### Fix Applied
1. Rewrote `.devcontainer/post-create.sh` with Unix-style LF line endings
2. Created `.gitattributes` to enforce LF endings for shell scripts and prevent future occurrences
3. Also removed emojis from echo statements for cleaner output
