from pathlib import Path

HERE = Path(__file__).resolve().parent
target = Path("C:/git/semio/🎓️teaching/🏛️architecture/📊️excel/🎬️clip/⚡️energy/🐍️.py")
text = target.read_text(encoding="utf-8")
start = text.index("    def present_energy")
end = text.index("    def _stack")
target.write_text(text[:start] + (HERE / "present_new.txt").read_text(encoding="utf-8") + text[end:], encoding="utf-8")
print("replaced")
