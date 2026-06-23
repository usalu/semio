import json
import os

root = "/workspaces/semio"
scopes = ["compose", "coda", "repo"]

for dirpath, dirnames, filenames in os.walk(root):
    if "node_modules" in dirpath or ".venv" in dirpath or ".git" in dirpath:
        continue
    if "package.json" in filenames:
        path = os.path.join(dirpath, "package.json")
        try:
            with open(path, "r") as f:
                data = json.load(f)

            changed = False

            # Update name
            if "name" in data:
                name = data["name"]
                for scope in scopes:
                    if name.startswith(f"{scope}/"):
                        data["name"] = f"@{name}"
                        changed = True
                        break

            # Update dependencies
            for dep_type in [
                "dependencies",
                "devDependencies",
                "peerDependencies",
                "optionalDependencies",
            ]:
                if dep_type in data:
                    new_deps = {}
                    for k, v in data[dep_type].items():
                        new_k = k
                        for scope in scopes:
                            if k.startswith(f"{scope}/"):
                                new_k = f"@{k}"
                                changed = True
                                break
                        new_deps[new_k] = v
                    data[dep_type] = new_deps

            if changed:
                with open(path, "w") as f:
                    json.dump(data, f, indent=2)
                    f.write("\n")
                print(f"Updated {path}")
        except Exception as e:
            print(f"Error processing {path}: {e}")
