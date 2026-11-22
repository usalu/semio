"""📦 Shared roots for the native folder reopen stages once the channel, receiver and hydration patches are installed."""
from pathlib import Path

TICKET = Path(__file__).resolve().parent
ROOT = next(parent for parent in TICKET.parents if (parent / "Cargo.toml").exists())
CHANNEL = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel"
OUT = TICKET / "🗑️generated" / "large-reopen"
OUT.mkdir(parents=True, exist_ok=True)
