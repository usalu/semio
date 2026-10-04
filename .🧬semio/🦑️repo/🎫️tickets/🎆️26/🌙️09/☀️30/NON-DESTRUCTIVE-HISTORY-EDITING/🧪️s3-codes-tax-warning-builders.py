"""🔤️ S3-CODES-TAX (C-3): one level spelling in the API as on the wire — renames the retired `warn` outcome builders to
`warning` (`MutationMessage::warn` → `MutationMessage::warning`, the chainable `.warn(` → `.warning(`, doc paths
`MutationOutcome::warn` → `MutationOutcome::warning`) in every git-visible Rust source outside `.🧬semio/` and in this
ticket's Rust-emitting generator scripts. Idempotent; `--dry-run` prints the census without writing. The two `warn`
definitions in `📡️replication/🎮️mutation/🦀️.rs` are the only `warn` methods in the repository (checked 2026-10-02), so
every `.warn(` call outside an embedded `console.warn(` is an outcome builder."""
import pathlib, re, subprocess, sys

REPO = pathlib.Path("/Users/ueli/Documents/semio")
TICKET = REPO / ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING"
RULES = [(re.compile(r"\bMutationMessage::warn\b"), "MutationMessage::warning"), (re.compile(r"\bMutationOutcome::warn\b"), "MutationOutcome::warning"), (re.compile(r"(?<!console)\.warn(?=\s*\()"), ".warning")]

listed = subprocess.run(["git", "ls-files", "-co", "--exclude-standard", "-z", "--", "*.rs"], cwd=REPO, capture_output=True, check=True).stdout.decode().split("\0")
paths = [REPO / p for p in listed if p and not p.startswith(".🧬semio/")] + sorted(path for path in TICKET.glob("🧪️*.py") if path.name != pathlib.Path(__file__).name)
dry = "--dry-run" in sys.argv
files = sites = 0
for path in paths:
    if not path.is_file() or path.is_symlink():
        continue
    text = path.read_text(encoding="utf-8")
    if "warn" not in text:
        continue
    updated, count = text, 0
    for pattern, replacement in RULES:
        updated, n = pattern.subn(replacement, updated)
        count += n
    if count:
        files += 1
        sites += count
        if not dry:
            path.write_text(updated, encoding="utf-8")
print(f"[s3-codes-tax] {'would rewrite' if dry else 'rewrote'} {sites} site(s) in {files} file(s)")
