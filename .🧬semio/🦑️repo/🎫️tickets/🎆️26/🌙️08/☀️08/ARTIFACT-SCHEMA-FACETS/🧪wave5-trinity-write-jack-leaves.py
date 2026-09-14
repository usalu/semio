#!/usr/bin/env python3
from pathlib import Path
import json

JACK = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack")
ART = JACK / "🧬️schema"
DIFF = JACK / "🔺️diff" / "🧬️schema"
ART.mkdir(parents=True, exist_ok=True)
DIFF.mkdir(parents=True, exist_ok=True)

art_rs = Path(__file__).resolve().parent
