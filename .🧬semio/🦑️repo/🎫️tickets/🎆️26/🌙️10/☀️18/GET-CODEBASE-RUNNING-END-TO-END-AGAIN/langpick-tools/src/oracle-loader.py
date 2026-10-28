def language_oracle():
    """🗣️ The independent oracle of the one language protocol (`⚙️runtime/🗣️language`) and the protocol it reads."""
    import importlib.util
    import os
    root = os.path.dirname(os.path.abspath(__file__))
    while not os.path.isdir(os.path.join(root, "🧰️framework")):
        root = os.path.dirname(root)
    folder = os.path.join(root, "🧰️framework", "🔨️modules", "⚙️runtime", "🗣️language")
    spec = importlib.util.spec_from_file_location("language_oracle", os.path.join(folder, "🧪️tests", "🐍️.py"))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    with open(os.path.join(folder, "🔣️.json"), encoding="utf-8") as handle:
        return module, json.load(handle)
