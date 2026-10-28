from pathlib import Path
base = Path("C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-OPTIMIZATION-CLIP-EXAMPLES")
p = base / "api.md"
s = p.read_text(encoding="utf-8")
a = s.index("```python"); b = s.index("```", a + 5) + 3
p.write_text(s[:a] + (base / "model-tools" / "api_model_block.txt").read_text(encoding="utf-8").rstrip() + s[b:], encoding="utf-8")
