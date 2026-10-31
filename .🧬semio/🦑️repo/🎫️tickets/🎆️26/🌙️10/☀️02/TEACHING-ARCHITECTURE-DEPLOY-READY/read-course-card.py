from pathlib import Path
path = Path("/home/deck/semio") / "\U0001f9ea\ufe0f" / "framework" 
# locate by walking only the course react target
root = Path("/home/deck/semio")
found = None
for path in root.joinpath("\U0001fa96\ufe0f" if False else ".").rglob("*.tsx"):
    pass
for directory in root.iterdir():
    if directory.name.endswith("framework"):
        products = directory / "\U0001f6cd\ufe0fproducts" / "\U0001f4da\ufe0fcourse" / "\U0001f3af\ufe0ftargets"
        if products.exists():
            for path in products.rglob("*.tsx"):
                if path.parent.name.endswith("chrome"):
                    found = path
print(found)
if found:
    text = found.read_text()
    start = text.find("export function CourseCard")
    print(text[start:start + 2800])
