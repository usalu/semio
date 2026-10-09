import os

base = r"C:\git\semio"
framework = next(name for name in os.listdir(base) if name.endswith("framework"))
products = os.path.join(base, framework, next(name for name in os.listdir(os.path.join(base, framework)) if name.endswith("products")))
server = os.path.join(products, next(name for name in os.listdir(products) if name.endswith("server")))
print("server", server.encode("unicode_escape").decode())
for dirpath, dirs, files in os.walk(server):
    dirs[:] = [name for name in dirs if name not in {"node_modules", "dist", "target"}]
    if "packages" in dirpath and os.path.basename(dirpath).endswith("typescript"):
        for name in files:
            path = os.path.join(dirpath, name)
            if name.endswith((".ts", ".tsx")):
                print(os.path.getsize(path), name.encode("unicode_escape").decode())
