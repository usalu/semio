import subprocess
import sys

files = [
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🎟️reservation/🟦️.ts",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🎟️reservation/🧪️tests/🟦️.ts",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🎟️reservation/🧪️tests/🔣️.json",
    ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️31/RESOLVE-CACHE-CHARGE-RESERVATION-CONFLICTS/resolution.md",
]
command = [
    "repo",
    "ticket",
    "close",
    "--year",
    "26",
    "--month",
    "10",
    "--day",
    "31",
    "--slug",
    "RESOLVE-CACHE-CHARGE-RESERVATION-CONFLICTS",
    "--summary",
    "Owner admission stays outside the receipt lease, and the charge receipt directory is flushed through the portable native metadata sync.",
]
for path in files:
    command.extend(["--files", path])
completed = subprocess.run(command, check=False)
sys.exit(completed.returncode)
