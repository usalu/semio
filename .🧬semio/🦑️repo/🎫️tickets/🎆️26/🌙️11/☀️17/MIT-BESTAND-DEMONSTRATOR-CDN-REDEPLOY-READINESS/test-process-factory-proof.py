"""🔗 Check both generated process factory labels against native Rust type identity."""
import json
from pathlib import Path
import re
import subprocess
import tempfile

ticket = Path(__file__).resolve().parent
editor = next(Path("✏️s").glob("🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/**/✏️editor/🦀️.rs")).parent
proofs = (editor / "🦀️.rs").read_text()
factories = (editor / "🧵️retained-commands/🦀️.rs").read_text()
vectors = json.loads((ticket / "🔣️process-factory-proof.json").read_text())
items = []
for case in vectors:
    name = case["factory"]
    declaration = re.search(r'pub\(super\) struct '+name+r'\s*\{[^}]+\}', factories)
    assert declaration, f"{name} must have its own concrete registration"
    items.append(declaration[0].removeprefix("pub(super) "))
oracle = "struct ToolFactoryKey;\n"+"\n".join(items)+"\nfn main() {\n"
for case in vectors:
    oracle += 'println!("{}", std::any::type_name::<'+case["factory"]+'>());\n'
with tempfile.TemporaryDirectory(dir=ticket / "🗑️generated") as directory:
    binary = Path(directory) / "probe"
    subprocess.run(["rustc", "--crate-name", "oracle", "-", "-o", binary], input=oracle+"}", text=True, check=True)
    names = subprocess.check_output([binary], text=True).splitlines()
assert len(names) == len(vectors)
for case, name in zip(vectors, names):
    match = re.search(r'factory: "([^"]+)",\s+factory_type: '+case["factory"]+r',\s+tools: \{([^}]+)', proofs)
    assert match and '"'+case["tool"]+'"' in match[2], case
    assert match[1] == name.rsplit("::", 1)[-1], (case["tool"], match[1], name)
print("Both process factory proofs match the native Rust oracle.")
