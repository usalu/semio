import sys
sys.path.insert(0, "./coda/client/bin/assistant")
from main import get_building_ui
import inspect

# `get_building_ui` is a fastmcp resource, so it's a decorator wrapped function.
# we can call its underlying function, or if it's the wrapper, just call it.
html = get_building_ui()
with open("test_ui.html", "w") as f:
    f.write(html)
print("Saved to test_ui.html")
