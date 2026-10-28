from pathlib import Path

T = Path(__file__).parent
path = Path("C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🧪️tests/🐍️.py")
raw = path.read_bytes().decode("utf-8")
crlf = "\r\n" in raw
text = raw.replace("\r\n", "\n")
addition = (T / "model-r3-tests-add.txt").read_text(encoding="utf-8")


def swap(old, new):
    global text
    assert text.count(old) == 1, old[:70]
    text = text.replace(old, new)


swap('''"solar_winter", "solar_summer", "shading_winter", "shading_summer", "sun_winter", "sun_summer", "daylight", "autonomy", "cost", "carbon"):
            self.assertIn(key, result)''', '''"solar_winter", "solar_summer", "shading_winter", "shading_summer", "sun_winter", "sun_summer", "daylight", "autonomy", "cost", "carbon",
                    *CASES["baseline"]):
            self.assertIn(key, result)''')
swap('''

class OrientationExtrema(unittest.TestCase):''', addition + '''
class OrientationExtrema(unittest.TestCase):''')
if crlf:
    text = text.replace("\n", "\r\n")
path.write_bytes(text.encode("utf-8"))
print("patched", crlf)
