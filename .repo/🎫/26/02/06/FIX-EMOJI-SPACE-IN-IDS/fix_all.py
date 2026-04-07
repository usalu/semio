import json
import os
import re

root = "/workspaces/semio"
scopes = ["semio", "coda", "repo"]

# 1. Fix package.json files
for dirpath, dirnames, filenames in os.walk(root):
    if any(d in dirpath for d in ["node_modules", ".venv", ".git", ".nx"]):
        continue
    if "package.json" in filenames:
        path = os.path.join(dirpath, "package.json")
        try:
            with open(path, "r") as f:
                content = f.read()

            # Use regex to find name and dependency keys
            # and only prefix them with @ if they start with a scope and contain /
            # Or if they are exactly the scope name and it's a known scope?
            # Actually, per root package.json, they are all like semio/js etc.

            data = json.loads(content)
            changed = False

            if "name" in data:
                name = data["name"]
                for scope in scopes:
                    if name.startswith(f"{scope}/") and not name.startswith("@"):
                        data["name"] = f"@{name}"
                        changed = True
                        break

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
                            if k.startswith(f"{scope}/") and not k.startswith("@"):
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
            pass

# 2. Fix imports in .ts, .tsx, .tsx files
import_regex = re.compile(r"path_patterns")  # placeholder


def fix_imports(content):
    # Regex to match imports: from "semio/..." or from 'semio/...'
    # Also match dynamic imports: import("semio/...")
    new_content = content
    for scope in scopes:
        # Match "scope/..." but not "@scope/..." or "./scope/..."
        # We look for quotes followed by scope/
        pattern = rf'([\'"]){scope}/'
        replacement = rf"\1@{scope}/"
        new_content = re.sub(pattern, replacement, new_content)
    return new_content


for dirpath, dirnames, filenames in os.walk(root):
    if any(d in dirpath for d in ["node_modules", ".venv", ".git", ".nx"]):
        continue
    for filename in filenames:
        if filename.endswith((".ts", ".tsx", ".js", ".jsx")):
            path = os.path.join(dirpath, filename)
            try:
                with open(path, "r") as f:
                    content = f.read()

                # We need to be careful NOT to replace file paths.
                # But in imports, they are always quoted and start with the scope.
                # File paths usually start with / or ./ or ../ or are inside a path.join.
                # If we only replace strings that START with scope/ or have "scope/
                # it might be too broad.

                # However, many imports are like: import ... from "semio/js/shared"
                # If we find "semio/ and it's at the start of the string value.

                new_content = fix_imports(content)

                if new_content != content:
                    with open(path, "w") as f:
                        f.write(new_content)
                    print(f"Updated imports in {path}")
            except Exception as e:
                print(f"Error processing {path}: {e}")

print("Done.")
