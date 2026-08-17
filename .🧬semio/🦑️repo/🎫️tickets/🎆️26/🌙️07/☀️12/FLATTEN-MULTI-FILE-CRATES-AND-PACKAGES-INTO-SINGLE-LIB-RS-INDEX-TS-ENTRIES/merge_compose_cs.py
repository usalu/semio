import re

D = "/Users/ueli/Documents/semio/compose/client/lib/net"

def body_after_namespace(path, ns_pattern):
    with open(path) as f:
        text = f.read()
    m = re.search(ns_pattern + r'\s*;\s*\n', text, re.MULTILINE)
    if not m:
        raise ValueError(f"namespace decl not found in {path}")
    return text[m.end():].strip('\n')

# #region-wrapped bodies for Compose.cs siblings
compose_diff = body_after_namespace(f"{D}/Compose/cs/ComposeDiff.Wire.cs", r'^namespace Compose')
kit_inplace = body_after_namespace(f"{D}/Compose/cs/KitInPlaceDiff.cs", r'^namespace Compose')
kit_state = body_after_namespace(f"{D}/Compose/cs/KitState.cs", r'^namespace Compose')

store_files = ["Events.cs", "StoreGraphqlSelection.cs", "StoreGraphql.cs", "StoreClient.cs", "StoreKit.cs", "StoreKitIO.cs"]
store_bodies = []
for fname in store_files:
    b = body_after_namespace(f"{D}/Compose/Store/{fname}", r'^namespace Compose\.Store')
    # drop redundant 'using Compose;' left inside body (StoreKitIO.cs has it right after namespace... actually it's before, already stripped)
    store_bodies.append((fname, b))

with open(f"{D}/Compose/cs/Compose.cs") as f:
    compose_text = f.read()

# 1. add missing usings to Adapters region (right after 'using Compose.Store;')
compose_text = compose_text.replace(
    "using Compose.Store;\n",
    "using Compose.Store;\nusing System.Diagnostics;\nusing System.Net.Sockets;\n",
    1,
)

# 2. convert file-scoped namespace to block form
assert "namespace Compose;\n" in compose_text
compose_text = compose_text.replace("namespace Compose;\n", "namespace Compose\n{\n", 1)

# 3. build appended content (inserted just before final EOF, inside the new closing brace)
appended = []
appended.append("//#region 🔀️ComposeDiff\n" + compose_diff + "\n//#endregion 🔀️ComposeDiff\n")
appended.append("//#region 🔧️KitInPlaceDiff\n" + kit_inplace + "\n//#endregion 🔧️KitInPlaceDiff\n")
appended.append("//#region 📥️KitState\n" + kit_state + "\n//#endregion 📥️KitState\n")

store_section = "//#region 🗄️Store\nnamespace Store\n{\n"
for fname, body in store_bodies:
    label = fname.replace(".cs", "")
    store_section += f"    //#region {label}\n{body}\n    //#endregion {label}\n\n"
store_section += "}\n//#endregion 🗄️Store\n"
appended.append(store_section)

new_text = compose_text.rstrip('\n') + "\n\n" + "\n\n".join(appended) + "\n}\n"

with open(f"{D}/Compose/cs/Compose.cs", "w") as f:
    f.write(new_text)

print("Compose.cs merge complete:", len(new_text.splitlines()), "lines")
