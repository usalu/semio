"""🧭️ Shared path vocabulary of the pm-seam tools (standard library only)."""
import os
HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.dirname(HERE)
ROOT = os.path.abspath(os.path.join(C, *[".."] * 7))
LIBRARY = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
BUILD = LIBRARY + "/🏗️build"
DEPENDENCIES = LIBRARY + "/⚡️caching/🚀️bootstrap/📦️dependencies"
MANAGERS = DEPENDENCIES + "/🔌️managers"
NEW = os.path.join(HERE, "staged", "new")
